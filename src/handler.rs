use std::sync::atomic;

use super::{
    IUnknown,
    IUnknownVtbl,
    IID_IUnknown,
    IID_IAgileObject,
    RtType,
    ComInterface,
    ComIid,
    ComPtr
};

use ::rt::{
    IAsyncOperation,
    IAsyncOperationCompletedHandler,
    IAsyncOperationCompletedHandlerVtbl,
    AsyncStatus
};

use ::w::{BOOL, S_OK, HRESULT, VOID, REFIID, ULONG, GUID};

// Custom COM component and implement IAsyncOperationCompletedHandler
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

/// This is a reusable implementation of Com_Release that works for any ComRepr-based type
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

trait ComClass<Interface: ComInterface> where Self: Sized {
    fn get_vtbl() -> &'static Interface::Vtbl;
    fn vtbl(&self) -> &'static Interface::Vtbl {
        Self::get_vtbl()
    }
    fn into_interface(self) -> ComPtr<Interface> {
        let com = Box::new(ComRepr {
            vtbl: Self::get_vtbl(),
            refcount: ::std::sync::atomic::AtomicUsize::new(1),
            data: self
        });
        unsafe { ComPtr::new(Box::into_raw(com) as *mut Interface) }
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

pub struct AsyncOperationCompletedHandler<TResult> where TResult: RtType {
    target_iid: REFIID,
    invoke: Box<FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT>
}

impl<TResult: 'static> AsyncOperationCompletedHandler<TResult> where TResult: RtType, IAsyncOperationCompletedHandler<TResult>: ComIid {
    pub fn new<F>(f: F) -> ComPtr<IAsyncOperationCompletedHandler<TResult>> where F: 'static + Send + FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT {
        AsyncOperationCompletedHandler::<TResult> {
            target_iid: <IAsyncOperationCompletedHandler<TResult> as ComIid>::get_iid(),
            invoke: Box::new(f)
        }.into_interface()
    }
}

// IAsyncOperationCompletedHandlerVtbl only references TResult in type parameter position, so the implementation
// should be the same regardless of TResult, which means that we can just use a dummy `BOOL` here.
const AsyncOperationCompletedHandlerVtbl: &'static IAsyncOperationCompletedHandlerVtbl<BOOL> = &IAsyncOperationCompletedHandlerVtbl::<BOOL> {
    parent: IUnknownVtbl {
        QueryInterface: {
            unsafe extern "system" fn QueryInterface(this_: *mut IUnknown, vTableGuid: REFIID, ppv: *mut *mut VOID) -> HRESULT
            {
                let this_ = this_ as *mut IAsyncOperationCompletedHandler<BOOL>;
                fn guid_eq(guid1: &GUID, guid2: &GUID) -> bool {
                    guid1.Data1 == guid2.Data1 && guid1.Data2 == guid2.Data2 && guid1.Data3 == guid2.Data3 && guid1.Data4 == guid2.Data4
                }
                
                fn print_guid(guid: &GUID) {
                    println!("{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
                        guid.Data1, guid.Data2, guid.Data3,
                        guid.Data4[0], guid.Data4[1], guid.Data4[2], guid.Data4[3],
                        guid.Data4[4], guid.Data4[5], guid.Data4[6], guid.Data4[7]);
                }

                print!("QueryInterface called with GUID ");
                print_guid(&*vTableGuid);
                
                let this: &mut AsyncOperationCompletedHandler<BOOL> = AsyncOperationCompletedHandler::<BOOL>::from_interface(this_);
                
                // TODO: How to determine which IIDs are allowed here?
                if !guid_eq(&*vTableGuid, &IID_IUnknown) &&
                    !guid_eq(&*vTableGuid, &IID_IAgileObject) && // IAgileObject is only supported for Send objects
                    !guid_eq(&*vTableGuid, &*this.target_iid) { 
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
                ComRepr_AddRef::<AsyncOperationCompletedHandler<BOOL>>(this_ as *mut IUnknown);

                // Let the caller know that he indeed has an object of the requested interface.
                return S_OK;
            }
            QueryInterface
        },
        AddRef: ComRepr_AddRef::<AsyncOperationCompletedHandler<BOOL>>,
        Release: ComRepr_Release::<AsyncOperationCompletedHandler<BOOL>>,
    },
    Invoke: {
        unsafe extern "system" fn Invoke(this_: *mut IAsyncOperationCompletedHandler<BOOL>, asyncOperation: *mut IAsyncOperation<BOOL>, status: AsyncStatus) -> HRESULT {
            let this: &mut AsyncOperationCompletedHandler<BOOL> = AsyncOperationCompletedHandler::<BOOL>::from_interface(this_);
            (this.invoke)(asyncOperation, status)
        }
        Invoke
    }
};

impl<TResult: 'static> ComClass<IAsyncOperationCompletedHandler<TResult>> for AsyncOperationCompletedHandler<TResult> where TResult: RtType {
    fn get_vtbl() -> &'static IAsyncOperationCompletedHandlerVtbl<TResult> {
        unsafe { ::std::mem::transmute(AsyncOperationCompletedHandlerVtbl) }
    }
}

impl<TResult> Drop for AsyncOperationCompletedHandler<TResult> where TResult: RtType {
    fn drop(&mut self) {
        println!("Dropped AsyncOperationCompletedHandler<...>!");
    }
}