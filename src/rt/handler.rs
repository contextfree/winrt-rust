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
    vtbl: Box<Vtbl>,
    refcount: atomic::AtomicUsize,
    data: T
}

/// This is a reusable implementation of AddRef that works for any ComRepr-based type
unsafe extern "system" fn ComRepr_AddRef<T>(this: *mut IUnknown) -> ULONG {
    let this = this as *mut _ as *mut ComRepr<T, IUnknownVtbl>;
    
    // Increment the reference count (count member).
    let old_size = (*this).refcount.fetch_add(1, atomic::Ordering::Relaxed);
    println!("AddRef: {} -> {}", old_size, old_size  + 1);

    // We're supposed to return the updated count.
    return (old_size + 1) as ULONG;
}

/// This is a reusable implementation of Release that works for any ComRepr-based type
unsafe extern "system" fn ComRepr_Release<T>(this: *mut IUnknown) -> ULONG {
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

unsafe extern "system" fn ComReprHandler_QueryInterface<T, I>(this_: *mut IUnknown, vTableGuid: REFIID, ppv: *mut *mut VOID) -> HRESULT
    where T: ComClass<I>, I: ComInterface + ComIid
{
    let this_ = this_ as *mut I;
    let guid: Guid = (*vTableGuid).into();
    println!("QueryInterface called with GUID {:?}", guid);

    // IAgileObject is only supported for Send objects
    if guid != *IUnknown::iid() && guid != *IAgileObject::iid() && guid != *<I as ComIid>::iid() { 
        *ppv = ::std::ptr::null_mut();
        return ::w::E_NOINTERFACE;
    }
    *ppv = this_ as *mut _ as *mut VOID;
    ComRepr_AddRef::<T>(this_ as *mut IUnknown);
    S_OK
}

pub trait ComClass<Interface: ComInterface> where Self: Sized {
    fn get_vtbl() -> Interface::Vtbl;
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
            vtbl: Box::new(Self::get_vtbl()),
            refcount: ::std::sync::atomic::AtomicUsize::new(1),
            data: self
        });
        unsafe { ComPtr::wrap(Box::into_raw(com) as *mut Interface) }
    }
}

pub struct AsyncOperationCompletedHandlerImpl<TResult: RtType> {
    invoke: Box<FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT>
}

impl<TResult: RtType + 'static> AsyncOperationCompletedHandlerImpl<TResult> where AsyncOperationCompletedHandler<TResult>: ComIid {
    pub fn new<F>(f: F) -> AsyncOperationCompletedHandlerImpl<TResult> where F: 'static + Send + FnMut(*mut IAsyncOperation<TResult>, AsyncStatus) -> HRESULT {
        AsyncOperationCompletedHandlerImpl::<TResult> {
            invoke: Box::new(f)
        }
    }
}

impl<TResult: RtType + 'static> ComClass<AsyncOperationCompletedHandler<TResult>> for AsyncOperationCompletedHandlerImpl<TResult> where AsyncOperationCompletedHandler<TResult>: ComIid {
    fn get_vtbl() -> AsyncOperationCompletedHandlerVtbl<TResult> {
        AsyncOperationCompletedHandlerVtbl::<TResult> {
            parent: IUnknownVtbl {
                QueryInterface: ComReprHandler_QueryInterface::<AsyncOperationCompletedHandlerImpl<TResult>, _>,
                AddRef: ComRepr_AddRef::<AsyncOperationCompletedHandlerImpl<TResult>>,
                Release: ComRepr_Release::<AsyncOperationCompletedHandlerImpl<TResult>>,
            },
            Invoke: {
                unsafe extern "system" fn Invoke<TResult: RtType + 'static>(this_: *mut AsyncOperationCompletedHandler<TResult>, asyncOperation: *mut IAsyncOperation<TResult>, status: AsyncStatus) -> HRESULT
                    where AsyncOperationCompletedHandler<TResult>: ComIid
                {
                    let this: &mut AsyncOperationCompletedHandlerImpl<TResult> = ComClass::from_interface(this_);
                    (this.invoke)(asyncOperation, status)
                }
                Invoke::<TResult>
            }
        }
    }
}

impl<TResult> Drop for AsyncOperationCompletedHandlerImpl<TResult> where TResult: RtType {
    fn drop(&mut self) {
        println!("Dropped AsyncOperationCompletedHandlerImpl<...>!");
    }
}