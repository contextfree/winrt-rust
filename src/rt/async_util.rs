use futures::future::Future;
use std::pin::Pin;
use std::ops::Deref;
use std::task::{Context, Poll};
use std::sync::{Arc, Mutex, Condvar};

use crate::{
    RtType,
    ComIid,
    Result
};

use crate::windows::foundation::{
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

/// Extension for `IAsyncAction` with helper method.
pub trait RtAsyncAction {
    /// Waits for the asynchronous action to complete, blocking the current thread.
    fn blocking_wait(&self);
}

/// Extension for `IAsyncOperation` with helper methods.
pub trait RtAsyncOperation: RtAsyncAction {
    type TResult;
    
    fn get_results(&self) -> Result<Self::TResult>;

    #[inline]
    /// Waits for the asynchronous operation to complete, blocking the current thread,
    /// then return the result.
    fn blocking_get(&self) -> Result<Self::TResult> {
        self.blocking_wait();
        self.get_results()
    }
}

// The handler type is different for each interface, and the easiest way to share code seems to be a macro
macro_rules! impl_blocking_wait {
    ($handler:ident) => {
        #[inline]
        fn blocking_wait(&self) {
            let info = crate::comptr::query_interface::<_, IAsyncInfo>(self).expect("query_interface failed");
            let status = info.get_status().expect("get_status failed");

            if status == crate::windows::foundation::AsyncStatus::Completed {
                return;
            }
            
            let pair = Arc::new((Mutex::new(false), Condvar::new()));
            {
                let pair2 = pair.clone();
                let handler = $handler::new(move |_op, _status| {
                    let &(ref lock, ref cvar) = &*pair2;
                    let mut completed = lock.lock().expect("lock failed");
                    *completed = true;
                    cvar.notify_one();
                    Ok(())
                });
                self.set_completed(&handler).expect("set_completed failed");
                // local reference to `handler` is dropped here -> Release() is called
            }
            
            // use condvar to wait until handler has been called
            let &(ref lock, ref cvar) = &*pair;
            let mut completed = lock.lock().expect("lock failed");
            while !*completed {
                completed = cvar.wait(completed).expect("wait failed");
            }
        }
    }
}

impl RtAsyncAction for IAsyncAction {
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
    fn get_results(&self) -> Result<Self::TResult> {
        self.get_results()
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
    fn get_results(&self) -> Result<Self::TResult> {
        self.get_results()
    }
}


macro_rules! impl_poll {
    ($handler:ident => $retexpr:tt) => {
        fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
            let info = crate::comptr::query_interface::<_, IAsyncInfo>(self.deref().deref()).expect("query_interface failed");
            let status = info.get_status().expect("get_status failed");
            match status {
                crate::windows::foundation::AsyncStatus::Completed => {
                    Poll::Ready($retexpr(self))
                },
                crate::windows::foundation::AsyncStatus::Started => {
                    // Calling poll multiple times must work correctly, so we have to check that we didn't already install the Completed handler
                    if self.get_completed().expect("get_completed failed").is_some() {
                        // TODO: We might have to check that the installed handler is actually
                        //       the one with the waker (because the user could have installed one independently).
                        //       Or we document that the user is not allowed to do that.
                        return Poll::Pending;
                    }
                    let waker = cx.waker().clone();

                    let handler = $handler::new(move |_op, _status| {
                        waker.wake_by_ref();
                        Ok(())
                    });

                    self.set_completed(&handler).expect("set_completed failed");
                    Poll::Pending
                }
                _ => unimplemented!() // FIXME
            }
        }
    }
}


impl Future for crate::ComPtr<IAsyncAction> {
    type Output = ();

    impl_poll!{ AsyncActionCompletedHandler => { |_: Pin<&mut Self>| () } }
}

impl<P: RtType + 'static> Future for crate::ComPtr<IAsyncActionWithProgress<P>>
    where AsyncActionWithProgressCompletedHandler<P>: ComIid
{
    type Output = ();

    impl_poll!{ AsyncActionWithProgressCompletedHandler => { |_: Pin<&mut Self>| () } }
}

impl<T: RtType + 'static> Future for crate::ComPtr<IAsyncOperation<T>>
    where AsyncOperationCompletedHandler<T>: ComIid
{
    type Output = Result<<T as RtType>::Out>;

    impl_poll!{ AsyncOperationCompletedHandler => { |s: Pin<&mut Self>| s.get_results() } }
}

impl<T: RtType + 'static, P: RtType + 'static> Future for crate::ComPtr<IAsyncOperationWithProgress<T, P>>
    where AsyncOperationWithProgressCompletedHandler<T, P>: ComIid
{
    type Output = Result<<T as RtType>::Out>;

    impl_poll!{ AsyncOperationWithProgressCompletedHandler => { |s: Pin<&mut Self>| s.get_results() } }
}
