use std::sync::atomic;

use ::{
    IUnknown,
    IUnknownVtbl,
    IAgileObject,
    RtType,
    ComInterface,
    ComIid,
    ComPtr,
    Guid
};

use ::rt::gen::windows::foundation::{
    IAsyncOperation,
    AsyncOperationCompletedHandler,
    AsyncOperationCompletedHandlerVtbl,
    AsyncStatus
};

use ::w::{S_OK, HRESULT, VOID, REFIID, ULONG};

// Define custom COM component and implement AsyncOperationCompletedHandler
#[repr(C)]
pub struct ComRepr<T, Vtbl> {
    vtbl: *const Vtbl,
    refcount: atomic::AtomicUsize,
    data: T
}

/// This is a reusable implementation of AddRef that works for any ComRepr-based type
unsafe extern "system" fn ComRepr_AddRef<T>(this: *mut IUnknown) -> ULONG
{
    let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
    
    // Increment the reference count (count member).
    let old_size = (*this).refcount.fetch_add(1, atomic::Ordering::Relaxed);
    println!("AddRef: {} -> {}", old_size, old_size  + 1);

    // We're supposed to return the updated count.
    return (old_size + 1) as ULONG;
}

/// This is a reusable implementation of Release that works for any ComRepr-based type
unsafe extern "system" fn ComRepr_Release<T>(this: *mut IUnknown) -> ULONG
{
    let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
    
    let old_size = (*this).refcount.fetch_sub(1, atomic::Ordering::Release);
    println!("Release: {} -> {}", old_size, old_size - 1);
    if old_size != 1 {
        return (old_size - 1) as ULONG; // return the updated count
    }
    
    atomic::fence(atomic::Ordering::Acquire); 
    let b = Box::from_raw(this); // creates a Box which is then dropped
    drop(b); // Arc uses a trick to call this in an inline(never) function
    return 0;
}

pub trait ComClass<Interface: ComInterface> where Self: Sized {
    fn get_vtbl() -> &'static Interface::Vtbl;
    fn vtbl(&self) -> &'static Interface::Vtbl {
        Self::get_vtbl()
    }
    unsafe fn from_interface<'a>(thing: *mut Interface) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn from_unknown<'a>(thing: *mut IUnknown) -> &'a mut Self {
        &mut (*(thing as *mut _ as *mut ComRepr<Self, Interface::Vtbl>)).data
    }
    unsafe fn destroy(thing: *mut IUnknown) {
        Box::from_raw(thing as *mut ComRepr<Self, Interface::Vtbl>);
    }
}

pub trait IntoInterface<Interface: ComInterface> {
    fn into_interface(self) -> ComPtr<Interface>;
}

impl<T, Interface: ComInterface> IntoInterface<Interface> for T where T: ComClass<Interface> + Sized {
    fn into_interface(self) -> ComPtr<Interface> {
        let com = Box::new(ComRepr {
            vtbl: Self::get_vtbl(),
            refcount: ::std::sync::atomic::AtomicUsize::new(1),
            data: self
        });
        unsafe { ComPtr::wrap(Box::into_raw(com) as *mut Interface) }
    }
}

pub struct AsyncOperationCompletedHandlerImpl<TResult> where TResult: RtType {
    target_iid: &'static Guid,
    invoke: Box<FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT>
}

impl<TResult: 'static> AsyncOperationCompletedHandlerImpl<TResult> where TResult: RtType, AsyncOperationCompletedHandler<TResult>: ComIid {
    pub fn new<F>(f: F) -> AsyncOperationCompletedHandlerImpl<TResult> where F: 'static + Send + FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT {
        AsyncOperationCompletedHandlerImpl::<TResult> {
            target_iid: <AsyncOperationCompletedHandler<TResult> as ComIid>::iid(),
            invoke: Box::new(f)
        }
    }
}

type __Any = bool;

// AsyncOperationCompletedHandlerVtbl only references TResult in type parameter position, so the implementation
// should be the same regardless of TResult, which means that we can just use a dummy `bool` here.
const AsyncOperationCompletedHandlerImplVtbl: &'static AsyncOperationCompletedHandlerVtbl<__Any> = &AsyncOperationCompletedHandlerVtbl::<__Any> {
    parent: IUnknownVtbl {
        QueryInterface: {
            unsafe extern "system" fn QueryInterface(this_: *mut IUnknown, vTableGuid: REFIID, ppv: *mut *mut VOID) -> HRESULT
            {
                let this_ = this_ as *mut AsyncOperationCompletedHandler<__Any>;
                let guid: Guid = (*vTableGuid).into();
                println!("QueryInterface called with GUID {:?}", guid);

                let this: &mut AsyncOperationCompletedHandlerImpl<__Any> = AsyncOperationCompletedHandlerImpl::<__Any>::from_interface(this_);
                
                // TODO: How to determine which IIDs are allowed here?
                if guid != *IUnknown::iid() &&
                    guid != *IAgileObject::iid() && // IAgileObject is only supported for Send objects
                    guid != *this.target_iid { 
                    // We don't recognize the GUID passed to us. Let the caller know this,
                    // by clearing his handle, and returning E_NOINTERFACE.
                    *ppv = ::std::ptr::null_mut();
                    return ::w::E_NOINTERFACE;
                }

                // It's a match!

                // First, we fill in his handle with the same object pointer he passed us.
                *ppv = this_ as *mut _ as *mut VOID;

                // Now we call our own AddRef function, which we can do without vtable lookup
                // Alternatively could call: (&mut *this_).AddRef();
                ComRepr_AddRef::<AsyncOperationCompletedHandlerImpl<__Any>>(this_ as *mut IUnknown);

                // Let the caller know that he indeed has an object of the requested interface.
                return S_OK;
            }
            QueryInterface
        },
        AddRef: ComRepr_AddRef::<AsyncOperationCompletedHandlerImpl<__Any>>,
        Release: ComRepr_Release::<AsyncOperationCompletedHandlerImpl<__Any>>,
    },
    Invoke: {
        unsafe extern "system" fn Invoke(this_: *mut AsyncOperationCompletedHandler<__Any>, asyncOperation: *mut IAsyncOperation<__Any>, status: AsyncStatus) -> HRESULT {
            let this: &mut AsyncOperationCompletedHandlerImpl<__Any> = AsyncOperationCompletedHandlerImpl::<__Any>::from_interface(this_);
            (this.invoke)(asyncOperation, status)
        }
        Invoke
    }
};

impl<TResult: 'static> ComClass<AsyncOperationCompletedHandler<TResult>> for AsyncOperationCompletedHandlerImpl<TResult> where TResult: RtType {
    fn get_vtbl() -> &'static AsyncOperationCompletedHandlerVtbl<TResult> {
        unsafe { ::std::mem::transmute(AsyncOperationCompletedHandlerImplVtbl) }
    }
}

impl<TResult> Drop for AsyncOperationCompletedHandlerImpl<TResult> where TResult: RtType {
    fn drop(&mut self) {
        println!("Dropped AsyncOperationCompletedHandlerImpl<...>!");
    }
}