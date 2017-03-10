use std::sync::{Arc, Mutex, Condvar};

use ::{
    RtType,
    ComIid
};

use ::windows::foundation::{
    IAsyncInfo,
    IAsyncAction,
    AsyncActionCompletedHandler,
    IAsyncActionWithProgress,
    AsyncActionWithProgressCompletedHandler,
    IAsyncOperation,
    AsyncOperationCompletedHandler,
    IAsyncOperationWithProgress,
    AsyncOperationWithProgressCompletedHandler
};

pub trait RtAsyncAction {
    fn blocking_wait(&mut self);
}

pub trait RtAsyncOperation: RtAsyncAction {
    type TResult;
    
    fn get_results(&mut self) -> Self::TResult;

    #[inline]
    fn blocking_get(&mut self) -> Self::TResult {
        self.blocking_wait();
        self.get_results()
    }
}

// The handler type is different for each interface, and the easiest way to share code seems to be a macro
macro_rules! impl_blocking_wait {
    ($handler:ident) => {
        #[inline]
        fn blocking_wait(&mut self) {
            let mut info = ::comptr::query_interface::<_, IAsyncInfo>(self).unwrap();
            let status = unsafe { info.get_status().unwrap() };
            if status == ::windows::foundation::AsyncStatus_Completed {
                return;
            }
            
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            {
                let pair2 = pair.clone();
                let mut handler = $handler::new(move |_op, _status| {
                    let &(ref lock, ref cvar) = &*pair2;
                    let mut completed = lock.lock().unwrap();
                    *completed = true;
                    cvar.notify_one();
                    Ok(())
                });
                unsafe { self.set_completed(&mut handler).unwrap() };
                // local reference to `handler` is dropped here -> Release() is called
            }
            
            // use condvar to wait until handler has been called
            let &(ref lock, ref cvar) = &*pair;
            let mut completed = lock.lock().unwrap();
            while !*completed {
                completed = cvar.wait(completed).unwrap();
            }
        }
    }
}

impl RtAsyncAction for IAsyncAction
{
    impl_blocking_wait!{ AsyncActionCompletedHandler }
}

impl<P: RtType + 'static> RtAsyncAction for IAsyncActionWithProgress<P>
    where AsyncActionWithProgressCompletedHandler<P>: ComIid
{
    impl_blocking_wait!{ AsyncActionWithProgressCompletedHandler }
}

impl<T: RtType + 'static> RtAsyncAction for IAsyncOperation<T>
    where AsyncOperationCompletedHandler<T>: ComIid
{
    impl_blocking_wait!{ AsyncOperationCompletedHandler }
}

impl<T: RtType + 'static> RtAsyncOperation for IAsyncOperation<T>
    where AsyncOperationCompletedHandler<T>: ComIid
{
    type TResult = <T as RtType>::Out;

    #[inline]
    fn get_results(&mut self) -> Self::TResult {
        unsafe { self.get_results().unwrap() }
    }
}

impl<T: RtType + 'static, P: RtType + 'static> RtAsyncAction for IAsyncOperationWithProgress<T, P>
    where AsyncOperationWithProgressCompletedHandler<T, P>: ComIid
{
    impl_blocking_wait!{ AsyncOperationWithProgressCompletedHandler }
}

impl<T: RtType + 'static, P: RtType + 'static> RtAsyncOperation for IAsyncOperationWithProgress<T, P>
    where AsyncOperationWithProgressCompletedHandler<T, P>: ComIid
{
    type TResult = <T as RtType>::Out;

    #[inline]
    fn get_results(&mut self) -> Self::TResult {
        unsafe { self.get_results().unwrap() }
    }
}