use crate::prelude::*;
RT_ENUM! { enum DomainNameType: i32 {
    Suffix = 0, FullyQualified = 1,
}}
DEFINE_IID!(IID_IEndpointPair, 866167350, 63738, 19248, 184, 86, 118, 81, 124, 59, 208, 109);
RT_INTERFACE!{interface IEndpointPair(IEndpointPairVtbl): IInspectable(IInspectableVtbl) [IID_IEndpointPair] {
    fn get_LocalHostName(&self, out: *mut *mut HostName) -> HRESULT,
    fn put_LocalHostName(&self, value: *mut HostName) -> HRESULT,
    fn get_LocalServiceName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_LocalServiceName(&self, value: HSTRING) -> HRESULT,
    fn get_RemoteHostName(&self, out: *mut *mut HostName) -> HRESULT,
    fn put_RemoteHostName(&self, value: *mut HostName) -> HRESULT,
    fn get_RemoteServiceName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_RemoteServiceName(&self, value: HSTRING) -> HRESULT
}}
impl IEndpointPair {
    #[inline] pub fn get_local_host_name(&self) -> Result<Option<ComPtr<HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalHostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_local_host_name(&self, value: &HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_LocalHostName)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_local_service_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalServiceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_local_service_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_LocalServiceName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_host_name(&self) -> Result<Option<ComPtr<HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteHostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_remote_host_name(&self, value: &HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RemoteHostName)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_service_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteServiceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_remote_service_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RemoteServiceName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class EndpointPair: IEndpointPair}
impl RtActivatable<IEndpointPairFactory> for EndpointPair {}
impl EndpointPair {
    #[inline] pub fn create_endpoint_pair(localHostName: &HostName, localServiceName: &HStringArg, remoteHostName: &HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<EndpointPair>> {
        <Self as RtActivatable<IEndpointPairFactory>>::get_activation_factory().create_endpoint_pair(localHostName, localServiceName, remoteHostName, remoteServiceName)
    }
}
DEFINE_CLSID!(EndpointPair(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,69,110,100,112,111,105,110,116,80,97,105,114,0]) [CLSID_EndpointPair]);
DEFINE_IID!(IID_IEndpointPairFactory, 3054098801, 25824, 17451, 170, 111, 204, 140, 143, 24, 31, 120);
RT_INTERFACE!{static interface IEndpointPairFactory(IEndpointPairFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IEndpointPairFactory] {
    fn CreateEndpointPair(&self, localHostName: *mut HostName, localServiceName: HSTRING, remoteHostName: *mut HostName, remoteServiceName: HSTRING, out: *mut *mut EndpointPair) -> HRESULT
}}
impl IEndpointPairFactory {
    #[inline] pub fn create_endpoint_pair(&self, localHostName: &HostName, localServiceName: &HStringArg, remoteHostName: &HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<EndpointPair>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEndpointPair)(self as *const _ as *mut _, localHostName as *const _ as *mut _, localServiceName.get(), remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHostName, 3213806253, 60822, 18855, 144, 132, 212, 22, 202, 232, 141, 203);
RT_INTERFACE!{interface IHostName(IHostNameVtbl): IInspectable(IInspectableVtbl) [IID_IHostName] {
    fn get_IPInformation(&self, out: *mut *mut connectivity::IPInformation) -> HRESULT,
    fn get_RawName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CanonicalName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Type(&self, out: *mut HostNameType) -> HRESULT,
    fn IsEqual(&self, hostName: *mut HostName, out: *mut bool) -> HRESULT
}}
impl IHostName {
    #[inline] pub fn get_ip_information(&self) -> Result<Option<ComPtr<connectivity::IPInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IPInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RawName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_canonical_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CanonicalName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<HostNameType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_equal(&self, hostName: &HostName) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsEqual)(self as *const _ as *mut _, hostName as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HostName: IHostName}
impl RtActivatable<IHostNameFactory> for HostName {}
impl RtActivatable<IHostNameStatics> for HostName {}
impl HostName {
    #[inline] pub fn create_host_name(hostName: &HStringArg) -> Result<ComPtr<HostName>> {
        <Self as RtActivatable<IHostNameFactory>>::get_activation_factory().create_host_name(hostName)
    }
    #[inline] pub fn compare(value1: &HStringArg, value2: &HStringArg) -> Result<i32> {
        <Self as RtActivatable<IHostNameStatics>>::get_activation_factory().compare(value1, value2)
    }
}
DEFINE_CLSID!(HostName(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,72,111,115,116,78,97,109,101,0]) [CLSID_HostName]);
DEFINE_IID!(IID_IHostNameFactory, 1166812141, 28975, 17782, 173, 241, 194, 11, 44, 100, 53, 88);
RT_INTERFACE!{static interface IHostNameFactory(IHostNameFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHostNameFactory] {
    fn CreateHostName(&self, hostName: HSTRING, out: *mut *mut HostName) -> HRESULT
}}
impl IHostNameFactory {
    #[inline] pub fn create_host_name(&self, hostName: &HStringArg) -> Result<ComPtr<HostName>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateHostName)(self as *const _ as *mut _, hostName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HostNameSortOptions: u32 {
    None = 0, OptimizeForLongConnections = 2,
}}
DEFINE_IID!(IID_IHostNameStatics, 4136424639, 41864, 20107, 145, 234, 84, 221, 109, 217, 1, 192);
RT_INTERFACE!{static interface IHostNameStatics(IHostNameStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHostNameStatics] {
    fn Compare(&self, value1: HSTRING, value2: HSTRING, out: *mut i32) -> HRESULT
}}
impl IHostNameStatics {
    #[inline] pub fn compare(&self, value1: &HStringArg, value2: &HStringArg) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).Compare)(self as *const _ as *mut _, value1.get(), value2.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum HostNameType: i32 {
    DomainName = 0, Ipv4 = 1, Ipv6 = 2, Bluetooth = 3,
}}
pub mod backgroundtransfer { // Windows.Networking.BackgroundTransfer
use crate::prelude::*;
DEFINE_IID!(IID_IBackgroundDownloader, 3251082035, 26185, 19229, 168, 38, 164, 179, 221, 35, 77, 11);
RT_INTERFACE!{interface IBackgroundDownloader(IBackgroundDownloaderVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloader] {
    #[cfg(feature="windows-storage")] fn CreateDownload(&self, uri: *mut foundation::Uri, resultFile: *mut super::super::storage::IStorageFile, out: *mut *mut DownloadOperation) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateDownloadFromFile(&self, uri: *mut foundation::Uri, resultFile: *mut super::super::storage::IStorageFile, requestBodyFile: *mut super::super::storage::IStorageFile, out: *mut *mut DownloadOperation) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateDownloadAsync(&self, uri: *mut foundation::Uri, resultFile: *mut super::super::storage::IStorageFile, requestBodyStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncOperation<DownloadOperation>) -> HRESULT
}}
impl IBackgroundDownloader {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_download(&self, uri: &foundation::Uri, resultFile: &super::super::storage::IStorageFile) -> Result<Option<ComPtr<DownloadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDownload)(self as *const _ as *mut _, uri as *const _ as *mut _, resultFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_download_from_file(&self, uri: &foundation::Uri, resultFile: &super::super::storage::IStorageFile, requestBodyFile: &super::super::storage::IStorageFile) -> Result<Option<ComPtr<DownloadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDownloadFromFile)(self as *const _ as *mut _, uri as *const _ as *mut _, resultFile as *const _ as *mut _, requestBodyFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_download_async(&self, uri: &foundation::Uri, resultFile: &super::super::storage::IStorageFile, requestBodyStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<foundation::IAsyncOperation<DownloadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateDownloadAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, resultFile as *const _ as *mut _, requestBodyStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundDownloader: IBackgroundDownloader}
impl RtActivatable<IBackgroundDownloaderFactory> for BackgroundDownloader {}
impl RtActivatable<IBackgroundDownloaderStaticMethods> for BackgroundDownloader {}
impl RtActivatable<IBackgroundDownloaderStaticMethods2> for BackgroundDownloader {}
impl RtActivatable<IBackgroundDownloaderUserConsent> for BackgroundDownloader {}
impl RtActivatable<IActivationFactory> for BackgroundDownloader {}
impl BackgroundDownloader {
    #[inline] pub fn create_with_completion_group(completionGroup: &BackgroundTransferCompletionGroup) -> Result<ComPtr<BackgroundDownloader>> {
        <Self as RtActivatable<IBackgroundDownloaderFactory>>::get_activation_factory().create_with_completion_group(completionGroup)
    }
    #[inline] pub fn get_current_downloads_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> {
        <Self as RtActivatable<IBackgroundDownloaderStaticMethods>>::get_activation_factory().get_current_downloads_async()
    }
    #[inline] pub fn get_current_downloads_for_group_async(group: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> {
        <Self as RtActivatable<IBackgroundDownloaderStaticMethods>>::get_activation_factory().get_current_downloads_for_group_async(group)
    }
    #[inline] pub fn get_current_downloads_for_transfer_group_async(group: &BackgroundTransferGroup) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> {
        <Self as RtActivatable<IBackgroundDownloaderStaticMethods2>>::get_activation_factory().get_current_downloads_for_transfer_group_async(group)
    }
    #[inline] pub fn request_unconstrained_downloads_async(operations: &foundation::collections::IIterable<DownloadOperation>) -> Result<ComPtr<foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>> {
        <Self as RtActivatable<IBackgroundDownloaderUserConsent>>::get_activation_factory().request_unconstrained_downloads_async(operations)
    }
}
DEFINE_CLSID!(BackgroundDownloader(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,68,111,119,110,108,111,97,100,101,114,0]) [CLSID_BackgroundDownloader]);
DEFINE_IID!(IID_IBackgroundDownloader2, 2840221767, 13453, 18997, 137, 14, 138, 30, 243, 121, 132, 121);
RT_INTERFACE!{interface IBackgroundDownloader2(IBackgroundDownloader2Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloader2] {
    fn get_TransferGroup(&self, out: *mut *mut BackgroundTransferGroup) -> HRESULT,
    fn put_TransferGroup(&self, value: *mut BackgroundTransferGroup) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_SuccessToastNotification(&self, out: *mut *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_SuccessToastNotification(&self, value: *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_FailureToastNotification(&self, out: *mut *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_FailureToastNotification(&self, value: *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_SuccessTileNotification(&self, out: *mut *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_SuccessTileNotification(&self, value: *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_FailureTileNotification(&self, out: *mut *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_FailureTileNotification(&self, value: *mut super::super::ui::notifications::TileNotification) -> HRESULT
}}
impl IBackgroundDownloader2 {
    #[inline] pub fn get_transfer_group(&self) -> Result<Option<ComPtr<BackgroundTransferGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_transfer_group(&self, value: &BackgroundTransferGroup) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TransferGroup)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_success_toast_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::ToastNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuccessToastNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_success_toast_notification(&self, value: &super::super::ui::notifications::ToastNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SuccessToastNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_failure_toast_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::ToastNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FailureToastNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_failure_toast_notification(&self, value: &super::super::ui::notifications::ToastNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FailureToastNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_success_tile_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::TileNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuccessTileNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_success_tile_notification(&self, value: &super::super::ui::notifications::TileNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SuccessTileNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_failure_tile_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::TileNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FailureTileNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_failure_tile_notification(&self, value: &super::super::ui::notifications::TileNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FailureTileNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundDownloader3, 3508177992, 34536, 18658, 182, 21, 105, 118, 170, 191, 134, 29);
RT_INTERFACE!{interface IBackgroundDownloader3(IBackgroundDownloader3Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloader3] {
    fn get_CompletionGroup(&self, out: *mut *mut BackgroundTransferCompletionGroup) -> HRESULT
}}
impl IBackgroundDownloader3 {
    #[inline] pub fn get_completion_group(&self) -> Result<Option<ComPtr<BackgroundTransferCompletionGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CompletionGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundDownloaderFactory, 646147108, 55454, 18164, 162, 154, 79, 77, 79, 20, 65, 85);
RT_INTERFACE!{static interface IBackgroundDownloaderFactory(IBackgroundDownloaderFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloaderFactory] {
    fn CreateWithCompletionGroup(&self, completionGroup: *mut BackgroundTransferCompletionGroup, out: *mut *mut BackgroundDownloader) -> HRESULT
}}
impl IBackgroundDownloaderFactory {
    #[inline] pub fn create_with_completion_group(&self, completionGroup: &BackgroundTransferCompletionGroup) -> Result<ComPtr<BackgroundDownloader>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithCompletionGroup)(self as *const _ as *mut _, completionGroup as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundDownloaderStaticMethods, 1386633781, 50766, 17004, 153, 25, 84, 13, 13, 33, 166, 80);
RT_INTERFACE!{static interface IBackgroundDownloaderStaticMethods(IBackgroundDownloaderStaticMethodsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloaderStaticMethods] {
    fn GetCurrentDownloadsAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>) -> HRESULT,
    fn GetCurrentDownloadsForGroupAsync(&self, group: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>) -> HRESULT
}}
impl IBackgroundDownloaderStaticMethods {
    #[inline] pub fn get_current_downloads_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentDownloadsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_downloads_for_group_async(&self, group: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentDownloadsForGroupAsync)(self as *const _ as *mut _, group.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundDownloaderStaticMethods2, 799675175, 6868, 19621, 178, 205, 8, 219, 240, 116, 106, 254);
RT_INTERFACE!{static interface IBackgroundDownloaderStaticMethods2(IBackgroundDownloaderStaticMethods2Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloaderStaticMethods2] {
    fn GetCurrentDownloadsForTransferGroupAsync(&self, group: *mut BackgroundTransferGroup, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>) -> HRESULT
}}
impl IBackgroundDownloaderStaticMethods2 {
    #[inline] pub fn get_current_downloads_for_transfer_group_async(&self, group: &BackgroundTransferGroup) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<DownloadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentDownloadsForTransferGroupAsync)(self as *const _ as *mut _, group as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundDownloaderUserConsent, 1561651462, 37478, 18440, 189, 113, 89, 37, 242, 163, 19, 10);
RT_INTERFACE!{static interface IBackgroundDownloaderUserConsent(IBackgroundDownloaderUserConsentVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundDownloaderUserConsent] {
    fn RequestUnconstrainedDownloadsAsync(&self, operations: *mut foundation::collections::IIterable<DownloadOperation>, out: *mut *mut foundation::IAsyncOperation<UnconstrainedTransferRequestResult>) -> HRESULT
}}
impl IBackgroundDownloaderUserConsent {
    #[inline] pub fn request_unconstrained_downloads_async(&self, operations: &foundation::collections::IIterable<DownloadOperation>) -> Result<ComPtr<foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestUnconstrainedDownloadsAsync)(self as *const _ as *mut _, operations as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct BackgroundDownloadProgress {
    BytesReceived: u64, TotalBytesToReceive: u64, Status: BackgroundTransferStatus, HasResponseChanged: bool, HasRestarted: bool,
}}
DEFINE_IID!(IID_IBackgroundTransferBase, 714973776, 51049, 17804, 175, 232, 254, 184, 212, 211, 178, 239);
RT_INTERFACE!{interface IBackgroundTransferBase(IBackgroundTransferBaseVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferBase] {
    fn SetRequestHeader(&self, headerName: HSTRING, headerValue: HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, credential: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, credential: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    fn get_Method(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Method(&self, value: HSTRING) -> HRESULT,
    fn get_Group(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Group(&self, value: HSTRING) -> HRESULT,
    fn get_CostPolicy(&self, out: *mut BackgroundTransferCostPolicy) -> HRESULT,
    fn put_CostPolicy(&self, value: BackgroundTransferCostPolicy) -> HRESULT
}}
impl IBackgroundTransferBase {
    #[inline] pub fn set_request_header(&self, headerName: &HStringArg, headerValue: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetRequestHeader)(self as *const _ as *mut _, headerName.get(), headerValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_server_credential(&self, credential: &super::super::security::credentials::PasswordCredential) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ServerCredential)(self as *const _ as *mut _, credential as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_proxy_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_proxy_credential(&self, credential: &super::super::security::credentials::PasswordCredential) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProxyCredential)(self as *const _ as *mut _, credential as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_method(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Method)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_method(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Method)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_group(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Group)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_group(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Group)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cost_policy(&self) -> Result<BackgroundTransferCostPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CostPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_cost_policy(&self, value: BackgroundTransferCostPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_CostPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum BackgroundTransferBehavior: i32 {
    Parallel = 0, Serialized = 1,
}}
DEFINE_IID!(IID_IBackgroundTransferCompletionGroup, 764609061, 39019, 22349, 121, 80, 10, 221, 71, 245, 215, 6);
RT_INTERFACE!{interface IBackgroundTransferCompletionGroup(IBackgroundTransferCompletionGroupVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferCompletionGroup] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Trigger(&self, out: *mut *mut super::super::applicationmodel::background::IBackgroundTrigger) -> HRESULT,
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn Enable(&self) -> HRESULT
}}
impl IBackgroundTransferCompletionGroup {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_trigger(&self) -> Result<Option<ComPtr<super::super::applicationmodel::background::IBackgroundTrigger>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Trigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn enable(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Enable)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundTransferCompletionGroup: IBackgroundTransferCompletionGroup}
impl RtActivatable<IActivationFactory> for BackgroundTransferCompletionGroup {}
DEFINE_CLSID!(BackgroundTransferCompletionGroup(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,67,111,109,112,108,101,116,105,111,110,71,114,111,117,112,0]) [CLSID_BackgroundTransferCompletionGroup]);
DEFINE_IID!(IID_IBackgroundTransferCompletionGroupTriggerDetails, 2070667910, 28231, 20790, 127, 203, 250, 67, 137, 244, 111, 91);
RT_INTERFACE!{interface IBackgroundTransferCompletionGroupTriggerDetails(IBackgroundTransferCompletionGroupTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferCompletionGroupTriggerDetails] {
    fn get_Downloads(&self, out: *mut *mut foundation::collections::IVectorView<DownloadOperation>) -> HRESULT,
    fn get_Uploads(&self, out: *mut *mut foundation::collections::IVectorView<UploadOperation>) -> HRESULT
}}
impl IBackgroundTransferCompletionGroupTriggerDetails {
    #[inline] pub fn get_downloads(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<DownloadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Downloads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uploads(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<UploadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uploads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundTransferCompletionGroupTriggerDetails: IBackgroundTransferCompletionGroupTriggerDetails}
DEFINE_IID!(IID_IBackgroundTransferContentPart, 3907081815, 55249, 20184, 131, 142, 103, 74, 194, 23, 172, 230);
RT_INTERFACE!{interface IBackgroundTransferContentPart(IBackgroundTransferContentPartVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferContentPart] {
    fn SetHeader(&self, headerName: HSTRING, headerValue: HSTRING) -> HRESULT,
    fn SetText(&self, value: HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SetFile(&self, value: *mut super::super::storage::IStorageFile) -> HRESULT
}}
impl IBackgroundTransferContentPart {
    #[inline] pub fn set_header(&self, headerName: &HStringArg, headerValue: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetHeader)(self as *const _ as *mut _, headerName.get(), headerValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_file(&self, value: &super::super::storage::IStorageFile) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetFile)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundTransferContentPart: IBackgroundTransferContentPart}
impl RtActivatable<IBackgroundTransferContentPartFactory> for BackgroundTransferContentPart {}
impl RtActivatable<IActivationFactory> for BackgroundTransferContentPart {}
impl BackgroundTransferContentPart {
    #[inline] pub fn create_with_name(name: &HStringArg) -> Result<ComPtr<BackgroundTransferContentPart>> {
        <Self as RtActivatable<IBackgroundTransferContentPartFactory>>::get_activation_factory().create_with_name(name)
    }
    #[inline] pub fn create_with_name_and_file_name(name: &HStringArg, fileName: &HStringArg) -> Result<ComPtr<BackgroundTransferContentPart>> {
        <Self as RtActivatable<IBackgroundTransferContentPartFactory>>::get_activation_factory().create_with_name_and_file_name(name, fileName)
    }
}
DEFINE_CLSID!(BackgroundTransferContentPart(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,67,111,110,116,101,110,116,80,97,114,116,0]) [CLSID_BackgroundTransferContentPart]);
DEFINE_IID!(IID_IBackgroundTransferContentPartFactory, 2431621289, 31233, 18955, 159, 128, 160, 176, 187, 55, 15, 141);
RT_INTERFACE!{static interface IBackgroundTransferContentPartFactory(IBackgroundTransferContentPartFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferContentPartFactory] {
    fn CreateWithName(&self, name: HSTRING, out: *mut *mut BackgroundTransferContentPart) -> HRESULT,
    fn CreateWithNameAndFileName(&self, name: HSTRING, fileName: HSTRING, out: *mut *mut BackgroundTransferContentPart) -> HRESULT
}}
impl IBackgroundTransferContentPartFactory {
    #[inline] pub fn create_with_name(&self, name: &HStringArg) -> Result<ComPtr<BackgroundTransferContentPart>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithName)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_name_and_file_name(&self, name: &HStringArg, fileName: &HStringArg) -> Result<ComPtr<BackgroundTransferContentPart>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithNameAndFileName)(self as *const _ as *mut _, name.get(), fileName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum BackgroundTransferCostPolicy: i32 {
    Default = 0, UnrestrictedOnly = 1, Always = 2,
}}
RT_CLASS!{static class BackgroundTransferError}
impl RtActivatable<IBackgroundTransferErrorStaticMethods> for BackgroundTransferError {}
impl BackgroundTransferError {
    #[cfg(feature="windows-web")] #[inline] pub fn get_status(hresult: i32) -> Result<super::super::web::WebErrorStatus> {
        <Self as RtActivatable<IBackgroundTransferErrorStaticMethods>>::get_activation_factory().get_status(hresult)
    }
}
DEFINE_CLSID!(BackgroundTransferError(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,69,114,114,111,114,0]) [CLSID_BackgroundTransferError]);
DEFINE_IID!(IID_IBackgroundTransferErrorStaticMethods, 2865969924, 4498, 19444, 139, 104, 57, 197, 173, 210, 68, 226);
RT_INTERFACE!{static interface IBackgroundTransferErrorStaticMethods(IBackgroundTransferErrorStaticMethodsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferErrorStaticMethods] {
    #[cfg(feature="windows-web")] fn GetStatus(&self, hresult: i32, out: *mut super::super::web::WebErrorStatus) -> HRESULT
}}
impl IBackgroundTransferErrorStaticMethods {
    #[cfg(feature="windows-web")] #[inline] pub fn get_status(&self, hresult: i32) -> Result<super::super::web::WebErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_STRUCT! { struct BackgroundTransferFileRange {
    Offset: u64, Length: u64,
}}
DEFINE_IID!(IID_IBackgroundTransferGroup, 3636716516, 25689, 17728, 133, 235, 170, 161, 200, 144, 54, 119);
RT_INTERFACE!{interface IBackgroundTransferGroup(IBackgroundTransferGroupVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferGroup] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TransferBehavior(&self, out: *mut BackgroundTransferBehavior) -> HRESULT,
    fn put_TransferBehavior(&self, value: BackgroundTransferBehavior) -> HRESULT
}}
impl IBackgroundTransferGroup {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_transfer_behavior(&self) -> Result<BackgroundTransferBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TransferBehavior)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_transfer_behavior(&self, value: BackgroundTransferBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TransferBehavior)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundTransferGroup: IBackgroundTransferGroup}
impl RtActivatable<IBackgroundTransferGroupStatics> for BackgroundTransferGroup {}
impl BackgroundTransferGroup {
    #[inline] pub fn create_group(name: &HStringArg) -> Result<Option<ComPtr<BackgroundTransferGroup>>> {
        <Self as RtActivatable<IBackgroundTransferGroupStatics>>::get_activation_factory().create_group(name)
    }
}
DEFINE_CLSID!(BackgroundTransferGroup(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,71,114,111,117,112,0]) [CLSID_BackgroundTransferGroup]);
DEFINE_IID!(IID_IBackgroundTransferGroupStatics, 49041586, 32024, 18779, 170, 34, 50, 169, 125, 69, 211, 226);
RT_INTERFACE!{static interface IBackgroundTransferGroupStatics(IBackgroundTransferGroupStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferGroupStatics] {
    fn CreateGroup(&self, name: HSTRING, out: *mut *mut BackgroundTransferGroup) -> HRESULT
}}
impl IBackgroundTransferGroupStatics {
    #[inline] pub fn create_group(&self, name: &HStringArg) -> Result<Option<ComPtr<BackgroundTransferGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateGroup)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundTransferOperation, 3738200134, 37066, 17659, 143, 177, 18, 65, 84, 192, 213, 57);
RT_INTERFACE!{interface IBackgroundTransferOperation(IBackgroundTransferOperationVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferOperation] {
    fn get_Guid(&self, out: *mut Guid) -> HRESULT,
    fn get_RequestedUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_Method(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Group(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CostPolicy(&self, out: *mut BackgroundTransferCostPolicy) -> HRESULT,
    fn put_CostPolicy(&self, value: BackgroundTransferCostPolicy) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetResultStreamAt(&self, position: u64, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT,
    fn GetResponseInformation(&self, out: *mut *mut ResponseInformation) -> HRESULT
}}
impl IBackgroundTransferOperation {
    #[inline] pub fn get_guid(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Guid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_requested_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestedUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_method(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Method)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_group(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Group)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cost_policy(&self) -> Result<BackgroundTransferCostPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CostPolicy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_cost_policy(&self, value: BackgroundTransferCostPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_CostPolicy)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_result_stream_at(&self, position: u64) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetResultStreamAt)(self as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_response_information(&self) -> Result<Option<ComPtr<ResponseInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetResponseInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundTransferOperationPriority, 75842343, 21076, 19258, 145, 94, 10, 164, 146, 117, 192, 249);
RT_INTERFACE!{interface IBackgroundTransferOperationPriority(IBackgroundTransferOperationPriorityVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferOperationPriority] {
    fn get_Priority(&self, out: *mut BackgroundTransferPriority) -> HRESULT,
    fn put_Priority(&self, value: BackgroundTransferPriority) -> HRESULT
}}
impl IBackgroundTransferOperationPriority {
    #[inline] pub fn get_priority(&self) -> Result<BackgroundTransferPriority> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Priority)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_priority(&self, value: BackgroundTransferPriority) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Priority)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum BackgroundTransferPriority: i32 {
    Default = 0, High = 1, Low = 2,
}}
DEFINE_IID!(IID_IBackgroundTransferRangesDownloadedEventArgs, 1052537939, 48968, 19080, 146, 72, 176, 193, 101, 24, 79, 92);
RT_INTERFACE!{interface IBackgroundTransferRangesDownloadedEventArgs(IBackgroundTransferRangesDownloadedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundTransferRangesDownloadedEventArgs] {
    fn get_WasDownloadRestarted(&self, out: *mut bool) -> HRESULT,
    fn get_AddedRanges(&self, out: *mut *mut foundation::collections::IVector<BackgroundTransferFileRange>) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IBackgroundTransferRangesDownloadedEventArgs {
    #[inline] pub fn get_was_download_restarted(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WasDownloadRestarted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_added_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<BackgroundTransferFileRange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AddedRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundTransferRangesDownloadedEventArgs: IBackgroundTransferRangesDownloadedEventArgs}
RT_ENUM! { enum BackgroundTransferStatus: i32 {
    Idle = 0, Running = 1, PausedByApplication = 2, PausedCostedNetwork = 3, PausedNoNetwork = 4, Completed = 5, Canceled = 6, Error = 7, PausedRecoverableWebErrorStatus = 8, PausedSystemPolicy = 32,
}}
DEFINE_IID!(IID_IBackgroundUploader, 3314928046, 52909, 18011, 136, 1, 197, 90, 201, 10, 1, 206);
RT_INTERFACE!{interface IBackgroundUploader(IBackgroundUploaderVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploader] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateUpload(&self, uri: *mut foundation::Uri, sourceFile: *mut super::super::storage::IStorageFile, out: *mut *mut UploadOperation) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateUploadFromStreamAsync(&self, uri: *mut foundation::Uri, sourceStream: *mut super::super::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncOperation<UploadOperation>) -> HRESULT,
    fn CreateUploadWithFormDataAndAutoBoundaryAsync(&self, uri: *mut foundation::Uri, parts: *mut foundation::collections::IIterable<BackgroundTransferContentPart>, out: *mut *mut foundation::IAsyncOperation<UploadOperation>) -> HRESULT,
    fn CreateUploadWithSubTypeAsync(&self, uri: *mut foundation::Uri, parts: *mut foundation::collections::IIterable<BackgroundTransferContentPart>, subType: HSTRING, out: *mut *mut foundation::IAsyncOperation<UploadOperation>) -> HRESULT,
    fn CreateUploadWithSubTypeAndBoundaryAsync(&self, uri: *mut foundation::Uri, parts: *mut foundation::collections::IIterable<BackgroundTransferContentPart>, subType: HSTRING, boundary: HSTRING, out: *mut *mut foundation::IAsyncOperation<UploadOperation>) -> HRESULT
}}
impl IBackgroundUploader {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_upload(&self, uri: &foundation::Uri, sourceFile: &super::super::storage::IStorageFile) -> Result<Option<ComPtr<UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateUpload)(self as *const _ as *mut _, uri as *const _ as *mut _, sourceFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_upload_from_stream_async(&self, uri: &foundation::Uri, sourceStream: &super::super::storage::streams::IInputStream) -> Result<ComPtr<foundation::IAsyncOperation<UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateUploadFromStreamAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, sourceStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_upload_with_form_data_and_auto_boundary_async(&self, uri: &foundation::Uri, parts: &foundation::collections::IIterable<BackgroundTransferContentPart>) -> Result<ComPtr<foundation::IAsyncOperation<UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateUploadWithFormDataAndAutoBoundaryAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, parts as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_upload_with_sub_type_async(&self, uri: &foundation::Uri, parts: &foundation::collections::IIterable<BackgroundTransferContentPart>, subType: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateUploadWithSubTypeAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, parts as *const _ as *mut _, subType.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_upload_with_sub_type_and_boundary_async(&self, uri: &foundation::Uri, parts: &foundation::collections::IIterable<BackgroundTransferContentPart>, subType: &HStringArg, boundary: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateUploadWithSubTypeAndBoundaryAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, parts as *const _ as *mut _, subType.get(), boundary.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BackgroundUploader: IBackgroundUploader}
impl RtActivatable<IBackgroundUploaderFactory> for BackgroundUploader {}
impl RtActivatable<IBackgroundUploaderStaticMethods> for BackgroundUploader {}
impl RtActivatable<IBackgroundUploaderStaticMethods2> for BackgroundUploader {}
impl RtActivatable<IBackgroundUploaderUserConsent> for BackgroundUploader {}
impl RtActivatable<IActivationFactory> for BackgroundUploader {}
impl BackgroundUploader {
    #[inline] pub fn create_with_completion_group(completionGroup: &BackgroundTransferCompletionGroup) -> Result<ComPtr<BackgroundUploader>> {
        <Self as RtActivatable<IBackgroundUploaderFactory>>::get_activation_factory().create_with_completion_group(completionGroup)
    }
    #[inline] pub fn get_current_uploads_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> {
        <Self as RtActivatable<IBackgroundUploaderStaticMethods>>::get_activation_factory().get_current_uploads_async()
    }
    #[inline] pub fn get_current_uploads_for_group_async(group: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> {
        <Self as RtActivatable<IBackgroundUploaderStaticMethods>>::get_activation_factory().get_current_uploads_for_group_async(group)
    }
    #[inline] pub fn get_current_uploads_for_transfer_group_async(group: &BackgroundTransferGroup) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> {
        <Self as RtActivatable<IBackgroundUploaderStaticMethods2>>::get_activation_factory().get_current_uploads_for_transfer_group_async(group)
    }
    #[inline] pub fn request_unconstrained_uploads_async(operations: &foundation::collections::IIterable<UploadOperation>) -> Result<ComPtr<foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>> {
        <Self as RtActivatable<IBackgroundUploaderUserConsent>>::get_activation_factory().request_unconstrained_uploads_async(operations)
    }
}
DEFINE_CLSID!(BackgroundUploader(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,66,97,99,107,103,114,111,117,110,100,85,112,108,111,97,100,101,114,0]) [CLSID_BackgroundUploader]);
DEFINE_IID!(IID_IBackgroundUploader2, 2382762702, 3124, 17507, 128, 127, 25, 138, 27, 139, 212, 173);
RT_INTERFACE!{interface IBackgroundUploader2(IBackgroundUploader2Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploader2] {
    fn get_TransferGroup(&self, out: *mut *mut BackgroundTransferGroup) -> HRESULT,
    fn put_TransferGroup(&self, value: *mut BackgroundTransferGroup) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_SuccessToastNotification(&self, out: *mut *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_SuccessToastNotification(&self, value: *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_FailureToastNotification(&self, out: *mut *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_FailureToastNotification(&self, value: *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_SuccessTileNotification(&self, out: *mut *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_SuccessTileNotification(&self, value: *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_FailureTileNotification(&self, out: *mut *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_FailureTileNotification(&self, value: *mut super::super::ui::notifications::TileNotification) -> HRESULT
}}
impl IBackgroundUploader2 {
    #[inline] pub fn get_transfer_group(&self) -> Result<Option<ComPtr<BackgroundTransferGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_transfer_group(&self, value: &BackgroundTransferGroup) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TransferGroup)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_success_toast_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::ToastNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuccessToastNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_success_toast_notification(&self, value: &super::super::ui::notifications::ToastNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SuccessToastNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_failure_toast_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::ToastNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FailureToastNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_failure_toast_notification(&self, value: &super::super::ui::notifications::ToastNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FailureToastNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_success_tile_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::TileNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuccessTileNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_success_tile_notification(&self, value: &super::super::ui::notifications::TileNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SuccessTileNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_failure_tile_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::TileNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FailureTileNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_failure_tile_notification(&self, value: &super::super::ui::notifications::TileNotification) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FailureTileNotification)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundUploader3, 3109983289, 23536, 19258, 140, 71, 44, 97, 153, 168, 84, 185);
RT_INTERFACE!{interface IBackgroundUploader3(IBackgroundUploader3Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploader3] {
    fn get_CompletionGroup(&self, out: *mut *mut BackgroundTransferCompletionGroup) -> HRESULT
}}
impl IBackgroundUploader3 {
    #[inline] pub fn get_completion_group(&self) -> Result<Option<ComPtr<BackgroundTransferCompletionGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CompletionGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundUploaderFactory, 1935803335, 4327, 18592, 172, 60, 26, 199, 16, 149, 236, 87);
RT_INTERFACE!{static interface IBackgroundUploaderFactory(IBackgroundUploaderFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploaderFactory] {
    fn CreateWithCompletionGroup(&self, completionGroup: *mut BackgroundTransferCompletionGroup, out: *mut *mut BackgroundUploader) -> HRESULT
}}
impl IBackgroundUploaderFactory {
    #[inline] pub fn create_with_completion_group(&self, completionGroup: &BackgroundTransferCompletionGroup) -> Result<ComPtr<BackgroundUploader>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithCompletionGroup)(self as *const _ as *mut _, completionGroup as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundUploaderStaticMethods, 4068957435, 39685, 18241, 145, 33, 116, 10, 131, 226, 71, 223);
RT_INTERFACE!{static interface IBackgroundUploaderStaticMethods(IBackgroundUploaderStaticMethodsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploaderStaticMethods] {
    fn GetCurrentUploadsAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>) -> HRESULT,
    fn GetCurrentUploadsForGroupAsync(&self, group: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>) -> HRESULT
}}
impl IBackgroundUploaderStaticMethods {
    #[inline] pub fn get_current_uploads_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentUploadsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_uploads_for_group_async(&self, group: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentUploadsForGroupAsync)(self as *const _ as *mut _, group.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundUploaderStaticMethods2, 3910773858, 59912, 17136, 162, 172, 7, 228, 103, 84, 144, 128);
RT_INTERFACE!{static interface IBackgroundUploaderStaticMethods2(IBackgroundUploaderStaticMethods2Vtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploaderStaticMethods2] {
    fn GetCurrentUploadsForTransferGroupAsync(&self, group: *mut BackgroundTransferGroup, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>) -> HRESULT
}}
impl IBackgroundUploaderStaticMethods2 {
    #[inline] pub fn get_current_uploads_for_transfer_group_async(&self, group: &BackgroundTransferGroup) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<UploadOperation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentUploadsForTransferGroupAsync)(self as *const _ as *mut _, group as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBackgroundUploaderUserConsent, 1001620683, 1888, 17949, 144, 127, 81, 56, 248, 77, 68, 193);
RT_INTERFACE!{static interface IBackgroundUploaderUserConsent(IBackgroundUploaderUserConsentVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundUploaderUserConsent] {
    fn RequestUnconstrainedUploadsAsync(&self, operations: *mut foundation::collections::IIterable<UploadOperation>, out: *mut *mut foundation::IAsyncOperation<UnconstrainedTransferRequestResult>) -> HRESULT
}}
impl IBackgroundUploaderUserConsent {
    #[inline] pub fn request_unconstrained_uploads_async(&self, operations: &foundation::collections::IIterable<UploadOperation>) -> Result<ComPtr<foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestUnconstrainedUploadsAsync)(self as *const _ as *mut _, operations as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct BackgroundUploadProgress {
    BytesReceived: u64, BytesSent: u64, TotalBytesToReceive: u64, TotalBytesToSend: u64, Status: BackgroundTransferStatus, HasResponseChanged: bool, HasRestarted: bool,
}}
DEFINE_IID!(IID_IContentPrefetcher, 2832660308, 32193, 19673, 136, 16, 42, 106, 169, 65, 126, 17);
RT_INTERFACE!{static interface IContentPrefetcher(IContentPrefetcherVtbl): IInspectable(IInspectableVtbl) [IID_IContentPrefetcher] {
    fn get_ContentUris(&self, out: *mut *mut foundation::collections::IVector<foundation::Uri>) -> HRESULT,
    fn put_IndirectContentUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_IndirectContentUri(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl IContentPrefetcher {
    #[inline] pub fn get_content_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVector<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentUris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_indirect_content_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IndirectContentUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_indirect_content_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IndirectContentUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class ContentPrefetcher}
impl RtActivatable<IContentPrefetcher> for ContentPrefetcher {}
impl RtActivatable<IContentPrefetcherTime> for ContentPrefetcher {}
impl ContentPrefetcher {
    #[inline] pub fn get_content_uris() -> Result<Option<ComPtr<foundation::collections::IVector<foundation::Uri>>>> {
        <Self as RtActivatable<IContentPrefetcher>>::get_activation_factory().get_content_uris()
    }
    #[inline] pub fn set_indirect_content_uri(value: &foundation::Uri) -> Result<()> {
        <Self as RtActivatable<IContentPrefetcher>>::get_activation_factory().set_indirect_content_uri(value)
    }
    #[inline] pub fn get_indirect_content_uri() -> Result<Option<ComPtr<foundation::Uri>>> {
        <Self as RtActivatable<IContentPrefetcher>>::get_activation_factory().get_indirect_content_uri()
    }
    #[inline] pub fn get_last_successful_prefetch_time() -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> {
        <Self as RtActivatable<IContentPrefetcherTime>>::get_activation_factory().get_last_successful_prefetch_time()
    }
}
DEFINE_CLSID!(ContentPrefetcher(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,66,97,99,107,103,114,111,117,110,100,84,114,97,110,115,102,101,114,46,67,111,110,116,101,110,116,80,114,101,102,101,116,99,104,101,114,0]) [CLSID_ContentPrefetcher]);
DEFINE_IID!(IID_IContentPrefetcherTime, 3814849800, 4906, 20446, 167, 204, 252, 176, 230, 101, 35, 175);
RT_INTERFACE!{static interface IContentPrefetcherTime(IContentPrefetcherTimeVtbl): IInspectable(IInspectableVtbl) [IID_IContentPrefetcherTime] {
    fn get_LastSuccessfulPrefetchTime(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT
}}
impl IContentPrefetcherTime {
    #[inline] pub fn get_last_successful_prefetch_time(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastSuccessfulPrefetchTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDownloadOperation, 3179801520, 22292, 19977, 186, 104, 190, 247, 57, 3, 176, 215);
RT_INTERFACE!{interface IDownloadOperation(IDownloadOperationVtbl): IInspectable(IInspectableVtbl) [IID_IDownloadOperation] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ResultFile(&self, out: *mut *mut super::super::storage::IStorageFile) -> HRESULT,
    fn get_Progress(&self, out: *mut BackgroundDownloadProgress) -> HRESULT,
    fn StartAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>) -> HRESULT,
    fn AttachAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>) -> HRESULT,
    fn Pause(&self) -> HRESULT,
    fn Resume(&self) -> HRESULT
}}
impl IDownloadOperation {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_result_file(&self) -> Result<Option<ComPtr<super::super::storage::IStorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResultFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_progress(&self) -> Result<BackgroundDownloadProgress> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Progress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn attach_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pause(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Pause)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn resume(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Resume)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DownloadOperation: IDownloadOperation}
DEFINE_IID!(IID_IDownloadOperation2, 2748116288, 36764, 17235, 156, 212, 41, 13, 238, 56, 124, 56);
RT_INTERFACE!{interface IDownloadOperation2(IDownloadOperation2Vtbl): IInspectable(IInspectableVtbl) [IID_IDownloadOperation2] {
    fn get_TransferGroup(&self, out: *mut *mut BackgroundTransferGroup) -> HRESULT
}}
impl IDownloadOperation2 {
    #[inline] pub fn get_transfer_group(&self) -> Result<Option<ComPtr<BackgroundTransferGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDownloadOperation3, 1344746780, 32094, 19164, 184, 211, 223, 92, 96, 49, 185, 204);
RT_INTERFACE!{interface IDownloadOperation3(IDownloadOperation3Vtbl): IInspectable(IInspectableVtbl) [IID_IDownloadOperation3] {
    fn get_IsRandomAccessRequired(&self, out: *mut bool) -> HRESULT,
    fn put_IsRandomAccessRequired(&self, value: bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetResultRandomAccessStreamReference(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStreamReference) -> HRESULT,
    fn GetDownloadedRanges(&self, out: *mut *mut foundation::collections::IVector<BackgroundTransferFileRange>) -> HRESULT,
    fn add_RangesDownloaded(&self, eventHandler: *mut foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RangesDownloaded(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn put_RequestedUri(&self, value: *mut foundation::Uri) -> HRESULT,
    #[cfg(feature="windows-web")] fn get_RecoverableWebErrorStatuses(&self, out: *mut *mut foundation::collections::IVector<super::super::web::WebErrorStatus>) -> HRESULT,
    #[cfg(feature="windows-web")] fn get_CurrentWebErrorStatus(&self, out: *mut *mut foundation::IReference<super::super::web::WebErrorStatus>) -> HRESULT
}}
impl IDownloadOperation3 {
    #[inline] pub fn get_is_random_access_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRandomAccessRequired)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_random_access_required(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsRandomAccessRequired)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_result_random_access_stream_reference(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStreamReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetResultRandomAccessStreamReference)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_downloaded_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<BackgroundTransferFileRange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDownloadedRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_ranges_downloaded(&self, eventHandler: &foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RangesDownloaded)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_ranges_downloaded(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RangesDownloaded)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_requested_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RequestedUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn get_recoverable_web_error_statuses(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::super::web::WebErrorStatus>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RecoverableWebErrorStatuses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn get_current_web_error_status(&self) -> Result<Option<ComPtr<foundation::IReference<super::super::web::WebErrorStatus>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentWebErrorStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDownloadOperation4, 215658228, 36079, 16458, 150, 109, 240, 88, 64, 11, 237, 128);
RT_INTERFACE!{interface IDownloadOperation4(IDownloadOperation4Vtbl): IInspectable(IInspectableVtbl) [IID_IDownloadOperation4] {
    fn MakeCurrentInTransferGroup(&self) -> HRESULT
}}
impl IDownloadOperation4 {
    #[inline] pub fn make_current_in_transfer_group(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).MakeCurrentInTransferGroup)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IResponseInformation, 4173044242, 63251, 18322, 139, 104, 217, 210, 151, 249, 29, 46);
RT_INTERFACE!{interface IResponseInformation(IResponseInformationVtbl): IInspectable(IInspectableVtbl) [IID_IResponseInformation] {
    fn get_IsResumable(&self, out: *mut bool) -> HRESULT,
    fn get_ActualUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_StatusCode(&self, out: *mut u32) -> HRESULT,
    fn get_Headers(&self, out: *mut *mut foundation::collections::IMapView<HString, HString>) -> HRESULT
}}
impl IResponseInformation {
    #[inline] pub fn get_is_resumable(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsResumable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_actual_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ActualUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_headers(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ResponseInformation: IResponseInformation}
DEFINE_IID!(IID_IUnconstrainedTransferRequestResult, 1277474847, 55620, 16658, 169, 142, 106, 105, 82, 43, 126, 187);
RT_INTERFACE!{interface IUnconstrainedTransferRequestResult(IUnconstrainedTransferRequestResultVtbl): IInspectable(IInspectableVtbl) [IID_IUnconstrainedTransferRequestResult] {
    fn get_IsUnconstrained(&self, out: *mut bool) -> HRESULT
}}
impl IUnconstrainedTransferRequestResult {
    #[inline] pub fn get_is_unconstrained(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsUnconstrained)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class UnconstrainedTransferRequestResult: IUnconstrainedTransferRequestResult}
DEFINE_IID!(IID_IUploadOperation, 1045832928, 29577, 17228, 139, 53, 66, 127, 211, 107, 189, 174);
RT_INTERFACE!{interface IUploadOperation(IUploadOperationVtbl): IInspectable(IInspectableVtbl) [IID_IUploadOperation] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_SourceFile(&self, out: *mut *mut super::super::storage::IStorageFile) -> HRESULT,
    fn get_Progress(&self, out: *mut BackgroundUploadProgress) -> HRESULT,
    fn StartAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>) -> HRESULT,
    fn AttachAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>) -> HRESULT
}}
impl IUploadOperation {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_source_file(&self) -> Result<Option<ComPtr<super::super::storage::IStorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_progress(&self) -> Result<BackgroundUploadProgress> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Progress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn attach_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UploadOperation: IUploadOperation}
DEFINE_IID!(IID_IUploadOperation2, 1432455666, 10100, 19958, 159, 165, 32, 159, 43, 251, 18, 247);
RT_INTERFACE!{interface IUploadOperation2(IUploadOperation2Vtbl): IInspectable(IInspectableVtbl) [IID_IUploadOperation2] {
    fn get_TransferGroup(&self, out: *mut *mut BackgroundTransferGroup) -> HRESULT
}}
impl IUploadOperation2 {
    #[inline] pub fn get_transfer_group(&self) -> Result<Option<ComPtr<BackgroundTransferGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransferGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUploadOperation3, 1120480419, 56889, 17734, 188, 98, 55, 116, 180, 41, 77, 227);
RT_INTERFACE!{interface IUploadOperation3(IUploadOperation3Vtbl): IInspectable(IInspectableVtbl) [IID_IUploadOperation3] {
    fn MakeCurrentInTransferGroup(&self) -> HRESULT
}}
impl IUploadOperation3 {
    #[inline] pub fn make_current_in_transfer_group(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).MakeCurrentInTransferGroup)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Networking.BackgroundTransfer
pub mod connectivity { // Windows.Networking.Connectivity
use crate::prelude::*;
DEFINE_IID!(IID_IAttributedNetworkUsage, 4150898745, 60578, 17899, 173, 225, 176, 54, 139, 117, 108, 73);
RT_INTERFACE!{interface IAttributedNetworkUsage(IAttributedNetworkUsageVtbl): IInspectable(IInspectableVtbl) [IID_IAttributedNetworkUsage] {
    fn get_BytesSent(&self, out: *mut u64) -> HRESULT,
    fn get_BytesReceived(&self, out: *mut u64) -> HRESULT,
    fn get_AttributionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AttributionName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_AttributionThumbnail(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStreamReference) -> HRESULT
}}
impl IAttributedNetworkUsage {
    #[inline] pub fn get_bytes_sent(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesSent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_received(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReceived)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_attribution_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttributionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attribution_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttributionName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_attribution_thumbnail(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStreamReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AttributionThumbnail)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AttributedNetworkUsage: IAttributedNetworkUsage}
RT_ENUM! { enum CellularApnAuthenticationType: i32 {
    None = 0, Pap = 1, Chap = 2, Mschapv2 = 3,
}}
DEFINE_IID!(IID_ICellularApnContext, 1873095156, 61437, 17730, 154, 178, 112, 91, 191, 148, 148, 58);
RT_INTERFACE!{interface ICellularApnContext(ICellularApnContextVtbl): IInspectable(IInspectableVtbl) [IID_ICellularApnContext] {
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ProviderId(&self, value: HSTRING) -> HRESULT,
    fn get_AccessPointName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_AccessPointName(&self, value: HSTRING) -> HRESULT,
    fn get_UserName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserName(&self, value: HSTRING) -> HRESULT,
    fn get_Password(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Password(&self, value: HSTRING) -> HRESULT,
    fn get_IsCompressionEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsCompressionEnabled(&self, value: bool) -> HRESULT,
    fn get_AuthenticationType(&self, out: *mut CellularApnAuthenticationType) -> HRESULT,
    fn put_AuthenticationType(&self, value: CellularApnAuthenticationType) -> HRESULT
}}
impl ICellularApnContext {
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_provider_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProviderId)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_access_point_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccessPointName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_access_point_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AccessPointName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_user_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UserName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_user_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_UserName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_password(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Password)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_password(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Password)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_compression_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsCompressionEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_compression_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsCompressionEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_type(&self) -> Result<CellularApnAuthenticationType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_authentication_type(&self, value: CellularApnAuthenticationType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AuthenticationType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CellularApnContext: ICellularApnContext}
impl RtActivatable<IActivationFactory> for CellularApnContext {}
DEFINE_CLSID!(CellularApnContext(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,67,111,110,110,101,99,116,105,118,105,116,121,46,67,101,108,108,117,108,97,114,65,112,110,67,111,110,116,101,120,116,0]) [CLSID_CellularApnContext]);
DEFINE_IID!(IID_ICellularApnContext2, 1991306010, 44105, 17232, 177, 229, 220, 71, 99, 188, 105, 199);
RT_INTERFACE!{interface ICellularApnContext2(ICellularApnContext2Vtbl): IInspectable(IInspectableVtbl) [IID_ICellularApnContext2] {
    fn get_ProfileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ProfileName(&self, value: HSTRING) -> HRESULT
}}
impl ICellularApnContext2 {
    #[inline] pub fn get_profile_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProfileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_profile_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProfileName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionCost, 3134707753, 13334, 19216, 162, 2, 186, 192, 176, 117, 189, 174);
RT_INTERFACE!{interface IConnectionCost(IConnectionCostVtbl): IInspectable(IInspectableVtbl) [IID_IConnectionCost] {
    fn get_NetworkCostType(&self, out: *mut NetworkCostType) -> HRESULT,
    fn get_Roaming(&self, out: *mut bool) -> HRESULT,
    fn get_OverDataLimit(&self, out: *mut bool) -> HRESULT,
    fn get_ApproachingDataLimit(&self, out: *mut bool) -> HRESULT
}}
impl IConnectionCost {
    #[inline] pub fn get_network_cost_type(&self) -> Result<NetworkCostType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkCostType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Roaming)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_over_data_limit(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OverDataLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_approaching_data_limit(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ApproachingDataLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectionCost: IConnectionCost}
DEFINE_IID!(IID_IConnectionCost2, 2383493637, 57865, 17737, 187, 37, 94, 13, 182, 145, 203, 5);
RT_INTERFACE!{interface IConnectionCost2(IConnectionCost2Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionCost2] {
    fn get_BackgroundDataUsageRestricted(&self, out: *mut bool) -> HRESULT
}}
impl IConnectionCost2 {
    #[inline] pub fn get_background_data_usage_restricted(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BackgroundDataUsageRestricted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionProfile, 1908020284, 22926, 18896, 132, 235, 143, 235, 174, 220, 193, 149);
RT_INTERFACE!{interface IConnectionProfile(IConnectionProfileVtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfile] {
    fn get_ProfileName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetNetworkConnectivityLevel(&self, out: *mut NetworkConnectivityLevel) -> HRESULT,
    fn GetNetworkNames(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn GetConnectionCost(&self, out: *mut *mut ConnectionCost) -> HRESULT,
    fn GetDataPlanStatus(&self, out: *mut *mut DataPlanStatus) -> HRESULT,
    fn get_NetworkAdapter(&self, out: *mut *mut NetworkAdapter) -> HRESULT,
    fn GetLocalUsage(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, out: *mut *mut DataUsage) -> HRESULT,
    fn GetLocalUsagePerRoamingStates(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: RoamingStates, out: *mut *mut DataUsage) -> HRESULT,
    fn get_NetworkSecuritySettings(&self, out: *mut *mut NetworkSecuritySettings) -> HRESULT
}}
impl IConnectionProfile {
    #[inline] pub fn get_profile_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProfileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_connectivity_level(&self) -> Result<NetworkConnectivityLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNetworkConnectivityLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_names(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNetworkNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_cost(&self) -> Result<Option<ComPtr<ConnectionCost>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectionCost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_plan_status(&self) -> Result<Option<ComPtr<DataPlanStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDataPlanStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_adapter(&self) -> Result<Option<ComPtr<NetworkAdapter>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAdapter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_usage(&self, startTime: foundation::DateTime, endTime: foundation::DateTime) -> Result<Option<ComPtr<DataUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLocalUsage)(self as *const _ as *mut _, startTime, endTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_usage_per_roaming_states(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: RoamingStates) -> Result<Option<ComPtr<DataUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLocalUsagePerRoamingStates)(self as *const _ as *mut _, startTime, endTime, states, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_security_settings(&self) -> Result<Option<ComPtr<NetworkSecuritySettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkSecuritySettings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectionProfile: IConnectionProfile}
DEFINE_IID!(IID_IConnectionProfile2, 3791933765, 19615, 16396, 145, 80, 126, 199, 214, 226, 136, 138);
RT_INTERFACE!{interface IConnectionProfile2(IConnectionProfile2Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfile2] {
    fn get_IsWwanConnectionProfile(&self, out: *mut bool) -> HRESULT,
    fn get_IsWlanConnectionProfile(&self, out: *mut bool) -> HRESULT,
    fn get_WwanConnectionProfileDetails(&self, out: *mut *mut WwanConnectionProfileDetails) -> HRESULT,
    fn get_WlanConnectionProfileDetails(&self, out: *mut *mut WlanConnectionProfileDetails) -> HRESULT,
    fn get_ServiceProviderGuid(&self, out: *mut *mut foundation::IReference<Guid>) -> HRESULT,
    fn GetSignalBars(&self, out: *mut *mut foundation::IReference<u8>) -> HRESULT,
    fn GetDomainConnectivityLevel(&self, out: *mut DomainConnectivityLevel) -> HRESULT,
    fn GetNetworkUsageAsync(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<NetworkUsage>>) -> HRESULT,
    fn GetConnectivityIntervalsAsync(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<ConnectivityInterval>>) -> HRESULT
}}
impl IConnectionProfile2 {
    #[inline] pub fn get_is_wwan_connection_profile(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWwanConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_wlan_connection_profile(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWlanConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_wwan_connection_profile_details(&self) -> Result<Option<ComPtr<WwanConnectionProfileDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WwanConnectionProfileDetails)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_wlan_connection_profile_details(&self) -> Result<Option<ComPtr<WlanConnectionProfileDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WlanConnectionProfileDetails)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_service_provider_guid(&self) -> Result<Option<ComPtr<foundation::IReference<Guid>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceProviderGuid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signal_bars(&self) -> Result<Option<ComPtr<foundation::IReference<u8>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSignalBars)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_connectivity_level(&self) -> Result<DomainConnectivityLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetDomainConnectivityLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_usage_async(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, granularity: DataUsageGranularity, states: NetworkUsageStates) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<NetworkUsage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetNetworkUsageAsync)(self as *const _ as *mut _, startTime, endTime, granularity, states, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_connectivity_intervals_async(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<ConnectivityInterval>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectivityIntervalsAsync)(self as *const _ as *mut _, startTime, endTime, states, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionProfile3, 1468802344, 19673, 16737, 128, 69, 32, 28, 253, 91, 17, 92);
RT_INTERFACE!{interface IConnectionProfile3(IConnectionProfile3Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfile3] {
    fn GetAttributedNetworkUsageAsync(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<AttributedNetworkUsage>>) -> HRESULT
}}
impl IConnectionProfile3 {
    #[inline] pub fn get_attributed_network_usage_async(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<AttributedNetworkUsage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAttributedNetworkUsageAsync)(self as *const _ as *mut _, startTime, endTime, states, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionProfile4, 2049786573, 33248, 19174, 171, 237, 171, 156, 161, 62, 183, 20);
RT_INTERFACE!{interface IConnectionProfile4(IConnectionProfile4Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfile4] {
    fn GetProviderNetworkUsageAsync(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<ProviderNetworkUsage>>) -> HRESULT
}}
impl IConnectionProfile4 {
    #[inline] pub fn get_provider_network_usage_async(&self, startTime: foundation::DateTime, endTime: foundation::DateTime, states: NetworkUsageStates) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<ProviderNetworkUsage>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProviderNetworkUsageAsync)(self as *const _ as *mut _, startTime, endTime, states, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionProfile5, 2234916551, 40051, 19424, 143, 20, 87, 142, 236, 113, 238, 14);
RT_INTERFACE!{interface IConnectionProfile5(IConnectionProfile5Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfile5] {
    fn get_CanDelete(&self, out: *mut bool) -> HRESULT,
    fn TryDeleteAsync(&self, out: *mut *mut foundation::IAsyncOperation<ConnectionProfileDeleteStatus>) -> HRESULT
}}
impl IConnectionProfile5 {
    #[inline] pub fn get_can_delete(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanDelete)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_delete_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ConnectionProfileDeleteStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryDeleteAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ConnectionProfileDeleteStatus: i32 {
    Success = 0, DeniedByUser = 1, DeniedBySystem = 2, UnknownError = 3,
}}
DEFINE_IID!(IID_IConnectionProfileFilter, 541883592, 48429, 20109, 164, 179, 69, 94, 195, 55, 56, 138);
RT_INTERFACE!{interface IConnectionProfileFilter(IConnectionProfileFilterVtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfileFilter] {
    fn put_IsConnected(&self, value: bool) -> HRESULT,
    fn get_IsConnected(&self, out: *mut bool) -> HRESULT,
    fn put_IsWwanConnectionProfile(&self, value: bool) -> HRESULT,
    fn get_IsWwanConnectionProfile(&self, out: *mut bool) -> HRESULT,
    fn put_IsWlanConnectionProfile(&self, value: bool) -> HRESULT,
    fn get_IsWlanConnectionProfile(&self, out: *mut bool) -> HRESULT,
    fn put_NetworkCostType(&self, value: NetworkCostType) -> HRESULT,
    fn get_NetworkCostType(&self, out: *mut NetworkCostType) -> HRESULT,
    fn put_ServiceProviderGuid(&self, value: *mut foundation::IReference<Guid>) -> HRESULT,
    fn get_ServiceProviderGuid(&self, out: *mut *mut foundation::IReference<Guid>) -> HRESULT
}}
impl IConnectionProfileFilter {
    #[inline] pub fn set_is_connected(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsConnected)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_connected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsConnected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_wwan_connection_profile(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsWwanConnectionProfile)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_wwan_connection_profile(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWwanConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_wlan_connection_profile(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsWlanConnectionProfile)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_wlan_connection_profile(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWlanConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_network_cost_type(&self, value: NetworkCostType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NetworkCostType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_network_cost_type(&self) -> Result<NetworkCostType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkCostType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_service_provider_guid(&self, value: &foundation::IReference<Guid>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ServiceProviderGuid)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_service_provider_guid(&self) -> Result<Option<ComPtr<foundation::IReference<Guid>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceProviderGuid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectionProfileFilter: IConnectionProfileFilter}
impl RtActivatable<IActivationFactory> for ConnectionProfileFilter {}
DEFINE_CLSID!(ConnectionProfileFilter(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,67,111,110,110,101,99,116,105,118,105,116,121,46,67,111,110,110,101,99,116,105,111,110,80,114,111,102,105,108,101,70,105,108,116,101,114,0]) [CLSID_ConnectionProfileFilter]);
DEFINE_IID!(IID_IConnectionProfileFilter2, 3439759073, 50172, 20397, 157, 220, 89, 63, 170, 75, 120, 133);
RT_INTERFACE!{interface IConnectionProfileFilter2(IConnectionProfileFilter2Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfileFilter2] {
    fn put_IsRoaming(&self, value: *mut foundation::IReference<bool>) -> HRESULT,
    fn get_IsRoaming(&self, out: *mut *mut foundation::IReference<bool>) -> HRESULT,
    fn put_IsOverDataLimit(&self, value: *mut foundation::IReference<bool>) -> HRESULT,
    fn get_IsOverDataLimit(&self, out: *mut *mut foundation::IReference<bool>) -> HRESULT,
    fn put_IsBackgroundDataUsageRestricted(&self, value: *mut foundation::IReference<bool>) -> HRESULT,
    fn get_IsBackgroundDataUsageRestricted(&self, out: *mut *mut foundation::IReference<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_RawData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IConnectionProfileFilter2 {
    #[inline] pub fn set_is_roaming(&self, value: &foundation::IReference<bool>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsRoaming)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_roaming(&self) -> Result<Option<ComPtr<foundation::IReference<bool>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsRoaming)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_is_over_data_limit(&self, value: &foundation::IReference<bool>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsOverDataLimit)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_over_data_limit(&self) -> Result<Option<ComPtr<foundation::IReference<bool>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsOverDataLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_is_background_data_usage_restricted(&self, value: &foundation::IReference<bool>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsBackgroundDataUsageRestricted)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_background_data_usage_restricted(&self) -> Result<Option<ComPtr<foundation::IReference<bool>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsBackgroundDataUsageRestricted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_raw_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RawData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionProfileFilter3, 178915776, 20500, 17532, 136, 9, 174, 228, 203, 10, 249, 74);
RT_INTERFACE!{interface IConnectionProfileFilter3(IConnectionProfileFilter3Vtbl): IInspectable(IInspectableVtbl) [IID_IConnectionProfileFilter3] {
    fn put_PurposeGuid(&self, value: *mut foundation::IReference<Guid>) -> HRESULT,
    fn get_PurposeGuid(&self, out: *mut *mut foundation::IReference<Guid>) -> HRESULT
}}
impl IConnectionProfileFilter3 {
    #[inline] pub fn set_purpose_guid(&self, value: &foundation::IReference<Guid>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PurposeGuid)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_purpose_guid(&self) -> Result<Option<ComPtr<foundation::IReference<Guid>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PurposeGuid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IConnectionSession, 4287651148, 63547, 16816, 138, 12, 20, 98, 217, 197, 107, 115);
RT_INTERFACE!{interface IConnectionSession(IConnectionSessionVtbl): IInspectable(IInspectableVtbl) [IID_IConnectionSession] {
    fn get_ConnectionProfile(&self, out: *mut *mut ConnectionProfile) -> HRESULT
}}
impl IConnectionSession {
    #[inline] pub fn get_connection_profile(&self) -> Result<Option<ComPtr<ConnectionProfile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectionSession: IConnectionSession}
DEFINE_IID!(IID_IConnectivityInterval, 1336557567, 26438, 18468, 169, 100, 238, 216, 232, 127, 135, 9);
RT_INTERFACE!{interface IConnectivityInterval(IConnectivityIntervalVtbl): IInspectable(IInspectableVtbl) [IID_IConnectivityInterval] {
    fn get_StartTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ConnectionDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl IConnectivityInterval {
    #[inline] pub fn get_start_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StartTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ConnectionDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectivityInterval: IConnectivityInterval}
RT_CLASS!{static class ConnectivityManager}
impl RtActivatable<IConnectivityManagerStatics> for ConnectivityManager {}
impl ConnectivityManager {
    #[inline] pub fn acquire_connection_async(cellularApnContext: &CellularApnContext) -> Result<ComPtr<foundation::IAsyncOperation<ConnectionSession>>> {
        <Self as RtActivatable<IConnectivityManagerStatics>>::get_activation_factory().acquire_connection_async(cellularApnContext)
    }
    #[inline] pub fn add_http_route_policy(routePolicy: &RoutePolicy) -> Result<()> {
        <Self as RtActivatable<IConnectivityManagerStatics>>::get_activation_factory().add_http_route_policy(routePolicy)
    }
    #[inline] pub fn remove_http_route_policy(routePolicy: &RoutePolicy) -> Result<()> {
        <Self as RtActivatable<IConnectivityManagerStatics>>::get_activation_factory().remove_http_route_policy(routePolicy)
    }
}
DEFINE_CLSID!(ConnectivityManager(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,67,111,110,110,101,99,116,105,118,105,116,121,46,67,111,110,110,101,99,116,105,118,105,116,121,77,97,110,97,103,101,114,0]) [CLSID_ConnectivityManager]);
DEFINE_IID!(IID_IConnectivityManagerStatics, 1361106097, 20401, 18608, 175, 201, 66, 224, 9, 42, 129, 100);
RT_INTERFACE!{static interface IConnectivityManagerStatics(IConnectivityManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IConnectivityManagerStatics] {
    fn AcquireConnectionAsync(&self, cellularApnContext: *mut CellularApnContext, out: *mut *mut foundation::IAsyncOperation<ConnectionSession>) -> HRESULT,
    fn AddHttpRoutePolicy(&self, routePolicy: *mut RoutePolicy) -> HRESULT,
    fn RemoveHttpRoutePolicy(&self, routePolicy: *mut RoutePolicy) -> HRESULT
}}
impl IConnectivityManagerStatics {
    #[inline] pub fn acquire_connection_async(&self, cellularApnContext: &CellularApnContext) -> Result<ComPtr<foundation::IAsyncOperation<ConnectionSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AcquireConnectionAsync)(self as *const _ as *mut _, cellularApnContext as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_http_route_policy(&self, routePolicy: &RoutePolicy) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddHttpRoutePolicy)(self as *const _ as *mut _, routePolicy as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_http_route_policy(&self, routePolicy: &RoutePolicy) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RemoveHttpRoutePolicy)(self as *const _ as *mut _, routePolicy as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDataPlanStatus, 2541390732, 14469, 16627, 136, 81, 66, 205, 43, 213, 104, 187);
RT_INTERFACE!{interface IDataPlanStatus(IDataPlanStatusVtbl): IInspectable(IInspectableVtbl) [IID_IDataPlanStatus] {
    fn get_DataPlanUsage(&self, out: *mut *mut DataPlanUsage) -> HRESULT,
    fn get_DataLimitInMegabytes(&self, out: *mut *mut foundation::IReference<u32>) -> HRESULT,
    fn get_InboundBitsPerSecond(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn get_OutboundBitsPerSecond(&self, out: *mut *mut foundation::IReference<u64>) -> HRESULT,
    fn get_NextBillingCycle(&self, out: *mut *mut foundation::IReference<foundation::DateTime>) -> HRESULT,
    fn get_MaxTransferSizeInMegabytes(&self, out: *mut *mut foundation::IReference<u32>) -> HRESULT
}}
impl IDataPlanStatus {
    #[inline] pub fn get_data_plan_usage(&self) -> Result<Option<ComPtr<DataPlanUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataPlanUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_limit_in_megabytes(&self) -> Result<Option<ComPtr<foundation::IReference<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataLimitInMegabytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inbound_bits_per_second(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InboundBitsPerSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_bits_per_second(&self) -> Result<Option<ComPtr<foundation::IReference<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutboundBitsPerSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_billing_cycle(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::DateTime>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NextBillingCycle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_transfer_size_in_megabytes(&self) -> Result<Option<ComPtr<foundation::IReference<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaxTransferSizeInMegabytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataPlanStatus: IDataPlanStatus}
DEFINE_IID!(IID_IDataPlanUsage, 3105966381, 15172, 18431, 179, 97, 190, 89, 230, 158, 209, 176);
RT_INTERFACE!{interface IDataPlanUsage(IDataPlanUsageVtbl): IInspectable(IInspectableVtbl) [IID_IDataPlanUsage] {
    fn get_MegabytesUsed(&self, out: *mut u32) -> HRESULT,
    fn get_LastSyncTime(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl IDataPlanUsage {
    #[inline] pub fn get_megabytes_used(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MegabytesUsed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_sync_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastSyncTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class DataPlanUsage: IDataPlanUsage}
DEFINE_IID!(IID_IDataUsage, 3242401235, 45382, 19769, 185, 89, 12, 105, 176, 150, 197, 18);
RT_INTERFACE!{interface IDataUsage(IDataUsageVtbl): IInspectable(IInspectableVtbl) [IID_IDataUsage] {
    fn get_BytesSent(&self, out: *mut u64) -> HRESULT,
    fn get_BytesReceived(&self, out: *mut u64) -> HRESULT
}}
impl IDataUsage {
    #[inline] pub fn get_bytes_sent(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesSent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_received(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReceived)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class DataUsage: IDataUsage}
RT_ENUM! { enum DataUsageGranularity: i32 {
    PerMinute = 0, PerHour = 1, PerDay = 2, Total = 3,
}}
RT_ENUM! { enum DomainConnectivityLevel: i32 {
    None = 0, Unauthenticated = 1, Authenticated = 2,
}}
DEFINE_IID!(IID_IIPInformation, 3629204960, 5007, 18391, 155, 58, 54, 187, 72, 140, 239, 51);
RT_INTERFACE!{interface IIPInformation(IIPInformationVtbl): IInspectable(IInspectableVtbl) [IID_IIPInformation] {
    fn get_NetworkAdapter(&self, out: *mut *mut NetworkAdapter) -> HRESULT,
    fn get_PrefixLength(&self, out: *mut *mut foundation::IReference<u8>) -> HRESULT
}}
impl IIPInformation {
    #[inline] pub fn get_network_adapter(&self) -> Result<Option<ComPtr<NetworkAdapter>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAdapter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_prefix_length(&self) -> Result<Option<ComPtr<foundation::IReference<u8>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrefixLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILanIdentifier, 1219122090, 4360, 17734, 166, 203, 154, 116, 218, 75, 123, 160);
RT_INTERFACE!{interface ILanIdentifier(ILanIdentifierVtbl): IInspectable(IInspectableVtbl) [IID_ILanIdentifier] {
    fn get_InfrastructureId(&self, out: *mut *mut LanIdentifierData) -> HRESULT,
    fn get_PortId(&self, out: *mut *mut LanIdentifierData) -> HRESULT,
    fn get_NetworkAdapterId(&self, out: *mut Guid) -> HRESULT
}}
impl ILanIdentifier {
    #[inline] pub fn get_infrastructure_id(&self) -> Result<Option<ComPtr<LanIdentifierData>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InfrastructureId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_port_id(&self) -> Result<Option<ComPtr<LanIdentifierData>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PortId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_adapter_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkAdapterId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LanIdentifier: ILanIdentifier}
DEFINE_IID!(IID_ILanIdentifierData, 2806940611, 54841, 17854, 163, 106, 196, 228, 174, 175, 109, 155);
RT_INTERFACE!{interface ILanIdentifierData(ILanIdentifierDataVtbl): IInspectable(IInspectableVtbl) [IID_ILanIdentifierData] {
    fn get_Type(&self, out: *mut u32) -> HRESULT,
    fn get_Value(&self, out: *mut *mut foundation::collections::IVectorView<u8>) -> HRESULT
}}
impl ILanIdentifierData {
    #[inline] pub fn get_type(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u8>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LanIdentifierData: ILanIdentifierData}
DEFINE_IID!(IID_INetworkAdapter, 995372547, 21384, 18796, 168, 163, 175, 253, 57, 174, 194, 230);
RT_INTERFACE!{interface INetworkAdapter(INetworkAdapterVtbl): IInspectable(IInspectableVtbl) [IID_INetworkAdapter] {
    fn get_OutboundMaxBitsPerSecond(&self, out: *mut u64) -> HRESULT,
    fn get_InboundMaxBitsPerSecond(&self, out: *mut u64) -> HRESULT,
    fn get_IanaInterfaceType(&self, out: *mut u32) -> HRESULT,
    fn get_NetworkItem(&self, out: *mut *mut NetworkItem) -> HRESULT,
    fn get_NetworkAdapterId(&self, out: *mut Guid) -> HRESULT,
    fn GetConnectedProfileAsync(&self, out: *mut *mut foundation::IAsyncOperation<ConnectionProfile>) -> HRESULT
}}
impl INetworkAdapter {
    #[inline] pub fn get_outbound_max_bits_per_second(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundMaxBitsPerSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_inbound_max_bits_per_second(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InboundMaxBitsPerSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_iana_interface_type(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IanaInterfaceType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_item(&self) -> Result<Option<ComPtr<NetworkItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkItem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_adapter_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkAdapterId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_connected_profile_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ConnectionProfile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectedProfileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkAdapter: INetworkAdapter}
RT_ENUM! { enum NetworkAuthenticationType: i32 {
    None = 0, Unknown = 1, Open80211 = 2, SharedKey80211 = 3, Wpa = 4, WpaPsk = 5, WpaNone = 6, Rsna = 7, RsnaPsk = 8, Ihv = 9,
}}
RT_ENUM! { enum NetworkConnectivityLevel: i32 {
    None = 0, LocalAccess = 1, ConstrainedInternetAccess = 2, InternetAccess = 3,
}}
RT_ENUM! { enum NetworkCostType: i32 {
    Unknown = 0, Unrestricted = 1, Fixed = 2, Variable = 3,
}}
RT_ENUM! { enum NetworkEncryptionType: i32 {
    None = 0, Unknown = 1, Wep = 2, Wep40 = 3, Wep104 = 4, Tkip = 5, Ccmp = 6, WpaUseGroup = 7, RsnUseGroup = 8, Ihv = 9,
}}
RT_CLASS!{static class NetworkInformation}
impl RtActivatable<INetworkInformationStatics> for NetworkInformation {}
impl RtActivatable<INetworkInformationStatics2> for NetworkInformation {}
impl NetworkInformation {
    #[inline] pub fn get_connection_profiles() -> Result<Option<ComPtr<foundation::collections::IVectorView<ConnectionProfile>>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_connection_profiles()
    }
    #[inline] pub fn get_internet_connection_profile() -> Result<Option<ComPtr<ConnectionProfile>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_internet_connection_profile()
    }
    #[inline] pub fn get_lan_identifiers() -> Result<Option<ComPtr<foundation::collections::IVectorView<LanIdentifier>>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_lan_identifiers()
    }
    #[inline] pub fn get_host_names() -> Result<Option<ComPtr<foundation::collections::IVectorView<super::HostName>>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_host_names()
    }
    #[inline] pub fn get_proxy_configuration_async(uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<ProxyConfiguration>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_proxy_configuration_async(uri)
    }
    #[inline] pub fn get_sorted_endpoint_pairs(destinationList: &foundation::collections::IIterable<super::EndpointPair>, sortOptions: super::HostNameSortOptions) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::EndpointPair>>>> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().get_sorted_endpoint_pairs(destinationList, sortOptions)
    }
    #[inline] pub fn add_network_status_changed(networkStatusHandler: &NetworkStatusChangedEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().add_network_status_changed(networkStatusHandler)
    }
    #[inline] pub fn remove_network_status_changed(eventCookie: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<INetworkInformationStatics>>::get_activation_factory().remove_network_status_changed(eventCookie)
    }
    #[inline] pub fn find_connection_profiles_async(pProfileFilter: &ConnectionProfileFilter) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<ConnectionProfile>>>> {
        <Self as RtActivatable<INetworkInformationStatics2>>::get_activation_factory().find_connection_profiles_async(pProfileFilter)
    }
}
DEFINE_CLSID!(NetworkInformation(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,67,111,110,110,101,99,116,105,118,105,116,121,46,78,101,116,119,111,114,107,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_NetworkInformation]);
DEFINE_IID!(IID_INetworkInformationStatics, 1349843025, 38157, 16741, 156, 21, 54, 86, 25, 72, 30, 234);
RT_INTERFACE!{static interface INetworkInformationStatics(INetworkInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkInformationStatics] {
    fn GetConnectionProfiles(&self, out: *mut *mut foundation::collections::IVectorView<ConnectionProfile>) -> HRESULT,
    fn GetInternetConnectionProfile(&self, out: *mut *mut ConnectionProfile) -> HRESULT,
    fn GetLanIdentifiers(&self, out: *mut *mut foundation::collections::IVectorView<LanIdentifier>) -> HRESULT,
    fn GetHostNames(&self, out: *mut *mut foundation::collections::IVectorView<super::HostName>) -> HRESULT,
    fn GetProxyConfigurationAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<ProxyConfiguration>) -> HRESULT,
    fn GetSortedEndpointPairs(&self, destinationList: *mut foundation::collections::IIterable<super::EndpointPair>, sortOptions: super::HostNameSortOptions, out: *mut *mut foundation::collections::IVectorView<super::EndpointPair>) -> HRESULT,
    fn add_NetworkStatusChanged(&self, networkStatusHandler: *mut NetworkStatusChangedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_NetworkStatusChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl INetworkInformationStatics {
    #[inline] pub fn get_connection_profiles(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ConnectionProfile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectionProfiles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_internet_connection_profile(&self) -> Result<Option<ComPtr<ConnectionProfile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetInternetConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_lan_identifiers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<LanIdentifier>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLanIdentifiers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_host_names(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetHostNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_proxy_configuration_async(&self, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<ProxyConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProxyConfigurationAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sorted_endpoint_pairs(&self, destinationList: &foundation::collections::IIterable<super::EndpointPair>, sortOptions: super::HostNameSortOptions) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::EndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSortedEndpointPairs)(self as *const _ as *mut _, destinationList as *const _ as *mut _, sortOptions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_network_status_changed(&self, networkStatusHandler: &NetworkStatusChangedEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_NetworkStatusChanged)(self as *const _ as *mut _, networkStatusHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_network_status_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_NetworkStatusChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkInformationStatics2, 1167912212, 10290, 18870, 186, 110, 226, 101, 240, 71, 134, 168);
RT_INTERFACE!{static interface INetworkInformationStatics2(INetworkInformationStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_INetworkInformationStatics2] {
    fn FindConnectionProfilesAsync(&self, pProfileFilter: *mut ConnectionProfileFilter, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<ConnectionProfile>>) -> HRESULT
}}
impl INetworkInformationStatics2 {
    #[inline] pub fn find_connection_profiles_async(&self, pProfileFilter: &ConnectionProfileFilter) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<ConnectionProfile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindConnectionProfilesAsync)(self as *const _ as *mut _, pProfileFilter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkItem, 29117753, 62944, 17767, 162, 140, 66, 8, 12, 131, 27, 43);
RT_INTERFACE!{interface INetworkItem(INetworkItemVtbl): IInspectable(IInspectableVtbl) [IID_INetworkItem] {
    fn get_NetworkId(&self, out: *mut Guid) -> HRESULT,
    fn GetNetworkTypes(&self, out: *mut NetworkTypes) -> HRESULT
}}
impl INetworkItem {
    #[inline] pub fn get_network_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_types(&self) -> Result<NetworkTypes> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNetworkTypes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkItem: INetworkItem}
DEFINE_IID!(IID_INetworkSecuritySettings, 2090892941, 37243, 19295, 184, 77, 40, 247, 165, 172, 84, 2);
RT_INTERFACE!{interface INetworkSecuritySettings(INetworkSecuritySettingsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkSecuritySettings] {
    fn get_NetworkAuthenticationType(&self, out: *mut NetworkAuthenticationType) -> HRESULT,
    fn get_NetworkEncryptionType(&self, out: *mut NetworkEncryptionType) -> HRESULT
}}
impl INetworkSecuritySettings {
    #[inline] pub fn get_network_authentication_type(&self) -> Result<NetworkAuthenticationType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkAuthenticationType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_encryption_type(&self) -> Result<NetworkEncryptionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkEncryptionType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkSecuritySettings: INetworkSecuritySettings}
DEFINE_IID!(IID_INetworkStateChangeEventDetails, 520942387, 55206, 17629, 164, 233, 104, 124, 71, 107, 144, 61);
RT_INTERFACE!{interface INetworkStateChangeEventDetails(INetworkStateChangeEventDetailsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkStateChangeEventDetails] {
    fn get_HasNewInternetConnectionProfile(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewConnectionCost(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewNetworkConnectivityLevel(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewDomainConnectivityLevel(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewHostNameList(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewWwanRegistrationState(&self, out: *mut bool) -> HRESULT
}}
impl INetworkStateChangeEventDetails {
    #[inline] pub fn get_has_new_internet_connection_profile(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewInternetConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_connection_cost(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewConnectionCost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_network_connectivity_level(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewNetworkConnectivityLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_domain_connectivity_level(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewDomainConnectivityLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_host_name_list(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewHostNameList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_wwan_registration_state(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewWwanRegistrationState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkStateChangeEventDetails: INetworkStateChangeEventDetails}
DEFINE_IID!(IID_INetworkStateChangeEventDetails2, 3594764520, 12499, 20330, 173, 71, 106, 24, 115, 206, 179, 193);
RT_INTERFACE!{interface INetworkStateChangeEventDetails2(INetworkStateChangeEventDetails2Vtbl): IInspectable(IInspectableVtbl) [IID_INetworkStateChangeEventDetails2] {
    fn get_HasNewTetheringOperationalState(&self, out: *mut bool) -> HRESULT,
    fn get_HasNewTetheringClientCount(&self, out: *mut bool) -> HRESULT
}}
impl INetworkStateChangeEventDetails2 {
    #[inline] pub fn get_has_new_tethering_operational_state(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewTetheringOperationalState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_new_tethering_client_count(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNewTetheringClientCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_NetworkStatusChangedEventHandler, 1908020287, 22926, 18896, 132, 235, 143, 235, 174, 220, 193, 149);
RT_DELEGATE!{delegate NetworkStatusChangedEventHandler(NetworkStatusChangedEventHandlerVtbl, NetworkStatusChangedEventHandlerImpl) [IID_NetworkStatusChangedEventHandler] {
    fn Invoke(&self, sender: *mut IInspectable) -> HRESULT
}}
impl NetworkStatusChangedEventHandler {
    #[inline] pub fn invoke(&self, sender: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum NetworkTypes: u32 {
    None = 0, Internet = 1, PrivateNetwork = 2,
}}
DEFINE_IID!(IID_INetworkUsage, 1239060430, 39301, 18727, 191, 91, 7, 43, 92, 101, 248, 217);
RT_INTERFACE!{interface INetworkUsage(INetworkUsageVtbl): IInspectable(IInspectableVtbl) [IID_INetworkUsage] {
    fn get_BytesSent(&self, out: *mut u64) -> HRESULT,
    fn get_BytesReceived(&self, out: *mut u64) -> HRESULT,
    fn get_ConnectionDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl INetworkUsage {
    #[inline] pub fn get_bytes_sent(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesSent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_received(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReceived)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ConnectionDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkUsage: INetworkUsage}
RT_STRUCT! { struct NetworkUsageStates {
    Roaming: TriStates, Shared: TriStates,
}}
RT_CLASS!{class IPInformation: IIPInformation}
DEFINE_IID!(IID_IProviderNetworkUsage, 1590074884, 31025, 18632, 184, 243, 70, 48, 15, 164, 39, 40);
RT_INTERFACE!{interface IProviderNetworkUsage(IProviderNetworkUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProviderNetworkUsage] {
    fn get_BytesSent(&self, out: *mut u64) -> HRESULT,
    fn get_BytesReceived(&self, out: *mut u64) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProviderNetworkUsage {
    #[inline] pub fn get_bytes_sent(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesSent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_received(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReceived)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProviderNetworkUsage: IProviderNetworkUsage}
DEFINE_IID!(IID_IProxyConfiguration, 4013580468, 36868, 19926, 183, 216, 179, 229, 2, 244, 170, 208);
RT_INTERFACE!{interface IProxyConfiguration(IProxyConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IProxyConfiguration] {
    fn get_ProxyUris(&self, out: *mut *mut foundation::collections::IVectorView<foundation::Uri>) -> HRESULT,
    fn get_CanConnectDirectly(&self, out: *mut bool) -> HRESULT
}}
impl IProxyConfiguration {
    #[inline] pub fn get_proxy_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyUris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_can_connect_directly(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanConnectDirectly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProxyConfiguration: IProxyConfiguration}
RT_ENUM! { enum RoamingStates: u32 {
    None = 0, NotRoaming = 1, Roaming = 2,
}}
DEFINE_IID!(IID_IRoutePolicy, 296469676, 4039, 17124, 135, 66, 86, 153, 35, 177, 202, 17);
RT_INTERFACE!{interface IRoutePolicy(IRoutePolicyVtbl): IInspectable(IInspectableVtbl) [IID_IRoutePolicy] {
    fn get_ConnectionProfile(&self, out: *mut *mut ConnectionProfile) -> HRESULT,
    fn get_HostName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_HostNameType(&self, out: *mut super::DomainNameType) -> HRESULT
}}
impl IRoutePolicy {
    #[inline] pub fn get_connection_profile(&self) -> Result<Option<ComPtr<ConnectionProfile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ConnectionProfile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_host_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_host_name_type(&self) -> Result<super::DomainNameType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HostNameType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RoutePolicy: IRoutePolicy}
impl RtActivatable<IRoutePolicyFactory> for RoutePolicy {}
impl RoutePolicy {
    #[inline] pub fn create_route_policy(connectionProfile: &ConnectionProfile, hostName: &super::HostName, type_: super::DomainNameType) -> Result<ComPtr<RoutePolicy>> {
        <Self as RtActivatable<IRoutePolicyFactory>>::get_activation_factory().create_route_policy(connectionProfile, hostName, type_)
    }
}
DEFINE_CLSID!(RoutePolicy(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,67,111,110,110,101,99,116,105,118,105,116,121,46,82,111,117,116,101,80,111,108,105,99,121,0]) [CLSID_RoutePolicy]);
DEFINE_IID!(IID_IRoutePolicyFactory, 906131763, 41358, 19893, 166, 151, 245, 143, 167, 54, 78, 68);
RT_INTERFACE!{static interface IRoutePolicyFactory(IRoutePolicyFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRoutePolicyFactory] {
    fn CreateRoutePolicy(&self, connectionProfile: *mut ConnectionProfile, hostName: *mut super::HostName, type_: super::DomainNameType, out: *mut *mut RoutePolicy) -> HRESULT
}}
impl IRoutePolicyFactory {
    #[inline] pub fn create_route_policy(&self, connectionProfile: &ConnectionProfile, hostName: &super::HostName, type_: super::DomainNameType) -> Result<ComPtr<RoutePolicy>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateRoutePolicy)(self as *const _ as *mut _, connectionProfile as *const _ as *mut _, hostName as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TriStates: i32 {
    DoNotCare = 0, No = 1, Yes = 2,
}}
DEFINE_IID!(IID_IWlanConnectionProfileDetails, 1444976843, 45914, 19441, 168, 132, 183, 85, 126, 136, 255, 134);
RT_INTERFACE!{interface IWlanConnectionProfileDetails(IWlanConnectionProfileDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IWlanConnectionProfileDetails] {
    fn GetConnectedSsid(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWlanConnectionProfileDetails {
    #[inline] pub fn get_connected_ssid(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectedSsid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WlanConnectionProfileDetails: IWlanConnectionProfileDetails}
DEFINE_IID!(IID_IWwanConnectionProfileDetails, 239970558, 33631, 19955, 130, 253, 223, 85, 110, 188, 9, 239);
RT_INTERFACE!{interface IWwanConnectionProfileDetails(IWwanConnectionProfileDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IWwanConnectionProfileDetails] {
    fn get_HomeProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AccessPointName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetNetworkRegistrationState(&self, out: *mut WwanNetworkRegistrationState) -> HRESULT,
    fn GetCurrentDataClass(&self, out: *mut WwanDataClass) -> HRESULT
}}
impl IWwanConnectionProfileDetails {
    #[inline] pub fn get_home_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_access_point_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccessPointName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_registration_state(&self) -> Result<WwanNetworkRegistrationState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetNetworkRegistrationState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_data_class(&self) -> Result<WwanDataClass> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetCurrentDataClass)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WwanConnectionProfileDetails: IWwanConnectionProfileDetails}
DEFINE_IID!(IID_IWwanConnectionProfileDetails2, 2054508254, 41453, 18610, 142, 146, 180, 96, 3, 61, 82, 226);
RT_INTERFACE!{interface IWwanConnectionProfileDetails2(IWwanConnectionProfileDetails2Vtbl): IInspectable(IInspectableVtbl) [IID_IWwanConnectionProfileDetails2] {
    fn get_IPKind(&self, out: *mut WwanNetworkIPKind) -> HRESULT,
    fn get_PurposeGuids(&self, out: *mut *mut foundation::collections::IVectorView<Guid>) -> HRESULT
}}
impl IWwanConnectionProfileDetails2 {
    #[inline] pub fn get_ip_kind(&self) -> Result<WwanNetworkIPKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IPKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_purpose_guids(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<Guid>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PurposeGuids)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WwanDataClass: u32 {
    None = 0, Gprs = 1, Edge = 2, Umts = 4, Hsdpa = 8, Hsupa = 16, LteAdvanced = 32, Cdma1xRtt = 65536, Cdma1xEvdo = 131072, Cdma1xEvdoRevA = 262144, Cdma1xEvdv = 524288, Cdma3xRtt = 1048576, Cdma1xEvdoRevB = 2097152, CdmaUmb = 4194304, Custom = 2147483648,
}}
RT_ENUM! { enum WwanNetworkIPKind: i32 {
    None = 0, Ipv4 = 1, Ipv6 = 2, Ipv4v6 = 3, Ipv4v6v4Xlat = 4,
}}
RT_ENUM! { enum WwanNetworkRegistrationState: i32 {
    None = 0, Deregistered = 1, Searching = 2, Home = 3, Roaming = 4, Partner = 5, Denied = 6,
}}
} // Windows.Networking.Connectivity
pub mod networkoperators { // Windows.Networking.NetworkOperators
use crate::prelude::*;
RT_ENUM! { enum DataClasses: u32 {
    None = 0, Gprs = 1, Edge = 2, Umts = 4, Hsdpa = 8, Hsupa = 16, LteAdvanced = 32, Cdma1xRtt = 65536, Cdma1xEvdo = 131072, Cdma1xEvdoRevA = 262144, Cdma1xEvdv = 524288, Cdma3xRtt = 1048576, Cdma1xEvdoRevB = 2097152, CdmaUmb = 4194304, Custom = 2147483648,
}}
DEFINE_IID!(IID_IESim, 1869508134, 61731, 17277, 140, 237, 220, 29, 43, 192, 195, 169);
RT_INTERFACE!{interface IESim(IESimVtbl): IInspectable(IInspectableVtbl) [IID_IESim] {
    fn get_AvailableMemoryInBytes(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_Eid(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirmwareVersion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MobileBroadbandModemDeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Policy(&self, out: *mut *mut ESimPolicy) -> HRESULT,
    fn get_State(&self, out: *mut ESimState) -> HRESULT,
    fn GetProfiles(&self, out: *mut *mut foundation::collections::IVectorView<ESimProfile>) -> HRESULT,
    fn DeleteProfileAsync(&self, profileId: HSTRING, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn DownloadProfileMetadataAsync(&self, activationCode: HSTRING, out: *mut *mut foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>) -> HRESULT,
    fn ResetAsync(&self, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn add_ProfileChanged(&self, handler: *mut foundation::TypedEventHandler<ESim, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProfileChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IESim {
    #[inline] pub fn get_available_memory_in_bytes(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AvailableMemoryInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_eid(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Eid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_firmware_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirmwareVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mobile_broadband_modem_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MobileBroadbandModemDeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_policy(&self) -> Result<Option<ComPtr<ESimPolicy>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Policy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<ESimState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_profiles(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ESimProfile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProfiles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_profile_async(&self, profileId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteProfileAsync)(self as *const _ as *mut _, profileId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn download_profile_metadata_async(&self, activationCode: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ESimDownloadProfileMetadataResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DownloadProfileMetadataAsync)(self as *const _ as *mut _, activationCode.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn reset_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ResetAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_profile_changed(&self, handler: &foundation::TypedEventHandler<ESim, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ProfileChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_profile_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ProfileChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ESim: IESim}
DEFINE_IID!(IID_IESimAddedEventArgs, 951913048, 19802, 19720, 141, 167, 231, 62, 255, 54, 157, 221);
RT_INTERFACE!{interface IESimAddedEventArgs(IESimAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IESimAddedEventArgs] {
    fn get_ESim(&self, out: *mut *mut ESim) -> HRESULT
}}
impl IESimAddedEventArgs {
    #[inline] pub fn get_esim(&self) -> Result<Option<ComPtr<ESim>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ESim)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimAddedEventArgs: IESimAddedEventArgs}
RT_ENUM! { enum ESimAuthenticationPreference: i32 {
    OnEntry = 0, OnAction = 1, Never = 2,
}}
DEFINE_IID!(IID_IESimDownloadProfileMetadataResult, 3290647966, 23254, 17005, 141, 0, 68, 52, 244, 73, 175, 236);
RT_INTERFACE!{interface IESimDownloadProfileMetadataResult(IESimDownloadProfileMetadataResultVtbl): IInspectable(IInspectableVtbl) [IID_IESimDownloadProfileMetadataResult] {
    fn get_Result(&self, out: *mut *mut ESimOperationResult) -> HRESULT,
    fn get_ProfileMetadata(&self, out: *mut *mut ESimProfileMetadata) -> HRESULT
}}
impl IESimDownloadProfileMetadataResult {
    #[inline] pub fn get_result(&self) -> Result<Option<ComPtr<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Result)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_profile_metadata(&self) -> Result<Option<ComPtr<ESimProfileMetadata>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProfileMetadata)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimDownloadProfileMetadataResult: IESimDownloadProfileMetadataResult}
RT_CLASS!{static class ESimManager}
impl RtActivatable<IESimManagerStatics> for ESimManager {}
impl ESimManager {
    #[inline] pub fn get_service_info() -> Result<Option<ComPtr<ESimServiceInfo>>> {
        <Self as RtActivatable<IESimManagerStatics>>::get_activation_factory().get_service_info()
    }
    #[inline] pub fn try_create_esim_watcher() -> Result<Option<ComPtr<ESimWatcher>>> {
        <Self as RtActivatable<IESimManagerStatics>>::get_activation_factory().try_create_esim_watcher()
    }
    #[inline] pub fn add_service_info_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IESimManagerStatics>>::get_activation_factory().add_service_info_changed(handler)
    }
    #[inline] pub fn remove_service_info_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IESimManagerStatics>>::get_activation_factory().remove_service_info_changed(token)
    }
}
DEFINE_CLSID!(ESimManager(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,69,83,105,109,77,97,110,97,103,101,114,0]) [CLSID_ESimManager]);
DEFINE_IID!(IID_IESimManagerStatics, 200944652, 57224, 17969, 191, 4, 193, 46, 40, 27, 57, 98);
RT_INTERFACE!{static interface IESimManagerStatics(IESimManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IESimManagerStatics] {
    fn get_ServiceInfo(&self, out: *mut *mut ESimServiceInfo) -> HRESULT,
    fn TryCreateESimWatcher(&self, out: *mut *mut ESimWatcher) -> HRESULT,
    fn add_ServiceInfoChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ServiceInfoChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IESimManagerStatics {
    #[inline] pub fn get_service_info(&self) -> Result<Option<ComPtr<ESimServiceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_esim_watcher(&self) -> Result<Option<ComPtr<ESimWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateESimWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_service_info_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ServiceInfoChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_service_info_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ServiceInfoChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IESimOperationResult, 2793104305, 12443, 20087, 158, 126, 205, 147, 241, 221, 199, 185);
RT_INTERFACE!{interface IESimOperationResult(IESimOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_IESimOperationResult] {
    fn get_Status(&self, out: *mut ESimOperationStatus) -> HRESULT
}}
impl IESimOperationResult {
    #[inline] pub fn get_status(&self) -> Result<ESimOperationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimOperationResult: IESimOperationResult}
RT_ENUM! { enum ESimOperationStatus: i32 {
    Success = 0, NotAuthorized = 1, NotFound = 2, PolicyViolation = 3, InsufficientSpaceOnCard = 4, ServerFailure = 5, ServerNotReachable = 6, TimeoutWaitingForUserConsent = 7, IncorrectConfirmationCode = 8, ConfirmationCodeMaxRetriesExceeded = 9, CardRemoved = 10, CardBusy = 11, Other = 12, CardGeneralFailure = 13, ConfirmationCodeMissing = 14, InvalidMatchingId = 15, NoEligibleProfileForThisDevice = 16, OperationAborted = 17, EidMismatch = 18, ProfileNotAvailableForNewBinding = 19, ProfileNotReleasedByOperator = 20, OperationProhibitedByProfileClass = 21, ProfileNotPresent = 22, NoCorrespondingRequest = 23,
}}
DEFINE_IID!(IID_IESimPolicy, 1105312157, 53118, 17173, 136, 43, 111, 30, 116, 176, 211, 143);
RT_INTERFACE!{interface IESimPolicy(IESimPolicyVtbl): IInspectable(IInspectableVtbl) [IID_IESimPolicy] {
    fn get_ShouldEnableManagingUi(&self, out: *mut bool) -> HRESULT
}}
impl IESimPolicy {
    #[inline] pub fn get_should_enable_managing_ui(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShouldEnableManagingUi)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimPolicy: IESimPolicy}
DEFINE_IID!(IID_IESimProfile, 3994974336, 1705, 16423, 180, 248, 221, 178, 61, 120, 16, 224);
RT_INTERFACE!{interface IESimProfile(IESimProfileVtbl): IInspectable(IInspectableVtbl) [IID_IESimProfile] {
    fn get_Class(&self, out: *mut ESimProfileClass) -> HRESULT,
    fn get_Nickname(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Policy(&self, out: *mut *mut ESimProfilePolicy) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ProviderIcon(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStreamReference) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut ESimProfileState) -> HRESULT,
    fn DisableAsync(&self, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn EnableAsync(&self, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn SetNicknameAsync(&self, newNickname: HSTRING, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT
}}
impl IESimProfile {
    #[inline] pub fn get_class(&self) -> Result<ESimProfileClass> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Class)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_nickname(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Nickname)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_policy(&self) -> Result<Option<ComPtr<ESimProfilePolicy>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Policy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_provider_icon(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStreamReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderIcon)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<ESimProfileState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn disable_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DisableAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enable_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EnableAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_nickname_async(&self, newNickname: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetNicknameAsync)(self as *const _ as *mut _, newNickname.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimProfile: IESimProfile}
RT_ENUM! { enum ESimProfileClass: i32 {
    Operational = 0, Test = 1, Provisioning = 2,
}}
RT_STRUCT! { struct ESimProfileInstallProgress {
    TotalSizeInBytes: i32, InstalledSizeInBytes: i32,
}}
DEFINE_IID!(IID_IESimProfileMetadata, 3978658591, 37083, 18829, 167, 180, 235, 206, 128, 125, 60, 35);
RT_INTERFACE!{interface IESimProfileMetadata(IESimProfileMetadataVtbl): IInspectable(IInspectableVtbl) [IID_IESimProfileMetadata] {
    fn get_IsConfirmationCodeRequired(&self, out: *mut bool) -> HRESULT,
    fn get_Policy(&self, out: *mut *mut ESimProfilePolicy) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ProviderIcon(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStreamReference) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut ESimProfileMetadataState) -> HRESULT,
    fn DenyInstallAsync(&self, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn ConfirmInstallAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>) -> HRESULT,
    fn ConfirmInstallWithConfirmationCodeAsync(&self, confirmationCode: HSTRING, out: *mut *mut foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>) -> HRESULT,
    fn PostponeInstallAsync(&self, out: *mut *mut foundation::IAsyncOperation<ESimOperationResult>) -> HRESULT,
    fn add_StateChanged(&self, handler: *mut foundation::TypedEventHandler<ESimProfileMetadata, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IESimProfileMetadata {
    #[inline] pub fn get_is_confirmation_code_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsConfirmationCodeRequired)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_policy(&self) -> Result<Option<ComPtr<ESimProfilePolicy>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Policy)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_provider_icon(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStreamReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderIcon)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<ESimProfileMetadataState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn deny_install_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DenyInstallAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn confirm_install_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConfirmInstallAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn confirm_install_with_confirmation_code_async(&self, confirmationCode: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<ESimOperationResult, ESimProfileInstallProgress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConfirmInstallWithConfirmationCodeAsync)(self as *const _ as *mut _, confirmationCode.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn postpone_install_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ESimOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PostponeInstallAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_state_changed(&self, handler: &foundation::TypedEventHandler<ESimProfileMetadata, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_StateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimProfileMetadata: IESimProfileMetadata}
RT_ENUM! { enum ESimProfileMetadataState: i32 {
    Unknown = 0, WaitingForInstall = 1, Downloading = 2, Installing = 3, Expired = 4, RejectingDownload = 5, NoLongerAvailable = 6, DeniedByPolicy = 7,
}}
DEFINE_IID!(IID_IESimProfilePolicy, 3873247005, 40028, 18117, 162, 137, 169, 72, 153, 155, 240, 98);
RT_INTERFACE!{interface IESimProfilePolicy(IESimProfilePolicyVtbl): IInspectable(IInspectableVtbl) [IID_IESimProfilePolicy] {
    fn get_CanDelete(&self, out: *mut bool) -> HRESULT,
    fn get_CanDisable(&self, out: *mut bool) -> HRESULT,
    fn get_IsManagedByEnterprise(&self, out: *mut bool) -> HRESULT
}}
impl IESimProfilePolicy {
    #[inline] pub fn get_can_delete(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanDelete)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_can_disable(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanDisable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_managed_by_enterprise(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsManagedByEnterprise)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimProfilePolicy: IESimProfilePolicy}
RT_ENUM! { enum ESimProfileState: i32 {
    Unknown = 0, Disabled = 1, Enabled = 2, Deleted = 3,
}}
DEFINE_IID!(IID_IESimRemovedEventArgs, 3737462651, 12249, 20185, 131, 118, 217, 181, 228, 18, 120, 163);
RT_INTERFACE!{interface IESimRemovedEventArgs(IESimRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IESimRemovedEventArgs] {
    fn get_ESim(&self, out: *mut *mut ESim) -> HRESULT
}}
impl IESimRemovedEventArgs {
    #[inline] pub fn get_esim(&self) -> Result<Option<ComPtr<ESim>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ESim)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimRemovedEventArgs: IESimRemovedEventArgs}
DEFINE_IID!(IID_IESimServiceInfo, 4050299855, 32601, 19025, 132, 148, 189, 137, 213, 255, 80, 238);
RT_INTERFACE!{interface IESimServiceInfo(IESimServiceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IESimServiceInfo] {
    fn get_AuthenticationPreference(&self, out: *mut ESimAuthenticationPreference) -> HRESULT,
    fn get_IsESimUiEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IESimServiceInfo {
    #[inline] pub fn get_authentication_preference(&self) -> Result<ESimAuthenticationPreference> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationPreference)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_esim_ui_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsESimUiEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimServiceInfo: IESimServiceInfo}
RT_ENUM! { enum ESimState: i32 {
    Unknown = 0, Idle = 1, Removed = 2, Busy = 3,
}}
DEFINE_IID!(IID_IESimUpdatedEventArgs, 1276271852, 20621, 19336, 131, 203, 104, 190, 248, 22, 141, 18);
RT_INTERFACE!{interface IESimUpdatedEventArgs(IESimUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IESimUpdatedEventArgs] {
    fn get_ESim(&self, out: *mut *mut ESim) -> HRESULT
}}
impl IESimUpdatedEventArgs {
    #[inline] pub fn get_esim(&self) -> Result<Option<ComPtr<ESim>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ESim)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimUpdatedEventArgs: IESimUpdatedEventArgs}
DEFINE_IID!(IID_IESimWatcher, 3254275307, 41613, 20415, 151, 113, 110, 49, 184, 28, 207, 34);
RT_INTERFACE!{interface IESimWatcher(IESimWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IESimWatcher] {
    fn get_Status(&self, out: *mut ESimWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<ESimWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<ESimWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IESimWatcher {
    #[inline] pub fn get_status(&self) -> Result<ESimWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<ESimWatcher, ESimAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<ESimWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<ESimWatcher, ESimRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<ESimWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<ESimWatcher, ESimUpdatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ESimWatcher: IESimWatcher}
RT_ENUM! { enum ESimWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4,
}}
DEFINE_IID!(IID_IHotspotAuthenticationContext, 3881224081, 4099, 19941, 131, 199, 222, 97, 216, 136, 49, 208);
RT_INTERFACE!{interface IHotspotAuthenticationContext(IHotspotAuthenticationContextVtbl): IInspectable(IInspectableVtbl) [IID_IHotspotAuthenticationContext] {
    fn get_WirelessNetworkId(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn get_NetworkAdapter(&self, out: *mut *mut super::connectivity::NetworkAdapter) -> HRESULT,
    fn get_RedirectMessageUrl(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-data")] fn get_RedirectMessageXml(&self, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT,
    fn get_AuthenticationUrl(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn IssueCredentials(&self, userName: HSTRING, password: HSTRING, extraParameters: HSTRING, markAsManualConnectOnFailure: bool) -> HRESULT,
    fn AbortAuthentication(&self, markAsManual: bool) -> HRESULT,
    fn SkipAuthentication(&self) -> HRESULT,
    fn TriggerAttentionRequired(&self, packageRelativeApplicationId: HSTRING, applicationParameters: HSTRING) -> HRESULT
}}
impl IHotspotAuthenticationContext {
    #[inline] pub fn get_wireless_network_id(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WirelessNetworkId)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_adapter(&self) -> Result<Option<ComPtr<super::connectivity::NetworkAdapter>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAdapter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_redirect_message_url(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RedirectMessageUrl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_redirect_message_xml(&self) -> Result<Option<ComPtr<super::super::data::xml::dom::XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RedirectMessageXml)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_url(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AuthenticationUrl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn issue_credentials(&self, userName: &HStringArg, password: &HStringArg, extraParameters: &HStringArg, markAsManualConnectOnFailure: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).IssueCredentials)(self as *const _ as *mut _, userName.get(), password.get(), extraParameters.get(), markAsManualConnectOnFailure);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn abort_authentication(&self, markAsManual: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AbortAuthentication)(self as *const _ as *mut _, markAsManual);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn skip_authentication(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SkipAuthentication)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn trigger_attention_required(&self, packageRelativeApplicationId: &HStringArg, applicationParameters: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TriggerAttentionRequired)(self as *const _ as *mut _, packageRelativeApplicationId.get(), applicationParameters.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HotspotAuthenticationContext: IHotspotAuthenticationContext}
impl RtActivatable<IHotspotAuthenticationContextStatics> for HotspotAuthenticationContext {}
impl HotspotAuthenticationContext {
    #[inline] pub fn try_get_authentication_context(evenToken: &HStringArg) -> Result<(Option<ComPtr<HotspotAuthenticationContext>>, bool)> {
        <Self as RtActivatable<IHotspotAuthenticationContextStatics>>::get_activation_factory().try_get_authentication_context(evenToken)
    }
}
DEFINE_CLSID!(HotspotAuthenticationContext(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,72,111,116,115,112,111,116,65,117,116,104,101,110,116,105,99,97,116,105,111,110,67,111,110,116,101,120,116,0]) [CLSID_HotspotAuthenticationContext]);
DEFINE_IID!(IID_IHotspotAuthenticationContext2, 3881224081, 4100, 19941, 131, 199, 222, 97, 216, 136, 49, 208);
RT_INTERFACE!{interface IHotspotAuthenticationContext2(IHotspotAuthenticationContext2Vtbl): IInspectable(IInspectableVtbl) [IID_IHotspotAuthenticationContext2] {
    fn IssueCredentialsAsync(&self, userName: HSTRING, password: HSTRING, extraParameters: HSTRING, markAsManualConnectOnFailure: bool, out: *mut *mut foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>) -> HRESULT
}}
impl IHotspotAuthenticationContext2 {
    #[inline] pub fn issue_credentials_async(&self, userName: &HStringArg, password: &HStringArg, extraParameters: &HStringArg, markAsManualConnectOnFailure: bool) -> Result<ComPtr<foundation::IAsyncOperation<HotspotCredentialsAuthenticationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).IssueCredentialsAsync)(self as *const _ as *mut _, userName.get(), password.get(), extraParameters.get(), markAsManualConnectOnFailure, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHotspotAuthenticationContextStatics, 3881224081, 4098, 19941, 131, 199, 222, 97, 216, 136, 49, 208);
RT_INTERFACE!{static interface IHotspotAuthenticationContextStatics(IHotspotAuthenticationContextStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHotspotAuthenticationContextStatics] {
    fn TryGetAuthenticationContext(&self, evenToken: HSTRING, context: *mut *mut HotspotAuthenticationContext, out: *mut bool) -> HRESULT
}}
impl IHotspotAuthenticationContextStatics {
    #[inline] pub fn try_get_authentication_context(&self, evenToken: &HStringArg) -> Result<(Option<ComPtr<HotspotAuthenticationContext>>, bool)> { unsafe { 
        let mut context = null_mut(); let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryGetAuthenticationContext)(self as *const _ as *mut _, evenToken.get(), &mut context, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(context), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHotspotAuthenticationEventDetails, 3881224081, 4097, 19941, 131, 199, 222, 97, 216, 136, 49, 208);
RT_INTERFACE!{interface IHotspotAuthenticationEventDetails(IHotspotAuthenticationEventDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IHotspotAuthenticationEventDetails] {
    fn get_EventToken(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHotspotAuthenticationEventDetails {
    #[inline] pub fn get_event_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EventToken)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HotspotAuthenticationEventDetails: IHotspotAuthenticationEventDetails}
RT_ENUM! { enum HotspotAuthenticationResponseCode: i32 {
    NoError = 0, LoginSucceeded = 50, LoginFailed = 100, RadiusServerError = 102, NetworkAdministratorError = 105, LoginAborted = 151, AccessGatewayInternalError = 255,
}}
DEFINE_IID!(IID_IHotspotCredentialsAuthenticationResult, 3881224081, 4101, 19941, 131, 199, 222, 97, 216, 136, 49, 208);
RT_INTERFACE!{interface IHotspotCredentialsAuthenticationResult(IHotspotCredentialsAuthenticationResultVtbl): IInspectable(IInspectableVtbl) [IID_IHotspotCredentialsAuthenticationResult] {
    fn get_HasNetworkErrorOccurred(&self, out: *mut bool) -> HRESULT,
    fn get_ResponseCode(&self, out: *mut HotspotAuthenticationResponseCode) -> HRESULT,
    fn get_LogoffUrl(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    #[cfg(feature="windows-data")] fn get_AuthenticationReplyXml(&self, out: *mut *mut super::super::data::xml::dom::XmlDocument) -> HRESULT
}}
impl IHotspotCredentialsAuthenticationResult {
    #[inline] pub fn get_has_network_error_occurred(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNetworkErrorOccurred)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_response_code(&self) -> Result<HotspotAuthenticationResponseCode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResponseCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_logoff_url(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LogoffUrl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_authentication_reply_xml(&self) -> Result<Option<ComPtr<super::super::data::xml::dom::XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AuthenticationReplyXml)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HotspotCredentialsAuthenticationResult: IHotspotCredentialsAuthenticationResult}
RT_CLASS!{static class KnownCSimFilePaths}
impl RtActivatable<IKnownCSimFilePathsStatics> for KnownCSimFilePaths {}
impl KnownCSimFilePaths {
    #[inline] pub fn get_efspn() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownCSimFilePathsStatics>>::get_activation_factory().get_efspn()
    }
    #[inline] pub fn get_gid1() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownCSimFilePathsStatics>>::get_activation_factory().get_gid1()
    }
    #[inline] pub fn get_gid2() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownCSimFilePathsStatics>>::get_activation_factory().get_gid2()
    }
}
DEFINE_CLSID!(KnownCSimFilePaths(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,75,110,111,119,110,67,83,105,109,70,105,108,101,80,97,116,104,115,0]) [CLSID_KnownCSimFilePaths]);
DEFINE_IID!(IID_IKnownCSimFilePathsStatics, 3025710829, 18929, 19490, 176, 115, 150, 213, 17, 191, 156, 53);
RT_INTERFACE!{static interface IKnownCSimFilePathsStatics(IKnownCSimFilePathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownCSimFilePathsStatics] {
    fn get_EFSpn(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid1(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid2(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl IKnownCSimFilePathsStatics {
    #[inline] pub fn get_efspn(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFSpn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid1(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid2(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class KnownRuimFilePaths}
impl RtActivatable<IKnownRuimFilePathsStatics> for KnownRuimFilePaths {}
impl KnownRuimFilePaths {
    #[inline] pub fn get_efspn() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownRuimFilePathsStatics>>::get_activation_factory().get_efspn()
    }
    #[inline] pub fn get_gid1() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownRuimFilePathsStatics>>::get_activation_factory().get_gid1()
    }
    #[inline] pub fn get_gid2() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownRuimFilePathsStatics>>::get_activation_factory().get_gid2()
    }
}
DEFINE_CLSID!(KnownRuimFilePaths(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,75,110,111,119,110,82,117,105,109,70,105,108,101,80,97,116,104,115,0]) [CLSID_KnownRuimFilePaths]);
DEFINE_IID!(IID_IKnownRuimFilePathsStatics, 948160697, 65316, 17777, 168, 103, 9, 249, 96, 66, 110, 20);
RT_INTERFACE!{static interface IKnownRuimFilePathsStatics(IKnownRuimFilePathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownRuimFilePathsStatics] {
    fn get_EFSpn(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid1(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid2(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl IKnownRuimFilePathsStatics {
    #[inline] pub fn get_efspn(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFSpn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid1(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid2(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class KnownSimFilePaths}
impl RtActivatable<IKnownSimFilePathsStatics> for KnownSimFilePaths {}
impl KnownSimFilePaths {
    #[inline] pub fn get_efons() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownSimFilePathsStatics>>::get_activation_factory().get_efons()
    }
    #[inline] pub fn get_efspn() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownSimFilePathsStatics>>::get_activation_factory().get_efspn()
    }
    #[inline] pub fn get_gid1() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownSimFilePathsStatics>>::get_activation_factory().get_gid1()
    }
    #[inline] pub fn get_gid2() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownSimFilePathsStatics>>::get_activation_factory().get_gid2()
    }
}
DEFINE_CLSID!(KnownSimFilePaths(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,75,110,111,119,110,83,105,109,70,105,108,101,80,97,116,104,115,0]) [CLSID_KnownSimFilePaths]);
DEFINE_IID!(IID_IKnownSimFilePathsStatics, 2160925283, 14245, 17363, 128, 163, 204, 210, 62, 143, 236, 238);
RT_INTERFACE!{static interface IKnownSimFilePathsStatics(IKnownSimFilePathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownSimFilePathsStatics] {
    fn get_EFOns(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_EFSpn(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid1(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid2(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl IKnownSimFilePathsStatics {
    #[inline] pub fn get_efons(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFOns)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_efspn(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFSpn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid1(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid2(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class KnownUSimFilePaths}
impl RtActivatable<IKnownUSimFilePathsStatics> for KnownUSimFilePaths {}
impl KnownUSimFilePaths {
    #[inline] pub fn get_efspn() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownUSimFilePathsStatics>>::get_activation_factory().get_efspn()
    }
    #[inline] pub fn get_efopl() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownUSimFilePathsStatics>>::get_activation_factory().get_efopl()
    }
    #[inline] pub fn get_efpnn() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownUSimFilePathsStatics>>::get_activation_factory().get_efpnn()
    }
    #[inline] pub fn get_gid1() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownUSimFilePathsStatics>>::get_activation_factory().get_gid1()
    }
    #[inline] pub fn get_gid2() -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> {
        <Self as RtActivatable<IKnownUSimFilePathsStatics>>::get_activation_factory().get_gid2()
    }
}
DEFINE_CLSID!(KnownUSimFilePaths(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,75,110,111,119,110,85,83,105,109,70,105,108,101,80,97,116,104,115,0]) [CLSID_KnownUSimFilePaths]);
DEFINE_IID!(IID_IKnownUSimFilePathsStatics, 2083841409, 7963, 17396, 149, 48, 139, 9, 45, 50, 215, 31);
RT_INTERFACE!{static interface IKnownUSimFilePathsStatics(IKnownUSimFilePathsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownUSimFilePathsStatics] {
    fn get_EFSpn(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_EFOpl(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_EFPnn(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid1(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_Gid2(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl IKnownUSimFilePathsStatics {
    #[inline] pub fn get_efspn(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFSpn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_efopl(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFOpl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_efpnn(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EFPnn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid1(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gid2(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Gid2)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandAccount, 918703309, 52962, 17376, 166, 3, 238, 134, 163, 109, 101, 112);
RT_INTERFACE!{interface IMobileBroadbandAccount(IMobileBroadbandAccountVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccount] {
    fn get_NetworkAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ServiceProviderGuid(&self, out: *mut Guid) -> HRESULT,
    fn get_ServiceProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CurrentNetwork(&self, out: *mut *mut MobileBroadbandNetwork) -> HRESULT,
    fn get_CurrentDeviceInformation(&self, out: *mut *mut MobileBroadbandDeviceInformation) -> HRESULT
}}
impl IMobileBroadbandAccount {
    #[inline] pub fn get_network_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_service_provider_guid(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServiceProviderGuid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_service_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_network(&self) -> Result<Option<ComPtr<MobileBroadbandNetwork>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentNetwork)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_device_information(&self) -> Result<Option<ComPtr<MobileBroadbandDeviceInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentDeviceInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandAccount: IMobileBroadbandAccount}
impl RtActivatable<IMobileBroadbandAccountStatics> for MobileBroadbandAccount {}
impl MobileBroadbandAccount {
    #[inline] pub fn get_available_network_account_ids() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<IMobileBroadbandAccountStatics>>::get_activation_factory().get_available_network_account_ids()
    }
    #[inline] pub fn create_from_network_account_id(networkAccountId: &HStringArg) -> Result<Option<ComPtr<MobileBroadbandAccount>>> {
        <Self as RtActivatable<IMobileBroadbandAccountStatics>>::get_activation_factory().create_from_network_account_id(networkAccountId)
    }
}
DEFINE_CLSID!(MobileBroadbandAccount(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,77,111,98,105,108,101,66,114,111,97,100,98,97,110,100,65,99,99,111,117,110,116,0]) [CLSID_MobileBroadbandAccount]);
DEFINE_IID!(IID_IMobileBroadbandAccount2, 955592476, 4406, 16983, 149, 159, 182, 88, 163, 82, 182, 212);
RT_INTERFACE!{interface IMobileBroadbandAccount2(IMobileBroadbandAccount2Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccount2] {
    fn GetConnectionProfiles(&self, out: *mut *mut foundation::collections::IVectorView<super::connectivity::ConnectionProfile>) -> HRESULT
}}
impl IMobileBroadbandAccount2 {
    #[inline] pub fn get_connection_profiles(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::connectivity::ConnectionProfile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetConnectionProfiles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandAccount3, 153755169, 37753, 19355, 173, 49, 213, 254, 226, 247, 72, 198);
RT_INTERFACE!{interface IMobileBroadbandAccount3(IMobileBroadbandAccount3Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccount3] {
    fn get_AccountExperienceUrl(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl IMobileBroadbandAccount3 {
    #[inline] pub fn get_account_experience_url(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccountExperienceUrl)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandAccountEventArgs, 945014912, 30686, 19460, 190, 173, 161, 35, 176, 140, 159, 89);
RT_INTERFACE!{interface IMobileBroadbandAccountEventArgs(IMobileBroadbandAccountEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccountEventArgs] {
    fn get_NetworkAccountId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMobileBroadbandAccountEventArgs {
    #[inline] pub fn get_network_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandAccountEventArgs: IMobileBroadbandAccountEventArgs}
DEFINE_IID!(IID_IMobileBroadbandAccountStatics, 2860469540, 44993, 20424, 174, 154, 169, 23, 83, 16, 250, 173);
RT_INTERFACE!{static interface IMobileBroadbandAccountStatics(IMobileBroadbandAccountStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccountStatics] {
    fn get_AvailableNetworkAccountIds(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn CreateFromNetworkAccountId(&self, networkAccountId: HSTRING, out: *mut *mut MobileBroadbandAccount) -> HRESULT
}}
impl IMobileBroadbandAccountStatics {
    #[inline] pub fn get_available_network_account_ids(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AvailableNetworkAccountIds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_network_account_id(&self, networkAccountId: &HStringArg) -> Result<Option<ComPtr<MobileBroadbandAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNetworkAccountId)(self as *const _ as *mut _, networkAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandAccountUpdatedEventArgs, 2076384648, 42685, 18913, 128, 171, 107, 145, 53, 74, 87, 212);
RT_INTERFACE!{interface IMobileBroadbandAccountUpdatedEventArgs(IMobileBroadbandAccountUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccountUpdatedEventArgs] {
    fn get_NetworkAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasDeviceInformationChanged(&self, out: *mut bool) -> HRESULT,
    fn get_HasNetworkChanged(&self, out: *mut bool) -> HRESULT
}}
impl IMobileBroadbandAccountUpdatedEventArgs {
    #[inline] pub fn get_network_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_device_information_changed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasDeviceInformationChanged)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_has_network_changed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasNetworkChanged)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandAccountUpdatedEventArgs: IMobileBroadbandAccountUpdatedEventArgs}
DEFINE_IID!(IID_IMobileBroadbandAccountWatcher, 1811100510, 9141, 17567, 146, 141, 94, 13, 62, 4, 71, 29);
RT_INTERFACE!{interface IMobileBroadbandAccountWatcher(IMobileBroadbandAccountWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAccountWatcher] {
    fn add_AccountAdded(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AccountAdded(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AccountUpdated(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AccountUpdated(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AccountRemoved(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AccountRemoved(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandAccountWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandAccountWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut MobileBroadbandAccountWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IMobileBroadbandAccountWatcher {
    #[inline] pub fn add_account_added(&self, handler: &foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AccountAdded)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_account_added(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AccountAdded)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_account_updated(&self, handler: &foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountUpdatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AccountUpdated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_account_updated(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AccountUpdated)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_account_removed(&self, handler: &foundation::TypedEventHandler<MobileBroadbandAccountWatcher, MobileBroadbandAccountEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AccountRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_account_removed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AccountRemoved)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<MobileBroadbandAccountWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<MobileBroadbandAccountWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<MobileBroadbandAccountWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandAccountWatcher: IMobileBroadbandAccountWatcher}
impl RtActivatable<IActivationFactory> for MobileBroadbandAccountWatcher {}
DEFINE_CLSID!(MobileBroadbandAccountWatcher(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,77,111,98,105,108,101,66,114,111,97,100,98,97,110,100,65,99,99,111,117,110,116,87,97,116,99,104,101,114,0]) [CLSID_MobileBroadbandAccountWatcher]);
RT_ENUM! { enum MobileBroadbandAccountWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopped = 3, Aborted = 4,
}}
DEFINE_IID!(IID_IMobileBroadbandAntennaSar, 3115273086, 52217, 16649, 144, 190, 92, 6, 191, 213, 19, 182);
RT_INTERFACE!{interface IMobileBroadbandAntennaSar(IMobileBroadbandAntennaSarVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAntennaSar] {
    fn get_AntennaIndex(&self, out: *mut i32) -> HRESULT,
    fn get_SarBackoffIndex(&self, out: *mut i32) -> HRESULT
}}
impl IMobileBroadbandAntennaSar {
    #[inline] pub fn get_antenna_index(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AntennaIndex)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sar_backoff_index(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SarBackoffIndex)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandAntennaSar: IMobileBroadbandAntennaSar}
impl RtActivatable<IMobileBroadbandAntennaSarFactory> for MobileBroadbandAntennaSar {}
impl MobileBroadbandAntennaSar {
    #[inline] pub fn create_with_index(antennaIndex: i32, sarBackoffIndex: i32) -> Result<ComPtr<MobileBroadbandAntennaSar>> {
        <Self as RtActivatable<IMobileBroadbandAntennaSarFactory>>::get_activation_factory().create_with_index(antennaIndex, sarBackoffIndex)
    }
}
DEFINE_CLSID!(MobileBroadbandAntennaSar(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,77,111,98,105,108,101,66,114,111,97,100,98,97,110,100,65,110,116,101,110,110,97,83,97,114,0]) [CLSID_MobileBroadbandAntennaSar]);
DEFINE_IID!(IID_IMobileBroadbandAntennaSarFactory, 2837321494, 49229, 18977, 134, 152, 20, 89, 220, 103, 44, 110);
RT_INTERFACE!{static interface IMobileBroadbandAntennaSarFactory(IMobileBroadbandAntennaSarFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandAntennaSarFactory] {
    fn CreateWithIndex(&self, antennaIndex: i32, sarBackoffIndex: i32, out: *mut *mut MobileBroadbandAntennaSar) -> HRESULT
}}
impl IMobileBroadbandAntennaSarFactory {
    #[inline] pub fn create_with_index(&self, antennaIndex: i32, sarBackoffIndex: i32) -> Result<ComPtr<MobileBroadbandAntennaSar>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithIndex)(self as *const _ as *mut _, antennaIndex, sarBackoffIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandCellCdma, 100774836, 16666, 20270, 130, 135, 118, 245, 101, 12, 96, 205);
RT_INTERFACE!{interface IMobileBroadbandCellCdma(IMobileBroadbandCellCdmaVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellCdma] {
    fn get_BaseStationId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_BaseStationPNCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_BaseStationLatitude(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_BaseStationLongitude(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_BaseStationLastBroadcastGpsTime(&self, out: *mut *mut foundation::IReference<foundation::TimeSpan>) -> HRESULT,
    fn get_NetworkId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PilotSignalStrengthInDB(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_SystemId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT
}}
impl IMobileBroadbandCellCdma {
    #[inline] pub fn get_base_station_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_base_station_pn_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationPNCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_base_station_latitude(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationLatitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_base_station_longitude(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationLongitude)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_base_station_last_broadcast_gps_time(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::TimeSpan>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationLastBroadcastGpsTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pilot_signal_strength_in_db(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PilotSignalStrengthInDB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellCdma: IMobileBroadbandCellCdma}
DEFINE_IID!(IID_IMobileBroadbandCellGsm, 3432087302, 32480, 18360, 158, 31, 195, 180, 141, 249, 223, 91);
RT_INTERFACE!{interface IMobileBroadbandCellGsm(IMobileBroadbandCellGsmVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellGsm] {
    fn get_BaseStationId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_CellId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ChannelNumber(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_LocationAreaCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReceivedSignalStrengthInDBm(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_TimingAdvanceInBitPeriods(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT
}}
impl IMobileBroadbandCellGsm {
    #[inline] pub fn get_base_station_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseStationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cell_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CellId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_channel_number(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChannelNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_location_area_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocationAreaCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_received_signal_strength_in_dbm(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReceivedSignalStrengthInDBm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_timing_advance_in_bit_periods(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TimingAdvanceInBitPeriods)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellGsm: IMobileBroadbandCellGsm}
DEFINE_IID!(IID_IMobileBroadbandCellLte, 2442643579, 11128, 17773, 139, 83, 170, 162, 93, 10, 247, 65);
RT_INTERFACE!{interface IMobileBroadbandCellLte(IMobileBroadbandCellLteVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellLte] {
    fn get_CellId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ChannelNumber(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PhysicalCellId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReferenceSignalReceivedPowerInDBm(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_ReferenceSignalReceivedQualityInDBm(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_TimingAdvanceInBitPeriods(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_TrackingAreaCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT
}}
impl IMobileBroadbandCellLte {
    #[inline] pub fn get_cell_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CellId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_channel_number(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChannelNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_physical_cell_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PhysicalCellId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reference_signal_received_power_in_dbm(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReferenceSignalReceivedPowerInDBm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reference_signal_received_quality_in_dbm(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReferenceSignalReceivedQualityInDBm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_timing_advance_in_bit_periods(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TimingAdvanceInBitPeriods)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tracking_area_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrackingAreaCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellLte: IMobileBroadbandCellLte}
DEFINE_IID!(IID_IMobileBroadbandCellsInfo, 2309576234, 58482, 19877, 146, 156, 222, 97, 113, 29, 210, 97);
RT_INTERFACE!{interface IMobileBroadbandCellsInfo(IMobileBroadbandCellsInfoVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellsInfo] {
    fn get_NeighboringCellsCdma(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellCdma>) -> HRESULT,
    fn get_NeighboringCellsGsm(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellGsm>) -> HRESULT,
    fn get_NeighboringCellsLte(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellLte>) -> HRESULT,
    fn get_NeighboringCellsTdscdma(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellTdscdma>) -> HRESULT,
    fn get_NeighboringCellsUmts(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellUmts>) -> HRESULT,
    fn get_ServingCellsCdma(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellCdma>) -> HRESULT,
    fn get_ServingCellsGsm(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellGsm>) -> HRESULT,
    fn get_ServingCellsLte(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellLte>) -> HRESULT,
    fn get_ServingCellsTdscdma(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellTdscdma>) -> HRESULT,
    fn get_ServingCellsUmts(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandCellUmts>) -> HRESULT
}}
impl IMobileBroadbandCellsInfo {
    #[inline] pub fn get_neighboring_cells_cdma(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellCdma>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringCellsCdma)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_neighboring_cells_gsm(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellGsm>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringCellsGsm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_neighboring_cells_lte(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellLte>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringCellsLte)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_neighboring_cells_tdscdma(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellTdscdma>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringCellsTdscdma)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_neighboring_cells_umts(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellUmts>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringCellsUmts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serving_cells_cdma(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellCdma>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServingCellsCdma)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serving_cells_gsm(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellGsm>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServingCellsGsm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serving_cells_lte(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellLte>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServingCellsLte)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serving_cells_tdscdma(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellTdscdma>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServingCellsTdscdma)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serving_cells_umts(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandCellUmts>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServingCellsUmts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellsInfo: IMobileBroadbandCellsInfo}
DEFINE_IID!(IID_IMobileBroadbandCellTdscdma, 249173589, 56078, 16770, 140, 218, 204, 65, 154, 123, 222, 8);
RT_INTERFACE!{interface IMobileBroadbandCellTdscdma(IMobileBroadbandCellTdscdmaVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellTdscdma] {
    fn get_CellId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_CellParameterId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ChannelNumber(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_LocationAreaCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PathLossInDB(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReceivedSignalCodePowerInDBm(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_TimingAdvanceInBitPeriods(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT
}}
impl IMobileBroadbandCellTdscdma {
    #[inline] pub fn get_cell_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CellId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cell_parameter_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CellParameterId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_channel_number(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChannelNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_location_area_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocationAreaCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_path_loss_in_db(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PathLossInDB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_received_signal_code_power_in_dbm(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReceivedSignalCodePowerInDBm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_timing_advance_in_bit_periods(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TimingAdvanceInBitPeriods)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellTdscdma: IMobileBroadbandCellTdscdma}
DEFINE_IID!(IID_IMobileBroadbandCellUmts, 2008331694, 18888, 20245, 178, 133, 76, 38, 167, 246, 114, 21);
RT_INTERFACE!{interface IMobileBroadbandCellUmts(IMobileBroadbandCellUmtsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandCellUmts] {
    fn get_CellId(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ChannelNumber(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_LocationAreaCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_PathLossInDB(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_PrimaryScramblingCode(&self, out: *mut *mut foundation::IReference<i32>) -> HRESULT,
    fn get_ProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReceivedSignalCodePowerInDBm(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT,
    fn get_SignalToNoiseRatioInDB(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT
}}
impl IMobileBroadbandCellUmts {
    #[inline] pub fn get_cell_id(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CellId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_channel_number(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChannelNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_location_area_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocationAreaCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_path_loss_in_db(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PathLossInDB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_primary_scrambling_code(&self) -> Result<Option<ComPtr<foundation::IReference<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrimaryScramblingCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_received_signal_code_power_in_dbm(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReceivedSignalCodePowerInDBm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signal_to_noise_ratio_in_db(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SignalToNoiseRatioInDB)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandCellUmts: IMobileBroadbandCellUmts}
DEFINE_IID!(IID_IMobileBroadbandDeviceInformation, 3872424296, 58241, 19566, 155, 232, 254, 21, 105, 105, 164, 70);
RT_INTERFACE!{interface IMobileBroadbandDeviceInformation(IMobileBroadbandDeviceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceInformation] {
    fn get_NetworkDeviceStatus(&self, out: *mut NetworkDeviceStatus) -> HRESULT,
    fn get_Manufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Model(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirmwareInformation(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_CellularClass(&self, out: *mut super::super::devices::sms::CellularClass) -> HRESULT,
    fn get_DataClasses(&self, out: *mut DataClasses) -> HRESULT,
    fn get_CustomDataClass(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MobileEquipmentId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TelephoneNumbers(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_SubscriberId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SimIccId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceType(&self, out: *mut MobileBroadbandDeviceType) -> HRESULT,
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CurrentRadioState(&self, out: *mut MobileBroadbandRadioState) -> HRESULT
}}
impl IMobileBroadbandDeviceInformation {
    #[inline] pub fn get_network_device_status(&self) -> Result<NetworkDeviceStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkDeviceStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_manufacturer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Manufacturer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Model)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_firmware_information(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirmwareInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_cellular_class(&self) -> Result<super::super::devices::sms::CellularClass> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CellularClass)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_data_classes(&self) -> Result<DataClasses> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataClasses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_data_class(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomDataClass)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mobile_equipment_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MobileEquipmentId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_telephone_numbers(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TelephoneNumbers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subscriber_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SubscriberId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sim_icc_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimIccId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_type(&self) -> Result<MobileBroadbandDeviceType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_radio_state(&self) -> Result<MobileBroadbandRadioState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentRadioState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceInformation: IMobileBroadbandDeviceInformation}
DEFINE_IID!(IID_IMobileBroadbandDeviceInformation2, 776370929, 63794, 18231, 167, 34, 3, 186, 114, 55, 12, 184);
RT_INTERFACE!{interface IMobileBroadbandDeviceInformation2(IMobileBroadbandDeviceInformation2Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceInformation2] {
    fn get_PinManager(&self, out: *mut *mut MobileBroadbandPinManager) -> HRESULT,
    fn get_Revision(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SerialNumber(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMobileBroadbandDeviceInformation2 {
    #[inline] pub fn get_pin_manager(&self) -> Result<Option<ComPtr<MobileBroadbandPinManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PinManager)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_revision(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Revision)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serial_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SerialNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandDeviceInformation3, 3767252157, 23856, 19290, 146, 204, 213, 77, 248, 129, 212, 158);
RT_INTERFACE!{interface IMobileBroadbandDeviceInformation3(IMobileBroadbandDeviceInformation3Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceInformation3] {
    fn get_SimSpn(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SimPnn(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SimGid1(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMobileBroadbandDeviceInformation3 {
    #[inline] pub fn get_sim_spn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimSpn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sim_pnn(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimPnn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sim_gid1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimGid1)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandDeviceService, 582883922, 48512, 16556, 142, 31, 46, 7, 131, 106, 61, 189);
RT_INTERFACE!{interface IMobileBroadbandDeviceService(IMobileBroadbandDeviceServiceVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceService] {
    fn get_DeviceServiceId(&self, out: *mut Guid) -> HRESULT,
    fn get_SupportedCommands(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn OpenDataSession(&self, out: *mut *mut MobileBroadbandDeviceServiceDataSession) -> HRESULT,
    fn OpenCommandSession(&self, out: *mut *mut MobileBroadbandDeviceServiceCommandSession) -> HRESULT
}}
impl IMobileBroadbandDeviceService {
    #[inline] pub fn get_device_service_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceServiceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_commands(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedCommands)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_data_session(&self) -> Result<Option<ComPtr<MobileBroadbandDeviceServiceDataSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenDataSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_command_session(&self) -> Result<Option<ComPtr<MobileBroadbandDeviceServiceCommandSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).OpenCommandSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceService: IMobileBroadbandDeviceService}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceCommandResult, 2968808123, 38102, 17593, 165, 56, 240, 129, 11, 100, 83, 137);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceCommandResult(IMobileBroadbandDeviceServiceCommandResultVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceCommandResult] {
    fn get_StatusCode(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_ResponseData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceCommandResult {
    #[inline] pub fn get_status_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StatusCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_response_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceCommandResult: IMobileBroadbandDeviceServiceCommandResult}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceCommandSession, 4228483653, 37179, 18708, 182, 195, 174, 99, 4, 89, 62, 117);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceCommandSession(IMobileBroadbandDeviceServiceCommandSessionVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceCommandSession] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn SendQueryCommandAsync(&self, commandId: u32, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn SendSetCommandAsync(&self, commandId: u32, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>) -> HRESULT,
    fn CloseSession(&self) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceCommandSession {
    #[cfg(feature="windows-storage")] #[inline] pub fn send_query_command_async(&self, commandId: u32, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendQueryCommandAsync)(self as *const _ as *mut _, commandId, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn send_set_command_async(&self, commandId: u32, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandDeviceServiceCommandResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendSetCommandAsync)(self as *const _ as *mut _, commandId, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn close_session(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CloseSession)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceCommandSession: IMobileBroadbandDeviceServiceCommandSession}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceDataReceivedEventArgs, 3064599518, 4992, 16611, 134, 24, 115, 203, 202, 72, 19, 140);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceDataReceivedEventArgs(IMobileBroadbandDeviceServiceDataReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceDataReceivedEventArgs] {
    #[cfg(feature="windows-storage")] fn get_ReceivedData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceDataReceivedEventArgs {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_received_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReceivedData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceDataReceivedEventArgs: IMobileBroadbandDeviceServiceDataReceivedEventArgs}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceDataSession, 3671466803, 35791, 17033, 138, 55, 4, 92, 33, 105, 72, 106);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceDataSession(IMobileBroadbandDeviceServiceDataSessionVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceDataSession] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn WriteDataAsync(&self, value: *mut super::super::storage::streams::IBuffer, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn CloseSession(&self) -> HRESULT,
    fn add_DataReceived(&self, eventHandler: *mut foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DataReceived(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceDataSession {
    #[cfg(feature="windows-storage")] #[inline] pub fn write_data_async(&self, value: &super::super::storage::streams::IBuffer) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).WriteDataAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn close_session(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CloseSession)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_data_received(&self, eventHandler: &foundation::TypedEventHandler<MobileBroadbandDeviceServiceDataSession, MobileBroadbandDeviceServiceDataReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DataReceived)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_data_received(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DataReceived)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceDataSession: IMobileBroadbandDeviceServiceDataSession}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceInformation, 1406573403, 50413, 17904, 128, 58, 217, 65, 122, 109, 152, 70);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceInformation(IMobileBroadbandDeviceServiceInformationVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceInformation] {
    fn get_DeviceServiceId(&self, out: *mut Guid) -> HRESULT,
    fn get_IsDataReadSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsDataWriteSupported(&self, out: *mut bool) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceInformation {
    #[inline] pub fn get_device_service_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceServiceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_data_read_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDataReadSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_data_write_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDataWriteSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceInformation: IMobileBroadbandDeviceServiceInformation}
DEFINE_IID!(IID_IMobileBroadbandDeviceServiceTriggerDetails, 1241865072, 47534, 17496, 146, 65, 166, 165, 251, 241, 138, 12);
RT_INTERFACE!{interface IMobileBroadbandDeviceServiceTriggerDetails(IMobileBroadbandDeviceServiceTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandDeviceServiceTriggerDetails] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceServiceId(&self, out: *mut Guid) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_ReceivedData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IMobileBroadbandDeviceServiceTriggerDetails {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_service_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceServiceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_received_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReceivedData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandDeviceServiceTriggerDetails: IMobileBroadbandDeviceServiceTriggerDetails}
RT_ENUM! { enum MobileBroadbandDeviceType: i32 {
    Unknown = 0, Embedded = 1, Removable = 2, Remote = 3,
}}
DEFINE_IID!(IID_IMobileBroadbandModem, 3493161234, 59897, 20327, 160, 61, 67, 24, 154, 49, 107, 241);
RT_INTERFACE!{interface IMobileBroadbandModem(IMobileBroadbandModemVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModem] {
    fn get_CurrentAccount(&self, out: *mut *mut MobileBroadbandAccount) -> HRESULT,
    fn get_DeviceInformation(&self, out: *mut *mut MobileBroadbandDeviceInformation) -> HRESULT,
    fn get_MaxDeviceServiceCommandSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn get_MaxDeviceServiceDataSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn get_DeviceServices(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandDeviceServiceInformation>) -> HRESULT,
    fn GetDeviceService(&self, deviceServiceId: Guid, out: *mut *mut MobileBroadbandDeviceService) -> HRESULT,
    fn get_IsResetSupported(&self, out: *mut bool) -> HRESULT,
    fn ResetAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn GetCurrentConfigurationAsync(&self, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandModemConfiguration>) -> HRESULT,
    fn get_CurrentNetwork(&self, out: *mut *mut MobileBroadbandNetwork) -> HRESULT
}}
impl IMobileBroadbandModem {
    #[inline] pub fn get_current_account(&self) -> Result<Option<ComPtr<MobileBroadbandAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentAccount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_information(&self) -> Result<Option<ComPtr<MobileBroadbandDeviceInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_device_service_command_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxDeviceServiceCommandSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_device_service_data_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxDeviceServiceDataSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_services(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandDeviceServiceInformation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceServices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_service(&self, deviceServiceId: Guid) -> Result<Option<ComPtr<MobileBroadbandDeviceService>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeviceService)(self as *const _ as *mut _, deviceServiceId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_reset_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsResetSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn reset_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ResetAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_configuration_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandModemConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentConfigurationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_network(&self) -> Result<Option<ComPtr<MobileBroadbandNetwork>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentNetwork)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandModem: IMobileBroadbandModem}
impl RtActivatable<IMobileBroadbandModemStatics> for MobileBroadbandModem {}
impl MobileBroadbandModem {
    #[inline] pub fn get_device_selector() -> Result<HString> {
        <Self as RtActivatable<IMobileBroadbandModemStatics>>::get_activation_factory().get_device_selector()
    }
    #[inline] pub fn from_id(deviceId: &HStringArg) -> Result<Option<ComPtr<MobileBroadbandModem>>> {
        <Self as RtActivatable<IMobileBroadbandModemStatics>>::get_activation_factory().from_id(deviceId)
    }
    #[inline] pub fn get_default() -> Result<Option<ComPtr<MobileBroadbandModem>>> {
        <Self as RtActivatable<IMobileBroadbandModemStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(MobileBroadbandModem(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,77,111,98,105,108,101,66,114,111,97,100,98,97,110,100,77,111,100,101,109,0]) [CLSID_MobileBroadbandModem]);
DEFINE_IID!(IID_IMobileBroadbandModem2, 310782760, 47595, 20194, 187, 227, 113, 31, 83, 238, 163, 115);
RT_INTERFACE!{interface IMobileBroadbandModem2(IMobileBroadbandModem2Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModem2] {
    fn GetIsPassthroughEnabledAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn SetIsPassthroughEnabledAsync(&self, value: bool, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandModemStatus>) -> HRESULT
}}
impl IMobileBroadbandModem2 {
    #[inline] pub fn get_is_passthrough_enabled_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIsPassthroughEnabledAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_is_passthrough_enabled_async(&self, value: bool) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandModemStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetIsPassthroughEnabledAsync)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandModem3, 3925788394, 12084, 17794, 145, 2, 195, 20, 210, 168, 126, 236);
RT_INTERFACE!{interface IMobileBroadbandModem3(IMobileBroadbandModem3Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModem3] {
    fn TryGetPcoAsync(&self, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPco>) -> HRESULT,
    fn get_IsInEmergencyCallMode(&self, out: *mut bool) -> HRESULT,
    fn add_IsInEmergencyCallModeChanged(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandModem, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsInEmergencyCallModeChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IMobileBroadbandModem3 {
    #[inline] pub fn try_get_pco_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPco>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetPcoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_in_emergency_call_mode(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInEmergencyCallMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_is_in_emergency_call_mode_changed(&self, handler: &foundation::TypedEventHandler<MobileBroadbandModem, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsInEmergencyCallModeChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_in_emergency_call_mode_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_IsInEmergencyCallModeChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandModemConfiguration, 4242552227, 54989, 17184, 185, 130, 190, 157, 62, 199, 137, 15);
RT_INTERFACE!{interface IMobileBroadbandModemConfiguration(IMobileBroadbandModemConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModemConfiguration] {
    fn get_Uicc(&self, out: *mut *mut MobileBroadbandUicc) -> HRESULT,
    fn get_HomeProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HomeProviderName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMobileBroadbandModemConfiguration {
    #[inline] pub fn get_uicc(&self) -> Result<Option<ComPtr<MobileBroadbandUicc>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uicc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_home_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_home_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandModemConfiguration: IMobileBroadbandModemConfiguration}
DEFINE_IID!(IID_IMobileBroadbandModemConfiguration2, 839906757, 58464, 17070, 170, 81, 105, 98, 30, 122, 68, 119);
RT_INTERFACE!{interface IMobileBroadbandModemConfiguration2(IMobileBroadbandModemConfiguration2Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModemConfiguration2] {
    fn get_SarManager(&self, out: *mut *mut MobileBroadbandSarManager) -> HRESULT
}}
impl IMobileBroadbandModemConfiguration2 {
    #[inline] pub fn get_sar_manager(&self) -> Result<Option<ComPtr<MobileBroadbandSarManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SarManager)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandModemIsolation, 3043069932, 58977, 17200, 155, 180, 52, 128, 33, 46, 195, 84);
RT_INTERFACE!{interface IMobileBroadbandModemIsolation(IMobileBroadbandModemIsolationVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModemIsolation] {
    fn AddAllowedHost(&self, host: *mut super::HostName) -> HRESULT,
    fn AddAllowedHostRange(&self, first: *mut super::HostName, last: *mut super::HostName) -> HRESULT,
    fn ApplyConfigurationAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ClearConfigurationAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IMobileBroadbandModemIsolation {
    #[inline] pub fn add_allowed_host(&self, host: &super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddAllowedHost)(self as *const _ as *mut _, host as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_allowed_host_range(&self, first: &super::HostName, last: &super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddAllowedHostRange)(self as *const _ as *mut _, first as *const _ as *mut _, last as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn apply_configuration_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ApplyConfigurationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_configuration_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ClearConfigurationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandModemIsolation: IMobileBroadbandModemIsolation}
impl RtActivatable<IMobileBroadbandModemIsolationFactory> for MobileBroadbandModemIsolation {}
impl MobileBroadbandModemIsolation {
    #[inline] pub fn create(modemDeviceId: &HStringArg, ruleGroupId: &HStringArg) -> Result<ComPtr<MobileBroadbandModemIsolation>> {
        <Self as RtActivatable<IMobileBroadbandModemIsolationFactory>>::get_activation_factory().create(modemDeviceId, ruleGroupId)
    }
}
DEFINE_CLSID!(MobileBroadbandModemIsolation(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,77,111,98,105,108,101,66,114,111,97,100,98,97,110,100,77,111,100,101,109,73,115,111,108,97,116,105,111,110,0]) [CLSID_MobileBroadbandModemIsolation]);
DEFINE_IID!(IID_IMobileBroadbandModemIsolationFactory, 567798872, 49841, 19503, 160, 48, 114, 130, 10, 36, 236, 217);
RT_INTERFACE!{static interface IMobileBroadbandModemIsolationFactory(IMobileBroadbandModemIsolationFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModemIsolationFactory] {
    fn Create(&self, modemDeviceId: HSTRING, ruleGroupId: HSTRING, out: *mut *mut MobileBroadbandModemIsolation) -> HRESULT
}}
impl IMobileBroadbandModemIsolationFactory {
    #[inline] pub fn create(&self, modemDeviceId: &HStringArg, ruleGroupId: &HStringArg) -> Result<ComPtr<MobileBroadbandModemIsolation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, modemDeviceId.get(), ruleGroupId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandModemStatics, 4187936311, 55025, 19064, 140, 188, 100, 33, 166, 80, 99, 200);
RT_INTERFACE!{static interface IMobileBroadbandModemStatics(IMobileBroadbandModemStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandModemStatics] {
    fn GetDeviceSelector(&self, out: *mut HSTRING) -> HRESULT,
    fn FromId(&self, deviceId: HSTRING, out: *mut *mut MobileBroadbandModem) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut MobileBroadbandModem) -> HRESULT
}}
impl IMobileBroadbandModemStatics {
    #[inline] pub fn get_device_selector(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeviceSelector)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_id(&self, deviceId: &HStringArg) -> Result<Option<ComPtr<MobileBroadbandModem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromId)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<MobileBroadbandModem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum MobileBroadbandModemStatus: i32 {
    Success = 0, OtherFailure = 1, Busy = 2, NoDeviceSupport = 3,
}}
DEFINE_IID!(IID_IMobileBroadbandNetwork, 3412300428, 777, 19638, 168, 193, 106, 90, 60, 142, 31, 246);
RT_INTERFACE!{interface IMobileBroadbandNetwork(IMobileBroadbandNetworkVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandNetwork] {
    fn get_NetworkAdapter(&self, out: *mut *mut super::connectivity::NetworkAdapter) -> HRESULT,
    fn get_NetworkRegistrationState(&self, out: *mut NetworkRegistrationState) -> HRESULT,
    fn get_RegistrationNetworkError(&self, out: *mut u32) -> HRESULT,
    fn get_PacketAttachNetworkError(&self, out: *mut u32) -> HRESULT,
    fn get_ActivationNetworkError(&self, out: *mut u32) -> HRESULT,
    fn get_AccessPointName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RegisteredDataClass(&self, out: *mut DataClasses) -> HRESULT,
    fn get_RegisteredProviderId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RegisteredProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn ShowConnectionUI(&self) -> HRESULT
}}
impl IMobileBroadbandNetwork {
    #[inline] pub fn get_network_adapter(&self) -> Result<Option<ComPtr<super::connectivity::NetworkAdapter>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAdapter)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network_registration_state(&self) -> Result<NetworkRegistrationState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkRegistrationState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_registration_network_error(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RegistrationNetworkError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_packet_attach_network_error(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PacketAttachNetworkError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_activation_network_error(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActivationNetworkError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_access_point_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccessPointName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_registered_data_class(&self) -> Result<DataClasses> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RegisteredDataClass)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_registered_provider_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RegisteredProviderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_registered_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RegisteredProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_connection_ui(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ShowConnectionUI)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandNetwork: IMobileBroadbandNetwork}
DEFINE_IID!(IID_IMobileBroadbandNetwork2, 1515576098, 25335, 19421, 186, 29, 71, 116, 65, 150, 11, 160);
RT_INTERFACE!{interface IMobileBroadbandNetwork2(IMobileBroadbandNetwork2Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandNetwork2] {
    fn GetVoiceCallSupportAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_RegistrationUiccApps(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandUiccApp>) -> HRESULT
}}
impl IMobileBroadbandNetwork2 {
    #[inline] pub fn get_voice_call_support_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVoiceCallSupportAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_registration_uicc_apps(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandUiccApp>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RegistrationUiccApps)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandNetwork3, 862390922, 51183, 17484, 171, 108, 223, 126, 247, 163, 144, 254);
RT_INTERFACE!{interface IMobileBroadbandNetwork3(IMobileBroadbandNetwork3Vtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandNetwork3] {
    fn GetCellsInfoAsync(&self, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandCellsInfo>) -> HRESULT
}}
impl IMobileBroadbandNetwork3 {
    #[inline] pub fn get_cells_info_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandCellsInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCellsInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMobileBroadbandNetworkRegistrationStateChange, 3199177953, 38415, 18868, 160, 141, 125, 133, 233, 104, 199, 236);
RT_INTERFACE!{interface IMobileBroadbandNetworkRegistrationStateChange(IMobileBroadbandNetworkRegistrationStateChangeVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandNetworkRegistrationStateChange] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Network(&self, out: *mut *mut MobileBroadbandNetwork) -> HRESULT
}}
impl IMobileBroadbandNetworkRegistrationStateChange {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_network(&self) -> Result<Option<ComPtr<MobileBroadbandNetwork>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Network)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandNetworkRegistrationStateChange: IMobileBroadbandNetworkRegistrationStateChange}
DEFINE_IID!(IID_IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails, 2299747583, 10424, 18090, 177, 55, 28, 75, 15, 33, 237, 254);
RT_INTERFACE!{interface IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails(IMobileBroadbandNetworkRegistrationStateChangeTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails] {
    fn get_NetworkRegistrationStateChanges(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>) -> HRESULT
}}
impl IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    #[inline] pub fn get_network_registration_state_changes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandNetworkRegistrationStateChange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkRegistrationStateChanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandNetworkRegistrationStateChangeTriggerDetails: IMobileBroadbandNetworkRegistrationStateChangeTriggerDetails}
DEFINE_IID!(IID_IMobileBroadbandPco, 3571776702, 58275, 17349, 168, 123, 108, 134, 210, 41, 215, 250);
RT_INTERFACE!{interface IMobileBroadbandPco(IMobileBroadbandPcoVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPco] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_IsComplete(&self, out: *mut bool) -> HRESULT,
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMobileBroadbandPco {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_complete(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsComplete)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPco: IMobileBroadbandPco}
DEFINE_IID!(IID_IMobileBroadbandPcoDataChangeTriggerDetails, 641683732, 25824, 17555, 144, 155, 45, 20, 160, 25, 98, 177);
RT_INTERFACE!{interface IMobileBroadbandPcoDataChangeTriggerDetails(IMobileBroadbandPcoDataChangeTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPcoDataChangeTriggerDetails] {
    fn get_UpdatedData(&self, out: *mut *mut MobileBroadbandPco) -> HRESULT
}}
impl IMobileBroadbandPcoDataChangeTriggerDetails {
    #[inline] pub fn get_updated_data(&self) -> Result<Option<ComPtr<MobileBroadbandPco>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UpdatedData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPcoDataChangeTriggerDetails: IMobileBroadbandPcoDataChangeTriggerDetails}
DEFINE_IID!(IID_IMobileBroadbandPin, 3865171721, 59257, 17855, 130, 129, 117, 50, 61, 249, 227, 33);
RT_INTERFACE!{interface IMobileBroadbandPin(IMobileBroadbandPinVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPin] {
    fn get_Type(&self, out: *mut MobileBroadbandPinType) -> HRESULT,
    fn get_LockState(&self, out: *mut MobileBroadbandPinLockState) -> HRESULT,
    fn get_Format(&self, out: *mut MobileBroadbandPinFormat) -> HRESULT,
    fn get_Enabled(&self, out: *mut bool) -> HRESULT,
    fn get_MaxLength(&self, out: *mut u32) -> HRESULT,
    fn get_MinLength(&self, out: *mut u32) -> HRESULT,
    fn get_AttemptsRemaining(&self, out: *mut u32) -> HRESULT,
    fn EnableAsync(&self, currentPin: HSTRING, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPinOperationResult>) -> HRESULT,
    fn DisableAsync(&self, currentPin: HSTRING, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPinOperationResult>) -> HRESULT,
    fn EnterAsync(&self, currentPin: HSTRING, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPinOperationResult>) -> HRESULT,
    fn ChangeAsync(&self, currentPin: HSTRING, newPin: HSTRING, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPinOperationResult>) -> HRESULT,
    fn UnblockAsync(&self, pinUnblockKey: HSTRING, newPin: HSTRING, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandPinOperationResult>) -> HRESULT
}}
impl IMobileBroadbandPin {
    #[inline] pub fn get_type(&self) -> Result<MobileBroadbandPinType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_lock_state(&self) -> Result<MobileBroadbandPinLockState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LockState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_format(&self) -> Result<MobileBroadbandPinFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Format)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Enabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_attempts_remaining(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AttemptsRemaining)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn enable_async(&self, currentPin: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EnableAsync)(self as *const _ as *mut _, currentPin.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn disable_async(&self, currentPin: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DisableAsync)(self as *const _ as *mut _, currentPin.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enter_async(&self, currentPin: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EnterAsync)(self as *const _ as *mut _, currentPin.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn change_async(&self, currentPin: &HStringArg, newPin: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ChangeAsync)(self as *const _ as *mut _, currentPin.get(), newPin.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn unblock_async(&self, pinUnblockKey: &HStringArg, newPin: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandPinOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnblockAsync)(self as *const _ as *mut _, pinUnblockKey.get(), newPin.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPin: IMobileBroadbandPin}
RT_ENUM! { enum MobileBroadbandPinFormat: i32 {
    Unknown = 0, Numeric = 1, Alphanumeric = 2,
}}
RT_ENUM! { enum MobileBroadbandPinLockState: i32 {
    Unknown = 0, Unlocked = 1, PinRequired = 2, PinUnblockKeyRequired = 3,
}}
DEFINE_IID!(IID_IMobileBroadbandPinLockStateChange, 3189139262, 7940, 20373, 139, 144, 231, 245, 89, 221, 231, 229);
RT_INTERFACE!{interface IMobileBroadbandPinLockStateChange(IMobileBroadbandPinLockStateChangeVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPinLockStateChange] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PinType(&self, out: *mut MobileBroadbandPinType) -> HRESULT,
    fn get_PinLockState(&self, out: *mut MobileBroadbandPinLockState) -> HRESULT
}}
impl IMobileBroadbandPinLockStateChange {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pin_type(&self) -> Result<MobileBroadbandPinType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PinType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pin_lock_state(&self) -> Result<MobileBroadbandPinLockState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PinLockState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPinLockStateChange: IMobileBroadbandPinLockStateChange}
DEFINE_IID!(IID_IMobileBroadbandPinLockStateChangeTriggerDetails, 3543711889, 16017, 19768, 144, 54, 174, 232, 58, 110, 121, 173);
RT_INTERFACE!{interface IMobileBroadbandPinLockStateChangeTriggerDetails(IMobileBroadbandPinLockStateChangeTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPinLockStateChangeTriggerDetails] {
    fn get_PinLockStateChanges(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandPinLockStateChange>) -> HRESULT
}}
impl IMobileBroadbandPinLockStateChangeTriggerDetails {
    #[inline] pub fn get_pin_lock_state_changes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandPinLockStateChange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PinLockStateChanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPinLockStateChangeTriggerDetails: IMobileBroadbandPinLockStateChangeTriggerDetails}
DEFINE_IID!(IID_IMobileBroadbandPinManager, 2203483869, 28191, 19355, 164, 19, 43, 31, 80, 204, 54, 223);
RT_INTERFACE!{interface IMobileBroadbandPinManager(IMobileBroadbandPinManagerVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPinManager] {
    fn get_SupportedPins(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandPinType>) -> HRESULT,
    fn GetPin(&self, pinType: MobileBroadbandPinType, out: *mut *mut MobileBroadbandPin) -> HRESULT
}}
impl IMobileBroadbandPinManager {
    #[inline] pub fn get_supported_pins(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandPinType>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedPins)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pin(&self, pinType: MobileBroadbandPinType) -> Result<Option<ComPtr<MobileBroadbandPin>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPin)(self as *const _ as *mut _, pinType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPinManager: IMobileBroadbandPinManager}
DEFINE_IID!(IID_IMobileBroadbandPinOperationResult, 299752498, 12775, 18933, 182, 99, 18, 61, 59, 239, 3, 98);
RT_INTERFACE!{interface IMobileBroadbandPinOperationResult(IMobileBroadbandPinOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandPinOperationResult] {
    fn get_IsSuccessful(&self, out: *mut bool) -> HRESULT,
    fn get_AttemptsRemaining(&self, out: *mut u32) -> HRESULT
}}
impl IMobileBroadbandPinOperationResult {
    #[inline] pub fn get_is_successful(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSuccessful)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_attempts_remaining(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AttemptsRemaining)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandPinOperationResult: IMobileBroadbandPinOperationResult}
RT_ENUM! { enum MobileBroadbandPinType: i32 {
    None = 0, Custom = 1, Pin1 = 2, Pin2 = 3, SimPin = 4, FirstSimPin = 5, NetworkPin = 6, NetworkSubsetPin = 7, ServiceProviderPin = 8, CorporatePin = 9, SubsidyLock = 10,
}}
RT_ENUM! { enum MobileBroadbandRadioState: i32 {
    Off = 0, On = 1,
}}
DEFINE_IID!(IID_IMobileBroadbandRadioStateChange, 2958337377, 38963, 19181, 151, 23, 67, 72, 178, 26, 36, 179);
RT_INTERFACE!{interface IMobileBroadbandRadioStateChange(IMobileBroadbandRadioStateChangeVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandRadioStateChange] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RadioState(&self, out: *mut MobileBroadbandRadioState) -> HRESULT
}}
impl IMobileBroadbandRadioStateChange {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_radio_state(&self) -> Result<MobileBroadbandRadioState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RadioState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandRadioStateChange: IMobileBroadbandRadioStateChange}
DEFINE_IID!(IID_IMobileBroadbandRadioStateChangeTriggerDetails, 1898977998, 2364, 17094, 176, 219, 173, 31, 117, 166, 84, 69);
RT_INTERFACE!{interface IMobileBroadbandRadioStateChangeTriggerDetails(IMobileBroadbandRadioStateChangeTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandRadioStateChangeTriggerDetails] {
    fn get_RadioStateChanges(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandRadioStateChange>) -> HRESULT
}}
impl IMobileBroadbandRadioStateChangeTriggerDetails {
    #[inline] pub fn get_radio_state_changes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandRadioStateChange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RadioStateChanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandRadioStateChangeTriggerDetails: IMobileBroadbandRadioStateChangeTriggerDetails}
DEFINE_IID!(IID_IMobileBroadbandSarManager, 3853674547, 38526, 16585, 164, 133, 25, 192, 221, 32, 158, 34);
RT_INTERFACE!{interface IMobileBroadbandSarManager(IMobileBroadbandSarManagerVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandSarManager] {
    fn get_IsBackoffEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_IsWiFiHardwareIntegrated(&self, out: *mut bool) -> HRESULT,
    fn get_IsSarControlledByHardware(&self, out: *mut bool) -> HRESULT,
    fn get_Antennas(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandAntennaSar>) -> HRESULT,
    fn get_HysteresisTimerPeriod(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn add_TransmissionStateChanged(&self, handler: *mut foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TransmissionStateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn EnableBackoffAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn DisableBackoffAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn SetConfigurationAsync(&self, antennas: *mut foundation::collections::IIterable<MobileBroadbandAntennaSar>, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn RevertSarToHardwareControlAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn SetTransmissionStateChangedHysteresisAsync(&self, timerPeriod: foundation::TimeSpan, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn GetIsTransmittingAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn StartTransmissionStateMonitoring(&self) -> HRESULT,
    fn StopTransmissionStateMonitoring(&self) -> HRESULT
}}
impl IMobileBroadbandSarManager {
    #[inline] pub fn get_is_backoff_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsBackoffEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_wi_fi_hardware_integrated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWiFiHardwareIntegrated)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_sar_controlled_by_hardware(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSarControlledByHardware)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_antennas(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandAntennaSar>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Antennas)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hysteresis_timer_period(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HysteresisTimerPeriod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_transmission_state_changed(&self, handler: &foundation::TypedEventHandler<MobileBroadbandSarManager, MobileBroadbandTransmissionStateChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_TransmissionStateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_transmission_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_TransmissionStateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn enable_backoff_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EnableBackoffAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn disable_backoff_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DisableBackoffAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_configuration_async(&self, antennas: &foundation::collections::IIterable<MobileBroadbandAntennaSar>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetConfigurationAsync)(self as *const _ as *mut _, antennas as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn revert_sar_to_hardware_control_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RevertSarToHardwareControlAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_transmission_state_changed_hysteresis_async(&self, timerPeriod: foundation::TimeSpan) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetTransmissionStateChangedHysteresisAsync)(self as *const _ as *mut _, timerPeriod, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_transmitting_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetIsTransmittingAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_transmission_state_monitoring(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartTransmissionStateMonitoring)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_transmission_state_monitoring(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StopTransmissionStateMonitoring)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandSarManager: IMobileBroadbandSarManager}
DEFINE_IID!(IID_IMobileBroadbandTransmissionStateChangedEventArgs, 1630419061, 1034, 20377, 164, 249, 97, 215, 195, 45, 161, 41);
RT_INTERFACE!{interface IMobileBroadbandTransmissionStateChangedEventArgs(IMobileBroadbandTransmissionStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandTransmissionStateChangedEventArgs] {
    fn get_IsTransmitting(&self, out: *mut bool) -> HRESULT
}}
impl IMobileBroadbandTransmissionStateChangedEventArgs {
    #[inline] pub fn get_is_transmitting(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTransmitting)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandTransmissionStateChangedEventArgs: IMobileBroadbandTransmissionStateChangedEventArgs}
DEFINE_IID!(IID_IMobileBroadbandUicc, 3862230673, 21082, 19682, 143, 206, 170, 65, 98, 87, 145, 84);
RT_INTERFACE!{interface IMobileBroadbandUicc(IMobileBroadbandUiccVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandUicc] {
    fn get_SimIccId(&self, out: *mut HSTRING) -> HRESULT,
    fn GetUiccAppsAsync(&self, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>) -> HRESULT
}}
impl IMobileBroadbandUicc {
    #[inline] pub fn get_sim_icc_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SimIccId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_uicc_apps_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandUiccAppsResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUiccAppsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandUicc: IMobileBroadbandUicc}
DEFINE_IID!(IID_IMobileBroadbandUiccApp, 1293354326, 39073, 17373, 178, 236, 80, 201, 12, 242, 72, 223);
RT_INTERFACE!{interface IMobileBroadbandUiccApp(IMobileBroadbandUiccAppVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandUiccApp] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Id(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_Kind(&self, out: *mut UiccAppKind) -> HRESULT,
    fn GetRecordDetailsAsync(&self, uiccFilePath: *mut foundation::collections::IIterable<u32>, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>) -> HRESULT,
    fn ReadRecordAsync(&self, uiccFilePath: *mut foundation::collections::IIterable<u32>, recordIndex: i32, out: *mut *mut foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>) -> HRESULT
}}
impl IMobileBroadbandUiccApp {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_id(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<UiccAppKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_record_details_async(&self, uiccFilePath: &foundation::collections::IIterable<u32>) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandUiccAppRecordDetailsResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetRecordDetailsAsync)(self as *const _ as *mut _, uiccFilePath as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_record_async(&self, uiccFilePath: &foundation::collections::IIterable<u32>, recordIndex: i32) -> Result<ComPtr<foundation::IAsyncOperation<MobileBroadbandUiccAppReadRecordResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadRecordAsync)(self as *const _ as *mut _, uiccFilePath as *const _ as *mut _, recordIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandUiccApp: IMobileBroadbandUiccApp}
RT_ENUM! { enum MobileBroadbandUiccAppOperationStatus: i32 {
    Success = 0, InvalidUiccFilePath = 1, AccessConditionNotHeld = 2, UiccBusy = 3,
}}
DEFINE_IID!(IID_IMobileBroadbandUiccAppReadRecordResult, 1690915461, 13710, 18373, 130, 73, 105, 95, 56, 59, 43, 219);
RT_INTERFACE!{interface IMobileBroadbandUiccAppReadRecordResult(IMobileBroadbandUiccAppReadRecordResultVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandUiccAppReadRecordResult] {
    fn get_Status(&self, out: *mut MobileBroadbandUiccAppOperationStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IMobileBroadbandUiccAppReadRecordResult {
    #[inline] pub fn get_status(&self) -> Result<MobileBroadbandUiccAppOperationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandUiccAppReadRecordResult: IMobileBroadbandUiccAppReadRecordResult}
DEFINE_IID!(IID_IMobileBroadbandUiccAppRecordDetailsResult, 3642320943, 48660, 18740, 152, 29, 47, 87, 185, 237, 131, 230);
RT_INTERFACE!{interface IMobileBroadbandUiccAppRecordDetailsResult(IMobileBroadbandUiccAppRecordDetailsResultVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandUiccAppRecordDetailsResult] {
    fn get_Status(&self, out: *mut MobileBroadbandUiccAppOperationStatus) -> HRESULT,
    fn get_Kind(&self, out: *mut UiccAppRecordKind) -> HRESULT,
    fn get_RecordCount(&self, out: *mut i32) -> HRESULT,
    fn get_RecordSize(&self, out: *mut i32) -> HRESULT,
    fn get_ReadAccessCondition(&self, out: *mut UiccAccessCondition) -> HRESULT,
    fn get_WriteAccessCondition(&self, out: *mut UiccAccessCondition) -> HRESULT
}}
impl IMobileBroadbandUiccAppRecordDetailsResult {
    #[inline] pub fn get_status(&self) -> Result<MobileBroadbandUiccAppOperationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<UiccAppRecordKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_record_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecordCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_record_size(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecordSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_read_access_condition(&self) -> Result<UiccAccessCondition> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReadAccessCondition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_write_access_condition(&self) -> Result<UiccAccessCondition> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WriteAccessCondition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandUiccAppRecordDetailsResult: IMobileBroadbandUiccAppRecordDetailsResult}
DEFINE_IID!(IID_IMobileBroadbandUiccAppsResult, 1950953707, 33111, 19009, 132, 148, 107, 245, 76, 155, 29, 43);
RT_INTERFACE!{interface IMobileBroadbandUiccAppsResult(IMobileBroadbandUiccAppsResultVtbl): IInspectable(IInspectableVtbl) [IID_IMobileBroadbandUiccAppsResult] {
    fn get_Status(&self, out: *mut MobileBroadbandUiccAppOperationStatus) -> HRESULT,
    fn get_UiccApps(&self, out: *mut *mut foundation::collections::IVectorView<MobileBroadbandUiccApp>) -> HRESULT
}}
impl IMobileBroadbandUiccAppsResult {
    #[inline] pub fn get_status(&self) -> Result<MobileBroadbandUiccAppOperationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_uicc_apps(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<MobileBroadbandUiccApp>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UiccApps)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MobileBroadbandUiccAppsResult: IMobileBroadbandUiccAppsResult}
RT_ENUM! { enum NetworkDeviceStatus: i32 {
    DeviceNotReady = 0, DeviceReady = 1, SimNotInserted = 2, BadSim = 3, DeviceHardwareFailure = 4, AccountNotActivated = 5, DeviceLocked = 6, DeviceBlocked = 7,
}}
RT_ENUM! { enum NetworkOperatorDataUsageNotificationKind: i32 {
    DataUsageProgress = 0,
}}
DEFINE_IID!(IID_INetworkOperatorDataUsageTriggerDetails, 1357058669, 42085, 20203, 147, 23, 40, 161, 103, 99, 12, 234);
RT_INTERFACE!{interface INetworkOperatorDataUsageTriggerDetails(INetworkOperatorDataUsageTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorDataUsageTriggerDetails] {
    fn get_NotificationKind(&self, out: *mut NetworkOperatorDataUsageNotificationKind) -> HRESULT
}}
impl INetworkOperatorDataUsageTriggerDetails {
    #[inline] pub fn get_notification_kind(&self) -> Result<NetworkOperatorDataUsageNotificationKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NotificationKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorDataUsageTriggerDetails: INetworkOperatorDataUsageTriggerDetails}
RT_ENUM! { enum NetworkOperatorEventMessageType: i32 {
    Gsm = 0, Cdma = 1, Ussd = 2, DataPlanThresholdReached = 3, DataPlanReset = 4, DataPlanDeleted = 5, ProfileConnected = 6, ProfileDisconnected = 7, RegisteredRoaming = 8, RegisteredHome = 9, TetheringEntitlementCheck = 10, TetheringOperationalStateChanged = 11, TetheringNumberOfClientsChanged = 12,
}}
DEFINE_IID!(IID_INetworkOperatorNotificationEventDetails, 3160975825, 33505, 17544, 159, 44, 18, 118, 194, 70, 143, 172);
RT_INTERFACE!{interface INetworkOperatorNotificationEventDetails(INetworkOperatorNotificationEventDetailsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorNotificationEventDetails] {
    fn get_NotificationType(&self, out: *mut NetworkOperatorEventMessageType) -> HRESULT,
    fn get_NetworkAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EncodingType(&self, out: *mut u8) -> HRESULT,
    fn get_Message(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RuleId(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-devices")] fn get_SmsMessage(&self, out: *mut *mut super::super::devices::sms::ISmsMessage) -> HRESULT
}}
impl INetworkOperatorNotificationEventDetails {
    #[inline] pub fn get_notification_type(&self) -> Result<NetworkOperatorEventMessageType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NotificationType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_encoding_type(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EncodingType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rule_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RuleId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_sms_message(&self) -> Result<Option<ComPtr<super::super::devices::sms::ISmsMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SmsMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorNotificationEventDetails: INetworkOperatorNotificationEventDetails}
DEFINE_IID!(IID_INetworkOperatorTetheringAccessPointConfiguration, 197919364, 16686, 16445, 172, 198, 183, 87, 227, 71, 116, 164);
RT_INTERFACE!{interface INetworkOperatorTetheringAccessPointConfiguration(INetworkOperatorTetheringAccessPointConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringAccessPointConfiguration] {
    fn get_Ssid(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Ssid(&self, value: HSTRING) -> HRESULT,
    fn get_Passphrase(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Passphrase(&self, value: HSTRING) -> HRESULT
}}
impl INetworkOperatorTetheringAccessPointConfiguration {
    #[inline] pub fn get_ssid(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ssid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_ssid(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Ssid)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_passphrase(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Passphrase)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_passphrase(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Passphrase)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorTetheringAccessPointConfiguration: INetworkOperatorTetheringAccessPointConfiguration}
impl RtActivatable<IActivationFactory> for NetworkOperatorTetheringAccessPointConfiguration {}
DEFINE_CLSID!(NetworkOperatorTetheringAccessPointConfiguration(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,84,101,116,104,101,114,105,110,103,65,99,99,101,115,115,80,111,105,110,116,67,111,110,102,105,103,117,114,97,116,105,111,110,0]) [CLSID_NetworkOperatorTetheringAccessPointConfiguration]);
DEFINE_IID!(IID_INetworkOperatorTetheringClient, 1889346892, 22879, 18503, 187, 48, 100, 105, 53, 84, 41, 24);
RT_INTERFACE!{interface INetworkOperatorTetheringClient(INetworkOperatorTetheringClientVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringClient] {
    fn get_MacAddress(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HostNames(&self, out: *mut *mut foundation::collections::IVectorView<super::HostName>) -> HRESULT
}}
impl INetworkOperatorTetheringClient {
    #[inline] pub fn get_mac_address(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MacAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_host_names(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HostNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorTetheringClient: INetworkOperatorTetheringClient}
DEFINE_IID!(IID_INetworkOperatorTetheringClientManager, 2444312598, 36298, 16933, 187, 237, 238, 248, 184, 215, 24, 215);
RT_INTERFACE!{interface INetworkOperatorTetheringClientManager(INetworkOperatorTetheringClientManagerVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringClientManager] {
    fn GetTetheringClients(&self, out: *mut *mut foundation::collections::IVectorView<NetworkOperatorTetheringClient>) -> HRESULT
}}
impl INetworkOperatorTetheringClientManager {
    #[inline] pub fn get_tethering_clients(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<NetworkOperatorTetheringClient>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTetheringClients)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkOperatorTetheringEntitlementCheck, 17338733, 40602, 19190, 141, 163, 96, 73, 59, 25, 194, 4);
RT_INTERFACE!{interface INetworkOperatorTetheringEntitlementCheck(INetworkOperatorTetheringEntitlementCheckVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringEntitlementCheck] {
    fn AuthorizeTethering(&self, allow: bool, entitlementFailureReason: HSTRING) -> HRESULT
}}
impl INetworkOperatorTetheringEntitlementCheck {
    #[inline] pub fn authorize_tethering(&self, allow: bool, entitlementFailureReason: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AuthorizeTethering)(self as *const _ as *mut _, allow, entitlementFailureReason.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkOperatorTetheringManager, 3562704288, 3718, 19864, 139, 164, 221, 112, 212, 183, 100, 211);
RT_INTERFACE!{interface INetworkOperatorTetheringManager(INetworkOperatorTetheringManagerVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringManager] {
    fn get_MaxClientCount(&self, out: *mut u32) -> HRESULT,
    fn get_ClientCount(&self, out: *mut u32) -> HRESULT,
    fn get_TetheringOperationalState(&self, out: *mut TetheringOperationalState) -> HRESULT,
    fn GetCurrentAccessPointConfiguration(&self, out: *mut *mut NetworkOperatorTetheringAccessPointConfiguration) -> HRESULT,
    fn ConfigureAccessPointAsync(&self, configuration: *mut NetworkOperatorTetheringAccessPointConfiguration, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn StartTetheringAsync(&self, out: *mut *mut foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>) -> HRESULT,
    fn StopTetheringAsync(&self, out: *mut *mut foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>) -> HRESULT
}}
impl INetworkOperatorTetheringManager {
    #[inline] pub fn get_max_client_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxClientCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_client_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ClientCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tethering_operational_state(&self) -> Result<TetheringOperationalState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TetheringOperationalState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_access_point_configuration(&self) -> Result<Option<ComPtr<NetworkOperatorTetheringAccessPointConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentAccessPointConfiguration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn configure_access_point_async(&self, configuration: &NetworkOperatorTetheringAccessPointConfiguration) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConfigureAccessPointAsync)(self as *const _ as *mut _, configuration as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_tethering_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartTetheringAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn stop_tethering_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<NetworkOperatorTetheringOperationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StopTetheringAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorTetheringManager: INetworkOperatorTetheringManager}
impl RtActivatable<INetworkOperatorTetheringManagerStatics> for NetworkOperatorTetheringManager {}
impl RtActivatable<INetworkOperatorTetheringManagerStatics2> for NetworkOperatorTetheringManager {}
impl RtActivatable<INetworkOperatorTetheringManagerStatics3> for NetworkOperatorTetheringManager {}
impl NetworkOperatorTetheringManager {
    #[inline] pub fn get_tethering_capability(networkAccountId: &HStringArg) -> Result<TetheringCapability> {
        <Self as RtActivatable<INetworkOperatorTetheringManagerStatics>>::get_activation_factory().get_tethering_capability(networkAccountId)
    }
    #[inline] pub fn create_from_network_account_id(networkAccountId: &HStringArg) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> {
        <Self as RtActivatable<INetworkOperatorTetheringManagerStatics>>::get_activation_factory().create_from_network_account_id(networkAccountId)
    }
    #[inline] pub fn get_tethering_capability_from_connection_profile(profile: &super::connectivity::ConnectionProfile) -> Result<TetheringCapability> {
        <Self as RtActivatable<INetworkOperatorTetheringManagerStatics2>>::get_activation_factory().get_tethering_capability_from_connection_profile(profile)
    }
    #[inline] pub fn create_from_connection_profile(profile: &super::connectivity::ConnectionProfile) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> {
        <Self as RtActivatable<INetworkOperatorTetheringManagerStatics2>>::get_activation_factory().create_from_connection_profile(profile)
    }
    #[inline] pub fn create_from_connection_profile_with_target_adapter(profile: &super::connectivity::ConnectionProfile, adapter: &super::connectivity::NetworkAdapter) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> {
        <Self as RtActivatable<INetworkOperatorTetheringManagerStatics3>>::get_activation_factory().create_from_connection_profile_with_target_adapter(profile, adapter)
    }
}
DEFINE_CLSID!(NetworkOperatorTetheringManager(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,84,101,116,104,101,114,105,110,103,77,97,110,97,103,101,114,0]) [CLSID_NetworkOperatorTetheringManager]);
DEFINE_IID!(IID_INetworkOperatorTetheringManagerStatics, 1052555980, 63683, 16476, 153, 100, 112, 161, 238, 171, 225, 148);
RT_INTERFACE!{static interface INetworkOperatorTetheringManagerStatics(INetworkOperatorTetheringManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringManagerStatics] {
    fn GetTetheringCapability(&self, networkAccountId: HSTRING, out: *mut TetheringCapability) -> HRESULT,
    fn CreateFromNetworkAccountId(&self, networkAccountId: HSTRING, out: *mut *mut NetworkOperatorTetheringManager) -> HRESULT
}}
impl INetworkOperatorTetheringManagerStatics {
    #[inline] pub fn get_tethering_capability(&self, networkAccountId: &HStringArg) -> Result<TetheringCapability> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetTetheringCapability)(self as *const _ as *mut _, networkAccountId.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn create_from_network_account_id(&self, networkAccountId: &HStringArg) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNetworkAccountId)(self as *const _ as *mut _, networkAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkOperatorTetheringManagerStatics2, 1529041938, 13808, 18919, 155, 8, 22, 210, 120, 251, 170, 66);
RT_INTERFACE!{static interface INetworkOperatorTetheringManagerStatics2(INetworkOperatorTetheringManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringManagerStatics2] {
    fn GetTetheringCapabilityFromConnectionProfile(&self, profile: *mut super::connectivity::ConnectionProfile, out: *mut TetheringCapability) -> HRESULT,
    fn CreateFromConnectionProfile(&self, profile: *mut super::connectivity::ConnectionProfile, out: *mut *mut NetworkOperatorTetheringManager) -> HRESULT
}}
impl INetworkOperatorTetheringManagerStatics2 {
    #[inline] pub fn get_tethering_capability_from_connection_profile(&self, profile: &super::connectivity::ConnectionProfile) -> Result<TetheringCapability> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetTetheringCapabilityFromConnectionProfile)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn create_from_connection_profile(&self, profile: &super::connectivity::ConnectionProfile) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromConnectionProfile)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkOperatorTetheringManagerStatics3, 2413473206, 19193, 20257, 155, 88, 213, 62, 159, 36, 35, 30);
RT_INTERFACE!{static interface INetworkOperatorTetheringManagerStatics3(INetworkOperatorTetheringManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringManagerStatics3] {
    fn CreateFromConnectionProfileWithTargetAdapter(&self, profile: *mut super::connectivity::ConnectionProfile, adapter: *mut super::connectivity::NetworkAdapter, out: *mut *mut NetworkOperatorTetheringManager) -> HRESULT
}}
impl INetworkOperatorTetheringManagerStatics3 {
    #[inline] pub fn create_from_connection_profile_with_target_adapter(&self, profile: &super::connectivity::ConnectionProfile, adapter: &super::connectivity::NetworkAdapter) -> Result<Option<ComPtr<NetworkOperatorTetheringManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromConnectionProfileWithTargetAdapter)(self as *const _ as *mut _, profile as *const _ as *mut _, adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_INetworkOperatorTetheringOperationResult, 3956409249, 442, 18285, 180, 179, 191, 61, 18, 200, 248, 12);
RT_INTERFACE!{interface INetworkOperatorTetheringOperationResult(INetworkOperatorTetheringOperationResultVtbl): IInspectable(IInspectableVtbl) [IID_INetworkOperatorTetheringOperationResult] {
    fn get_Status(&self, out: *mut TetheringOperationStatus) -> HRESULT,
    fn get_AdditionalErrorMessage(&self, out: *mut HSTRING) -> HRESULT
}}
impl INetworkOperatorTetheringOperationResult {
    #[inline] pub fn get_status(&self) -> Result<TetheringOperationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_additional_error_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdditionalErrorMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class NetworkOperatorTetheringOperationResult: INetworkOperatorTetheringOperationResult}
RT_ENUM! { enum NetworkRegistrationState: i32 {
    None = 0, Deregistered = 1, Searching = 2, Home = 3, Roaming = 4, Partner = 5, Denied = 6,
}}
RT_ENUM! { enum ProfileMediaType: i32 {
    Wlan = 0, Wwan = 1,
}}
RT_STRUCT! { struct ProfileUsage {
    UsageInMegabytes: u32, LastSyncTime: foundation::DateTime,
}}
DEFINE_IID!(IID_IProvisionedProfile, 561447136, 33282, 4575, 173, 185, 244, 206, 70, 45, 145, 55);
RT_INTERFACE!{interface IProvisionedProfile(IProvisionedProfileVtbl): IInspectable(IInspectableVtbl) [IID_IProvisionedProfile] {
    fn UpdateCost(&self, value: super::connectivity::NetworkCostType) -> HRESULT,
    fn UpdateUsage(&self, value: ProfileUsage) -> HRESULT
}}
impl IProvisionedProfile {
    #[inline] pub fn update_cost(&self, value: super::connectivity::NetworkCostType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateCost)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_usage(&self, value: ProfileUsage) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateUsage)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ProvisionedProfile: IProvisionedProfile}
DEFINE_IID!(IID_IProvisionFromXmlDocumentResults, 561447136, 33283, 4575, 173, 185, 244, 206, 70, 45, 145, 55);
RT_INTERFACE!{interface IProvisionFromXmlDocumentResults(IProvisionFromXmlDocumentResultsVtbl): IInspectable(IInspectableVtbl) [IID_IProvisionFromXmlDocumentResults] {
    fn get_AllElementsProvisioned(&self, out: *mut bool) -> HRESULT,
    fn get_ProvisionResultsXml(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProvisionFromXmlDocumentResults {
    #[inline] pub fn get_all_elements_provisioned(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllElementsProvisioned)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_provision_results_xml(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProvisionResultsXml)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProvisionFromXmlDocumentResults: IProvisionFromXmlDocumentResults}
DEFINE_IID!(IID_IProvisioningAgent, 561447136, 33281, 4575, 173, 185, 244, 206, 70, 45, 145, 55);
RT_INTERFACE!{interface IProvisioningAgent(IProvisioningAgentVtbl): IInspectable(IInspectableVtbl) [IID_IProvisioningAgent] {
    fn ProvisionFromXmlDocumentAsync(&self, provisioningXmlDocument: HSTRING, out: *mut *mut foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>) -> HRESULT,
    fn GetProvisionedProfile(&self, mediaType: ProfileMediaType, profileName: HSTRING, out: *mut *mut ProvisionedProfile) -> HRESULT
}}
impl IProvisioningAgent {
    #[inline] pub fn provision_from_xml_document_async(&self, provisioningXmlDocument: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ProvisionFromXmlDocumentResults>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ProvisionFromXmlDocumentAsync)(self as *const _ as *mut _, provisioningXmlDocument.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provisioned_profile(&self, mediaType: ProfileMediaType, profileName: &HStringArg) -> Result<Option<ComPtr<ProvisionedProfile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProvisionedProfile)(self as *const _ as *mut _, mediaType, profileName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProvisioningAgent: IProvisioningAgent}
impl RtActivatable<IProvisioningAgentStaticMethods> for ProvisioningAgent {}
impl RtActivatable<IActivationFactory> for ProvisioningAgent {}
impl ProvisioningAgent {
    #[inline] pub fn create_from_network_account_id(networkAccountId: &HStringArg) -> Result<Option<ComPtr<ProvisioningAgent>>> {
        <Self as RtActivatable<IProvisioningAgentStaticMethods>>::get_activation_factory().create_from_network_account_id(networkAccountId)
    }
}
DEFINE_CLSID!(ProvisioningAgent(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,80,114,111,118,105,115,105,111,110,105,110,103,65,103,101,110,116,0]) [CLSID_ProvisioningAgent]);
DEFINE_IID!(IID_IProvisioningAgentStaticMethods, 561447136, 33025, 4575, 173, 185, 244, 206, 70, 45, 145, 55);
RT_INTERFACE!{static interface IProvisioningAgentStaticMethods(IProvisioningAgentStaticMethodsVtbl): IInspectable(IInspectableVtbl) [IID_IProvisioningAgentStaticMethods] {
    fn CreateFromNetworkAccountId(&self, networkAccountId: HSTRING, out: *mut *mut ProvisioningAgent) -> HRESULT
}}
impl IProvisioningAgentStaticMethods {
    #[inline] pub fn create_from_network_account_id(&self, networkAccountId: &HStringArg) -> Result<Option<ComPtr<ProvisioningAgent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNetworkAccountId)(self as *const _ as *mut _, networkAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TetheringCapability: i32 {
    Enabled = 0, DisabledByGroupPolicy = 1, DisabledByHardwareLimitation = 2, DisabledByOperator = 3, DisabledBySku = 4, DisabledByRequiredAppNotInstalled = 5, DisabledDueToUnknownCause = 6, DisabledBySystemCapability = 7,
}}
DEFINE_IID!(IID_ITetheringEntitlementCheckTriggerDetails, 63331997, 22822, 16883, 169, 78, 181, 9, 38, 252, 66, 27);
RT_INTERFACE!{interface ITetheringEntitlementCheckTriggerDetails(ITetheringEntitlementCheckTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_ITetheringEntitlementCheckTriggerDetails] {
    fn get_NetworkAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn AllowTethering(&self) -> HRESULT,
    fn DenyTethering(&self, entitlementFailureReason: HSTRING) -> HRESULT
}}
impl ITetheringEntitlementCheckTriggerDetails {
    #[inline] pub fn get_network_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NetworkAccountId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn allow_tethering(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AllowTethering)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn deny_tethering(&self, entitlementFailureReason: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).DenyTethering)(self as *const _ as *mut _, entitlementFailureReason.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class TetheringEntitlementCheckTriggerDetails: ITetheringEntitlementCheckTriggerDetails}
RT_ENUM! { enum TetheringOperationalState: i32 {
    Unknown = 0, On = 1, Off = 2, InTransition = 3,
}}
RT_ENUM! { enum TetheringOperationStatus: i32 {
    Success = 0, Unknown = 1, MobileBroadbandDeviceOff = 2, WiFiDeviceOff = 3, EntitlementCheckTimeout = 4, EntitlementCheckFailure = 5, OperationInProgress = 6, BluetoothDeviceOff = 7, NetworkLimitedConnectivity = 8,
}}
RT_ENUM! { enum UiccAccessCondition: i32 {
    AlwaysAllowed = 0, Pin1 = 1, Pin2 = 2, Pin3 = 3, Pin4 = 4, Administrative5 = 5, Administrative6 = 6, NeverAllowed = 7,
}}
RT_ENUM! { enum UiccAppKind: i32 {
    Unknown = 0, MF = 1, MFSim = 2, MFRuim = 3, USim = 4, CSim = 5, ISim = 6,
}}
RT_ENUM! { enum UiccAppRecordKind: i32 {
    Unknown = 0, Transparent = 1, RecordOriented = 2,
}}
DEFINE_IID!(IID_IUssdMessage, 798674818, 8196, 19805, 191, 129, 42, 186, 27, 75, 228, 168);
RT_INTERFACE!{interface IUssdMessage(IUssdMessageVtbl): IInspectable(IInspectableVtbl) [IID_IUssdMessage] {
    fn get_DataCodingScheme(&self, out: *mut u8) -> HRESULT,
    fn put_DataCodingScheme(&self, value: u8) -> HRESULT,
    fn GetPayload(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn SetPayload(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn get_PayloadAsText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PayloadAsText(&self, value: HSTRING) -> HRESULT
}}
impl IUssdMessage {
    #[inline] pub fn get_data_coding_scheme(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataCodingScheme)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_data_coding_scheme(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DataCodingScheme)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_payload(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPayload)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn set_payload(&self, value: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetPayload)(self as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_payload_as_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PayloadAsText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_payload_as_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PayloadAsText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class UssdMessage: IUssdMessage}
impl RtActivatable<IUssdMessageFactory> for UssdMessage {}
impl UssdMessage {
    #[inline] pub fn create_message(messageText: &HStringArg) -> Result<ComPtr<UssdMessage>> {
        <Self as RtActivatable<IUssdMessageFactory>>::get_activation_factory().create_message(messageText)
    }
}
DEFINE_CLSID!(UssdMessage(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,85,115,115,100,77,101,115,115,97,103,101,0]) [CLSID_UssdMessage]);
DEFINE_IID!(IID_IUssdMessageFactory, 798674818, 4099, 19805, 191, 129, 42, 186, 27, 75, 228, 168);
RT_INTERFACE!{static interface IUssdMessageFactory(IUssdMessageFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IUssdMessageFactory] {
    fn CreateMessage(&self, messageText: HSTRING, out: *mut *mut UssdMessage) -> HRESULT
}}
impl IUssdMessageFactory {
    #[inline] pub fn create_message(&self, messageText: &HStringArg) -> Result<ComPtr<UssdMessage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateMessage)(self as *const _ as *mut _, messageText.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUssdReply, 798674818, 8197, 19805, 191, 129, 42, 186, 27, 75, 228, 168);
RT_INTERFACE!{interface IUssdReply(IUssdReplyVtbl): IInspectable(IInspectableVtbl) [IID_IUssdReply] {
    fn get_ResultCode(&self, out: *mut UssdResultCode) -> HRESULT,
    fn get_Message(&self, out: *mut *mut UssdMessage) -> HRESULT
}}
impl IUssdReply {
    #[inline] pub fn get_result_code(&self) -> Result<UssdResultCode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResultCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<Option<ComPtr<UssdMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UssdReply: IUssdReply}
RT_ENUM! { enum UssdResultCode: i32 {
    NoActionRequired = 0, ActionRequired = 1, Terminated = 2, OtherLocalClient = 3, OperationNotSupported = 4, NetworkTimeout = 5,
}}
DEFINE_IID!(IID_IUssdSession, 798674818, 8194, 19805, 191, 129, 42, 186, 27, 75, 228, 168);
RT_INTERFACE!{interface IUssdSession(IUssdSessionVtbl): IInspectable(IInspectableVtbl) [IID_IUssdSession] {
    fn SendMessageAndGetReplyAsync(&self, message: *mut UssdMessage, out: *mut *mut foundation::IAsyncOperation<UssdReply>) -> HRESULT,
    fn Close(&self) -> HRESULT
}}
impl IUssdSession {
    #[inline] pub fn send_message_and_get_reply_async(&self, message: &UssdMessage) -> Result<ComPtr<foundation::IAsyncOperation<UssdReply>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendMessageAndGetReplyAsync)(self as *const _ as *mut _, message as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn close(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Close)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class UssdSession: IUssdSession}
impl RtActivatable<IUssdSessionStatics> for UssdSession {}
impl UssdSession {
    #[inline] pub fn create_from_network_account_id(networkAccountId: &HStringArg) -> Result<Option<ComPtr<UssdSession>>> {
        <Self as RtActivatable<IUssdSessionStatics>>::get_activation_factory().create_from_network_account_id(networkAccountId)
    }
    #[inline] pub fn create_from_network_interface_id(networkInterfaceId: &HStringArg) -> Result<Option<ComPtr<UssdSession>>> {
        <Self as RtActivatable<IUssdSessionStatics>>::get_activation_factory().create_from_network_interface_id(networkInterfaceId)
    }
}
DEFINE_CLSID!(UssdSession(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,78,101,116,119,111,114,107,79,112,101,114,97,116,111,114,115,46,85,115,115,100,83,101,115,115,105,111,110,0]) [CLSID_UssdSession]);
DEFINE_IID!(IID_IUssdSessionStatics, 798674818, 4097, 19805, 191, 129, 42, 186, 27, 75, 228, 168);
RT_INTERFACE!{static interface IUssdSessionStatics(IUssdSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUssdSessionStatics] {
    fn CreateFromNetworkAccountId(&self, networkAccountId: HSTRING, out: *mut *mut UssdSession) -> HRESULT,
    fn CreateFromNetworkInterfaceId(&self, networkInterfaceId: HSTRING, out: *mut *mut UssdSession) -> HRESULT
}}
impl IUssdSessionStatics {
    #[inline] pub fn create_from_network_account_id(&self, networkAccountId: &HStringArg) -> Result<Option<ComPtr<UssdSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNetworkAccountId)(self as *const _ as *mut _, networkAccountId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_network_interface_id(&self, networkInterfaceId: &HStringArg) -> Result<Option<ComPtr<UssdSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNetworkInterfaceId)(self as *const _ as *mut _, networkInterfaceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.Networking.NetworkOperators
pub mod proximity { // Windows.Networking.Proximity
use crate::prelude::*;
DEFINE_IID!(IID_IConnectionRequestedEventArgs, 3949498798, 20254, 19558, 189, 13, 70, 146, 74, 148, 46, 8);
RT_INTERFACE!{interface IConnectionRequestedEventArgs(IConnectionRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IConnectionRequestedEventArgs] {
    fn get_PeerInformation(&self, out: *mut *mut PeerInformation) -> HRESULT
}}
impl IConnectionRequestedEventArgs {
    #[inline] pub fn get_peer_information(&self) -> Result<Option<ComPtr<PeerInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PeerInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ConnectionRequestedEventArgs: IConnectionRequestedEventArgs}
DEFINE_IID!(IID_DeviceArrivedEventHandler, 4020886121, 63201, 18889, 164, 158, 142, 15, 197, 143, 185, 17);
RT_DELEGATE!{delegate DeviceArrivedEventHandler(DeviceArrivedEventHandlerVtbl, DeviceArrivedEventHandlerImpl) [IID_DeviceArrivedEventHandler] {
    fn Invoke(&self, sender: *mut ProximityDevice) -> HRESULT
}}
impl DeviceArrivedEventHandler {
    #[inline] pub fn invoke(&self, sender: &ProximityDevice) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_DeviceDepartedEventHandler, 4020886121, 63202, 18889, 164, 158, 142, 15, 197, 143, 185, 17);
RT_DELEGATE!{delegate DeviceDepartedEventHandler(DeviceDepartedEventHandlerVtbl, DeviceDepartedEventHandlerImpl) [IID_DeviceDepartedEventHandler] {
    fn Invoke(&self, sender: *mut ProximityDevice) -> HRESULT
}}
impl DeviceDepartedEventHandler {
    #[inline] pub fn invoke(&self, sender: &ProximityDevice) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_MessageReceivedHandler, 4020963202, 63202, 18037, 160, 69, 216, 227, 32, 194, 72, 8);
RT_DELEGATE!{delegate MessageReceivedHandler(MessageReceivedHandlerVtbl, MessageReceivedHandlerImpl) [IID_MessageReceivedHandler] {
    fn Invoke(&self, sender: *mut ProximityDevice, message: *mut ProximityMessage) -> HRESULT
}}
impl MessageReceivedHandler {
    #[inline] pub fn invoke(&self, sender: &ProximityDevice, message: &ProximityMessage) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _, message as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_MessageTransmittedHandler, 4020898634, 63202, 19837, 133, 108, 120, 252, 142, 252, 2, 30);
RT_DELEGATE!{delegate MessageTransmittedHandler(MessageTransmittedHandlerVtbl, MessageTransmittedHandlerImpl) [IID_MessageTransmittedHandler] {
    fn Invoke(&self, sender: *mut ProximityDevice, messageId: i64) -> HRESULT
}}
impl MessageTransmittedHandler {
    #[inline] pub fn invoke(&self, sender: &ProximityDevice, messageId: i64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _, messageId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum PeerDiscoveryTypes: u32 {
    None = 0, Browse = 1, Triggered = 2,
}}
RT_CLASS!{static class PeerFinder}
impl RtActivatable<IPeerFinderStatics> for PeerFinder {}
impl RtActivatable<IPeerFinderStatics2> for PeerFinder {}
impl PeerFinder {
    #[inline] pub fn get_allow_bluetooth() -> Result<bool> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_allow_bluetooth()
    }
    #[inline] pub fn set_allow_bluetooth(value: bool) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().set_allow_bluetooth(value)
    }
    #[inline] pub fn get_allow_infrastructure() -> Result<bool> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_allow_infrastructure()
    }
    #[inline] pub fn set_allow_infrastructure(value: bool) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().set_allow_infrastructure(value)
    }
    #[inline] pub fn get_allow_wi_fi_direct() -> Result<bool> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_allow_wi_fi_direct()
    }
    #[inline] pub fn set_allow_wi_fi_direct(value: bool) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().set_allow_wi_fi_direct(value)
    }
    #[inline] pub fn get_display_name() -> Result<HString> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_display_name()
    }
    #[inline] pub fn set_display_name(value: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().set_display_name(value)
    }
    #[inline] pub fn get_supported_discovery_types() -> Result<PeerDiscoveryTypes> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_supported_discovery_types()
    }
    #[inline] pub fn get_alternate_identities() -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().get_alternate_identities()
    }
    #[inline] pub fn start() -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().start()
    }
    #[inline] pub fn start_with_message(peerMessage: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().start_with_message(peerMessage)
    }
    #[inline] pub fn stop() -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().stop()
    }
    #[inline] pub fn add_triggered_connection_state_changed(handler: &foundation::TypedEventHandler<IInspectable, TriggeredConnectionStateChangedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().add_triggered_connection_state_changed(handler)
    }
    #[inline] pub fn remove_triggered_connection_state_changed(cookie: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().remove_triggered_connection_state_changed(cookie)
    }
    #[inline] pub fn add_connection_requested(handler: &foundation::TypedEventHandler<IInspectable, ConnectionRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().add_connection_requested(handler)
    }
    #[inline] pub fn remove_connection_requested(cookie: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().remove_connection_requested(cookie)
    }
    #[inline] pub fn find_all_peers_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<PeerInformation>>>> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().find_all_peers_async()
    }
    #[inline] pub fn connect_async(peerInformation: &PeerInformation) -> Result<ComPtr<foundation::IAsyncOperation<super::sockets::StreamSocket>>> {
        <Self as RtActivatable<IPeerFinderStatics>>::get_activation_factory().connect_async(peerInformation)
    }
    #[inline] pub fn get_role() -> Result<PeerRole> {
        <Self as RtActivatable<IPeerFinderStatics2>>::get_activation_factory().get_role()
    }
    #[inline] pub fn set_role(value: PeerRole) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics2>>::get_activation_factory().set_role(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_discovery_data() -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> {
        <Self as RtActivatable<IPeerFinderStatics2>>::get_activation_factory().get_discovery_data()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_discovery_data(value: &super::super::storage::streams::IBuffer) -> Result<()> {
        <Self as RtActivatable<IPeerFinderStatics2>>::get_activation_factory().set_discovery_data(value)
    }
    #[inline] pub fn create_watcher() -> Result<Option<ComPtr<PeerWatcher>>> {
        <Self as RtActivatable<IPeerFinderStatics2>>::get_activation_factory().create_watcher()
    }
}
DEFINE_CLSID!(PeerFinder(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,80,114,111,120,105,109,105,116,121,46,80,101,101,114,70,105,110,100,101,114,0]) [CLSID_PeerFinder]);
DEFINE_IID!(IID_IPeerFinderStatics, 2437626721, 63201, 18372, 161, 76, 20, 138, 25, 3, 208, 198);
RT_INTERFACE!{static interface IPeerFinderStatics(IPeerFinderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPeerFinderStatics] {
    fn get_AllowBluetooth(&self, out: *mut bool) -> HRESULT,
    fn put_AllowBluetooth(&self, value: bool) -> HRESULT,
    fn get_AllowInfrastructure(&self, out: *mut bool) -> HRESULT,
    fn put_AllowInfrastructure(&self, value: bool) -> HRESULT,
    fn get_AllowWiFiDirect(&self, out: *mut bool) -> HRESULT,
    fn put_AllowWiFiDirect(&self, value: bool) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_SupportedDiscoveryTypes(&self, out: *mut PeerDiscoveryTypes) -> HRESULT,
    fn get_AlternateIdentities(&self, out: *mut *mut foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn StartWithMessage(&self, peerMessage: HSTRING) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_TriggeredConnectionStateChanged(&self, handler: *mut foundation::TypedEventHandler<IInspectable, TriggeredConnectionStateChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TriggeredConnectionStateChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ConnectionRequested(&self, handler: *mut foundation::TypedEventHandler<IInspectable, ConnectionRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ConnectionRequested(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn FindAllPeersAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<PeerInformation>>) -> HRESULT,
    fn ConnectAsync(&self, peerInformation: *mut PeerInformation, out: *mut *mut foundation::IAsyncOperation<super::sockets::StreamSocket>) -> HRESULT
}}
impl IPeerFinderStatics {
    #[inline] pub fn get_allow_bluetooth(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowBluetooth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_bluetooth(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowBluetooth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_infrastructure(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowInfrastructure)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_infrastructure(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowInfrastructure)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_wi_fi_direct(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowWiFiDirect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_wi_fi_direct(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowWiFiDirect)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_discovery_types(&self) -> Result<PeerDiscoveryTypes> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SupportedDiscoveryTypes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_alternate_identities(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AlternateIdentities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_with_message(&self, peerMessage: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartWithMessage)(self as *const _ as *mut _, peerMessage.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_triggered_connection_state_changed(&self, handler: &foundation::TypedEventHandler<IInspectable, TriggeredConnectionStateChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_TriggeredConnectionStateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_triggered_connection_state_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_TriggeredConnectionStateChanged)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_connection_requested(&self, handler: &foundation::TypedEventHandler<IInspectable, ConnectionRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ConnectionRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_connection_requested(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ConnectionRequested)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn find_all_peers_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<PeerInformation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllPeersAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_async(&self, peerInformation: &PeerInformation) -> Result<ComPtr<foundation::IAsyncOperation<super::sockets::StreamSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectAsync)(self as *const _ as *mut _, peerInformation as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPeerFinderStatics2, 3605478501, 64976, 19211, 147, 18, 134, 100, 8, 147, 93, 130);
RT_INTERFACE!{static interface IPeerFinderStatics2(IPeerFinderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IPeerFinderStatics2] {
    fn get_Role(&self, out: *mut PeerRole) -> HRESULT,
    fn put_Role(&self, value: PeerRole) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_DiscoveryData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_DiscoveryData(&self, value: *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn CreateWatcher(&self, out: *mut *mut PeerWatcher) -> HRESULT
}}
impl IPeerFinderStatics2 {
    #[inline] pub fn get_role(&self) -> Result<PeerRole> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Role)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_role(&self, value: PeerRole) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Role)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_discovery_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DiscoveryData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_discovery_data(&self, value: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DiscoveryData)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_watcher(&self) -> Result<Option<ComPtr<PeerWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPeerInformation, 537022216, 40959, 17908, 182, 233, 64, 139, 46, 190, 243, 115);
RT_INTERFACE!{interface IPeerInformation(IPeerInformationVtbl): IInspectable(IInspectableVtbl) [IID_IPeerInformation] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPeerInformation {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PeerInformation: IPeerInformation}
DEFINE_IID!(IID_IPeerInformation3, 2987352362, 56272, 16632, 149, 189, 45, 66, 9, 199, 131, 111);
RT_INTERFACE!{interface IPeerInformation3(IPeerInformation3Vtbl): IInspectable(IInspectableVtbl) [IID_IPeerInformation3] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_DiscoveryData(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IPeerInformation3 {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_discovery_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DiscoveryData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPeerInformationWithHostAndService, 3972517037, 7024, 20107, 146, 219, 187, 231, 129, 65, 147, 8);
RT_INTERFACE!{interface IPeerInformationWithHostAndService(IPeerInformationWithHostAndServiceVtbl): IInspectable(IInspectableVtbl) [IID_IPeerInformationWithHostAndService] {
    fn get_HostName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_ServiceName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPeerInformationWithHostAndService {
    #[inline] pub fn get_host_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_service_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServiceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum PeerRole: i32 {
    Peer = 0, Host = 1, Client = 2,
}}
DEFINE_IID!(IID_IPeerWatcher, 1022239224, 12198, 18041, 150, 145, 3, 201, 74, 66, 15, 52);
RT_INTERFACE!{interface IPeerWatcher(IPeerWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IPeerWatcher] {
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<PeerWatcher, PeerInformation>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<PeerWatcher, PeerInformation>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut foundation::TypedEventHandler<PeerWatcher, PeerInformation>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<PeerWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<PeerWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut PeerWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IPeerWatcher {
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<PeerWatcher, PeerInformation>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<PeerWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<PeerWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<PeerWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PeerWatcher: IPeerWatcher}
RT_ENUM! { enum PeerWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
DEFINE_IID!(IID_IProximityDevice, 4020806994, 63201, 17193, 160, 252, 171, 107, 15, 210, 130, 98);
RT_INTERFACE!{interface IProximityDevice(IProximityDeviceVtbl): IInspectable(IInspectableVtbl) [IID_IProximityDevice] {
    fn SubscribeForMessage(&self, messageType: HSTRING, messageReceivedHandler: *mut MessageReceivedHandler, out: *mut i64) -> HRESULT,
    fn PublishMessage(&self, messageType: HSTRING, message: HSTRING, out: *mut i64) -> HRESULT,
    fn PublishMessageWithCallback(&self, messageType: HSTRING, message: HSTRING, messageTransmittedHandler: *mut MessageTransmittedHandler, out: *mut i64) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn PublishBinaryMessage(&self, messageType: HSTRING, message: *mut super::super::storage::streams::IBuffer, out: *mut i64) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn PublishBinaryMessageWithCallback(&self, messageType: HSTRING, message: *mut super::super::storage::streams::IBuffer, messageTransmittedHandler: *mut MessageTransmittedHandler, out: *mut i64) -> HRESULT,
    fn PublishUriMessage(&self, message: *mut foundation::Uri, out: *mut i64) -> HRESULT,
    fn PublishUriMessageWithCallback(&self, message: *mut foundation::Uri, messageTransmittedHandler: *mut MessageTransmittedHandler, out: *mut i64) -> HRESULT,
    fn StopSubscribingForMessage(&self, subscriptionId: i64) -> HRESULT,
    fn StopPublishingMessage(&self, messageId: i64) -> HRESULT,
    fn add_DeviceArrived(&self, arrivedHandler: *mut DeviceArrivedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DeviceArrived(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DeviceDeparted(&self, departedHandler: *mut DeviceDepartedEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DeviceDeparted(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn get_MaxMessageBytes(&self, out: *mut u32) -> HRESULT,
    fn get_BitsPerSecond(&self, out: *mut u64) -> HRESULT,
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProximityDevice {
    #[inline] pub fn subscribe_for_message(&self, messageType: &HStringArg, messageReceivedHandler: &MessageReceivedHandler) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).SubscribeForMessage)(self as *const _ as *mut _, messageType.get(), messageReceivedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn publish_message(&self, messageType: &HStringArg, message: &HStringArg) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishMessage)(self as *const _ as *mut _, messageType.get(), message.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn publish_message_with_callback(&self, messageType: &HStringArg, message: &HStringArg, messageTransmittedHandler: &MessageTransmittedHandler) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishMessageWithCallback)(self as *const _ as *mut _, messageType.get(), message.get(), messageTransmittedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn publish_binary_message(&self, messageType: &HStringArg, message: &super::super::storage::streams::IBuffer) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishBinaryMessage)(self as *const _ as *mut _, messageType.get(), message as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn publish_binary_message_with_callback(&self, messageType: &HStringArg, message: &super::super::storage::streams::IBuffer, messageTransmittedHandler: &MessageTransmittedHandler) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishBinaryMessageWithCallback)(self as *const _ as *mut _, messageType.get(), message as *const _ as *mut _, messageTransmittedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn publish_uri_message(&self, message: &foundation::Uri) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishUriMessage)(self as *const _ as *mut _, message as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn publish_uri_message_with_callback(&self, message: &foundation::Uri, messageTransmittedHandler: &MessageTransmittedHandler) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PublishUriMessageWithCallback)(self as *const _ as *mut _, message as *const _ as *mut _, messageTransmittedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn stop_subscribing_for_message(&self, subscriptionId: i64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StopSubscribingForMessage)(self as *const _ as *mut _, subscriptionId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_publishing_message(&self, messageId: i64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StopPublishingMessage)(self as *const _ as *mut _, messageId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_device_arrived(&self, arrivedHandler: &DeviceArrivedEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DeviceArrived)(self as *const _ as *mut _, arrivedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_device_arrived(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DeviceArrived)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_device_departed(&self, departedHandler: &DeviceDepartedEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DeviceDeparted)(self as *const _ as *mut _, departedHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_device_departed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DeviceDeparted)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_message_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxMessageBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bits_per_second(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitsPerSecond)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProximityDevice: IProximityDevice}
impl RtActivatable<IProximityDeviceStatics> for ProximityDevice {}
impl ProximityDevice {
    #[inline] pub fn get_device_selector() -> Result<HString> {
        <Self as RtActivatable<IProximityDeviceStatics>>::get_activation_factory().get_device_selector()
    }
    #[inline] pub fn get_default() -> Result<Option<ComPtr<ProximityDevice>>> {
        <Self as RtActivatable<IProximityDeviceStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn from_id(deviceId: &HStringArg) -> Result<Option<ComPtr<ProximityDevice>>> {
        <Self as RtActivatable<IProximityDeviceStatics>>::get_activation_factory().from_id(deviceId)
    }
}
DEFINE_CLSID!(ProximityDevice(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,80,114,111,120,105,109,105,116,121,46,80,114,111,120,105,109,105,116,121,68,101,118,105,99,101,0]) [CLSID_ProximityDevice]);
DEFINE_IID!(IID_IProximityDeviceStatics, 2437652509, 63201, 18372, 161, 76, 20, 138, 25, 3, 208, 198);
RT_INTERFACE!{static interface IProximityDeviceStatics(IProximityDeviceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProximityDeviceStatics] {
    fn GetDeviceSelector(&self, out: *mut HSTRING) -> HRESULT,
    fn GetDefault(&self, out: *mut *mut ProximityDevice) -> HRESULT,
    fn FromId(&self, deviceId: HSTRING, out: *mut *mut ProximityDevice) -> HRESULT
}}
impl IProximityDeviceStatics {
    #[inline] pub fn get_device_selector(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeviceSelector)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<ProximityDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_id(&self, deviceId: &HStringArg) -> Result<Option<ComPtr<ProximityDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FromId)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProximityMessage, 4020963202, 63201, 18037, 160, 69, 216, 227, 32, 194, 72, 8);
RT_INTERFACE!{interface IProximityMessage(IProximityMessageVtbl): IInspectable(IInspectableVtbl) [IID_IProximityMessage] {
    fn get_MessageType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SubscriptionId(&self, out: *mut i64) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_DataAsString(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProximityMessage {
    #[inline] pub fn get_message_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MessageType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subscription_id(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SubscriptionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DataAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProximityMessage: IProximityMessage}
DEFINE_IID!(IID_ITriggeredConnectionStateChangedEventArgs, 3332866221, 63201, 19796, 150, 226, 51, 246, 32, 188, 168, 138);
RT_INTERFACE!{interface ITriggeredConnectionStateChangedEventArgs(ITriggeredConnectionStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITriggeredConnectionStateChangedEventArgs] {
    fn get_State(&self, out: *mut TriggeredConnectState) -> HRESULT,
    fn get_Id(&self, out: *mut u32) -> HRESULT,
    fn get_Socket(&self, out: *mut *mut super::sockets::StreamSocket) -> HRESULT
}}
impl ITriggeredConnectionStateChangedEventArgs {
    #[inline] pub fn get_state(&self) -> Result<TriggeredConnectState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_socket(&self) -> Result<Option<ComPtr<super::sockets::StreamSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Socket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TriggeredConnectionStateChangedEventArgs: ITriggeredConnectionStateChangedEventArgs}
RT_ENUM! { enum TriggeredConnectState: i32 {
    PeerFound = 0, Listening = 1, Connecting = 2, Completed = 3, Canceled = 4, Failed = 5,
}}
} // Windows.Networking.Proximity
pub mod pushnotifications { // Windows.Networking.PushNotifications
use crate::prelude::*;
DEFINE_IID!(IID_IPushNotificationChannel, 724045870, 61195, 20281, 155, 138, 163, 193, 148, 222, 112, 129);
RT_INTERFACE!{interface IPushNotificationChannel(IPushNotificationChannelVtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannel] {
    fn get_Uri(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ExpirationTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn Close(&self) -> HRESULT,
    fn add_PushNotificationReceived(&self, handler: *mut foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PushNotificationReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPushNotificationChannel {
    #[inline] pub fn get_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Uri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_expiration_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpirationTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn close(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Close)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_push_notification_received(&self, handler: &foundation::TypedEventHandler<PushNotificationChannel, PushNotificationReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PushNotificationReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_push_notification_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_PushNotificationReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PushNotificationChannel: IPushNotificationChannel}
RT_CLASS!{static class PushNotificationChannelManager}
impl RtActivatable<IPushNotificationChannelManagerStatics> for PushNotificationChannelManager {}
impl RtActivatable<IPushNotificationChannelManagerStatics2> for PushNotificationChannelManager {}
impl RtActivatable<IPushNotificationChannelManagerStatics3> for PushNotificationChannelManager {}
impl PushNotificationChannelManager {
    #[inline] pub fn create_push_notification_channel_for_application_async() -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> {
        <Self as RtActivatable<IPushNotificationChannelManagerStatics>>::get_activation_factory().create_push_notification_channel_for_application_async()
    }
    #[inline] pub fn create_push_notification_channel_for_application_async_with_id(applicationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> {
        <Self as RtActivatable<IPushNotificationChannelManagerStatics>>::get_activation_factory().create_push_notification_channel_for_application_async_with_id(applicationId)
    }
    #[inline] pub fn create_push_notification_channel_for_secondary_tile_async(tileId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> {
        <Self as RtActivatable<IPushNotificationChannelManagerStatics>>::get_activation_factory().create_push_notification_channel_for_secondary_tile_async(tileId)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::super::system::User) -> Result<Option<ComPtr<PushNotificationChannelManagerForUser>>> {
        <Self as RtActivatable<IPushNotificationChannelManagerStatics2>>::get_activation_factory().get_for_user(user)
    }
    #[inline] pub fn get_default() -> Result<Option<ComPtr<PushNotificationChannelManagerForUser>>> {
        <Self as RtActivatable<IPushNotificationChannelManagerStatics3>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(PushNotificationChannelManager(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,80,117,115,104,78,111,116,105,102,105,99,97,116,105,111,110,115,46,80,117,115,104,78,111,116,105,102,105,99,97,116,105,111,110,67,104,97,110,110,101,108,77,97,110,97,103,101,114,0]) [CLSID_PushNotificationChannelManager]);
DEFINE_IID!(IID_IPushNotificationChannelManagerForUser, 2764330756, 4482, 17095, 136, 144, 245, 99, 196, 137, 13, 196);
RT_INTERFACE!{interface IPushNotificationChannelManagerForUser(IPushNotificationChannelManagerForUserVtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannelManagerForUser] {
    fn CreatePushNotificationChannelForApplicationAsync(&self, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut *mut super::super::system::User) -> HRESULT
}}
impl IPushNotificationChannelManagerForUser {
    #[inline] pub fn create_push_notification_channel_for_application_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForApplicationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_push_notification_channel_for_application_async_with_id(&self, applicationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForApplicationAsyncWithId)(self as *const _ as *mut _, applicationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_push_notification_channel_for_secondary_tile_async(&self, tileId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForSecondaryTileAsync)(self as *const _ as *mut _, tileId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::super::system::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PushNotificationChannelManagerForUser: IPushNotificationChannelManagerForUser}
DEFINE_IID!(IID_IPushNotificationChannelManagerForUser2, 3280668266, 31937, 19884, 135, 253, 190, 110, 146, 4, 20, 164);
RT_INTERFACE!{interface IPushNotificationChannelManagerForUser2(IPushNotificationChannelManagerForUser2Vtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannelManagerForUser2] {
    #[cfg(feature="windows-storage")] fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync(&self, appServerKey: *mut super::super::storage::streams::IBuffer, channelId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId(&self, appServerKey: *mut super::super::storage::streams::IBuffer, channelId: HSTRING, appId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT
}}
impl IPushNotificationChannelManagerForUser2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_raw_push_notification_channel_with_alternate_key_for_application_async(&self, appServerKey: &super::super::storage::streams::IBuffer, channelId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsync)(self as *const _ as *mut _, appServerKey as *const _ as *mut _, channelId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_raw_push_notification_channel_with_alternate_key_for_application_async_with_id(&self, appServerKey: &super::super::storage::streams::IBuffer, channelId: &HStringArg, appId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateRawPushNotificationChannelWithAlternateKeyForApplicationAsyncWithId)(self as *const _ as *mut _, appServerKey as *const _ as *mut _, channelId.get(), appId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPushNotificationChannelManagerStatics, 2343541605, 30625, 17800, 189, 25, 134, 21, 41, 169, 220, 240);
RT_INTERFACE!{static interface IPushNotificationChannelManagerStatics(IPushNotificationChannelManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannelManagerStatics] {
    fn CreatePushNotificationChannelForApplicationAsync(&self, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    fn CreatePushNotificationChannelForApplicationAsyncWithId(&self, applicationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT,
    fn CreatePushNotificationChannelForSecondaryTileAsync(&self, tileId: HSTRING, out: *mut *mut foundation::IAsyncOperation<PushNotificationChannel>) -> HRESULT
}}
impl IPushNotificationChannelManagerStatics {
    #[inline] pub fn create_push_notification_channel_for_application_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForApplicationAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_push_notification_channel_for_application_async_with_id(&self, applicationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForApplicationAsyncWithId)(self as *const _ as *mut _, applicationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_push_notification_channel_for_secondary_tile_async(&self, tileId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PushNotificationChannel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePushNotificationChannelForSecondaryTileAsync)(self as *const _ as *mut _, tileId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPushNotificationChannelManagerStatics2, 3024397917, 42985, 19240, 149, 14, 243, 117, 169, 7, 249, 223);
RT_INTERFACE!{static interface IPushNotificationChannelManagerStatics2(IPushNotificationChannelManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannelManagerStatics2] {
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: *mut super::super::system::User, out: *mut *mut PushNotificationChannelManagerForUser) -> HRESULT
}}
impl IPushNotificationChannelManagerStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &super::super::system::User) -> Result<Option<ComPtr<PushNotificationChannelManagerForUser>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPushNotificationChannelManagerStatics3, 1191313150, 3806, 19007, 174, 120, 191, 164, 113, 73, 105, 37);
RT_INTERFACE!{static interface IPushNotificationChannelManagerStatics3(IPushNotificationChannelManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationChannelManagerStatics3] {
    fn GetDefault(&self, out: *mut *mut PushNotificationChannelManagerForUser) -> HRESULT
}}
impl IPushNotificationChannelManagerStatics3 {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<PushNotificationChannelManagerForUser>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPushNotificationReceivedEventArgs, 3506855436, 14029, 18508, 185, 53, 10, 153, 183, 83, 207, 0);
RT_INTERFACE!{interface IPushNotificationReceivedEventArgs(IPushNotificationReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPushNotificationReceivedEventArgs] {
    fn put_Cancel(&self, value: bool) -> HRESULT,
    fn get_Cancel(&self, out: *mut bool) -> HRESULT,
    fn get_NotificationType(&self, out: *mut PushNotificationType) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_ToastNotification(&self, out: *mut *mut super::super::ui::notifications::ToastNotification) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_TileNotification(&self, out: *mut *mut super::super::ui::notifications::TileNotification) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_BadgeNotification(&self, out: *mut *mut super::super::ui::notifications::BadgeNotification) -> HRESULT,
    fn get_RawNotification(&self, out: *mut *mut RawNotification) -> HRESULT
}}
impl IPushNotificationReceivedEventArgs {
    #[inline] pub fn set_cancel(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Cancel)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cancel(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Cancel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_notification_type(&self) -> Result<PushNotificationType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NotificationType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_toast_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::ToastNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ToastNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_tile_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::TileNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TileNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_badge_notification(&self) -> Result<Option<ComPtr<super::super::ui::notifications::BadgeNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BadgeNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_notification(&self) -> Result<Option<ComPtr<RawNotification>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RawNotification)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PushNotificationReceivedEventArgs: IPushNotificationReceivedEventArgs}
RT_ENUM! { enum PushNotificationType: i32 {
    Toast = 0, Tile = 1, Badge = 2, Raw = 3, TileFlyout = 4,
}}
DEFINE_IID!(IID_IRawNotification, 438465153, 15225, 17068, 153, 99, 34, 171, 0, 212, 240, 183);
RT_INTERFACE!{interface IRawNotification(IRawNotificationVtbl): IInspectable(IInspectableVtbl) [IID_IRawNotification] {
    fn get_Content(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRawNotification {
    #[inline] pub fn get_content(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Content)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RawNotification: IRawNotification}
DEFINE_IID!(IID_IRawNotification2, 3872444185, 3183, 19677, 148, 36, 238, 197, 190, 1, 77, 38);
RT_INTERFACE!{interface IRawNotification2(IRawNotification2Vtbl): IInspectable(IInspectableVtbl) [IID_IRawNotification2] {
    fn get_Headers(&self, out: *mut *mut foundation::collections::IMapView<HString, HString>) -> HRESULT,
    fn get_ChannelId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRawNotification2 {
    #[inline] pub fn get_headers(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Headers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_channel_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ChannelId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Networking.PushNotifications
pub mod servicediscovery { // Windows.Networking.ServiceDiscovery
pub mod dnssd { // Windows.Networking.ServiceDiscovery.Dnssd
use crate::prelude::*;
DEFINE_IID!(IID_IDnssdRegistrationResult, 1031301842, 58886, 21328, 115, 234, 126, 151, 240, 102, 22, 47);
RT_INTERFACE!{interface IDnssdRegistrationResult(IDnssdRegistrationResultVtbl): IInspectable(IInspectableVtbl) [IID_IDnssdRegistrationResult] {
    fn get_Status(&self, out: *mut DnssdRegistrationStatus) -> HRESULT,
    fn get_IPAddress(&self, out: *mut *mut super::super::HostName) -> HRESULT,
    fn get_HasInstanceNameChanged(&self, out: *mut bool) -> HRESULT
}}
impl IDnssdRegistrationResult {
    #[inline] pub fn get_status(&self) -> Result<DnssdRegistrationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_ip_address(&self) -> Result<Option<ComPtr<super::super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IPAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_instance_name_changed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HasInstanceNameChanged)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class DnssdRegistrationResult: IDnssdRegistrationResult}
impl RtActivatable<IActivationFactory> for DnssdRegistrationResult {}
DEFINE_CLSID!(DnssdRegistrationResult(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,101,114,118,105,99,101,68,105,115,99,111,118,101,114,121,46,68,110,115,115,100,46,68,110,115,115,100,82,101,103,105,115,116,114,97,116,105,111,110,82,101,115,117,108,116,0]) [CLSID_DnssdRegistrationResult]);
RT_ENUM! { enum DnssdRegistrationStatus: i32 {
    Success = 0, InvalidServiceName = 1, ServerError = 2, SecurityError = 3,
}}
DEFINE_IID!(IID_IDnssdServiceInstance, 3796294526, 39077, 19617, 185, 228, 194, 83, 211, 60, 53, 255);
RT_INTERFACE!{interface IDnssdServiceInstance(IDnssdServiceInstanceVtbl): IInspectable(IInspectableVtbl) [IID_IDnssdServiceInstance] {
    fn get_DnssdServiceInstanceName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DnssdServiceInstanceName(&self, value: HSTRING) -> HRESULT,
    fn get_HostName(&self, out: *mut *mut super::super::HostName) -> HRESULT,
    fn put_HostName(&self, value: *mut super::super::HostName) -> HRESULT,
    fn get_Port(&self, out: *mut u16) -> HRESULT,
    fn put_Port(&self, value: u16) -> HRESULT,
    fn get_Priority(&self, out: *mut u16) -> HRESULT,
    fn put_Priority(&self, value: u16) -> HRESULT,
    fn get_Weight(&self, out: *mut u16) -> HRESULT,
    fn put_Weight(&self, value: u16) -> HRESULT,
    fn get_TextAttributes(&self, out: *mut *mut foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn RegisterStreamSocketListenerAsync1(&self, socket: *mut super::super::sockets::StreamSocketListener, out: *mut *mut foundation::IAsyncOperation<DnssdRegistrationResult>) -> HRESULT,
    fn RegisterStreamSocketListenerAsync2(&self, socket: *mut super::super::sockets::StreamSocketListener, adapter: *mut super::super::connectivity::NetworkAdapter, out: *mut *mut foundation::IAsyncOperation<DnssdRegistrationResult>) -> HRESULT,
    fn RegisterDatagramSocketAsync1(&self, socket: *mut super::super::sockets::DatagramSocket, out: *mut *mut foundation::IAsyncOperation<DnssdRegistrationResult>) -> HRESULT,
    fn RegisterDatagramSocketAsync2(&self, socket: *mut super::super::sockets::DatagramSocket, adapter: *mut super::super::connectivity::NetworkAdapter, out: *mut *mut foundation::IAsyncOperation<DnssdRegistrationResult>) -> HRESULT
}}
impl IDnssdServiceInstance {
    #[inline] pub fn get_dnssd_service_instance_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DnssdServiceInstanceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_dnssd_service_instance_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DnssdServiceInstanceName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_host_name(&self) -> Result<Option<ComPtr<super::super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_host_name(&self, value: &super::super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_HostName)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_port(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Port)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_port(&self, value: u16) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Port)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_priority(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Priority)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_priority(&self, value: u16) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Priority)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_weight(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Weight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_weight(&self, value: u16) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Weight)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_text_attributes(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TextAttributes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_stream_socket_listener_async1(&self, socket: &super::super::sockets::StreamSocketListener) -> Result<ComPtr<foundation::IAsyncOperation<DnssdRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterStreamSocketListenerAsync1)(self as *const _ as *mut _, socket as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_stream_socket_listener_async2(&self, socket: &super::super::sockets::StreamSocketListener, adapter: &super::super::connectivity::NetworkAdapter) -> Result<ComPtr<foundation::IAsyncOperation<DnssdRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterStreamSocketListenerAsync2)(self as *const _ as *mut _, socket as *const _ as *mut _, adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_datagram_socket_async1(&self, socket: &super::super::sockets::DatagramSocket) -> Result<ComPtr<foundation::IAsyncOperation<DnssdRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterDatagramSocketAsync1)(self as *const _ as *mut _, socket as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_datagram_socket_async2(&self, socket: &super::super::sockets::DatagramSocket, adapter: &super::super::connectivity::NetworkAdapter) -> Result<ComPtr<foundation::IAsyncOperation<DnssdRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterDatagramSocketAsync2)(self as *const _ as *mut _, socket as *const _ as *mut _, adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DnssdServiceInstance: IDnssdServiceInstance}
impl RtActivatable<IDnssdServiceInstanceFactory> for DnssdServiceInstance {}
impl DnssdServiceInstance {
    #[inline] pub fn create(dnssdServiceInstanceName: &HStringArg, hostName: &super::super::HostName, port: u16) -> Result<ComPtr<DnssdServiceInstance>> {
        <Self as RtActivatable<IDnssdServiceInstanceFactory>>::get_activation_factory().create(dnssdServiceInstanceName, hostName, port)
    }
}
DEFINE_CLSID!(DnssdServiceInstance(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,101,114,118,105,99,101,68,105,115,99,111,118,101,114,121,46,68,110,115,115,100,46,68,110,115,115,100,83,101,114,118,105,99,101,73,110,115,116,97,110,99,101,0]) [CLSID_DnssdServiceInstance]);
RT_CLASS!{class DnssdServiceInstanceCollection: foundation::collections::IVectorView<DnssdServiceInstance>}
DEFINE_IID!(IID_IDnssdServiceInstanceFactory, 1823498657, 50296, 17201, 150, 132, 74, 242, 24, 108, 10, 43);
RT_INTERFACE!{static interface IDnssdServiceInstanceFactory(IDnssdServiceInstanceFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDnssdServiceInstanceFactory] {
    fn Create(&self, dnssdServiceInstanceName: HSTRING, hostName: *mut super::super::HostName, port: u16, out: *mut *mut DnssdServiceInstance) -> HRESULT
}}
impl IDnssdServiceInstanceFactory {
    #[inline] pub fn create(&self, dnssdServiceInstanceName: &HStringArg, hostName: &super::super::HostName, port: u16) -> Result<ComPtr<DnssdServiceInstance>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, dnssdServiceInstanceName.get(), hostName as *const _ as *mut _, port, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDnssdServiceWatcher, 3426015681, 56189, 19305, 152, 61, 198, 248, 63, 32, 86, 130);
RT_INTERFACE!{interface IDnssdServiceWatcher(IDnssdServiceWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IDnssdServiceWatcher] {
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<DnssdServiceWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<DnssdServiceWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut DnssdServiceWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IDnssdServiceWatcher {
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<DnssdServiceWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<DnssdServiceWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<DnssdServiceWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DnssdServiceWatcher: IDnssdServiceWatcher}
RT_ENUM! { enum DnssdServiceWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
} // Windows.Networking.ServiceDiscovery.Dnssd
} // Windows.Networking.ServiceDiscovery
pub mod sockets { // Windows.Networking.Sockets
use crate::prelude::*;
RT_STRUCT! { struct BandwidthStatistics {
    OutboundBitsPerSecond: u64, InboundBitsPerSecond: u64, OutboundBitsPerSecondInstability: u64, InboundBitsPerSecondInstability: u64, OutboundBandwidthPeaked: bool, InboundBandwidthPeaked: bool,
}}
DEFINE_IID!(IID_IControlChannelTrigger, 2098475431, 61078, 16616, 161, 153, 135, 3, 205, 150, 158, 195);
RT_INTERFACE!{interface IControlChannelTrigger(IControlChannelTriggerVtbl): IInspectable(IInspectableVtbl) [IID_IControlChannelTrigger] {
    fn get_ControlChannelTriggerId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ServerKeepAliveIntervalInMinutes(&self, out: *mut u32) -> HRESULT,
    fn put_ServerKeepAliveIntervalInMinutes(&self, value: u32) -> HRESULT,
    fn get_CurrentKeepAliveIntervalInMinutes(&self, out: *mut u32) -> HRESULT,
    fn get_TransportObject(&self, out: *mut *mut IInspectable) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_KeepAliveTrigger(&self, out: *mut *mut super::super::applicationmodel::background::IBackgroundTrigger) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_PushNotificationTrigger(&self, out: *mut *mut super::super::applicationmodel::background::IBackgroundTrigger) -> HRESULT,
    fn UsingTransport(&self, transport: *mut IInspectable) -> HRESULT,
    fn WaitForPushEnabled(&self, out: *mut ControlChannelTriggerStatus) -> HRESULT,
    fn DecreaseNetworkKeepAliveInterval(&self) -> HRESULT,
    fn FlushTransport(&self) -> HRESULT
}}
impl IControlChannelTrigger {
    #[inline] pub fn get_control_channel_trigger_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControlChannelTriggerId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_server_keep_alive_interval_in_minutes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerKeepAliveIntervalInMinutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_server_keep_alive_interval_in_minutes(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ServerKeepAliveIntervalInMinutes)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_current_keep_alive_interval_in_minutes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentKeepAliveIntervalInMinutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_transport_object(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransportObject)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_keep_alive_trigger(&self) -> Result<Option<ComPtr<super::super::applicationmodel::background::IBackgroundTrigger>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_KeepAliveTrigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_push_notification_trigger(&self) -> Result<Option<ComPtr<super::super::applicationmodel::background::IBackgroundTrigger>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PushNotificationTrigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn using_transport(&self, transport: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UsingTransport)(self as *const _ as *mut _, transport as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_push_enabled(&self) -> Result<ControlChannelTriggerStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).WaitForPushEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn decrease_network_keep_alive_interval(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).DecreaseNetworkKeepAliveInterval)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn flush_transport(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).FlushTransport)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ControlChannelTrigger: IControlChannelTrigger}
impl RtActivatable<IControlChannelTriggerFactory> for ControlChannelTrigger {}
impl ControlChannelTrigger {
    #[inline] pub fn create_control_channel_trigger(channelId: &HStringArg, serverKeepAliveIntervalInMinutes: u32) -> Result<ComPtr<ControlChannelTrigger>> {
        <Self as RtActivatable<IControlChannelTriggerFactory>>::get_activation_factory().create_control_channel_trigger(channelId, serverKeepAliveIntervalInMinutes)
    }
    #[inline] pub fn create_control_channel_trigger_ex(channelId: &HStringArg, serverKeepAliveIntervalInMinutes: u32, resourceRequestType: ControlChannelTriggerResourceType) -> Result<ComPtr<ControlChannelTrigger>> {
        <Self as RtActivatable<IControlChannelTriggerFactory>>::get_activation_factory().create_control_channel_trigger_ex(channelId, serverKeepAliveIntervalInMinutes, resourceRequestType)
    }
}
DEFINE_CLSID!(ControlChannelTrigger(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,67,111,110,116,114,111,108,67,104,97,110,110,101,108,84,114,105,103,103,101,114,0]) [CLSID_ControlChannelTrigger]);
DEFINE_IID!(IID_IControlChannelTrigger2, 2936066615, 20926, 17684, 151, 37, 53, 86, 225, 135, 149, 128);
RT_INTERFACE!{interface IControlChannelTrigger2(IControlChannelTrigger2Vtbl): IInspectable(IInspectableVtbl) [IID_IControlChannelTrigger2] {
    fn get_IsWakeFromLowPowerSupported(&self, out: *mut bool) -> HRESULT
}}
impl IControlChannelTrigger2 {
    #[inline] pub fn get_is_wake_from_low_power_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWakeFromLowPowerSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IControlChannelTriggerEventDetails, 456581191, 35259, 16950, 150, 172, 113, 208, 18, 187, 72, 105);
RT_INTERFACE!{interface IControlChannelTriggerEventDetails(IControlChannelTriggerEventDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IControlChannelTriggerEventDetails] {
    fn get_ControlChannelTrigger(&self, out: *mut *mut ControlChannelTrigger) -> HRESULT
}}
impl IControlChannelTriggerEventDetails {
    #[inline] pub fn get_control_channel_trigger(&self) -> Result<Option<ComPtr<ControlChannelTrigger>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControlChannelTrigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IControlChannelTriggerFactory, 3662380272, 36209, 17519, 136, 195, 185, 81, 132, 162, 214, 205);
RT_INTERFACE!{static interface IControlChannelTriggerFactory(IControlChannelTriggerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IControlChannelTriggerFactory] {
    fn CreateControlChannelTrigger(&self, channelId: HSTRING, serverKeepAliveIntervalInMinutes: u32, out: *mut *mut ControlChannelTrigger) -> HRESULT,
    fn CreateControlChannelTriggerEx(&self, channelId: HSTRING, serverKeepAliveIntervalInMinutes: u32, resourceRequestType: ControlChannelTriggerResourceType, out: *mut *mut ControlChannelTrigger) -> HRESULT
}}
impl IControlChannelTriggerFactory {
    #[inline] pub fn create_control_channel_trigger(&self, channelId: &HStringArg, serverKeepAliveIntervalInMinutes: u32) -> Result<ComPtr<ControlChannelTrigger>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateControlChannelTrigger)(self as *const _ as *mut _, channelId.get(), serverKeepAliveIntervalInMinutes, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_control_channel_trigger_ex(&self, channelId: &HStringArg, serverKeepAliveIntervalInMinutes: u32, resourceRequestType: ControlChannelTriggerResourceType) -> Result<ComPtr<ControlChannelTrigger>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateControlChannelTriggerEx)(self as *const _ as *mut _, channelId.get(), serverKeepAliveIntervalInMinutes, resourceRequestType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IControlChannelTriggerResetEventDetails, 1750139790, 36548, 17150, 155, 178, 33, 233, 27, 123, 252, 177);
RT_INTERFACE!{interface IControlChannelTriggerResetEventDetails(IControlChannelTriggerResetEventDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IControlChannelTriggerResetEventDetails] {
    fn get_ResetReason(&self, out: *mut ControlChannelTriggerResetReason) -> HRESULT,
    fn get_HardwareSlotReset(&self, out: *mut bool) -> HRESULT,
    fn get_SoftwareSlotReset(&self, out: *mut bool) -> HRESULT
}}
impl IControlChannelTriggerResetEventDetails {
    #[inline] pub fn get_reset_reason(&self) -> Result<ControlChannelTriggerResetReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResetReason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_slot_reset(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HardwareSlotReset)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_software_slot_reset(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SoftwareSlotReset)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum ControlChannelTriggerResetReason: i32 {
    FastUserSwitched = 0, LowPowerExit = 1, QuietHoursExit = 2, ApplicationRestart = 3,
}}
RT_ENUM! { enum ControlChannelTriggerResourceType: i32 {
    RequestSoftwareSlot = 0, RequestHardwareSlot = 1,
}}
RT_ENUM! { enum ControlChannelTriggerStatus: i32 {
    HardwareSlotRequested = 0, SoftwareSlotAllocated = 1, HardwareSlotAllocated = 2, PolicyError = 3, SystemError = 4, TransportDisconnected = 5, ServiceUnavailable = 6,
}}
DEFINE_IID!(IID_IDatagramSocket, 2145541051, 50108, 18039, 132, 70, 202, 40, 164, 101, 163, 175);
RT_INTERFACE!{interface IDatagramSocket(IDatagramSocketVtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocket] {
    fn get_Control(&self, out: *mut *mut DatagramSocketControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut DatagramSocketInformation) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_OutputStream(&self, out: *mut *mut super::super::storage::streams::IOutputStream) -> HRESULT,
    fn ConnectAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ConnectWithEndpointPairAsync(&self, endpointPair: *mut super::EndpointPair, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn BindServiceNameAsync(&self, localServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn BindEndpointAsync(&self, localHostName: *mut super::HostName, localServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn JoinMulticastGroup(&self, host: *mut super::HostName) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetOutputStreamAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, out: *mut *mut foundation::IAsyncOperation<super::super::storage::streams::IOutputStream>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetOutputStreamWithEndpointPairAsync(&self, endpointPair: *mut super::EndpointPair, out: *mut *mut foundation::IAsyncOperation<super::super::storage::streams::IOutputStream>) -> HRESULT,
    fn add_MessageReceived(&self, eventHandler: *mut foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MessageReceived(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDatagramSocket {
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<DatagramSocketControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<DatagramSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_with_endpoint_pair_async(&self, endpointPair: &super::EndpointPair) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectWithEndpointPairAsync)(self as *const _ as *mut _, endpointPair as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn bind_service_name_async(&self, localServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindServiceNameAsync)(self as *const _ as *mut _, localServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn bind_endpoint_async(&self, localHostName: &super::HostName, localServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindEndpointAsync)(self as *const _ as *mut _, localHostName as *const _ as *mut _, localServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn join_multicast_group(&self, host: &super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).JoinMulticastGroup)(self as *const _ as *mut _, host as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOutputStreamAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream_with_endpoint_pair_async(&self, endpointPair: &super::EndpointPair) -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOutputStreamWithEndpointPairAsync)(self as *const _ as *mut _, endpointPair as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_message_received(&self, eventHandler: &foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_MessageReceived)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_message_received(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_MessageReceived)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DatagramSocket: IDatagramSocket}
impl RtActivatable<IDatagramSocketStatics> for DatagramSocket {}
impl RtActivatable<IActivationFactory> for DatagramSocket {}
impl DatagramSocket {
    #[inline] pub fn get_endpoint_pairs_async(remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> {
        <Self as RtActivatable<IDatagramSocketStatics>>::get_activation_factory().get_endpoint_pairs_async(remoteHostName, remoteServiceName)
    }
    #[inline] pub fn get_endpoint_pairs_with_sort_options_async(remoteHostName: &super::HostName, remoteServiceName: &HStringArg, sortOptions: super::HostNameSortOptions) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> {
        <Self as RtActivatable<IDatagramSocketStatics>>::get_activation_factory().get_endpoint_pairs_with_sort_options_async(remoteHostName, remoteServiceName, sortOptions)
    }
}
DEFINE_CLSID!(DatagramSocket(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,68,97,116,97,103,114,97,109,83,111,99,107,101,116,0]) [CLSID_DatagramSocket]);
DEFINE_IID!(IID_IDatagramSocket2, 3627787092, 39581, 16773, 162, 10, 20, 36, 201, 194, 167, 205);
RT_INTERFACE!{interface IDatagramSocket2(IDatagramSocket2Vtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocket2] {
    fn BindServiceNameAndAdapterAsync(&self, localServiceName: HSTRING, adapter: *mut super::connectivity::NetworkAdapter, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IDatagramSocket2 {
    #[inline] pub fn bind_service_name_and_adapter_async(&self, localServiceName: &HStringArg, adapter: &super::connectivity::NetworkAdapter) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindServiceNameAndAdapterAsync)(self as *const _ as *mut _, localServiceName.get(), adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDatagramSocket3, 928272137, 43922, 17158, 154, 193, 12, 56, 18, 131, 217, 198);
RT_INTERFACE!{interface IDatagramSocket3(IDatagramSocket3Vtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocket3] {
    fn CancelIOAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn EnableTransferOwnership(&self, taskId: Guid) -> HRESULT,
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> HRESULT,
    fn TransferOwnership(&self, socketId: HSTRING) -> HRESULT,
    fn TransferOwnershipWithContext(&self, socketId: HSTRING, data: *mut SocketActivityContext) -> HRESULT,
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketId: HSTRING, data: *mut SocketActivityContext, keepAliveTime: foundation::TimeSpan) -> HRESULT
}}
impl IDatagramSocket3 {
    #[inline] pub fn cancel_io_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CancelIOAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership(&self, taskId: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnership)(self as *const _ as *mut _, taskId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership_with_connected_standby_action(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnershipWithConnectedStandbyAction)(self as *const _ as *mut _, taskId, connectedStandbyAction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership(&self, socketId: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnership)(self as *const _ as *mut _, socketId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership_with_context(&self, socketId: &HStringArg, data: &SocketActivityContext) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnershipWithContext)(self as *const _ as *mut _, socketId.get(), data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership_with_context_and_keep_alive_time(&self, socketId: &HStringArg, data: &SocketActivityContext, keepAliveTime: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnershipWithContextAndKeepAliveTime)(self as *const _ as *mut _, socketId.get(), data as *const _ as *mut _, keepAliveTime);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDatagramSocketControl, 1387020078, 13466, 16693, 187, 88, 183, 155, 38, 71, 211, 144);
RT_INTERFACE!{interface IDatagramSocketControl(IDatagramSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketControl] {
    fn get_QualityOfService(&self, out: *mut SocketQualityOfService) -> HRESULT,
    fn put_QualityOfService(&self, value: SocketQualityOfService) -> HRESULT,
    fn get_OutboundUnicastHopLimit(&self, out: *mut u8) -> HRESULT,
    fn put_OutboundUnicastHopLimit(&self, value: u8) -> HRESULT
}}
impl IDatagramSocketControl {
    #[inline] pub fn get_quality_of_service(&self) -> Result<SocketQualityOfService> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_QualityOfService)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_quality_of_service(&self, value: SocketQualityOfService) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_QualityOfService)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_unicast_hop_limit(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundUnicastHopLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_unicast_hop_limit(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundUnicastHopLimit)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DatagramSocketControl: IDatagramSocketControl}
DEFINE_IID!(IID_IDatagramSocketControl2, 871028162, 38812, 17429, 130, 161, 60, 250, 246, 70, 193, 146);
RT_INTERFACE!{interface IDatagramSocketControl2(IDatagramSocketControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketControl2] {
    fn get_InboundBufferSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn put_InboundBufferSizeInBytes(&self, value: u32) -> HRESULT,
    fn get_DontFragment(&self, out: *mut bool) -> HRESULT,
    fn put_DontFragment(&self, value: bool) -> HRESULT
}}
impl IDatagramSocketControl2 {
    #[inline] pub fn get_inbound_buffer_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InboundBufferSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_inbound_buffer_size_in_bytes(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InboundBufferSizeInBytes)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dont_fragment(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DontFragment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dont_fragment(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DontFragment)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDatagramSocketControl3, 3572204118, 8045, 17816, 155, 87, 212, 42, 0, 29, 243, 73);
RT_INTERFACE!{interface IDatagramSocketControl3(IDatagramSocketControl3Vtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketControl3] {
    fn get_MulticastOnly(&self, out: *mut bool) -> HRESULT,
    fn put_MulticastOnly(&self, value: bool) -> HRESULT
}}
impl IDatagramSocketControl3 {
    #[inline] pub fn get_multicast_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MulticastOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_multicast_only(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MulticastOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDatagramSocketInformation, 1595561626, 22011, 18637, 151, 6, 122, 151, 79, 123, 21, 133);
RT_INTERFACE!{interface IDatagramSocketInformation(IDatagramSocketInformationVtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketInformation] {
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_LocalPort(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RemoteAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_RemotePort(&self, out: *mut HSTRING) -> HRESULT
}}
impl IDatagramSocketInformation {
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalPort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemotePort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DatagramSocketInformation: IDatagramSocketInformation}
DEFINE_IID!(IID_IDatagramSocketMessageReceivedEventArgs, 2653805730, 5906, 19684, 177, 121, 140, 101, 44, 109, 16, 126);
RT_INTERFACE!{interface IDatagramSocketMessageReceivedEventArgs(IDatagramSocketMessageReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketMessageReceivedEventArgs] {
    fn get_RemoteAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_RemotePort(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetDataReader(&self, out: *mut *mut super::super::storage::streams::DataReader) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetDataStream(&self, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT
}}
impl IDatagramSocketMessageReceivedEventArgs {
    #[inline] pub fn get_remote_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemotePort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data_reader(&self) -> Result<Option<ComPtr<super::super::storage::streams::DataReader>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDataReader)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDataStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DatagramSocketMessageReceivedEventArgs: IDatagramSocketMessageReceivedEventArgs}
DEFINE_IID!(IID_IDatagramSocketStatics, 3922078446, 5268, 18977, 187, 126, 133, 137, 252, 117, 29, 157);
RT_INTERFACE!{static interface IDatagramSocketStatics(IDatagramSocketStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDatagramSocketStatics] {
    fn GetEndpointPairsAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>) -> HRESULT,
    fn GetEndpointPairsWithSortOptionsAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, sortOptions: super::HostNameSortOptions, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>) -> HRESULT
}}
impl IDatagramSocketStatics {
    #[inline] pub fn get_endpoint_pairs_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetEndpointPairsAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_endpoint_pairs_with_sort_options_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg, sortOptions: super::HostNameSortOptions) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetEndpointPairsWithSortOptionsAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), sortOptions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMessageWebSocket, 863141128, 13525, 18246, 173, 123, 141, 222, 91, 194, 239, 136);
RT_INTERFACE!{interface IMessageWebSocket(IMessageWebSocketVtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocket] {
    fn get_Control(&self, out: *mut *mut MessageWebSocketControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut MessageWebSocketInformation) -> HRESULT,
    fn add_MessageReceived(&self, eventHandler: *mut foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MessageReceived(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IMessageWebSocket {
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<MessageWebSocketControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<MessageWebSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_message_received(&self, eventHandler: &foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_MessageReceived)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_message_received(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_MessageReceived)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MessageWebSocket: IMessageWebSocket}
impl RtActivatable<IActivationFactory> for MessageWebSocket {}
DEFINE_CLSID!(MessageWebSocket(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,77,101,115,115,97,103,101,87,101,98,83,111,99,107,101,116,0]) [CLSID_MessageWebSocket]);
DEFINE_IID!(IID_IMessageWebSocket2, 3201355495, 63944, 17418, 154, 213, 115, 114, 129, 217, 116, 46);
RT_INTERFACE!{interface IMessageWebSocket2(IMessageWebSocket2Vtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocket2] {
    fn add_ServerCustomValidationRequested(&self, eventHandler: *mut foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ServerCustomValidationRequested(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IMessageWebSocket2 {
    #[inline] pub fn add_server_custom_validation_requested(&self, eventHandler: &foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ServerCustomValidationRequested)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_server_custom_validation_requested(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ServerCustomValidationRequested)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMessageWebSocket3, 1507450619, 29103, 17225, 132, 135, 145, 31, 207, 104, 21, 151);
RT_INTERFACE!{interface IMessageWebSocket3(IMessageWebSocket3Vtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocket3] {
    #[cfg(feature="windows-storage")] fn SendNonfinalFrameAsync(&self, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut foundation::IAsyncOperationWithProgress<u32, u32>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SendFinalFrameAsync(&self, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut foundation::IAsyncOperationWithProgress<u32, u32>) -> HRESULT
}}
impl IMessageWebSocket3 {
    #[cfg(feature="windows-storage")] #[inline] pub fn send_nonfinal_frame_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<u32, u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendNonfinalFrameAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn send_final_frame_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<u32, u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendFinalFrameAsync)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMessageWebSocketControl, 2165848202, 50729, 20234, 128, 251, 129, 252, 5, 83, 136, 98);
RT_INTERFACE!{interface IMessageWebSocketControl(IMessageWebSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocketControl] {
    fn get_MaxMessageSize(&self, out: *mut u32) -> HRESULT,
    fn put_MaxMessageSize(&self, value: u32) -> HRESULT,
    fn get_MessageType(&self, out: *mut SocketMessageType) -> HRESULT,
    fn put_MessageType(&self, value: SocketMessageType) -> HRESULT
}}
impl IMessageWebSocketControl {
    #[inline] pub fn get_max_message_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxMessageSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_message_size(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxMessageSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_message_type(&self) -> Result<SocketMessageType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MessageType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_message_type(&self, value: SocketMessageType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MessageType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MessageWebSocketControl: IMessageWebSocketControl}
DEFINE_IID!(IID_IMessageWebSocketControl2, 3809466257, 2060, 16394, 167, 18, 39, 223, 169, 231, 68, 216);
RT_INTERFACE!{interface IMessageWebSocketControl2(IMessageWebSocketControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocketControl2] {
    fn get_DesiredUnsolicitedPongInterval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn put_DesiredUnsolicitedPongInterval(&self, value: foundation::TimeSpan) -> HRESULT,
    fn get_ActualUnsolicitedPongInterval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_ReceiveMode(&self, out: *mut MessageWebSocketReceiveMode) -> HRESULT,
    fn put_ReceiveMode(&self, value: MessageWebSocketReceiveMode) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ClientCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(feature="windows-security")] fn put_ClientCertificate(&self, value: *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT
}}
impl IMessageWebSocketControl2 {
    #[inline] pub fn get_desired_unsolicited_pong_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredUnsolicitedPongInterval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_desired_unsolicited_pong_interval(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DesiredUnsolicitedPongInterval)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_actual_unsolicited_pong_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActualUnsolicitedPongInterval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_receive_mode(&self) -> Result<MessageWebSocketReceiveMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReceiveMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_receive_mode(&self, value: MessageWebSocketReceiveMode) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ReceiveMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_client_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_client_certificate(&self, value: &super::super::security::cryptography::certificates::Certificate) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ClientCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MessageWebSocketInformation: IWebSocketInformation}
DEFINE_IID!(IID_IMessageWebSocketMessageReceivedEventArgs, 1200366252, 19531, 17133, 158, 215, 30, 249, 249, 79, 163, 213);
RT_INTERFACE!{interface IMessageWebSocketMessageReceivedEventArgs(IMessageWebSocketMessageReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocketMessageReceivedEventArgs] {
    fn get_MessageType(&self, out: *mut SocketMessageType) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetDataReader(&self, out: *mut *mut super::super::storage::streams::DataReader) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetDataStream(&self, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT
}}
impl IMessageWebSocketMessageReceivedEventArgs {
    #[inline] pub fn get_message_type(&self) -> Result<SocketMessageType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MessageType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data_reader(&self) -> Result<Option<ComPtr<super::super::storage::streams::DataReader>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDataReader)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDataStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MessageWebSocketMessageReceivedEventArgs: IMessageWebSocketMessageReceivedEventArgs}
DEFINE_IID!(IID_IMessageWebSocketMessageReceivedEventArgs2, 2311980797, 56687, 18951, 135, 249, 249, 235, 77, 137, 216, 61);
RT_INTERFACE!{interface IMessageWebSocketMessageReceivedEventArgs2(IMessageWebSocketMessageReceivedEventArgs2Vtbl): IInspectable(IInspectableVtbl) [IID_IMessageWebSocketMessageReceivedEventArgs2] {
    fn get_IsMessageComplete(&self, out: *mut bool) -> HRESULT
}}
impl IMessageWebSocketMessageReceivedEventArgs2 {
    #[inline] pub fn get_is_message_complete(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsMessageComplete)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum MessageWebSocketReceiveMode: i32 {
    FullMessage = 0, PartialMessage = 1,
}}
RT_STRUCT! { struct RoundTripTimeStatistics {
    Variance: u32, Max: u32, Min: u32, Sum: u32,
}}
DEFINE_IID!(IID_IServerMessageWebSocket, 3819737664, 33083, 24317, 126, 17, 174, 35, 5, 252, 119, 241);
RT_INTERFACE!{interface IServerMessageWebSocket(IServerMessageWebSocketVtbl): IInspectable(IInspectableVtbl) [IID_IServerMessageWebSocket] {
    fn add_MessageReceived(&self, value: *mut foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_MessageReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Control(&self, out: *mut *mut ServerMessageWebSocketControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut ServerMessageWebSocketInformation) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_OutputStream(&self, out: *mut *mut super::super::storage::streams::IOutputStream) -> HRESULT,
    fn add_Closed(&self, value: *mut foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CloseWithStatus(&self, code: u16, reason: HSTRING) -> HRESULT
}}
impl IServerMessageWebSocket {
    #[inline] pub fn add_message_received(&self, value: &foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_MessageReceived)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_message_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_MessageReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<ServerMessageWebSocketControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<ServerMessageWebSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_closed(&self, value: &foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn close_with_status(&self, code: u16, reason: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CloseWithStatus)(self as *const _ as *mut _, code, reason.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ServerMessageWebSocket: IServerMessageWebSocket}
DEFINE_IID!(IID_IServerMessageWebSocketControl, 1774383185, 7199, 22650, 69, 25, 33, 129, 97, 1, 146, 183);
RT_INTERFACE!{interface IServerMessageWebSocketControl(IServerMessageWebSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IServerMessageWebSocketControl] {
    fn get_MessageType(&self, out: *mut SocketMessageType) -> HRESULT,
    fn put_MessageType(&self, value: SocketMessageType) -> HRESULT
}}
impl IServerMessageWebSocketControl {
    #[inline] pub fn get_message_type(&self) -> Result<SocketMessageType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MessageType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_message_type(&self, value: SocketMessageType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MessageType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ServerMessageWebSocketControl: IServerMessageWebSocketControl}
DEFINE_IID!(IID_IServerMessageWebSocketInformation, 4231181407, 17480, 21765, 108, 201, 9, 175, 168, 145, 95, 93);
RT_INTERFACE!{interface IServerMessageWebSocketInformation(IServerMessageWebSocketInformationVtbl): IInspectable(IInspectableVtbl) [IID_IServerMessageWebSocketInformation] {
    fn get_BandwidthStatistics(&self, out: *mut BandwidthStatistics) -> HRESULT,
    fn get_Protocol(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT
}}
impl IServerMessageWebSocketInformation {
    #[inline] pub fn get_bandwidth_statistics(&self) -> Result<BandwidthStatistics> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BandwidthStatistics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_protocol(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Protocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ServerMessageWebSocketInformation: IServerMessageWebSocketInformation}
DEFINE_IID!(IID_IServerStreamWebSocket, 753753023, 29942, 21988, 121, 223, 145, 50, 104, 13, 254, 232);
RT_INTERFACE!{interface IServerStreamWebSocket(IServerStreamWebSocketVtbl): IInspectable(IInspectableVtbl) [IID_IServerStreamWebSocket] {
    fn get_Information(&self, out: *mut *mut ServerStreamWebSocketInformation) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_InputStream(&self, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_OutputStream(&self, out: *mut *mut super::super::storage::streams::IOutputStream) -> HRESULT,
    fn add_Closed(&self, value: *mut foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CloseWithStatus(&self, code: u16, reason: HSTRING) -> HRESULT
}}
impl IServerStreamWebSocket {
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<ServerStreamWebSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_closed(&self, value: &foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn close_with_status(&self, code: u16, reason: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CloseWithStatus)(self as *const _ as *mut _, code, reason.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ServerStreamWebSocket: IServerStreamWebSocket}
DEFINE_IID!(IID_IServerStreamWebSocketInformation, 4231181407, 17480, 21765, 108, 201, 9, 171, 168, 145, 95, 93);
RT_INTERFACE!{interface IServerStreamWebSocketInformation(IServerStreamWebSocketInformationVtbl): IInspectable(IInspectableVtbl) [IID_IServerStreamWebSocketInformation] {
    fn get_BandwidthStatistics(&self, out: *mut BandwidthStatistics) -> HRESULT,
    fn get_Protocol(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT
}}
impl IServerStreamWebSocketInformation {
    #[inline] pub fn get_bandwidth_statistics(&self) -> Result<BandwidthStatistics> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BandwidthStatistics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_protocol(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Protocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ServerStreamWebSocketInformation: IServerStreamWebSocketInformation}
RT_ENUM! { enum SocketActivityConnectedStandbyAction: i32 {
    DoNotWake = 0, Wake = 1,
}}
DEFINE_IID!(IID_ISocketActivityContext, 1135627620, 19589, 17302, 166, 55, 29, 151, 63, 110, 189, 73);
RT_INTERFACE!{interface ISocketActivityContext(ISocketActivityContextVtbl): IInspectable(IInspectableVtbl) [IID_ISocketActivityContext] {
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl ISocketActivityContext {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Data)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SocketActivityContext: ISocketActivityContext}
impl RtActivatable<ISocketActivityContextFactory> for SocketActivityContext {}
impl SocketActivityContext {
    #[cfg(feature="windows-storage")] #[inline] pub fn create(data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<SocketActivityContext>> {
        <Self as RtActivatable<ISocketActivityContextFactory>>::get_activation_factory().create(data)
    }
}
DEFINE_CLSID!(SocketActivityContext(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,111,99,107,101,116,65,99,116,105,118,105,116,121,67,111,110,116,101,120,116,0]) [CLSID_SocketActivityContext]);
DEFINE_IID!(IID_ISocketActivityContextFactory, 3114255299, 2188, 17288, 131, 174, 37, 37, 19, 142, 4, 154);
RT_INTERFACE!{static interface ISocketActivityContextFactory(ISocketActivityContextFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISocketActivityContextFactory] {
    #[cfg(feature="windows-storage")] fn Create(&self, data: *mut super::super::storage::streams::IBuffer, out: *mut *mut SocketActivityContext) -> HRESULT
}}
impl ISocketActivityContextFactory {
    #[cfg(feature="windows-storage")] #[inline] pub fn create(&self, data: &super::super::storage::streams::IBuffer) -> Result<ComPtr<SocketActivityContext>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, data as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISocketActivityInformation, 2374648548, 43134, 19316, 153, 104, 24, 91, 37, 17, 222, 254);
RT_INTERFACE!{interface ISocketActivityInformation(ISocketActivityInformationVtbl): IInspectable(IInspectableVtbl) [IID_ISocketActivityInformation] {
    fn get_TaskId(&self, out: *mut Guid) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SocketKind(&self, out: *mut SocketActivityKind) -> HRESULT,
    fn get_Context(&self, out: *mut *mut SocketActivityContext) -> HRESULT,
    fn get_DatagramSocket(&self, out: *mut *mut DatagramSocket) -> HRESULT,
    fn get_StreamSocket(&self, out: *mut *mut StreamSocket) -> HRESULT,
    fn get_StreamSocketListener(&self, out: *mut *mut StreamSocketListener) -> HRESULT
}}
impl ISocketActivityInformation {
    #[inline] pub fn get_task_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TaskId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_socket_kind(&self) -> Result<SocketActivityKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SocketKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_context(&self) -> Result<Option<ComPtr<SocketActivityContext>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Context)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_datagram_socket(&self) -> Result<Option<ComPtr<DatagramSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DatagramSocket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_stream_socket(&self) -> Result<Option<ComPtr<StreamSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StreamSocket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_stream_socket_listener(&self) -> Result<Option<ComPtr<StreamSocketListener>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StreamSocketListener)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SocketActivityInformation: ISocketActivityInformation}
impl RtActivatable<ISocketActivityInformationStatics> for SocketActivityInformation {}
impl SocketActivityInformation {
    #[inline] pub fn get_all_sockets() -> Result<Option<ComPtr<foundation::collections::IMapView<HString, SocketActivityInformation>>>> {
        <Self as RtActivatable<ISocketActivityInformationStatics>>::get_activation_factory().get_all_sockets()
    }
}
DEFINE_CLSID!(SocketActivityInformation(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,111,99,107,101,116,65,99,116,105,118,105,116,121,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_SocketActivityInformation]);
DEFINE_IID!(IID_ISocketActivityInformationStatics, 2238755962, 32381, 18230, 128, 65, 19, 39, 166, 84, 60, 86);
RT_INTERFACE!{static interface ISocketActivityInformationStatics(ISocketActivityInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISocketActivityInformationStatics] {
    fn get_AllSockets(&self, out: *mut *mut foundation::collections::IMapView<HString, SocketActivityInformation>) -> HRESULT
}}
impl ISocketActivityInformationStatics {
    #[inline] pub fn get_all_sockets(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, SocketActivityInformation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AllSockets)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SocketActivityKind: i32 {
    None = 0, StreamSocketListener = 1, DatagramSocket = 2, StreamSocket = 3,
}}
DEFINE_IID!(IID_ISocketActivityTriggerDetails, 1173620391, 64671, 20353, 172, 173, 53, 95, 239, 81, 230, 123);
RT_INTERFACE!{interface ISocketActivityTriggerDetails(ISocketActivityTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_ISocketActivityTriggerDetails] {
    fn get_Reason(&self, out: *mut SocketActivityTriggerReason) -> HRESULT,
    fn get_SocketInformation(&self, out: *mut *mut SocketActivityInformation) -> HRESULT
}}
impl ISocketActivityTriggerDetails {
    #[inline] pub fn get_reason(&self) -> Result<SocketActivityTriggerReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_socket_information(&self) -> Result<Option<ComPtr<SocketActivityInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SocketInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SocketActivityTriggerDetails: ISocketActivityTriggerDetails}
RT_ENUM! { enum SocketActivityTriggerReason: i32 {
    None = 0, SocketActivity = 1, ConnectionAccepted = 2, KeepAliveTimerExpired = 3, SocketClosed = 4,
}}
RT_CLASS!{static class SocketError}
impl RtActivatable<ISocketErrorStatics> for SocketError {}
impl SocketError {
    #[inline] pub fn get_status(hresult: i32) -> Result<SocketErrorStatus> {
        <Self as RtActivatable<ISocketErrorStatics>>::get_activation_factory().get_status(hresult)
    }
}
DEFINE_CLSID!(SocketError(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,111,99,107,101,116,69,114,114,111,114,0]) [CLSID_SocketError]);
DEFINE_IID!(IID_ISocketErrorStatics, 2189637620, 32086, 19854, 183, 180, 160, 125, 215, 193, 188, 169);
RT_INTERFACE!{static interface ISocketErrorStatics(ISocketErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISocketErrorStatics] {
    fn GetStatus(&self, hresult: i32, out: *mut SocketErrorStatus) -> HRESULT
}}
impl ISocketErrorStatics {
    #[inline] pub fn get_status(&self, hresult: i32) -> Result<SocketErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum SocketErrorStatus: i32 {
    Unknown = 0, OperationAborted = 1, HttpInvalidServerResponse = 2, ConnectionTimedOut = 3, AddressFamilyNotSupported = 4, SocketTypeNotSupported = 5, HostNotFound = 6, NoDataRecordOfRequestedType = 7, NonAuthoritativeHostNotFound = 8, ClassTypeNotFound = 9, AddressAlreadyInUse = 10, CannotAssignRequestedAddress = 11, ConnectionRefused = 12, NetworkIsUnreachable = 13, UnreachableHost = 14, NetworkIsDown = 15, NetworkDroppedConnectionOnReset = 16, SoftwareCausedConnectionAbort = 17, ConnectionResetByPeer = 18, HostIsDown = 19, NoAddressesFound = 20, TooManyOpenFiles = 21, MessageTooLong = 22, CertificateExpired = 23, CertificateUntrustedRoot = 24, CertificateCommonNameIsIncorrect = 25, CertificateWrongUsage = 26, CertificateRevoked = 27, CertificateNoRevocationCheck = 28, CertificateRevocationServerOffline = 29, CertificateIsInvalid = 30,
}}
RT_ENUM! { enum SocketMessageType: i32 {
    Binary = 0, Utf8 = 1,
}}
RT_ENUM! { enum SocketProtectionLevel: i32 {
    PlainSocket = 0, Ssl = 1, SslAllowNullEncryption = 2, BluetoothEncryptionAllowNullAuthentication = 3, BluetoothEncryptionWithAuthentication = 4, Ssl3AllowWeakEncryption = 5, Tls10 = 6, Tls11 = 7, Tls12 = 8, Unspecified = 9,
}}
RT_ENUM! { enum SocketQualityOfService: i32 {
    Normal = 0, LowLatency = 1,
}}
RT_ENUM! { enum SocketSslErrorSeverity: i32 {
    None = 0, Ignorable = 1, Fatal = 2,
}}
DEFINE_IID!(IID_IStreamSocket, 1772236019, 64635, 18519, 175, 56, 246, 231, 222, 106, 91, 73);
RT_INTERFACE!{interface IStreamSocket(IStreamSocketVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocket] {
    fn get_Control(&self, out: *mut *mut StreamSocketControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut StreamSocketInformation) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_InputStream(&self, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_OutputStream(&self, out: *mut *mut super::super::storage::streams::IOutputStream) -> HRESULT,
    fn ConnectWithEndpointPairAsync(&self, endpointPair: *mut super::EndpointPair, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ConnectAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ConnectWithEndpointPairAndProtectionLevelAsync(&self, endpointPair: *mut super::EndpointPair, protectionLevel: SocketProtectionLevel, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ConnectWithProtectionLevelAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, protectionLevel: SocketProtectionLevel, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn UpgradeToSslAsync(&self, protectionLevel: SocketProtectionLevel, validationHostName: *mut super::HostName, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IStreamSocket {
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<StreamSocketControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<StreamSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_with_endpoint_pair_async(&self, endpointPair: &super::EndpointPair) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectWithEndpointPairAsync)(self as *const _ as *mut _, endpointPair as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_with_endpoint_pair_and_protection_level_async(&self, endpointPair: &super::EndpointPair, protectionLevel: SocketProtectionLevel) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectWithEndpointPairAndProtectionLevelAsync)(self as *const _ as *mut _, endpointPair as *const _ as *mut _, protectionLevel, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_with_protection_level_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg, protectionLevel: SocketProtectionLevel) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectWithProtectionLevelAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), protectionLevel, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn upgrade_to_ssl_async(&self, protectionLevel: SocketProtectionLevel, validationHostName: &super::HostName) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpgradeToSslAsync)(self as *const _ as *mut _, protectionLevel, validationHostName as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocket: IStreamSocket}
impl RtActivatable<IStreamSocketStatics> for StreamSocket {}
impl RtActivatable<IActivationFactory> for StreamSocket {}
impl StreamSocket {
    #[inline] pub fn get_endpoint_pairs_async(remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> {
        <Self as RtActivatable<IStreamSocketStatics>>::get_activation_factory().get_endpoint_pairs_async(remoteHostName, remoteServiceName)
    }
    #[inline] pub fn get_endpoint_pairs_with_sort_options_async(remoteHostName: &super::HostName, remoteServiceName: &HStringArg, sortOptions: super::HostNameSortOptions) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> {
        <Self as RtActivatable<IStreamSocketStatics>>::get_activation_factory().get_endpoint_pairs_with_sort_options_async(remoteHostName, remoteServiceName, sortOptions)
    }
}
DEFINE_CLSID!(StreamSocket(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,116,114,101,97,109,83,111,99,107,101,116,0]) [CLSID_StreamSocket]);
DEFINE_IID!(IID_IStreamSocket2, 701556085, 62228, 19721, 173, 240, 15, 189, 150, 127, 189, 159);
RT_INTERFACE!{interface IStreamSocket2(IStreamSocket2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocket2] {
    fn ConnectWithProtectionLevelAndAdapterAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, protectionLevel: SocketProtectionLevel, adapter: *mut super::connectivity::NetworkAdapter, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IStreamSocket2 {
    #[inline] pub fn connect_with_protection_level_and_adapter_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg, protectionLevel: SocketProtectionLevel, adapter: &super::connectivity::NetworkAdapter) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectWithProtectionLevelAndAdapterAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), protectionLevel, adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocket3, 1061358336, 40232, 18516, 186, 195, 35, 1, 148, 30, 194, 35);
RT_INTERFACE!{interface IStreamSocket3(IStreamSocket3Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocket3] {
    fn CancelIOAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn EnableTransferOwnership(&self, taskId: Guid) -> HRESULT,
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> HRESULT,
    fn TransferOwnership(&self, socketId: HSTRING) -> HRESULT,
    fn TransferOwnershipWithContext(&self, socketId: HSTRING, data: *mut SocketActivityContext) -> HRESULT,
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketId: HSTRING, data: *mut SocketActivityContext, keepAliveTime: foundation::TimeSpan) -> HRESULT
}}
impl IStreamSocket3 {
    #[inline] pub fn cancel_io_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CancelIOAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership(&self, taskId: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnership)(self as *const _ as *mut _, taskId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership_with_connected_standby_action(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnershipWithConnectedStandbyAction)(self as *const _ as *mut _, taskId, connectedStandbyAction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership(&self, socketId: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnership)(self as *const _ as *mut _, socketId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership_with_context(&self, socketId: &HStringArg, data: &SocketActivityContext) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnershipWithContext)(self as *const _ as *mut _, socketId.get(), data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership_with_context_and_keep_alive_time(&self, socketId: &HStringArg, data: &SocketActivityContext, keepAliveTime: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnershipWithContextAndKeepAliveTime)(self as *const _ as *mut _, socketId.get(), data as *const _ as *mut _, keepAliveTime);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketControl, 4263882225, 37547, 19187, 153, 146, 15, 76, 133, 227, 108, 196);
RT_INTERFACE!{interface IStreamSocketControl(IStreamSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketControl] {
    fn get_NoDelay(&self, out: *mut bool) -> HRESULT,
    fn put_NoDelay(&self, value: bool) -> HRESULT,
    fn get_KeepAlive(&self, out: *mut bool) -> HRESULT,
    fn put_KeepAlive(&self, value: bool) -> HRESULT,
    fn get_OutboundBufferSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn put_OutboundBufferSizeInBytes(&self, value: u32) -> HRESULT,
    fn get_QualityOfService(&self, out: *mut SocketQualityOfService) -> HRESULT,
    fn put_QualityOfService(&self, value: SocketQualityOfService) -> HRESULT,
    fn get_OutboundUnicastHopLimit(&self, out: *mut u8) -> HRESULT,
    fn put_OutboundUnicastHopLimit(&self, value: u8) -> HRESULT
}}
impl IStreamSocketControl {
    #[inline] pub fn get_no_delay(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NoDelay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_no_delay(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NoDelay)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keep_alive(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeepAlive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_keep_alive(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_KeepAlive)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_buffer_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundBufferSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_buffer_size_in_bytes(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundBufferSizeInBytes)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_quality_of_service(&self) -> Result<SocketQualityOfService> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_QualityOfService)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_quality_of_service(&self, value: SocketQualityOfService) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_QualityOfService)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_unicast_hop_limit(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundUnicastHopLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_unicast_hop_limit(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundUnicastHopLimit)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketControl: IStreamSocketControl}
DEFINE_IID!(IID_IStreamSocketControl2, 3268450902, 1551, 17601, 184, 226, 31, 191, 96, 189, 98, 197);
RT_INTERFACE!{interface IStreamSocketControl2(IStreamSocketControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketControl2] {
    #[cfg(feature="windows-security")] fn get_IgnorableServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVector<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT
}}
impl IStreamSocketControl2 {
    #[cfg(feature="windows-security")] #[inline] pub fn get_ignorable_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IgnorableServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketControl3, 3312075852, 20084, 16446, 137, 76, 179, 28, 174, 92, 115, 66);
RT_INTERFACE!{interface IStreamSocketControl3(IStreamSocketControl3Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketControl3] {
    fn get_SerializeConnectionAttempts(&self, out: *mut bool) -> HRESULT,
    fn put_SerializeConnectionAttempts(&self, value: bool) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ClientCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(feature="windows-security")] fn put_ClientCertificate(&self, value: *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT
}}
impl IStreamSocketControl3 {
    #[inline] pub fn get_serialize_connection_attempts(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SerializeConnectionAttempts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_serialize_connection_attempts(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SerializeConnectionAttempts)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_client_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_client_certificate(&self, value: &super::super::security::cryptography::certificates::Certificate) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ClientCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketControl4, 2521705277, 60455, 18568, 179, 206, 199, 75, 65, 132, 35, 173);
RT_INTERFACE!{interface IStreamSocketControl4(IStreamSocketControl4Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketControl4] {
    fn get_MinProtectionLevel(&self, out: *mut SocketProtectionLevel) -> HRESULT,
    fn put_MinProtectionLevel(&self, value: SocketProtectionLevel) -> HRESULT
}}
impl IStreamSocketControl4 {
    #[inline] pub fn get_min_protection_level(&self) -> Result<SocketProtectionLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinProtectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_min_protection_level(&self, value: SocketProtectionLevel) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MinProtectionLevel)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketInformation, 998288944, 24168, 16901, 136, 240, 220, 133, 210, 226, 93, 237);
RT_INTERFACE!{interface IStreamSocketInformation(IStreamSocketInformationVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketInformation] {
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_LocalPort(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RemoteHostName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_RemoteAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_RemoteServiceName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RemotePort(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RoundTripTimeStatistics(&self, out: *mut RoundTripTimeStatistics) -> HRESULT,
    fn get_BandwidthStatistics(&self, out: *mut BandwidthStatistics) -> HRESULT,
    fn get_ProtectionLevel(&self, out: *mut SocketProtectionLevel) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_SessionKey(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IStreamSocketInformation {
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalPort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_host_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteHostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_service_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteServiceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemotePort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_round_trip_time_statistics(&self) -> Result<RoundTripTimeStatistics> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoundTripTimeStatistics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bandwidth_statistics(&self) -> Result<BandwidthStatistics> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BandwidthStatistics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_protection_level(&self) -> Result<SocketProtectionLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProtectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_session_key(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionKey)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketInformation: IStreamSocketInformation}
DEFINE_IID!(IID_IStreamSocketInformation2, 314737746, 19420, 20196, 151, 106, 207, 19, 14, 157, 146, 227);
RT_INTERFACE!{interface IStreamSocketInformation2(IStreamSocketInformation2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketInformation2] {
    fn get_ServerCertificateErrorSeverity(&self, out: *mut SocketSslErrorSeverity) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>) -> HRESULT
}}
impl IStreamSocketInformation2 {
    #[inline] pub fn get_server_certificate_error_severity(&self) -> Result<SocketSslErrorSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrorSeverity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_intermediate_certificates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerIntermediateCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketListener, 4283511863, 57247, 19952, 191, 130, 14, 197, 215, 179, 90, 174);
RT_INTERFACE!{interface IStreamSocketListener(IStreamSocketListenerVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListener] {
    fn get_Control(&self, out: *mut *mut StreamSocketListenerControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut StreamSocketListenerInformation) -> HRESULT,
    fn BindServiceNameAsync(&self, localServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn BindEndpointAsync(&self, localHostName: *mut super::HostName, localServiceName: HSTRING, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn add_ConnectionReceived(&self, eventHandler: *mut foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ConnectionReceived(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IStreamSocketListener {
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<StreamSocketListenerControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<StreamSocketListenerInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn bind_service_name_async(&self, localServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindServiceNameAsync)(self as *const _ as *mut _, localServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn bind_endpoint_async(&self, localHostName: &super::HostName, localServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindEndpointAsync)(self as *const _ as *mut _, localHostName as *const _ as *mut _, localServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_connection_received(&self, eventHandler: &foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ConnectionReceived)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_connection_received(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ConnectionReceived)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketListener: IStreamSocketListener}
impl RtActivatable<IActivationFactory> for StreamSocketListener {}
DEFINE_CLSID!(StreamSocketListener(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,116,114,101,97,109,83,111,99,107,101,116,76,105,115,116,101,110,101,114,0]) [CLSID_StreamSocketListener]);
DEFINE_IID!(IID_IStreamSocketListener2, 1703788862, 47934, 17496, 178, 50, 237, 16, 136, 105, 75, 152);
RT_INTERFACE!{interface IStreamSocketListener2(IStreamSocketListener2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListener2] {
    fn BindServiceNameWithProtectionLevelAsync(&self, localServiceName: HSTRING, protectionLevel: SocketProtectionLevel, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn BindServiceNameWithProtectionLevelAndAdapterAsync(&self, localServiceName: HSTRING, protectionLevel: SocketProtectionLevel, adapter: *mut super::connectivity::NetworkAdapter, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IStreamSocketListener2 {
    #[inline] pub fn bind_service_name_with_protection_level_async(&self, localServiceName: &HStringArg, protectionLevel: SocketProtectionLevel) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindServiceNameWithProtectionLevelAsync)(self as *const _ as *mut _, localServiceName.get(), protectionLevel, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn bind_service_name_with_protection_level_and_adapter_async(&self, localServiceName: &HStringArg, protectionLevel: SocketProtectionLevel, adapter: &super::connectivity::NetworkAdapter) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BindServiceNameWithProtectionLevelAndAdapterAsync)(self as *const _ as *mut _, localServiceName.get(), protectionLevel, adapter as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketListener3, 1201152028, 48632, 18713, 133, 66, 40, 212, 80, 231, 69, 7);
RT_INTERFACE!{interface IStreamSocketListener3(IStreamSocketListener3Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListener3] {
    fn CancelIOAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn EnableTransferOwnership(&self, taskId: Guid) -> HRESULT,
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> HRESULT,
    fn TransferOwnership(&self, socketId: HSTRING) -> HRESULT,
    fn TransferOwnershipWithContext(&self, socketId: HSTRING, data: *mut SocketActivityContext) -> HRESULT
}}
impl IStreamSocketListener3 {
    #[inline] pub fn cancel_io_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CancelIOAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership(&self, taskId: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnership)(self as *const _ as *mut _, taskId);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn enable_transfer_ownership_with_connected_standby_action(&self, taskId: Guid, connectedStandbyAction: SocketActivityConnectedStandbyAction) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnableTransferOwnershipWithConnectedStandbyAction)(self as *const _ as *mut _, taskId, connectedStandbyAction);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership(&self, socketId: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnership)(self as *const _ as *mut _, socketId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn transfer_ownership_with_context(&self, socketId: &HStringArg, data: &SocketActivityContext) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TransferOwnershipWithContext)(self as *const _ as *mut _, socketId.get(), data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketListenerConnectionReceivedEventArgs, 205991593, 14143, 17531, 133, 177, 221, 212, 84, 136, 3, 186);
RT_INTERFACE!{interface IStreamSocketListenerConnectionReceivedEventArgs(IStreamSocketListenerConnectionReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListenerConnectionReceivedEventArgs] {
    fn get_Socket(&self, out: *mut *mut StreamSocket) -> HRESULT
}}
impl IStreamSocketListenerConnectionReceivedEventArgs {
    #[inline] pub fn get_socket(&self) -> Result<Option<ComPtr<StreamSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Socket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketListenerConnectionReceivedEventArgs: IStreamSocketListenerConnectionReceivedEventArgs}
DEFINE_IID!(IID_IStreamSocketListenerControl, 551077238, 36234, 19898, 151, 34, 161, 108, 77, 152, 73, 128);
RT_INTERFACE!{interface IStreamSocketListenerControl(IStreamSocketListenerControlVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListenerControl] {
    fn get_QualityOfService(&self, out: *mut SocketQualityOfService) -> HRESULT,
    fn put_QualityOfService(&self, value: SocketQualityOfService) -> HRESULT
}}
impl IStreamSocketListenerControl {
    #[inline] pub fn get_quality_of_service(&self) -> Result<SocketQualityOfService> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_QualityOfService)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_quality_of_service(&self, value: SocketQualityOfService) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_QualityOfService)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketListenerControl: IStreamSocketListenerControl}
DEFINE_IID!(IID_IStreamSocketListenerControl2, 2492184165, 11326, 16459, 184, 176, 142, 178, 73, 162, 176, 161);
RT_INTERFACE!{interface IStreamSocketListenerControl2(IStreamSocketListenerControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListenerControl2] {
    fn get_NoDelay(&self, out: *mut bool) -> HRESULT,
    fn put_NoDelay(&self, value: bool) -> HRESULT,
    fn get_KeepAlive(&self, out: *mut bool) -> HRESULT,
    fn put_KeepAlive(&self, value: bool) -> HRESULT,
    fn get_OutboundBufferSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn put_OutboundBufferSizeInBytes(&self, value: u32) -> HRESULT,
    fn get_OutboundUnicastHopLimit(&self, out: *mut u8) -> HRESULT,
    fn put_OutboundUnicastHopLimit(&self, value: u8) -> HRESULT
}}
impl IStreamSocketListenerControl2 {
    #[inline] pub fn get_no_delay(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NoDelay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_no_delay(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NoDelay)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keep_alive(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeepAlive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_keep_alive(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_KeepAlive)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_buffer_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundBufferSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_buffer_size_in_bytes(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundBufferSizeInBytes)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_outbound_unicast_hop_limit(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundUnicastHopLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_unicast_hop_limit(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundUnicastHopLimit)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamSocketListenerInformation, 3861620783, 42554, 17163, 191, 98, 41, 233, 62, 86, 51, 180);
RT_INTERFACE!{interface IStreamSocketListenerInformation(IStreamSocketListenerInformationVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketListenerInformation] {
    fn get_LocalPort(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStreamSocketListenerInformation {
    #[inline] pub fn get_local_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalPort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamSocketListenerInformation: IStreamSocketListenerInformation}
DEFINE_IID!(IID_IStreamSocketStatics, 2753608778, 28206, 19189, 181, 86, 53, 90, 224, 205, 79, 41);
RT_INTERFACE!{static interface IStreamSocketStatics(IStreamSocketStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IStreamSocketStatics] {
    fn GetEndpointPairsAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>) -> HRESULT,
    fn GetEndpointPairsWithSortOptionsAsync(&self, remoteHostName: *mut super::HostName, remoteServiceName: HSTRING, sortOptions: super::HostNameSortOptions, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>) -> HRESULT
}}
impl IStreamSocketStatics {
    #[inline] pub fn get_endpoint_pairs_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetEndpointPairsAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_endpoint_pairs_with_sort_options_async(&self, remoteHostName: &super::HostName, remoteServiceName: &HStringArg, sortOptions: super::HostNameSortOptions) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::EndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetEndpointPairsWithSortOptionsAsync)(self as *const _ as *mut _, remoteHostName as *const _ as *mut _, remoteServiceName.get(), sortOptions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamWebSocket, 3175762392, 45705, 17851, 151, 235, 199, 82, 82, 5, 168, 67);
RT_INTERFACE!{interface IStreamWebSocket(IStreamWebSocketVtbl): IInspectable(IInspectableVtbl) [IID_IStreamWebSocket] {
    fn get_Control(&self, out: *mut *mut StreamWebSocketControl) -> HRESULT,
    fn get_Information(&self, out: *mut *mut StreamWebSocketInformation) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_InputStream(&self, out: *mut *mut super::super::storage::streams::IInputStream) -> HRESULT
}}
impl IStreamWebSocket {
    #[inline] pub fn get_control(&self) -> Result<Option<ComPtr<StreamWebSocketControl>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Control)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_information(&self) -> Result<Option<ComPtr<StreamWebSocketInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Information)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamWebSocket: IStreamWebSocket}
impl RtActivatable<IActivationFactory> for StreamWebSocket {}
DEFINE_CLSID!(StreamWebSocket(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,83,116,114,101,97,109,87,101,98,83,111,99,107,101,116,0]) [CLSID_StreamWebSocket]);
DEFINE_IID!(IID_IStreamWebSocket2, 2857175243, 37877, 18040, 130, 54, 87, 204, 229, 65, 126, 213);
RT_INTERFACE!{interface IStreamWebSocket2(IStreamWebSocket2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamWebSocket2] {
    fn add_ServerCustomValidationRequested(&self, eventHandler: *mut foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ServerCustomValidationRequested(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IStreamWebSocket2 {
    #[inline] pub fn add_server_custom_validation_requested(&self, eventHandler: &foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ServerCustomValidationRequested)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_server_custom_validation_requested(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ServerCustomValidationRequested)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStreamWebSocketControl, 3035920561, 42074, 18651, 149, 58, 100, 91, 125, 150, 76, 7);
RT_INTERFACE!{interface IStreamWebSocketControl(IStreamWebSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IStreamWebSocketControl] {
    fn get_NoDelay(&self, out: *mut bool) -> HRESULT,
    fn put_NoDelay(&self, value: bool) -> HRESULT
}}
impl IStreamWebSocketControl {
    #[inline] pub fn get_no_delay(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NoDelay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_no_delay(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NoDelay)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamWebSocketControl: IStreamWebSocketControl}
DEFINE_IID!(IID_IStreamWebSocketControl2, 559783806, 64088, 16602, 159, 17, 164, 141, 175, 233, 80, 55);
RT_INTERFACE!{interface IStreamWebSocketControl2(IStreamWebSocketControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IStreamWebSocketControl2] {
    fn get_DesiredUnsolicitedPongInterval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn put_DesiredUnsolicitedPongInterval(&self, value: foundation::TimeSpan) -> HRESULT,
    fn get_ActualUnsolicitedPongInterval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ClientCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    #[cfg(feature="windows-security")] fn put_ClientCertificate(&self, value: *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT
}}
impl IStreamWebSocketControl2 {
    #[inline] pub fn get_desired_unsolicited_pong_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredUnsolicitedPongInterval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_desired_unsolicited_pong_interval(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DesiredUnsolicitedPongInterval)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_actual_unsolicited_pong_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActualUnsolicitedPongInterval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_client_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ClientCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_client_certificate(&self, value: &super::super::security::cryptography::certificates::Certificate) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ClientCertificate)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamWebSocketInformation: IWebSocketInformation}
DEFINE_IID!(IID_IWebSocket, 4168563055, 39345, 19992, 188, 8, 133, 12, 154, 223, 21, 110);
RT_INTERFACE!{interface IWebSocket(IWebSocketVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocket] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_OutputStream(&self, out: *mut *mut super::super::storage::streams::IOutputStream) -> HRESULT,
    fn ConnectAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn SetRequestHeader(&self, headerName: HSTRING, headerValue: HSTRING) -> HRESULT,
    fn add_Closed(&self, eventHandler: *mut foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn CloseWithStatus(&self, code: u16, reason: HSTRING) -> HRESULT
}}
impl IWebSocket {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_async(&self, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_request_header(&self, headerName: &HStringArg, headerValue: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetRequestHeader)(self as *const _ as *mut _, headerName.get(), headerValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_closed(&self, eventHandler: &foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn close_with_status(&self, code: u16, reason: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CloseWithStatus)(self as *const _ as *mut _, code, reason.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebSocketClosedEventArgs, 3468135687, 53416, 18179, 160, 145, 200, 194, 192, 145, 91, 195);
RT_INTERFACE!{interface IWebSocketClosedEventArgs(IWebSocketClosedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketClosedEventArgs] {
    fn get_Code(&self, out: *mut u16) -> HRESULT,
    fn get_Reason(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebSocketClosedEventArgs {
    #[inline] pub fn get_code(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Code)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_reason(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebSocketClosedEventArgs: IWebSocketClosedEventArgs}
DEFINE_IID!(IID_IWebSocketControl, 784645571, 55717, 17754, 152, 17, 222, 36, 212, 83, 55, 233);
RT_INTERFACE!{interface IWebSocketControl(IWebSocketControlVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketControl] {
    fn get_OutboundBufferSizeInBytes(&self, out: *mut u32) -> HRESULT,
    fn put_OutboundBufferSizeInBytes(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ServerCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ProxyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-security")] fn put_ProxyCredential(&self, value: *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    fn get_SupportedProtocols(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT
}}
impl IWebSocketControl {
    #[inline] pub fn get_outbound_buffer_size_in_bytes(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutboundBufferSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_outbound_buffer_size_in_bytes(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OutboundBufferSizeInBytes)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_server_credential(&self, value: &super::super::security::credentials::PasswordCredential) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ServerCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_proxy_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn set_proxy_credential(&self, value: &super::super::security::credentials::PasswordCredential) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProxyCredential)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_protocols(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedProtocols)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebSocketControl2, 2042871299, 62154, 17950, 175, 78, 150, 101, 188, 45, 6, 32);
RT_INTERFACE!{interface IWebSocketControl2(IWebSocketControl2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketControl2] {
    #[cfg(feature="windows-security")] fn get_IgnorableServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVector<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT
}}
impl IWebSocketControl2 {
    #[cfg(feature="windows-security")] #[inline] pub fn get_ignorable_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IgnorableServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class WebSocketError}
impl RtActivatable<IWebSocketErrorStatics> for WebSocketError {}
impl WebSocketError {
    #[cfg(feature="windows-web")] #[inline] pub fn get_status(hresult: i32) -> Result<super::super::web::WebErrorStatus> {
        <Self as RtActivatable<IWebSocketErrorStatics>>::get_activation_factory().get_status(hresult)
    }
}
DEFINE_CLSID!(WebSocketError(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,87,101,98,83,111,99,107,101,116,69,114,114,111,114,0]) [CLSID_WebSocketError]);
DEFINE_IID!(IID_IWebSocketErrorStatics, 667808603, 8033, 18185, 142, 2, 97, 40, 58, 218, 78, 157);
RT_INTERFACE!{static interface IWebSocketErrorStatics(IWebSocketErrorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketErrorStatics] {
    #[cfg(feature="windows-web")] fn GetStatus(&self, hresult: i32, out: *mut super::super::web::WebErrorStatus) -> HRESULT
}}
impl IWebSocketErrorStatics {
    #[cfg(feature="windows-web")] #[inline] pub fn get_status(&self, hresult: i32) -> Result<super::super::web::WebErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetStatus)(self as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebSocketInformation, 1577181974, 51498, 18341, 178, 95, 7, 132, 118, 57, 209, 129);
RT_INTERFACE!{interface IWebSocketInformation(IWebSocketInformationVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketInformation] {
    fn get_LocalAddress(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_BandwidthStatistics(&self, out: *mut BandwidthStatistics) -> HRESULT,
    fn get_Protocol(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebSocketInformation {
    #[inline] pub fn get_local_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bandwidth_statistics(&self) -> Result<BandwidthStatistics> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BandwidthStatistics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_protocol(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Protocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebSocketInformation2, 3458021838, 41399, 19779, 130, 105, 141, 91, 152, 27, 212, 122);
RT_INTERFACE!{interface IWebSocketInformation2(IWebSocketInformation2Vtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketInformation2] {
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    fn get_ServerCertificateErrorSeverity(&self, out: *mut SocketSslErrorSeverity) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>) -> HRESULT
}}
impl IWebSocketInformation2 {
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_server_certificate_error_severity(&self) -> Result<SocketSslErrorSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrorSeverity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_intermediate_certificates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerIntermediateCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
#[cfg(feature="windows-applicationmodel")] RT_CLASS!{class WebSocketKeepAlive: super::super::applicationmodel::background::IBackgroundTask}
#[cfg(not(feature="windows-applicationmodel"))] RT_CLASS!{class WebSocketKeepAlive: IInspectable}
impl RtActivatable<IActivationFactory> for WebSocketKeepAlive {}
DEFINE_CLSID!(WebSocketKeepAlive(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,83,111,99,107,101,116,115,46,87,101,98,83,111,99,107,101,116,75,101,101,112,65,108,105,118,101,0]) [CLSID_WebSocketKeepAlive]);
DEFINE_IID!(IID_IWebSocketServerCustomValidationRequestedEventArgs, 4293918280, 554, 19127, 139, 54, 225, 10, 244, 100, 14, 107);
RT_INTERFACE!{interface IWebSocketServerCustomValidationRequestedEventArgs(IWebSocketServerCustomValidationRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IWebSocketServerCustomValidationRequestedEventArgs] {
    #[cfg(not(feature="windows-security"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificate(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    fn get_ServerCertificateErrorSeverity(&self, out: *mut SocketSslErrorSeverity) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerCertificateErrors(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn get_ServerIntermediateCertificates(&self, out: *mut *mut foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>) -> HRESULT,
    fn Reject(&self) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IWebSocketServerCustomValidationRequestedEventArgs {
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_server_certificate_error_severity(&self) -> Result<SocketSslErrorSeverity> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrorSeverity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_certificate_errors(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::ChainValidationResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerCertificateErrors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_server_intermediate_certificates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::security::cryptography::certificates::Certificate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerIntermediateCertificates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn reject(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Reject)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebSocketServerCustomValidationRequestedEventArgs: IWebSocketServerCustomValidationRequestedEventArgs}
} // Windows.Networking.Sockets
pub mod vpn { // Windows.Networking.Vpn
use crate::prelude::*;
DEFINE_IID!(IID_IVpnAppId, 2064033333, 23640, 16857, 148, 167, 191, 188, 241, 216, 202, 84);
RT_INTERFACE!{interface IVpnAppId(IVpnAppIdVtbl): IInspectable(IInspectableVtbl) [IID_IVpnAppId] {
    fn get_Type(&self, out: *mut VpnAppIdType) -> HRESULT,
    fn put_Type(&self, value: VpnAppIdType) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IVpnAppId {
    #[inline] pub fn get_type(&self) -> Result<VpnAppIdType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_type(&self, value: VpnAppIdType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Type)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnAppId: IVpnAppId}
impl RtActivatable<IVpnAppIdFactory> for VpnAppId {}
impl VpnAppId {
    #[inline] pub fn create(type_: VpnAppIdType, value: &HStringArg) -> Result<ComPtr<VpnAppId>> {
        <Self as RtActivatable<IVpnAppIdFactory>>::get_activation_factory().create(type_, value)
    }
}
DEFINE_CLSID!(VpnAppId(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,65,112,112,73,100,0]) [CLSID_VpnAppId]);
DEFINE_IID!(IID_IVpnAppIdFactory, 1185807658, 2731, 20443, 130, 29, 211, 221, 201, 25, 120, 139);
RT_INTERFACE!{static interface IVpnAppIdFactory(IVpnAppIdFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnAppIdFactory] {
    fn Create(&self, type_: VpnAppIdType, value: HSTRING, out: *mut *mut VpnAppId) -> HRESULT
}}
impl IVpnAppIdFactory {
    #[inline] pub fn create(&self, type_: VpnAppIdType, value: &HStringArg) -> Result<ComPtr<VpnAppId>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, type_, value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnAppIdType: i32 {
    PackageFamilyName = 0, FullyQualifiedBinaryName = 1, FilePath = 2,
}}
RT_ENUM! { enum VpnAuthenticationMethod: i32 {
    Mschapv2 = 0, Eap = 1, Certificate = 2, PresharedKey = 3,
}}
DEFINE_IID!(IID_IVpnChannel, 1254591751, 53672, 17155, 160, 145, 200, 210, 224, 145, 91, 195);
RT_INTERFACE!{interface IVpnChannel(IVpnChannelVtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannel] {
    fn AssociateTransport(&self, mainOuterTunnelTransport: *mut IInspectable, optionalOuterTunnelTransport: *mut IInspectable) -> HRESULT,
    fn Start(&self, assignedClientIPv4list: *mut foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: *mut foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: *mut VpnInterfaceId, routeScope: *mut VpnRouteAssignment, namespaceScope: *mut VpnNamespaceAssignment, mtuSize: u32, maxFrameSize: u32, optimizeForLowCostNetwork: bool, mainOuterTunnelTransport: *mut IInspectable, optionalOuterTunnelTransport: *mut IInspectable) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-security")] fn RequestCredentials(&self, credType: VpnCredentialType, isRetry: bool, isSingleSignOnCredential: bool, certificate: *mut super::super::security::cryptography::certificates::Certificate, out: *mut *mut VpnPickedCredential) -> HRESULT,
    fn RequestVpnPacketBuffer(&self, type_: VpnDataPathType, vpnPacketBuffer: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn LogDiagnosticMessage(&self, message: HSTRING) -> HRESULT,
    fn get_Id(&self, out: *mut u32) -> HRESULT,
    fn get_Configuration(&self, out: *mut *mut VpnChannelConfiguration) -> HRESULT,
    fn add_ActivityChange(&self, handler: *mut foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ActivityChange(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn put_PlugInContext(&self, value: *mut IInspectable) -> HRESULT,
    fn get_PlugInContext(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_SystemHealth(&self, out: *mut *mut VpnSystemHealth) -> HRESULT,
    fn RequestCustomPrompt(&self, customPrompt: *mut foundation::collections::IVectorView<IVpnCustomPrompt>) -> HRESULT,
    fn SetErrorMessage(&self, message: HSTRING) -> HRESULT,
    fn SetAllowedSslTlsVersions(&self, tunnelTransport: *mut IInspectable, useTls12: bool) -> HRESULT
}}
impl IVpnChannel {
    #[inline] pub fn associate_transport(&self, mainOuterTunnelTransport: &IInspectable, optionalOuterTunnelTransport: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AssociateTransport)(self as *const _ as *mut _, mainOuterTunnelTransport as *const _ as *mut _, optionalOuterTunnelTransport as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start(&self, assignedClientIPv4list: &foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: &foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: &VpnInterfaceId, routeScope: &VpnRouteAssignment, namespaceScope: &VpnNamespaceAssignment, mtuSize: u32, maxFrameSize: u32, optimizeForLowCostNetwork: bool, mainOuterTunnelTransport: &IInspectable, optionalOuterTunnelTransport: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _, assignedClientIPv4list as *const _ as *mut _, assignedClientIPv6list as *const _ as *mut _, vpnInterfaceId as *const _ as *mut _, routeScope as *const _ as *mut _, namespaceScope as *const _ as *mut _, mtuSize, maxFrameSize, optimizeForLowCostNetwork, mainOuterTunnelTransport as *const _ as *mut _, optionalOuterTunnelTransport as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn request_credentials(&self, credType: VpnCredentialType, isRetry: bool, isSingleSignOnCredential: bool, certificate: &super::super::security::cryptography::certificates::Certificate) -> Result<Option<ComPtr<VpnPickedCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCredentials)(self as *const _ as *mut _, credType, isRetry, isSingleSignOnCredential, certificate as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_vpn_packet_buffer(&self, type_: VpnDataPathType) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut vpnPacketBuffer = null_mut();
        let hr = ((*self.lpVtbl).RequestVpnPacketBuffer)(self as *const _ as *mut _, type_, &mut vpnPacketBuffer);
        if hr == S_OK { Ok(ComPtr::wrap_optional(vpnPacketBuffer)) } else { err(hr) }
    }}
    #[inline] pub fn log_diagnostic_message(&self, message: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).LogDiagnosticMessage)(self as *const _ as *mut _, message.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<ComPtr<VpnChannelConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Configuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_activity_change(&self, handler: &foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ActivityChange)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_activity_change(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ActivityChange)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_plug_in_context(&self, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PlugInContext)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_plug_in_context(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PlugInContext)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_health(&self) -> Result<Option<ComPtr<VpnSystemHealth>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemHealth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_custom_prompt(&self, customPrompt: &foundation::collections::IVectorView<IVpnCustomPrompt>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RequestCustomPrompt)(self as *const _ as *mut _, customPrompt as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_error_message(&self, message: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetErrorMessage)(self as *const _ as *mut _, message.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_allowed_ssl_tls_versions(&self, tunnelTransport: &IInspectable, useTls12: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetAllowedSslTlsVersions)(self as *const _ as *mut _, tunnelTransport as *const _ as *mut _, useTls12);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnChannel: IVpnChannel}
impl RtActivatable<IVpnChannelStatics> for VpnChannel {}
impl VpnChannel {
    #[inline] pub fn process_event_async(thirdPartyPlugIn: &IInspectable, event: &IInspectable) -> Result<()> {
        <Self as RtActivatable<IVpnChannelStatics>>::get_activation_factory().process_event_async(thirdPartyPlugIn, event)
    }
}
DEFINE_CLSID!(VpnChannel(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,104,97,110,110,101,108,0]) [CLSID_VpnChannel]);
DEFINE_IID!(IID_IVpnChannel2, 576049509, 39227, 17961, 173, 96, 241, 195, 243, 83, 127, 80);
RT_INTERFACE!{interface IVpnChannel2(IVpnChannel2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannel2] {
    fn StartWithMainTransport(&self, assignedClientIPv4list: *mut foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: *mut foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: *mut VpnInterfaceId, assignedRoutes: *mut VpnRouteAssignment, assignedDomainName: *mut VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, mainOuterTunnelTransport: *mut IInspectable) -> HRESULT,
    fn StartExistingTransports(&self, assignedClientIPv4list: *mut foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: *mut foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: *mut VpnInterfaceId, assignedRoutes: *mut VpnRouteAssignment, assignedDomainName: *mut VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool) -> HRESULT,
    fn add_ActivityStateChange(&self, handler: *mut foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ActivityStateChange(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn GetVpnSendPacketBuffer(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn GetVpnReceivePacketBuffer(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn RequestCustomPromptAsync(&self, customPromptElement: *mut foundation::collections::IVectorView<IVpnCustomPromptElement>, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-security")] fn RequestCredentialsWithCertificateAsync(&self, credType: VpnCredentialType, credOptions: u32, certificate: *mut super::super::security::cryptography::certificates::Certificate, out: *mut *mut foundation::IAsyncOperation<VpnCredential>) -> HRESULT,
    fn RequestCredentialsWithOptionsAsync(&self, credType: VpnCredentialType, credOptions: u32, out: *mut *mut foundation::IAsyncOperation<VpnCredential>) -> HRESULT,
    fn RequestCredentialsSimpleAsync(&self, credType: VpnCredentialType, out: *mut *mut foundation::IAsyncOperation<VpnCredential>) -> HRESULT,
    fn TerminateConnection(&self, message: HSTRING) -> HRESULT,
    fn StartWithTrafficFilter(&self, assignedClientIpv4List: *mut foundation::collections::IVectorView<super::HostName>, assignedClientIpv6List: *mut foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: *mut VpnInterfaceId, assignedRoutes: *mut VpnRouteAssignment, assignedNamespace: *mut VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, mainOuterTunnelTransport: *mut IInspectable, optionalOuterTunnelTransport: *mut IInspectable, assignedTrafficFilters: *mut VpnTrafficFilterAssignment) -> HRESULT
}}
impl IVpnChannel2 {
    #[inline] pub fn start_with_main_transport(&self, assignedClientIPv4list: &foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: &foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: &VpnInterfaceId, assignedRoutes: &VpnRouteAssignment, assignedDomainName: &VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, mainOuterTunnelTransport: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartWithMainTransport)(self as *const _ as *mut _, assignedClientIPv4list as *const _ as *mut _, assignedClientIPv6list as *const _ as *mut _, vpnInterfaceId as *const _ as *mut _, assignedRoutes as *const _ as *mut _, assignedDomainName as *const _ as *mut _, mtuSize, maxFrameSize, reserved, mainOuterTunnelTransport as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_existing_transports(&self, assignedClientIPv4list: &foundation::collections::IVectorView<super::HostName>, assignedClientIPv6list: &foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: &VpnInterfaceId, assignedRoutes: &VpnRouteAssignment, assignedDomainName: &VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartExistingTransports)(self as *const _ as *mut _, assignedClientIPv4list as *const _ as *mut _, assignedClientIPv6list as *const _ as *mut _, vpnInterfaceId as *const _ as *mut _, assignedRoutes as *const _ as *mut _, assignedDomainName as *const _ as *mut _, mtuSize, maxFrameSize, reserved);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_activity_state_change(&self, handler: &foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ActivityStateChange)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_activity_state_change(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ActivityStateChange)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vpn_send_packet_buffer(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVpnSendPacketBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vpn_receive_packet_buffer(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVpnReceivePacketBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_custom_prompt_async(&self, customPromptElement: &foundation::collections::IVectorView<IVpnCustomPromptElement>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCustomPromptAsync)(self as *const _ as *mut _, customPromptElement as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn request_credentials_with_certificate_async(&self, credType: VpnCredentialType, credOptions: u32, certificate: &super::super::security::cryptography::certificates::Certificate) -> Result<ComPtr<foundation::IAsyncOperation<VpnCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCredentialsWithCertificateAsync)(self as *const _ as *mut _, credType, credOptions, certificate as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_credentials_with_options_async(&self, credType: VpnCredentialType, credOptions: u32) -> Result<ComPtr<foundation::IAsyncOperation<VpnCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCredentialsWithOptionsAsync)(self as *const _ as *mut _, credType, credOptions, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_credentials_simple_async(&self, credType: VpnCredentialType) -> Result<ComPtr<foundation::IAsyncOperation<VpnCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestCredentialsSimpleAsync)(self as *const _ as *mut _, credType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn terminate_connection(&self, message: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).TerminateConnection)(self as *const _ as *mut _, message.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_with_traffic_filter(&self, assignedClientIpv4List: &foundation::collections::IVectorView<super::HostName>, assignedClientIpv6List: &foundation::collections::IVectorView<super::HostName>, vpnInterfaceId: &VpnInterfaceId, assignedRoutes: &VpnRouteAssignment, assignedNamespace: &VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, mainOuterTunnelTransport: &IInspectable, optionalOuterTunnelTransport: &IInspectable, assignedTrafficFilters: &VpnTrafficFilterAssignment) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartWithTrafficFilter)(self as *const _ as *mut _, assignedClientIpv4List as *const _ as *mut _, assignedClientIpv6List as *const _ as *mut _, vpnInterfaceId as *const _ as *mut _, assignedRoutes as *const _ as *mut _, assignedNamespace as *const _ as *mut _, mtuSize, maxFrameSize, reserved, mainOuterTunnelTransport as *const _ as *mut _, optionalOuterTunnelTransport as *const _ as *mut _, assignedTrafficFilters as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnChannel4, 3609620190, 10551, 16797, 149, 112, 72, 106, 235, 184, 24, 3);
RT_INTERFACE!{interface IVpnChannel4(IVpnChannel4Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannel4] {
    fn AddAndAssociateTransport(&self, transport: *mut IInspectable, context: *mut IInspectable) -> HRESULT,
    fn StartWithMultipleTransports(&self, assignedClientIpv4Addresses: *mut foundation::collections::IIterable<super::HostName>, assignedClientIpv6Addresses: *mut foundation::collections::IIterable<super::HostName>, vpninterfaceId: *mut VpnInterfaceId, assignedRoutes: *mut VpnRouteAssignment, assignedNamespace: *mut VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, transports: *mut foundation::collections::IIterable<IInspectable>, assignedTrafficFilters: *mut VpnTrafficFilterAssignment) -> HRESULT,
    fn ReplaceAndAssociateTransport(&self, transport: *mut IInspectable, context: *mut IInspectable) -> HRESULT,
    fn StartReconnectingTransport(&self, transport: *mut IInspectable, context: *mut IInspectable) -> HRESULT,
    fn GetSlotTypeForTransportContext(&self, context: *mut IInspectable, out: *mut super::sockets::ControlChannelTriggerStatus) -> HRESULT,
    fn get_CurrentRequestTransportContext(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IVpnChannel4 {
    #[inline] pub fn add_and_associate_transport(&self, transport: &IInspectable, context: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddAndAssociateTransport)(self as *const _ as *mut _, transport as *const _ as *mut _, context as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_with_multiple_transports(&self, assignedClientIpv4Addresses: &foundation::collections::IIterable<super::HostName>, assignedClientIpv6Addresses: &foundation::collections::IIterable<super::HostName>, vpninterfaceId: &VpnInterfaceId, assignedRoutes: &VpnRouteAssignment, assignedNamespace: &VpnDomainNameAssignment, mtuSize: u32, maxFrameSize: u32, reserved: bool, transports: &foundation::collections::IIterable<IInspectable>, assignedTrafficFilters: &VpnTrafficFilterAssignment) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartWithMultipleTransports)(self as *const _ as *mut _, assignedClientIpv4Addresses as *const _ as *mut _, assignedClientIpv6Addresses as *const _ as *mut _, vpninterfaceId as *const _ as *mut _, assignedRoutes as *const _ as *mut _, assignedNamespace as *const _ as *mut _, mtuSize, maxFrameSize, reserved, transports as *const _ as *mut _, assignedTrafficFilters as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn replace_and_associate_transport(&self, transport: &IInspectable, context: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ReplaceAndAssociateTransport)(self as *const _ as *mut _, transport as *const _ as *mut _, context as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_reconnecting_transport(&self, transport: &IInspectable, context: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartReconnectingTransport)(self as *const _ as *mut _, transport as *const _ as *mut _, context as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_slot_type_for_transport_context(&self, context: &IInspectable) -> Result<super::sockets::ControlChannelTriggerStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetSlotTypeForTransportContext)(self as *const _ as *mut _, context as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_request_transport_context(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentRequestTransportContext)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnChannelActivityEventArgs, 2741799154, 45020, 18293, 133, 93, 212, 172, 10, 53, 252, 85);
RT_INTERFACE!{interface IVpnChannelActivityEventArgs(IVpnChannelActivityEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannelActivityEventArgs] {
    fn get_Type(&self, out: *mut VpnChannelActivityEventType) -> HRESULT
}}
impl IVpnChannelActivityEventArgs {
    #[inline] pub fn get_type(&self) -> Result<VpnChannelActivityEventType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnChannelActivityEventArgs: IVpnChannelActivityEventArgs}
RT_ENUM! { enum VpnChannelActivityEventType: i32 {
    Idle = 0, Active = 1,
}}
DEFINE_IID!(IID_IVpnChannelActivityStateChangedArgs, 1031079269, 64960, 19390, 162, 59, 69, 255, 252, 109, 151, 161);
RT_INTERFACE!{interface IVpnChannelActivityStateChangedArgs(IVpnChannelActivityStateChangedArgsVtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannelActivityStateChangedArgs] {
    fn get_ActivityState(&self, out: *mut VpnChannelActivityEventType) -> HRESULT
}}
impl IVpnChannelActivityStateChangedArgs {
    #[inline] pub fn get_activity_state(&self) -> Result<VpnChannelActivityEventType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ActivityState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnChannelActivityStateChangedArgs: IVpnChannelActivityStateChangedArgs}
DEFINE_IID!(IID_IVpnChannelConfiguration, 237886626, 8210, 20452, 177, 121, 140, 101, 44, 109, 16, 126);
RT_INTERFACE!{interface IVpnChannelConfiguration(IVpnChannelConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannelConfiguration] {
    fn get_ServerServiceName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ServerHostNameList(&self, out: *mut *mut foundation::collections::IVectorView<super::HostName>) -> HRESULT,
    fn get_CustomField(&self, out: *mut HSTRING) -> HRESULT
}}
impl IVpnChannelConfiguration {
    #[inline] pub fn get_server_service_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerServiceName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_server_host_name_list(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerHostNameList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_field(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomField)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnChannelConfiguration: IVpnChannelConfiguration}
DEFINE_IID!(IID_IVpnChannelConfiguration2, 4077606732, 30756, 18204, 161, 24, 99, 219, 201, 58, 228, 199);
RT_INTERFACE!{interface IVpnChannelConfiguration2(IVpnChannelConfiguration2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannelConfiguration2] {
    fn get_ServerUris(&self, out: *mut *mut foundation::collections::IVectorView<foundation::Uri>) -> HRESULT
}}
impl IVpnChannelConfiguration2 {
    #[inline] pub fn get_server_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerUris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnChannelRequestCredentialsOptions: u32 {
    None = 0, Retrying = 1, UseForSingleSignIn = 2,
}}
DEFINE_IID!(IID_IVpnChannelStatics, 2297103917, 59416, 20477, 152, 166, 54, 62, 55, 54, 201, 93);
RT_INTERFACE!{static interface IVpnChannelStatics(IVpnChannelStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IVpnChannelStatics] {
    fn ProcessEventAsync(&self, thirdPartyPlugIn: *mut IInspectable, event: *mut IInspectable) -> HRESULT
}}
impl IVpnChannelStatics {
    #[inline] pub fn process_event_async(&self, thirdPartyPlugIn: &IInspectable, event: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ProcessEventAsync)(self as *const _ as *mut _, thirdPartyPlugIn as *const _ as *mut _, event as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnCredential, 3085404915, 42093, 16459, 135, 41, 24, 50, 82, 40, 83, 172);
RT_INTERFACE!{interface IVpnCredential(IVpnCredentialVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCredential] {
    #[cfg(feature="windows-security")] fn get_PasskeyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_CertificateCredential(&self, out: *mut *mut super::super::security::cryptography::certificates::Certificate) -> HRESULT,
    fn get_AdditionalPin(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_OldPasswordCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT
}}
impl IVpnCredential {
    #[cfg(feature="windows-security")] #[inline] pub fn get_passkey_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PasskeyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_certificate_credential(&self) -> Result<Option<ComPtr<super::super::security::cryptography::certificates::Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CertificateCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_additional_pin(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdditionalPin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_old_password_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OldPasswordCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCredential: IVpnCredential}
RT_ENUM! { enum VpnCredentialType: i32 {
    UsernamePassword = 0, UsernameOtpPin = 1, UsernamePasswordAndPin = 2, UsernamePasswordChange = 3, SmartCard = 4, ProtectedCertificate = 5, UnProtectedCertificate = 6,
}}
DEFINE_IID!(IID_IVpnCustomCheckBox, 1132955475, 965, 20065, 147, 215, 169, 87, 113, 76, 66, 130);
RT_INTERFACE!{interface IVpnCustomCheckBox(IVpnCustomCheckBoxVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomCheckBox] {
    fn put_InitialCheckState(&self, value: bool) -> HRESULT,
    fn get_InitialCheckState(&self, out: *mut bool) -> HRESULT,
    fn get_Checked(&self, out: *mut bool) -> HRESULT
}}
impl IVpnCustomCheckBox {
    #[inline] pub fn set_initial_check_state(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InitialCheckState)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_initial_check_state(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InitialCheckState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_checked(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Checked)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomCheckBox: IVpnCustomCheckBox}
impl RtActivatable<IActivationFactory> for VpnCustomCheckBox {}
DEFINE_CLSID!(VpnCustomCheckBox(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,67,104,101,99,107,66,111,120,0]) [CLSID_VpnCustomCheckBox]);
DEFINE_IID!(IID_IVpnCustomComboBox, 2586056078, 56225, 19567, 130, 112, 220, 243, 201, 118, 28, 76);
RT_INTERFACE!{interface IVpnCustomComboBox(IVpnCustomComboBoxVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomComboBox] {
    fn put_OptionsText(&self, value: *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_OptionsText(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Selected(&self, out: *mut u32) -> HRESULT
}}
impl IVpnCustomComboBox {
    #[inline] pub fn set_options_text(&self, value: &foundation::collections::IVectorView<HString>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_OptionsText)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_options_text(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OptionsText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_selected(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Selected)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomComboBox: IVpnCustomComboBox}
impl RtActivatable<IActivationFactory> for VpnCustomComboBox {}
DEFINE_CLSID!(VpnCustomComboBox(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,67,111,109,98,111,66,111,120,0]) [CLSID_VpnCustomComboBox]);
DEFINE_IID!(IID_IVpnCustomEditBox, 805493152, 53183, 19467, 143, 60, 102, 245, 3, 194, 11, 57);
RT_INTERFACE!{interface IVpnCustomEditBox(IVpnCustomEditBoxVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomEditBox] {
    fn put_DefaultText(&self, value: HSTRING) -> HRESULT,
    fn get_DefaultText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_NoEcho(&self, value: bool) -> HRESULT,
    fn get_NoEcho(&self, out: *mut bool) -> HRESULT,
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT
}}
impl IVpnCustomEditBox {
    #[inline] pub fn set_default_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DefaultText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_default_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DefaultText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_no_echo(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NoEcho)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_no_echo(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NoEcho)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomEditBox: IVpnCustomEditBox}
impl RtActivatable<IActivationFactory> for VpnCustomEditBox {}
DEFINE_CLSID!(VpnCustomEditBox(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,69,100,105,116,66,111,120,0]) [CLSID_VpnCustomEditBox]);
DEFINE_IID!(IID_IVpnCustomErrorBox, 2663706546, 51522, 17071, 178, 35, 88, 139, 72, 50, 135, 33);
RT_INTERFACE!{interface IVpnCustomErrorBox(IVpnCustomErrorBoxVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomErrorBox] {
    
}}
RT_CLASS!{class VpnCustomErrorBox: IVpnCustomErrorBox}
impl RtActivatable<IActivationFactory> for VpnCustomErrorBox {}
DEFINE_CLSID!(VpnCustomErrorBox(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,69,114,114,111,114,66,111,120,0]) [CLSID_VpnCustomErrorBox]);
DEFINE_IID!(IID_IVpnCustomPrompt, 2603531899, 34773, 17212, 180, 246, 238, 230, 170, 104, 162, 68);
RT_INTERFACE!{interface IVpnCustomPrompt(IVpnCustomPromptVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPrompt] {
    fn put_Label(&self, value: HSTRING) -> HRESULT,
    fn get_Label(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Compulsory(&self, value: bool) -> HRESULT,
    fn get_Compulsory(&self, out: *mut bool) -> HRESULT,
    fn put_Bordered(&self, value: bool) -> HRESULT,
    fn get_Bordered(&self, out: *mut bool) -> HRESULT
}}
impl IVpnCustomPrompt {
    #[inline] pub fn set_label(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Label)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_label(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Label)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_compulsory(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Compulsory)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_compulsory(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Compulsory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bordered(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Bordered)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bordered(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Bordered)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnCustomPromptBooleanInput, 3301549726, 65351, 17703, 159, 39, 164, 146, 146, 1, 153, 121);
RT_INTERFACE!{interface IVpnCustomPromptBooleanInput(IVpnCustomPromptBooleanInputVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPromptBooleanInput] {
    fn put_InitialValue(&self, value: bool) -> HRESULT,
    fn get_InitialValue(&self, out: *mut bool) -> HRESULT,
    fn get_Value(&self, out: *mut bool) -> HRESULT
}}
impl IVpnCustomPromptBooleanInput {
    #[inline] pub fn set_initial_value(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InitialValue)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_initial_value(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InitialValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomPromptBooleanInput: IVpnCustomPromptBooleanInput}
impl RtActivatable<IActivationFactory> for VpnCustomPromptBooleanInput {}
DEFINE_CLSID!(VpnCustomPromptBooleanInput(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,80,114,111,109,112,116,66,111,111,108,101,97,110,73,110,112,117,116,0]) [CLSID_VpnCustomPromptBooleanInput]);
DEFINE_IID!(IID_IVpnCustomPromptElement, 1941788216, 28420, 16461, 147, 221, 80, 164, 73, 36, 163, 139);
RT_INTERFACE!{interface IVpnCustomPromptElement(IVpnCustomPromptElementVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPromptElement] {
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Compulsory(&self, value: bool) -> HRESULT,
    fn get_Compulsory(&self, out: *mut bool) -> HRESULT,
    fn put_Emphasized(&self, value: bool) -> HRESULT,
    fn get_Emphasized(&self, out: *mut bool) -> HRESULT
}}
impl IVpnCustomPromptElement {
    #[inline] pub fn set_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_compulsory(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Compulsory)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_compulsory(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Compulsory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_emphasized(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Emphasized)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_emphasized(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Emphasized)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnCustomPromptOptionSelector, 999240921, 36545, 20117, 154, 78, 123, 166, 77, 56, 243, 48);
RT_INTERFACE!{interface IVpnCustomPromptOptionSelector(IVpnCustomPromptOptionSelectorVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPromptOptionSelector] {
    fn get_Options(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_SelectedIndex(&self, out: *mut u32) -> HRESULT
}}
impl IVpnCustomPromptOptionSelector {
    #[inline] pub fn get_options(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Options)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_selected_index(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SelectedIndex)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomPromptOptionSelector: IVpnCustomPromptOptionSelector}
impl RtActivatable<IActivationFactory> for VpnCustomPromptOptionSelector {}
DEFINE_CLSID!(VpnCustomPromptOptionSelector(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,80,114,111,109,112,116,79,112,116,105,111,110,83,101,108,101,99,116,111,114,0]) [CLSID_VpnCustomPromptOptionSelector]);
DEFINE_IID!(IID_IVpnCustomPromptText, 1003011566, 14914, 18851, 171, 221, 7, 178, 237, 234, 117, 45);
RT_INTERFACE!{interface IVpnCustomPromptText(IVpnCustomPromptTextVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPromptText] {
    fn put_Text(&self, value: HSTRING) -> HRESULT,
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT
}}
impl IVpnCustomPromptText {
    #[inline] pub fn set_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Text)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomPromptText: IVpnCustomPromptText}
impl RtActivatable<IActivationFactory> for VpnCustomPromptText {}
DEFINE_CLSID!(VpnCustomPromptText(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,80,114,111,109,112,116,84,101,120,116,0]) [CLSID_VpnCustomPromptText]);
DEFINE_IID!(IID_IVpnCustomPromptTextInput, 3386547317, 37180, 18389, 136, 186, 72, 252, 72, 147, 2, 53);
RT_INTERFACE!{interface IVpnCustomPromptTextInput(IVpnCustomPromptTextInputVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomPromptTextInput] {
    fn put_PlaceholderText(&self, value: HSTRING) -> HRESULT,
    fn get_PlaceholderText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IsTextHidden(&self, value: bool) -> HRESULT,
    fn get_IsTextHidden(&self, out: *mut bool) -> HRESULT,
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT
}}
impl IVpnCustomPromptTextInput {
    #[inline] pub fn set_placeholder_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PlaceholderText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_placeholder_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PlaceholderText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_is_text_hidden(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsTextHidden)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_text_hidden(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTextHidden)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Text)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomPromptTextInput: IVpnCustomPromptTextInput}
impl RtActivatable<IActivationFactory> for VpnCustomPromptTextInput {}
DEFINE_CLSID!(VpnCustomPromptTextInput(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,80,114,111,109,112,116,84,101,120,116,73,110,112,117,116,0]) [CLSID_VpnCustomPromptTextInput]);
DEFINE_IID!(IID_IVpnCustomTextBox, 3668231114, 36643, 19766, 145, 241, 118, 217, 55, 130, 121, 66);
RT_INTERFACE!{interface IVpnCustomTextBox(IVpnCustomTextBoxVtbl): IInspectable(IInspectableVtbl) [IID_IVpnCustomTextBox] {
    fn put_DisplayText(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT
}}
impl IVpnCustomTextBox {
    #[inline] pub fn set_display_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DisplayText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnCustomTextBox: IVpnCustomTextBox}
impl RtActivatable<IActivationFactory> for VpnCustomTextBox {}
DEFINE_CLSID!(VpnCustomTextBox(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,67,117,115,116,111,109,84,101,120,116,66,111,120,0]) [CLSID_VpnCustomTextBox]);
RT_ENUM! { enum VpnDataPathType: i32 {
    Send = 0, Receive = 1,
}}
DEFINE_IID!(IID_IVpnDomainNameAssignment, 1094037825, 52443, 18869, 148, 1, 3, 154, 138, 231, 103, 233);
RT_INTERFACE!{interface IVpnDomainNameAssignment(IVpnDomainNameAssignmentVtbl): IInspectable(IInspectableVtbl) [IID_IVpnDomainNameAssignment] {
    fn get_DomainNameList(&self, out: *mut *mut foundation::collections::IVector<VpnDomainNameInfo>) -> HRESULT,
    fn put_ProxyAutoConfigurationUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_ProxyAutoConfigurationUri(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl IVpnDomainNameAssignment {
    #[inline] pub fn get_domain_name_list(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnDomainNameInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DomainNameList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_proxy_auto_configuration_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProxyAutoConfigurationUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_proxy_auto_configuration_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyAutoConfigurationUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnDomainNameAssignment: IVpnDomainNameAssignment}
impl RtActivatable<IActivationFactory> for VpnDomainNameAssignment {}
DEFINE_CLSID!(VpnDomainNameAssignment(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,68,111,109,97,105,110,78,97,109,101,65,115,115,105,103,110,109,101,110,116,0]) [CLSID_VpnDomainNameAssignment]);
DEFINE_IID!(IID_IVpnDomainNameInfo, 2905520175, 60046, 20346, 132, 62, 26, 135, 227, 46, 27, 154);
RT_INTERFACE!{interface IVpnDomainNameInfo(IVpnDomainNameInfoVtbl): IInspectable(IInspectableVtbl) [IID_IVpnDomainNameInfo] {
    fn put_DomainName(&self, value: *mut super::HostName) -> HRESULT,
    fn get_DomainName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn put_DomainNameType(&self, value: VpnDomainNameType) -> HRESULT,
    fn get_DomainNameType(&self, out: *mut VpnDomainNameType) -> HRESULT,
    fn get_DnsServers(&self, out: *mut *mut foundation::collections::IVector<super::HostName>) -> HRESULT,
    fn get_WebProxyServers(&self, out: *mut *mut foundation::collections::IVector<super::HostName>) -> HRESULT
}}
impl IVpnDomainNameInfo {
    #[inline] pub fn set_domain_name(&self, value: &super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DomainName)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DomainName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_domain_name_type(&self, value: VpnDomainNameType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DomainNameType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_name_type(&self) -> Result<VpnDomainNameType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DomainNameType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dns_servers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DnsServers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_web_proxy_servers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebProxyServers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnDomainNameInfo: IVpnDomainNameInfo}
impl RtActivatable<IVpnDomainNameInfoFactory> for VpnDomainNameInfo {}
impl VpnDomainNameInfo {
    #[inline] pub fn create_vpn_domain_name_info(name: &HStringArg, nameType: VpnDomainNameType, dnsServerList: &foundation::collections::IIterable<super::HostName>, proxyServerList: &foundation::collections::IIterable<super::HostName>) -> Result<ComPtr<VpnDomainNameInfo>> {
        <Self as RtActivatable<IVpnDomainNameInfoFactory>>::get_activation_factory().create_vpn_domain_name_info(name, nameType, dnsServerList, proxyServerList)
    }
}
DEFINE_CLSID!(VpnDomainNameInfo(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,68,111,109,97,105,110,78,97,109,101,73,110,102,111,0]) [CLSID_VpnDomainNameInfo]);
DEFINE_IID!(IID_IVpnDomainNameInfo2, 2877755729, 27731, 18472, 152, 131, 216, 134, 222, 16, 68, 7);
RT_INTERFACE!{interface IVpnDomainNameInfo2(IVpnDomainNameInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnDomainNameInfo2] {
    fn get_WebProxyUris(&self, out: *mut *mut foundation::collections::IVector<foundation::Uri>) -> HRESULT
}}
impl IVpnDomainNameInfo2 {
    #[inline] pub fn get_web_proxy_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVector<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebProxyUris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnDomainNameInfoFactory, 621263733, 655, 18056, 141, 58, 196, 83, 29, 243, 125, 168);
RT_INTERFACE!{static interface IVpnDomainNameInfoFactory(IVpnDomainNameInfoFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnDomainNameInfoFactory] {
    fn CreateVpnDomainNameInfo(&self, name: HSTRING, nameType: VpnDomainNameType, dnsServerList: *mut foundation::collections::IIterable<super::HostName>, proxyServerList: *mut foundation::collections::IIterable<super::HostName>, out: *mut *mut VpnDomainNameInfo) -> HRESULT
}}
impl IVpnDomainNameInfoFactory {
    #[inline] pub fn create_vpn_domain_name_info(&self, name: &HStringArg, nameType: VpnDomainNameType, dnsServerList: &foundation::collections::IIterable<super::HostName>, proxyServerList: &foundation::collections::IIterable<super::HostName>) -> Result<ComPtr<VpnDomainNameInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateVpnDomainNameInfo)(self as *const _ as *mut _, name.get(), nameType, dnsServerList as *const _ as *mut _, proxyServerList as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnDomainNameType: i32 {
    Suffix = 0, FullyQualified = 1, Reserved = 65535,
}}
DEFINE_IID!(IID_IVpnInterfaceId, 2653805730, 5906, 19684, 177, 121, 140, 101, 44, 109, 16, 17);
RT_INTERFACE!{interface IVpnInterfaceId(IVpnInterfaceIdVtbl): IInspectable(IInspectableVtbl) [IID_IVpnInterfaceId] {
    fn GetAddressInfo(&self, idSize: *mut u32, id: *mut *mut u8) -> HRESULT
}}
impl IVpnInterfaceId {
    #[inline] pub fn get_address_info(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut idSize = 0; let mut id = null_mut();
        let hr = ((*self.lpVtbl).GetAddressInfo)(self as *const _ as *mut _, &mut idSize, &mut id);
        if hr == S_OK { Ok(ComArray::from_raw(idSize, id)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnInterfaceId: IVpnInterfaceId}
impl RtActivatable<IVpnInterfaceIdFactory> for VpnInterfaceId {}
impl VpnInterfaceId {
    #[inline] pub fn create_vpn_interface_id(address: &[u8]) -> Result<ComPtr<VpnInterfaceId>> {
        <Self as RtActivatable<IVpnInterfaceIdFactory>>::get_activation_factory().create_vpn_interface_id(address)
    }
}
DEFINE_CLSID!(VpnInterfaceId(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,73,110,116,101,114,102,97,99,101,73,100,0]) [CLSID_VpnInterfaceId]);
DEFINE_IID!(IID_IVpnInterfaceIdFactory, 2653805730, 5906, 19684, 177, 121, 140, 101, 44, 109, 16, 0);
RT_INTERFACE!{static interface IVpnInterfaceIdFactory(IVpnInterfaceIdFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnInterfaceIdFactory] {
    fn CreateVpnInterfaceId(&self, addressSize: u32, address: *mut u8, out: *mut *mut VpnInterfaceId) -> HRESULT
}}
impl IVpnInterfaceIdFactory {
    #[inline] pub fn create_vpn_interface_id(&self, address: &[u8]) -> Result<ComPtr<VpnInterfaceId>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateVpnInterfaceId)(self as *const _ as *mut _, address.len() as u32, address.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnIPProtocol: i32 {
    None = 0, Tcp = 6, Udp = 17, Icmp = 1, Ipv6Icmp = 58, Igmp = 2, Pgm = 113,
}}
DEFINE_IID!(IID_IVpnManagementAgent, 423007949, 42436, 19134, 133, 43, 120, 91, 228, 203, 62, 52);
RT_INTERFACE!{interface IVpnManagementAgent(IVpnManagementAgentVtbl): IInspectable(IInspectableVtbl) [IID_IVpnManagementAgent] {
    fn AddProfileFromXmlAsync(&self, xml: HSTRING, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn AddProfileFromObjectAsync(&self, profile: *mut IVpnProfile, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn UpdateProfileFromXmlAsync(&self, xml: HSTRING, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn UpdateProfileFromObjectAsync(&self, profile: *mut IVpnProfile, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn GetProfilesAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<IVpnProfile>>) -> HRESULT,
    fn DeleteProfileAsync(&self, profile: *mut IVpnProfile, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn ConnectProfileAsync(&self, profile: *mut IVpnProfile, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    #[cfg(not(feature="windows-security"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-security")] fn ConnectProfileWithPasswordCredentialAsync(&self, profile: *mut IVpnProfile, passwordCredential: *mut super::super::security::credentials::PasswordCredential, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT,
    fn DisconnectProfileAsync(&self, profile: *mut IVpnProfile, out: *mut *mut foundation::IAsyncOperation<VpnManagementErrorStatus>) -> HRESULT
}}
impl IVpnManagementAgent {
    #[inline] pub fn add_profile_from_xml_async(&self, xml: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddProfileFromXmlAsync)(self as *const _ as *mut _, xml.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_profile_from_object_async(&self, profile: &IVpnProfile) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AddProfileFromObjectAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_profile_from_xml_async(&self, xml: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateProfileFromXmlAsync)(self as *const _ as *mut _, xml.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_profile_from_object_async(&self, profile: &IVpnProfile) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UpdateProfileFromObjectAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_profiles_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<IVpnProfile>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProfilesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_profile_async(&self, profile: &IVpnProfile) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteProfileAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn connect_profile_async(&self, profile: &IVpnProfile) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectProfileAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn connect_profile_with_password_credential_async(&self, profile: &IVpnProfile, passwordCredential: &super::super::security::credentials::PasswordCredential) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConnectProfileWithPasswordCredentialAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, passwordCredential as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn disconnect_profile_async(&self, profile: &IVpnProfile) -> Result<ComPtr<foundation::IAsyncOperation<VpnManagementErrorStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DisconnectProfileAsync)(self as *const _ as *mut _, profile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnManagementAgent: IVpnManagementAgent}
impl RtActivatable<IActivationFactory> for VpnManagementAgent {}
DEFINE_CLSID!(VpnManagementAgent(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,77,97,110,97,103,101,109,101,110,116,65,103,101,110,116,0]) [CLSID_VpnManagementAgent]);
RT_ENUM! { enum VpnManagementConnectionStatus: i32 {
    Disconnected = 0, Disconnecting = 1, Connected = 2, Connecting = 3,
}}
RT_ENUM! { enum VpnManagementErrorStatus: i32 {
    Ok = 0, Other = 1, InvalidXmlSyntax = 2, ProfileNameTooLong = 3, ProfileInvalidAppId = 4, AccessDenied = 5, CannotFindProfile = 6, AlreadyDisconnecting = 7, AlreadyConnected = 8, GeneralAuthenticationFailure = 9, EapFailure = 10, SmartCardFailure = 11, CertificateFailure = 12, ServerConfiguration = 13, NoConnection = 14, ServerConnection = 15, UserNamePassword = 16, DnsNotResolvable = 17, InvalidIP = 18,
}}
DEFINE_IID!(IID_IVpnNamespaceAssignment, 3623344920, 12413, 19470, 189, 98, 143, 162, 112, 187, 173, 214);
RT_INTERFACE!{interface IVpnNamespaceAssignment(IVpnNamespaceAssignmentVtbl): IInspectable(IInspectableVtbl) [IID_IVpnNamespaceAssignment] {
    fn put_NamespaceList(&self, value: *mut foundation::collections::IVector<VpnNamespaceInfo>) -> HRESULT,
    fn get_NamespaceList(&self, out: *mut *mut foundation::collections::IVector<VpnNamespaceInfo>) -> HRESULT,
    fn put_ProxyAutoConfigUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_ProxyAutoConfigUri(&self, out: *mut *mut foundation::Uri) -> HRESULT
}}
impl IVpnNamespaceAssignment {
    #[inline] pub fn set_namespace_list(&self, value: &foundation::collections::IVector<VpnNamespaceInfo>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NamespaceList)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_namespace_list(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnNamespaceInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NamespaceList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_proxy_auto_config_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProxyAutoConfigUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_proxy_auto_config_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProxyAutoConfigUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnNamespaceAssignment: IVpnNamespaceAssignment}
impl RtActivatable<IActivationFactory> for VpnNamespaceAssignment {}
DEFINE_CLSID!(VpnNamespaceAssignment(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,78,97,109,101,115,112,97,99,101,65,115,115,105,103,110,109,101,110,116,0]) [CLSID_VpnNamespaceAssignment]);
DEFINE_IID!(IID_IVpnNamespaceInfo, 820902723, 17487, 17605, 129, 103, 163, 90, 145, 241, 175, 148);
RT_INTERFACE!{interface IVpnNamespaceInfo(IVpnNamespaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IVpnNamespaceInfo] {
    fn put_Namespace(&self, value: HSTRING) -> HRESULT,
    fn get_Namespace(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DnsServers(&self, value: *mut foundation::collections::IVector<super::HostName>) -> HRESULT,
    fn get_DnsServers(&self, out: *mut *mut foundation::collections::IVector<super::HostName>) -> HRESULT,
    fn put_WebProxyServers(&self, value: *mut foundation::collections::IVector<super::HostName>) -> HRESULT,
    fn get_WebProxyServers(&self, out: *mut *mut foundation::collections::IVector<super::HostName>) -> HRESULT
}}
impl IVpnNamespaceInfo {
    #[inline] pub fn set_namespace(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Namespace)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Namespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_dns_servers(&self, value: &foundation::collections::IVector<super::HostName>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DnsServers)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dns_servers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DnsServers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_web_proxy_servers(&self, value: &foundation::collections::IVector<super::HostName>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WebProxyServers)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_web_proxy_servers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebProxyServers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnNamespaceInfo: IVpnNamespaceInfo}
impl RtActivatable<IVpnNamespaceInfoFactory> for VpnNamespaceInfo {}
impl VpnNamespaceInfo {
    #[inline] pub fn create_vpn_namespace_info(name: &HStringArg, dnsServerList: &foundation::collections::IVector<super::HostName>, proxyServerList: &foundation::collections::IVector<super::HostName>) -> Result<ComPtr<VpnNamespaceInfo>> {
        <Self as RtActivatable<IVpnNamespaceInfoFactory>>::get_activation_factory().create_vpn_namespace_info(name, dnsServerList, proxyServerList)
    }
}
DEFINE_CLSID!(VpnNamespaceInfo(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,78,97,109,101,115,112,97,99,101,73,110,102,111,0]) [CLSID_VpnNamespaceInfo]);
DEFINE_IID!(IID_IVpnNamespaceInfoFactory, 3409876250, 45262, 17451, 172, 187, 95, 153, 178, 2, 195, 28);
RT_INTERFACE!{static interface IVpnNamespaceInfoFactory(IVpnNamespaceInfoFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnNamespaceInfoFactory] {
    fn CreateVpnNamespaceInfo(&self, name: HSTRING, dnsServerList: *mut foundation::collections::IVector<super::HostName>, proxyServerList: *mut foundation::collections::IVector<super::HostName>, out: *mut *mut VpnNamespaceInfo) -> HRESULT
}}
impl IVpnNamespaceInfoFactory {
    #[inline] pub fn create_vpn_namespace_info(&self, name: &HStringArg, dnsServerList: &foundation::collections::IVector<super::HostName>, proxyServerList: &foundation::collections::IVector<super::HostName>) -> Result<ComPtr<VpnNamespaceInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateVpnNamespaceInfo)(self as *const _ as *mut _, name.get(), dnsServerList as *const _ as *mut _, proxyServerList as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnNativeProfile, 2762924702, 25623, 17203, 152, 66, 240, 166, 109, 182, 152, 2);
RT_INTERFACE!{interface IVpnNativeProfile(IVpnNativeProfileVtbl): IInspectable(IInspectableVtbl) [IID_IVpnNativeProfile] {
    fn get_Servers(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_RoutingPolicyType(&self, out: *mut VpnRoutingPolicyType) -> HRESULT,
    fn put_RoutingPolicyType(&self, value: VpnRoutingPolicyType) -> HRESULT,
    fn get_NativeProtocolType(&self, out: *mut VpnNativeProtocolType) -> HRESULT,
    fn put_NativeProtocolType(&self, value: VpnNativeProtocolType) -> HRESULT,
    fn get_UserAuthenticationMethod(&self, out: *mut VpnAuthenticationMethod) -> HRESULT,
    fn put_UserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> HRESULT,
    fn get_TunnelAuthenticationMethod(&self, out: *mut VpnAuthenticationMethod) -> HRESULT,
    fn put_TunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> HRESULT,
    fn get_EapConfiguration(&self, out: *mut HSTRING) -> HRESULT,
    fn put_EapConfiguration(&self, value: HSTRING) -> HRESULT
}}
impl IVpnNativeProfile {
    #[inline] pub fn get_servers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Servers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_routing_policy_type(&self) -> Result<VpnRoutingPolicyType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoutingPolicyType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_routing_policy_type(&self, value: VpnRoutingPolicyType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RoutingPolicyType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_native_protocol_type(&self) -> Result<VpnNativeProtocolType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NativeProtocolType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_native_protocol_type(&self, value: VpnNativeProtocolType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NativeProtocolType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_user_authentication_method(&self) -> Result<VpnAuthenticationMethod> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserAuthenticationMethod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_user_authentication_method(&self, value: VpnAuthenticationMethod) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_UserAuthenticationMethod)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_tunnel_authentication_method(&self) -> Result<VpnAuthenticationMethod> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TunnelAuthenticationMethod)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_tunnel_authentication_method(&self, value: VpnAuthenticationMethod) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TunnelAuthenticationMethod)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_eap_configuration(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EapConfiguration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_eap_configuration(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_EapConfiguration)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnNativeProfile: IVpnNativeProfile}
impl RtActivatable<IActivationFactory> for VpnNativeProfile {}
DEFINE_CLSID!(VpnNativeProfile(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,78,97,116,105,118,101,80,114,111,102,105,108,101,0]) [CLSID_VpnNativeProfile]);
DEFINE_IID!(IID_IVpnNativeProfile2, 267134055, 52661, 19143, 181, 163, 10, 251, 94, 196, 118, 130);
RT_INTERFACE!{interface IVpnNativeProfile2(IVpnNativeProfile2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnNativeProfile2] {
    fn get_RequireVpnClientAppUI(&self, out: *mut bool) -> HRESULT,
    fn put_RequireVpnClientAppUI(&self, value: bool) -> HRESULT,
    fn get_ConnectionStatus(&self, out: *mut VpnManagementConnectionStatus) -> HRESULT
}}
impl IVpnNativeProfile2 {
    #[inline] pub fn get_require_vpn_client_app_ui(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RequireVpnClientAppUI)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_require_vpn_client_app_ui(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RequireVpnClientAppUI)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_status(&self) -> Result<VpnManagementConnectionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ConnectionStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnNativeProtocolType: i32 {
    Pptp = 0, L2tp = 1, IpsecIkev2 = 2,
}}
DEFINE_IID!(IID_IVpnPacketBuffer, 3271070204, 19804, 19043, 183, 13, 78, 48, 126, 172, 206, 85);
RT_INTERFACE!{interface IVpnPacketBuffer(IVpnPacketBufferVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBuffer] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Buffer(&self, out: *mut *mut super::super::storage::streams::Buffer) -> HRESULT,
    fn put_Status(&self, value: VpnPacketBufferStatus) -> HRESULT,
    fn get_Status(&self, out: *mut VpnPacketBufferStatus) -> HRESULT,
    fn put_TransportAffinity(&self, value: u32) -> HRESULT,
    fn get_TransportAffinity(&self, out: *mut u32) -> HRESULT
}}
impl IVpnPacketBuffer {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_buffer(&self) -> Result<Option<ComPtr<super::super::storage::streams::Buffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Buffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_status(&self, value: VpnPacketBufferStatus) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Status)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<VpnPacketBufferStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_transport_affinity(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TransportAffinity)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_transport_affinity(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TransportAffinity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnPacketBuffer: IVpnPacketBuffer}
impl RtActivatable<IVpnPacketBufferFactory> for VpnPacketBuffer {}
impl VpnPacketBuffer {
    #[inline] pub fn create_vpn_packet_buffer(parentBuffer: &VpnPacketBuffer, offset: u32, length: u32) -> Result<ComPtr<VpnPacketBuffer>> {
        <Self as RtActivatable<IVpnPacketBufferFactory>>::get_activation_factory().create_vpn_packet_buffer(parentBuffer, offset, length)
    }
}
DEFINE_CLSID!(VpnPacketBuffer(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,80,97,99,107,101,116,66,117,102,102,101,114,0]) [CLSID_VpnPacketBuffer]);
DEFINE_IID!(IID_IVpnPacketBuffer2, 1717473776, 34821, 19445, 166, 25, 46, 132, 136, 46, 107, 79);
RT_INTERFACE!{interface IVpnPacketBuffer2(IVpnPacketBuffer2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBuffer2] {
    fn get_AppId(&self, out: *mut *mut VpnAppId) -> HRESULT
}}
impl IVpnPacketBuffer2 {
    #[inline] pub fn get_app_id(&self) -> Result<Option<ComPtr<VpnAppId>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnPacketBuffer3, 3797288751, 4219, 19520, 177, 39, 91, 197, 62, 10, 217, 96);
RT_INTERFACE!{interface IVpnPacketBuffer3(IVpnPacketBuffer3Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBuffer3] {
    fn put_TransportContext(&self, value: *mut IInspectable) -> HRESULT,
    fn get_TransportContext(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IVpnPacketBuffer3 {
    #[inline] pub fn set_transport_context(&self, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TransportContext)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_transport_context(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TransportContext)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnPacketBufferFactory, 2653805730, 5906, 19684, 177, 121, 140, 101, 44, 109, 153, 153);
RT_INTERFACE!{static interface IVpnPacketBufferFactory(IVpnPacketBufferFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBufferFactory] {
    fn CreateVpnPacketBuffer(&self, parentBuffer: *mut VpnPacketBuffer, offset: u32, length: u32, out: *mut *mut VpnPacketBuffer) -> HRESULT
}}
impl IVpnPacketBufferFactory {
    #[inline] pub fn create_vpn_packet_buffer(&self, parentBuffer: &VpnPacketBuffer, offset: u32, length: u32) -> Result<ComPtr<VpnPacketBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateVpnPacketBuffer)(self as *const _ as *mut _, parentBuffer as *const _ as *mut _, offset, length, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnPacketBufferList, 3271070204, 19804, 19043, 183, 13, 78, 48, 126, 172, 206, 119);
RT_INTERFACE!{interface IVpnPacketBufferList(IVpnPacketBufferListVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBufferList] {
    fn Append(&self, nextVpnPacketBuffer: *mut VpnPacketBuffer) -> HRESULT,
    fn AddAtBegin(&self, nextVpnPacketBuffer: *mut VpnPacketBuffer) -> HRESULT,
    fn RemoveAtEnd(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn RemoveAtBegin(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn Clear(&self) -> HRESULT,
    fn put_Status(&self, value: VpnPacketBufferStatus) -> HRESULT,
    fn get_Status(&self, out: *mut VpnPacketBufferStatus) -> HRESULT,
    fn get_Size(&self, out: *mut u32) -> HRESULT
}}
impl IVpnPacketBufferList {
    #[inline] pub fn append(&self, nextVpnPacketBuffer: &VpnPacketBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Append)(self as *const _ as *mut _, nextVpnPacketBuffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_at_begin(&self, nextVpnPacketBuffer: &VpnPacketBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddAtBegin)(self as *const _ as *mut _, nextVpnPacketBuffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_at_end(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveAtEnd)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_at_begin(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveAtBegin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Clear)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_status(&self, value: VpnPacketBufferStatus) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Status)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<VpnPacketBufferStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnPacketBufferList: IVpnPacketBufferList}
DEFINE_IID!(IID_IVpnPacketBufferList2, 1048236005, 59934, 18474, 141, 152, 192, 101, 245, 125, 137, 234);
RT_INTERFACE!{interface IVpnPacketBufferList2(IVpnPacketBufferList2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnPacketBufferList2] {
    fn AddLeadingPacket(&self, nextVpnPacketBuffer: *mut VpnPacketBuffer) -> HRESULT,
    fn RemoveLeadingPacket(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn AddTrailingPacket(&self, nextVpnPacketBuffer: *mut VpnPacketBuffer) -> HRESULT,
    fn RemoveTrailingPacket(&self, out: *mut *mut VpnPacketBuffer) -> HRESULT
}}
impl IVpnPacketBufferList2 {
    #[inline] pub fn add_leading_packet(&self, nextVpnPacketBuffer: &VpnPacketBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddLeadingPacket)(self as *const _ as *mut _, nextVpnPacketBuffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_leading_packet(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveLeadingPacket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_trailing_packet(&self, nextVpnPacketBuffer: &VpnPacketBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddTrailingPacket)(self as *const _ as *mut _, nextVpnPacketBuffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_trailing_packet(&self) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveTrailingPacket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnPacketBufferStatus: i32 {
    Ok = 0, InvalidBufferSize = 1,
}}
DEFINE_IID!(IID_IVpnPickedCredential, 2591636167, 34900, 20050, 173, 151, 36, 221, 154, 132, 43, 206);
RT_INTERFACE!{interface IVpnPickedCredential(IVpnPickedCredentialVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPickedCredential] {
    #[cfg(feature="windows-security")] fn get_PasskeyCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT,
    fn get_AdditionalPin(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-security")] fn get_OldPasswordCredential(&self, out: *mut *mut super::super::security::credentials::PasswordCredential) -> HRESULT
}}
impl IVpnPickedCredential {
    #[cfg(feature="windows-security")] #[inline] pub fn get_passkey_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PasskeyCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_additional_pin(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdditionalPin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-security")] #[inline] pub fn get_old_password_credential(&self) -> Result<Option<ComPtr<super::super::security::credentials::PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OldPasswordCredential)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnPickedCredential: IVpnPickedCredential}
DEFINE_IID!(IID_IVpnPlugIn, 3468135687, 53416, 18179, 160, 145, 200, 194, 192, 145, 91, 196);
RT_INTERFACE!{interface IVpnPlugIn(IVpnPlugInVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPlugIn] {
    fn Connect(&self, channel: *mut VpnChannel) -> HRESULT,
    fn Disconnect(&self, channel: *mut VpnChannel) -> HRESULT,
    fn GetKeepAlivePayload(&self, channel: *mut VpnChannel, keepAlivePacket: *mut *mut VpnPacketBuffer) -> HRESULT,
    fn Encapsulate(&self, channel: *mut VpnChannel, packets: *mut VpnPacketBufferList, encapulatedPackets: *mut VpnPacketBufferList) -> HRESULT,
    fn Decapsulate(&self, channel: *mut VpnChannel, encapBuffer: *mut VpnPacketBuffer, decapsulatedPackets: *mut VpnPacketBufferList, controlPacketsToSend: *mut VpnPacketBufferList) -> HRESULT
}}
impl IVpnPlugIn {
    #[inline] pub fn connect(&self, channel: &VpnChannel) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Connect)(self as *const _ as *mut _, channel as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn disconnect(&self, channel: &VpnChannel) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Disconnect)(self as *const _ as *mut _, channel as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keep_alive_payload(&self, channel: &VpnChannel) -> Result<Option<ComPtr<VpnPacketBuffer>>> { unsafe { 
        let mut keepAlivePacket = null_mut();
        let hr = ((*self.lpVtbl).GetKeepAlivePayload)(self as *const _ as *mut _, channel as *const _ as *mut _, &mut keepAlivePacket);
        if hr == S_OK { Ok(ComPtr::wrap_optional(keepAlivePacket)) } else { err(hr) }
    }}
    #[inline] pub fn encapsulate(&self, channel: &VpnChannel, packets: &VpnPacketBufferList, encapulatedPackets: &VpnPacketBufferList) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Encapsulate)(self as *const _ as *mut _, channel as *const _ as *mut _, packets as *const _ as *mut _, encapulatedPackets as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn decapsulate(&self, channel: &VpnChannel, encapBuffer: &VpnPacketBuffer, decapsulatedPackets: &VpnPacketBufferList, controlPacketsToSend: &VpnPacketBufferList) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Decapsulate)(self as *const _ as *mut _, channel as *const _ as *mut _, encapBuffer as *const _ as *mut _, decapsulatedPackets as *const _ as *mut _, controlPacketsToSend as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnPlugInProfile, 249499044, 20224, 17801, 141, 123, 75, 249, 136, 246, 84, 44);
RT_INTERFACE!{interface IVpnPlugInProfile(IVpnPlugInProfileVtbl): IInspectable(IInspectableVtbl) [IID_IVpnPlugInProfile] {
    fn get_ServerUris(&self, out: *mut *mut foundation::collections::IVector<foundation::Uri>) -> HRESULT,
    fn get_CustomConfiguration(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CustomConfiguration(&self, value: HSTRING) -> HRESULT,
    fn get_VpnPluginPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_VpnPluginPackageFamilyName(&self, value: HSTRING) -> HRESULT
}}
impl IVpnPlugInProfile {
    #[inline] pub fn get_server_uris(&self) -> Result<Option<ComPtr<foundation::collections::IVector<foundation::Uri>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ServerUris)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_configuration(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomConfiguration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_custom_configuration(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_CustomConfiguration)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vpn_plugin_package_family_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VpnPluginPackageFamilyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_vpn_plugin_package_family_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_VpnPluginPackageFamilyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnPlugInProfile: IVpnPlugInProfile}
impl RtActivatable<IActivationFactory> for VpnPlugInProfile {}
DEFINE_CLSID!(VpnPlugInProfile(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,80,108,117,103,73,110,80,114,111,102,105,108,101,0]) [CLSID_VpnPlugInProfile]);
DEFINE_IID!(IID_IVpnPlugInProfile2, 1629243538, 53140, 19158, 186, 153, 0, 244, 255, 52, 86, 94);
RT_INTERFACE!{interface IVpnPlugInProfile2(IVpnPlugInProfile2Vtbl): IInspectable(IInspectableVtbl) [IID_IVpnPlugInProfile2] {
    fn get_RequireVpnClientAppUI(&self, out: *mut bool) -> HRESULT,
    fn put_RequireVpnClientAppUI(&self, value: bool) -> HRESULT,
    fn get_ConnectionStatus(&self, out: *mut VpnManagementConnectionStatus) -> HRESULT
}}
impl IVpnPlugInProfile2 {
    #[inline] pub fn get_require_vpn_client_app_ui(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RequireVpnClientAppUI)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_require_vpn_client_app_ui(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RequireVpnClientAppUI)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_connection_status(&self) -> Result<VpnManagementConnectionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ConnectionStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnProfile, 2020980561, 45271, 17371, 138, 147, 211, 254, 36, 121, 229, 106);
RT_INTERFACE!{interface IVpnProfile(IVpnProfileVtbl): IInspectable(IInspectableVtbl) [IID_IVpnProfile] {
    fn get_ProfileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ProfileName(&self, value: HSTRING) -> HRESULT,
    fn get_AppTriggers(&self, out: *mut *mut foundation::collections::IVector<VpnAppId>) -> HRESULT,
    fn get_Routes(&self, out: *mut *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn get_DomainNameInfoList(&self, out: *mut *mut foundation::collections::IVector<VpnDomainNameInfo>) -> HRESULT,
    fn get_TrafficFilters(&self, out: *mut *mut foundation::collections::IVector<VpnTrafficFilter>) -> HRESULT,
    fn get_RememberCredentials(&self, out: *mut bool) -> HRESULT,
    fn put_RememberCredentials(&self, value: bool) -> HRESULT,
    fn get_AlwaysOn(&self, out: *mut bool) -> HRESULT,
    fn put_AlwaysOn(&self, value: bool) -> HRESULT
}}
impl IVpnProfile {
    #[inline] pub fn get_profile_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProfileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_profile_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ProfileName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_app_triggers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnAppId>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppTriggers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Routes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_name_info_list(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnDomainNameInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DomainNameInfoList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_traffic_filters(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnTrafficFilter>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrafficFilters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remember_credentials(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RememberCredentials)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_remember_credentials(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RememberCredentials)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_always_on(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AlwaysOn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_always_on(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AlwaysOn)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IVpnRoute, 3044219779, 2409, 18073, 147, 142, 119, 118, 219, 41, 207, 179);
RT_INTERFACE!{interface IVpnRoute(IVpnRouteVtbl): IInspectable(IInspectableVtbl) [IID_IVpnRoute] {
    fn put_Address(&self, value: *mut super::HostName) -> HRESULT,
    fn get_Address(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn put_PrefixSize(&self, value: u8) -> HRESULT,
    fn get_PrefixSize(&self, out: *mut u8) -> HRESULT
}}
impl IVpnRoute {
    #[inline] pub fn set_address(&self, value: &super::HostName) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Address)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_address(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Address)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_prefix_size(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PrefixSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_prefix_size(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrefixSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnRoute: IVpnRoute}
impl RtActivatable<IVpnRouteFactory> for VpnRoute {}
impl VpnRoute {
    #[inline] pub fn create_vpn_route(address: &super::HostName, prefixSize: u8) -> Result<ComPtr<VpnRoute>> {
        <Self as RtActivatable<IVpnRouteFactory>>::get_activation_factory().create_vpn_route(address, prefixSize)
    }
}
DEFINE_CLSID!(VpnRoute(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,82,111,117,116,101,0]) [CLSID_VpnRoute]);
DEFINE_IID!(IID_IVpnRouteAssignment, 3680820770, 52793, 19062, 149, 80, 246, 16, 57, 248, 14, 72);
RT_INTERFACE!{interface IVpnRouteAssignment(IVpnRouteAssignmentVtbl): IInspectable(IInspectableVtbl) [IID_IVpnRouteAssignment] {
    fn put_Ipv4InclusionRoutes(&self, value: *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn put_Ipv6InclusionRoutes(&self, value: *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn get_Ipv4InclusionRoutes(&self, out: *mut *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn get_Ipv6InclusionRoutes(&self, out: *mut *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn put_Ipv4ExclusionRoutes(&self, value: *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn put_Ipv6ExclusionRoutes(&self, value: *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn get_Ipv4ExclusionRoutes(&self, out: *mut *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn get_Ipv6ExclusionRoutes(&self, out: *mut *mut foundation::collections::IVector<VpnRoute>) -> HRESULT,
    fn put_ExcludeLocalSubnets(&self, value: bool) -> HRESULT,
    fn get_ExcludeLocalSubnets(&self, out: *mut bool) -> HRESULT
}}
impl IVpnRouteAssignment {
    #[inline] pub fn set_ipv4_inclusion_routes(&self, value: &foundation::collections::IVector<VpnRoute>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Ipv4InclusionRoutes)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_ipv6_inclusion_routes(&self, value: &foundation::collections::IVector<VpnRoute>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Ipv6InclusionRoutes)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_ipv4_inclusion_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ipv4InclusionRoutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ipv6_inclusion_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ipv6InclusionRoutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_ipv4_exclusion_routes(&self, value: &foundation::collections::IVector<VpnRoute>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Ipv4ExclusionRoutes)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_ipv6_exclusion_routes(&self, value: &foundation::collections::IVector<VpnRoute>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Ipv6ExclusionRoutes)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_ipv4_exclusion_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ipv4ExclusionRoutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ipv6_exclusion_routes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnRoute>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Ipv6ExclusionRoutes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_exclude_local_subnets(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ExcludeLocalSubnets)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_exclude_local_subnets(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExcludeLocalSubnets)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnRouteAssignment: IVpnRouteAssignment}
impl RtActivatable<IActivationFactory> for VpnRouteAssignment {}
DEFINE_CLSID!(VpnRouteAssignment(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,82,111,117,116,101,65,115,115,105,103,110,109,101,110,116,0]) [CLSID_VpnRouteAssignment]);
DEFINE_IID!(IID_IVpnRouteFactory, 3186275839, 17871, 19353, 131, 251, 219, 59, 194, 103, 43, 2);
RT_INTERFACE!{static interface IVpnRouteFactory(IVpnRouteFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnRouteFactory] {
    fn CreateVpnRoute(&self, address: *mut super::HostName, prefixSize: u8, out: *mut *mut VpnRoute) -> HRESULT
}}
impl IVpnRouteFactory {
    #[inline] pub fn create_vpn_route(&self, address: &super::HostName, prefixSize: u8) -> Result<ComPtr<VpnRoute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateVpnRoute)(self as *const _ as *mut _, address as *const _ as *mut _, prefixSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum VpnRoutingPolicyType: i32 {
    SplitRouting = 0, ForceAllTrafficOverVpn = 1,
}}
DEFINE_IID!(IID_IVpnSystemHealth, 2577987759, 49390, 20085, 129, 122, 242, 49, 174, 229, 18, 61);
RT_INTERFACE!{interface IVpnSystemHealth(IVpnSystemHealthVtbl): IInspectable(IInspectableVtbl) [IID_IVpnSystemHealth] {
    #[cfg(feature="windows-storage")] fn get_StatementOfHealth(&self, out: *mut *mut super::super::storage::streams::Buffer) -> HRESULT
}}
impl IVpnSystemHealth {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_statement_of_health(&self) -> Result<Option<ComPtr<super::super::storage::streams::Buffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StatementOfHealth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnSystemHealth: IVpnSystemHealth}
DEFINE_IID!(IID_IVpnTrafficFilter, 795417440, 27807, 18421, 172, 54, 187, 27, 4, 46, 44, 80);
RT_INTERFACE!{interface IVpnTrafficFilter(IVpnTrafficFilterVtbl): IInspectable(IInspectableVtbl) [IID_IVpnTrafficFilter] {
    fn get_AppId(&self, out: *mut *mut VpnAppId) -> HRESULT,
    fn put_AppId(&self, value: *mut VpnAppId) -> HRESULT,
    fn get_AppClaims(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Protocol(&self, out: *mut VpnIPProtocol) -> HRESULT,
    fn put_Protocol(&self, value: VpnIPProtocol) -> HRESULT,
    fn get_LocalPortRanges(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_RemotePortRanges(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_LocalAddressRanges(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_RemoteAddressRanges(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_RoutingPolicyType(&self, out: *mut VpnRoutingPolicyType) -> HRESULT,
    fn put_RoutingPolicyType(&self, value: VpnRoutingPolicyType) -> HRESULT
}}
impl IVpnTrafficFilter {
    #[inline] pub fn get_app_id(&self) -> Result<Option<ComPtr<VpnAppId>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_app_id(&self, value: &VpnAppId) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AppId)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_app_claims(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppClaims)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_protocol(&self) -> Result<VpnIPProtocol> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Protocol)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_protocol(&self, value: VpnIPProtocol) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Protocol)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_local_port_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalPortRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_port_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemotePortRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_address_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalAddressRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_address_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteAddressRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_routing_policy_type(&self) -> Result<VpnRoutingPolicyType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RoutingPolicyType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_routing_policy_type(&self, value: VpnRoutingPolicyType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_RoutingPolicyType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnTrafficFilter: IVpnTrafficFilter}
impl RtActivatable<IVpnTrafficFilterFactory> for VpnTrafficFilter {}
impl VpnTrafficFilter {
    #[inline] pub fn create(appId: &VpnAppId) -> Result<ComPtr<VpnTrafficFilter>> {
        <Self as RtActivatable<IVpnTrafficFilterFactory>>::get_activation_factory().create(appId)
    }
}
DEFINE_CLSID!(VpnTrafficFilter(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,84,114,97,102,102,105,99,70,105,108,116,101,114,0]) [CLSID_VpnTrafficFilter]);
DEFINE_IID!(IID_IVpnTrafficFilterAssignment, 1456264284, 58980, 18206, 137, 205, 96, 22, 3, 185, 224, 243);
RT_INTERFACE!{interface IVpnTrafficFilterAssignment(IVpnTrafficFilterAssignmentVtbl): IInspectable(IInspectableVtbl) [IID_IVpnTrafficFilterAssignment] {
    fn get_TrafficFilterList(&self, out: *mut *mut foundation::collections::IVector<VpnTrafficFilter>) -> HRESULT,
    fn get_AllowOutbound(&self, out: *mut bool) -> HRESULT,
    fn put_AllowOutbound(&self, value: bool) -> HRESULT,
    fn get_AllowInbound(&self, out: *mut bool) -> HRESULT,
    fn put_AllowInbound(&self, value: bool) -> HRESULT
}}
impl IVpnTrafficFilterAssignment {
    #[inline] pub fn get_traffic_filter_list(&self) -> Result<Option<ComPtr<foundation::collections::IVector<VpnTrafficFilter>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TrafficFilterList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_outbound(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowOutbound)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_outbound(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowOutbound)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_inbound(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowInbound)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_inbound(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowInbound)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class VpnTrafficFilterAssignment: IVpnTrafficFilterAssignment}
impl RtActivatable<IActivationFactory> for VpnTrafficFilterAssignment {}
DEFINE_CLSID!(VpnTrafficFilterAssignment(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,86,112,110,46,86,112,110,84,114,97,102,102,105,99,70,105,108,116,101,114,65,115,115,105,103,110,109,101,110,116,0]) [CLSID_VpnTrafficFilterAssignment]);
DEFINE_IID!(IID_IVpnTrafficFilterFactory, 1208828373, 32665, 18252, 134, 238, 150, 223, 22, 131, 24, 241);
RT_INTERFACE!{static interface IVpnTrafficFilterFactory(IVpnTrafficFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IVpnTrafficFilterFactory] {
    fn Create(&self, appId: *mut VpnAppId, out: *mut *mut VpnTrafficFilter) -> HRESULT
}}
impl IVpnTrafficFilterFactory {
    #[inline] pub fn create(&self, appId: &VpnAppId) -> Result<ComPtr<VpnTrafficFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, appId as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Networking.Vpn
pub mod xboxlive { // Windows.Networking.XboxLive
use crate::prelude::*;
DEFINE_IID!(IID_IXboxLiveDeviceAddress, 4122727033, 15494, 19287, 163, 26, 185, 70, 36, 8, 253, 1);
RT_INTERFACE!{interface IXboxLiveDeviceAddress(IXboxLiveDeviceAddressVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveDeviceAddress] {
    fn add_SnapshotChanged(&self, handler: *mut foundation::TypedEventHandler<XboxLiveDeviceAddress, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SnapshotChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn GetSnapshotAsBase64(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetSnapshotAsBuffer(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn GetSnapshotAsBytes(&self, bufferSize: u32, buffer: *mut u8, bytesWritten: *mut u32) -> HRESULT,
    fn Compare(&self, otherDeviceAddress: *mut XboxLiveDeviceAddress, out: *mut i32) -> HRESULT,
    fn get_IsValid(&self, out: *mut bool) -> HRESULT,
    fn get_IsLocal(&self, out: *mut bool) -> HRESULT,
    fn get_NetworkAccessKind(&self, out: *mut XboxLiveNetworkAccessKind) -> HRESULT
}}
impl IXboxLiveDeviceAddress {
    #[inline] pub fn add_snapshot_changed(&self, handler: &foundation::TypedEventHandler<XboxLiveDeviceAddress, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_SnapshotChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_snapshot_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_SnapshotChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_snapshot_as_base64(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSnapshotAsBase64)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_snapshot_as_buffer(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSnapshotAsBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_snapshot_as_bytes(&self, buffer: &mut [u8]) -> Result<u32> { unsafe { 
        let mut bytesWritten = zeroed();
        let hr = ((*self.lpVtbl).GetSnapshotAsBytes)(self as *const _ as *mut _, buffer.len() as u32, buffer.as_mut_ptr() as *mut _, &mut bytesWritten);
        if hr == S_OK { Ok(bytesWritten) } else { err(hr) }
    }}
    #[inline] pub fn compare(&self, otherDeviceAddress: &XboxLiveDeviceAddress) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).Compare)(self as *const _ as *mut _, otherDeviceAddress as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_valid(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsValid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_local(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsLocal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_network_access_kind(&self) -> Result<XboxLiveNetworkAccessKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NetworkAccessKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveDeviceAddress: IXboxLiveDeviceAddress}
impl RtActivatable<IXboxLiveDeviceAddressStatics> for XboxLiveDeviceAddress {}
impl XboxLiveDeviceAddress {
    #[inline] pub fn create_from_snapshot_base64(base64: &HStringArg) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> {
        <Self as RtActivatable<IXboxLiveDeviceAddressStatics>>::get_activation_factory().create_from_snapshot_base64(base64)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_snapshot_buffer(buffer: &super::super::storage::streams::IBuffer) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> {
        <Self as RtActivatable<IXboxLiveDeviceAddressStatics>>::get_activation_factory().create_from_snapshot_buffer(buffer)
    }
    #[inline] pub fn create_from_snapshot_bytes(buffer: &[u8]) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> {
        <Self as RtActivatable<IXboxLiveDeviceAddressStatics>>::get_activation_factory().create_from_snapshot_bytes(buffer)
    }
    #[inline] pub fn get_local() -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> {
        <Self as RtActivatable<IXboxLiveDeviceAddressStatics>>::get_activation_factory().get_local()
    }
    #[inline] pub fn get_max_snapshot_bytes_size() -> Result<u32> {
        <Self as RtActivatable<IXboxLiveDeviceAddressStatics>>::get_activation_factory().get_max_snapshot_bytes_size()
    }
}
DEFINE_CLSID!(XboxLiveDeviceAddress(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,88,98,111,120,76,105,118,101,46,88,98,111,120,76,105,118,101,68,101,118,105,99,101,65,100,100,114,101,115,115,0]) [CLSID_XboxLiveDeviceAddress]);
DEFINE_IID!(IID_IXboxLiveDeviceAddressStatics, 1498720281, 19065, 18737, 130, 124, 127, 80, 62, 150, 50, 99);
RT_INTERFACE!{static interface IXboxLiveDeviceAddressStatics(IXboxLiveDeviceAddressStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveDeviceAddressStatics] {
    fn CreateFromSnapshotBase64(&self, base64: HSTRING, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateFromSnapshotBuffer(&self, buffer: *mut super::super::storage::streams::IBuffer, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn CreateFromSnapshotBytes(&self, bufferSize: u32, buffer: *mut u8, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn GetLocal(&self, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn get_MaxSnapshotBytesSize(&self, out: *mut u32) -> HRESULT
}}
impl IXboxLiveDeviceAddressStatics {
    #[inline] pub fn create_from_snapshot_base64(&self, base64: &HStringArg) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromSnapshotBase64)(self as *const _ as *mut _, base64.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_snapshot_buffer(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromSnapshotBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_snapshot_bytes(&self, buffer: &[u8]) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromSnapshotBytes)(self as *const _ as *mut _, buffer.len() as u32, buffer.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local(&self) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLocal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_snapshot_bytes_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxSnapshotBytesSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXboxLiveEndpointPair, 513442715, 33086, 17632, 184, 127, 200, 122, 9, 52, 117, 228);
RT_INTERFACE!{interface IXboxLiveEndpointPair(IXboxLiveEndpointPairVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPair] {
    fn add_StateChanged(&self, handler: *mut foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn DeleteAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn GetRemoteSocketAddressBytes(&self, socketAddressSize: u32, socketAddress: *mut u8) -> HRESULT,
    fn GetLocalSocketAddressBytes(&self, socketAddressSize: u32, socketAddress: *mut u8) -> HRESULT,
    fn get_State(&self, out: *mut XboxLiveEndpointPairState) -> HRESULT,
    fn get_Template(&self, out: *mut *mut XboxLiveEndpointPairTemplate) -> HRESULT,
    fn get_RemoteDeviceAddress(&self, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn get_RemoteHostName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_RemotePort(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalHostName(&self, out: *mut *mut super::HostName) -> HRESULT,
    fn get_LocalPort(&self, out: *mut HSTRING) -> HRESULT
}}
impl IXboxLiveEndpointPair {
    #[inline] pub fn add_state_changed(&self, handler: &foundation::TypedEventHandler<XboxLiveEndpointPair, XboxLiveEndpointPairStateChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_StateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn delete_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).DeleteAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_socket_address_bytes(&self, socketAddress: &mut [u8]) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).GetRemoteSocketAddressBytes)(self as *const _ as *mut _, socketAddress.len() as u32, socketAddress.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_local_socket_address_bytes(&self, socketAddress: &mut [u8]) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).GetLocalSocketAddressBytes)(self as *const _ as *mut _, socketAddress.len() as u32, socketAddress.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<XboxLiveEndpointPairState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_template(&self) -> Result<Option<ComPtr<XboxLiveEndpointPairTemplate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Template)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_device_address(&self) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteDeviceAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_host_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteHostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemotePort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_host_name(&self) -> Result<Option<ComPtr<super::HostName>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalHostName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_port(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalPort)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveEndpointPair: IXboxLiveEndpointPair}
impl RtActivatable<IXboxLiveEndpointPairStatics> for XboxLiveEndpointPair {}
impl XboxLiveEndpointPair {
    #[inline] pub fn find_endpoint_pair_by_socket_address_bytes(localSocketAddress: &[u8], remoteSocketAddress: &[u8]) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> {
        <Self as RtActivatable<IXboxLiveEndpointPairStatics>>::get_activation_factory().find_endpoint_pair_by_socket_address_bytes(localSocketAddress, remoteSocketAddress)
    }
    #[inline] pub fn find_endpoint_pair_by_host_names_and_ports(localHostName: &super::HostName, localPort: &HStringArg, remoteHostName: &super::HostName, remotePort: &HStringArg) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> {
        <Self as RtActivatable<IXboxLiveEndpointPairStatics>>::get_activation_factory().find_endpoint_pair_by_host_names_and_ports(localHostName, localPort, remoteHostName, remotePort)
    }
}
DEFINE_CLSID!(XboxLiveEndpointPair(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,88,98,111,120,76,105,118,101,46,88,98,111,120,76,105,118,101,69,110,100,112,111,105,110,116,80,97,105,114,0]) [CLSID_XboxLiveEndpointPair]);
RT_ENUM! { enum XboxLiveEndpointPairCreationBehaviors: u32 {
    None = 0, ReevaluatePath = 1,
}}
DEFINE_IID!(IID_IXboxLiveEndpointPairCreationResult, 3651713941, 10923, 19742, 151, 148, 51, 236, 192, 220, 240, 254);
RT_INTERFACE!{interface IXboxLiveEndpointPairCreationResult(IXboxLiveEndpointPairCreationResultVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPairCreationResult] {
    fn get_DeviceAddress(&self, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn get_Status(&self, out: *mut XboxLiveEndpointPairCreationStatus) -> HRESULT,
    fn get_IsExistingPathEvaluation(&self, out: *mut bool) -> HRESULT,
    fn get_EndpointPair(&self, out: *mut *mut XboxLiveEndpointPair) -> HRESULT
}}
impl IXboxLiveEndpointPairCreationResult {
    #[inline] pub fn get_device_address(&self) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<XboxLiveEndpointPairCreationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_existing_path_evaluation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsExistingPathEvaluation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_endpoint_pair(&self) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EndpointPair)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveEndpointPairCreationResult: IXboxLiveEndpointPairCreationResult}
RT_ENUM! { enum XboxLiveEndpointPairCreationStatus: i32 {
    Succeeded = 0, NoLocalNetworks = 1, NoCompatibleNetworkPaths = 2, LocalSystemNotAuthorized = 3, Canceled = 4, TimedOut = 5, RemoteSystemNotAuthorized = 6, RefusedDueToConfiguration = 7, UnexpectedInternalError = 8,
}}
RT_ENUM! { enum XboxLiveEndpointPairState: i32 {
    Invalid = 0, CreatingOutbound = 1, CreatingInbound = 2, Ready = 3, DeletingLocally = 4, RemoteEndpointTerminating = 5, Deleted = 6,
}}
DEFINE_IID!(IID_IXboxLiveEndpointPairStateChangedEventArgs, 1496202069, 56840, 17639, 172, 59, 185, 185, 161, 105, 88, 58);
RT_INTERFACE!{interface IXboxLiveEndpointPairStateChangedEventArgs(IXboxLiveEndpointPairStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPairStateChangedEventArgs] {
    fn get_OldState(&self, out: *mut XboxLiveEndpointPairState) -> HRESULT,
    fn get_NewState(&self, out: *mut XboxLiveEndpointPairState) -> HRESULT
}}
impl IXboxLiveEndpointPairStateChangedEventArgs {
    #[inline] pub fn get_old_state(&self) -> Result<XboxLiveEndpointPairState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OldState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_new_state(&self) -> Result<XboxLiveEndpointPairState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NewState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveEndpointPairStateChangedEventArgs: IXboxLiveEndpointPairStateChangedEventArgs}
DEFINE_IID!(IID_IXboxLiveEndpointPairStatics, 1680960304, 8570, 16963, 142, 225, 103, 41, 40, 29, 39, 219);
RT_INTERFACE!{static interface IXboxLiveEndpointPairStatics(IXboxLiveEndpointPairStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPairStatics] {
    fn FindEndpointPairBySocketAddressBytes(&self, localSocketAddressSize: u32, localSocketAddress: *mut u8, remoteSocketAddressSize: u32, remoteSocketAddress: *mut u8, out: *mut *mut XboxLiveEndpointPair) -> HRESULT,
    fn FindEndpointPairByHostNamesAndPorts(&self, localHostName: *mut super::HostName, localPort: HSTRING, remoteHostName: *mut super::HostName, remotePort: HSTRING, out: *mut *mut XboxLiveEndpointPair) -> HRESULT
}}
impl IXboxLiveEndpointPairStatics {
    #[inline] pub fn find_endpoint_pair_by_socket_address_bytes(&self, localSocketAddress: &[u8], remoteSocketAddress: &[u8]) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindEndpointPairBySocketAddressBytes)(self as *const _ as *mut _, localSocketAddress.len() as u32, localSocketAddress.as_ptr() as *mut _, remoteSocketAddress.len() as u32, remoteSocketAddress.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_endpoint_pair_by_host_names_and_ports(&self, localHostName: &super::HostName, localPort: &HStringArg, remoteHostName: &super::HostName, remotePort: &HStringArg) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindEndpointPairByHostNamesAndPorts)(self as *const _ as *mut _, localHostName as *const _ as *mut _, localPort.get(), remoteHostName as *const _ as *mut _, remotePort.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXboxLiveEndpointPairTemplate, 1797811919, 13399, 16590, 185, 161, 192, 207, 224, 33, 62, 167);
RT_INTERFACE!{interface IXboxLiveEndpointPairTemplate(IXboxLiveEndpointPairTemplateVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPairTemplate] {
    fn add_InboundEndpointPairCreated(&self, handler: *mut foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_InboundEndpointPairCreated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateEndpointPairDefaultAsync(&self, deviceAddress: *mut XboxLiveDeviceAddress, out: *mut *mut foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>) -> HRESULT,
    fn CreateEndpointPairWithBehaviorsAsync(&self, deviceAddress: *mut XboxLiveDeviceAddress, behaviors: XboxLiveEndpointPairCreationBehaviors, out: *mut *mut foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>) -> HRESULT,
    fn CreateEndpointPairForPortsDefaultAsync(&self, deviceAddress: *mut XboxLiveDeviceAddress, initiatorPort: HSTRING, acceptorPort: HSTRING, out: *mut *mut foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>) -> HRESULT,
    fn CreateEndpointPairForPortsWithBehaviorsAsync(&self, deviceAddress: *mut XboxLiveDeviceAddress, initiatorPort: HSTRING, acceptorPort: HSTRING, behaviors: XboxLiveEndpointPairCreationBehaviors, out: *mut *mut foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SocketKind(&self, out: *mut XboxLiveSocketKind) -> HRESULT,
    fn get_InitiatorBoundPortRangeLower(&self, out: *mut u16) -> HRESULT,
    fn get_InitiatorBoundPortRangeUpper(&self, out: *mut u16) -> HRESULT,
    fn get_AcceptorBoundPortRangeLower(&self, out: *mut u16) -> HRESULT,
    fn get_AcceptorBoundPortRangeUpper(&self, out: *mut u16) -> HRESULT,
    fn get_EndpointPairs(&self, out: *mut *mut foundation::collections::IVectorView<XboxLiveEndpointPair>) -> HRESULT
}}
impl IXboxLiveEndpointPairTemplate {
    #[inline] pub fn add_inbound_endpoint_pair_created(&self, handler: &foundation::TypedEventHandler<XboxLiveEndpointPairTemplate, XboxLiveInboundEndpointPairCreatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_InboundEndpointPairCreated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_inbound_endpoint_pair_created(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_InboundEndpointPairCreated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_endpoint_pair_default_async(&self, deviceAddress: &XboxLiveDeviceAddress) -> Result<ComPtr<foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEndpointPairDefaultAsync)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_endpoint_pair_with_behaviors_async(&self, deviceAddress: &XboxLiveDeviceAddress, behaviors: XboxLiveEndpointPairCreationBehaviors) -> Result<ComPtr<foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEndpointPairWithBehaviorsAsync)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, behaviors, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_endpoint_pair_for_ports_default_async(&self, deviceAddress: &XboxLiveDeviceAddress, initiatorPort: &HStringArg, acceptorPort: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEndpointPairForPortsDefaultAsync)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, initiatorPort.get(), acceptorPort.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_endpoint_pair_for_ports_with_behaviors_async(&self, deviceAddress: &XboxLiveDeviceAddress, initiatorPort: &HStringArg, acceptorPort: &HStringArg, behaviors: XboxLiveEndpointPairCreationBehaviors) -> Result<ComPtr<foundation::IAsyncOperation<XboxLiveEndpointPairCreationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateEndpointPairForPortsWithBehaviorsAsync)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, initiatorPort.get(), acceptorPort.get(), behaviors, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_socket_kind(&self) -> Result<XboxLiveSocketKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SocketKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_initiator_bound_port_range_lower(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InitiatorBoundPortRangeLower)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_initiator_bound_port_range_upper(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InitiatorBoundPortRangeUpper)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_acceptor_bound_port_range_lower(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AcceptorBoundPortRangeLower)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_acceptor_bound_port_range_upper(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AcceptorBoundPortRangeUpper)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_endpoint_pairs(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveEndpointPair>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EndpointPairs)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveEndpointPairTemplate: IXboxLiveEndpointPairTemplate}
impl RtActivatable<IXboxLiveEndpointPairTemplateStatics> for XboxLiveEndpointPairTemplate {}
impl XboxLiveEndpointPairTemplate {
    #[inline] pub fn get_template_by_name(name: &HStringArg) -> Result<Option<ComPtr<XboxLiveEndpointPairTemplate>>> {
        <Self as RtActivatable<IXboxLiveEndpointPairTemplateStatics>>::get_activation_factory().get_template_by_name(name)
    }
    #[inline] pub fn get_templates() -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveEndpointPairTemplate>>>> {
        <Self as RtActivatable<IXboxLiveEndpointPairTemplateStatics>>::get_activation_factory().get_templates()
    }
}
DEFINE_CLSID!(XboxLiveEndpointPairTemplate(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,88,98,111,120,76,105,118,101,46,88,98,111,120,76,105,118,101,69,110,100,112,111,105,110,116,80,97,105,114,84,101,109,112,108,97,116,101,0]) [CLSID_XboxLiveEndpointPairTemplate]);
DEFINE_IID!(IID_IXboxLiveEndpointPairTemplateStatics, 504566651, 29563, 18979, 188, 100, 8, 112, 247, 86, 85, 186);
RT_INTERFACE!{static interface IXboxLiveEndpointPairTemplateStatics(IXboxLiveEndpointPairTemplateStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveEndpointPairTemplateStatics] {
    fn GetTemplateByName(&self, name: HSTRING, out: *mut *mut XboxLiveEndpointPairTemplate) -> HRESULT,
    fn get_Templates(&self, out: *mut *mut foundation::collections::IVectorView<XboxLiveEndpointPairTemplate>) -> HRESULT
}}
impl IXboxLiveEndpointPairTemplateStatics {
    #[inline] pub fn get_template_by_name(&self, name: &HStringArg) -> Result<Option<ComPtr<XboxLiveEndpointPairTemplate>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTemplateByName)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_templates(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveEndpointPairTemplate>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Templates)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXboxLiveInboundEndpointPairCreatedEventArgs, 3692575586, 8890, 18642, 128, 222, 194, 57, 104, 189, 25, 139);
RT_INTERFACE!{interface IXboxLiveInboundEndpointPairCreatedEventArgs(IXboxLiveInboundEndpointPairCreatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveInboundEndpointPairCreatedEventArgs] {
    fn get_EndpointPair(&self, out: *mut *mut XboxLiveEndpointPair) -> HRESULT
}}
impl IXboxLiveInboundEndpointPairCreatedEventArgs {
    #[inline] pub fn get_endpoint_pair(&self) -> Result<Option<ComPtr<XboxLiveEndpointPair>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EndpointPair)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveInboundEndpointPairCreatedEventArgs: IXboxLiveInboundEndpointPairCreatedEventArgs}
RT_ENUM! { enum XboxLiveNetworkAccessKind: i32 {
    Open = 0, Moderate = 1, Strict = 2,
}}
DEFINE_IID!(IID_IXboxLiveQualityOfServiceMeasurement, 1298672590, 42454, 18406, 162, 54, 207, 222, 95, 189, 242, 237);
RT_INTERFACE!{interface IXboxLiveQualityOfServiceMeasurement(IXboxLiveQualityOfServiceMeasurementVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveQualityOfServiceMeasurement] {
    fn MeasureAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn GetMetricResultsForDevice(&self, deviceAddress: *mut XboxLiveDeviceAddress, out: *mut *mut foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>) -> HRESULT,
    fn GetMetricResultsForMetric(&self, metric: XboxLiveQualityOfServiceMetric, out: *mut *mut foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>) -> HRESULT,
    fn GetMetricResult(&self, deviceAddress: *mut XboxLiveDeviceAddress, metric: XboxLiveQualityOfServiceMetric, out: *mut *mut XboxLiveQualityOfServiceMetricResult) -> HRESULT,
    fn GetPrivatePayloadResult(&self, deviceAddress: *mut XboxLiveDeviceAddress, out: *mut *mut XboxLiveQualityOfServicePrivatePayloadResult) -> HRESULT,
    fn get_Metrics(&self, out: *mut *mut foundation::collections::IVector<XboxLiveQualityOfServiceMetric>) -> HRESULT,
    fn get_DeviceAddresses(&self, out: *mut *mut foundation::collections::IVector<XboxLiveDeviceAddress>) -> HRESULT,
    fn get_ShouldRequestPrivatePayloads(&self, out: *mut bool) -> HRESULT,
    fn put_ShouldRequestPrivatePayloads(&self, value: bool) -> HRESULT,
    fn get_TimeoutInMilliseconds(&self, out: *mut u32) -> HRESULT,
    fn put_TimeoutInMilliseconds(&self, value: u32) -> HRESULT,
    fn get_NumberOfProbesToAttempt(&self, out: *mut u32) -> HRESULT,
    fn put_NumberOfProbesToAttempt(&self, value: u32) -> HRESULT,
    fn get_NumberOfResultsPending(&self, out: *mut u32) -> HRESULT,
    fn get_MetricResults(&self, out: *mut *mut foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>) -> HRESULT,
    fn get_PrivatePayloadResults(&self, out: *mut *mut foundation::collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>) -> HRESULT
}}
impl IXboxLiveQualityOfServiceMeasurement {
    #[inline] pub fn measure_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MeasureAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metric_results_for_device(&self, deviceAddress: &XboxLiveDeviceAddress) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMetricResultsForDevice)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metric_results_for_metric(&self, metric: XboxLiveQualityOfServiceMetric) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMetricResultsForMetric)(self as *const _ as *mut _, metric, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metric_result(&self, deviceAddress: &XboxLiveDeviceAddress, metric: XboxLiveQualityOfServiceMetric) -> Result<Option<ComPtr<XboxLiveQualityOfServiceMetricResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMetricResult)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, metric, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_private_payload_result(&self, deviceAddress: &XboxLiveDeviceAddress) -> Result<Option<ComPtr<XboxLiveQualityOfServicePrivatePayloadResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPrivatePayloadResult)(self as *const _ as *mut _, deviceAddress as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metrics(&self) -> Result<Option<ComPtr<foundation::collections::IVector<XboxLiveQualityOfServiceMetric>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Metrics)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_addresses(&self) -> Result<Option<ComPtr<foundation::collections::IVector<XboxLiveDeviceAddress>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceAddresses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_should_request_private_payloads(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShouldRequestPrivatePayloads)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_should_request_private_payloads(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ShouldRequestPrivatePayloads)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_timeout_in_milliseconds(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TimeoutInMilliseconds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_timeout_in_milliseconds(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TimeoutInMilliseconds)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_probes_to_attempt(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfProbesToAttempt)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_number_of_probes_to_attempt(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NumberOfProbesToAttempt)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_results_pending(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfResultsPending)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metric_results(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveQualityOfServiceMetricResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MetricResults)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_private_payload_results(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<XboxLiveQualityOfServicePrivatePayloadResult>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrivatePayloadResults)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveQualityOfServiceMeasurement: IXboxLiveQualityOfServiceMeasurement}
impl RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics> for XboxLiveQualityOfServiceMeasurement {}
impl RtActivatable<IActivationFactory> for XboxLiveQualityOfServiceMeasurement {}
impl XboxLiveQualityOfServiceMeasurement {
    #[inline] pub fn publish_private_payload_bytes(payload: &[u8]) -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().publish_private_payload_bytes(payload)
    }
    #[inline] pub fn clear_private_payload() -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().clear_private_payload()
    }
    #[inline] pub fn get_max_simultaneous_probe_connections() -> Result<u32> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().get_max_simultaneous_probe_connections()
    }
    #[inline] pub fn set_max_simultaneous_probe_connections(value: u32) -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().set_max_simultaneous_probe_connections(value)
    }
    #[inline] pub fn get_is_system_outbound_bandwidth_constrained() -> Result<bool> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().get_is_system_outbound_bandwidth_constrained()
    }
    #[inline] pub fn set_is_system_outbound_bandwidth_constrained(value: bool) -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().set_is_system_outbound_bandwidth_constrained(value)
    }
    #[inline] pub fn get_is_system_inbound_bandwidth_constrained() -> Result<bool> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().get_is_system_inbound_bandwidth_constrained()
    }
    #[inline] pub fn set_is_system_inbound_bandwidth_constrained(value: bool) -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().set_is_system_inbound_bandwidth_constrained(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_published_private_payload() -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().get_published_private_payload()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_published_private_payload(value: &super::super::storage::streams::IBuffer) -> Result<()> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().set_published_private_payload(value)
    }
    #[inline] pub fn get_max_private_payload_size() -> Result<u32> {
        <Self as RtActivatable<IXboxLiveQualityOfServiceMeasurementStatics>>::get_activation_factory().get_max_private_payload_size()
    }
}
DEFINE_CLSID!(XboxLiveQualityOfServiceMeasurement(&[87,105,110,100,111,119,115,46,78,101,116,119,111,114,107,105,110,103,46,88,98,111,120,76,105,118,101,46,88,98,111,120,76,105,118,101,81,117,97,108,105,116,121,79,102,83,101,114,118,105,99,101,77,101,97,115,117,114,101,109,101,110,116,0]) [CLSID_XboxLiveQualityOfServiceMeasurement]);
DEFINE_IID!(IID_IXboxLiveQualityOfServiceMeasurementStatics, 1848978890, 9167, 17418, 176, 119, 94, 48, 133, 122, 130, 52);
RT_INTERFACE!{static interface IXboxLiveQualityOfServiceMeasurementStatics(IXboxLiveQualityOfServiceMeasurementStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveQualityOfServiceMeasurementStatics] {
    fn PublishPrivatePayloadBytes(&self, payloadSize: u32, payload: *mut u8) -> HRESULT,
    fn ClearPrivatePayload(&self) -> HRESULT,
    fn get_MaxSimultaneousProbeConnections(&self, out: *mut u32) -> HRESULT,
    fn put_MaxSimultaneousProbeConnections(&self, value: u32) -> HRESULT,
    fn get_IsSystemOutboundBandwidthConstrained(&self, out: *mut bool) -> HRESULT,
    fn put_IsSystemOutboundBandwidthConstrained(&self, value: bool) -> HRESULT,
    fn get_IsSystemInboundBandwidthConstrained(&self, out: *mut bool) -> HRESULT,
    fn put_IsSystemInboundBandwidthConstrained(&self, value: bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_PublishedPrivatePayload(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_PublishedPrivatePayload(&self, value: *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_MaxPrivatePayloadSize(&self, out: *mut u32) -> HRESULT
}}
impl IXboxLiveQualityOfServiceMeasurementStatics {
    #[inline] pub fn publish_private_payload_bytes(&self, payload: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).PublishPrivatePayloadBytes)(self as *const _ as *mut _, payload.len() as u32, payload.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear_private_payload(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ClearPrivatePayload)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_simultaneous_probe_connections(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxSimultaneousProbeConnections)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_simultaneous_probe_connections(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxSimultaneousProbeConnections)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_system_outbound_bandwidth_constrained(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSystemOutboundBandwidthConstrained)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_system_outbound_bandwidth_constrained(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsSystemOutboundBandwidthConstrained)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_system_inbound_bandwidth_constrained(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSystemInboundBandwidthConstrained)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_system_inbound_bandwidth_constrained(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsSystemInboundBandwidthConstrained)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_published_private_payload(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PublishedPrivatePayload)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_published_private_payload(&self, value: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PublishedPrivatePayload)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_private_payload_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxPrivatePayloadSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum XboxLiveQualityOfServiceMeasurementStatus: i32 {
    NotStarted = 0, InProgress = 1, InProgressWithProvisionalResults = 2, Succeeded = 3, NoLocalNetworks = 4, NoCompatibleNetworkPaths = 5, LocalSystemNotAuthorized = 6, Canceled = 7, TimedOut = 8, RemoteSystemNotAuthorized = 9, RefusedDueToConfiguration = 10, UnexpectedInternalError = 11,
}}
RT_ENUM! { enum XboxLiveQualityOfServiceMetric: i32 {
    AverageLatencyInMilliseconds = 0, MinLatencyInMilliseconds = 1, MaxLatencyInMilliseconds = 2, AverageOutboundBitsPerSecond = 3, MinOutboundBitsPerSecond = 4, MaxOutboundBitsPerSecond = 5, AverageInboundBitsPerSecond = 6, MinInboundBitsPerSecond = 7, MaxInboundBitsPerSecond = 8,
}}
DEFINE_IID!(IID_IXboxLiveQualityOfServiceMetricResult, 2934723537, 13665, 18306, 176, 207, 211, 174, 41, 217, 250, 135);
RT_INTERFACE!{interface IXboxLiveQualityOfServiceMetricResult(IXboxLiveQualityOfServiceMetricResultVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveQualityOfServiceMetricResult] {
    fn get_Status(&self, out: *mut XboxLiveQualityOfServiceMeasurementStatus) -> HRESULT,
    fn get_DeviceAddress(&self, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    fn get_Metric(&self, out: *mut XboxLiveQualityOfServiceMetric) -> HRESULT,
    fn get_Value(&self, out: *mut u64) -> HRESULT
}}
impl IXboxLiveQualityOfServiceMetricResult {
    #[inline] pub fn get_status(&self) -> Result<XboxLiveQualityOfServiceMeasurementStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_address(&self) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metric(&self) -> Result<XboxLiveQualityOfServiceMetric> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Metric)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveQualityOfServiceMetricResult: IXboxLiveQualityOfServiceMetricResult}
DEFINE_IID!(IID_IXboxLiveQualityOfServicePrivatePayloadResult, 1516438190, 28472, 16832, 159, 204, 234, 108, 185, 120, 202, 252);
RT_INTERFACE!{interface IXboxLiveQualityOfServicePrivatePayloadResult(IXboxLiveQualityOfServicePrivatePayloadResultVtbl): IInspectable(IInspectableVtbl) [IID_IXboxLiveQualityOfServicePrivatePayloadResult] {
    fn get_Status(&self, out: *mut XboxLiveQualityOfServiceMeasurementStatus) -> HRESULT,
    fn get_DeviceAddress(&self, out: *mut *mut XboxLiveDeviceAddress) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Value(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IXboxLiveQualityOfServicePrivatePayloadResult {
    #[inline] pub fn get_status(&self) -> Result<XboxLiveQualityOfServiceMeasurementStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_address(&self) -> Result<Option<ComPtr<XboxLiveDeviceAddress>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceAddress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XboxLiveQualityOfServicePrivatePayloadResult: IXboxLiveQualityOfServicePrivatePayloadResult}
RT_ENUM! { enum XboxLiveSocketKind: i32 {
    None = 0, Datagram = 1, Stream = 2,
}}
} // Windows.Networking.XboxLive
