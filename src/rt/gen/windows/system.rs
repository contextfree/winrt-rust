use crate::prelude::*;
DEFINE_IID!(IID_IAppActivationResult, 1800571136, 62574, 20144, 170, 108, 56, 175, 85, 124, 249, 237);
RT_INTERFACE!{interface IAppActivationResult(IAppActivationResultVtbl): IInspectable(IInspectableVtbl) [IID_IAppActivationResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_AppResourceGroupInfo(&self, out: *mut *mut AppResourceGroupInfo) -> HRESULT
}}
impl IAppActivationResult {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_resource_group_info(&self) -> Result<Option<ComPtr<AppResourceGroupInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppResourceGroupInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppActivationResult: IAppActivationResult}
DEFINE_IID!(IID_IAppDiagnosticInfo, 3813189274, 34953, 19619, 190, 7, 213, 255, 255, 95, 8, 4);
RT_INTERFACE!{interface IAppDiagnosticInfo(IAppDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfo] {
    #[cfg(feature="windows-applicationmodel")] fn get_AppInfo(&self, out: *mut *mut super::applicationmodel::AppInfo) -> HRESULT
}}
impl IAppDiagnosticInfo {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_app_info(&self) -> Result<Option<ComPtr<super::applicationmodel::AppInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppDiagnosticInfo: IAppDiagnosticInfo}
impl RtActivatable<IAppDiagnosticInfoStatics> for AppDiagnosticInfo {}
impl RtActivatable<IAppDiagnosticInfoStatics2> for AppDiagnosticInfo {}
impl AppDiagnosticInfo {
    #[inline] pub fn request_info_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics>>::get_activation_factory().request_info_async()
    }
    #[inline] pub fn create_watcher() -> Result<Option<ComPtr<AppDiagnosticInfoWatcher>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().create_watcher()
    }
    #[inline] pub fn request_access_async() -> Result<ComPtr<foundation::IAsyncOperation<DiagnosticAccessStatus>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_access_async()
    }
    #[inline] pub fn request_info_for_package_async(packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_package_async(packageFamilyName)
    }
    #[inline] pub fn request_info_for_app_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_app_async()
    }
    #[inline] pub fn request_info_for_app_user_model_id(appUserModelId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> {
        <Self as RtActivatable<IAppDiagnosticInfoStatics2>>::get_activation_factory().request_info_for_app_user_model_id(appUserModelId)
    }
}
DEFINE_CLSID!(AppDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,65,112,112,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_AppDiagnosticInfo]);
DEFINE_IID!(IID_IAppDiagnosticInfo2, 3745971159, 6426, 17516, 148, 115, 143, 188, 35, 116, 163, 84);
RT_INTERFACE!{interface IAppDiagnosticInfo2(IAppDiagnosticInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfo2] {
    fn GetResourceGroups(&self, out: *mut *mut foundation::collections::IVector<AppResourceGroupInfo>) -> HRESULT,
    fn CreateResourceGroupWatcher(&self, out: *mut *mut AppResourceGroupInfoWatcher) -> HRESULT
}}
impl IAppDiagnosticInfo2 {
    #[inline] pub fn get_resource_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<AppResourceGroupInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetResourceGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_resource_group_watcher(&self) -> Result<Option<ComPtr<AppResourceGroupInfoWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateResourceGroupWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppDiagnosticInfo3, 3365258813, 56673, 19557, 186, 189, 129, 161, 11, 79, 152, 21);
RT_INTERFACE!{interface IAppDiagnosticInfo3(IAppDiagnosticInfo3Vtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfo3] {
    fn LaunchAsync(&self, out: *mut *mut foundation::IAsyncOperation<AppActivationResult>) -> HRESULT
}}
impl IAppDiagnosticInfo3 {
    #[inline] pub fn launch_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<AppActivationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppDiagnosticInfoStatics, 3462997439, 4298, 16584, 169, 202, 197, 201, 101, 1, 134, 110);
RT_INTERFACE!{static interface IAppDiagnosticInfoStatics(IAppDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoStatics] {
    fn RequestInfoAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT
}}
impl IAppDiagnosticInfoStatics {
    #[inline] pub fn request_info_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppDiagnosticInfoStatics2, 95570822, 4096, 19600, 187, 159, 114, 53, 7, 28, 80, 254);
RT_INTERFACE!{static interface IAppDiagnosticInfoStatics2(IAppDiagnosticInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoStatics2] {
    fn CreateWatcher(&self, out: *mut *mut AppDiagnosticInfoWatcher) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut foundation::IAsyncOperation<DiagnosticAccessStatus>) -> HRESULT,
    fn RequestInfoForPackageAsync(&self, packageFamilyName: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT,
    fn RequestInfoForAppAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT,
    fn RequestInfoForAppUserModelId(&self, appUserModelId: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>) -> HRESULT
}}
impl IAppDiagnosticInfoStatics2 {
    #[inline] pub fn create_watcher(&self) -> Result<Option<ComPtr<AppDiagnosticInfoWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<DiagnosticAccessStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_info_for_package_async(&self, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForPackageAsync)(self as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_info_for_app_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForAppAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_info_for_app_user_model_id(&self, appUserModelId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestInfoForAppUserModelId)(self as *const _ as *mut _, appUserModelId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppDiagnosticInfoWatcher, 1968656496, 467, 18586, 147, 37, 82, 249, 204, 110, 222, 10);
RT_INTERFACE!{interface IAppDiagnosticInfoWatcher(IAppDiagnosticInfoWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoWatcher] {
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut AppDiagnosticInfoWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IAppDiagnosticInfoWatcher {
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<AppDiagnosticInfoWatcher, AppDiagnosticInfoWatcherEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<AppDiagnosticInfoWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<AppDiagnosticInfoWatcherStatus> { unsafe { 
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
RT_CLASS!{class AppDiagnosticInfoWatcher: IAppDiagnosticInfoWatcher}
DEFINE_IID!(IID_IAppDiagnosticInfoWatcherEventArgs, 1880606486, 57818, 19557, 153, 223, 4, 109, 255, 91, 231, 26);
RT_INTERFACE!{interface IAppDiagnosticInfoWatcherEventArgs(IAppDiagnosticInfoWatcherEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppDiagnosticInfoWatcherEventArgs] {
    fn get_AppDiagnosticInfo(&self, out: *mut *mut AppDiagnosticInfo) -> HRESULT
}}
impl IAppDiagnosticInfoWatcherEventArgs {
    #[inline] pub fn get_app_diagnostic_info(&self) -> Result<Option<ComPtr<AppDiagnosticInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppDiagnosticInfoWatcherEventArgs: IAppDiagnosticInfoWatcherEventArgs}
RT_ENUM! { enum AppDiagnosticInfoWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
DEFINE_IID!(IID_IAppExecutionStateChangeResult, 1862507504, 63771, 19960, 174, 119, 48, 51, 204, 182, 145, 20);
RT_INTERFACE!{interface IAppExecutionStateChangeResult(IAppExecutionStateChangeResultVtbl): IInspectable(IInspectableVtbl) [IID_IAppExecutionStateChangeResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl IAppExecutionStateChangeResult {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AppExecutionStateChangeResult: IAppExecutionStateChangeResult}
DEFINE_IID!(IID_IAppMemoryReport, 1835348891, 19823, 17852, 156, 94, 228, 155, 63, 242, 117, 141);
RT_INTERFACE!{interface IAppMemoryReport(IAppMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryReport] {
    fn get_PrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryReport {
    #[inline] pub fn get_private_commit_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_private_commit_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_total_commit_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_total_commit_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AppMemoryReport: IAppMemoryReport}
DEFINE_IID!(IID_IAppMemoryReport2, 1602172728, 20919, 17116, 183, 237, 121, 186, 70, 210, 136, 87);
RT_INTERFACE!{interface IAppMemoryReport2(IAppMemoryReport2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryReport2] {
    fn get_ExpectedTotalCommitLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryReport2 {
    #[inline] pub fn get_expected_total_commit_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpectedTotalCommitLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum AppMemoryUsageLevel: i32 {
    Low = 0, Medium = 1, High = 2, OverLimit = 3,
}}
DEFINE_IID!(IID_IAppMemoryUsageLimitChangingEventArgs, 2046322276, 65226, 19877, 158, 64, 43, 198, 62, 253, 201, 121);
RT_INTERFACE!{interface IAppMemoryUsageLimitChangingEventArgs(IAppMemoryUsageLimitChangingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppMemoryUsageLimitChangingEventArgs] {
    fn get_OldLimit(&self, out: *mut u64) -> HRESULT,
    fn get_NewLimit(&self, out: *mut u64) -> HRESULT
}}
impl IAppMemoryUsageLimitChangingEventArgs {
    #[inline] pub fn get_old_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OldLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_new_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NewLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AppMemoryUsageLimitChangingEventArgs: IAppMemoryUsageLimitChangingEventArgs}
DEFINE_IID!(IID_IAppResourceGroupBackgroundTaskReport, 627500878, 45149, 16578, 157, 193, 26, 79, 3, 158, 161, 32);
RT_INTERFACE!{interface IAppResourceGroupBackgroundTaskReport(IAppResourceGroupBackgroundTaskReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupBackgroundTaskReport] {
    fn get_TaskId(&self, out: *mut Guid) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Trigger(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EntryPoint(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAppResourceGroupBackgroundTaskReport {
    #[inline] pub fn get_task_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TaskId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_trigger(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Trigger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_entry_point(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EntryPoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupBackgroundTaskReport: IAppResourceGroupBackgroundTaskReport}
RT_ENUM! { enum AppResourceGroupEnergyQuotaState: i32 {
    Unknown = 0, Over = 1, Under = 2,
}}
RT_ENUM! { enum AppResourceGroupExecutionState: i32 {
    Unknown = 0, Running = 1, Suspending = 2, Suspended = 3, NotRunning = 4,
}}
DEFINE_IID!(IID_IAppResourceGroupInfo, 3105093498, 59399, 18932, 132, 94, 123, 139, 220, 254, 142, 231);
RT_INTERFACE!{interface IAppResourceGroupInfo(IAppResourceGroupInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfo] {
    fn get_InstanceId(&self, out: *mut Guid) -> HRESULT,
    fn get_IsShared(&self, out: *mut bool) -> HRESULT,
    fn GetBackgroundTaskReports(&self, out: *mut *mut foundation::collections::IVector<AppResourceGroupBackgroundTaskReport>) -> HRESULT,
    fn GetMemoryReport(&self, out: *mut *mut AppResourceGroupMemoryReport) -> HRESULT,
    fn GetProcessDiagnosticInfos(&self, out: *mut *mut foundation::collections::IVector<diagnostics::ProcessDiagnosticInfo>) -> HRESULT,
    fn GetStateReport(&self, out: *mut *mut AppResourceGroupStateReport) -> HRESULT
}}
impl IAppResourceGroupInfo {
    #[inline] pub fn get_instance_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InstanceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_shared(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsShared)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_background_task_reports(&self) -> Result<Option<ComPtr<foundation::collections::IVector<AppResourceGroupBackgroundTaskReport>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetBackgroundTaskReports)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_memory_report(&self) -> Result<Option<ComPtr<AppResourceGroupMemoryReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_process_diagnostic_infos(&self) -> Result<Option<ComPtr<foundation::collections::IVector<diagnostics::ProcessDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProcessDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state_report(&self) -> Result<Option<ComPtr<AppResourceGroupStateReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetStateReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupInfo: IAppResourceGroupInfo}
DEFINE_IID!(IID_IAppResourceGroupInfo2, 4003144557, 54021, 19819, 146, 247, 106, 253, 173, 114, 222, 220);
RT_INTERFACE!{interface IAppResourceGroupInfo2(IAppResourceGroupInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfo2] {
    fn StartSuspendAsync(&self, out: *mut *mut foundation::IAsyncOperation<AppExecutionStateChangeResult>) -> HRESULT,
    fn StartResumeAsync(&self, out: *mut *mut foundation::IAsyncOperation<AppExecutionStateChangeResult>) -> HRESULT,
    fn StartTerminateAsync(&self, out: *mut *mut foundation::IAsyncOperation<AppExecutionStateChangeResult>) -> HRESULT
}}
impl IAppResourceGroupInfo2 {
    #[inline] pub fn start_suspend_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<AppExecutionStateChangeResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartSuspendAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_resume_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<AppExecutionStateChangeResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartResumeAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_terminate_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<AppExecutionStateChangeResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).StartTerminateAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcher, 3652231421, 28250, 19570, 139, 23, 9, 254, 196, 162, 18, 189);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcher(IAppResourceGroupInfoWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcher] {
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ExecutionStateChanged(&self, handler: *mut foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ExecutionStateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut AppResourceGroupInfoWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl IAppResourceGroupInfoWatcher {
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<AppResourceGroupInfoWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_execution_state_changed(&self, handler: &foundation::TypedEventHandler<AppResourceGroupInfoWatcher, AppResourceGroupInfoWatcherExecutionStateChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ExecutionStateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_execution_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ExecutionStateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<AppResourceGroupInfoWatcherStatus> { unsafe { 
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
RT_CLASS!{class AppResourceGroupInfoWatcher: IAppResourceGroupInfoWatcher}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcherEventArgs, 2054714935, 25346, 19759, 191, 137, 28, 18, 208, 178, 166, 185);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcherEventArgs(IAppResourceGroupInfoWatcherEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcherEventArgs] {
    fn get_AppDiagnosticInfos(&self, out: *mut *mut foundation::collections::IVectorView<AppDiagnosticInfo>) -> HRESULT,
    fn get_AppResourceGroupInfo(&self, out: *mut *mut AppResourceGroupInfo) -> HRESULT
}}
impl IAppResourceGroupInfoWatcherEventArgs {
    #[inline] pub fn get_app_diagnostic_infos(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_resource_group_info(&self) -> Result<Option<ComPtr<AppResourceGroupInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppResourceGroupInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupInfoWatcherEventArgs: IAppResourceGroupInfoWatcherEventArgs}
DEFINE_IID!(IID_IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs, 467398103, 65254, 20436, 152, 221, 233, 42, 44, 194, 153, 243);
RT_INTERFACE!{interface IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs(IAppResourceGroupInfoWatcherExecutionStateChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs] {
    fn get_AppDiagnosticInfos(&self, out: *mut *mut foundation::collections::IVectorView<AppDiagnosticInfo>) -> HRESULT,
    fn get_AppResourceGroupInfo(&self, out: *mut *mut AppResourceGroupInfo) -> HRESULT
}}
impl IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs {
    #[inline] pub fn get_app_diagnostic_infos(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_resource_group_info(&self) -> Result<Option<ComPtr<AppResourceGroupInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppResourceGroupInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupInfoWatcherExecutionStateChangedEventArgs: IAppResourceGroupInfoWatcherExecutionStateChangedEventArgs}
RT_ENUM! { enum AppResourceGroupInfoWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
DEFINE_IID!(IID_IAppResourceGroupMemoryReport, 747374257, 32177, 19537, 162, 37, 127, 174, 45, 73, 228, 49);
RT_INTERFACE!{interface IAppResourceGroupMemoryReport(IAppResourceGroupMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupMemoryReport] {
    fn get_CommitUsageLimit(&self, out: *mut u64) -> HRESULT,
    fn get_CommitUsageLevel(&self, out: *mut AppMemoryUsageLevel) -> HRESULT,
    fn get_PrivateCommitUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalCommitUsage(&self, out: *mut u64) -> HRESULT
}}
impl IAppResourceGroupMemoryReport {
    #[inline] pub fn get_commit_usage_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommitUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_commit_usage_level(&self) -> Result<AppMemoryUsageLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommitUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_private_commit_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_total_commit_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalCommitUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupMemoryReport: IAppResourceGroupMemoryReport}
DEFINE_IID!(IID_IAppResourceGroupStateReport, 1384423192, 12144, 16950, 171, 64, 208, 77, 176, 199, 185, 49);
RT_INTERFACE!{interface IAppResourceGroupStateReport(IAppResourceGroupStateReportVtbl): IInspectable(IInspectableVtbl) [IID_IAppResourceGroupStateReport] {
    fn get_ExecutionState(&self, out: *mut AppResourceGroupExecutionState) -> HRESULT,
    fn get_EnergyQuotaState(&self, out: *mut AppResourceGroupEnergyQuotaState) -> HRESULT
}}
impl IAppResourceGroupStateReport {
    #[inline] pub fn get_execution_state(&self) -> Result<AppResourceGroupExecutionState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExecutionState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_energy_quota_state(&self) -> Result<AppResourceGroupEnergyQuotaState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EnergyQuotaState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AppResourceGroupStateReport: IAppResourceGroupStateReport}
DEFINE_IID!(IID_IAppUriHandlerHost, 1565575877, 37586, 21513, 181, 111, 127, 115, 225, 14, 164, 195);
RT_INTERFACE!{interface IAppUriHandlerHost(IAppUriHandlerHostVtbl): IInspectable(IInspectableVtbl) [IID_IAppUriHandlerHost] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT
}}
impl IAppUriHandlerHost {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class AppUriHandlerHost: IAppUriHandlerHost}
impl RtActivatable<IAppUriHandlerHostFactory> for AppUriHandlerHost {}
impl RtActivatable<IActivationFactory> for AppUriHandlerHost {}
impl AppUriHandlerHost {
    #[inline] pub fn create_instance(name: &HStringArg) -> Result<ComPtr<AppUriHandlerHost>> {
        <Self as RtActivatable<IAppUriHandlerHostFactory>>::get_activation_factory().create_instance(name)
    }
}
DEFINE_CLSID!(AppUriHandlerHost(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,65,112,112,85,114,105,72,97,110,100,108,101,114,72,111,115,116,0]) [CLSID_AppUriHandlerHost]);
DEFINE_IID!(IID_IAppUriHandlerHostFactory, 628898966, 52740, 24472, 150, 187, 62, 189, 62, 146, 117, 187);
RT_INTERFACE!{static interface IAppUriHandlerHostFactory(IAppUriHandlerHostFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IAppUriHandlerHostFactory] {
    fn CreateInstance(&self, name: HSTRING, out: *mut *mut AppUriHandlerHost) -> HRESULT
}}
impl IAppUriHandlerHostFactory {
    #[inline] pub fn create_instance(&self, name: &HStringArg) -> Result<ComPtr<AppUriHandlerHost>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateInstance)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAppUriHandlerRegistration, 1869852337, 17769, 23615, 155, 160, 153, 18, 62, 234, 50, 195);
RT_INTERFACE!{interface IAppUriHandlerRegistration(IAppUriHandlerRegistrationVtbl): IInspectable(IInspectableVtbl) [IID_IAppUriHandlerRegistration] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_User(&self, out: *mut *mut User) -> HRESULT,
    fn GetAppAddedHostsAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVector<AppUriHandlerHost>>) -> HRESULT,
    fn SetAppAddedHostsAsync(&self, hosts: *mut foundation::collections::IIterable<AppUriHandlerHost>, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IAppUriHandlerRegistration {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_added_hosts_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVector<AppUriHandlerHost>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppAddedHostsAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_app_added_hosts_async(&self, hosts: &foundation::collections::IIterable<AppUriHandlerHost>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAppAddedHostsAsync)(self as *const _ as *mut _, hosts as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppUriHandlerRegistration: IAppUriHandlerRegistration}
DEFINE_IID!(IID_IAppUriHandlerRegistrationManager, 3861682770, 44180, 22352, 172, 27, 108, 251, 111, 37, 2, 99);
RT_INTERFACE!{interface IAppUriHandlerRegistrationManager(IAppUriHandlerRegistrationManagerVtbl): IInspectable(IInspectableVtbl) [IID_IAppUriHandlerRegistrationManager] {
    fn get_User(&self, out: *mut *mut User) -> HRESULT,
    fn TryGetRegistration(&self, name: HSTRING, out: *mut *mut AppUriHandlerRegistration) -> HRESULT
}}
impl IAppUriHandlerRegistrationManager {
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_get_registration(&self, name: &HStringArg) -> Result<Option<ComPtr<AppUriHandlerRegistration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetRegistration)(self as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppUriHandlerRegistrationManager: IAppUriHandlerRegistrationManager}
impl RtActivatable<IAppUriHandlerRegistrationManagerStatics> for AppUriHandlerRegistrationManager {}
impl AppUriHandlerRegistrationManager {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<AppUriHandlerRegistrationManager>>> {
        <Self as RtActivatable<IAppUriHandlerRegistrationManagerStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn get_for_user(user: &User) -> Result<Option<ComPtr<AppUriHandlerRegistrationManager>>> {
        <Self as RtActivatable<IAppUriHandlerRegistrationManagerStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(AppUriHandlerRegistrationManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,65,112,112,85,114,105,72,97,110,100,108,101,114,82,101,103,105,115,116,114,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_AppUriHandlerRegistrationManager]);
DEFINE_IID!(IID_IAppUriHandlerRegistrationManagerStatics, 3587104159, 22313, 23414, 161, 212, 2, 133, 242, 149, 193, 36);
RT_INTERFACE!{static interface IAppUriHandlerRegistrationManagerStatics(IAppUriHandlerRegistrationManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAppUriHandlerRegistrationManagerStatics] {
    fn GetDefault(&self, out: *mut *mut AppUriHandlerRegistrationManager) -> HRESULT,
    fn GetForUser(&self, user: *mut User, out: *mut *mut AppUriHandlerRegistrationManager) -> HRESULT
}}
impl IAppUriHandlerRegistrationManagerStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<AppUriHandlerRegistrationManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_user(&self, user: &User) -> Result<Option<ComPtr<AppUriHandlerRegistrationManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum AutoUpdateTimeZoneStatus: i32 {
    Attempted = 0, TimedOut = 1, Failed = 2,
}}
RT_CLASS!{static class DateTimeSettings}
impl RtActivatable<IDateTimeSettingsStatics> for DateTimeSettings {}
impl DateTimeSettings {
    #[inline] pub fn set_system_date_time(utcDateTime: foundation::DateTime) -> Result<()> {
        <Self as RtActivatable<IDateTimeSettingsStatics>>::get_activation_factory().set_system_date_time(utcDateTime)
    }
}
DEFINE_CLSID!(DateTimeSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,97,116,101,84,105,109,101,83,101,116,116,105,110,103,115,0]) [CLSID_DateTimeSettings]);
DEFINE_IID!(IID_IDateTimeSettingsStatics, 1562464465, 18414, 18603, 165, 43, 159, 25, 84, 39, 141, 130);
RT_INTERFACE!{static interface IDateTimeSettingsStatics(IDateTimeSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDateTimeSettingsStatics] {
    fn SetSystemDateTime(&self, utcDateTime: foundation::DateTime) -> HRESULT
}}
impl IDateTimeSettingsStatics {
    #[inline] pub fn set_system_date_time(&self, utcDateTime: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetSystemDateTime)(self as *const _ as *mut _, utcDateTime);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum DiagnosticAccessStatus: i32 {
    Unspecified = 0, Denied = 1, Limited = 2, Allowed = 3,
}}
DEFINE_IID!(IID_IDispatcherQueue, 1614711012, 41784, 20478, 164, 87, 165, 207, 185, 206, 184, 153);
RT_INTERFACE!{interface IDispatcherQueue(IDispatcherQueueVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueue] {
    fn CreateTimer(&self, out: *mut *mut DispatcherQueueTimer) -> HRESULT,
    fn TryEnqueue(&self, callback: *mut DispatcherQueueHandler, out: *mut bool) -> HRESULT,
    fn TryEnqueueWithPriority(&self, priority: DispatcherQueuePriority, callback: *mut DispatcherQueueHandler, out: *mut bool) -> HRESULT,
    fn add_ShutdownStarting(&self, handler: *mut foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ShutdownStarting(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ShutdownCompleted(&self, handler: *mut foundation::TypedEventHandler<DispatcherQueue, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ShutdownCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDispatcherQueue {
    #[inline] pub fn create_timer(&self) -> Result<Option<ComPtr<DispatcherQueueTimer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_enqueue(&self, callback: &DispatcherQueueHandler) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEnqueue)(self as *const _ as *mut _, callback as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_enqueue_with_priority(&self, priority: DispatcherQueuePriority, callback: &DispatcherQueueHandler) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEnqueueWithPriority)(self as *const _ as *mut _, priority, callback as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_shutdown_starting(&self, handler: &foundation::TypedEventHandler<DispatcherQueue, DispatcherQueueShutdownStartingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ShutdownStarting)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_shutdown_starting(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ShutdownStarting)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_shutdown_completed(&self, handler: &foundation::TypedEventHandler<DispatcherQueue, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ShutdownCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_shutdown_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ShutdownCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DispatcherQueue: IDispatcherQueue}
impl RtActivatable<IDispatcherQueueStatics> for DispatcherQueue {}
impl DispatcherQueue {
    #[inline] pub fn get_for_current_thread() -> Result<Option<ComPtr<DispatcherQueue>>> {
        <Self as RtActivatable<IDispatcherQueueStatics>>::get_activation_factory().get_for_current_thread()
    }
}
DEFINE_CLSID!(DispatcherQueue(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,97,116,99,104,101,114,81,117,101,117,101,0]) [CLSID_DispatcherQueue]);
DEFINE_IID!(IID_IDispatcherQueueController, 586370662, 20699, 20022, 169, 141, 97, 192, 27, 56, 77, 32);
RT_INTERFACE!{interface IDispatcherQueueController(IDispatcherQueueControllerVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueController] {
    fn get_DispatcherQueue(&self, out: *mut *mut DispatcherQueue) -> HRESULT,
    fn ShutdownQueueAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IDispatcherQueueController {
    #[inline] pub fn get_dispatcher_queue(&self) -> Result<Option<ComPtr<DispatcherQueue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DispatcherQueue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn shutdown_queue_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ShutdownQueueAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DispatcherQueueController: IDispatcherQueueController}
impl RtActivatable<IDispatcherQueueControllerStatics> for DispatcherQueueController {}
impl DispatcherQueueController {
    #[inline] pub fn create_on_dedicated_thread() -> Result<Option<ComPtr<DispatcherQueueController>>> {
        <Self as RtActivatable<IDispatcherQueueControllerStatics>>::get_activation_factory().create_on_dedicated_thread()
    }
}
DEFINE_CLSID!(DispatcherQueueController(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,97,116,99,104,101,114,81,117,101,117,101,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_DispatcherQueueController]);
DEFINE_IID!(IID_IDispatcherQueueControllerStatics, 174889184, 20888, 18850, 163, 19, 63, 112, 209, 241, 60, 39);
RT_INTERFACE!{static interface IDispatcherQueueControllerStatics(IDispatcherQueueControllerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueControllerStatics] {
    fn CreateOnDedicatedThread(&self, out: *mut *mut DispatcherQueueController) -> HRESULT
}}
impl IDispatcherQueueControllerStatics {
    #[inline] pub fn create_on_dedicated_thread(&self) -> Result<Option<ComPtr<DispatcherQueueController>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateOnDedicatedThread)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_DispatcherQueueHandler, 3751992476, 6701, 18711, 152, 242, 147, 154, 241, 214, 224, 200);
RT_DELEGATE!{delegate DispatcherQueueHandler(DispatcherQueueHandlerVtbl, DispatcherQueueHandlerImpl) [IID_DispatcherQueueHandler] {
    fn Invoke(&self) -> HRESULT
}}
impl DispatcherQueueHandler {
    #[inline] pub fn invoke(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum DispatcherQueuePriority: i32 {
    Low = -10, Normal = 0, High = 10,
}}
DEFINE_IID!(IID_IDispatcherQueueShutdownStartingEventArgs, 3295824972, 65431, 16576, 162, 38, 204, 10, 170, 84, 94, 137);
RT_INTERFACE!{interface IDispatcherQueueShutdownStartingEventArgs(IDispatcherQueueShutdownStartingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueShutdownStartingEventArgs] {
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IDispatcherQueueShutdownStartingEventArgs {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DispatcherQueueShutdownStartingEventArgs: IDispatcherQueueShutdownStartingEventArgs}
DEFINE_IID!(IID_IDispatcherQueueStatics, 2842526679, 37745, 17687, 146, 69, 208, 130, 74, 193, 44, 116);
RT_INTERFACE!{static interface IDispatcherQueueStatics(IDispatcherQueueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueStatics] {
    fn GetForCurrentThread(&self, out: *mut *mut DispatcherQueue) -> HRESULT
}}
impl IDispatcherQueueStatics {
    #[inline] pub fn get_for_current_thread(&self) -> Result<Option<ComPtr<DispatcherQueue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentThread)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDispatcherQueueTimer, 1609218845, 41756, 18215, 177, 172, 55, 69, 70, 73, 213, 106);
RT_INTERFACE!{interface IDispatcherQueueTimer(IDispatcherQueueTimerVtbl): IInspectable(IInspectableVtbl) [IID_IDispatcherQueueTimer] {
    fn get_Interval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn put_Interval(&self, value: foundation::TimeSpan) -> HRESULT,
    fn get_IsRunning(&self, out: *mut bool) -> HRESULT,
    fn get_IsRepeating(&self, out: *mut bool) -> HRESULT,
    fn put_IsRepeating(&self, value: bool) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_Tick(&self, handler: *mut foundation::TypedEventHandler<DispatcherQueueTimer, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Tick(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDispatcherQueueTimer {
    #[inline] pub fn get_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Interval)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_interval(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Interval)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_running(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRunning)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_repeating(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRepeating)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_repeating(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsRepeating)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_tick(&self, handler: &foundation::TypedEventHandler<DispatcherQueueTimer, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Tick)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_tick(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Tick)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DispatcherQueueTimer: IDispatcherQueueTimer}
DEFINE_IID!(IID_IFolderLauncherOptions, 3146891901, 27527, 17194, 189, 4, 119, 108, 111, 95, 178, 171);
RT_INTERFACE!{interface IFolderLauncherOptions(IFolderLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IFolderLauncherOptions] {
    #[cfg(feature="windows-storage")] fn get_ItemsToSelect(&self, out: *mut *mut foundation::collections::IVector<super::storage::IStorageItem>) -> HRESULT
}}
impl IFolderLauncherOptions {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_items_to_select(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::storage::IStorageItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemsToSelect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FolderLauncherOptions: IFolderLauncherOptions}
impl RtActivatable<IActivationFactory> for FolderLauncherOptions {}
DEFINE_CLSID!(FolderLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,70,111,108,100,101,114,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_FolderLauncherOptions]);
RT_CLASS!{static class KnownUserProperties}
impl RtActivatable<IKnownUserPropertiesStatics> for KnownUserProperties {}
impl KnownUserProperties {
    #[inline] pub fn get_display_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_display_name()
    }
    #[inline] pub fn get_first_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_first_name()
    }
    #[inline] pub fn get_last_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_last_name()
    }
    #[inline] pub fn get_provider_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_provider_name()
    }
    #[inline] pub fn get_account_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_account_name()
    }
    #[inline] pub fn get_guest_host() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_guest_host()
    }
    #[inline] pub fn get_principal_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_principal_name()
    }
    #[inline] pub fn get_domain_name() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_domain_name()
    }
    #[inline] pub fn get_session_initiation_protocol_uri() -> Result<HString> {
        <Self as RtActivatable<IKnownUserPropertiesStatics>>::get_activation_factory().get_session_initiation_protocol_uri()
    }
}
DEFINE_CLSID!(KnownUserProperties(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,75,110,111,119,110,85,115,101,114,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_KnownUserProperties]);
DEFINE_IID!(IID_IKnownUserPropertiesStatics, 2002096410, 28869, 18661, 182, 55, 91, 163, 68, 30, 78, 228);
RT_INTERFACE!{static interface IKnownUserPropertiesStatics(IKnownUserPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownUserPropertiesStatics] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirstName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AccountName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GuestHost(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PrincipalName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DomainName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionInitiationProtocolUri(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownUserPropertiesStatics {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FirstName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProviderName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_account_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AccountName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_guest_host(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GuestHost)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_principal_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrincipalName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DomainName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_initiation_protocol_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInitiationProtocolUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class Launcher}
impl RtActivatable<ILauncherStatics> for Launcher {}
impl RtActivatable<ILauncherStatics2> for Launcher {}
impl RtActivatable<ILauncherStatics3> for Launcher {}
impl RtActivatable<ILauncherStatics4> for Launcher {}
impl RtActivatable<ILauncherStatics5> for Launcher {}
impl Launcher {
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_async(file: &super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_file_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_with_options_async(file: &super::storage::IStorageFile, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_file_with_options_async(file, options)
    }
    #[inline] pub fn launch_uri_async(uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_uri_async(uri)
    }
    #[inline] pub fn launch_uri_with_options_async(uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics>>::get_activation_factory().launch_uri_with_options_async(uri, options)
    }
    #[inline] pub fn launch_uri_for_results_async(uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_for_results_async(uri, options)
    }
    #[inline] pub fn launch_uri_for_results_with_data_async(uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_for_results_with_data_async(uri, options, inputData)
    }
    #[inline] pub fn launch_uri_with_data_async(uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().launch_uri_with_data_async(uri, options, inputData)
    }
    #[inline] pub fn query_uri_support_async(uri: &foundation::Uri, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_uri_support_async(uri, launchQuerySupportType)
    }
    #[inline] pub fn query_uri_support_with_package_family_name_async(uri: &foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_uri_support_with_package_family_name_async(uri, launchQuerySupportType, packageFamilyName)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_async(file: &super::storage::StorageFile) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_file_support_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_with_package_family_name_async(file: &super::storage::StorageFile, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().query_file_support_with_package_family_name_async(file, packageFamilyName)
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_async(scheme: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_uri_scheme_handlers_async(scheme)
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_with_launch_uri_type_async(scheme: &HStringArg, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_uri_scheme_handlers_with_launch_uri_type_async(scheme, launchQuerySupportType)
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_file_handlers_async(extension: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        <Self as RtActivatable<ILauncherStatics2>>::get_activation_factory().find_file_handlers_async(extension)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_async(folder: &super::storage::IStorageFolder) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics3>>::get_activation_factory().launch_folder_async(folder)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_with_options_async(folder: &super::storage::IStorageFolder, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics3>>::get_activation_factory().launch_folder_with_options_async(folder, options)
    }
    #[inline] pub fn query_app_uri_support_async(uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().query_app_uri_support_async(uri)
    }
    #[inline] pub fn query_app_uri_support_with_package_family_name_async(uri: &foundation::Uri, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().query_app_uri_support_with_package_family_name_async(uri, packageFamilyName)
    }
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_app_uri_handlers_async(uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().find_app_uri_handlers_async(uri)
    }
    #[inline] pub fn launch_uri_for_user_async(user: &User, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_user_async(user, uri)
    }
    #[inline] pub fn launch_uri_with_options_for_user_async(user: &User, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_with_options_for_user_async(user, uri, options)
    }
    #[inline] pub fn launch_uri_with_data_for_user_async(user: &User, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_with_data_for_user_async(user, uri, options, inputData)
    }
    #[inline] pub fn launch_uri_for_results_for_user_async(user: &User, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_results_for_user_async(user, uri, options)
    }
    #[inline] pub fn launch_uri_for_results_with_data_for_user_async(user: &User, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> {
        <Self as RtActivatable<ILauncherStatics4>>::get_activation_factory().launch_uri_for_results_with_data_for_user_async(user, uri, options, inputData)
    }
    #[inline] pub fn launch_folder_path_async(path: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics5>>::get_activation_factory().launch_folder_path_async(path)
    }
    #[inline] pub fn launch_folder_path_with_options_async(path: &HStringArg, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics5>>::get_activation_factory().launch_folder_path_with_options_async(path, options)
    }
    #[inline] pub fn launch_folder_path_for_user_async(user: &User, path: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics5>>::get_activation_factory().launch_folder_path_for_user_async(user, path)
    }
    #[inline] pub fn launch_folder_path_with_options_for_user_async(user: &User, path: &HStringArg, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ILauncherStatics5>>::get_activation_factory().launch_folder_path_with_options_for_user_async(user, path, options)
    }
}
DEFINE_CLSID!(Launcher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,76,97,117,110,99,104,101,114,0]) [CLSID_Launcher]);
DEFINE_IID!(IID_ILauncherOptions, 3136954840, 45169, 19672, 133, 62, 52, 18, 3, 229, 87, 211);
RT_INTERFACE!{interface ILauncherOptions(ILauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions] {
    fn get_TreatAsUntrusted(&self, out: *mut bool) -> HRESULT,
    fn put_TreatAsUntrusted(&self, value: bool) -> HRESULT,
    fn get_DisplayApplicationPicker(&self, out: *mut bool) -> HRESULT,
    fn put_DisplayApplicationPicker(&self, value: bool) -> HRESULT,
    fn get_UI(&self, out: *mut *mut LauncherUIOptions) -> HRESULT,
    fn get_PreferredApplicationPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PreferredApplicationPackageFamilyName(&self, value: HSTRING) -> HRESULT,
    fn get_PreferredApplicationDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PreferredApplicationDisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_FallbackUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_FallbackUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContentType(&self, value: HSTRING) -> HRESULT
}}
impl ILauncherOptions {
    #[inline] pub fn get_treat_as_untrusted(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TreatAsUntrusted)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_treat_as_untrusted(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TreatAsUntrusted)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_application_picker(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DisplayApplicationPicker)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_display_application_picker(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DisplayApplicationPicker)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_ui(&self) -> Result<Option<ComPtr<LauncherUIOptions>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UI)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_preferred_application_package_family_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredApplicationPackageFamilyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_preferred_application_package_family_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PreferredApplicationPackageFamilyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_preferred_application_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredApplicationDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_preferred_application_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PreferredApplicationDisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_fallback_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_fallback_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FallbackUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ContentType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ContentType)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LauncherOptions: ILauncherOptions}
impl RtActivatable<IActivationFactory> for LauncherOptions {}
DEFINE_CLSID!(LauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_LauncherOptions]);
DEFINE_IID!(IID_ILauncherOptions2, 1000378036, 28224, 19918, 161, 163, 47, 83, 149, 10, 251, 73);
RT_INTERFACE!{interface ILauncherOptions2(ILauncherOptions2Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions2] {
    fn get_TargetApplicationPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_TargetApplicationPackageFamilyName(&self, value: HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_NeighboringFilesQuery(&self, out: *mut *mut super::storage::search::StorageFileQueryResult) -> HRESULT,
    #[cfg(feature="windows-storage")] fn put_NeighboringFilesQuery(&self, value: *mut super::storage::search::StorageFileQueryResult) -> HRESULT
}}
impl ILauncherOptions2 {
    #[inline] pub fn get_target_application_package_family_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetApplicationPackageFamilyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_target_application_package_family_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TargetApplicationPackageFamilyName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_neighboring_files_query(&self) -> Result<Option<ComPtr<super::storage::search::StorageFileQueryResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NeighboringFilesQuery)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_neighboring_files_query(&self, value: &super::storage::search::StorageFileQueryResult) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NeighboringFilesQuery)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherOptions3, 4034332245, 19299, 20026, 145, 7, 78, 104, 120, 65, 146, 58);
RT_INTERFACE!{interface ILauncherOptions3(ILauncherOptions3Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions3] {
    fn get_IgnoreAppUriHandlers(&self, out: *mut bool) -> HRESULT,
    fn put_IgnoreAppUriHandlers(&self, value: bool) -> HRESULT
}}
impl ILauncherOptions3 {
    #[inline] pub fn get_ignore_app_uri_handlers(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IgnoreAppUriHandlers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_ignore_app_uri_handlers(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IgnoreAppUriHandlers)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherOptions4, 4017082638, 59131, 18452, 164, 78, 87, 232, 185, 217, 160, 27);
RT_INTERFACE!{interface ILauncherOptions4(ILauncherOptions4Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherOptions4] {
    fn get_LimitPickerToCurrentAppAndAppUriHandlers(&self, out: *mut bool) -> HRESULT,
    fn put_LimitPickerToCurrentAppAndAppUriHandlers(&self, value: bool) -> HRESULT
}}
impl ILauncherOptions4 {
    #[inline] pub fn get_limit_picker_to_current_app_and_app_uri_handlers(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LimitPickerToCurrentAppAndAppUriHandlers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_limit_picker_to_current_app_and_app_uri_handlers(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_LimitPickerToCurrentAppAndAppUriHandlers)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherStatics, 661737923, 40510, 17142, 145, 164, 93, 253, 235, 35, 36, 81);
RT_INTERFACE!{static interface ILauncherStatics(ILauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn LaunchFileAsync(&self, file: *mut super::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn LaunchFileWithOptionsAsync(&self, file: *mut super::storage::IStorageFile, options: *mut LauncherOptions, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchUriAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchUriWithOptionsAsync(&self, uri: *mut foundation::Uri, options: *mut LauncherOptions, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ILauncherStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_async(&self, file: &super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFileAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_file_with_options_async(&self, file: &super::storage::IStorageFile, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFileWithOptionsAsync)(self as *const _ as *mut _, file as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_async(&self, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_options_async(&self, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherStatics2, 1505374139, 9419, 19458, 164, 196, 130, 148, 86, 157, 84, 241);
RT_INTERFACE!{static interface ILauncherStatics2(ILauncherStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics2] {
    fn LaunchUriForResultsAsync(&self, uri: *mut foundation::Uri, options: *mut LauncherOptions, out: *mut *mut foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriForResultsWithDataAsync(&self, uri: *mut foundation::Uri, options: *mut LauncherOptions, inputData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriWithDataAsync(&self, uri: *mut foundation::Uri, options: *mut LauncherOptions, inputData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn QueryUriSupportAsync(&self, uri: *mut foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    fn QueryUriSupportWithPackageFamilyNameAsync(&self, uri: *mut foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: HSTRING, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn QueryFileSupportAsync(&self, file: *mut super::storage::StorageFile, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn QueryFileSupportWithPackageFamilyNameAsync(&self, file: *mut super::storage::StorageFile, packageFamilyName: HSTRING, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindUriSchemeHandlersAsync(&self, scheme: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindUriSchemeHandlersWithLaunchUriTypeAsync(&self, scheme: HSTRING, launchQuerySupportType: LaunchQuerySupportType, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindFileHandlersAsync(&self, extension: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT
}}
impl ILauncherStatics2 {
    #[inline] pub fn launch_uri_for_results_async(&self, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_for_results_with_data_async(&self, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsWithDataAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_data_async(&self, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn query_uri_support_async(&self, uri: &foundation::Uri, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryUriSupportAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, launchQuerySupportType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn query_uri_support_with_package_family_name_async(&self, uri: &foundation::Uri, launchQuerySupportType: LaunchQuerySupportType, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryUriSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, launchQuerySupportType, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_async(&self, file: &super::storage::StorageFile) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryFileSupportAsync)(self as *const _ as *mut _, file as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn query_file_support_with_package_family_name_async(&self, file: &super::storage::StorageFile, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryFileSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, file as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_async(&self, scheme: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUriSchemeHandlersAsync)(self as *const _ as *mut _, scheme.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_uri_scheme_handlers_with_launch_uri_type_async(&self, scheme: &HStringArg, launchQuerySupportType: LaunchQuerySupportType) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUriSchemeHandlersWithLaunchUriTypeAsync)(self as *const _ as *mut _, scheme.get(), launchQuerySupportType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_file_handlers_async(&self, extension: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindFileHandlersAsync)(self as *const _ as *mut _, extension.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherStatics3, 591552936, 40371, 18051, 170, 66, 220, 111, 81, 211, 56, 71);
RT_INTERFACE!{static interface ILauncherStatics3(ILauncherStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics3] {
    #[cfg(feature="windows-storage")] fn LaunchFolderAsync(&self, folder: *mut super::storage::IStorageFolder, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LaunchFolderWithOptionsAsync(&self, folder: *mut super::storage::IStorageFolder, options: *mut FolderLauncherOptions, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ILauncherStatics3 {
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_async(&self, folder: &super::storage::IStorageFolder) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderAsync)(self as *const _ as *mut _, folder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn launch_folder_with_options_async(&self, folder: &super::storage::IStorageFolder, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderWithOptionsAsync)(self as *const _ as *mut _, folder as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherStatics4, 3119284639, 46501, 16838, 179, 179, 221, 27, 49, 120, 188, 242);
RT_INTERFACE!{static interface ILauncherStatics4(ILauncherStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics4] {
    fn QueryAppUriSupportAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    fn QueryAppUriSupportWithPackageFamilyNameAsync(&self, uri: *mut foundation::Uri, packageFamilyName: HSTRING, out: *mut *mut foundation::IAsyncOperation<LaunchQuerySupportStatus>) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindAppUriHandlersAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>) -> HRESULT,
    fn LaunchUriForUserAsync(&self, user: *mut User, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithOptionsForUserAsync(&self, user: *mut User, uri: *mut foundation::Uri, options: *mut LauncherOptions, out: *mut *mut foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithDataForUserAsync(&self, user: *mut User, uri: *mut foundation::Uri, options: *mut LauncherOptions, inputData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<LaunchUriStatus>) -> HRESULT,
    fn LaunchUriForResultsForUserAsync(&self, user: *mut User, uri: *mut foundation::Uri, options: *mut LauncherOptions, out: *mut *mut foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT,
    fn LaunchUriForResultsWithDataForUserAsync(&self, user: *mut User, uri: *mut foundation::Uri, options: *mut LauncherOptions, inputData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<LaunchUriResult>) -> HRESULT
}}
impl ILauncherStatics4 {
    #[inline] pub fn query_app_uri_support_async(&self, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryAppUriSupportAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn query_app_uri_support_with_package_family_name_async(&self, uri: &foundation::Uri, packageFamilyName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LaunchQuerySupportStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).QueryAppUriSupportWithPackageFamilyNameAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_app_uri_handlers_async(&self, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<super::applicationmodel::AppInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAppUriHandlersAsync)(self as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_for_user_async(&self, user: &User, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_options_for_user_async(&self, user: &User, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_data_for_user_async(&self, user: &User, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_for_results_for_user_async(&self, user: &User, uri: &foundation::Uri, options: &LauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_for_results_with_data_for_user_async(&self, user: &User, uri: &foundation::Uri, options: &LauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<LaunchUriResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriForResultsWithDataForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherStatics5, 1529147268, 55445, 24554, 145, 83, 26, 196, 154, 237, 155, 169);
RT_INTERFACE!{static interface ILauncherStatics5(ILauncherStatics5Vtbl): IInspectable(IInspectableVtbl) [IID_ILauncherStatics5] {
    fn LaunchFolderPathAsync(&self, path: HSTRING, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchFolderPathWithOptionsAsync(&self, path: HSTRING, options: *mut FolderLauncherOptions, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchFolderPathForUserAsync(&self, user: *mut User, path: HSTRING, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn LaunchFolderPathWithOptionsForUserAsync(&self, user: *mut User, path: HSTRING, options: *mut FolderLauncherOptions, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ILauncherStatics5 {
    #[inline] pub fn launch_folder_path_async(&self, path: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderPathAsync)(self as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_folder_path_with_options_async(&self, path: &HStringArg, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderPathWithOptionsAsync)(self as *const _ as *mut _, path.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_folder_path_for_user_async(&self, user: &User, path: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderPathForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_folder_path_with_options_for_user_async(&self, user: &User, path: &HStringArg, options: &FolderLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchFolderPathWithOptionsForUserAsync)(self as *const _ as *mut _, user as *const _ as *mut _, path.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILauncherUIOptions, 455465582, 35494, 16873, 130, 81, 65, 101, 245, 152, 95, 73);
RT_INTERFACE!{interface ILauncherUIOptions(ILauncherUIOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherUIOptions] {
    fn get_InvocationPoint(&self, out: *mut *mut foundation::IReference<foundation::Point>) -> HRESULT,
    fn put_InvocationPoint(&self, value: *mut foundation::IReference<foundation::Point>) -> HRESULT,
    fn get_SelectionRect(&self, out: *mut *mut foundation::IReference<foundation::Rect>) -> HRESULT,
    fn put_SelectionRect(&self, value: *mut foundation::IReference<foundation::Rect>) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_PreferredPlacement(&self, out: *mut super::ui::popups::Placement) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_PreferredPlacement(&self, value: super::ui::popups::Placement) -> HRESULT
}}
impl ILauncherUIOptions {
    #[inline] pub fn get_invocation_point(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::Point>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InvocationPoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_invocation_point(&self, value: &foundation::IReference<foundation::Point>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InvocationPoint)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_selection_rect(&self) -> Result<Option<ComPtr<foundation::IReference<foundation::Rect>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SelectionRect)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_selection_rect(&self, value: &foundation::IReference<foundation::Rect>) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SelectionRect)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_preferred_placement(&self) -> Result<super::ui::popups::Placement> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PreferredPlacement)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_preferred_placement(&self, value: super::ui::popups::Placement) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PreferredPlacement)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LauncherUIOptions: ILauncherUIOptions}
DEFINE_IID!(IID_ILauncherViewOptions, 2325424625, 31911, 18910, 155, 211, 60, 91, 113, 132, 246, 22);
RT_INTERFACE!{interface ILauncherViewOptions(ILauncherViewOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILauncherViewOptions] {
    #[cfg(feature="windows-ui")] fn get_DesiredRemainingView(&self, out: *mut super::ui::viewmanagement::ViewSizePreference) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_DesiredRemainingView(&self, value: super::ui::viewmanagement::ViewSizePreference) -> HRESULT
}}
impl ILauncherViewOptions {
    #[cfg(feature="windows-ui")] #[inline] pub fn get_desired_remaining_view(&self) -> Result<super::ui::viewmanagement::ViewSizePreference> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredRemainingView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_desired_remaining_view(&self, value: super::ui::viewmanagement::ViewSizePreference) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DesiredRemainingView)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum LaunchFileStatus: i32 {
    Success = 0, AppUnavailable = 1, DeniedByPolicy = 2, FileTypeNotSupported = 3, Unknown = 4,
}}
RT_ENUM! { enum LaunchQuerySupportStatus: i32 {
    Available = 0, AppNotInstalled = 1, AppUnavailable = 2, NotSupported = 3, Unknown = 4,
}}
RT_ENUM! { enum LaunchQuerySupportType: i32 {
    Uri = 0, UriForResults = 1,
}}
DEFINE_IID!(IID_ILaunchUriResult, 3962022111, 63189, 17866, 145, 58, 112, 164, 12, 92, 130, 33);
RT_INTERFACE!{interface ILaunchUriResult(ILaunchUriResultVtbl): IInspectable(IInspectableVtbl) [IID_ILaunchUriResult] {
    fn get_Status(&self, out: *mut LaunchUriStatus) -> HRESULT,
    fn get_Result(&self, out: *mut *mut foundation::collections::ValueSet) -> HRESULT
}}
impl ILaunchUriResult {
    #[inline] pub fn get_status(&self) -> Result<LaunchUriStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_result(&self) -> Result<Option<ComPtr<foundation::collections::ValueSet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Result)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LaunchUriResult: ILaunchUriResult}
RT_ENUM! { enum LaunchUriStatus: i32 {
    Success = 0, AppUnavailable = 1, ProtocolUnavailable = 2, Unknown = 3,
}}
RT_CLASS!{static class MemoryManager}
impl RtActivatable<IMemoryManagerStatics> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics2> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics3> for MemoryManager {}
impl RtActivatable<IMemoryManagerStatics4> for MemoryManager {}
impl MemoryManager {
    #[inline] pub fn get_app_memory_usage() -> Result<u64> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage()
    }
    #[inline] pub fn get_app_memory_usage_limit() -> Result<u64> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage_limit()
    }
    #[inline] pub fn get_app_memory_usage_level() -> Result<AppMemoryUsageLevel> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().get_app_memory_usage_level()
    }
    #[inline] pub fn add_app_memory_usage_increased(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_increased(handler)
    }
    #[inline] pub fn remove_app_memory_usage_increased(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_increased(token)
    }
    #[inline] pub fn add_app_memory_usage_decreased(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_decreased(handler)
    }
    #[inline] pub fn remove_app_memory_usage_decreased(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_decreased(token)
    }
    #[inline] pub fn add_app_memory_usage_limit_changing(handler: &foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().add_app_memory_usage_limit_changing(handler)
    }
    #[inline] pub fn remove_app_memory_usage_limit_changing(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IMemoryManagerStatics>>::get_activation_factory().remove_app_memory_usage_limit_changing(token)
    }
    #[inline] pub fn get_app_memory_report() -> Result<Option<ComPtr<AppMemoryReport>>> {
        <Self as RtActivatable<IMemoryManagerStatics2>>::get_activation_factory().get_app_memory_report()
    }
    #[inline] pub fn get_process_memory_report() -> Result<Option<ComPtr<ProcessMemoryReport>>> {
        <Self as RtActivatable<IMemoryManagerStatics2>>::get_activation_factory().get_process_memory_report()
    }
    #[inline] pub fn try_set_app_memory_usage_limit(value: u64) -> Result<bool> {
        <Self as RtActivatable<IMemoryManagerStatics3>>::get_activation_factory().try_set_app_memory_usage_limit(value)
    }
    #[inline] pub fn get_expected_app_memory_usage_limit() -> Result<u64> {
        <Self as RtActivatable<IMemoryManagerStatics4>>::get_activation_factory().get_expected_app_memory_usage_limit()
    }
}
DEFINE_CLSID!(MemoryManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,77,101,109,111,114,121,77,97,110,97,103,101,114,0]) [CLSID_MemoryManager]);
DEFINE_IID!(IID_IMemoryManagerStatics, 1550591900, 55242, 18297, 145, 136, 64, 87, 33, 156, 230, 76);
RT_INTERFACE!{static interface IMemoryManagerStatics(IMemoryManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics] {
    fn get_AppMemoryUsage(&self, out: *mut u64) -> HRESULT,
    fn get_AppMemoryUsageLimit(&self, out: *mut u64) -> HRESULT,
    fn get_AppMemoryUsageLevel(&self, out: *mut AppMemoryUsageLevel) -> HRESULT,
    fn add_AppMemoryUsageIncreased(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageIncreased(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AppMemoryUsageDecreased(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageDecreased(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AppMemoryUsageLimitChanging(&self, handler: *mut foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AppMemoryUsageLimitChanging(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IMemoryManagerStatics {
    #[inline] pub fn get_app_memory_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_memory_usage_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_memory_usage_level(&self) -> Result<AppMemoryUsageLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppMemoryUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_app_memory_usage_increased(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_app_memory_usage_increased(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_app_memory_usage_decreased(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageDecreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_app_memory_usage_decreased(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageDecreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_app_memory_usage_limit_changing(&self, handler: &foundation::EventHandler<AppMemoryUsageLimitChangingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AppMemoryUsageLimitChanging)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_app_memory_usage_limit_changing(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AppMemoryUsageLimitChanging)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMemoryManagerStatics2, 1861104927, 28002, 16959, 148, 121, 176, 31, 156, 159, 118, 105);
RT_INTERFACE!{static interface IMemoryManagerStatics2(IMemoryManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics2] {
    fn GetAppMemoryReport(&self, out: *mut *mut AppMemoryReport) -> HRESULT,
    fn GetProcessMemoryReport(&self, out: *mut *mut ProcessMemoryReport) -> HRESULT
}}
impl IMemoryManagerStatics2 {
    #[inline] pub fn get_app_memory_report(&self) -> Result<Option<ComPtr<AppMemoryReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_process_memory_report(&self) -> Result<Option<ComPtr<ProcessMemoryReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetProcessMemoryReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMemoryManagerStatics3, 345725390, 37549, 20021, 137, 235, 80, 223, 180, 192, 217, 28);
RT_INTERFACE!{static interface IMemoryManagerStatics3(IMemoryManagerStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics3] {
    fn TrySetAppMemoryUsageLimit(&self, value: u64, out: *mut bool) -> HRESULT
}}
impl IMemoryManagerStatics3 {
    #[inline] pub fn try_set_app_memory_usage_limit(&self, value: u64) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetAppMemoryUsageLimit)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMemoryManagerStatics4, 3316205608, 59470, 18566, 138, 13, 68, 179, 25, 14, 59, 114);
RT_INTERFACE!{static interface IMemoryManagerStatics4(IMemoryManagerStatics4Vtbl): IInspectable(IInspectableVtbl) [IID_IMemoryManagerStatics4] {
    fn get_ExpectedAppMemoryUsageLimit(&self, out: *mut u64) -> HRESULT
}}
impl IMemoryManagerStatics4 {
    #[inline] pub fn get_expected_app_memory_usage_limit(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExpectedAppMemoryUsageLimit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum PowerState: i32 {
    ConnectedStandby = 0, SleepS3 = 1,
}}
RT_CLASS!{static class ProcessLauncher}
impl RtActivatable<IProcessLauncherStatics> for ProcessLauncher {}
impl ProcessLauncher {
    #[inline] pub fn run_to_completion_async(fileName: &HStringArg, args: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ProcessLauncherResult>>> {
        <Self as RtActivatable<IProcessLauncherStatics>>::get_activation_factory().run_to_completion_async(fileName, args)
    }
    #[inline] pub fn run_to_completion_async_with_options(fileName: &HStringArg, args: &HStringArg, options: &ProcessLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<ProcessLauncherResult>>> {
        <Self as RtActivatable<IProcessLauncherStatics>>::get_activation_factory().run_to_completion_async_with_options(fileName, args, options)
    }
}
DEFINE_CLSID!(ProcessLauncher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,99,101,115,115,76,97,117,110,99,104,101,114,0]) [CLSID_ProcessLauncher]);
DEFINE_IID!(IID_IProcessLauncherOptions, 813742543, 62532, 19075, 190, 175, 165, 73, 160, 243, 34, 156);
RT_INTERFACE!{interface IProcessLauncherOptions(IProcessLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherOptions] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardInput(&self, out: *mut *mut super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardInput(&self, value: *mut super::storage::streams::IInputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardOutput(&self, out: *mut *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardOutput(&self, value: *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_StandardError(&self, out: *mut *mut super::storage::streams::IOutputStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_StandardError(&self, value: *mut super::storage::streams::IOutputStream) -> HRESULT,
    fn get_WorkingDirectory(&self, out: *mut HSTRING) -> HRESULT,
    fn put_WorkingDirectory(&self, value: HSTRING) -> HRESULT
}}
impl IProcessLauncherOptions {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_standard_input(&self) -> Result<Option<ComPtr<super::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardInput)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_standard_input(&self, value: &super::storage::streams::IInputStream) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_StandardInput)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_standard_output(&self) -> Result<Option<ComPtr<super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardOutput)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_standard_output(&self, value: &super::storage::streams::IOutputStream) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_StandardOutput)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_standard_error(&self) -> Result<Option<ComPtr<super::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StandardError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_standard_error(&self, value: &super::storage::streams::IOutputStream) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_StandardError)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_working_directory(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WorkingDirectory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_working_directory(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WorkingDirectory)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessLauncherOptions: IProcessLauncherOptions}
impl RtActivatable<IActivationFactory> for ProcessLauncherOptions {}
DEFINE_CLSID!(ProcessLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,99,101,115,115,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_ProcessLauncherOptions]);
DEFINE_IID!(IID_IProcessLauncherResult, 1414302004, 34520, 18833, 142, 117, 236, 232, 164, 59, 107, 109);
RT_INTERFACE!{interface IProcessLauncherResult(IProcessLauncherResultVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherResult] {
    fn get_ExitCode(&self, out: *mut u32) -> HRESULT
}}
impl IProcessLauncherResult {
    #[inline] pub fn get_exit_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExitCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessLauncherResult: IProcessLauncherResult}
DEFINE_IID!(IID_IProcessLauncherStatics, 866871015, 11534, 17547, 166, 160, 193, 60, 56, 54, 208, 156);
RT_INTERFACE!{static interface IProcessLauncherStatics(IProcessLauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessLauncherStatics] {
    fn RunToCompletionAsync(&self, fileName: HSTRING, args: HSTRING, out: *mut *mut foundation::IAsyncOperation<ProcessLauncherResult>) -> HRESULT,
    fn RunToCompletionAsyncWithOptions(&self, fileName: HSTRING, args: HSTRING, options: *mut ProcessLauncherOptions, out: *mut *mut foundation::IAsyncOperation<ProcessLauncherResult>) -> HRESULT
}}
impl IProcessLauncherStatics {
    #[inline] pub fn run_to_completion_async(&self, fileName: &HStringArg, args: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<ProcessLauncherResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunToCompletionAsync)(self as *const _ as *mut _, fileName.get(), args.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn run_to_completion_async_with_options(&self, fileName: &HStringArg, args: &HStringArg, options: &ProcessLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<ProcessLauncherResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunToCompletionAsyncWithOptions)(self as *const _ as *mut _, fileName.get(), args.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProcessMemoryReport, 141755816, 39792, 18306, 135, 65, 58, 152, 43, 108, 229, 228);
RT_INTERFACE!{interface IProcessMemoryReport(IProcessMemoryReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryReport] {
    fn get_PrivateWorkingSetUsage(&self, out: *mut u64) -> HRESULT,
    fn get_TotalWorkingSetUsage(&self, out: *mut u64) -> HRESULT
}}
impl IProcessMemoryReport {
    #[inline] pub fn get_private_working_set_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivateWorkingSetUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_total_working_set_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalWorkingSetUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessMemoryReport: IProcessMemoryReport}
RT_ENUM! { enum ProcessorArchitecture: i32 {
    X86 = 0, Arm = 5, X64 = 9, Neutral = 11, Unknown = 65535,
}}
DEFINE_IID!(IID_IProtocolForResultsOperation, 3582011706, 28137, 19752, 147, 120, 248, 103, 130, 225, 130, 187);
RT_INTERFACE!{interface IProtocolForResultsOperation(IProtocolForResultsOperationVtbl): IInspectable(IInspectableVtbl) [IID_IProtocolForResultsOperation] {
    fn ReportCompleted(&self, data: *mut foundation::collections::ValueSet) -> HRESULT
}}
impl IProtocolForResultsOperation {
    #[inline] pub fn report_completed(&self, data: &foundation::collections::ValueSet) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ReportCompleted)(self as *const _ as *mut _, data as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtocolForResultsOperation: IProtocolForResultsOperation}
RT_CLASS!{static class RemoteLauncher}
impl RtActivatable<IRemoteLauncherStatics> for RemoteLauncher {}
impl RemoteLauncher {
    #[inline] pub fn launch_uri_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_async(remoteSystemConnectionRequest, uri)
    }
    #[inline] pub fn launch_uri_with_options_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri, options: &RemoteLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_with_options_async(remoteSystemConnectionRequest, uri, options)
    }
    #[inline] pub fn launch_uri_with_data_async(remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri, options: &RemoteLauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> {
        <Self as RtActivatable<IRemoteLauncherStatics>>::get_activation_factory().launch_uri_with_data_async(remoteSystemConnectionRequest, uri, options, inputData)
    }
}
DEFINE_CLSID!(RemoteLauncher(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,76,97,117,110,99,104,101,114,0]) [CLSID_RemoteLauncher]);
DEFINE_IID!(IID_IRemoteLauncherOptions, 2654611336, 10385, 19679, 162, 214, 157, 255, 125, 2, 230, 147);
RT_INTERFACE!{interface IRemoteLauncherOptions(IRemoteLauncherOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteLauncherOptions] {
    fn get_FallbackUri(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn put_FallbackUri(&self, value: *mut foundation::Uri) -> HRESULT,
    fn get_PreferredAppIds(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT
}}
impl IRemoteLauncherOptions {
    #[inline] pub fn get_fallback_uri(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FallbackUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_fallback_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_FallbackUri)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_preferred_app_ids(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PreferredAppIds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteLauncherOptions: IRemoteLauncherOptions}
impl RtActivatable<IActivationFactory> for RemoteLauncherOptions {}
DEFINE_CLSID!(RemoteLauncherOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,76,97,117,110,99,104,101,114,79,112,116,105,111,110,115,0]) [CLSID_RemoteLauncherOptions]);
DEFINE_IID!(IID_IRemoteLauncherStatics, 3621485203, 41740, 18615, 159, 33, 5, 16, 38, 164, 229, 23);
RT_INTERFACE!{static interface IRemoteLauncherStatics(IRemoteLauncherStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteLauncherStatics] {
    fn LaunchUriAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithOptionsAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut foundation::Uri, options: *mut RemoteLauncherOptions, out: *mut *mut foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT,
    fn LaunchUriWithDataAsync(&self, remoteSystemConnectionRequest: *mut remotesystems::RemoteSystemConnectionRequest, uri: *mut foundation::Uri, options: *mut RemoteLauncherOptions, inputData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<RemoteLaunchUriStatus>) -> HRESULT
}}
impl IRemoteLauncherStatics {
    #[inline] pub fn launch_uri_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_options_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri, options: &RemoteLauncherOptions) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithOptionsAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn launch_uri_with_data_async(&self, remoteSystemConnectionRequest: &remotesystems::RemoteSystemConnectionRequest, uri: &foundation::Uri, options: &RemoteLauncherOptions, inputData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<RemoteLaunchUriStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LaunchUriWithDataAsync)(self as *const _ as *mut _, remoteSystemConnectionRequest as *const _ as *mut _, uri as *const _ as *mut _, options as *const _ as *mut _, inputData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteLaunchUriStatus: i32 {
    Unknown = 0, Success = 1, AppUnavailable = 2, ProtocolUnavailable = 3, RemoteSystemUnavailable = 4, ValueSetTooLarge = 5, DeniedByLocalSystem = 6, DeniedByRemoteSystem = 7,
}}
RT_ENUM! { enum ShutdownKind: i32 {
    Shutdown = 0, Restart = 1,
}}
RT_CLASS!{static class ShutdownManager}
impl RtActivatable<IShutdownManagerStatics> for ShutdownManager {}
impl RtActivatable<IShutdownManagerStatics2> for ShutdownManager {}
impl ShutdownManager {
    #[inline] pub fn begin_shutdown(shutdownKind: ShutdownKind, timeout: foundation::TimeSpan) -> Result<()> {
        <Self as RtActivatable<IShutdownManagerStatics>>::get_activation_factory().begin_shutdown(shutdownKind, timeout)
    }
    #[inline] pub fn cancel_shutdown() -> Result<()> {
        <Self as RtActivatable<IShutdownManagerStatics>>::get_activation_factory().cancel_shutdown()
    }
    #[inline] pub fn is_power_state_supported(powerState: PowerState) -> Result<bool> {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().is_power_state_supported(powerState)
    }
    #[inline] pub fn enter_power_state(powerState: PowerState) -> Result<()> {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().enter_power_state(powerState)
    }
    #[inline] pub fn enter_power_state_with_time_span(powerState: PowerState, wakeUpAfter: foundation::TimeSpan) -> Result<()> {
        <Self as RtActivatable<IShutdownManagerStatics2>>::get_activation_factory().enter_power_state_with_time_span(powerState, wakeUpAfter)
    }
}
DEFINE_CLSID!(ShutdownManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,83,104,117,116,100,111,119,110,77,97,110,97,103,101,114,0]) [CLSID_ShutdownManager]);
DEFINE_IID!(IID_IShutdownManagerStatics, 1927432173, 56667, 19820, 177, 208, 197, 122, 123, 187, 95, 148);
RT_INTERFACE!{static interface IShutdownManagerStatics(IShutdownManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IShutdownManagerStatics] {
    fn BeginShutdown(&self, shutdownKind: ShutdownKind, timeout: foundation::TimeSpan) -> HRESULT,
    fn CancelShutdown(&self) -> HRESULT
}}
impl IShutdownManagerStatics {
    #[inline] pub fn begin_shutdown(&self, shutdownKind: ShutdownKind, timeout: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).BeginShutdown)(self as *const _ as *mut _, shutdownKind, timeout);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn cancel_shutdown(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CancelShutdown)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IShutdownManagerStatics2, 258580527, 39988, 17351, 168, 195, 112, 179, 10, 127, 117, 4);
RT_INTERFACE!{static interface IShutdownManagerStatics2(IShutdownManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IShutdownManagerStatics2] {
    fn IsPowerStateSupported(&self, powerState: PowerState, out: *mut bool) -> HRESULT,
    fn EnterPowerState(&self, powerState: PowerState) -> HRESULT,
    fn EnterPowerStateWithTimeSpan(&self, powerState: PowerState, wakeUpAfter: foundation::TimeSpan) -> HRESULT
}}
impl IShutdownManagerStatics2 {
    #[inline] pub fn is_power_state_supported(&self, powerState: PowerState) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsPowerStateSupported)(self as *const _ as *mut _, powerState, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn enter_power_state(&self, powerState: PowerState) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnterPowerState)(self as *const _ as *mut _, powerState);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn enter_power_state_with_time_span(&self, powerState: PowerState, wakeUpAfter: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).EnterPowerStateWithTimeSpan)(self as *const _ as *mut _, powerState, wakeUpAfter);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class TimeZoneSettings}
impl RtActivatable<ITimeZoneSettingsStatics> for TimeZoneSettings {}
impl RtActivatable<ITimeZoneSettingsStatics2> for TimeZoneSettings {}
impl TimeZoneSettings {
    #[inline] pub fn get_current_time_zone_display_name() -> Result<HString> {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_current_time_zone_display_name()
    }
    #[inline] pub fn get_supported_time_zone_display_names() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_supported_time_zone_display_names()
    }
    #[inline] pub fn get_can_change_time_zone() -> Result<bool> {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().get_can_change_time_zone()
    }
    #[inline] pub fn change_time_zone_by_display_name(timeZoneDisplayName: &HStringArg) -> Result<()> {
        <Self as RtActivatable<ITimeZoneSettingsStatics>>::get_activation_factory().change_time_zone_by_display_name(timeZoneDisplayName)
    }
    #[inline] pub fn auto_update_time_zone_async(timeout: foundation::TimeSpan) -> Result<ComPtr<foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>> {
        <Self as RtActivatable<ITimeZoneSettingsStatics2>>::get_activation_factory().auto_update_time_zone_async(timeout)
    }
}
DEFINE_CLSID!(TimeZoneSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,105,109,101,90,111,110,101,83,101,116,116,105,110,103,115,0]) [CLSID_TimeZoneSettings]);
DEFINE_IID!(IID_ITimeZoneSettingsStatics, 2604346346, 41217, 16814, 159, 189, 2, 135, 40, 186, 183, 61);
RT_INTERFACE!{static interface ITimeZoneSettingsStatics(ITimeZoneSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITimeZoneSettingsStatics] {
    fn get_CurrentTimeZoneDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SupportedTimeZoneDisplayNames(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_CanChangeTimeZone(&self, out: *mut bool) -> HRESULT,
    fn ChangeTimeZoneByDisplayName(&self, timeZoneDisplayName: HSTRING) -> HRESULT
}}
impl ITimeZoneSettingsStatics {
    #[inline] pub fn get_current_time_zone_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentTimeZoneDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_time_zone_display_names(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportedTimeZoneDisplayNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_can_change_time_zone(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanChangeTimeZone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn change_time_zone_by_display_name(&self, timeZoneDisplayName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ChangeTimeZoneByDisplayName)(self as *const _ as *mut _, timeZoneDisplayName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITimeZoneSettingsStatics2, 1432096184, 14760, 18938, 180, 246, 162, 199, 252, 40, 66, 236);
RT_INTERFACE!{static interface ITimeZoneSettingsStatics2(ITimeZoneSettingsStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ITimeZoneSettingsStatics2] {
    fn AutoUpdateTimeZoneAsync(&self, timeout: foundation::TimeSpan, out: *mut *mut foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>) -> HRESULT
}}
impl ITimeZoneSettingsStatics2 {
    #[inline] pub fn auto_update_time_zone_async(&self, timeout: foundation::TimeSpan) -> Result<ComPtr<foundation::IAsyncOperation<AutoUpdateTimeZoneStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AutoUpdateTimeZoneAsync)(self as *const _ as *mut _, timeout, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUser, 3751421638, 59206, 19405, 181, 212, 18, 1, 3, 196, 32, 155);
RT_INTERFACE!{interface IUser(IUserVtbl): IInspectable(IInspectableVtbl) [IID_IUser] {
    fn get_NonRoamableId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AuthenticationStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT,
    fn get_Type(&self, out: *mut UserType) -> HRESULT,
    fn GetPropertyAsync(&self, value: HSTRING, out: *mut *mut foundation::IAsyncOperation<IInspectable>) -> HRESULT,
    fn GetPropertiesAsync(&self, values: *mut foundation::collections::IVectorView<HString>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IPropertySet>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetPictureAsync(&self, desiredSize: UserPictureSize, out: *mut *mut foundation::IAsyncOperation<super::storage::streams::IRandomAccessStreamReference>) -> HRESULT
}}
impl IUser {
    #[inline] pub fn get_non_roamable_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NonRoamableId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_status(&self) -> Result<UserAuthenticationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AuthenticationStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<UserType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_property_async(&self, value: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertyAsync)(self as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties_async(&self, values: &foundation::collections::IVectorView<HString>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IPropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertiesAsync)(self as *const _ as *mut _, values as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_picture_async(&self, desiredSize: UserPictureSize) -> Result<ComPtr<foundation::IAsyncOperation<super::storage::streams::IRandomAccessStreamReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPictureAsync)(self as *const _ as *mut _, desiredSize, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class User: IUser}
impl RtActivatable<IUserStatics> for User {}
impl User {
    #[inline] pub fn create_watcher() -> Result<Option<ComPtr<UserWatcher>>> {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().create_watcher()
    }
    #[inline] pub fn find_all_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async()
    }
    #[inline] pub fn find_all_async_by_type(type_: UserType) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async_by_type(type_)
    }
    #[inline] pub fn find_all_async_by_type_and_status(type_: UserType, status: UserAuthenticationStatus) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().find_all_async_by_type_and_status(type_, status)
    }
    #[inline] pub fn get_from_id(nonRoamableId: &HStringArg) -> Result<Option<ComPtr<User>>> {
        <Self as RtActivatable<IUserStatics>>::get_activation_factory().get_from_id(nonRoamableId)
    }
}
DEFINE_CLSID!(User(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,0]) [CLSID_User]);
RT_ENUM! { enum UserAuthenticationStatus: i32 {
    Unauthenticated = 0, LocallyAuthenticated = 1, RemotelyAuthenticated = 2,
}}
DEFINE_IID!(IID_IUserAuthenticationStatusChangeDeferral, 2293601640, 47920, 17147, 162, 112, 233, 144, 46, 64, 239, 167);
RT_INTERFACE!{interface IUserAuthenticationStatusChangeDeferral(IUserAuthenticationStatusChangeDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IUserAuthenticationStatusChangeDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IUserAuthenticationStatusChangeDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class UserAuthenticationStatusChangeDeferral: IUserAuthenticationStatusChangeDeferral}
DEFINE_IID!(IID_IUserAuthenticationStatusChangingEventArgs, 2349010728, 42769, 19486, 171, 72, 4, 23, 156, 21, 147, 143);
RT_INTERFACE!{interface IUserAuthenticationStatusChangingEventArgs(IUserAuthenticationStatusChangingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserAuthenticationStatusChangingEventArgs] {
    fn GetDeferral(&self, out: *mut *mut UserAuthenticationStatusChangeDeferral) -> HRESULT,
    fn get_User(&self, out: *mut *mut User) -> HRESULT,
    fn get_NewStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT,
    fn get_CurrentStatus(&self, out: *mut UserAuthenticationStatus) -> HRESULT
}}
impl IUserAuthenticationStatusChangingEventArgs {
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<UserAuthenticationStatusChangeDeferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_new_status(&self) -> Result<UserAuthenticationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NewStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_status(&self) -> Result<UserAuthenticationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class UserAuthenticationStatusChangingEventArgs: IUserAuthenticationStatusChangingEventArgs}
DEFINE_IID!(IID_IUserChangedEventArgs, 140794332, 6342, 18651, 188, 153, 114, 79, 185, 32, 60, 204);
RT_INTERFACE!{interface IUserChangedEventArgs(IUserChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserChangedEventArgs] {
    fn get_User(&self, out: *mut *mut User) -> HRESULT
}}
impl IUserChangedEventArgs {
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserChangedEventArgs: IUserChangedEventArgs}
RT_CLASS!{static class UserDeviceAssociation}
impl RtActivatable<IUserDeviceAssociationStatics> for UserDeviceAssociation {}
impl UserDeviceAssociation {
    #[inline] pub fn find_user_from_device_id(deviceId: &HStringArg) -> Result<Option<ComPtr<User>>> {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().find_user_from_device_id(deviceId)
    }
    #[inline] pub fn add_user_device_association_changed(handler: &foundation::EventHandler<UserDeviceAssociationChangedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().add_user_device_association_changed(handler)
    }
    #[inline] pub fn remove_user_device_association_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IUserDeviceAssociationStatics>>::get_activation_factory().remove_user_device_association_changed(token)
    }
}
DEFINE_CLSID!(UserDeviceAssociation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,68,101,118,105,99,101,65,115,115,111,99,105,97,116,105,111,110,0]) [CLSID_UserDeviceAssociation]);
DEFINE_IID!(IID_IUserDeviceAssociationChangedEventArgs, 3172953964, 47965, 19835, 165, 240, 200, 205, 17, 163, 141, 66);
RT_INTERFACE!{interface IUserDeviceAssociationChangedEventArgs(IUserDeviceAssociationChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDeviceAssociationChangedEventArgs] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NewUser(&self, out: *mut *mut User) -> HRESULT,
    fn get_OldUser(&self, out: *mut *mut User) -> HRESULT
}}
impl IUserDeviceAssociationChangedEventArgs {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_new_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NewUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_old_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OldUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserDeviceAssociationChangedEventArgs: IUserDeviceAssociationChangedEventArgs}
DEFINE_IID!(IID_IUserDeviceAssociationStatics, 2118721044, 63578, 19463, 141, 169, 127, 227, 208, 84, 35, 67);
RT_INTERFACE!{static interface IUserDeviceAssociationStatics(IUserDeviceAssociationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserDeviceAssociationStatics] {
    fn FindUserFromDeviceId(&self, deviceId: HSTRING, out: *mut *mut User) -> HRESULT,
    fn add_UserDeviceAssociationChanged(&self, handler: *mut foundation::EventHandler<UserDeviceAssociationChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserDeviceAssociationChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IUserDeviceAssociationStatics {
    #[inline] pub fn find_user_from_device_id(&self, deviceId: &HStringArg) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindUserFromDeviceId)(self as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_user_device_association_changed(&self, handler: &foundation::EventHandler<UserDeviceAssociationChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserDeviceAssociationChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_device_association_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_UserDeviceAssociationChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUserPicker, 2102689800, 61923, 19052, 141, 220, 169, 187, 15, 72, 138, 237);
RT_INTERFACE!{interface IUserPicker(IUserPickerVtbl): IInspectable(IInspectableVtbl) [IID_IUserPicker] {
    fn get_AllowGuestAccounts(&self, out: *mut bool) -> HRESULT,
    fn put_AllowGuestAccounts(&self, value: bool) -> HRESULT,
    fn get_SuggestedSelectedUser(&self, out: *mut *mut User) -> HRESULT,
    fn put_SuggestedSelectedUser(&self, value: *mut User) -> HRESULT,
    fn PickSingleUserAsync(&self, out: *mut *mut foundation::IAsyncOperation<User>) -> HRESULT
}}
impl IUserPicker {
    #[inline] pub fn get_allow_guest_accounts(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowGuestAccounts)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_guest_accounts(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowGuestAccounts)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_selected_user(&self) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SuggestedSelectedUser)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_selected_user(&self, value: &User) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_SuggestedSelectedUser)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn pick_single_user_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleUserAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserPicker: IUserPicker}
impl RtActivatable<IUserPickerStatics> for UserPicker {}
impl RtActivatable<IActivationFactory> for UserPicker {}
impl UserPicker {
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IUserPickerStatics>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(UserPicker(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,105,99,107,101,114,0]) [CLSID_UserPicker]);
DEFINE_IID!(IID_IUserPickerStatics, 3727855836, 32371, 19958, 161, 174, 77, 126, 202, 130, 180, 13);
RT_INTERFACE!{static interface IUserPickerStatics(IUserPickerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserPickerStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IUserPickerStatics {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum UserPictureSize: i32 {
    Size64x64 = 0, Size208x208 = 1, Size424x424 = 2, Size1080x1080 = 3,
}}
DEFINE_IID!(IID_IUserStatics, 358527547, 9258, 17888, 162, 233, 49, 113, 252, 106, 127, 221);
RT_INTERFACE!{static interface IUserStatics(IUserStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserStatics] {
    fn CreateWatcher(&self, out: *mut *mut UserWatcher) -> HRESULT,
    fn FindAllAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<User>>) -> HRESULT,
    fn FindAllAsyncByType(&self, type_: UserType, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<User>>) -> HRESULT,
    fn FindAllAsyncByTypeAndStatus(&self, type_: UserType, status: UserAuthenticationStatus, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<User>>) -> HRESULT,
    fn GetFromId(&self, nonRoamableId: HSTRING, out: *mut *mut User) -> HRESULT
}}
impl IUserStatics {
    #[inline] pub fn create_watcher(&self) -> Result<Option<ComPtr<UserWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_async_by_type(&self, type_: UserType) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsyncByType)(self as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_async_by_type_and_status(&self, type_: UserType, status: UserAuthenticationStatus) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<User>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindAllAsyncByTypeAndStatus)(self as *const _ as *mut _, type_, status, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_from_id(&self, nonRoamableId: &HStringArg) -> Result<Option<ComPtr<User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFromId)(self as *const _ as *mut _, nonRoamableId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum UserType: i32 {
    LocalUser = 0, RemoteUser = 1, LocalGuest = 2, RemoteGuest = 3,
}}
DEFINE_IID!(IID_IUserWatcher, 358527547, 9258, 17888, 162, 233, 49, 113, 252, 106, 127, 187);
RT_INTERFACE!{interface IUserWatcher(IUserWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IUserWatcher] {
    fn get_Status(&self, out: *mut UserWatcherStatus) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AuthenticationStatusChanged(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_AuthenticationStatusChanging(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStatusChanging(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Stopped(&self, handler: *mut foundation::TypedEventHandler<UserWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Stopped(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IUserWatcher {
    #[inline] pub fn get_status(&self) -> Result<UserWatcherStatus> { unsafe { 
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
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_authentication_status_changed(&self, handler: &foundation::TypedEventHandler<UserWatcher, UserChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AuthenticationStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_authentication_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AuthenticationStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_authentication_status_changing(&self, handler: &foundation::TypedEventHandler<UserWatcher, UserAuthenticationStatusChangingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AuthenticationStatusChanging)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_authentication_status_changing(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AuthenticationStatusChanging)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<UserWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_stopped(&self, handler: &foundation::TypedEventHandler<UserWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Stopped)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stopped(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Stopped)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class UserWatcher: IUserWatcher}
RT_ENUM! { enum UserWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
RT_ENUM! { enum VirtualKey: i32 {
    None = 0, LeftButton = 1, RightButton = 2, Cancel = 3, MiddleButton = 4, XButton1 = 5, XButton2 = 6, Back = 8, Tab = 9, Clear = 12, Enter = 13, Shift = 16, Control = 17, Menu = 18, Pause = 19, CapitalLock = 20, Kana = 21, Hangul = 21, Junja = 23, Final = 24, Hanja = 25, Kanji = 25, Escape = 27, Convert = 28, NonConvert = 29, Accept = 30, ModeChange = 31, Space = 32, PageUp = 33, PageDown = 34, End = 35, Home = 36, Left = 37, Up = 38, Right = 39, Down = 40, Select = 41, Print = 42, Execute = 43, Snapshot = 44, Insert = 45, Delete = 46, Help = 47, Number0 = 48, Number1 = 49, Number2 = 50, Number3 = 51, Number4 = 52, Number5 = 53, Number6 = 54, Number7 = 55, Number8 = 56, Number9 = 57, A = 65, B = 66, C = 67, D = 68, E = 69, F = 70, G = 71, H = 72, I = 73, J = 74, K = 75, L = 76, M = 77, N = 78, O = 79, P = 80, Q = 81, R = 82, S = 83, T = 84, U = 85, V = 86, W = 87, X = 88, Y = 89, Z = 90, LeftWindows = 91, RightWindows = 92, Application = 93, Sleep = 95, NumberPad0 = 96, NumberPad1 = 97, NumberPad2 = 98, NumberPad3 = 99, NumberPad4 = 100, NumberPad5 = 101, NumberPad6 = 102, NumberPad7 = 103, NumberPad8 = 104, NumberPad9 = 105, Multiply = 106, Add = 107, Separator = 108, Subtract = 109, Decimal = 110, Divide = 111, F1 = 112, F2 = 113, F3 = 114, F4 = 115, F5 = 116, F6 = 117, F7 = 118, F8 = 119, F9 = 120, F10 = 121, F11 = 122, F12 = 123, F13 = 124, F14 = 125, F15 = 126, F16 = 127, F17 = 128, F18 = 129, F19 = 130, F20 = 131, F21 = 132, F22 = 133, F23 = 134, F24 = 135, NavigationView = 136, NavigationMenu = 137, NavigationUp = 138, NavigationDown = 139, NavigationLeft = 140, NavigationRight = 141, NavigationAccept = 142, NavigationCancel = 143, NumberKeyLock = 144, Scroll = 145, LeftShift = 160, RightShift = 161, LeftControl = 162, RightControl = 163, LeftMenu = 164, RightMenu = 165, GoBack = 166, GoForward = 167, Refresh = 168, Stop = 169, Search = 170, Favorites = 171, GoHome = 172, GamepadA = 195, GamepadB = 196, GamepadX = 197, GamepadY = 198, GamepadRightShoulder = 199, GamepadLeftShoulder = 200, GamepadLeftTrigger = 201, GamepadRightTrigger = 202, GamepadDPadUp = 203, GamepadDPadDown = 204, GamepadDPadLeft = 205, GamepadDPadRight = 206, GamepadMenu = 207, GamepadView = 208, GamepadLeftThumbstickButton = 209, GamepadRightThumbstickButton = 210, GamepadLeftThumbstickUp = 211, GamepadLeftThumbstickDown = 212, GamepadLeftThumbstickRight = 213, GamepadLeftThumbstickLeft = 214, GamepadRightThumbstickUp = 215, GamepadRightThumbstickDown = 216, GamepadRightThumbstickRight = 217, GamepadRightThumbstickLeft = 218,
}}
RT_ENUM! { enum VirtualKeyModifiers: u32 {
    None = 0, Control = 1, Menu = 2, Shift = 4, Windows = 8,
}}
pub mod diagnostics { // Windows.System.Diagnostics
use crate::prelude::*;
DEFINE_IID!(IID_IDiagnosticActionResult, 3261440662, 59195, 16535, 178, 143, 52, 66, 240, 61, 216, 49);
RT_INTERFACE!{interface IDiagnosticActionResult(IDiagnosticActionResultVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticActionResult] {
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_Results(&self, out: *mut *mut foundation::collections::ValueSet) -> HRESULT
}}
impl IDiagnosticActionResult {
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_results(&self) -> Result<Option<ComPtr<foundation::collections::ValueSet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Results)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DiagnosticActionResult: IDiagnosticActionResult}
RT_ENUM! { enum DiagnosticActionState: i32 {
    Initializing = 0, Downloading = 1, VerifyingTrust = 2, Detecting = 3, Resolving = 4, VerifyingResolution = 5,
}}
DEFINE_IID!(IID_IDiagnosticInvoker, 410724106, 739, 20358, 132, 252, 253, 216, 146, 181, 148, 15);
RT_INTERFACE!{interface IDiagnosticInvoker(IDiagnosticInvokerVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticInvoker] {
    #[cfg(feature="windows-data")] fn RunDiagnosticActionAsync(&self, context: *mut super::super::data::json::JsonObject, out: *mut *mut foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>) -> HRESULT
}}
impl IDiagnosticInvoker {
    #[cfg(feature="windows-data")] #[inline] pub fn run_diagnostic_action_async(&self, context: &super::super::data::json::JsonObject) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunDiagnosticActionAsync)(self as *const _ as *mut _, context as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DiagnosticInvoker: IDiagnosticInvoker}
impl RtActivatable<IDiagnosticInvokerStatics> for DiagnosticInvoker {}
impl DiagnosticInvoker {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<DiagnosticInvoker>>> {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<DiagnosticInvoker>>> {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_for_user(user)
    }
    #[inline] pub fn get_is_supported() -> Result<bool> {
        <Self as RtActivatable<IDiagnosticInvokerStatics>>::get_activation_factory().get_is_supported()
    }
}
DEFINE_CLSID!(DiagnosticInvoker(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,68,105,97,103,110,111,115,116,105,99,73,110,118,111,107,101,114,0]) [CLSID_DiagnosticInvoker]);
DEFINE_IID!(IID_IDiagnosticInvoker2, 3820983388, 5466, 19282, 168, 236, 7, 12, 68, 249, 80, 0);
RT_INTERFACE!{interface IDiagnosticInvoker2(IDiagnosticInvoker2Vtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticInvoker2] {
    fn RunDiagnosticActionFromStringAsync(&self, context: HSTRING, out: *mut *mut foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>) -> HRESULT
}}
impl IDiagnosticInvoker2 {
    #[inline] pub fn run_diagnostic_action_from_string_async(&self, context: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunDiagnosticActionFromStringAsync)(self as *const _ as *mut _, context.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDiagnosticInvokerStatics, 1559943390, 61788, 17748, 168, 19, 193, 19, 195, 136, 27, 9);
RT_INTERFACE!{static interface IDiagnosticInvokerStatics(IDiagnosticInvokerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticInvokerStatics] {
    fn GetDefault(&self, out: *mut *mut DiagnosticInvoker) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut DiagnosticInvoker) -> HRESULT,
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IDiagnosticInvokerStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<DiagnosticInvoker>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<DiagnosticInvoker>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProcessCpuUsage, 196813938, 51391, 16954, 168, 16, 181, 89, 174, 67, 84, 226);
RT_INTERFACE!{interface IProcessCpuUsage(IProcessCpuUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessCpuUsage] {
    fn GetReport(&self, out: *mut *mut ProcessCpuUsageReport) -> HRESULT
}}
impl IProcessCpuUsage {
    #[inline] pub fn get_report(&self) -> Result<Option<ComPtr<ProcessCpuUsageReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessCpuUsage: IProcessCpuUsage}
DEFINE_IID!(IID_IProcessCpuUsageReport, 2322439340, 14727, 20015, 161, 25, 107, 95, 162, 20, 241, 180);
RT_INTERFACE!{interface IProcessCpuUsageReport(IProcessCpuUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessCpuUsageReport] {
    fn get_KernelTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_UserTime(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl IProcessCpuUsageReport {
    #[inline] pub fn get_kernel_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KernelTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessCpuUsageReport: IProcessCpuUsageReport}
DEFINE_IID!(IID_IProcessDiagnosticInfo, 3895504971, 12302, 20198, 160, 171, 91, 95, 82, 49, 180, 52);
RT_INTERFACE!{interface IProcessDiagnosticInfo(IProcessDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfo] {
    fn get_ProcessId(&self, out: *mut u32) -> HRESULT,
    fn get_ExecutableFileName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Parent(&self, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT,
    fn get_ProcessStartTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_DiskUsage(&self, out: *mut *mut ProcessDiskUsage) -> HRESULT,
    fn get_MemoryUsage(&self, out: *mut *mut ProcessMemoryUsage) -> HRESULT,
    fn get_CpuUsage(&self, out: *mut *mut ProcessCpuUsage) -> HRESULT
}}
impl IProcessDiagnosticInfo {
    #[inline] pub fn get_process_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_executable_file_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ExecutableFileName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parent(&self) -> Result<Option<ComPtr<ProcessDiagnosticInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Parent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_process_start_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProcessStartTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_disk_usage(&self) -> Result<Option<ComPtr<ProcessDiskUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DiskUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_memory_usage(&self) -> Result<Option<ComPtr<ProcessMemoryUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cpu_usage(&self) -> Result<Option<ComPtr<ProcessCpuUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessDiagnosticInfo: IProcessDiagnosticInfo}
impl RtActivatable<IProcessDiagnosticInfoStatics> for ProcessDiagnosticInfo {}
impl RtActivatable<IProcessDiagnosticInfoStatics2> for ProcessDiagnosticInfo {}
impl ProcessDiagnosticInfo {
    #[inline] pub fn get_for_processes() -> Result<Option<ComPtr<foundation::collections::IVectorView<ProcessDiagnosticInfo>>>> {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics>>::get_activation_factory().get_for_processes()
    }
    #[inline] pub fn get_for_current_process() -> Result<Option<ComPtr<ProcessDiagnosticInfo>>> {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics>>::get_activation_factory().get_for_current_process()
    }
    #[inline] pub fn try_get_for_process_id(processId: u32) -> Result<Option<ComPtr<ProcessDiagnosticInfo>>> {
        <Self as RtActivatable<IProcessDiagnosticInfoStatics2>>::get_activation_factory().try_get_for_process_id(processId)
    }
}
DEFINE_CLSID!(ProcessDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,80,114,111,99,101,115,115,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_ProcessDiagnosticInfo]);
DEFINE_IID!(IID_IProcessDiagnosticInfo2, 2505624346, 15627, 18924, 171, 112, 79, 122, 17, 40, 5, 222);
RT_INTERFACE!{interface IProcessDiagnosticInfo2(IProcessDiagnosticInfo2Vtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfo2] {
    fn GetAppDiagnosticInfos(&self, out: *mut *mut foundation::collections::IVector<super::AppDiagnosticInfo>) -> HRESULT,
    fn get_IsPackaged(&self, out: *mut bool) -> HRESULT
}}
impl IProcessDiagnosticInfo2 {
    #[inline] pub fn get_app_diagnostic_infos(&self) -> Result<Option<ComPtr<foundation::collections::IVector<super::AppDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAppDiagnosticInfos)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_packaged(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPackaged)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProcessDiagnosticInfoStatics, 792834656, 46239, 17036, 170, 14, 132, 116, 79, 73, 202, 149);
RT_INTERFACE!{static interface IProcessDiagnosticInfoStatics(IProcessDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfoStatics] {
    fn GetForProcesses(&self, out: *mut *mut foundation::collections::IVectorView<ProcessDiagnosticInfo>) -> HRESULT,
    fn GetForCurrentProcess(&self, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT
}}
impl IProcessDiagnosticInfoStatics {
    #[inline] pub fn get_for_processes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ProcessDiagnosticInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForProcesses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_current_process(&self) -> Result<Option<ComPtr<ProcessDiagnosticInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentProcess)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProcessDiagnosticInfoStatics2, 1250334871, 39065, 19012, 162, 155, 9, 22, 99, 190, 9, 182);
RT_INTERFACE!{static interface IProcessDiagnosticInfoStatics2(IProcessDiagnosticInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiagnosticInfoStatics2] {
    fn TryGetForProcessId(&self, processId: u32, out: *mut *mut ProcessDiagnosticInfo) -> HRESULT
}}
impl IProcessDiagnosticInfoStatics2 {
    #[inline] pub fn try_get_for_process_id(&self, processId: u32) -> Result<Option<ComPtr<ProcessDiagnosticInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetForProcessId)(self as *const _ as *mut _, processId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProcessDiskUsage, 1524075517, 32337, 20051, 191, 170, 90, 110, 225, 170, 187, 248);
RT_INTERFACE!{interface IProcessDiskUsage(IProcessDiskUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiskUsage] {
    fn GetReport(&self, out: *mut *mut ProcessDiskUsageReport) -> HRESULT
}}
impl IProcessDiskUsage {
    #[inline] pub fn get_report(&self) -> Result<Option<ComPtr<ProcessDiskUsageReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessDiskUsage: IProcessDiskUsage}
DEFINE_IID!(IID_IProcessDiskUsageReport, 1075193853, 21341, 19487, 129, 184, 218, 84, 225, 190, 99, 94);
RT_INTERFACE!{interface IProcessDiskUsageReport(IProcessDiskUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessDiskUsageReport] {
    fn get_ReadOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_WriteOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_OtherOperationCount(&self, out: *mut i64) -> HRESULT,
    fn get_BytesReadCount(&self, out: *mut i64) -> HRESULT,
    fn get_BytesWrittenCount(&self, out: *mut i64) -> HRESULT,
    fn get_OtherBytesCount(&self, out: *mut i64) -> HRESULT
}}
impl IProcessDiskUsageReport {
    #[inline] pub fn get_read_operation_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReadOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_write_operation_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WriteOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_other_operation_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OtherOperationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_read_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesReadCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bytes_written_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BytesWrittenCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_other_bytes_count(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OtherBytesCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessDiskUsageReport: IProcessDiskUsageReport}
DEFINE_IID!(IID_IProcessMemoryUsage, 4111147675, 33404, 17079, 176, 124, 14, 50, 98, 126, 107, 62);
RT_INTERFACE!{interface IProcessMemoryUsage(IProcessMemoryUsageVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryUsage] {
    fn GetReport(&self, out: *mut *mut ProcessMemoryUsageReport) -> HRESULT
}}
impl IProcessMemoryUsage {
    #[inline] pub fn get_report(&self) -> Result<Option<ComPtr<ProcessMemoryUsageReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessMemoryUsage: IProcessMemoryUsage}
DEFINE_IID!(IID_IProcessMemoryUsageReport, 3267853498, 6481, 18053, 133, 50, 126, 116, 158, 207, 142, 235);
RT_INTERFACE!{interface IProcessMemoryUsageReport(IProcessMemoryUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_IProcessMemoryUsageReport] {
    fn get_NonPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PageFaultCount(&self, out: *mut u32) -> HRESULT,
    fn get_PageFileSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakNonPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPageFileSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakPagedPoolSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakVirtualMemorySizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PeakWorkingSetSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_PrivatePageCount(&self, out: *mut u64) -> HRESULT,
    fn get_VirtualMemorySizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_WorkingSetSizeInBytes(&self, out: *mut u64) -> HRESULT
}}
impl IProcessMemoryUsageReport {
    #[inline] pub fn get_non_paged_pool_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NonPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_page_fault_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageFaultCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_page_file_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageFileSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_paged_pool_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_non_paged_pool_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakNonPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_page_file_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPageFileSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_paged_pool_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakPagedPoolSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_virtual_memory_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakVirtualMemorySizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_peak_working_set_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PeakWorkingSetSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_private_page_count(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrivatePageCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_virtual_memory_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VirtualMemorySizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_working_set_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WorkingSetSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ProcessMemoryUsageReport: IProcessMemoryUsageReport}
DEFINE_IID!(IID_ISystemCpuUsage, 1614263212, 726, 16948, 131, 98, 127, 227, 173, 200, 31, 95);
RT_INTERFACE!{interface ISystemCpuUsage(ISystemCpuUsageVtbl): IInspectable(IInspectableVtbl) [IID_ISystemCpuUsage] {
    fn GetReport(&self, out: *mut *mut SystemCpuUsageReport) -> HRESULT
}}
impl ISystemCpuUsage {
    #[inline] pub fn get_report(&self) -> Result<Option<ComPtr<SystemCpuUsageReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemCpuUsage: ISystemCpuUsage}
DEFINE_IID!(IID_ISystemCpuUsageReport, 740741298, 38019, 20322, 171, 87, 130, 178, 157, 151, 25, 184);
RT_INTERFACE!{interface ISystemCpuUsageReport(ISystemCpuUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_ISystemCpuUsageReport] {
    fn get_KernelTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_UserTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_IdleTime(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl ISystemCpuUsageReport {
    #[inline] pub fn get_kernel_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KernelTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_idle_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IdleTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemCpuUsageReport: ISystemCpuUsageReport}
DEFINE_IID!(IID_ISystemDiagnosticInfo, 2727411205, 57331, 16511, 154, 27, 11, 43, 49, 124, 168, 0);
RT_INTERFACE!{interface ISystemDiagnosticInfo(ISystemDiagnosticInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDiagnosticInfo] {
    fn get_MemoryUsage(&self, out: *mut *mut SystemMemoryUsage) -> HRESULT,
    fn get_CpuUsage(&self, out: *mut *mut SystemCpuUsage) -> HRESULT
}}
impl ISystemDiagnosticInfo {
    #[inline] pub fn get_memory_usage(&self) -> Result<Option<ComPtr<SystemMemoryUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MemoryUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cpu_usage(&self) -> Result<Option<ComPtr<SystemCpuUsage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CpuUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemDiagnosticInfo: ISystemDiagnosticInfo}
impl RtActivatable<ISystemDiagnosticInfoStatics> for SystemDiagnosticInfo {}
impl SystemDiagnosticInfo {
    #[inline] pub fn get_for_current_system() -> Result<Option<ComPtr<SystemDiagnosticInfo>>> {
        <Self as RtActivatable<ISystemDiagnosticInfoStatics>>::get_activation_factory().get_for_current_system()
    }
}
DEFINE_CLSID!(SystemDiagnosticInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,83,121,115,116,101,109,68,105,97,103,110,111,115,116,105,99,73,110,102,111,0]) [CLSID_SystemDiagnosticInfo]);
DEFINE_IID!(IID_ISystemDiagnosticInfoStatics, 3557076001, 64637, 17904, 154, 63, 57, 32, 58, 237, 159, 126);
RT_INTERFACE!{static interface ISystemDiagnosticInfoStatics(ISystemDiagnosticInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemDiagnosticInfoStatics] {
    fn GetForCurrentSystem(&self, out: *mut *mut SystemDiagnosticInfo) -> HRESULT
}}
impl ISystemDiagnosticInfoStatics {
    #[inline] pub fn get_for_current_system(&self) -> Result<Option<ComPtr<SystemDiagnosticInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISystemMemoryUsage, 402638229, 5890, 18895, 170, 39, 47, 10, 50, 89, 20, 4);
RT_INTERFACE!{interface ISystemMemoryUsage(ISystemMemoryUsageVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMemoryUsage] {
    fn GetReport(&self, out: *mut *mut SystemMemoryUsageReport) -> HRESULT
}}
impl ISystemMemoryUsage {
    #[inline] pub fn get_report(&self) -> Result<Option<ComPtr<SystemMemoryUsageReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemMemoryUsage: ISystemMemoryUsage}
DEFINE_IID!(IID_ISystemMemoryUsageReport, 946224263, 10911, 16442, 189, 25, 44, 243, 232, 22, 149, 0);
RT_INTERFACE!{interface ISystemMemoryUsageReport(ISystemMemoryUsageReportVtbl): IInspectable(IInspectableVtbl) [IID_ISystemMemoryUsageReport] {
    fn get_TotalPhysicalSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_AvailableSizeInBytes(&self, out: *mut u64) -> HRESULT,
    fn get_CommittedSizeInBytes(&self, out: *mut u64) -> HRESULT
}}
impl ISystemMemoryUsageReport {
    #[inline] pub fn get_total_physical_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TotalPhysicalSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_available_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AvailableSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_committed_size_in_bytes(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CommittedSizeInBytes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemMemoryUsageReport: ISystemMemoryUsageReport}
pub mod deviceportal { // Windows.System.Diagnostics.DevicePortal
use crate::prelude::*;
DEFINE_IID!(IID_IDevicePortalConnection, 256147281, 4504, 19873, 141, 84, 189, 239, 57, 62, 9, 182);
RT_INTERFACE!{interface IDevicePortalConnection(IDevicePortalConnectionVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnection] {
    fn add_Closed(&self, handler: *mut foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RequestReceived(&self, handler: *mut foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RequestReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDevicePortalConnection {
    #[inline] pub fn add_closed(&self, handler: &foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_request_received(&self, handler: &foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RequestReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_request_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RequestReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DevicePortalConnection: IDevicePortalConnection}
impl RtActivatable<IDevicePortalConnectionStatics> for DevicePortalConnection {}
impl DevicePortalConnection {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_for_app_service_connection(appServiceConnection: &crate::windows::applicationmodel::appservice::AppServiceConnection) -> Result<Option<ComPtr<DevicePortalConnection>>> {
        <Self as RtActivatable<IDevicePortalConnectionStatics>>::get_activation_factory().get_for_app_service_connection(appServiceConnection)
    }
}
DEFINE_CLSID!(DevicePortalConnection(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,68,101,118,105,99,101,80,111,114,116,97,108,46,68,101,118,105,99,101,80,111,114,116,97,108,67,111,110,110,101,99,116,105,111,110,0]) [CLSID_DevicePortalConnection]);
DEFINE_IID!(IID_IDevicePortalConnectionClosedEventArgs, 4244049464, 28722, 17036, 159, 80, 148, 92, 21, 169, 240, 203);
RT_INTERFACE!{interface IDevicePortalConnectionClosedEventArgs(IDevicePortalConnectionClosedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionClosedEventArgs] {
    fn get_Reason(&self, out: *mut DevicePortalConnectionClosedReason) -> HRESULT
}}
impl IDevicePortalConnectionClosedEventArgs {
    #[inline] pub fn get_reason(&self) -> Result<DevicePortalConnectionClosedReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class DevicePortalConnectionClosedEventArgs: IDevicePortalConnectionClosedEventArgs}
RT_ENUM! { enum DevicePortalConnectionClosedReason: i32 {
    Unknown = 0, ResourceLimitsExceeded = 1, ProtocolError = 2, NotAuthorized = 3, UserNotPresent = 4, ServiceTerminated = 5,
}}
DEFINE_IID!(IID_IDevicePortalConnectionRequestReceivedEventArgs, 1692065861, 28634, 17497, 158, 189, 236, 206, 34, 227, 133, 89);
RT_INTERFACE!{interface IDevicePortalConnectionRequestReceivedEventArgs(IDevicePortalConnectionRequestReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionRequestReceivedEventArgs] {
    #[cfg(feature="windows-web")] fn get_RequestMessage(&self, out: *mut *mut crate::windows::web::http::HttpRequestMessage) -> HRESULT,
    #[cfg(feature="windows-web")] fn get_ResponseMessage(&self, out: *mut *mut crate::windows::web::http::HttpResponseMessage) -> HRESULT
}}
impl IDevicePortalConnectionRequestReceivedEventArgs {
    #[cfg(feature="windows-web")] #[inline] pub fn get_request_message(&self) -> Result<Option<ComPtr<crate::windows::web::http::HttpRequestMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequestMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn get_response_message(&self) -> Result<Option<ComPtr<crate::windows::web::http::HttpResponseMessage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ResponseMessage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DevicePortalConnectionRequestReceivedEventArgs: IDevicePortalConnectionRequestReceivedEventArgs}
DEFINE_IID!(IID_IDevicePortalConnectionStatics, 1270755815, 59833, 17989, 143, 237, 165, 62, 234, 14, 219, 214);
RT_INTERFACE!{static interface IDevicePortalConnectionStatics(IDevicePortalConnectionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalConnectionStatics] {
    #[cfg(feature="windows-applicationmodel")] fn GetForAppServiceConnection(&self, appServiceConnection: *mut crate::windows::applicationmodel::appservice::AppServiceConnection, out: *mut *mut DevicePortalConnection) -> HRESULT
}}
impl IDevicePortalConnectionStatics {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_for_app_service_connection(&self, appServiceConnection: &crate::windows::applicationmodel::appservice::AppServiceConnection) -> Result<Option<ComPtr<DevicePortalConnection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForAppServiceConnection)(self as *const _ as *mut _, appServiceConnection as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDevicePortalWebSocketConnection, 1734703392, 54874, 17136, 174, 244, 120, 120, 8, 9, 139, 123);
RT_INTERFACE!{interface IDevicePortalWebSocketConnection(IDevicePortalWebSocketConnectionVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalWebSocketConnection] {
    #[cfg(all(feature="windows-networking",feature="windows-web"))] fn GetServerMessageWebSocketForRequest(&self, request: *mut crate::windows::web::http::HttpRequestMessage, out: *mut *mut crate::windows::networking::sockets::ServerMessageWebSocket) -> HRESULT,
    #[cfg(all(feature="windows-networking",feature="windows-web"))] fn GetServerMessageWebSocketForRequest2(&self, request: *mut crate::windows::web::http::HttpRequestMessage, messageType: crate::windows::networking::sockets::SocketMessageType, protocol: HSTRING, out: *mut *mut crate::windows::networking::sockets::ServerMessageWebSocket) -> HRESULT,
    #[cfg(all(feature="windows-networking",feature="windows-web"))] fn GetServerMessageWebSocketForRequest3(&self, request: *mut crate::windows::web::http::HttpRequestMessage, messageType: crate::windows::networking::sockets::SocketMessageType, protocol: HSTRING, outboundBufferSizeInBytes: u32, maxMessageSize: u32, receiveMode: crate::windows::networking::sockets::MessageWebSocketReceiveMode, out: *mut *mut crate::windows::networking::sockets::ServerMessageWebSocket) -> HRESULT,
    #[cfg(all(feature="windows-networking",feature="windows-web"))] fn GetServerStreamWebSocketForRequest(&self, request: *mut crate::windows::web::http::HttpRequestMessage, out: *mut *mut crate::windows::networking::sockets::ServerStreamWebSocket) -> HRESULT,
    #[cfg(all(feature="windows-networking",feature="windows-web"))] fn GetServerStreamWebSocketForRequest2(&self, request: *mut crate::windows::web::http::HttpRequestMessage, protocol: HSTRING, outboundBufferSizeInBytes: u32, noDelay: bool, out: *mut *mut crate::windows::networking::sockets::ServerStreamWebSocket) -> HRESULT
}}
impl IDevicePortalWebSocketConnection {
    #[cfg(all(feature="windows-networking",feature="windows-web"))] #[inline] pub fn get_server_message_web_socket_for_request(&self, request: &crate::windows::web::http::HttpRequestMessage) -> Result<Option<ComPtr<crate::windows::networking::sockets::ServerMessageWebSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetServerMessageWebSocketForRequest)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(all(feature="windows-networking",feature="windows-web"))] #[inline] pub fn get_server_message_web_socket_for_request2(&self, request: &crate::windows::web::http::HttpRequestMessage, messageType: crate::windows::networking::sockets::SocketMessageType, protocol: &HStringArg) -> Result<Option<ComPtr<crate::windows::networking::sockets::ServerMessageWebSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetServerMessageWebSocketForRequest2)(self as *const _ as *mut _, request as *const _ as *mut _, messageType, protocol.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(all(feature="windows-networking",feature="windows-web"))] #[inline] pub fn get_server_message_web_socket_for_request3(&self, request: &crate::windows::web::http::HttpRequestMessage, messageType: crate::windows::networking::sockets::SocketMessageType, protocol: &HStringArg, outboundBufferSizeInBytes: u32, maxMessageSize: u32, receiveMode: crate::windows::networking::sockets::MessageWebSocketReceiveMode) -> Result<Option<ComPtr<crate::windows::networking::sockets::ServerMessageWebSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetServerMessageWebSocketForRequest3)(self as *const _ as *mut _, request as *const _ as *mut _, messageType, protocol.get(), outboundBufferSizeInBytes, maxMessageSize, receiveMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(all(feature="windows-networking",feature="windows-web"))] #[inline] pub fn get_server_stream_web_socket_for_request(&self, request: &crate::windows::web::http::HttpRequestMessage) -> Result<Option<ComPtr<crate::windows::networking::sockets::ServerStreamWebSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetServerStreamWebSocketForRequest)(self as *const _ as *mut _, request as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(all(feature="windows-networking",feature="windows-web"))] #[inline] pub fn get_server_stream_web_socket_for_request2(&self, request: &crate::windows::web::http::HttpRequestMessage, protocol: &HStringArg, outboundBufferSizeInBytes: u32, noDelay: bool) -> Result<Option<ComPtr<crate::windows::networking::sockets::ServerStreamWebSocket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetServerStreamWebSocketForRequest2)(self as *const _ as *mut _, request as *const _ as *mut _, protocol.get(), outboundBufferSizeInBytes, noDelay, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDevicePortalWebSocketConnectionRequestReceivedEventArgs, 2046675642, 5980, 18233, 159, 116, 221, 167, 151, 195, 91, 63);
RT_INTERFACE!{interface IDevicePortalWebSocketConnectionRequestReceivedEventArgs(IDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDevicePortalWebSocketConnectionRequestReceivedEventArgs] {
    fn get_IsWebSocketUpgradeRequest(&self, out: *mut bool) -> HRESULT,
    fn get_WebSocketProtocolsRequested(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    #[inline] pub fn get_is_web_socket_upgrade_request(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsWebSocketUpgradeRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_web_socket_protocols_requested(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WebSocketProtocolsRequested)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.System.Diagnostics.DevicePortal
pub mod telemetry { // Windows.System.Diagnostics.Telemetry
use crate::prelude::*;
RT_CLASS!{static class PlatformTelemetryClient}
impl RtActivatable<IPlatformTelemetryClientStatics> for PlatformTelemetryClient {}
impl PlatformTelemetryClient {
    #[inline] pub fn register(id: &HStringArg) -> Result<Option<ComPtr<PlatformTelemetryRegistrationResult>>> {
        <Self as RtActivatable<IPlatformTelemetryClientStatics>>::get_activation_factory().register(id)
    }
    #[inline] pub fn register_with_settings(id: &HStringArg, settings: &PlatformTelemetryRegistrationSettings) -> Result<Option<ComPtr<PlatformTelemetryRegistrationResult>>> {
        <Self as RtActivatable<IPlatformTelemetryClientStatics>>::get_activation_factory().register_with_settings(id, settings)
    }
}
DEFINE_CLSID!(PlatformTelemetryClient(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,101,108,101,109,101,116,114,121,46,80,108,97,116,102,111,114,109,84,101,108,101,109,101,116,114,121,67,108,105,101,110,116,0]) [CLSID_PlatformTelemetryClient]);
DEFINE_IID!(IID_IPlatformTelemetryClientStatics, 2616455773, 54723, 20202, 141, 190, 156, 141, 187, 13, 157, 143);
RT_INTERFACE!{static interface IPlatformTelemetryClientStatics(IPlatformTelemetryClientStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryClientStatics] {
    fn Register(&self, id: HSTRING, out: *mut *mut PlatformTelemetryRegistrationResult) -> HRESULT,
    fn RegisterWithSettings(&self, id: HSTRING, settings: *mut PlatformTelemetryRegistrationSettings, out: *mut *mut PlatformTelemetryRegistrationResult) -> HRESULT
}}
impl IPlatformTelemetryClientStatics {
    #[inline] pub fn register(&self, id: &HStringArg) -> Result<Option<ComPtr<PlatformTelemetryRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Register)(self as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_with_settings(&self, id: &HStringArg, settings: &PlatformTelemetryRegistrationSettings) -> Result<Option<ComPtr<PlatformTelemetryRegistrationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RegisterWithSettings)(self as *const _ as *mut _, id.get(), settings as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPlatformTelemetryRegistrationResult, 1300568235, 8850, 18877, 161, 90, 61, 113, 210, 20, 81, 18);
RT_INTERFACE!{interface IPlatformTelemetryRegistrationResult(IPlatformTelemetryRegistrationResultVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryRegistrationResult] {
    fn get_Status(&self, out: *mut PlatformTelemetryRegistrationStatus) -> HRESULT
}}
impl IPlatformTelemetryRegistrationResult {
    #[inline] pub fn get_status(&self) -> Result<PlatformTelemetryRegistrationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PlatformTelemetryRegistrationResult: IPlatformTelemetryRegistrationResult}
DEFINE_IID!(IID_IPlatformTelemetryRegistrationSettings, 2174387586, 51737, 16734, 187, 121, 156, 34, 75, 250, 58, 115);
RT_INTERFACE!{interface IPlatformTelemetryRegistrationSettings(IPlatformTelemetryRegistrationSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformTelemetryRegistrationSettings] {
    fn get_StorageSize(&self, out: *mut u32) -> HRESULT,
    fn put_StorageSize(&self, value: u32) -> HRESULT,
    fn get_UploadQuotaSize(&self, out: *mut u32) -> HRESULT,
    fn put_UploadQuotaSize(&self, value: u32) -> HRESULT
}}
impl IPlatformTelemetryRegistrationSettings {
    #[inline] pub fn get_storage_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StorageSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_storage_size(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_StorageSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_upload_quota_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UploadQuotaSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_upload_quota_size(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_UploadQuotaSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PlatformTelemetryRegistrationSettings: IPlatformTelemetryRegistrationSettings}
impl RtActivatable<IActivationFactory> for PlatformTelemetryRegistrationSettings {}
DEFINE_CLSID!(PlatformTelemetryRegistrationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,101,108,101,109,101,116,114,121,46,80,108,97,116,102,111,114,109,84,101,108,101,109,101,116,114,121,82,101,103,105,115,116,114,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_PlatformTelemetryRegistrationSettings]);
RT_ENUM! { enum PlatformTelemetryRegistrationStatus: i32 {
    Success = 0, SettingsOutOfRange = 1, UnknownFailure = 2,
}}
} // Windows.System.Diagnostics.Telemetry
pub mod tracereporting { // Windows.System.Diagnostics.TraceReporting
use crate::prelude::*;
RT_CLASS!{static class PlatformDiagnosticActions}
impl RtActivatable<IPlatformDiagnosticActionsStatics> for PlatformDiagnosticActions {}
impl PlatformDiagnosticActions {
    #[inline] pub fn is_scenario_enabled(scenarioId: Guid) -> Result<bool> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().is_scenario_enabled(scenarioId)
    }
    #[inline] pub fn try_escalate_scenario(scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: &HStringArg, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: &foundation::collections::IMapView<HString, HString>) -> Result<bool> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().try_escalate_scenario(scenarioId, escalationType, outputDirectory, timestampOutputDirectory, forceEscalationUpload, triggers)
    }
    #[inline] pub fn download_latest_settings_for_namespace(partner: &HStringArg, feature: &HStringArg, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool) -> Result<PlatformDiagnosticActionState> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().download_latest_settings_for_namespace(partner, feature, isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery)
    }
    #[inline] pub fn get_active_scenario_list() -> Result<Option<ComPtr<foundation::collections::IVectorView<Guid>>>> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_active_scenario_list()
    }
    #[inline] pub fn force_upload(latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool) -> Result<PlatformDiagnosticActionState> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().force_upload(latency, uploadOverCostedNetwork, uploadOverBattery)
    }
    #[inline] pub fn is_trace_running(slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64) -> Result<PlatformDiagnosticTraceSlotState> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().is_trace_running(slotType, scenarioId, traceProfileHash)
    }
    #[inline] pub fn get_active_trace_runtime(slotType: PlatformDiagnosticTraceSlotType) -> Result<Option<ComPtr<PlatformDiagnosticTraceRuntimeInfo>>> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_active_trace_runtime(slotType)
    }
    #[inline] pub fn get_known_trace_list(slotType: PlatformDiagnosticTraceSlotType) -> Result<Option<ComPtr<foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>>>> {
        <Self as RtActivatable<IPlatformDiagnosticActionsStatics>>::get_activation_factory().get_known_trace_list(slotType)
    }
}
DEFINE_CLSID!(PlatformDiagnosticActions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,97,103,110,111,115,116,105,99,115,46,84,114,97,99,101,82,101,112,111,114,116,105,110,103,46,80,108,97,116,102,111,114,109,68,105,97,103,110,111,115,116,105,99,65,99,116,105,111,110,115,0]) [CLSID_PlatformDiagnosticActions]);
DEFINE_IID!(IID_IPlatformDiagnosticActionsStatics, 3239337210, 37522, 16999, 137, 10, 158, 163, 237, 7, 35, 18);
RT_INTERFACE!{static interface IPlatformDiagnosticActionsStatics(IPlatformDiagnosticActionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticActionsStatics] {
    fn IsScenarioEnabled(&self, scenarioId: Guid, out: *mut bool) -> HRESULT,
    fn TryEscalateScenario(&self, scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: HSTRING, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: *mut foundation::collections::IMapView<HString, HString>, out: *mut bool) -> HRESULT,
    fn DownloadLatestSettingsForNamespace(&self, partner: HSTRING, feature: HSTRING, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool, out: *mut PlatformDiagnosticActionState) -> HRESULT,
    fn GetActiveScenarioList(&self, out: *mut *mut foundation::collections::IVectorView<Guid>) -> HRESULT,
    fn ForceUpload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool, out: *mut PlatformDiagnosticActionState) -> HRESULT,
    fn IsTraceRunning(&self, slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64, out: *mut PlatformDiagnosticTraceSlotState) -> HRESULT,
    fn GetActiveTraceRuntime(&self, slotType: PlatformDiagnosticTraceSlotType, out: *mut *mut PlatformDiagnosticTraceRuntimeInfo) -> HRESULT,
    fn GetKnownTraceList(&self, slotType: PlatformDiagnosticTraceSlotType, out: *mut *mut foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>) -> HRESULT
}}
impl IPlatformDiagnosticActionsStatics {
    #[inline] pub fn is_scenario_enabled(&self, scenarioId: Guid) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsScenarioEnabled)(self as *const _ as *mut _, scenarioId, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_escalate_scenario(&self, scenarioId: Guid, escalationType: PlatformDiagnosticEscalationType, outputDirectory: &HStringArg, timestampOutputDirectory: bool, forceEscalationUpload: bool, triggers: &foundation::collections::IMapView<HString, HString>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryEscalateScenario)(self as *const _ as *mut _, scenarioId, escalationType, outputDirectory.get(), timestampOutputDirectory, forceEscalationUpload, triggers as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn download_latest_settings_for_namespace(&self, partner: &HStringArg, feature: &HStringArg, isScenarioNamespace: bool, downloadOverCostedNetwork: bool, downloadOverBattery: bool) -> Result<PlatformDiagnosticActionState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).DownloadLatestSettingsForNamespace)(self as *const _ as *mut _, partner.get(), feature.get(), isScenarioNamespace, downloadOverCostedNetwork, downloadOverBattery, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_active_scenario_list(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<Guid>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetActiveScenarioList)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn force_upload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadOverCostedNetwork: bool, uploadOverBattery: bool) -> Result<PlatformDiagnosticActionState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ForceUpload)(self as *const _ as *mut _, latency, uploadOverCostedNetwork, uploadOverBattery, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_trace_running(&self, slotType: PlatformDiagnosticTraceSlotType, scenarioId: Guid, traceProfileHash: u64) -> Result<PlatformDiagnosticTraceSlotState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsTraceRunning)(self as *const _ as *mut _, slotType, scenarioId, traceProfileHash, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_active_trace_runtime(&self, slotType: PlatformDiagnosticTraceSlotType) -> Result<Option<ComPtr<PlatformDiagnosticTraceRuntimeInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetActiveTraceRuntime)(self as *const _ as *mut _, slotType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_known_trace_list(&self, slotType: PlatformDiagnosticTraceSlotType) -> Result<Option<ComPtr<foundation::collections::IVectorView<PlatformDiagnosticTraceInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetKnownTraceList)(self as *const _ as *mut _, slotType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum PlatformDiagnosticActionState: i32 {
    Success = 0, FreeNetworkNotAvailable = 1, ACPowerNotAvailable = 2,
}}
RT_ENUM! { enum PlatformDiagnosticEscalationType: i32 {
    OnCompletion = 0, OnFailure = 1,
}}
RT_ENUM! { enum PlatformDiagnosticEventBufferLatencies: u32 {
    Normal = 1, CostDeferred = 2, Realtime = 4,
}}
DEFINE_IID!(IID_IPlatformDiagnosticTraceInfo, 4168150423, 54679, 19447, 136, 220, 207, 92, 125, 194, 161, 210);
RT_INTERFACE!{interface IPlatformDiagnosticTraceInfo(IPlatformDiagnosticTraceInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticTraceInfo] {
    fn get_ScenarioId(&self, out: *mut Guid) -> HRESULT,
    fn get_ProfileHash(&self, out: *mut u64) -> HRESULT,
    fn get_IsExclusive(&self, out: *mut bool) -> HRESULT,
    fn get_IsAutoLogger(&self, out: *mut bool) -> HRESULT,
    fn get_MaxTraceDurationFileTime(&self, out: *mut i64) -> HRESULT,
    fn get_Priority(&self, out: *mut PlatformDiagnosticTracePriority) -> HRESULT
}}
impl IPlatformDiagnosticTraceInfo {
    #[inline] pub fn get_scenario_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScenarioId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_profile_hash(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProfileHash)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_exclusive(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsExclusive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_auto_logger(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAutoLogger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_trace_duration_file_time(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxTraceDurationFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_priority(&self) -> Result<PlatformDiagnosticTracePriority> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Priority)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PlatformDiagnosticTraceInfo: IPlatformDiagnosticTraceInfo}
RT_ENUM! { enum PlatformDiagnosticTracePriority: i32 {
    Normal = 0, UserElevated = 1,
}}
DEFINE_IID!(IID_IPlatformDiagnosticTraceRuntimeInfo, 1028480557, 472, 18280, 133, 84, 30, 177, 202, 97, 9, 134);
RT_INTERFACE!{interface IPlatformDiagnosticTraceRuntimeInfo(IPlatformDiagnosticTraceRuntimeInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticTraceRuntimeInfo] {
    fn get_RuntimeFileTime(&self, out: *mut i64) -> HRESULT,
    fn get_EtwRuntimeFileTime(&self, out: *mut i64) -> HRESULT
}}
impl IPlatformDiagnosticTraceRuntimeInfo {
    #[inline] pub fn get_runtime_file_time(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RuntimeFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_etw_runtime_file_time(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EtwRuntimeFileTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PlatformDiagnosticTraceRuntimeInfo: IPlatformDiagnosticTraceRuntimeInfo}
RT_ENUM! { enum PlatformDiagnosticTraceSlotState: i32 {
    NotRunning = 0, Running = 1, Throttled = 2,
}}
RT_ENUM! { enum PlatformDiagnosticTraceSlotType: i32 {
    Alternative = 0, AlwaysOn = 1, Mini = 2,
}}
} // Windows.System.Diagnostics.TraceReporting
} // Windows.System.Diagnostics
pub mod display { // Windows.System.Display
use crate::prelude::*;
DEFINE_IID!(IID_IDisplayRequest, 3849527364, 62623, 19296, 141, 212, 94, 126, 58, 99, 42, 192);
RT_INTERFACE!{interface IDisplayRequest(IDisplayRequestVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayRequest] {
    fn RequestActive(&self) -> HRESULT,
    fn RequestRelease(&self) -> HRESULT
}}
impl IDisplayRequest {
    #[inline] pub fn request_active(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RequestActive)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn request_release(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RequestRelease)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayRequest: IDisplayRequest}
impl RtActivatable<IActivationFactory> for DisplayRequest {}
DEFINE_CLSID!(DisplayRequest(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,82,101,113,117,101,115,116,0]) [CLSID_DisplayRequest]);
} // Windows.System.Display
pub mod inventory { // Windows.System.Inventory
use crate::prelude::*;
DEFINE_IID!(IID_IInstalledDesktopApp, 1978317037, 49340, 21348, 76, 40, 22, 110, 5, 69, 22, 122);
RT_INTERFACE!{interface IInstalledDesktopApp(IInstalledDesktopAppVtbl): IInspectable(IInspectableVtbl) [IID_IInstalledDesktopApp] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IInstalledDesktopApp {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_publisher(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Publisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class InstalledDesktopApp: IInstalledDesktopApp}
impl RtActivatable<IInstalledDesktopAppStatics> for InstalledDesktopApp {}
impl InstalledDesktopApp {
    #[inline] pub fn get_inventory_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<InstalledDesktopApp>>>> {
        <Self as RtActivatable<IInstalledDesktopAppStatics>>::get_activation_factory().get_inventory_async()
    }
}
DEFINE_CLSID!(InstalledDesktopApp(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,73,110,118,101,110,116,111,114,121,46,73,110,115,116,97,108,108,101,100,68,101,115,107,116,111,112,65,112,112,0]) [CLSID_InstalledDesktopApp]);
DEFINE_IID!(IID_IInstalledDesktopAppStatics, 642578254, 8653, 24475, 96, 86, 120, 102, 173, 114, 72, 154);
RT_INTERFACE!{static interface IInstalledDesktopAppStatics(IInstalledDesktopAppStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IInstalledDesktopAppStatics] {
    fn GetInventoryAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<InstalledDesktopApp>>) -> HRESULT
}}
impl IInstalledDesktopAppStatics {
    #[inline] pub fn get_inventory_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<InstalledDesktopApp>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetInventoryAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.System.Inventory
pub mod power { // Windows.System.Power
use crate::prelude::*;
RT_CLASS!{static class BackgroundEnergyManager}
impl RtActivatable<IBackgroundEnergyManagerStatics> for BackgroundEnergyManager {}
impl BackgroundEnergyManager {
    #[inline] pub fn get_low_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_low_usage_level()
    }
    #[inline] pub fn get_near_max_acceptable_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_near_max_acceptable_usage_level()
    }
    #[inline] pub fn get_max_acceptable_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_max_acceptable_usage_level()
    }
    #[inline] pub fn get_excessive_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_excessive_usage_level()
    }
    #[inline] pub fn get_near_termination_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_near_termination_usage_level()
    }
    #[inline] pub fn get_termination_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_termination_usage_level()
    }
    #[inline] pub fn get_recent_energy_usage() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage()
    }
    #[inline] pub fn get_recent_energy_usage_level() -> Result<u32> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage_level()
    }
    #[inline] pub fn add_recent_energy_usage_increased(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_increased(handler)
    }
    #[inline] pub fn remove_recent_energy_usage_increased(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_increased(token)
    }
    #[inline] pub fn add_recent_energy_usage_returned_to_low(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_returned_to_low(handler)
    }
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IBackgroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_returned_to_low(token)
    }
}
DEFINE_CLSID!(BackgroundEnergyManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,66,97,99,107,103,114,111,117,110,100,69,110,101,114,103,121,77,97,110,97,103,101,114,0]) [CLSID_BackgroundEnergyManager]);
DEFINE_IID!(IID_IBackgroundEnergyManagerStatics, 3004571029, 4480, 17270, 150, 225, 64, 149, 86, 129, 71, 206);
RT_INTERFACE!{static interface IBackgroundEnergyManagerStatics(IBackgroundEnergyManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundEnergyManagerStatics] {
    fn get_LowUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearMaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_MaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_ExcessiveUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearTerminationUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_TerminationUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsage(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn add_RecentEnergyUsageIncreased(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageIncreased(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RecentEnergyUsageReturnedToLow(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageReturnedToLow(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IBackgroundEnergyManagerStatics {
    #[inline] pub fn get_low_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LowUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_near_max_acceptable_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearMaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_acceptable_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_excessive_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExcessiveUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_near_termination_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearTerminationUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_termination_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TerminationUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recent_energy_usage(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recent_energy_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_recent_energy_usage_increased(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_recent_energy_usage_increased(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_recent_energy_usage_returned_to_low(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum BatteryStatus: i32 {
    NotPresent = 0, Discharging = 1, Idle = 2, Charging = 3,
}}
RT_ENUM! { enum EnergySaverStatus: i32 {
    Disabled = 0, Off = 1, On = 2,
}}
RT_CLASS!{static class ForegroundEnergyManager}
impl RtActivatable<IForegroundEnergyManagerStatics> for ForegroundEnergyManager {}
impl ForegroundEnergyManager {
    #[inline] pub fn get_low_usage_level() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_low_usage_level()
    }
    #[inline] pub fn get_near_max_acceptable_usage_level() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_near_max_acceptable_usage_level()
    }
    #[inline] pub fn get_max_acceptable_usage_level() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_max_acceptable_usage_level()
    }
    #[inline] pub fn get_excessive_usage_level() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_excessive_usage_level()
    }
    #[inline] pub fn get_recent_energy_usage() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage()
    }
    #[inline] pub fn get_recent_energy_usage_level() -> Result<u32> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().get_recent_energy_usage_level()
    }
    #[inline] pub fn add_recent_energy_usage_increased(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_increased(handler)
    }
    #[inline] pub fn remove_recent_energy_usage_increased(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_increased(token)
    }
    #[inline] pub fn add_recent_energy_usage_returned_to_low(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().add_recent_energy_usage_returned_to_low(handler)
    }
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IForegroundEnergyManagerStatics>>::get_activation_factory().remove_recent_energy_usage_returned_to_low(token)
    }
}
DEFINE_CLSID!(ForegroundEnergyManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,70,111,114,101,103,114,111,117,110,100,69,110,101,114,103,121,77,97,110,97,103,101,114,0]) [CLSID_ForegroundEnergyManager]);
DEFINE_IID!(IID_IForegroundEnergyManagerStatics, 2683857010, 58999, 18452, 154, 32, 83, 55, 202, 115, 43, 152);
RT_INTERFACE!{static interface IForegroundEnergyManagerStatics(IForegroundEnergyManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IForegroundEnergyManagerStatics] {
    fn get_LowUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_NearMaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_MaxAcceptableUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_ExcessiveUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsage(&self, out: *mut u32) -> HRESULT,
    fn get_RecentEnergyUsageLevel(&self, out: *mut u32) -> HRESULT,
    fn add_RecentEnergyUsageIncreased(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageIncreased(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RecentEnergyUsageReturnedToLow(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RecentEnergyUsageReturnedToLow(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IForegroundEnergyManagerStatics {
    #[inline] pub fn get_low_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LowUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_near_max_acceptable_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearMaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_acceptable_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAcceptableUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_excessive_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExcessiveUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recent_energy_usage(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recent_energy_usage_level(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RecentEnergyUsageLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_recent_energy_usage_increased(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageIncreased)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_recent_energy_usage_increased(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageIncreased)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_recent_energy_usage_returned_to_low(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_recent_energy_usage_returned_to_low(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RecentEnergyUsageReturnedToLow)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class PowerManager}
impl RtActivatable<IPowerManagerStatics> for PowerManager {}
impl PowerManager {
    #[inline] pub fn get_energy_saver_status() -> Result<EnergySaverStatus> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_energy_saver_status()
    }
    #[inline] pub fn add_energy_saver_status_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_energy_saver_status_changed(handler)
    }
    #[inline] pub fn remove_energy_saver_status_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_energy_saver_status_changed(token)
    }
    #[inline] pub fn get_battery_status() -> Result<BatteryStatus> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_battery_status()
    }
    #[inline] pub fn add_battery_status_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_battery_status_changed(handler)
    }
    #[inline] pub fn remove_battery_status_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_battery_status_changed(token)
    }
    #[inline] pub fn get_power_supply_status() -> Result<PowerSupplyStatus> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_power_supply_status()
    }
    #[inline] pub fn add_power_supply_status_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_power_supply_status_changed(handler)
    }
    #[inline] pub fn remove_power_supply_status_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_power_supply_status_changed(token)
    }
    #[inline] pub fn get_remaining_charge_percent() -> Result<i32> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_remaining_charge_percent()
    }
    #[inline] pub fn add_remaining_charge_percent_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_remaining_charge_percent_changed(handler)
    }
    #[inline] pub fn remove_remaining_charge_percent_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_remaining_charge_percent_changed(token)
    }
    #[inline] pub fn get_remaining_discharge_time() -> Result<foundation::TimeSpan> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().get_remaining_discharge_time()
    }
    #[inline] pub fn add_remaining_discharge_time_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().add_remaining_discharge_time_changed(handler)
    }
    #[inline] pub fn remove_remaining_discharge_time_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPowerManagerStatics>>::get_activation_factory().remove_remaining_discharge_time_changed(token)
    }
}
DEFINE_CLSID!(PowerManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,80,111,119,101,114,77,97,110,97,103,101,114,0]) [CLSID_PowerManager]);
DEFINE_IID!(IID_IPowerManagerStatics, 328499805, 25294, 17252, 152, 213, 170, 40, 199, 251, 209, 91);
RT_INTERFACE!{static interface IPowerManagerStatics(IPowerManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPowerManagerStatics] {
    fn get_EnergySaverStatus(&self, out: *mut EnergySaverStatus) -> HRESULT,
    fn add_EnergySaverStatusChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnergySaverStatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_BatteryStatus(&self, out: *mut BatteryStatus) -> HRESULT,
    fn add_BatteryStatusChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BatteryStatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_PowerSupplyStatus(&self, out: *mut PowerSupplyStatus) -> HRESULT,
    fn add_PowerSupplyStatusChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PowerSupplyStatusChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_RemainingChargePercent(&self, out: *mut i32) -> HRESULT,
    fn add_RemainingChargePercentChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemainingChargePercentChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_RemainingDischargeTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn add_RemainingDischargeTimeChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemainingDischargeTimeChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPowerManagerStatics {
    #[inline] pub fn get_energy_saver_status(&self) -> Result<EnergySaverStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_EnergySaverStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_energy_saver_status_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnergySaverStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_energy_saver_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnergySaverStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_battery_status(&self) -> Result<BatteryStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BatteryStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_battery_status_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_BatteryStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_battery_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_BatteryStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_power_supply_status(&self) -> Result<PowerSupplyStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PowerSupplyStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_power_supply_status_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PowerSupplyStatusChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_power_supply_status_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_PowerSupplyStatusChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_remaining_charge_percent(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemainingChargePercent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_remaining_charge_percent_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemainingChargePercentChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_remaining_charge_percent_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RemainingChargePercentChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_remaining_discharge_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemainingDischargeTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_remaining_discharge_time_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemainingDischargeTimeChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_remaining_discharge_time_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RemainingDischargeTimeChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum PowerSupplyStatus: i32 {
    NotPresent = 0, Inadequate = 1, Adequate = 2,
}}
pub mod diagnostics { // Windows.System.Power.Diagnostics
use crate::prelude::*;
RT_CLASS!{static class BackgroundEnergyDiagnostics}
impl RtActivatable<IBackgroundEnergyDiagnosticsStatics> for BackgroundEnergyDiagnostics {}
impl BackgroundEnergyDiagnostics {
    #[inline] pub fn get_device_specific_conversion_factor() -> Result<f64> {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().get_device_specific_conversion_factor()
    }
    #[inline] pub fn compute_total_energy_usage() -> Result<u64> {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().compute_total_energy_usage()
    }
    #[inline] pub fn reset_total_energy_usage() -> Result<()> {
        <Self as RtActivatable<IBackgroundEnergyDiagnosticsStatics>>::get_activation_factory().reset_total_energy_usage()
    }
}
DEFINE_CLSID!(BackgroundEnergyDiagnostics(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,68,105,97,103,110,111,115,116,105,99,115,46,66,97,99,107,103,114,111,117,110,100,69,110,101,114,103,121,68,105,97,103,110,111,115,116,105,99,115,0]) [CLSID_BackgroundEnergyDiagnostics]);
DEFINE_IID!(IID_IBackgroundEnergyDiagnosticsStatics, 3613800194, 54182, 18144, 143, 155, 80, 185, 91, 180, 249, 197);
RT_INTERFACE!{static interface IBackgroundEnergyDiagnosticsStatics(IBackgroundEnergyDiagnosticsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBackgroundEnergyDiagnosticsStatics] {
    fn get_DeviceSpecificConversionFactor(&self, out: *mut f64) -> HRESULT,
    fn ComputeTotalEnergyUsage(&self, out: *mut u64) -> HRESULT,
    fn ResetTotalEnergyUsage(&self) -> HRESULT
}}
impl IBackgroundEnergyDiagnosticsStatics {
    #[inline] pub fn get_device_specific_conversion_factor(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceSpecificConversionFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn compute_total_energy_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ComputeTotalEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn reset_total_energy_usage(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ResetTotalEnergyUsage)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class ForegroundEnergyDiagnostics}
impl RtActivatable<IForegroundEnergyDiagnosticsStatics> for ForegroundEnergyDiagnostics {}
impl ForegroundEnergyDiagnostics {
    #[inline] pub fn get_device_specific_conversion_factor() -> Result<f64> {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().get_device_specific_conversion_factor()
    }
    #[inline] pub fn compute_total_energy_usage() -> Result<u64> {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().compute_total_energy_usage()
    }
    #[inline] pub fn reset_total_energy_usage() -> Result<()> {
        <Self as RtActivatable<IForegroundEnergyDiagnosticsStatics>>::get_activation_factory().reset_total_energy_usage()
    }
}
DEFINE_CLSID!(ForegroundEnergyDiagnostics(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,111,119,101,114,46,68,105,97,103,110,111,115,116,105,99,115,46,70,111,114,101,103,114,111,117,110,100,69,110,101,114,103,121,68,105,97,103,110,111,115,116,105,99,115,0]) [CLSID_ForegroundEnergyDiagnostics]);
DEFINE_IID!(IID_IForegroundEnergyDiagnosticsStatics, 600443159, 52487, 17929, 190, 21, 143, 232, 148, 197, 228, 30);
RT_INTERFACE!{static interface IForegroundEnergyDiagnosticsStatics(IForegroundEnergyDiagnosticsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IForegroundEnergyDiagnosticsStatics] {
    fn get_DeviceSpecificConversionFactor(&self, out: *mut f64) -> HRESULT,
    fn ComputeTotalEnergyUsage(&self, out: *mut u64) -> HRESULT,
    fn ResetTotalEnergyUsage(&self) -> HRESULT
}}
impl IForegroundEnergyDiagnosticsStatics {
    #[inline] pub fn get_device_specific_conversion_factor(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DeviceSpecificConversionFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn compute_total_energy_usage(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).ComputeTotalEnergyUsage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn reset_total_energy_usage(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).ResetTotalEnergyUsage)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.System.Power.Diagnostics
} // Windows.System.Power
pub mod preview { // Windows.System.Preview
use crate::prelude::*;
RT_ENUM! { enum HingeState: i32 {
    Unknown = 0, Closed = 1, Concave = 2, Flat = 3, Convex = 4, Full = 5,
}}
DEFINE_IID!(IID_ITwoPanelHingedDevicePosturePreview, 1914985521, 19257, 17062, 142, 115, 114, 53, 173, 225, 104, 83);
RT_INTERFACE!{interface ITwoPanelHingedDevicePosturePreview(ITwoPanelHingedDevicePosturePreviewVtbl): IInspectable(IInspectableVtbl) [IID_ITwoPanelHingedDevicePosturePreview] {
    fn GetCurrentPostureAsync(&self, out: *mut *mut foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>) -> HRESULT,
    fn add_PostureChanged(&self, handler: *mut foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PostureChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ITwoPanelHingedDevicePosturePreview {
    #[inline] pub fn get_current_posture_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentPostureAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_posture_changed(&self, handler: &foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PostureChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_posture_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_PostureChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class TwoPanelHingedDevicePosturePreview: ITwoPanelHingedDevicePosturePreview}
impl RtActivatable<ITwoPanelHingedDevicePosturePreviewStatics> for TwoPanelHingedDevicePosturePreview {}
impl TwoPanelHingedDevicePosturePreview {
    #[inline] pub fn get_default_async() -> Result<ComPtr<foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>> {
        <Self as RtActivatable<ITwoPanelHingedDevicePosturePreviewStatics>>::get_activation_factory().get_default_async()
    }
}
DEFINE_CLSID!(TwoPanelHingedDevicePosturePreview(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,101,118,105,101,119,46,84,119,111,80,97,110,101,108,72,105,110,103,101,100,68,101,118,105,99,101,80,111,115,116,117,114,101,80,114,101,118,105,101,119,0]) [CLSID_TwoPanelHingedDevicePosturePreview]);
DEFINE_IID!(IID_ITwoPanelHingedDevicePosturePreviewReading, 2686784594, 19158, 19256, 132, 38, 197, 154, 21, 73, 58, 125);
RT_INTERFACE!{interface ITwoPanelHingedDevicePosturePreviewReading(ITwoPanelHingedDevicePosturePreviewReadingVtbl): IInspectable(IInspectableVtbl) [IID_ITwoPanelHingedDevicePosturePreviewReading] {
    fn get_Timestamp(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_HingeState(&self, out: *mut HingeState) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Panel1Orientation(&self, out: *mut super::super::devices::sensors::SimpleOrientation) -> HRESULT,
    fn get_Panel1Id(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-devices"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-devices")] fn get_Panel2Orientation(&self, out: *mut super::super::devices::sensors::SimpleOrientation) -> HRESULT,
    fn get_Panel2Id(&self, out: *mut HSTRING) -> HRESULT
}}
impl ITwoPanelHingedDevicePosturePreviewReading {
    #[inline] pub fn get_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_hinge_state(&self) -> Result<HingeState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HingeState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_panel1_orientation(&self) -> Result<super::super::devices::sensors::SimpleOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Panel1Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_panel1_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Panel1Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn get_panel2_orientation(&self) -> Result<super::super::devices::sensors::SimpleOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Panel2Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_panel2_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Panel2Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TwoPanelHingedDevicePosturePreviewReading: ITwoPanelHingedDevicePosturePreviewReading}
DEFINE_IID!(IID_ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs, 757930950, 718, 18250, 165, 86, 167, 91, 28, 249, 58, 3);
RT_INTERFACE!{interface ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs] {
    fn get_Reading(&self, out: *mut *mut TwoPanelHingedDevicePosturePreviewReading) -> HRESULT
}}
impl ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    #[inline] pub fn get_reading(&self) -> Result<Option<ComPtr<TwoPanelHingedDevicePosturePreviewReading>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Reading)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs: ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs}
DEFINE_IID!(IID_ITwoPanelHingedDevicePosturePreviewStatics, 205992914, 22496, 16768, 189, 94, 243, 26, 33, 56, 66, 62);
RT_INTERFACE!{static interface ITwoPanelHingedDevicePosturePreviewStatics(ITwoPanelHingedDevicePosturePreviewStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITwoPanelHingedDevicePosturePreviewStatics] {
    fn GetDefaultAsync(&self, out: *mut *mut foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>) -> HRESULT
}}
impl ITwoPanelHingedDevicePosturePreviewStatics {
    #[inline] pub fn get_default_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefaultAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.System.Preview
pub mod profile { // Windows.System.Profile
use crate::prelude::*;
RT_CLASS!{static class AnalyticsInfo}
impl RtActivatable<IAnalyticsInfoStatics> for AnalyticsInfo {}
impl RtActivatable<IAnalyticsInfoStatics2> for AnalyticsInfo {}
impl AnalyticsInfo {
    #[inline] pub fn get_version_info() -> Result<Option<ComPtr<AnalyticsVersionInfo>>> {
        <Self as RtActivatable<IAnalyticsInfoStatics>>::get_activation_factory().get_version_info()
    }
    #[inline] pub fn get_device_form() -> Result<HString> {
        <Self as RtActivatable<IAnalyticsInfoStatics>>::get_activation_factory().get_device_form()
    }
    #[inline] pub fn get_system_properties_async(attributeNames: &foundation::collections::IIterable<HString>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IMapView<HString, HString>>>> {
        <Self as RtActivatable<IAnalyticsInfoStatics2>>::get_activation_factory().get_system_properties_async(attributeNames)
    }
}
DEFINE_CLSID!(AnalyticsInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,65,110,97,108,121,116,105,99,115,73,110,102,111,0]) [CLSID_AnalyticsInfo]);
DEFINE_IID!(IID_IAnalyticsInfoStatics, 492757094, 6285, 23465, 67, 135, 172, 174, 176, 231, 227, 5);
RT_INTERFACE!{static interface IAnalyticsInfoStatics(IAnalyticsInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAnalyticsInfoStatics] {
    fn get_VersionInfo(&self, out: *mut *mut AnalyticsVersionInfo) -> HRESULT,
    fn get_DeviceForm(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAnalyticsInfoStatics {
    #[inline] pub fn get_version_info(&self) -> Result<Option<ComPtr<AnalyticsVersionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VersionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_form(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceForm)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAnalyticsInfoStatics2, 269944042, 43001, 18130, 171, 148, 1, 104, 101, 175, 219, 37);
RT_INTERFACE!{static interface IAnalyticsInfoStatics2(IAnalyticsInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAnalyticsInfoStatics2] {
    fn GetSystemPropertiesAsync(&self, attributeNames: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IMapView<HString, HString>>) -> HRESULT
}}
impl IAnalyticsInfoStatics2 {
    #[inline] pub fn get_system_properties_async(&self, attributeNames: &foundation::collections::IIterable<HString>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSystemPropertiesAsync)(self as *const _ as *mut _, attributeNames as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAnalyticsVersionInfo, 2455843000, 39253, 19572, 189, 193, 124, 208, 222, 207, 155, 3);
RT_INTERFACE!{interface IAnalyticsVersionInfo(IAnalyticsVersionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAnalyticsVersionInfo] {
    fn get_DeviceFamily(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceFamilyVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAnalyticsVersionInfo {
    #[inline] pub fn get_device_family(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceFamily)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_family_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DeviceFamilyVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AnalyticsVersionInfo: IAnalyticsVersionInfo}
RT_CLASS!{static class EducationSettings}
impl RtActivatable<IEducationSettingsStatics> for EducationSettings {}
impl EducationSettings {
    #[inline] pub fn get_is_education_environment() -> Result<bool> {
        <Self as RtActivatable<IEducationSettingsStatics>>::get_activation_factory().get_is_education_environment()
    }
}
DEFINE_CLSID!(EducationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,69,100,117,99,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_EducationSettings]);
DEFINE_IID!(IID_IEducationSettingsStatics, 4233359599, 19774, 19987, 155, 35, 80, 95, 77, 9, 30, 146);
RT_INTERFACE!{static interface IEducationSettingsStatics(IEducationSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IEducationSettingsStatics] {
    fn get_IsEducationEnvironment(&self, out: *mut bool) -> HRESULT
}}
impl IEducationSettingsStatics {
    #[inline] pub fn get_is_education_environment(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEducationEnvironment)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class HardwareIdentification}
impl RtActivatable<IHardwareIdentificationStatics> for HardwareIdentification {}
impl HardwareIdentification {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_package_specific_token(nonce: &super::super::storage::streams::IBuffer) -> Result<Option<ComPtr<HardwareToken>>> {
        <Self as RtActivatable<IHardwareIdentificationStatics>>::get_activation_factory().get_package_specific_token(nonce)
    }
}
DEFINE_CLSID!(HardwareIdentification(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,72,97,114,100,119,97,114,101,73,100,101,110,116,105,102,105,99,97,116,105,111,110,0]) [CLSID_HardwareIdentification]);
DEFINE_IID!(IID_IHardwareIdentificationStatics, 2534564064, 61808, 19010, 189, 85, 169, 0, 178, 18, 218, 226);
RT_INTERFACE!{static interface IHardwareIdentificationStatics(IHardwareIdentificationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHardwareIdentificationStatics] {
    #[cfg(feature="windows-storage")] fn GetPackageSpecificToken(&self, nonce: *mut super::super::storage::streams::IBuffer, out: *mut *mut HardwareToken) -> HRESULT
}}
impl IHardwareIdentificationStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_package_specific_token(&self, nonce: &super::super::storage::streams::IBuffer) -> Result<Option<ComPtr<HardwareToken>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPackageSpecificToken)(self as *const _ as *mut _, nonce as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHardwareToken, 687264960, 64274, 16548, 129, 103, 127, 78, 3, 210, 114, 76);
RT_INTERFACE!{interface IHardwareToken(IHardwareTokenVtbl): IInspectable(IInspectableVtbl) [IID_IHardwareToken] {
    #[cfg(feature="windows-storage")] fn get_Id(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Signature(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Certificate(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT
}}
impl IHardwareToken {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_id(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_signature(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Signature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_certificate(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Certificate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HardwareToken: IHardwareToken}
RT_CLASS!{static class KnownRetailInfoProperties}
impl RtActivatable<IKnownRetailInfoPropertiesStatics> for KnownRetailInfoProperties {}
impl KnownRetailInfoProperties {
    #[inline] pub fn get_retail_access_code() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_retail_access_code()
    }
    #[inline] pub fn get_manufacturer_name() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_manufacturer_name()
    }
    #[inline] pub fn get_model_name() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_model_name()
    }
    #[inline] pub fn get_display_model_name() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_display_model_name()
    }
    #[inline] pub fn get_price() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_price()
    }
    #[inline] pub fn get_is_featured() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_is_featured()
    }
    #[inline] pub fn get_form_factor() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_form_factor()
    }
    #[inline] pub fn get_screen_size() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_screen_size()
    }
    #[inline] pub fn get_weight() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_weight()
    }
    #[inline] pub fn get_display_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_display_description()
    }
    #[inline] pub fn get_battery_life_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_battery_life_description()
    }
    #[inline] pub fn get_processor_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_processor_description()
    }
    #[inline] pub fn get_memory() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_memory()
    }
    #[inline] pub fn get_storage_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_storage_description()
    }
    #[inline] pub fn get_graphics_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_graphics_description()
    }
    #[inline] pub fn get_front_camera_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_front_camera_description()
    }
    #[inline] pub fn get_rear_camera_description() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_rear_camera_description()
    }
    #[inline] pub fn get_has_nfc() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_nfc()
    }
    #[inline] pub fn get_has_sd_slot() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_sd_slot()
    }
    #[inline] pub fn get_has_optical_drive() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_has_optical_drive()
    }
    #[inline] pub fn get_is_office_installed() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_is_office_installed()
    }
    #[inline] pub fn get_windows_edition() -> Result<HString> {
        <Self as RtActivatable<IKnownRetailInfoPropertiesStatics>>::get_activation_factory().get_windows_edition()
    }
}
DEFINE_CLSID!(KnownRetailInfoProperties(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,75,110,111,119,110,82,101,116,97,105,108,73,110,102,111,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_KnownRetailInfoProperties]);
DEFINE_IID!(IID_IKnownRetailInfoPropertiesStatics, 2572620152, 20495, 18558, 142, 117, 41, 229, 81, 114, 135, 18);
RT_INTERFACE!{static interface IKnownRetailInfoPropertiesStatics(IKnownRetailInfoPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownRetailInfoPropertiesStatics] {
    fn get_RetailAccessCode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ManufacturerName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayModelName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Price(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsFeatured(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FormFactor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ScreenSize(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Weight(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BatteryLifeDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProcessorDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Memory(&self, out: *mut HSTRING) -> HRESULT,
    fn get_StorageDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_GraphicsDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrontCameraDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RearCameraDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasNfc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasSdSlot(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasOpticalDrive(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsOfficeInstalled(&self, out: *mut HSTRING) -> HRESULT,
    fn get_WindowsEdition(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownRetailInfoPropertiesStatics {
    #[inline] pub fn get_retail_access_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RetailAccessCode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_manufacturer_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ManufacturerName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModelName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_model_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayModelName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_price(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Price)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_featured(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsFeatured)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_form_factor(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FormFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_screen_size(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ScreenSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_weight(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Weight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_battery_life_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BatteryLifeDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_processor_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ProcessorDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_memory(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Memory)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_storage_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_StorageDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_graphics_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_GraphicsDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_front_camera_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FrontCameraDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rear_camera_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RearCameraDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_nfc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasNfc)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_sd_slot(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasSdSlot)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_optical_drive(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HasOpticalDrive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_office_installed(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_IsOfficeInstalled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_windows_edition(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WindowsEdition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum PlatformDataCollectionLevel: i32 {
    Security = 0, Basic = 1, Enhanced = 2, Full = 3,
}}
RT_CLASS!{static class PlatformDiagnosticsAndUsageDataSettings}
impl RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics> for PlatformDiagnosticsAndUsageDataSettings {}
impl PlatformDiagnosticsAndUsageDataSettings {
    #[inline] pub fn get_collection_level() -> Result<PlatformDataCollectionLevel> {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().get_collection_level()
    }
    #[inline] pub fn add_collection_level_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().add_collection_level_changed(handler)
    }
    #[inline] pub fn remove_collection_level_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().remove_collection_level_changed(token)
    }
    #[inline] pub fn can_collect_diagnostics(level: PlatformDataCollectionLevel) -> Result<bool> {
        <Self as RtActivatable<IPlatformDiagnosticsAndUsageDataSettingsStatics>>::get_activation_factory().can_collect_diagnostics(level)
    }
}
DEFINE_CLSID!(PlatformDiagnosticsAndUsageDataSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,80,108,97,116,102,111,114,109,68,105,97,103,110,111,115,116,105,99,115,65,110,100,85,115,97,103,101,68,97,116,97,83,101,116,116,105,110,103,115,0]) [CLSID_PlatformDiagnosticsAndUsageDataSettings]);
DEFINE_IID!(IID_IPlatformDiagnosticsAndUsageDataSettingsStatics, 3068283931, 31516, 19250, 140, 98, 166, 101, 151, 206, 114, 58);
RT_INTERFACE!{static interface IPlatformDiagnosticsAndUsageDataSettingsStatics(IPlatformDiagnosticsAndUsageDataSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPlatformDiagnosticsAndUsageDataSettingsStatics] {
    fn get_CollectionLevel(&self, out: *mut PlatformDataCollectionLevel) -> HRESULT,
    fn add_CollectionLevelChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CollectionLevelChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CanCollectDiagnostics(&self, level: PlatformDataCollectionLevel, out: *mut bool) -> HRESULT
}}
impl IPlatformDiagnosticsAndUsageDataSettingsStatics {
    #[inline] pub fn get_collection_level(&self) -> Result<PlatformDataCollectionLevel> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CollectionLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_collection_level_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CollectionLevelChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_collection_level_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_CollectionLevelChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn can_collect_diagnostics(&self, level: PlatformDataCollectionLevel) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).CanCollectDiagnostics)(self as *const _ as *mut _, level, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class RetailInfo}
impl RtActivatable<IRetailInfoStatics> for RetailInfo {}
impl RetailInfo {
    #[inline] pub fn get_is_demo_mode_enabled() -> Result<bool> {
        <Self as RtActivatable<IRetailInfoStatics>>::get_activation_factory().get_is_demo_mode_enabled()
    }
    #[inline] pub fn get_properties() -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> {
        <Self as RtActivatable<IRetailInfoStatics>>::get_activation_factory().get_properties()
    }
}
DEFINE_CLSID!(RetailInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,82,101,116,97,105,108,73,110,102,111,0]) [CLSID_RetailInfo]);
DEFINE_IID!(IID_IRetailInfoStatics, 118671032, 35730, 20266, 132, 153, 3, 31, 23, 152, 214, 239);
RT_INTERFACE!{static interface IRetailInfoStatics(IRetailInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRetailInfoStatics] {
    fn get_IsDemoModeEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::IMapView<HString, IInspectable>) -> HRESULT
}}
impl IRetailInfoStatics {
    #[inline] pub fn get_is_demo_mode_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDemoModeEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class SharedModeSettings}
impl RtActivatable<ISharedModeSettingsStatics> for SharedModeSettings {}
impl RtActivatable<ISharedModeSettingsStatics2> for SharedModeSettings {}
impl SharedModeSettings {
    #[inline] pub fn get_is_enabled() -> Result<bool> {
        <Self as RtActivatable<ISharedModeSettingsStatics>>::get_activation_factory().get_is_enabled()
    }
    #[inline] pub fn get_should_avoid_local_storage() -> Result<bool> {
        <Self as RtActivatable<ISharedModeSettingsStatics2>>::get_activation_factory().get_should_avoid_local_storage()
    }
}
DEFINE_CLSID!(SharedModeSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,104,97,114,101,100,77,111,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_SharedModeSettings]);
DEFINE_IID!(IID_ISharedModeSettingsStatics, 2302538766, 51926, 19792, 140, 73, 111, 207, 192, 62, 219, 41);
RT_INTERFACE!{static interface ISharedModeSettingsStatics(ISharedModeSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISharedModeSettingsStatics] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT
}}
impl ISharedModeSettingsStatics {
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISharedModeSettingsStatics2, 1619626148, 52465, 20200, 165, 226, 253, 106, 29, 12, 250, 200);
RT_INTERFACE!{static interface ISharedModeSettingsStatics2(ISharedModeSettingsStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISharedModeSettingsStatics2] {
    fn get_ShouldAvoidLocalStorage(&self, out: *mut bool) -> HRESULT
}}
impl ISharedModeSettingsStatics2 {
    #[inline] pub fn get_should_avoid_local_storage(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ShouldAvoidLocalStorage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class SystemIdentification}
impl RtActivatable<ISystemIdentificationStatics> for SystemIdentification {}
impl SystemIdentification {
    #[inline] pub fn get_system_id_for_publisher() -> Result<Option<ComPtr<SystemIdentificationInfo>>> {
        <Self as RtActivatable<ISystemIdentificationStatics>>::get_activation_factory().get_system_id_for_publisher()
    }
    #[inline] pub fn get_system_id_for_user(user: &super::User) -> Result<Option<ComPtr<SystemIdentificationInfo>>> {
        <Self as RtActivatable<ISystemIdentificationStatics>>::get_activation_factory().get_system_id_for_user(user)
    }
}
DEFINE_CLSID!(SystemIdentification(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,73,100,101,110,116,105,102,105,99,97,116,105,111,110,0]) [CLSID_SystemIdentification]);
DEFINE_IID!(IID_ISystemIdentificationInfo, 207986301, 50114, 19763, 162, 223, 33, 188, 65, 145, 110, 179);
RT_INTERFACE!{interface ISystemIdentificationInfo(ISystemIdentificationInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemIdentificationInfo] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Id(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn get_Source(&self, out: *mut SystemIdentificationSource) -> HRESULT
}}
impl ISystemIdentificationInfo {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_id(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<SystemIdentificationSource> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemIdentificationInfo: ISystemIdentificationInfo}
RT_ENUM! { enum SystemIdentificationSource: i32 {
    None = 0, Tpm = 1, Uefi = 2, Registry = 3,
}}
DEFINE_IID!(IID_ISystemIdentificationStatics, 1434580010, 54239, 19859, 163, 125, 196, 26, 97, 108, 109, 1);
RT_INTERFACE!{static interface ISystemIdentificationStatics(ISystemIdentificationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemIdentificationStatics] {
    fn GetSystemIdForPublisher(&self, out: *mut *mut SystemIdentificationInfo) -> HRESULT,
    fn GetSystemIdForUser(&self, user: *mut super::User, out: *mut *mut SystemIdentificationInfo) -> HRESULT
}}
impl ISystemIdentificationStatics {
    #[inline] pub fn get_system_id_for_publisher(&self) -> Result<Option<ComPtr<SystemIdentificationInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSystemIdForPublisher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id_for_user(&self, user: &super::User) -> Result<Option<ComPtr<SystemIdentificationInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSystemIdForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SystemOutOfBoxExperienceState: i32 {
    NotStarted = 0, InProgress = 1, Completed = 2,
}}
RT_CLASS!{static class SystemSetupInfo}
impl RtActivatable<ISystemSetupInfoStatics> for SystemSetupInfo {}
impl SystemSetupInfo {
    #[inline] pub fn get_out_of_box_experience_state() -> Result<SystemOutOfBoxExperienceState> {
        <Self as RtActivatable<ISystemSetupInfoStatics>>::get_activation_factory().get_out_of_box_experience_state()
    }
    #[inline] pub fn add_out_of_box_experience_state_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<ISystemSetupInfoStatics>>::get_activation_factory().add_out_of_box_experience_state_changed(handler)
    }
    #[inline] pub fn remove_out_of_box_experience_state_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<ISystemSetupInfoStatics>>::get_activation_factory().remove_out_of_box_experience_state_changed(token)
    }
}
DEFINE_CLSID!(SystemSetupInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,83,101,116,117,112,73,110,102,111,0]) [CLSID_SystemSetupInfo]);
DEFINE_IID!(IID_ISystemSetupInfoStatics, 748036264, 7560, 24109, 163, 36, 165, 67, 175, 66, 71, 238);
RT_INTERFACE!{static interface ISystemSetupInfoStatics(ISystemSetupInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemSetupInfoStatics] {
    fn get_OutOfBoxExperienceState(&self, out: *mut SystemOutOfBoxExperienceState) -> HRESULT,
    fn add_OutOfBoxExperienceStateChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OutOfBoxExperienceStateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ISystemSetupInfoStatics {
    #[inline] pub fn get_out_of_box_experience_state(&self) -> Result<SystemOutOfBoxExperienceState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OutOfBoxExperienceState)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_out_of_box_experience_state_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OutOfBoxExperienceStateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_out_of_box_experience_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_OutOfBoxExperienceStateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class WindowsIntegrityPolicy}
impl RtActivatable<IWindowsIntegrityPolicyStatics> for WindowsIntegrityPolicy {}
impl WindowsIntegrityPolicy {
    #[inline] pub fn get_is_enabled() -> Result<bool> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().get_is_enabled()
    }
    #[inline] pub fn get_is_enabled_for_trial() -> Result<bool> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().get_is_enabled_for_trial()
    }
    #[inline] pub fn get_can_disable() -> Result<bool> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().get_can_disable()
    }
    #[inline] pub fn get_is_disable_supported() -> Result<bool> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().get_is_disable_supported()
    }
    #[inline] pub fn add_policy_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().add_policy_changed(handler)
    }
    #[inline] pub fn remove_policy_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IWindowsIntegrityPolicyStatics>>::get_activation_factory().remove_policy_changed(token)
    }
}
DEFINE_CLSID!(WindowsIntegrityPolicy(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,87,105,110,100,111,119,115,73,110,116,101,103,114,105,116,121,80,111,108,105,99,121,0]) [CLSID_WindowsIntegrityPolicy]);
DEFINE_IID!(IID_IWindowsIntegrityPolicyStatics, 2099085787, 36195, 18313, 158, 165, 221, 207, 101, 169, 79, 60);
RT_INTERFACE!{static interface IWindowsIntegrityPolicyStatics(IWindowsIntegrityPolicyStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IWindowsIntegrityPolicyStatics] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_IsEnabledForTrial(&self, out: *mut bool) -> HRESULT,
    fn get_CanDisable(&self, out: *mut bool) -> HRESULT,
    fn get_IsDisableSupported(&self, out: *mut bool) -> HRESULT,
    fn add_PolicyChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PolicyChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IWindowsIntegrityPolicyStatics {
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_enabled_for_trial(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabledForTrial)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_can_disable(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanDisable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_disable_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDisableSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_policy_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PolicyChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_policy_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_PolicyChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
pub mod systemmanufacturers { // Windows.System.Profile.SystemManufacturers
use crate::prelude::*;
DEFINE_IID!(IID_IOemSupportInfo, 2368646741, 34799, 16998, 134, 208, 196, 175, 190, 178, 155, 185);
RT_INTERFACE!{interface IOemSupportInfo(IOemSupportInfoVtbl): IInspectable(IInspectableVtbl) [IID_IOemSupportInfo] {
    fn get_SupportLink(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_SupportAppLink(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    fn get_SupportProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOemSupportInfo {
    #[inline] pub fn get_support_link(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportLink)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_support_app_link(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportAppLink)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_support_provider(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SupportProvider)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OemSupportInfo: IOemSupportInfo}
RT_CLASS!{static class SmbiosInformation}
impl RtActivatable<ISmbiosInformationStatics> for SmbiosInformation {}
impl SmbiosInformation {
    #[inline] pub fn get_serial_number() -> Result<HString> {
        <Self as RtActivatable<ISmbiosInformationStatics>>::get_activation_factory().get_serial_number()
    }
}
DEFINE_CLSID!(SmbiosInformation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,77,97,110,117,102,97,99,116,117,114,101,114,115,46,83,109,98,105,111,115,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_SmbiosInformation]);
DEFINE_IID!(IID_ISmbiosInformationStatics, 135055996, 25468, 18628, 183, 40, 249, 39, 56, 18, 219, 142);
RT_INTERFACE!{static interface ISmbiosInformationStatics(ISmbiosInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISmbiosInformationStatics] {
    fn get_SerialNumber(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISmbiosInformationStatics {
    #[inline] pub fn get_serial_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SerialNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISystemSupportDeviceInfo, 92801945, 33351, 17435, 169, 150, 161, 120, 75, 171, 121, 168);
RT_INTERFACE!{interface ISystemSupportDeviceInfo(ISystemSupportDeviceInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemSupportDeviceInfo] {
    fn get_OperatingSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemProductName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemSku(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemHardwareVersion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemFirmwareVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemSupportDeviceInfo {
    #[inline] pub fn get_operating_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OperatingSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_manufacturer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemManufacturer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_product_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemProductName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_sku(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemSku)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_hardware_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemHardwareVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_firmware_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SystemFirmwareVersion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemSupportDeviceInfo: ISystemSupportDeviceInfo}
RT_CLASS!{static class SystemSupportInfo}
impl RtActivatable<ISystemSupportInfoStatics> for SystemSupportInfo {}
impl RtActivatable<ISystemSupportInfoStatics2> for SystemSupportInfo {}
impl SystemSupportInfo {
    #[inline] pub fn get_local_system_edition() -> Result<HString> {
        <Self as RtActivatable<ISystemSupportInfoStatics>>::get_activation_factory().get_local_system_edition()
    }
    #[inline] pub fn get_oem_support_info() -> Result<Option<ComPtr<OemSupportInfo>>> {
        <Self as RtActivatable<ISystemSupportInfoStatics>>::get_activation_factory().get_oem_support_info()
    }
    #[inline] pub fn get_local_device_info() -> Result<Option<ComPtr<SystemSupportDeviceInfo>>> {
        <Self as RtActivatable<ISystemSupportInfoStatics2>>::get_activation_factory().get_local_device_info()
    }
}
DEFINE_CLSID!(SystemSupportInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,80,114,111,102,105,108,101,46,83,121,115,116,101,109,77,97,110,117,102,97,99,116,117,114,101,114,115,46,83,121,115,116,101,109,83,117,112,112,111,114,116,73,110,102,111,0]) [CLSID_SystemSupportInfo]);
DEFINE_IID!(IID_ISystemSupportInfoStatics, 4017424756, 50210, 17879, 164, 77, 92, 28, 0, 67, 162, 179);
RT_INTERFACE!{static interface ISystemSupportInfoStatics(ISystemSupportInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemSupportInfoStatics] {
    fn get_LocalSystemEdition(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OemSupportInfo(&self, out: *mut *mut OemSupportInfo) -> HRESULT
}}
impl ISystemSupportInfoStatics {
    #[inline] pub fn get_local_system_edition(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalSystemEdition)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_oem_support_info(&self) -> Result<Option<ComPtr<OemSupportInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OemSupportInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISystemSupportInfoStatics2, 871582116, 16289, 18822, 170, 75, 5, 116, 32, 69, 94, 109);
RT_INTERFACE!{static interface ISystemSupportInfoStatics2(ISystemSupportInfoStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISystemSupportInfoStatics2] {
    fn get_LocalDeviceInfo(&self, out: *mut *mut SystemSupportDeviceInfo) -> HRESULT
}}
impl ISystemSupportInfoStatics2 {
    #[inline] pub fn get_local_device_info(&self) -> Result<Option<ComPtr<SystemSupportDeviceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LocalDeviceInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.System.Profile.SystemManufacturers
} // Windows.System.Profile
pub mod remotedesktop { // Windows.System.RemoteDesktop
use crate::prelude::*;
RT_CLASS!{static class InteractiveSession}
impl RtActivatable<IInteractiveSessionStatics> for InteractiveSession {}
impl InteractiveSession {
    #[inline] pub fn get_is_remote() -> Result<bool> {
        <Self as RtActivatable<IInteractiveSessionStatics>>::get_activation_factory().get_is_remote()
    }
}
DEFINE_CLSID!(InteractiveSession(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,68,101,115,107,116,111,112,46,73,110,116,101,114,97,99,116,105,118,101,83,101,115,115,105,111,110,0]) [CLSID_InteractiveSession]);
DEFINE_IID!(IID_IInteractiveSessionStatics, 1619543601, 56634, 17782, 156, 141, 232, 2, 118, 24, 189, 206);
RT_INTERFACE!{static interface IInteractiveSessionStatics(IInteractiveSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IInteractiveSessionStatics] {
    fn get_IsRemote(&self, out: *mut bool) -> HRESULT
}}
impl IInteractiveSessionStatics {
    #[inline] pub fn get_is_remote(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRemote)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.System.RemoteDesktop
pub mod remotesystems { // Windows.System.RemoteSystems
use crate::prelude::*;
RT_CLASS!{static class KnownRemoteSystemCapabilities}
impl RtActivatable<IKnownRemoteSystemCapabilitiesStatics> for KnownRemoteSystemCapabilities {}
impl KnownRemoteSystemCapabilities {
    #[inline] pub fn get_app_service() -> Result<HString> {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_app_service()
    }
    #[inline] pub fn get_launch_uri() -> Result<HString> {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_launch_uri()
    }
    #[inline] pub fn get_remote_session() -> Result<HString> {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_remote_session()
    }
    #[inline] pub fn get_spatial_entity() -> Result<HString> {
        <Self as RtActivatable<IKnownRemoteSystemCapabilitiesStatics>>::get_activation_factory().get_spatial_entity()
    }
}
DEFINE_CLSID!(KnownRemoteSystemCapabilities(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,75,110,111,119,110,82,101,109,111,116,101,83,121,115,116,101,109,67,97,112,97,98,105,108,105,116,105,101,115,0]) [CLSID_KnownRemoteSystemCapabilities]);
DEFINE_IID!(IID_IKnownRemoteSystemCapabilitiesStatics, 2164843392, 32650, 17636, 146, 205, 3, 182, 70, 155, 148, 163);
RT_INTERFACE!{static interface IKnownRemoteSystemCapabilitiesStatics(IKnownRemoteSystemCapabilitiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IKnownRemoteSystemCapabilitiesStatics] {
    fn get_AppService(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LaunchUri(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RemoteSession(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SpatialEntity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKnownRemoteSystemCapabilitiesStatics {
    #[inline] pub fn get_app_service(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AppService)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_launch_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LaunchUri)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_remote_session(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_spatial_entity(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SpatialEntity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystem, 3981981901, 7696, 19084, 180, 166, 78, 95, 214, 249, 119, 33);
RT_INTERFACE!{interface IRemoteSystem(IRemoteSystemVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemStatus) -> HRESULT,
    fn get_IsAvailableByProximity(&self, out: *mut bool) -> HRESULT
}}
impl IRemoteSystem {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Kind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<RemoteSystemStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_available_by_proximity(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableByProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystem: IRemoteSystem}
impl RtActivatable<IRemoteSystemStatics> for RemoteSystem {}
impl RtActivatable<IRemoteSystemStatics2> for RemoteSystem {}
impl RemoteSystem {
    #[cfg(feature="windows-networking")] #[inline] pub fn find_by_host_name_async(hostName: &super::super::networking::HostName) -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystem>>> {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().find_by_host_name_async(hostName)
    }
    #[inline] pub fn create_watcher() -> Result<Option<ComPtr<RemoteSystemWatcher>>> {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().create_watcher()
    }
    #[inline] pub fn create_watcher_with_filters(filters: &foundation::collections::IIterable<IRemoteSystemFilter>) -> Result<Option<ComPtr<RemoteSystemWatcher>>> {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().create_watcher_with_filters(filters)
    }
    #[inline] pub fn request_access_async() -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystemAccessStatus>>> {
        <Self as RtActivatable<IRemoteSystemStatics>>::get_activation_factory().request_access_async()
    }
    #[inline] pub fn is_authorization_kind_enabled(kind: RemoteSystemAuthorizationKind) -> Result<bool> {
        <Self as RtActivatable<IRemoteSystemStatics2>>::get_activation_factory().is_authorization_kind_enabled(kind)
    }
}
DEFINE_CLSID!(RemoteSystem(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,0]) [CLSID_RemoteSystem]);
DEFINE_IID!(IID_IRemoteSystem2, 165668076, 64395, 18952, 167, 88, 104, 118, 67, 93, 118, 158);
RT_INTERFACE!{interface IRemoteSystem2(IRemoteSystem2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem2] {
    fn get_IsAvailableBySpatialProximity(&self, out: *mut bool) -> HRESULT,
    fn GetCapabilitySupportedAsync(&self, capabilityName: HSTRING, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IRemoteSystem2 {
    #[inline] pub fn get_is_available_by_spatial_proximity(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableBySpatialProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_capability_supported_async(&self, capabilityName: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCapabilitySupportedAsync)(self as *const _ as *mut _, capabilityName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystem3, 1924445333, 47046, 16574, 131, 27, 115, 86, 47, 18, 255, 168);
RT_INTERFACE!{interface IRemoteSystem3(IRemoteSystem3Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem3] {
    fn get_ManufacturerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelDisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystem3 {
    #[inline] pub fn get_manufacturer_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ManufacturerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModelDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystem4, 4049928165, 47495, 19621, 153, 38, 250, 4, 56, 190, 98, 115);
RT_INTERFACE!{interface IRemoteSystem4(IRemoteSystem4Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem4] {
    fn get_Platform(&self, out: *mut RemoteSystemPlatform) -> HRESULT
}}
impl IRemoteSystem4 {
    #[inline] pub fn get_platform(&self) -> Result<RemoteSystemPlatform> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Platform)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystem5, 3945453347, 58850, 19170, 167, 167, 161, 9, 122, 9, 142, 144);
RT_INTERFACE!{interface IRemoteSystem5(IRemoteSystem5Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystem5] {
    fn get_Apps(&self, out: *mut *mut foundation::collections::IVectorView<RemoteSystemApp>) -> HRESULT
}}
impl IRemoteSystem5 {
    #[inline] pub fn get_apps(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<RemoteSystemApp>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Apps)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemAccessStatus: i32 {
    Unspecified = 0, Allowed = 1, DeniedByUser = 2, DeniedBySystem = 3,
}}
DEFINE_IID!(IID_IRemoteSystemAddedEventArgs, 2402899471, 58676, 18071, 136, 54, 122, 190, 161, 81, 81, 110);
RT_INTERFACE!{interface IRemoteSystemAddedEventArgs(IRemoteSystemAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAddedEventArgs] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemAddedEventArgs {
    #[inline] pub fn get_remote_system(&self) -> Result<Option<ComPtr<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemAddedEventArgs: IRemoteSystemAddedEventArgs}
DEFINE_IID!(IID_IRemoteSystemApp, 2162539709, 54605, 16817, 155, 22, 104, 16, 168, 113, 237, 79);
RT_INTERFACE!{interface IRemoteSystemApp(IRemoteSystemAppVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemApp] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsAvailableByProximity(&self, out: *mut bool) -> HRESULT,
    fn get_IsAvailableBySpatialProximity(&self, out: *mut bool) -> HRESULT,
    fn get_Attributes(&self, out: *mut *mut foundation::collections::IMapView<HString, HString>) -> HRESULT
}}
impl IRemoteSystemApp {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_available_by_proximity(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableByProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_available_by_spatial_proximity(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailableBySpatialProximity)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_attributes(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Attributes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemApp: IRemoteSystemApp}
DEFINE_IID!(IID_IRemoteSystemAppRegistration, 3027847093, 28725, 19034, 184, 223, 150, 45, 143, 132, 49, 244);
RT_INTERFACE!{interface IRemoteSystemAppRegistration(IRemoteSystemAppRegistrationVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAppRegistration] {
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT,
    fn get_Attributes(&self, out: *mut *mut foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn SaveAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IRemoteSystemAppRegistration {
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attributes(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Attributes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemAppRegistration: IRemoteSystemAppRegistration}
impl RtActivatable<IRemoteSystemAppRegistrationStatics> for RemoteSystemAppRegistration {}
impl RemoteSystemAppRegistration {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<RemoteSystemAppRegistration>>> {
        <Self as RtActivatable<IRemoteSystemAppRegistrationStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<RemoteSystemAppRegistration>>> {
        <Self as RtActivatable<IRemoteSystemAppRegistrationStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(RemoteSystemAppRegistration(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,65,112,112,82,101,103,105,115,116,114,97,116,105,111,110,0]) [CLSID_RemoteSystemAppRegistration]);
DEFINE_IID!(IID_IRemoteSystemAppRegistrationStatics, 28940352, 53202, 17727, 174, 37, 194, 83, 159, 8, 106, 253);
RT_INTERFACE!{static interface IRemoteSystemAppRegistrationStatics(IRemoteSystemAppRegistrationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAppRegistrationStatics] {
    fn GetDefault(&self, out: *mut *mut RemoteSystemAppRegistration) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut RemoteSystemAppRegistration) -> HRESULT
}}
impl IRemoteSystemAppRegistrationStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<RemoteSystemAppRegistration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<RemoteSystemAppRegistration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemAuthorizationKind: i32 {
    SameUser = 0, Anonymous = 1,
}}
DEFINE_IID!(IID_IRemoteSystemAuthorizationKindFilter, 1796071054, 1232, 16628, 162, 127, 194, 172, 187, 214, 183, 52);
RT_INTERFACE!{interface IRemoteSystemAuthorizationKindFilter(IRemoteSystemAuthorizationKindFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAuthorizationKindFilter] {
    fn get_RemoteSystemAuthorizationKind(&self, out: *mut RemoteSystemAuthorizationKind) -> HRESULT
}}
impl IRemoteSystemAuthorizationKindFilter {
    #[inline] pub fn get_remote_system_authorization_kind(&self) -> Result<RemoteSystemAuthorizationKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemAuthorizationKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemAuthorizationKindFilter: IRemoteSystemAuthorizationKindFilter}
impl RtActivatable<IRemoteSystemAuthorizationKindFilterFactory> for RemoteSystemAuthorizationKindFilter {}
impl RemoteSystemAuthorizationKindFilter {
    #[inline] pub fn create(remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind) -> Result<ComPtr<RemoteSystemAuthorizationKindFilter>> {
        <Self as RtActivatable<IRemoteSystemAuthorizationKindFilterFactory>>::get_activation_factory().create(remoteSystemAuthorizationKind)
    }
}
DEFINE_CLSID!(RemoteSystemAuthorizationKindFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,65,117,116,104,111,114,105,122,97,116,105,111,110,75,105,110,100,70,105,108,116,101,114,0]) [CLSID_RemoteSystemAuthorizationKindFilter]);
DEFINE_IID!(IID_IRemoteSystemAuthorizationKindFilterFactory, 2909134669, 46698, 17828, 129, 119, 140, 174, 215, 93, 158, 90);
RT_INTERFACE!{static interface IRemoteSystemAuthorizationKindFilterFactory(IRemoteSystemAuthorizationKindFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemAuthorizationKindFilterFactory] {
    fn Create(&self, remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind, out: *mut *mut RemoteSystemAuthorizationKindFilter) -> HRESULT
}}
impl IRemoteSystemAuthorizationKindFilterFactory {
    #[inline] pub fn create(&self, remoteSystemAuthorizationKind: RemoteSystemAuthorizationKind) -> Result<ComPtr<RemoteSystemAuthorizationKindFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemAuthorizationKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemConnectionInfo, 589794243, 3337, 21195, 156, 106, 238, 210, 148, 11, 238, 67);
RT_INTERFACE!{interface IRemoteSystemConnectionInfo(IRemoteSystemConnectionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionInfo] {
    fn get_IsProximal(&self, out: *mut bool) -> HRESULT
}}
impl IRemoteSystemConnectionInfo {
    #[inline] pub fn get_is_proximal(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsProximal)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemConnectionInfo: IRemoteSystemConnectionInfo}
impl RtActivatable<IRemoteSystemConnectionInfoStatics> for RemoteSystemConnectionInfo {}
impl RemoteSystemConnectionInfo {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn try_create_from_app_service_connection(connection: &super::super::applicationmodel::appservice::AppServiceConnection) -> Result<Option<ComPtr<RemoteSystemConnectionInfo>>> {
        <Self as RtActivatable<IRemoteSystemConnectionInfoStatics>>::get_activation_factory().try_create_from_app_service_connection(connection)
    }
}
DEFINE_CLSID!(RemoteSystemConnectionInfo(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,67,111,110,110,101,99,116,105,111,110,73,110,102,111,0]) [CLSID_RemoteSystemConnectionInfo]);
DEFINE_IID!(IID_IRemoteSystemConnectionInfoStatics, 2894274093, 26309, 22231, 164, 206, 112, 93, 148, 146, 90, 214);
RT_INTERFACE!{static interface IRemoteSystemConnectionInfoStatics(IRemoteSystemConnectionInfoStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionInfoStatics] {
    #[cfg(feature="windows-applicationmodel")] fn TryCreateFromAppServiceConnection(&self, connection: *mut super::super::applicationmodel::appservice::AppServiceConnection, out: *mut *mut RemoteSystemConnectionInfo) -> HRESULT
}}
impl IRemoteSystemConnectionInfoStatics {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn try_create_from_app_service_connection(&self, connection: &super::super::applicationmodel::appservice::AppServiceConnection) -> Result<Option<ComPtr<RemoteSystemConnectionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryCreateFromAppServiceConnection)(self as *const _ as *mut _, connection as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemConnectionRequest, 2230141188, 36190, 19826, 130, 56, 118, 33, 87, 108, 122, 103);
RT_INTERFACE!{interface IRemoteSystemConnectionRequest(IRemoteSystemConnectionRequestVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequest] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemConnectionRequest {
    #[inline] pub fn get_remote_system(&self) -> Result<Option<ComPtr<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemConnectionRequest: IRemoteSystemConnectionRequest}
impl RtActivatable<IRemoteSystemConnectionRequestFactory> for RemoteSystemConnectionRequest {}
impl RtActivatable<IRemoteSystemConnectionRequestStatics> for RemoteSystemConnectionRequest {}
impl RemoteSystemConnectionRequest {
    #[inline] pub fn create(remoteSystem: &RemoteSystem) -> Result<ComPtr<RemoteSystemConnectionRequest>> {
        <Self as RtActivatable<IRemoteSystemConnectionRequestFactory>>::get_activation_factory().create(remoteSystem)
    }
    #[inline] pub fn create_for_app(remoteSystemApp: &RemoteSystemApp) -> Result<Option<ComPtr<RemoteSystemConnectionRequest>>> {
        <Self as RtActivatable<IRemoteSystemConnectionRequestStatics>>::get_activation_factory().create_for_app(remoteSystemApp)
    }
}
DEFINE_CLSID!(RemoteSystemConnectionRequest(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,67,111,110,110,101,99,116,105,111,110,82,101,113,117,101,115,116,0]) [CLSID_RemoteSystemConnectionRequest]);
DEFINE_IID!(IID_IRemoteSystemConnectionRequest2, 316632431, 49148, 18490, 138, 190, 211, 74, 108, 25, 249, 43);
RT_INTERFACE!{interface IRemoteSystemConnectionRequest2(IRemoteSystemConnectionRequest2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequest2] {
    fn get_RemoteSystemApp(&self, out: *mut *mut RemoteSystemApp) -> HRESULT
}}
impl IRemoteSystemConnectionRequest2 {
    #[inline] pub fn get_remote_system_app(&self) -> Result<Option<ComPtr<RemoteSystemApp>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystemApp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemConnectionRequestFactory, 2852784672, 47851, 17781, 181, 48, 129, 11, 185, 120, 99, 52);
RT_INTERFACE!{static interface IRemoteSystemConnectionRequestFactory(IRemoteSystemConnectionRequestFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequestFactory] {
    fn Create(&self, remoteSystem: *mut RemoteSystem, out: *mut *mut RemoteSystemConnectionRequest) -> HRESULT
}}
impl IRemoteSystemConnectionRequestFactory {
    #[inline] pub fn create(&self, remoteSystem: &RemoteSystem) -> Result<ComPtr<RemoteSystemConnectionRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemConnectionRequestStatics, 2261390397, 33300, 16988, 137, 50, 219, 73, 3, 45, 19, 6);
RT_INTERFACE!{static interface IRemoteSystemConnectionRequestStatics(IRemoteSystemConnectionRequestStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemConnectionRequestStatics] {
    fn CreateForApp(&self, remoteSystemApp: *mut RemoteSystemApp, out: *mut *mut RemoteSystemConnectionRequest) -> HRESULT
}}
impl IRemoteSystemConnectionRequestStatics {
    #[inline] pub fn create_for_app(&self, remoteSystemApp: &RemoteSystemApp) -> Result<Option<ComPtr<RemoteSystemConnectionRequest>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateForApp)(self as *const _ as *mut _, remoteSystemApp as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemDiscoveryType: i32 {
    Any = 0, Proximal = 1, Cloud = 2, SpatiallyProximal = 3,
}}
DEFINE_IID!(IID_IRemoteSystemDiscoveryTypeFilter, 1121518623, 61018, 17370, 172, 106, 111, 238, 37, 70, 7, 65);
RT_INTERFACE!{interface IRemoteSystemDiscoveryTypeFilter(IRemoteSystemDiscoveryTypeFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemDiscoveryTypeFilter] {
    fn get_RemoteSystemDiscoveryType(&self, out: *mut RemoteSystemDiscoveryType) -> HRESULT
}}
impl IRemoteSystemDiscoveryTypeFilter {
    #[inline] pub fn get_remote_system_discovery_type(&self) -> Result<RemoteSystemDiscoveryType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemDiscoveryType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemDiscoveryTypeFilter: IRemoteSystemDiscoveryTypeFilter}
impl RtActivatable<IRemoteSystemDiscoveryTypeFilterFactory> for RemoteSystemDiscoveryTypeFilter {}
impl RemoteSystemDiscoveryTypeFilter {
    #[inline] pub fn create(discoveryType: RemoteSystemDiscoveryType) -> Result<ComPtr<RemoteSystemDiscoveryTypeFilter>> {
        <Self as RtActivatable<IRemoteSystemDiscoveryTypeFilterFactory>>::get_activation_factory().create(discoveryType)
    }
}
DEFINE_CLSID!(RemoteSystemDiscoveryTypeFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,68,105,115,99,111,118,101,114,121,84,121,112,101,70,105,108,116,101,114,0]) [CLSID_RemoteSystemDiscoveryTypeFilter]);
DEFINE_IID!(IID_IRemoteSystemDiscoveryTypeFilterFactory, 2677979539, 49760, 16737, 146, 242, 156, 2, 31, 35, 254, 93);
RT_INTERFACE!{static interface IRemoteSystemDiscoveryTypeFilterFactory(IRemoteSystemDiscoveryTypeFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemDiscoveryTypeFilterFactory] {
    fn Create(&self, discoveryType: RemoteSystemDiscoveryType, out: *mut *mut RemoteSystemDiscoveryTypeFilter) -> HRESULT
}}
impl IRemoteSystemDiscoveryTypeFilterFactory {
    #[inline] pub fn create(&self, discoveryType: RemoteSystemDiscoveryType) -> Result<ComPtr<RemoteSystemDiscoveryTypeFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, discoveryType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemEnumerationCompletedEventArgs, 3337108831, 16432, 17236, 160, 96, 20, 241, 178, 44, 84, 93);
RT_INTERFACE!{interface IRemoteSystemEnumerationCompletedEventArgs(IRemoteSystemEnumerationCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemEnumerationCompletedEventArgs] {
    
}}
RT_CLASS!{class RemoteSystemEnumerationCompletedEventArgs: IRemoteSystemEnumerationCompletedEventArgs}
DEFINE_IID!(IID_IRemoteSystemFilter, 1245424100, 39403, 17899, 186, 22, 3, 103, 114, 143, 243, 116);
RT_INTERFACE!{interface IRemoteSystemFilter(IRemoteSystemFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemFilter] {
    
}}
DEFINE_IID!(IID_IRemoteSystemKindFilter, 954321388, 8899, 20214, 144, 26, 187, 177, 199, 170, 212, 237);
RT_INTERFACE!{interface IRemoteSystemKindFilter(IRemoteSystemKindFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindFilter] {
    fn get_RemoteSystemKinds(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IRemoteSystemKindFilter {
    #[inline] pub fn get_remote_system_kinds(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystemKinds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemKindFilter: IRemoteSystemKindFilter}
impl RtActivatable<IRemoteSystemKindFilterFactory> for RemoteSystemKindFilter {}
impl RemoteSystemKindFilter {
    #[inline] pub fn create(remoteSystemKinds: &foundation::collections::IIterable<HString>) -> Result<ComPtr<RemoteSystemKindFilter>> {
        <Self as RtActivatable<IRemoteSystemKindFilterFactory>>::get_activation_factory().create(remoteSystemKinds)
    }
}
DEFINE_CLSID!(RemoteSystemKindFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,75,105,110,100,70,105,108,116,101,114,0]) [CLSID_RemoteSystemKindFilter]);
DEFINE_IID!(IID_IRemoteSystemKindFilterFactory, 2717587694, 39402, 16572, 154, 57, 198, 112, 170, 128, 74, 40);
RT_INTERFACE!{static interface IRemoteSystemKindFilterFactory(IRemoteSystemKindFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindFilterFactory] {
    fn Create(&self, remoteSystemKinds: *mut foundation::collections::IIterable<HString>, out: *mut *mut RemoteSystemKindFilter) -> HRESULT
}}
impl IRemoteSystemKindFilterFactory {
    #[inline] pub fn create(&self, remoteSystemKinds: &foundation::collections::IIterable<HString>) -> Result<ComPtr<RemoteSystemKindFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemKinds as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class RemoteSystemKinds}
impl RtActivatable<IRemoteSystemKindStatics> for RemoteSystemKinds {}
impl RtActivatable<IRemoteSystemKindStatics2> for RemoteSystemKinds {}
impl RemoteSystemKinds {
    #[inline] pub fn get_phone() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_phone()
    }
    #[inline] pub fn get_hub() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_hub()
    }
    #[inline] pub fn get_holographic() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_holographic()
    }
    #[inline] pub fn get_desktop() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_desktop()
    }
    #[inline] pub fn get_xbox() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics>>::get_activation_factory().get_xbox()
    }
    #[inline] pub fn get_iot() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_iot()
    }
    #[inline] pub fn get_tablet() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_tablet()
    }
    #[inline] pub fn get_laptop() -> Result<HString> {
        <Self as RtActivatable<IRemoteSystemKindStatics2>>::get_activation_factory().get_laptop()
    }
}
DEFINE_CLSID!(RemoteSystemKinds(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,75,105,110,100,115,0]) [CLSID_RemoteSystemKinds]);
DEFINE_IID!(IID_IRemoteSystemKindStatics, 4130436659, 43796, 16848, 149, 83, 121, 106, 173, 184, 130, 219);
RT_INTERFACE!{static interface IRemoteSystemKindStatics(IRemoteSystemKindStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindStatics] {
    fn get_Phone(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Hub(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Holographic(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Xbox(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemKindStatics {
    #[inline] pub fn get_phone(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Phone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hub(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Hub)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_holographic(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Holographic)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_desktop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Desktop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xbox(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Xbox)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemKindStatics2, 3118703568, 1126, 18249, 145, 232, 101, 249, 209, 154, 150, 165);
RT_INTERFACE!{static interface IRemoteSystemKindStatics2(IRemoteSystemKindStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemKindStatics2] {
    fn get_Iot(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Tablet(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Laptop(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemKindStatics2 {
    #[inline] pub fn get_iot(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Iot)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tablet(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Tablet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_laptop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Laptop)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemPlatform: i32 {
    Unknown = 0, Windows = 1, Android = 2, Ios = 3, Linux = 4,
}}
DEFINE_IID!(IID_IRemoteSystemRemovedEventArgs, 2336036539, 29446, 18922, 183, 223, 103, 213, 113, 76, 176, 19);
RT_INTERFACE!{interface IRemoteSystemRemovedEventArgs(IRemoteSystemRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemRemovedEventArgs] {
    fn get_RemoteSystemId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IRemoteSystemRemovedEventArgs {
    #[inline] pub fn get_remote_system_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemRemovedEventArgs: IRemoteSystemRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSession, 1766287873, 39642, 18703, 149, 73, 211, 28, 177, 76, 158, 149);
RT_INTERFACE!{interface IRemoteSystemSession(IRemoteSystemSessionVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSession] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ControllerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn add_Disconnected(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Disconnected(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateParticipantWatcher(&self, out: *mut *mut RemoteSystemSessionParticipantWatcher) -> HRESULT,
    fn SendInvitationAsync(&self, invitee: *mut RemoteSystem, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IRemoteSystemSession {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_controller_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControllerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_disconnected(&self, handler: &foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Disconnected)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_disconnected(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Disconnected)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_participant_watcher(&self) -> Result<Option<ComPtr<RemoteSystemSessionParticipantWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateParticipantWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn send_invitation_async(&self, invitee: &RemoteSystem) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendInvitationAsync)(self as *const _ as *mut _, invitee as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSession: IRemoteSystemSession}
impl RtActivatable<IRemoteSystemSessionStatics> for RemoteSystemSession {}
impl RemoteSystemSession {
    #[inline] pub fn create_watcher() -> Result<Option<ComPtr<RemoteSystemSessionWatcher>>> {
        <Self as RtActivatable<IRemoteSystemSessionStatics>>::get_activation_factory().create_watcher()
    }
}
DEFINE_CLSID!(RemoteSystemSession(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,0]) [CLSID_RemoteSystemSession]);
DEFINE_IID!(IID_IRemoteSystemSessionAddedEventArgs, 3582318420, 48279, 19513, 153, 180, 190, 202, 118, 224, 76, 63);
RT_INTERFACE!{interface IRemoteSystemSessionAddedEventArgs(IRemoteSystemSessionAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionAddedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionAddedEventArgs {
    #[inline] pub fn get_session_info(&self) -> Result<Option<ComPtr<RemoteSystemSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionAddedEventArgs: IRemoteSystemSessionAddedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionController, 3834326482, 26656, 18535, 180, 37, 216, 156, 10, 62, 247, 186);
RT_INTERFACE!{interface IRemoteSystemSessionController(IRemoteSystemSessionControllerVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionController] {
    fn add_JoinRequested(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_JoinRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn RemoveParticipantAsync(&self, pParticipant: *mut RemoteSystemSessionParticipant, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn CreateSessionAsync(&self, out: *mut *mut foundation::IAsyncOperation<RemoteSystemSessionCreationResult>) -> HRESULT
}}
impl IRemoteSystemSessionController {
    #[inline] pub fn add_join_requested(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_JoinRequested)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_join_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_JoinRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_participant_async(&self, pParticipant: &RemoteSystemSessionParticipant) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RemoveParticipantAsync)(self as *const _ as *mut _, pParticipant as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_session_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateSessionAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionController: IRemoteSystemSessionController}
impl RtActivatable<IRemoteSystemSessionControllerFactory> for RemoteSystemSessionController {}
impl RemoteSystemSessionController {
    #[inline] pub fn create_controller(displayName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionController>> {
        <Self as RtActivatable<IRemoteSystemSessionControllerFactory>>::get_activation_factory().create_controller(displayName)
    }
    #[inline] pub fn create_controller_with_session_options(displayName: &HStringArg, options: &RemoteSystemSessionOptions) -> Result<ComPtr<RemoteSystemSessionController>> {
        <Self as RtActivatable<IRemoteSystemSessionControllerFactory>>::get_activation_factory().create_controller_with_session_options(displayName, options)
    }
}
DEFINE_CLSID!(RemoteSystemSessionController(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,67,111,110,116,114,111,108,108,101,114,0]) [CLSID_RemoteSystemSessionController]);
DEFINE_IID!(IID_IRemoteSystemSessionControllerFactory, 3217829739, 44093, 16793, 130, 205, 102, 112, 167, 115, 239, 46);
RT_INTERFACE!{static interface IRemoteSystemSessionControllerFactory(IRemoteSystemSessionControllerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionControllerFactory] {
    fn CreateController(&self, displayName: HSTRING, out: *mut *mut RemoteSystemSessionController) -> HRESULT,
    fn CreateControllerWithSessionOptions(&self, displayName: HSTRING, options: *mut RemoteSystemSessionOptions, out: *mut *mut RemoteSystemSessionController) -> HRESULT
}}
impl IRemoteSystemSessionControllerFactory {
    #[inline] pub fn create_controller(&self, displayName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionController>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateController)(self as *const _ as *mut _, displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_controller_with_session_options(&self, displayName: &HStringArg, options: &RemoteSystemSessionOptions) -> Result<ComPtr<RemoteSystemSessionController>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateControllerWithSessionOptions)(self as *const _ as *mut _, displayName.get(), options as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemSessionCreationResult, 2811761346, 14302, 17548, 139, 131, 163, 10, 163, 196, 234, 214);
RT_INTERFACE!{interface IRemoteSystemSessionCreationResult(IRemoteSystemSessionCreationResultVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionCreationResult] {
    fn get_Status(&self, out: *mut RemoteSystemSessionCreationStatus) -> HRESULT,
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT
}}
impl IRemoteSystemSessionCreationResult {
    #[inline] pub fn get_status(&self) -> Result<RemoteSystemSessionCreationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_session(&self) -> Result<Option<ComPtr<RemoteSystemSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionCreationResult: IRemoteSystemSessionCreationResult}
RT_ENUM! { enum RemoteSystemSessionCreationStatus: i32 {
    Success = 0, SessionLimitsExceeded = 1, OperationAborted = 2,
}}
DEFINE_IID!(IID_IRemoteSystemSessionDisconnectedEventArgs, 3725313691, 30661, 17948, 130, 9, 124, 108, 93, 49, 17, 171);
RT_INTERFACE!{interface IRemoteSystemSessionDisconnectedEventArgs(IRemoteSystemSessionDisconnectedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionDisconnectedEventArgs] {
    fn get_Reason(&self, out: *mut RemoteSystemSessionDisconnectedReason) -> HRESULT
}}
impl IRemoteSystemSessionDisconnectedEventArgs {
    #[inline] pub fn get_reason(&self) -> Result<RemoteSystemSessionDisconnectedReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Reason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionDisconnectedEventArgs: IRemoteSystemSessionDisconnectedEventArgs}
RT_ENUM! { enum RemoteSystemSessionDisconnectedReason: i32 {
    SessionUnavailable = 0, RemovedByController = 1, SessionClosed = 2,
}}
DEFINE_IID!(IID_IRemoteSystemSessionInfo, 4283299400, 35594, 20122, 153, 5, 105, 228, 184, 65, 197, 136);
RT_INTERFACE!{interface IRemoteSystemSessionInfo(IRemoteSystemSessionInfoVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInfo] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ControllerDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn JoinAsync(&self, out: *mut *mut foundation::IAsyncOperation<RemoteSystemSessionJoinResult>) -> HRESULT
}}
impl IRemoteSystemSessionInfo {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_controller_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ControllerDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn join_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).JoinAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionInfo: IRemoteSystemSessionInfo}
DEFINE_IID!(IID_IRemoteSystemSessionInvitation, 1043516561, 20951, 18278, 161, 33, 37, 81, 108, 59, 130, 148);
RT_INTERFACE!{interface IRemoteSystemSessionInvitation(IRemoteSystemSessionInvitationVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitation] {
    fn get_Sender(&self, out: *mut *mut RemoteSystem) -> HRESULT,
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionInvitation {
    #[inline] pub fn get_sender(&self) -> Result<Option<ComPtr<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sender)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_info(&self) -> Result<Option<ComPtr<RemoteSystemSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionInvitation: IRemoteSystemSessionInvitation}
DEFINE_IID!(IID_IRemoteSystemSessionInvitationListener, 150208575, 48241, 18913, 135, 74, 49, 221, 255, 154, 39, 185);
RT_INTERFACE!{interface IRemoteSystemSessionInvitationListener(IRemoteSystemSessionInvitationListenerVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitationListener] {
    fn add_InvitationReceived(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_InvitationReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionInvitationListener {
    #[inline] pub fn add_invitation_received(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_InvitationReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_invitation_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_InvitationReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionInvitationListener: IRemoteSystemSessionInvitationListener}
impl RtActivatable<IActivationFactory> for RemoteSystemSessionInvitationListener {}
DEFINE_CLSID!(RemoteSystemSessionInvitationListener(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,73,110,118,105,116,97,116,105,111,110,76,105,115,116,101,110,101,114,0]) [CLSID_RemoteSystemSessionInvitationListener]);
DEFINE_IID!(IID_IRemoteSystemSessionInvitationReceivedEventArgs, 1586907693, 41229, 20187, 141, 234, 84, 210, 10, 193, 149, 67);
RT_INTERFACE!{interface IRemoteSystemSessionInvitationReceivedEventArgs(IRemoteSystemSessionInvitationReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionInvitationReceivedEventArgs] {
    fn get_Invitation(&self, out: *mut *mut RemoteSystemSessionInvitation) -> HRESULT
}}
impl IRemoteSystemSessionInvitationReceivedEventArgs {
    #[inline] pub fn get_invitation(&self) -> Result<Option<ComPtr<RemoteSystemSessionInvitation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Invitation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionInvitationReceivedEventArgs: IRemoteSystemSessionInvitationReceivedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionJoinRequest, 543162472, 31124, 17201, 134, 209, 216, 157, 136, 37, 133, 238);
RT_INTERFACE!{interface IRemoteSystemSessionJoinRequest(IRemoteSystemSessionJoinRequestVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinRequest] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT,
    fn Accept(&self) -> HRESULT
}}
impl IRemoteSystemSessionJoinRequest {
    #[inline] pub fn get_participant(&self) -> Result<Option<ComPtr<RemoteSystemSessionParticipant>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn accept(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Accept)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionJoinRequest: IRemoteSystemSessionJoinRequest}
DEFINE_IID!(IID_IRemoteSystemSessionJoinRequestedEventArgs, 3687468995, 33465, 18454, 156, 36, 228, 14, 97, 119, 75, 216);
RT_INTERFACE!{interface IRemoteSystemSessionJoinRequestedEventArgs(IRemoteSystemSessionJoinRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinRequestedEventArgs] {
    fn get_JoinRequest(&self, out: *mut *mut RemoteSystemSessionJoinRequest) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IRemoteSystemSessionJoinRequestedEventArgs {
    #[inline] pub fn get_join_request(&self) -> Result<Option<ComPtr<RemoteSystemSessionJoinRequest>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JoinRequest)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionJoinRequestedEventArgs: IRemoteSystemSessionJoinRequestedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionJoinResult, 3464175364, 41022, 16804, 144, 11, 30, 121, 50, 140, 18, 103);
RT_INTERFACE!{interface IRemoteSystemSessionJoinResult(IRemoteSystemSessionJoinResultVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionJoinResult] {
    fn get_Status(&self, out: *mut RemoteSystemSessionJoinStatus) -> HRESULT,
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT
}}
impl IRemoteSystemSessionJoinResult {
    #[inline] pub fn get_status(&self) -> Result<RemoteSystemSessionJoinStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_session(&self) -> Result<Option<ComPtr<RemoteSystemSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionJoinResult: IRemoteSystemSessionJoinResult}
RT_ENUM! { enum RemoteSystemSessionJoinStatus: i32 {
    Success = 0, SessionLimitsExceeded = 1, OperationAborted = 2, SessionUnavailable = 3, RejectedByController = 4,
}}
DEFINE_IID!(IID_IRemoteSystemSessionMessageChannel, 2502218026, 29657, 19472, 183, 81, 194, 103, 132, 67, 113, 39);
RT_INTERFACE!{interface IRemoteSystemSessionMessageChannel(IRemoteSystemSessionMessageChannelVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionMessageChannel] {
    fn get_Session(&self, out: *mut *mut RemoteSystemSession) -> HRESULT,
    fn BroadcastValueSetAsync(&self, messageData: *mut foundation::collections::ValueSet, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn SendValueSetAsync(&self, messageData: *mut foundation::collections::ValueSet, participant: *mut RemoteSystemSessionParticipant, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn SendValueSetToParticipantsAsync(&self, messageData: *mut foundation::collections::ValueSet, participants: *mut foundation::collections::IIterable<RemoteSystemSessionParticipant>, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn add_ValueSetReceived(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ValueSetReceived(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionMessageChannel {
    #[inline] pub fn get_session(&self) -> Result<Option<ComPtr<RemoteSystemSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Session)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn broadcast_value_set_async(&self, messageData: &foundation::collections::ValueSet) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BroadcastValueSetAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn send_value_set_async(&self, messageData: &foundation::collections::ValueSet, participant: &RemoteSystemSessionParticipant) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendValueSetAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, participant as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn send_value_set_to_participants_async(&self, messageData: &foundation::collections::ValueSet, participants: &foundation::collections::IIterable<RemoteSystemSessionParticipant>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SendValueSetToParticipantsAsync)(self as *const _ as *mut _, messageData as *const _ as *mut _, participants as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_value_set_received(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ValueSetReceived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_value_set_received(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ValueSetReceived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionMessageChannel: IRemoteSystemSessionMessageChannel}
impl RtActivatable<IRemoteSystemSessionMessageChannelFactory> for RemoteSystemSessionMessageChannel {}
impl RemoteSystemSessionMessageChannel {
    #[inline] pub fn create(session: &RemoteSystemSession, channelName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> {
        <Self as RtActivatable<IRemoteSystemSessionMessageChannelFactory>>::get_activation_factory().create(session, channelName)
    }
    #[inline] pub fn create_with_reliability(session: &RemoteSystemSession, channelName: &HStringArg, reliability: RemoteSystemSessionMessageChannelReliability) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> {
        <Self as RtActivatable<IRemoteSystemSessionMessageChannelFactory>>::get_activation_factory().create_with_reliability(session, channelName, reliability)
    }
}
DEFINE_CLSID!(RemoteSystemSessionMessageChannel(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,77,101,115,115,97,103,101,67,104,97,110,110,101,108,0]) [CLSID_RemoteSystemSessionMessageChannel]);
DEFINE_IID!(IID_IRemoteSystemSessionMessageChannelFactory, 694033482, 48406, 17048, 183, 206, 65, 84, 130, 176, 225, 29);
RT_INTERFACE!{static interface IRemoteSystemSessionMessageChannelFactory(IRemoteSystemSessionMessageChannelFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionMessageChannelFactory] {
    fn Create(&self, session: *mut RemoteSystemSession, channelName: HSTRING, out: *mut *mut RemoteSystemSessionMessageChannel) -> HRESULT,
    fn CreateWithReliability(&self, session: *mut RemoteSystemSession, channelName: HSTRING, reliability: RemoteSystemSessionMessageChannelReliability, out: *mut *mut RemoteSystemSessionMessageChannel) -> HRESULT
}}
impl IRemoteSystemSessionMessageChannelFactory {
    #[inline] pub fn create(&self, session: &RemoteSystemSession, channelName: &HStringArg) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, session as *const _ as *mut _, channelName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_reliability(&self, session: &RemoteSystemSession, channelName: &HStringArg, reliability: RemoteSystemSessionMessageChannelReliability) -> Result<ComPtr<RemoteSystemSessionMessageChannel>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithReliability)(self as *const _ as *mut _, session as *const _ as *mut _, channelName.get(), reliability, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemSessionMessageChannelReliability: i32 {
    Reliable = 0, Unreliable = 1,
}}
DEFINE_IID!(IID_IRemoteSystemSessionOptions, 1947129685, 33816, 20225, 147, 83, 226, 28, 158, 204, 108, 252);
RT_INTERFACE!{interface IRemoteSystemSessionOptions(IRemoteSystemSessionOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionOptions] {
    fn get_IsInviteOnly(&self, out: *mut bool) -> HRESULT,
    fn put_IsInviteOnly(&self, value: bool) -> HRESULT
}}
impl IRemoteSystemSessionOptions {
    #[inline] pub fn get_is_invite_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInviteOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_invite_only(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsInviteOnly)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionOptions: IRemoteSystemSessionOptions}
impl RtActivatable<IActivationFactory> for RemoteSystemSessionOptions {}
DEFINE_CLSID!(RemoteSystemSessionOptions(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,101,115,115,105,111,110,79,112,116,105,111,110,115,0]) [CLSID_RemoteSystemSessionOptions]);
DEFINE_IID!(IID_IRemoteSystemSessionParticipant, 2123367820, 44281, 18217, 138, 23, 68, 231, 186, 237, 93, 204);
RT_INTERFACE!{interface IRemoteSystemSessionParticipant(IRemoteSystemSessionParticipantVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipant] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT,
    #[cfg(feature="windows-networking")] fn GetHostNames(&self, out: *mut *mut foundation::collections::IVectorView<super::super::networking::HostName>) -> HRESULT
}}
impl IRemoteSystemSessionParticipant {
    #[inline] pub fn get_remote_system(&self) -> Result<Option<ComPtr<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_host_names(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<super::super::networking::HostName>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetHostNames)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionParticipant: IRemoteSystemSessionParticipant}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantAddedEventArgs, 3545913304, 51617, 19383, 182, 176, 121, 187, 145, 173, 249, 61);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantAddedEventArgs(IRemoteSystemSessionParticipantAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantAddedEventArgs] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT
}}
impl IRemoteSystemSessionParticipantAddedEventArgs {
    #[inline] pub fn get_participant(&self) -> Result<Option<ComPtr<RemoteSystemSessionParticipant>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionParticipantAddedEventArgs: IRemoteSystemSessionParticipantAddedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantRemovedEventArgs, 2255417480, 56936, 19135, 136, 161, 249, 13, 22, 39, 65, 146);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantRemovedEventArgs(IRemoteSystemSessionParticipantRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantRemovedEventArgs] {
    fn get_Participant(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT
}}
impl IRemoteSystemSessionParticipantRemovedEventArgs {
    #[inline] pub fn get_participant(&self) -> Result<Option<ComPtr<RemoteSystemSessionParticipant>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Participant)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionParticipantRemovedEventArgs: IRemoteSystemSessionParticipantRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionParticipantWatcher, 3705471692, 43655, 19833, 182, 204, 68, 89, 179, 233, 32, 117);
RT_INTERFACE!{interface IRemoteSystemSessionParticipantWatcher(IRemoteSystemSessionParticipantWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionParticipantWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemSessionParticipantWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionParticipantWatcher {
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<RemoteSystemSessionParticipantWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionParticipantWatcher: IRemoteSystemSessionParticipantWatcher}
RT_ENUM! { enum RemoteSystemSessionParticipantWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
DEFINE_IID!(IID_IRemoteSystemSessionRemovedEventArgs, 2944569678, 14753, 19946, 157, 99, 67, 121, 141, 91, 187, 208);
RT_INTERFACE!{interface IRemoteSystemSessionRemovedEventArgs(IRemoteSystemSessionRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionRemovedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionRemovedEventArgs {
    #[inline] pub fn get_session_info(&self) -> Result<Option<ComPtr<RemoteSystemSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionRemovedEventArgs: IRemoteSystemSessionRemovedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionStatics, 2233764255, 64800, 17635, 149, 101, 231, 90, 59, 20, 198, 110);
RT_INTERFACE!{static interface IRemoteSystemSessionStatics(IRemoteSystemSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionStatics] {
    fn CreateWatcher(&self, out: *mut *mut RemoteSystemSessionWatcher) -> HRESULT
}}
impl IRemoteSystemSessionStatics {
    #[inline] pub fn create_watcher(&self) -> Result<Option<ComPtr<RemoteSystemSessionWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemSessionUpdatedEventArgs, 377966697, 8990, 19601, 142, 200, 179, 163, 157, 158, 85, 163);
RT_INTERFACE!{interface IRemoteSystemSessionUpdatedEventArgs(IRemoteSystemSessionUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionUpdatedEventArgs] {
    fn get_SessionInfo(&self, out: *mut *mut RemoteSystemSessionInfo) -> HRESULT
}}
impl IRemoteSystemSessionUpdatedEventArgs {
    #[inline] pub fn get_session_info(&self) -> Result<Option<ComPtr<RemoteSystemSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionUpdatedEventArgs: IRemoteSystemSessionUpdatedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionValueSetReceivedEventArgs, 116594565, 11685, 20056, 167, 143, 158, 141, 7, 132, 238, 37);
RT_INTERFACE!{interface IRemoteSystemSessionValueSetReceivedEventArgs(IRemoteSystemSessionValueSetReceivedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionValueSetReceivedEventArgs] {
    fn get_Sender(&self, out: *mut *mut RemoteSystemSessionParticipant) -> HRESULT,
    fn get_Message(&self, out: *mut *mut foundation::collections::ValueSet) -> HRESULT
}}
impl IRemoteSystemSessionValueSetReceivedEventArgs {
    #[inline] pub fn get_sender(&self) -> Result<Option<ComPtr<RemoteSystemSessionParticipant>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Sender)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<Option<ComPtr<foundation::collections::ValueSet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Message)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionValueSetReceivedEventArgs: IRemoteSystemSessionValueSetReceivedEventArgs}
DEFINE_IID!(IID_IRemoteSystemSessionWatcher, 2147738432, 3137, 19042, 182, 215, 189, 190, 43, 25, 190, 45);
RT_INTERFACE!{interface IRemoteSystemSessionWatcher(IRemoteSystemSessionWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemSessionWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn get_Status(&self, out: *mut RemoteSystemSessionWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemSessionWatcher {
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<RemoteSystemSessionWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Added)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Added)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Updated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Updated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Removed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Removed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemSessionWatcher: IRemoteSystemSessionWatcher}
RT_ENUM! { enum RemoteSystemSessionWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
DEFINE_IID!(IID_IRemoteSystemStatics, 2760225682, 65323, 19271, 190, 98, 116, 63, 47, 20, 15, 48);
RT_INTERFACE!{static interface IRemoteSystemStatics(IRemoteSystemStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatics] {
    #[cfg(not(feature="windows-networking"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-networking")] fn FindByHostNameAsync(&self, hostName: *mut super::super::networking::HostName, out: *mut *mut foundation::IAsyncOperation<RemoteSystem>) -> HRESULT,
    fn CreateWatcher(&self, out: *mut *mut RemoteSystemWatcher) -> HRESULT,
    fn CreateWatcherWithFilters(&self, filters: *mut foundation::collections::IIterable<IRemoteSystemFilter>, out: *mut *mut RemoteSystemWatcher) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut foundation::IAsyncOperation<RemoteSystemAccessStatus>) -> HRESULT
}}
impl IRemoteSystemStatics {
    #[cfg(feature="windows-networking")] #[inline] pub fn find_by_host_name_async(&self, hostName: &super::super::networking::HostName) -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FindByHostNameAsync)(self as *const _ as *mut _, hostName as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_watcher(&self) -> Result<Option<ComPtr<RemoteSystemWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcher)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_watcher_with_filters(&self, filters: &foundation::collections::IIterable<IRemoteSystemFilter>) -> Result<Option<ComPtr<RemoteSystemWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWatcherWithFilters)(self as *const _ as *mut _, filters as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<RemoteSystemAccessStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestAccessAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemStatics2, 211348938, 28569, 19538, 162, 114, 234, 79, 54, 71, 23, 68);
RT_INTERFACE!{static interface IRemoteSystemStatics2(IRemoteSystemStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatics2] {
    fn IsAuthorizationKindEnabled(&self, kind: RemoteSystemAuthorizationKind, out: *mut bool) -> HRESULT
}}
impl IRemoteSystemStatics2 {
    #[inline] pub fn is_authorization_kind_enabled(&self, kind: RemoteSystemAuthorizationKind) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsAuthorizationKindEnabled)(self as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemStatus: i32 {
    Unavailable = 0, DiscoveringAvailability = 1, Available = 2, Unknown = 3,
}}
RT_ENUM! { enum RemoteSystemStatusType: i32 {
    Any = 0, Available = 1,
}}
DEFINE_IID!(IID_IRemoteSystemStatusTypeFilter, 205082958, 52150, 18295, 133, 52, 46, 12, 82, 26, 255, 162);
RT_INTERFACE!{interface IRemoteSystemStatusTypeFilter(IRemoteSystemStatusTypeFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatusTypeFilter] {
    fn get_RemoteSystemStatusType(&self, out: *mut RemoteSystemStatusType) -> HRESULT
}}
impl IRemoteSystemStatusTypeFilter {
    #[inline] pub fn get_remote_system_status_type(&self) -> Result<RemoteSystemStatusType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RemoteSystemStatusType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemStatusTypeFilter: IRemoteSystemStatusTypeFilter}
impl RtActivatable<IRemoteSystemStatusTypeFilterFactory> for RemoteSystemStatusTypeFilter {}
impl RemoteSystemStatusTypeFilter {
    #[inline] pub fn create(remoteSystemStatusType: RemoteSystemStatusType) -> Result<ComPtr<RemoteSystemStatusTypeFilter>> {
        <Self as RtActivatable<IRemoteSystemStatusTypeFilterFactory>>::get_activation_factory().create(remoteSystemStatusType)
    }
}
DEFINE_CLSID!(RemoteSystemStatusTypeFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,83,116,97,116,117,115,84,121,112,101,70,105,108,116,101,114,0]) [CLSID_RemoteSystemStatusTypeFilter]);
DEFINE_IID!(IID_IRemoteSystemStatusTypeFilterFactory, 869234938, 55076, 16677, 172, 122, 141, 40, 30, 68, 201, 73);
RT_INTERFACE!{static interface IRemoteSystemStatusTypeFilterFactory(IRemoteSystemStatusTypeFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemStatusTypeFilterFactory] {
    fn Create(&self, remoteSystemStatusType: RemoteSystemStatusType, out: *mut *mut RemoteSystemStatusTypeFilter) -> HRESULT
}}
impl IRemoteSystemStatusTypeFilterFactory {
    #[inline] pub fn create(&self, remoteSystemStatusType: RemoteSystemStatusType) -> Result<ComPtr<RemoteSystemStatusTypeFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, remoteSystemStatusType, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRemoteSystemUpdatedEventArgs, 1963130638, 56267, 16725, 180, 202, 179, 10, 4, 242, 118, 39);
RT_INTERFACE!{interface IRemoteSystemUpdatedEventArgs(IRemoteSystemUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemUpdatedEventArgs] {
    fn get_RemoteSystem(&self, out: *mut *mut RemoteSystem) -> HRESULT
}}
impl IRemoteSystemUpdatedEventArgs {
    #[inline] pub fn get_remote_system(&self) -> Result<Option<ComPtr<RemoteSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemoteSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemUpdatedEventArgs: IRemoteSystemUpdatedEventArgs}
DEFINE_IID!(IID_IRemoteSystemWatcher, 1566575742, 11271, 18629, 136, 156, 69, 93, 43, 9, 151, 113);
RT_INTERFACE!{interface IRemoteSystemWatcher(IRemoteSystemWatcherVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWatcher] {
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT,
    fn add_RemoteSystemAdded(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemAdded(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RemoteSystemUpdated(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemUpdated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_RemoteSystemRemoved(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RemoteSystemRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemWatcher {
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Stop)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_remote_system_added(&self, handler: &foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemAdded)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_remote_system_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RemoteSystemAdded)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_remote_system_updated(&self, handler: &foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemUpdated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_remote_system_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RemoteSystemUpdated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_remote_system_removed(&self, handler: &foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_RemoteSystemRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_remote_system_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_RemoteSystemRemoved)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemWatcher: IRemoteSystemWatcher}
DEFINE_IID!(IID_IRemoteSystemWatcher2, 1933797120, 6602, 18681, 164, 205, 120, 15, 122, 213, 140, 113);
RT_INTERFACE!{interface IRemoteSystemWatcher2(IRemoteSystemWatcher2Vtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWatcher2] {
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ErrorOccurred(&self, handler: *mut foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ErrorOccurred(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IRemoteSystemWatcher2 {
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_EnumerationCompleted)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_EnumerationCompleted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_error_occurred(&self, handler: &foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ErrorOccurred)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_error_occurred(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ErrorOccurred)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemoteSystemWatcherError: i32 {
    Unknown = 0, InternetNotAvailable = 1, AuthenticationError = 2,
}}
DEFINE_IID!(IID_IRemoteSystemWatcherErrorOccurredEventArgs, 1959118511, 20756, 17446, 146, 22, 32, 216, 31, 133, 25, 174);
RT_INTERFACE!{interface IRemoteSystemWatcherErrorOccurredEventArgs(IRemoteSystemWatcherErrorOccurredEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWatcherErrorOccurredEventArgs] {
    fn get_Error(&self, out: *mut RemoteSystemWatcherError) -> HRESULT
}}
impl IRemoteSystemWatcherErrorOccurredEventArgs {
    #[inline] pub fn get_error(&self) -> Result<RemoteSystemWatcherError> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Error)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemWatcherErrorOccurredEventArgs: IRemoteSystemWatcherErrorOccurredEventArgs}
DEFINE_IID!(IID_IRemoteSystemWebAccountFilter, 1068980339, 34760, 23951, 151, 126, 246, 159, 150, 214, 114, 56);
RT_INTERFACE!{interface IRemoteSystemWebAccountFilter(IRemoteSystemWebAccountFilterVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWebAccountFilter] {
    #[cfg(feature="windows-security")] fn get_Account(&self, out: *mut *mut super::super::security::credentials::WebAccount) -> HRESULT
}}
impl IRemoteSystemWebAccountFilter {
    #[cfg(feature="windows-security")] #[inline] pub fn get_account(&self) -> Result<Option<ComPtr<super::super::security::credentials::WebAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Account)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RemoteSystemWebAccountFilter: IRemoteSystemWebAccountFilter}
impl RtActivatable<IRemoteSystemWebAccountFilterFactory> for RemoteSystemWebAccountFilter {}
impl RemoteSystemWebAccountFilter {
    #[cfg(feature="windows-security")] #[inline] pub fn create(account: &super::super::security::credentials::WebAccount) -> Result<ComPtr<RemoteSystemWebAccountFilter>> {
        <Self as RtActivatable<IRemoteSystemWebAccountFilterFactory>>::get_activation_factory().create(account)
    }
}
DEFINE_CLSID!(RemoteSystemWebAccountFilter(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,82,101,109,111,116,101,83,121,115,116,101,109,115,46,82,101,109,111,116,101,83,121,115,116,101,109,87,101,98,65,99,99,111,117,110,116,70,105,108,116,101,114,0]) [CLSID_RemoteSystemWebAccountFilter]);
DEFINE_IID!(IID_IRemoteSystemWebAccountFilterFactory, 881469193, 24397, 20775, 180, 167, 191, 153, 213, 37, 43, 27);
RT_INTERFACE!{static interface IRemoteSystemWebAccountFilterFactory(IRemoteSystemWebAccountFilterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IRemoteSystemWebAccountFilterFactory] {
    #[cfg(feature="windows-security")] fn Create(&self, account: *mut super::super::security::credentials::WebAccount, out: *mut *mut RemoteSystemWebAccountFilter) -> HRESULT
}}
impl IRemoteSystemWebAccountFilterFactory {
    #[cfg(feature="windows-security")] #[inline] pub fn create(&self, account: &super::super::security::credentials::WebAccount) -> Result<ComPtr<RemoteSystemWebAccountFilter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, account as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.System.RemoteSystems
pub mod threading { // Windows.System.Threading
use crate::prelude::*;
RT_CLASS!{static class ThreadPool}
impl RtActivatable<IThreadPoolStatics> for ThreadPool {}
impl ThreadPool {
    #[inline] pub fn run_async(handler: &WorkItemHandler) -> Result<ComPtr<foundation::IAsyncAction>> {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_async(handler)
    }
    #[inline] pub fn run_with_priority_async(handler: &WorkItemHandler, priority: WorkItemPriority) -> Result<ComPtr<foundation::IAsyncAction>> {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_with_priority_async(handler, priority)
    }
    #[inline] pub fn run_with_priority_and_options_async(handler: &WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions) -> Result<ComPtr<foundation::IAsyncAction>> {
        <Self as RtActivatable<IThreadPoolStatics>>::get_activation_factory().run_with_priority_and_options_async(handler, priority, options)
    }
}
DEFINE_CLSID!(ThreadPool(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,84,104,114,101,97,100,80,111,111,108,0]) [CLSID_ThreadPool]);
DEFINE_IID!(IID_IThreadPoolStatics, 3065997277, 33981, 17656, 172, 28, 147, 235, 203, 157, 186, 145);
RT_INTERFACE!{static interface IThreadPoolStatics(IThreadPoolStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolStatics] {
    fn RunAsync(&self, handler: *mut WorkItemHandler, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn RunWithPriorityAsync(&self, handler: *mut WorkItemHandler, priority: WorkItemPriority, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn RunWithPriorityAndOptionsAsync(&self, handler: *mut WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IThreadPoolStatics {
    #[inline] pub fn run_async(&self, handler: &WorkItemHandler) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn run_with_priority_async(&self, handler: &WorkItemHandler, priority: WorkItemPriority) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunWithPriorityAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn run_with_priority_and_options_async(&self, handler: &WorkItemHandler, priority: WorkItemPriority, options: WorkItemOptions) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunWithPriorityAndOptionsAsync)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IThreadPoolTimer, 1498332792, 21994, 19080, 165, 13, 52, 2, 174, 31, 156, 242);
RT_INTERFACE!{interface IThreadPoolTimer(IThreadPoolTimerVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolTimer] {
    fn get_Period(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Delay(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn Cancel(&self) -> HRESULT
}}
impl IThreadPoolTimer {
    #[inline] pub fn get_period(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Period)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_delay(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Delay)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn cancel(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Cancel)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ThreadPoolTimer: IThreadPoolTimer}
impl RtActivatable<IThreadPoolTimerStatics> for ThreadPoolTimer {}
impl ThreadPoolTimer {
    #[inline] pub fn create_periodic_timer(handler: &TimerElapsedHandler, period: foundation::TimeSpan) -> Result<Option<ComPtr<ThreadPoolTimer>>> {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_periodic_timer(handler, period)
    }
    #[inline] pub fn create_timer(handler: &TimerElapsedHandler, delay: foundation::TimeSpan) -> Result<Option<ComPtr<ThreadPoolTimer>>> {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_timer(handler, delay)
    }
    #[inline] pub fn create_periodic_timer_with_completion(handler: &TimerElapsedHandler, period: foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<Option<ComPtr<ThreadPoolTimer>>> {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_periodic_timer_with_completion(handler, period, destroyed)
    }
    #[inline] pub fn create_timer_with_completion(handler: &TimerElapsedHandler, delay: foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<Option<ComPtr<ThreadPoolTimer>>> {
        <Self as RtActivatable<IThreadPoolTimerStatics>>::get_activation_factory().create_timer_with_completion(handler, delay, destroyed)
    }
}
DEFINE_CLSID!(ThreadPoolTimer(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,84,104,114,101,97,100,80,111,111,108,84,105,109,101,114,0]) [CLSID_ThreadPoolTimer]);
DEFINE_IID!(IID_IThreadPoolTimerStatics, 445291778, 58498, 17947, 184, 199, 142, 250, 209, 204, 229, 144);
RT_INTERFACE!{static interface IThreadPoolTimerStatics(IThreadPoolTimerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IThreadPoolTimerStatics] {
    fn CreatePeriodicTimer(&self, handler: *mut TimerElapsedHandler, period: foundation::TimeSpan, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreateTimer(&self, handler: *mut TimerElapsedHandler, delay: foundation::TimeSpan, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreatePeriodicTimerWithCompletion(&self, handler: *mut TimerElapsedHandler, period: foundation::TimeSpan, destroyed: *mut TimerDestroyedHandler, out: *mut *mut ThreadPoolTimer) -> HRESULT,
    fn CreateTimerWithCompletion(&self, handler: *mut TimerElapsedHandler, delay: foundation::TimeSpan, destroyed: *mut TimerDestroyedHandler, out: *mut *mut ThreadPoolTimer) -> HRESULT
}}
impl IThreadPoolTimerStatics {
    #[inline] pub fn create_periodic_timer(&self, handler: &TimerElapsedHandler, period: foundation::TimeSpan) -> Result<Option<ComPtr<ThreadPoolTimer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePeriodicTimer)(self as *const _ as *mut _, handler as *const _ as *mut _, period, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_timer(&self, handler: &TimerElapsedHandler, delay: foundation::TimeSpan) -> Result<Option<ComPtr<ThreadPoolTimer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimer)(self as *const _ as *mut _, handler as *const _ as *mut _, delay, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_periodic_timer_with_completion(&self, handler: &TimerElapsedHandler, period: foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<Option<ComPtr<ThreadPoolTimer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePeriodicTimerWithCompletion)(self as *const _ as *mut _, handler as *const _ as *mut _, period, destroyed as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_timer_with_completion(&self, handler: &TimerElapsedHandler, delay: foundation::TimeSpan, destroyed: &TimerDestroyedHandler) -> Result<Option<ComPtr<ThreadPoolTimer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTimerWithCompletion)(self as *const _ as *mut _, handler as *const _ as *mut _, delay, destroyed as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_TimerDestroyedHandler, 887953914, 33668, 20153, 130, 9, 251, 80, 148, 238, 236, 53);
RT_DELEGATE!{delegate TimerDestroyedHandler(TimerDestroyedHandlerVtbl, TimerDestroyedHandlerImpl) [IID_TimerDestroyedHandler] {
    fn Invoke(&self, timer: *mut ThreadPoolTimer) -> HRESULT
}}
impl TimerDestroyedHandler {
    #[inline] pub fn invoke(&self, timer: &ThreadPoolTimer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, timer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_TimerElapsedHandler, 4205749863, 64491, 18891, 173, 178, 113, 24, 76, 85, 110, 67);
RT_DELEGATE!{delegate TimerElapsedHandler(TimerElapsedHandlerVtbl, TimerElapsedHandlerImpl) [IID_TimerElapsedHandler] {
    fn Invoke(&self, timer: *mut ThreadPoolTimer) -> HRESULT
}}
impl TimerElapsedHandler {
    #[inline] pub fn invoke(&self, timer: &ThreadPoolTimer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, timer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_WorkItemHandler, 488278923, 64102, 16719, 156, 189, 182, 95, 201, 157, 23, 250);
RT_DELEGATE!{delegate WorkItemHandler(WorkItemHandlerVtbl, WorkItemHandlerImpl) [IID_WorkItemHandler] {
    fn Invoke(&self, operation: *mut foundation::IAsyncAction) -> HRESULT
}}
impl WorkItemHandler {
    #[inline] pub fn invoke(&self, operation: &foundation::IAsyncAction) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, operation as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum WorkItemOptions: u32 {
    None = 0, TimeSliced = 1,
}}
RT_ENUM! { enum WorkItemPriority: i32 {
    Low = -1, Normal = 0, High = 1,
}}
pub mod core { // Windows.System.Threading.Core
use crate::prelude::*;
DEFINE_IID!(IID_IPreallocatedWorkItem, 3067783676, 48219, 16410, 168, 178, 110, 117, 77, 20, 218, 166);
RT_INTERFACE!{interface IPreallocatedWorkItem(IPreallocatedWorkItemVtbl): IInspectable(IInspectableVtbl) [IID_IPreallocatedWorkItem] {
    fn RunAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IPreallocatedWorkItem {
    #[inline] pub fn run_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RunAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PreallocatedWorkItem: IPreallocatedWorkItem}
impl RtActivatable<IPreallocatedWorkItemFactory> for PreallocatedWorkItem {}
impl PreallocatedWorkItem {
    #[inline] pub fn create_work_item(handler: &super::WorkItemHandler) -> Result<ComPtr<PreallocatedWorkItem>> {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item(handler)
    }
    #[inline] pub fn create_work_item_with_priority(handler: &super::WorkItemHandler, priority: super::WorkItemPriority) -> Result<ComPtr<PreallocatedWorkItem>> {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item_with_priority(handler, priority)
    }
    #[inline] pub fn create_work_item_with_priority_and_options(handler: &super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> Result<ComPtr<PreallocatedWorkItem>> {
        <Self as RtActivatable<IPreallocatedWorkItemFactory>>::get_activation_factory().create_work_item_with_priority_and_options(handler, priority, options)
    }
}
DEFINE_CLSID!(PreallocatedWorkItem(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,67,111,114,101,46,80,114,101,97,108,108,111,99,97,116,101,100,87,111,114,107,73,116,101,109,0]) [CLSID_PreallocatedWorkItem]);
DEFINE_IID!(IID_IPreallocatedWorkItemFactory, 3822267205, 57322, 18075, 130, 197, 246, 227, 206, 253, 234, 251);
RT_INTERFACE!{static interface IPreallocatedWorkItemFactory(IPreallocatedWorkItemFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPreallocatedWorkItemFactory] {
    fn CreateWorkItem(&self, handler: *mut super::WorkItemHandler, out: *mut *mut PreallocatedWorkItem) -> HRESULT,
    fn CreateWorkItemWithPriority(&self, handler: *mut super::WorkItemHandler, priority: super::WorkItemPriority, out: *mut *mut PreallocatedWorkItem) -> HRESULT,
    fn CreateWorkItemWithPriorityAndOptions(&self, handler: *mut super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions, out: *mut *mut PreallocatedWorkItem) -> HRESULT
}}
impl IPreallocatedWorkItemFactory {
    #[inline] pub fn create_work_item(&self, handler: &super::WorkItemHandler) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItem)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_work_item_with_priority(&self, handler: &super::WorkItemHandler, priority: super::WorkItemPriority) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItemWithPriority)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_work_item_with_priority_and_options(&self, handler: &super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> Result<ComPtr<PreallocatedWorkItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWorkItemWithPriorityAndOptions)(self as *const _ as *mut _, handler as *const _ as *mut _, priority, options, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_SignalHandler, 2453422126, 18209, 17422, 157, 218, 85, 182, 242, 224, 119, 16);
RT_DELEGATE!{delegate SignalHandler(SignalHandlerVtbl, SignalHandlerImpl) [IID_SignalHandler] {
    fn Invoke(&self, signalNotifier: *mut SignalNotifier, timedOut: bool) -> HRESULT
}}
impl SignalHandler {
    #[inline] pub fn invoke(&self, signalNotifier: &SignalNotifier, timedOut: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, signalNotifier as *const _ as *mut _, timedOut);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISignalNotifier, 338189830, 25511, 18195, 182, 217, 98, 246, 75, 86, 251, 139);
RT_INTERFACE!{interface ISignalNotifier(ISignalNotifierVtbl): IInspectable(IInspectableVtbl) [IID_ISignalNotifier] {
    fn Enable(&self) -> HRESULT,
    fn Terminate(&self) -> HRESULT
}}
impl ISignalNotifier {
    #[inline] pub fn enable(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Enable)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn terminate(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Terminate)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SignalNotifier: ISignalNotifier}
impl RtActivatable<ISignalNotifierStatics> for SignalNotifier {}
impl SignalNotifier {
    #[inline] pub fn attach_to_event(name: &HStringArg, handler: &SignalHandler) -> Result<Option<ComPtr<SignalNotifier>>> {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_event(name, handler)
    }
    #[inline] pub fn attach_to_event_with_timeout(name: &HStringArg, handler: &SignalHandler, timeout: foundation::TimeSpan) -> Result<Option<ComPtr<SignalNotifier>>> {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_event_with_timeout(name, handler, timeout)
    }
    #[inline] pub fn attach_to_semaphore(name: &HStringArg, handler: &SignalHandler) -> Result<Option<ComPtr<SignalNotifier>>> {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_semaphore(name, handler)
    }
    #[inline] pub fn attach_to_semaphore_with_timeout(name: &HStringArg, handler: &SignalHandler, timeout: foundation::TimeSpan) -> Result<Option<ComPtr<SignalNotifier>>> {
        <Self as RtActivatable<ISignalNotifierStatics>>::get_activation_factory().attach_to_semaphore_with_timeout(name, handler, timeout)
    }
}
DEFINE_CLSID!(SignalNotifier(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,84,104,114,101,97,100,105,110,103,46,67,111,114,101,46,83,105,103,110,97,108,78,111,116,105,102,105,101,114,0]) [CLSID_SignalNotifier]);
DEFINE_IID!(IID_ISignalNotifierStatics, 474891622, 33792, 18131, 161, 21, 125, 12, 13, 252, 159, 98);
RT_INTERFACE!{static interface ISignalNotifierStatics(ISignalNotifierStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISignalNotifierStatics] {
    fn AttachToEvent(&self, name: HSTRING, handler: *mut SignalHandler, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToEventWithTimeout(&self, name: HSTRING, handler: *mut SignalHandler, timeout: foundation::TimeSpan, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToSemaphore(&self, name: HSTRING, handler: *mut SignalHandler, out: *mut *mut SignalNotifier) -> HRESULT,
    fn AttachToSemaphoreWithTimeout(&self, name: HSTRING, handler: *mut SignalHandler, timeout: foundation::TimeSpan, out: *mut *mut SignalNotifier) -> HRESULT
}}
impl ISignalNotifierStatics {
    #[inline] pub fn attach_to_event(&self, name: &HStringArg, handler: &SignalHandler) -> Result<Option<ComPtr<SignalNotifier>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToEvent)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn attach_to_event_with_timeout(&self, name: &HStringArg, handler: &SignalHandler, timeout: foundation::TimeSpan) -> Result<Option<ComPtr<SignalNotifier>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToEventWithTimeout)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, timeout, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn attach_to_semaphore(&self, name: &HStringArg, handler: &SignalHandler) -> Result<Option<ComPtr<SignalNotifier>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToSemaphore)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn attach_to_semaphore_with_timeout(&self, name: &HStringArg, handler: &SignalHandler, timeout: foundation::TimeSpan) -> Result<Option<ComPtr<SignalNotifier>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AttachToSemaphoreWithTimeout)(self as *const _ as *mut _, name.get(), handler as *const _ as *mut _, timeout, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.System.Threading.Core
} // Windows.System.Threading
pub mod update { // Windows.System.Update
use crate::prelude::*;
RT_ENUM! { enum SystemUpdateAttentionRequiredReason: i32 {
    None = 0, NetworkRequired = 1, InsufficientDiskSpace = 2, InsufficientBattery = 3, UpdateBlocked = 4,
}}
DEFINE_IID!(IID_ISystemUpdateItem, 2006401259, 22052, 20894, 168, 226, 9, 233, 23, 59, 63, 183);
RT_INTERFACE!{interface ISystemUpdateItem(ISystemUpdateItemVtbl): IInspectable(IInspectableVtbl) [IID_ISystemUpdateItem] {
    fn get_State(&self, out: *mut SystemUpdateItemState) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Revision(&self, out: *mut u32) -> HRESULT,
    fn get_DownloadProgress(&self, out: *mut f64) -> HRESULT,
    fn get_InstallProgress(&self, out: *mut f64) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl ISystemUpdateItem {
    #[inline] pub fn get_state(&self) -> Result<SystemUpdateItemState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Title)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_revision(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Revision)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_download_progress(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DownloadProgress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_install_progress(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InstallProgress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemUpdateItem: ISystemUpdateItem}
RT_ENUM! { enum SystemUpdateItemState: i32 {
    NotStarted = 0, Initializing = 1, Preparing = 2, Calculating = 3, Downloading = 4, Installing = 5, Completed = 6, RebootRequired = 7, Error = 8,
}}
DEFINE_IID!(IID_ISystemUpdateLastErrorInfo, 2129168375, 35396, 23406, 189, 7, 122, 236, 228, 17, 110, 169);
RT_INTERFACE!{interface ISystemUpdateLastErrorInfo(ISystemUpdateLastErrorInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISystemUpdateLastErrorInfo] {
    fn get_State(&self, out: *mut SystemUpdateManagerState) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_IsInteractive(&self, out: *mut bool) -> HRESULT
}}
impl ISystemUpdateLastErrorInfo {
    #[inline] pub fn get_state(&self) -> Result<SystemUpdateManagerState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_interactive(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsInteractive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemUpdateLastErrorInfo: ISystemUpdateLastErrorInfo}
RT_CLASS!{static class SystemUpdateManager}
impl RtActivatable<ISystemUpdateManagerStatics> for SystemUpdateManager {}
impl SystemUpdateManager {
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().is_supported()
    }
    #[inline] pub fn get_state() -> Result<SystemUpdateManagerState> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_state()
    }
    #[inline] pub fn add_state_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().add_state_changed(handler)
    }
    #[inline] pub fn remove_state_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().remove_state_changed(token)
    }
    #[inline] pub fn get_download_progress() -> Result<f64> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_download_progress()
    }
    #[inline] pub fn get_install_progress() -> Result<f64> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_install_progress()
    }
    #[inline] pub fn get_user_active_hours_start() -> Result<foundation::TimeSpan> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_user_active_hours_start()
    }
    #[inline] pub fn get_user_active_hours_end() -> Result<foundation::TimeSpan> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_user_active_hours_end()
    }
    #[inline] pub fn get_user_active_hours_max() -> Result<i32> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_user_active_hours_max()
    }
    #[inline] pub fn try_set_user_active_hours(start: foundation::TimeSpan, end: foundation::TimeSpan) -> Result<bool> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().try_set_user_active_hours(start, end)
    }
    #[inline] pub fn get_last_update_check_time() -> Result<foundation::DateTime> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_last_update_check_time()
    }
    #[inline] pub fn get_last_update_install_time() -> Result<foundation::DateTime> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_last_update_install_time()
    }
    #[inline] pub fn get_last_error_info() -> Result<Option<ComPtr<SystemUpdateLastErrorInfo>>> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_last_error_info()
    }
    #[inline] pub fn get_automatic_reboot_block_ids() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_automatic_reboot_block_ids()
    }
    #[inline] pub fn block_automatic_reboot_async(lockId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().block_automatic_reboot_async(lockId)
    }
    #[inline] pub fn unblock_automatic_reboot_async(lockId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().unblock_automatic_reboot_async(lockId)
    }
    #[inline] pub fn get_extended_error() -> Result<foundation::HResult> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_extended_error()
    }
    #[inline] pub fn get_update_items() -> Result<Option<ComPtr<foundation::collections::IVectorView<SystemUpdateItem>>>> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_update_items()
    }
    #[inline] pub fn get_attention_required_reason() -> Result<SystemUpdateAttentionRequiredReason> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_attention_required_reason()
    }
    #[inline] pub fn set_flight_ring(flightRing: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().set_flight_ring(flightRing)
    }
    #[inline] pub fn get_flight_ring() -> Result<HString> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().get_flight_ring()
    }
    #[inline] pub fn start_install(action: SystemUpdateStartInstallAction) -> Result<()> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().start_install(action)
    }
    #[inline] pub fn reboot_to_complete_install() -> Result<()> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().reboot_to_complete_install()
    }
    #[inline] pub fn start_cancel_updates() -> Result<()> {
        <Self as RtActivatable<ISystemUpdateManagerStatics>>::get_activation_factory().start_cancel_updates()
    }
}
DEFINE_CLSID!(SystemUpdateManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,112,100,97,116,101,46,83,121,115,116,101,109,85,112,100,97,116,101,77,97,110,97,103,101,114,0]) [CLSID_SystemUpdateManager]);
RT_ENUM! { enum SystemUpdateManagerState: i32 {
    Idle = 0, Detecting = 1, ReadyToDownload = 2, Downloading = 3, ReadyToInstall = 4, Installing = 5, RebootRequired = 6, ReadyToFinalize = 7, Finalizing = 8, Completed = 9, AttentionRequired = 10, Error = 11,
}}
DEFINE_IID!(IID_ISystemUpdateManagerStatics, 3000237295, 10609, 20926, 180, 26, 139, 215, 3, 187, 112, 26);
RT_INTERFACE!{static interface ISystemUpdateManagerStatics(ISystemUpdateManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISystemUpdateManagerStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT,
    fn get_State(&self, out: *mut SystemUpdateManagerState) -> HRESULT,
    fn add_StateChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StateChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_DownloadProgress(&self, out: *mut f64) -> HRESULT,
    fn get_InstallProgress(&self, out: *mut f64) -> HRESULT,
    fn get_UserActiveHoursStart(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_UserActiveHoursEnd(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_UserActiveHoursMax(&self, out: *mut i32) -> HRESULT,
    fn TrySetUserActiveHours(&self, start: foundation::TimeSpan, end: foundation::TimeSpan, out: *mut bool) -> HRESULT,
    fn get_LastUpdateCheckTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_LastUpdateInstallTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_LastErrorInfo(&self, out: *mut *mut SystemUpdateLastErrorInfo) -> HRESULT,
    fn GetAutomaticRebootBlockIds(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn BlockAutomaticRebootAsync(&self, lockId: HSTRING, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn UnblockAutomaticRebootAsync(&self, lockId: HSTRING, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn GetUpdateItems(&self, out: *mut *mut foundation::collections::IVectorView<SystemUpdateItem>) -> HRESULT,
    fn get_AttentionRequiredReason(&self, out: *mut SystemUpdateAttentionRequiredReason) -> HRESULT,
    fn SetFlightRing(&self, flightRing: HSTRING, out: *mut bool) -> HRESULT,
    fn GetFlightRing(&self, out: *mut HSTRING) -> HRESULT,
    fn StartInstall(&self, action: SystemUpdateStartInstallAction) -> HRESULT,
    fn RebootToCompleteInstall(&self) -> HRESULT,
    fn StartCancelUpdates(&self) -> HRESULT
}}
impl ISystemUpdateManagerStatics {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<SystemUpdateManagerState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_state_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StateChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_state_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_StateChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_download_progress(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DownloadProgress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_install_progress(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InstallProgress)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user_active_hours_start(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserActiveHoursStart)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user_active_hours_end(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserActiveHoursEnd)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user_active_hours_max(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserActiveHoursMax)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_set_user_active_hours(&self, start: foundation::TimeSpan, end: foundation::TimeSpan) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetUserActiveHours)(self as *const _ as *mut _, start, end, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_update_check_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastUpdateCheckTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_update_install_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastUpdateInstallTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_error_info(&self) -> Result<Option<ComPtr<SystemUpdateLastErrorInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LastErrorInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_automatic_reboot_block_ids(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAutomaticRebootBlockIds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn block_automatic_reboot_async(&self, lockId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).BlockAutomaticRebootAsync)(self as *const _ as *mut _, lockId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn unblock_automatic_reboot_async(&self, lockId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).UnblockAutomaticRebootAsync)(self as *const _ as *mut _, lockId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_update_items(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<SystemUpdateItem>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUpdateItems)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attention_required_reason(&self) -> Result<SystemUpdateAttentionRequiredReason> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AttentionRequiredReason)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_flight_ring(&self, flightRing: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).SetFlightRing)(self as *const _ as *mut _, flightRing.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_flight_ring(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFlightRing)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_install(&self, action: SystemUpdateStartInstallAction) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartInstall)(self as *const _ as *mut _, action);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn reboot_to_complete_install(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RebootToCompleteInstall)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_cancel_updates(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartCancelUpdates)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum SystemUpdateStartInstallAction: i32 {
    UpToReboot = 0, AllowReboot = 1,
}}
} // Windows.System.Update
pub mod userprofile { // Windows.System.UserProfile
use crate::prelude::*;
RT_ENUM! { enum AccountPictureKind: i32 {
    SmallImage = 0, LargeImage = 1, Video = 2,
}}
RT_CLASS!{static class AdvertisingManager}
impl RtActivatable<IAdvertisingManagerStatics> for AdvertisingManager {}
impl RtActivatable<IAdvertisingManagerStatics2> for AdvertisingManager {}
impl AdvertisingManager {
    #[inline] pub fn get_advertising_id() -> Result<HString> {
        <Self as RtActivatable<IAdvertisingManagerStatics>>::get_activation_factory().get_advertising_id()
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<AdvertisingManagerForUser>>> {
        <Self as RtActivatable<IAdvertisingManagerStatics2>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(AdvertisingManager(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,65,100,118,101,114,116,105,115,105,110,103,77,97,110,97,103,101,114,0]) [CLSID_AdvertisingManager]);
DEFINE_IID!(IID_IAdvertisingManagerForUser, 2458645456, 53116, 19120, 167, 220, 109, 197, 188, 212, 66, 82);
RT_INTERFACE!{interface IAdvertisingManagerForUser(IAdvertisingManagerForUserVtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerForUser] {
    fn get_AdvertisingId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT
}}
impl IAdvertisingManagerForUser {
    #[inline] pub fn get_advertising_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdvertisingId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AdvertisingManagerForUser: IAdvertisingManagerForUser}
DEFINE_IID!(IID_IAdvertisingManagerStatics, 2916304524, 41587, 18635, 179, 70, 53, 68, 82, 45, 85, 129);
RT_INTERFACE!{static interface IAdvertisingManagerStatics(IAdvertisingManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerStatics] {
    fn get_AdvertisingId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAdvertisingManagerStatics {
    #[inline] pub fn get_advertising_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AdvertisingId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAdvertisingManagerStatics2, 3708372911, 6765, 18096, 149, 188, 243, 249, 214, 190, 185, 251);
RT_INTERFACE!{static interface IAdvertisingManagerStatics2(IAdvertisingManagerStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IAdvertisingManagerStatics2] {
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut AdvertisingManagerForUser) -> HRESULT
}}
impl IAdvertisingManagerStatics2 {
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<AdvertisingManagerForUser>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAssignedAccessSettings, 465927964, 59761, 22359, 184, 224, 81, 47, 139, 140, 70, 210);
RT_INTERFACE!{interface IAssignedAccessSettings(IAssignedAccessSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IAssignedAccessSettings] {
    fn get_IsEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_IsSingleAppKioskMode(&self, out: *mut bool) -> HRESULT,
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT
}}
impl IAssignedAccessSettings {
    #[inline] pub fn get_is_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_single_app_kiosk_mode(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSingleAppKioskMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AssignedAccessSettings: IAssignedAccessSettings}
impl RtActivatable<IAssignedAccessSettingsStatics> for AssignedAccessSettings {}
impl AssignedAccessSettings {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<AssignedAccessSettings>>> {
        <Self as RtActivatable<IAssignedAccessSettingsStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<AssignedAccessSettings>>> {
        <Self as RtActivatable<IAssignedAccessSettingsStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(AssignedAccessSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,65,115,115,105,103,110,101,100,65,99,99,101,115,115,83,101,116,116,105,110,103,115,0]) [CLSID_AssignedAccessSettings]);
DEFINE_IID!(IID_IAssignedAccessSettingsStatics, 883432717, 35369, 24307, 167, 190, 97, 142, 106, 195, 189, 1);
RT_INTERFACE!{static interface IAssignedAccessSettingsStatics(IAssignedAccessSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAssignedAccessSettingsStatics] {
    fn GetDefault(&self, out: *mut *mut AssignedAccessSettings) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut AssignedAccessSettings) -> HRESULT
}}
impl IAssignedAccessSettingsStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<AssignedAccessSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<AssignedAccessSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDiagnosticsSettings, 3857312973, 10001, 17632, 151, 60, 73, 29, 120, 4, 141, 36);
RT_INTERFACE!{interface IDiagnosticsSettings(IDiagnosticsSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticsSettings] {
    fn get_CanUseDiagnosticsToTailorExperiences(&self, out: *mut bool) -> HRESULT,
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT
}}
impl IDiagnosticsSettings {
    #[inline] pub fn get_can_use_diagnostics_to_tailor_experiences(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanUseDiagnosticsToTailorExperiences)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DiagnosticsSettings: IDiagnosticsSettings}
impl RtActivatable<IDiagnosticsSettingsStatics> for DiagnosticsSettings {}
impl DiagnosticsSettings {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<DiagnosticsSettings>>> {
        <Self as RtActivatable<IDiagnosticsSettingsStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<DiagnosticsSettings>>> {
        <Self as RtActivatable<IDiagnosticsSettingsStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(DiagnosticsSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,68,105,97,103,110,111,115,116,105,99,115,83,101,116,116,105,110,103,115,0]) [CLSID_DiagnosticsSettings]);
DEFINE_IID!(IID_IDiagnosticsSettingsStatics, 1926424591, 21392, 18323, 153, 11, 60, 204, 125, 106, 201, 200);
RT_INTERFACE!{static interface IDiagnosticsSettingsStatics(IDiagnosticsSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDiagnosticsSettingsStatics] {
    fn GetDefault(&self, out: *mut *mut DiagnosticsSettings) -> HRESULT,
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut DiagnosticsSettings) -> HRESULT
}}
impl IDiagnosticsSettingsStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<DiagnosticsSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<DiagnosticsSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFirstSignInSettings, 1049907539, 14942, 17710, 166, 1, 245, 186, 173, 42, 72, 112);
RT_INTERFACE!{interface IFirstSignInSettings(IFirstSignInSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IFirstSignInSettings] {
    
}}
RT_CLASS!{class FirstSignInSettings: IFirstSignInSettings}
impl RtActivatable<IFirstSignInSettingsStatics> for FirstSignInSettings {}
impl FirstSignInSettings {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<FirstSignInSettings>>> {
        <Self as RtActivatable<IFirstSignInSettingsStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(FirstSignInSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,70,105,114,115,116,83,105,103,110,73,110,83,101,116,116,105,110,103,115,0]) [CLSID_FirstSignInSettings]);
DEFINE_IID!(IID_IFirstSignInSettingsStatics, 484544271, 7233, 20128, 183, 162, 111, 12, 28, 126, 132, 56);
RT_INTERFACE!{static interface IFirstSignInSettingsStatics(IFirstSignInSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IFirstSignInSettingsStatics] {
    fn GetDefault(&self, out: *mut *mut FirstSignInSettings) -> HRESULT
}}
impl IFirstSignInSettingsStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<FirstSignInSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class GlobalizationPreferences}
impl RtActivatable<IGlobalizationPreferencesStatics> for GlobalizationPreferences {}
impl RtActivatable<IGlobalizationPreferencesStatics2> for GlobalizationPreferences {}
impl RtActivatable<IGlobalizationPreferencesStatics3> for GlobalizationPreferences {}
impl GlobalizationPreferences {
    #[inline] pub fn get_calendars() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_calendars()
    }
    #[inline] pub fn get_clocks() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_clocks()
    }
    #[inline] pub fn get_currencies() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_currencies()
    }
    #[inline] pub fn get_languages() -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_languages()
    }
    #[inline] pub fn get_home_geographic_region() -> Result<HString> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_home_geographic_region()
    }
    #[cfg(feature="windows-globalization")] #[inline] pub fn get_week_starts_on() -> Result<super::super::globalization::DayOfWeek> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics>>::get_activation_factory().get_week_starts_on()
    }
    #[inline] pub fn try_set_home_geographic_region(region: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics2>>::get_activation_factory().try_set_home_geographic_region(region)
    }
    #[inline] pub fn try_set_languages(languageTags: &foundation::collections::IIterable<HString>) -> Result<bool> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics2>>::get_activation_factory().try_set_languages(languageTags)
    }
    #[inline] pub fn get_for_user(user: &super::User) -> Result<Option<ComPtr<GlobalizationPreferencesForUser>>> {
        <Self as RtActivatable<IGlobalizationPreferencesStatics3>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(GlobalizationPreferences(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,71,108,111,98,97,108,105,122,97,116,105,111,110,80,114,101,102,101,114,101,110,99,101,115,0]) [CLSID_GlobalizationPreferences]);
DEFINE_IID!(IID_IGlobalizationPreferencesForUser, 353306517, 20334, 16570, 160, 16, 226, 125, 129, 189, 167, 245);
RT_INTERFACE!{interface IGlobalizationPreferencesForUser(IGlobalizationPreferencesForUserVtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesForUser] {
    fn get_User(&self, out: *mut *mut super::User) -> HRESULT,
    fn get_Calendars(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Clocks(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Currencies(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Languages(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_HomeGeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-globalization")] fn get_WeekStartsOn(&self, out: *mut super::super::globalization::DayOfWeek) -> HRESULT
}}
impl IGlobalizationPreferencesForUser {
    #[inline] pub fn get_user(&self) -> Result<Option<ComPtr<super::User>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_User)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_calendars(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Calendars)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_clocks(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Clocks)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_currencies(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Currencies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_languages(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_home_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeGeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-globalization")] #[inline] pub fn get_week_starts_on(&self) -> Result<super::super::globalization::DayOfWeek> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WeekStartsOn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class GlobalizationPreferencesForUser: IGlobalizationPreferencesForUser}
DEFINE_IID!(IID_IGlobalizationPreferencesStatics, 29311782, 60727, 20118, 176, 233, 193, 52, 13, 30, 161, 88);
RT_INTERFACE!{static interface IGlobalizationPreferencesStatics(IGlobalizationPreferencesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesStatics] {
    fn get_Calendars(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Clocks(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Currencies(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_Languages(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_HomeGeographicRegion(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-globalization")] fn get_WeekStartsOn(&self, out: *mut super::super::globalization::DayOfWeek) -> HRESULT
}}
impl IGlobalizationPreferencesStatics {
    #[inline] pub fn get_calendars(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Calendars)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_clocks(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Clocks)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_currencies(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Currencies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_languages(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Languages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_home_geographic_region(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HomeGeographicRegion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-globalization")] #[inline] pub fn get_week_starts_on(&self) -> Result<super::super::globalization::DayOfWeek> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WeekStartsOn)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGlobalizationPreferencesStatics2, 4241393137, 17152, 19664, 156, 172, 26, 142, 123, 126, 24, 244);
RT_INTERFACE!{static interface IGlobalizationPreferencesStatics2(IGlobalizationPreferencesStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesStatics2] {
    fn TrySetHomeGeographicRegion(&self, region: HSTRING, out: *mut bool) -> HRESULT,
    fn TrySetLanguages(&self, languageTags: *mut foundation::collections::IIterable<HString>, out: *mut bool) -> HRESULT
}}
impl IGlobalizationPreferencesStatics2 {
    #[inline] pub fn try_set_home_geographic_region(&self, region: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetHomeGeographicRegion)(self as *const _ as *mut _, region.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_set_languages(&self, languageTags: &foundation::collections::IIterable<HString>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetLanguages)(self as *const _ as *mut _, languageTags as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGlobalizationPreferencesStatics3, 503682867, 13813, 16600, 185, 232, 174, 243, 239, 133, 111, 206);
RT_INTERFACE!{static interface IGlobalizationPreferencesStatics3(IGlobalizationPreferencesStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IGlobalizationPreferencesStatics3] {
    fn GetForUser(&self, user: *mut super::User, out: *mut *mut GlobalizationPreferencesForUser) -> HRESULT
}}
impl IGlobalizationPreferencesStatics3 {
    #[inline] pub fn get_for_user(&self, user: &super::User) -> Result<Option<ComPtr<GlobalizationPreferencesForUser>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForUser)(self as *const _ as *mut _, user as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class LockScreen}
impl RtActivatable<ILockScreenImageFeedStatics> for LockScreen {}
impl RtActivatable<ILockScreenStatics> for LockScreen {}
impl LockScreen {
    #[inline] pub fn request_set_image_feed_async(syndicationFeedUri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<SetImageFeedResult>>> {
        <Self as RtActivatable<ILockScreenImageFeedStatics>>::get_activation_factory().request_set_image_feed_async(syndicationFeedUri)
    }
    #[inline] pub fn try_remove_image_feed() -> Result<bool> {
        <Self as RtActivatable<ILockScreenImageFeedStatics>>::get_activation_factory().try_remove_image_feed()
    }
    #[inline] pub fn get_original_image_file() -> Result<Option<ComPtr<foundation::Uri>>> {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().get_original_image_file()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_image_stream() -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStream>>> {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().get_image_stream()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_file_async(value: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncAction>> {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().set_image_file_async(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_stream_async(value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncAction>> {
        <Self as RtActivatable<ILockScreenStatics>>::get_activation_factory().set_image_stream_async(value)
    }
}
DEFINE_CLSID!(LockScreen(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,76,111,99,107,83,99,114,101,101,110,0]) [CLSID_LockScreen]);
DEFINE_IID!(IID_ILockScreenImageFeedStatics, 739079158, 937, 16806, 155, 1, 73, 82, 81, 255, 81, 213);
RT_INTERFACE!{static interface ILockScreenImageFeedStatics(ILockScreenImageFeedStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILockScreenImageFeedStatics] {
    fn RequestSetImageFeedAsync(&self, syndicationFeedUri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<SetImageFeedResult>) -> HRESULT,
    fn TryRemoveImageFeed(&self, out: *mut bool) -> HRESULT
}}
impl ILockScreenImageFeedStatics {
    #[inline] pub fn request_set_image_feed_async(&self, syndicationFeedUri: &foundation::Uri) -> Result<ComPtr<foundation::IAsyncOperation<SetImageFeedResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSetImageFeedAsync)(self as *const _ as *mut _, syndicationFeedUri as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_remove_image_feed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TryRemoveImageFeed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILockScreenStatics, 1055511469, 46599, 16558, 180, 38, 118, 49, 217, 130, 18, 105);
RT_INTERFACE!{static interface ILockScreenStatics(ILockScreenStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILockScreenStatics] {
    fn get_OriginalImageFile(&self, out: *mut *mut foundation::Uri) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetImageStream(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SetImageFileAsync(&self, value: *mut super::super::storage::IStorageFile, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SetImageStreamAsync(&self, value: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ILockScreenStatics {
    #[inline] pub fn get_original_image_file(&self) -> Result<Option<ComPtr<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OriginalImageFile)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_image_stream(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetImageStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_file_async(&self, value: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetImageFileAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_image_stream_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetImageStreamAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SetAccountPictureResult: i32 {
    Success = 0, ChangeDisabled = 1, LargeOrDynamicError = 2, VideoFrameSizeError = 3, FileSizeError = 4, Failure = 5,
}}
RT_ENUM! { enum SetImageFeedResult: i32 {
    Success = 0, ChangeDisabled = 1, UserCanceled = 2,
}}
RT_CLASS!{static class UserInformation}
impl RtActivatable<IUserInformationStatics> for UserInformation {}
impl UserInformation {
    #[inline] pub fn get_account_picture_change_enabled() -> Result<bool> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_account_picture_change_enabled()
    }
    #[inline] pub fn get_name_access_allowed() -> Result<bool> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_name_access_allowed()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_account_picture(kind: AccountPictureKind) -> Result<Option<ComPtr<super::super::storage::IStorageFile>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_account_picture(kind)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_async(image: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_picture_async(image)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_async(smallImage: &super::super::storage::IStorageFile, largeImage: &super::super::storage::IStorageFile, video: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_pictures_async(smallImage, largeImage, video)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_from_stream_async(image: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_picture_from_stream_async(image)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_from_streams_async(smallImage: &super::super::storage::streams::IRandomAccessStream, largeImage: &super::super::storage::streams::IRandomAccessStream, video: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().set_account_pictures_from_streams_async(smallImage, largeImage, video)
    }
    #[inline] pub fn add_account_picture_changed(changeHandler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().add_account_picture_changed(changeHandler)
    }
    #[inline] pub fn remove_account_picture_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().remove_account_picture_changed(token)
    }
    #[inline] pub fn get_display_name_async() -> Result<ComPtr<foundation::IAsyncOperation<HString>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_display_name_async()
    }
    #[inline] pub fn get_first_name_async() -> Result<ComPtr<foundation::IAsyncOperation<HString>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_first_name_async()
    }
    #[inline] pub fn get_last_name_async() -> Result<ComPtr<foundation::IAsyncOperation<HString>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_last_name_async()
    }
    #[inline] pub fn get_principal_name_async() -> Result<ComPtr<foundation::IAsyncOperation<HString>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_principal_name_async()
    }
    #[inline] pub fn get_session_initiation_protocol_uri_async() -> Result<ComPtr<foundation::IAsyncOperation<foundation::Uri>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_session_initiation_protocol_uri_async()
    }
    #[inline] pub fn get_domain_name_async() -> Result<ComPtr<foundation::IAsyncOperation<HString>>> {
        <Self as RtActivatable<IUserInformationStatics>>::get_activation_factory().get_domain_name_async()
    }
}
DEFINE_CLSID!(UserInformation(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,85,115,101,114,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_UserInformation]);
DEFINE_IID!(IID_IUserInformationStatics, 2012457232, 18682, 18588, 147, 78, 42, 232, 91, 168, 247, 114);
RT_INTERFACE!{static interface IUserInformationStatics(IUserInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserInformationStatics] {
    fn get_AccountPictureChangeEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_NameAccessAllowed(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetAccountPicture(&self, kind: AccountPictureKind, out: *mut *mut super::super::storage::IStorageFile) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPictureAsync(&self, image: *mut super::super::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPicturesAsync(&self, smallImage: *mut super::super::storage::IStorageFile, largeImage: *mut super::super::storage::IStorageFile, video: *mut super::super::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPictureFromStreamAsync(&self, image: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetAccountPicturesFromStreamsAsync(&self, smallImage: *mut super::super::storage::streams::IRandomAccessStream, largeImage: *mut super::super::storage::streams::IRandomAccessStream, video: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<SetAccountPictureResult>) -> HRESULT,
    fn add_AccountPictureChanged(&self, changeHandler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AccountPictureChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn GetDisplayNameAsync(&self, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetFirstNameAsync(&self, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetLastNameAsync(&self, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetPrincipalNameAsync(&self, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT,
    fn GetSessionInitiationProtocolUriAsync(&self, out: *mut *mut foundation::IAsyncOperation<foundation::Uri>) -> HRESULT,
    fn GetDomainNameAsync(&self, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT
}}
impl IUserInformationStatics {
    #[inline] pub fn get_account_picture_change_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AccountPictureChangeEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_name_access_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NameAccessAllowed)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_account_picture(&self, kind: AccountPictureKind) -> Result<Option<ComPtr<super::super::storage::IStorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAccountPicture)(self as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_async(&self, image: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPictureAsync)(self as *const _ as *mut _, image as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_async(&self, smallImage: &super::super::storage::IStorageFile, largeImage: &super::super::storage::IStorageFile, video: &super::super::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPicturesAsync)(self as *const _ as *mut _, smallImage as *const _ as *mut _, largeImage as *const _ as *mut _, video as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_picture_from_stream_async(&self, image: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPictureFromStreamAsync)(self as *const _ as *mut _, image as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_account_pictures_from_streams_async(&self, smallImage: &super::super::storage::streams::IRandomAccessStream, largeImage: &super::super::storage::streams::IRandomAccessStream, video: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<SetAccountPictureResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetAccountPicturesFromStreamsAsync)(self as *const _ as *mut _, smallImage as *const _ as *mut _, largeImage as *const _ as *mut _, video as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_account_picture_changed(&self, changeHandler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AccountPictureChanged)(self as *const _ as *mut _, changeHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_account_picture_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AccountPictureChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDisplayNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_name_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFirstNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_name_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetLastNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_principal_name_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPrincipalNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_initiation_protocol_uri_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<foundation::Uri>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSessionInitiationProtocolUriAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain_name_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDomainNameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUserProfilePersonalizationSettings, 2364398260, 31128, 18133, 141, 211, 24, 79, 28, 95, 154, 185);
RT_INTERFACE!{interface IUserProfilePersonalizationSettings(IUserProfilePersonalizationSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IUserProfilePersonalizationSettings] {
    #[cfg(feature="windows-storage")] fn TrySetLockScreenImageAsync(&self, imageFile: *mut super::super::storage::StorageFile, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn TrySetWallpaperImageAsync(&self, imageFile: *mut super::super::storage::StorageFile, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IUserProfilePersonalizationSettings {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_set_lock_screen_image_async(&self, imageFile: &super::super::storage::StorageFile) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TrySetLockScreenImageAsync)(self as *const _ as *mut _, imageFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_set_wallpaper_image_async(&self, imageFile: &super::super::storage::StorageFile) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TrySetWallpaperImageAsync)(self as *const _ as *mut _, imageFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserProfilePersonalizationSettings: IUserProfilePersonalizationSettings}
impl RtActivatable<IUserProfilePersonalizationSettingsStatics> for UserProfilePersonalizationSettings {}
impl UserProfilePersonalizationSettings {
    #[inline] pub fn get_current() -> Result<Option<ComPtr<UserProfilePersonalizationSettings>>> {
        <Self as RtActivatable<IUserProfilePersonalizationSettingsStatics>>::get_activation_factory().get_current()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IUserProfilePersonalizationSettingsStatics>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(UserProfilePersonalizationSettings(&[87,105,110,100,111,119,115,46,83,121,115,116,101,109,46,85,115,101,114,80,114,111,102,105,108,101,46,85,115,101,114,80,114,111,102,105,108,101,80,101,114,115,111,110,97,108,105,122,97,116,105,111,110,83,101,116,116,105,110,103,115,0]) [CLSID_UserProfilePersonalizationSettings]);
DEFINE_IID!(IID_IUserProfilePersonalizationSettingsStatics, 2444015681, 20535, 17739, 152, 131, 187, 119, 45, 8, 221, 22);
RT_INTERFACE!{static interface IUserProfilePersonalizationSettingsStatics(IUserProfilePersonalizationSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUserProfilePersonalizationSettingsStatics] {
    fn get_Current(&self, out: *mut *mut UserProfilePersonalizationSettings) -> HRESULT,
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IUserProfilePersonalizationSettingsStatics {
    #[inline] pub fn get_current(&self) -> Result<Option<ComPtr<UserProfilePersonalizationSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Current)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.System.UserProfile
