use crate::prelude::*;
DEFINE_IID!(IID_IMdmAlert, 2969289511, 10433, 19282, 165, 72, 197, 128, 124, 175, 112, 182);
RT_INTERFACE!{interface IMdmAlert(IMdmAlertVtbl, IMdmAlert_Abi): IInspectable(IInspectableVtbl) [IID_IMdmAlert] {
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT,
    fn get_Format(&self, out: *mut MdmAlertDataType) -> HRESULT,
    fn put_Format(&self, value: MdmAlertDataType) -> HRESULT,
    fn get_Mark(&self, out: *mut MdmAlertMark) -> HRESULT,
    fn put_Mark(&self, value: MdmAlertMark) -> HRESULT,
    fn get_Source(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Source(&self, value: HSTRING) -> HRESULT,
    fn get_Status(&self, out: *mut u32) -> HRESULT,
    fn get_Target(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Target(&self, value: HSTRING) -> HRESULT,
    fn get_Type(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Type(&self, value: HSTRING) -> HRESULT
}}
impl IMdmAlert {
    #[inline] pub fn get_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Data)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Data)(self.0.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_format(&self) -> Result<MdmAlertDataType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_Format)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_format(&self, value: MdmAlertDataType) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Format)(self.0.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_mark(&self) -> Result<MdmAlertMark> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_Mark)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_mark(&self, value: MdmAlertMark) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Mark)(self.0.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Source)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Source)(self.0.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_Status)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_target(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Target)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_target(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Target)(self.0.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Type)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_Type)(self.0.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MdmAlert: IMdmAlert}
impl RtActivatable<IActivationFactory> for MdmAlert {}
DEFINE_CLSID!(MdmAlert(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,77,100,109,65,108,101,114,116,0]) [CLSID_MdmAlert]);
RT_ENUM! { enum MdmAlertDataType: i32 {
    String = 0, Base64 = 1, Boolean = 2, Integer = 3,
}}
RT_ENUM! { enum MdmAlertMark: i32 {
    None = 0, Fatal = 1, Critical = 2, Warning = 3, Informational = 4,
}}
DEFINE_IID!(IID_IMdmSession, 4270403916, 36708, 18327, 169, 215, 157, 136, 248, 106, 225, 102);
RT_INTERFACE!{interface IMdmSession(IMdmSessionVtbl, IMdmSession_Abi): IInspectable(IInspectableVtbl) [IID_IMdmSession] {
    fn get_Alerts(&self, out: *mut <foundation::collections::IVectorView<MdmAlert> as RtType>::Abi) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut MdmSessionState) -> HRESULT,
    fn AttachAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn Delete(&self) -> HRESULT,
    fn StartAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn StartWithAlertsAsync(&self, alerts: <foundation::collections::IIterable<MdmAlert> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IMdmSession {
    #[inline] pub fn get_alerts(&self) -> Result<Option<foundation::collections::IVectorView<MdmAlert>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Alerts)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_ExtendedError)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Id)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<MdmSessionState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_State)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn attach_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AttachAsync)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).Delete)(self.0.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StartAsync)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn start_with_alerts_async(&self, alerts: &foundation::collections::IIterable<MdmAlert>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StartWithAlertsAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(alerts) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MdmSession: IMdmSession}
RT_CLASS!{static class MdmSessionManager}
impl RtActivatable<IMdmSessionManagerStatics> for MdmSessionManager {}
impl MdmSessionManager {
    #[inline] pub fn get_session_ids() -> Result<Option<foundation::collections::IVectorView<HString>>> {
        <Self as RtActivatable<IMdmSessionManagerStatics>>::get_activation_factory().get_session_ids()
    }
    #[inline] pub fn try_create_session() -> Result<Option<MdmSession>> {
        <Self as RtActivatable<IMdmSessionManagerStatics>>::get_activation_factory().try_create_session()
    }
    #[inline] pub fn delete_session_by_id(sessionId: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IMdmSessionManagerStatics>>::get_activation_factory().delete_session_by_id(sessionId)
    }
    #[inline] pub fn get_session_by_id(sessionId: &HStringArg) -> Result<Option<MdmSession>> {
        <Self as RtActivatable<IMdmSessionManagerStatics>>::get_activation_factory().get_session_by_id(sessionId)
    }
}
DEFINE_CLSID!(MdmSessionManager(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,77,100,109,83,101,115,115,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_MdmSessionManager]);
DEFINE_IID!(IID_IMdmSessionManagerStatics, 3477789017, 63301, 19321, 155, 92, 222, 11, 248, 239, 228, 75);
RT_INTERFACE!{static interface IMdmSessionManagerStatics(IMdmSessionManagerStaticsVtbl, IMdmSessionManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IMdmSessionManagerStatics] {
    fn get_SessionIds(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn TryCreateSession(&self, out: *mut <MdmSession as RtType>::Abi) -> HRESULT,
    fn DeleteSessionById(&self, sessionId: HSTRING) -> HRESULT,
    fn GetSessionById(&self, sessionId: HSTRING, out: *mut <MdmSession as RtType>::Abi) -> HRESULT
}}
impl IMdmSessionManagerStatics {
    #[inline] pub fn get_session_ids(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_SessionIds)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_session(&self) -> Result<Option<MdmSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).TryCreateSession)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(MdmSession::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_session_by_id(&self, sessionId: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).DeleteSessionById)(self.0.as_abi() as *const _ as *mut _, sessionId.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_session_by_id(&self, sessionId: &HStringArg) -> Result<Option<MdmSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetSessionById)(self.0.as_abi() as *const _ as *mut _, sessionId.get(), &mut out);
        if hr == S_OK { Ok(MdmSession::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum MdmSessionState: i32 {
    NotStarted = 0, Starting = 1, Connecting = 2, Communicating = 3, AlertStatusAvailable = 4, Retrying = 5, Completed = 6,
}}
pub mod core { // Windows.Management.Core
use crate::prelude::*;
DEFINE_IID!(IID_IApplicationDataManager, 1959855154, 11929, 16384, 154, 58, 100, 48, 126, 133, 129, 41);
RT_INTERFACE!{interface IApplicationDataManager(IApplicationDataManagerVtbl, IApplicationDataManager_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationDataManager] {
    
}}
RT_CLASS!{class ApplicationDataManager: IApplicationDataManager}
impl RtActivatable<IApplicationDataManagerStatics> for ApplicationDataManager {}
impl ApplicationDataManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_package_family(packageFamilyName: &HStringArg) -> Result<Option<super::super::storage::ApplicationData>> {
        <Self as RtActivatable<IApplicationDataManagerStatics>>::get_activation_factory().create_for_package_family(packageFamilyName)
    }
}
DEFINE_CLSID!(ApplicationDataManager(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,67,111,114,101,46,65,112,112,108,105,99,97,116,105,111,110,68,97,116,97,77,97,110,97,103,101,114,0]) [CLSID_ApplicationDataManager]);
DEFINE_IID!(IID_IApplicationDataManagerStatics, 504914659, 27022, 18849, 151, 82, 222, 233, 73, 37, 185, 179);
RT_INTERFACE!{static interface IApplicationDataManagerStatics(IApplicationDataManagerStaticsVtbl, IApplicationDataManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationDataManagerStatics] {
    #[cfg(feature="windows-storage")] fn CreateForPackageFamily(&self, packageFamilyName: HSTRING, out: *mut <super::super::storage::ApplicationData as RtType>::Abi) -> HRESULT
}}
impl IApplicationDataManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_package_family(&self, packageFamilyName: &HStringArg) -> Result<Option<super::super::storage::ApplicationData>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).CreateForPackageFamily)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(super::super::storage::ApplicationData::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Management.Core
pub mod deployment { // Windows.Management.Deployment
use crate::prelude::*;
RT_ENUM! { enum AddPackageByAppInstallerOptions: u32 {
    None = 0, InstallAllResources = 32, ForceTargetAppShutdown = 64, RequiredContentGroupOnly = 256,
}}
RT_ENUM! { enum DeploymentOptions: u32 {
    None = 0, ForceApplicationShutdown = 1, DevelopmentMode = 2, InstallAllResources = 32, ForceTargetApplicationShutdown = 64, RequiredContentGroupOnly = 256, ForceUpdateFromAnyVersion = 262144,
}}
RT_STRUCT! { struct DeploymentProgress {
    state: DeploymentProgressState, percentage: u32,
}}
RT_ENUM! { enum DeploymentProgressState: i32 {
    Queued = 0, Processing = 1,
}}
DEFINE_IID!(IID_IDeploymentResult, 627292590, 46973, 19487, 138, 123, 32, 230, 173, 81, 94, 243);
RT_INTERFACE!{interface IDeploymentResult(IDeploymentResultVtbl, IDeploymentResult_Abi): IInspectable(IInspectableVtbl) [IID_IDeploymentResult] {
    fn get_ErrorText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ActivityId(&self, out: *mut Guid) -> HRESULT,
    fn get_ExtendedErrorCode(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl IDeploymentResult {
    #[inline] pub fn get_error_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_ErrorText)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_activity_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_ActivityId)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error_code(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_ExtendedErrorCode)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class DeploymentResult: IDeploymentResult}
DEFINE_IID!(IID_IDeploymentResult2, 4228804956, 23041, 19415, 188, 241, 56, 28, 140, 130, 224, 74);
RT_INTERFACE!{interface IDeploymentResult2(IDeploymentResult2Vtbl, IDeploymentResult2_Abi): IInspectable(IInspectableVtbl) [IID_IDeploymentResult2] {
    fn get_IsRegistered(&self, out: *mut bool) -> HRESULT
}}
impl IDeploymentResult2 {
    #[inline] pub fn get_is_registered(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsRegistered)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum PackageInstallState: i32 {
    NotInstalled = 0, Staged = 1, Installed = 2, Paused = 6,
}}
DEFINE_IID!(IID_IPackageManager, 2591902565, 24207, 20423, 162, 229, 127, 105, 37, 203, 139, 83);
RT_INTERFACE!{interface IPackageManager(IPackageManagerVtbl, IPackageManager_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager] {
    fn AddPackageAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn UpdatePackageAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RemovePackageAsync(&self, packageFullName: HSTRING, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StagePackageAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RegisterPackageAsync(&self, manifestUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackages(&self, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityId(&self, userSecurityId: HSTRING, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByNamePublisher(&self, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdNamePublisher(&self, userSecurityId: HSTRING, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    fn FindUsers(&self, packageFullName: HSTRING, out: *mut <foundation::collections::IIterable<PackageUserInformation> as RtType>::Abi) -> HRESULT,
    fn SetPackageState(&self, packageFullName: HSTRING, packageState: PackageState) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackageByPackageFullName(&self, packageFullName: HSTRING, out: *mut <super::super::applicationmodel::Package as RtType>::Abi) -> HRESULT,
    fn CleanupPackageForUserAsync(&self, packageName: HSTRING, userSecurityId: HSTRING, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByPackageFamilyName(&self, packageFamilyName: HSTRING, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdPackageFamilyName(&self, userSecurityId: HSTRING, packageFamilyName: HSTRING, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackageByUserSecurityIdPackageFullName(&self, userSecurityId: HSTRING, packageFullName: HSTRING, out: *mut <super::super::applicationmodel::Package as RtType>::Abi) -> HRESULT
}}
impl IPackageManager {
    #[inline] pub fn add_package_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_package_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).UpdatePackageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_package_async(&self, packageFullName: &HStringArg) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RemovePackageAsync)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_package_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StagePackageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_package_async(&self, manifestUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RegisterPackageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(manifestUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages(&self) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackages)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id(&self, userSecurityId: &HStringArg) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityId)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_name_publisher(&self, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByNamePublisher)(self.0.as_abi() as *const _ as *mut _, packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_name_publisher(&self, userSecurityId: &HStringArg, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdNamePublisher)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_users(&self, packageFullName: &HStringArg) -> Result<Option<foundation::collections::IIterable<PackageUserInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindUsers)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_package_state(&self, packageFullName: &HStringArg, packageState: PackageState) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).SetPackageState)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), packageState);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_package_by_package_full_name(&self, packageFullName: &HStringArg) -> Result<Option<super::super::applicationmodel::Package>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageByPackageFullName)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), &mut out);
        if hr == S_OK { Ok(super::super::applicationmodel::Package::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn cleanup_package_for_user_async(&self, packageName: &HStringArg, userSecurityId: &HStringArg) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).CleanupPackageForUserAsync)(self.0.as_abi() as *const _ as *mut _, packageName.get(), userSecurityId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_package_family_name(&self, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByPackageFamilyName)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_package_family_name(&self, userSecurityId: &HStringArg, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdPackageFamilyName)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_package_by_user_security_id_package_full_name(&self, userSecurityId: &HStringArg, packageFullName: &HStringArg) -> Result<Option<super::super::applicationmodel::Package>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageByUserSecurityIdPackageFullName)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageFullName.get(), &mut out);
        if hr == S_OK { Ok(super::super::applicationmodel::Package::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PackageManager: IPackageManager}
impl RtActivatable<IActivationFactory> for PackageManager {}
DEFINE_CLSID!(PackageManager(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,68,101,112,108,111,121,109,101,110,116,46,80,97,99,107,97,103,101,77,97,110,97,103,101,114,0]) [CLSID_PackageManager]);
DEFINE_IID!(IID_IPackageManager2, 4155166861, 2112, 18162, 181, 216, 202, 212, 118, 147, 160, 149);
RT_INTERFACE!{interface IPackageManager2(IPackageManager2Vtbl, IPackageManager2_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager2] {
    fn RemovePackageWithOptionsAsync(&self, packageFullName: HSTRING, removalOptions: RemovalOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StagePackageWithOptionsAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RegisterPackageByFullNameAsync(&self, mainPackageFullName: HSTRING, dependencyPackageFullNames: <foundation::collections::IIterable<HString> as RtType>::Abi, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesWithPackageTypes(&self, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdWithPackageTypes(&self, userSecurityId: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByNamePublisherWithPackageTypes(&self, packageName: HSTRING, packagePublisher: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, userSecurityId: HSTRING, packageName: HSTRING, packagePublisher: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packageFamilyName: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes(&self, userSecurityId: HSTRING, packageFamilyName: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IIterable<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    fn StageUserDataAsync(&self, packageFullName: HSTRING, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager2 {
    #[inline] pub fn remove_package_with_options_async(&self, packageFullName: &HStringArg, removalOptions: RemovalOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RemovePackageWithOptionsAsync)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), removalOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_package_with_options_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StagePackageWithOptionsAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_package_by_full_name_async(&self, mainPackageFullName: &HStringArg, dependencyPackageFullNames: &foundation::collections::IIterable<HString>, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RegisterPackageByFullNameAsync)(self.0.as_abi() as *const _ as *mut _, mainPackageFullName.get(), get_abi(dependencyPackageFullNames) as *const _ as *mut _, deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_with_package_types(&self, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_with_package_types(&self, userSecurityId: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_name_publisher_with_package_types(&self, packageName: &HStringArg, packagePublisher: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByNamePublisherWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, packageName.get(), packagePublisher.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_name_publisher_with_package_types(&self, userSecurityId: &HStringArg, packageName: &HStringArg, packagePublisher: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageName.get(), packagePublisher.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_package_family_name_with_package_types(&self, packageFamilyName: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByPackageFamilyNameWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_package_family_name_with_package_types(&self, userSecurityId: &HStringArg, packageFamilyName: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IIterable<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdPackageFamilyNameWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageFamilyName.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_user_data_async(&self, packageFullName: &HStringArg) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StageUserDataAsync)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager3, 3668810056, 14065, 16807, 145, 136, 188, 38, 62, 13, 203, 114);
RT_INTERFACE!{interface IPackageManager3(IPackageManager3Vtbl, IPackageManager3_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager3] {
    fn AddPackageVolumeAsync(&self, packageStorePath: HSTRING, out: *mut <foundation::IAsyncOperation<PackageVolume> as RtType>::Abi) -> HRESULT,
    fn AddPackageToVolumeAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn ClearPackageStatus(&self, packageFullName: HSTRING, status: PackageStatus) -> HRESULT,
    fn RegisterPackageWithAppDataVolumeAsync(&self, manifestUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, appDataVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn FindPackageVolumeByName(&self, volumeName: HSTRING, out: *mut <PackageVolume as RtType>::Abi) -> HRESULT,
    fn FindPackageVolumes(&self, out: *mut <foundation::collections::IIterable<PackageVolume> as RtType>::Abi) -> HRESULT,
    fn GetDefaultPackageVolume(&self, out: *mut <PackageVolume as RtType>::Abi) -> HRESULT,
    fn MovePackageToVolumeAsync(&self, packageFullName: HSTRING, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RemovePackageVolumeAsync(&self, volume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn SetDefaultPackageVolume(&self, volume: <PackageVolume as RtType>::Abi) -> HRESULT,
    fn SetPackageStatus(&self, packageFullName: HSTRING, status: PackageStatus) -> HRESULT,
    fn SetPackageVolumeOfflineAsync(&self, packageVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn SetPackageVolumeOnlineAsync(&self, packageVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StagePackageToVolumeAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StageUserDataWithOptionsAsync(&self, packageFullName: HSTRING, deploymentOptions: DeploymentOptions, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager3 {
    #[inline] pub fn add_package_volume_async(&self, packageStorePath: &HStringArg) -> Result<foundation::IAsyncOperation<PackageVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageVolumeAsync)(self.0.as_abi() as *const _ as *mut _, packageStorePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_package_to_volume_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageToVolumeAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_package_status(&self, packageFullName: &HStringArg, status: PackageStatus) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).ClearPackageStatus)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), status);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn register_package_with_app_data_volume_async(&self, manifestUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, appDataVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RegisterPackageWithAppDataVolumeAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(manifestUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(appDataVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_package_volume_by_name(&self, volumeName: &HStringArg) -> Result<Option<PackageVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageVolumeByName)(self.0.as_abi() as *const _ as *mut _, volumeName.get(), &mut out);
        if hr == S_OK { Ok(PackageVolume::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_package_volumes(&self) -> Result<Option<foundation::collections::IIterable<PackageVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageVolumes)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default_package_volume(&self) -> Result<Option<PackageVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetDefaultPackageVolume)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PackageVolume::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn move_package_to_volume_async(&self, packageFullName: &HStringArg, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).MovePackageToVolumeAsync)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_package_volume_async(&self, volume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RemovePackageVolumeAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(volume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_default_package_volume(&self, volume: &PackageVolume) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).SetDefaultPackageVolume)(self.0.as_abi() as *const _ as *mut _, get_abi(volume) as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_package_status(&self, packageFullName: &HStringArg, status: PackageStatus) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).SetPackageStatus)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), status);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_package_volume_offline_async(&self, packageVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).SetPackageVolumeOfflineAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_package_volume_online_async(&self, packageVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).SetPackageVolumeOnlineAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_package_to_volume_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StagePackageToVolumeAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_user_data_with_options_async(&self, packageFullName: &HStringArg, deploymentOptions: DeploymentOptions) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StageUserDataWithOptionsAsync)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), deploymentOptions, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager4, 1014077795, 47798, 18111, 143, 247, 218, 71, 25, 35, 10, 230);
RT_INTERFACE!{interface IPackageManager4(IPackageManager4Vtbl, IPackageManager4_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager4] {
    fn GetPackageVolumesAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<PackageVolume>> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager4 {
    #[inline] pub fn get_package_volumes_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<PackageVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetPackageVolumesAsync)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager5, 1897869591, 6909, 17171, 151, 140, 155, 182, 225, 184, 100, 167);
RT_INTERFACE!{interface IPackageManager5(IPackageManager5Vtbl, IPackageManager5_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager5] {
    fn AddPackageToVolumeAndOptionalPackagesAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, externalPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StagePackageToVolumeAndOptionalPackagesAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, externalPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RegisterPackageByFamilyNameAndOptionalPackagesAsync(&self, mainPackageFamilyName: HSTRING, dependencyPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, deploymentOptions: DeploymentOptions, appDataVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn get_DebugSettings(&self, out: *mut <PackageManagerDebugSettings as RtType>::Abi) -> HRESULT
}}
impl IPackageManager5 {
    #[inline] pub fn add_package_to_volume_and_optional_packages_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, externalPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageToVolumeAndOptionalPackagesAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(externalPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_package_to_volume_and_optional_packages_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, externalPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StagePackageToVolumeAndOptionalPackagesAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(externalPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn register_package_by_family_name_and_optional_packages_async(&self, mainPackageFamilyName: &HStringArg, dependencyPackageFamilyNames: &foundation::collections::IIterable<HString>, deploymentOptions: DeploymentOptions, appDataVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RegisterPackageByFamilyNameAndOptionalPackagesAsync)(self.0.as_abi() as *const _ as *mut _, mainPackageFamilyName.get(), get_abi(dependencyPackageFamilyNames) as *const _ as *mut _, deploymentOptions, get_abi(appDataVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_debug_settings(&self) -> Result<Option<PackageManagerDebugSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_DebugSettings)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PackageManagerDebugSettings::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager6, 138930441, 21453, 20047, 131, 46, 87, 209, 128, 246, 228, 71);
RT_INTERFACE!{interface IPackageManager6(IPackageManager6Vtbl, IPackageManager6_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager6] {
    fn ProvisionPackageForAllUsersAsync(&self, packageFamilyName: HSTRING, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn AddPackageByAppInstallerFileAsync(&self, appInstallerFileUri: <foundation::Uri as RtType>::Abi, options: AddPackageByAppInstallerOptions, targetVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RequestAddPackageByAppInstallerFileAsync(&self, appInstallerFileUri: <foundation::Uri as RtType>::Abi, options: AddPackageByAppInstallerOptions, targetVolume: <PackageVolume as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn AddPackageToVolumeAndRelatedSetAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, options: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, packageUrisToInstall: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, relatedPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn StagePackageToVolumeAndRelatedSetAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, options: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, packageUrisToInstall: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, relatedPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT,
    fn RequestAddPackageAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, relatedPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager6 {
    #[inline] pub fn provision_package_for_all_users_async(&self, packageFamilyName: &HStringArg) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).ProvisionPackageForAllUsersAsync)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_package_by_app_installer_file_async(&self, appInstallerFileUri: &foundation::Uri, options: AddPackageByAppInstallerOptions, targetVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageByAppInstallerFileAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(appInstallerFileUri) as *const _ as *mut _, options, get_abi(targetVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_add_package_by_app_installer_file_async(&self, appInstallerFileUri: &foundation::Uri, options: AddPackageByAppInstallerOptions, targetVolume: &PackageVolume) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RequestAddPackageByAppInstallerFileAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(appInstallerFileUri) as *const _ as *mut _, options, get_abi(targetVolume) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_package_to_volume_and_related_set_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, options: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, packageUrisToInstall: &foundation::collections::IIterable<foundation::Uri>, relatedPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).AddPackageToVolumeAndRelatedSetAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, options, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(packageUrisToInstall) as *const _ as *mut _, get_abi(relatedPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn stage_package_to_volume_and_related_set_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, options: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, packageUrisToInstall: &foundation::collections::IIterable<foundation::Uri>, relatedPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).StagePackageToVolumeAndRelatedSetAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, options, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(packageUrisToInstall) as *const _ as *mut _, get_abi(relatedPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_add_package_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, relatedPackageUris: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RequestAddPackageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(relatedPackageUris) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager7, 4068889844, 11175, 19328, 136, 214, 190, 21, 249, 162, 63, 186);
RT_INTERFACE!{interface IPackageManager7(IPackageManager7Vtbl, IPackageManager7_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager7] {
    fn RequestAddPackageAndRelatedSetAsync(&self, packageUri: <foundation::Uri as RtType>::Abi, dependencyPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, deploymentOptions: DeploymentOptions, targetVolume: <PackageVolume as RtType>::Abi, optionalPackageFamilyNames: <foundation::collections::IIterable<HString> as RtType>::Abi, relatedPackageUris: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, packageUrisToInstall: <foundation::collections::IIterable<foundation::Uri> as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager7 {
    #[inline] pub fn request_add_package_and_related_set_async(&self, packageUri: &foundation::Uri, dependencyPackageUris: &foundation::collections::IIterable<foundation::Uri>, deploymentOptions: DeploymentOptions, targetVolume: &PackageVolume, optionalPackageFamilyNames: &foundation::collections::IIterable<HString>, relatedPackageUris: &foundation::collections::IIterable<foundation::Uri>, packageUrisToInstall: &foundation::collections::IIterable<foundation::Uri>) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).RequestAddPackageAndRelatedSetAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(packageUri) as *const _ as *mut _, get_abi(dependencyPackageUris) as *const _ as *mut _, deploymentOptions, get_abi(targetVolume) as *const _ as *mut _, get_abi(optionalPackageFamilyNames) as *const _ as *mut _, get_abi(relatedPackageUris) as *const _ as *mut _, get_abi(packageUrisToInstall) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManager8, 3092730672, 4760, 20194, 128, 238, 127, 101, 156, 93, 39, 130);
RT_INTERFACE!{interface IPackageManager8(IPackageManager8Vtbl, IPackageManager8_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManager8] {
    fn DeprovisionPackageForAllUsersAsync(&self, packageFamilyName: HSTRING, out: *mut <foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress> as RtType>::Abi) -> HRESULT
}}
impl IPackageManager8 {
    #[inline] pub fn deprovision_package_for_all_users_async(&self, packageFamilyName: &HStringArg) -> Result<foundation::IAsyncOperationWithProgress<DeploymentResult, DeploymentProgress>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).DeprovisionPackageForAllUsersAsync)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPackageManagerDebugSettings, 442570371, 43400, 20431, 143, 15, 206, 23, 88, 152, 232, 235);
RT_INTERFACE!{interface IPackageManagerDebugSettings(IPackageManagerDebugSettingsVtbl, IPackageManagerDebugSettings_Abi): IInspectable(IInspectableVtbl) [IID_IPackageManagerDebugSettings] {
    #[cfg(feature="windows-applicationmodel")] fn SetContentGroupStateAsync(&self, package: <super::super::applicationmodel::Package as RtType>::Abi, contentGroupName: HSTRING, state: super::super::applicationmodel::PackageContentGroupState, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn SetContentGroupStateWithPercentageAsync(&self, package: <super::super::applicationmodel::Package as RtType>::Abi, contentGroupName: HSTRING, state: super::super::applicationmodel::PackageContentGroupState, completionPercentage: f64, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IPackageManagerDebugSettings {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn set_content_group_state_async(&self, package: &super::super::applicationmodel::Package, contentGroupName: &HStringArg, state: super::super::applicationmodel::PackageContentGroupState) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).SetContentGroupStateAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(package) as *const _ as *mut _, contentGroupName.get(), state, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn set_content_group_state_with_percentage_async(&self, package: &super::super::applicationmodel::Package, contentGroupName: &HStringArg, state: super::super::applicationmodel::PackageContentGroupState, completionPercentage: f64) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).SetContentGroupStateWithPercentageAsync)(self.0.as_abi() as *const _ as *mut _, get_abi(package) as *const _ as *mut _, contentGroupName.get(), state, completionPercentage, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PackageManagerDebugSettings: IPackageManagerDebugSettings}
RT_ENUM! { enum PackageState: i32 {
    Normal = 0, LicenseInvalid = 1, Modified = 2, Tampered = 3,
}}
RT_ENUM! { enum PackageStatus: u32 {
    OK = 0, LicenseIssue = 1, Modified = 2, Tampered = 4, Disabled = 8,
}}
RT_ENUM! { enum PackageTypes: u32 {
    None = 0, Main = 1, Framework = 2, Resource = 4, Bundle = 8, Xap = 16, Optional = 32,
}}
DEFINE_IID!(IID_IPackageUserInformation, 4130878499, 64009, 19644, 144, 85, 21, 202, 39, 94, 46, 126);
RT_INTERFACE!{interface IPackageUserInformation(IPackageUserInformationVtbl, IPackageUserInformation_Abi): IInspectable(IInspectableVtbl) [IID_IPackageUserInformation] {
    fn get_UserSecurityId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InstallState(&self, out: *mut PackageInstallState) -> HRESULT
}}
impl IPackageUserInformation {
    #[inline] pub fn get_user_security_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_UserSecurityId)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_install_state(&self) -> Result<PackageInstallState> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_InstallState)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PackageUserInformation: IPackageUserInformation}
DEFINE_IID!(IID_IPackageVolume, 3475403459, 6720, 17488, 151, 57, 42, 206, 46, 137, 136, 83);
RT_INTERFACE!{interface IPackageVolume(IPackageVolumeVtbl, IPackageVolume_Abi): IInspectable(IInspectableVtbl) [IID_IPackageVolume] {
    fn get_IsOffline(&self, out: *mut bool) -> HRESULT,
    fn get_IsSystemVolume(&self, out: *mut bool) -> HRESULT,
    fn get_MountPoint(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PackageStorePath(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SupportsHardLinks(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackages(&self, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByNamePublisher(&self, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByPackageFamilyName(&self, packageFamilyName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesWithPackageTypes(&self, packageTypes: PackageTypes, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByNamePublisherWithPackagesTypes(&self, packageTypes: PackageTypes, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByPackageFamilyNameWithPackageTypes(&self, packageTypes: PackageTypes, packageFamilyName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackageByPackageFullName(&self, packageFullName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityId(&self, userSecurityId: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdNamePublisher(&self, userSecurityId: HSTRING, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdPackageFamilyName(&self, userSecurityId: HSTRING, packageFamilyName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdWithPackageTypes(&self, userSecurityId: HSTRING, packageTypes: PackageTypes, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdNamePublisherWithPackageTypes(&self, userSecurityId: HSTRING, packageTypes: PackageTypes, packageName: HSTRING, packagePublisher: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes(&self, userSecurityId: HSTRING, packageTypes: PackageTypes, packageFamilyName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-applicationmodel")] fn FindPackageByUserSecurityIdPackageFullName(&self, userSecurityId: HSTRING, packageFullName: HSTRING, out: *mut <foundation::collections::IVector<super::super::applicationmodel::Package> as RtType>::Abi) -> HRESULT
}}
impl IPackageVolume {
    #[inline] pub fn get_is_offline(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsOffline)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_system_volume(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsSystemVolume)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_mount_point(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_MountPoint)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Name)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_package_store_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_PackageStorePath)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_supports_hard_links(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_SupportsHardLinks)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages(&self) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackages)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_name_publisher(&self, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByNamePublisher)(self.0.as_abi() as *const _ as *mut _, packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_package_family_name(&self, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByPackageFamilyName)(self.0.as_abi() as *const _ as *mut _, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_with_package_types(&self, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_name_publisher_with_packages_types(&self, packageTypes: PackageTypes, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByNamePublisherWithPackagesTypes)(self.0.as_abi() as *const _ as *mut _, packageTypes, packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_package_family_name_with_package_types(&self, packageTypes: PackageTypes, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByPackageFamilyNameWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, packageTypes, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_package_by_package_full_name(&self, packageFullName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageByPackageFullName)(self.0.as_abi() as *const _ as *mut _, packageFullName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id(&self, userSecurityId: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityId)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_name_publisher(&self, userSecurityId: &HStringArg, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdNamePublisher)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_package_family_name(&self, userSecurityId: &HStringArg, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdPackageFamilyName)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_with_package_types(&self, userSecurityId: &HStringArg, packageTypes: PackageTypes) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageTypes, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_name_publisher_with_package_types(&self, userSecurityId: &HStringArg, packageTypes: PackageTypes, packageName: &HStringArg, packagePublisher: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdNamePublisherWithPackageTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageTypes, packageName.get(), packagePublisher.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_packages_by_user_security_id_package_family_name_with_packages_types(&self, userSecurityId: &HStringArg, packageTypes: PackageTypes, packageFamilyName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackagesByUserSecurityIdPackageFamilyNameWithPackagesTypes)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageTypes, packageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn find_package_by_user_security_id_package_full_name(&self, userSecurityId: &HStringArg, packageFullName: &HStringArg) -> Result<Option<foundation::collections::IVector<super::super::applicationmodel::Package>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindPackageByUserSecurityIdPackageFullName)(self.0.as_abi() as *const _ as *mut _, userSecurityId.get(), packageFullName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PackageVolume: IPackageVolume}
DEFINE_IID!(IID_IPackageVolume2, 1185664814, 40404, 18338, 171, 140, 198, 64, 131, 73, 188, 216);
RT_INTERFACE!{interface IPackageVolume2(IPackageVolume2Vtbl, IPackageVolume2_Abi): IInspectable(IInspectableVtbl) [IID_IPackageVolume2] {
    fn get_IsFullTrustPackageSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsAppxInstallSupported(&self, out: *mut bool) -> HRESULT,
    fn GetAvailableSpaceAsync(&self, out: *mut <foundation::IAsyncOperation<u64> as RtType>::Abi) -> HRESULT
}}
impl IPackageVolume2 {
    #[inline] pub fn get_is_full_trust_package_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsFullTrustPackageSupported)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_appx_install_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsAppxInstallSupported)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_available_space_async(&self) -> Result<foundation::IAsyncOperation<u64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetAvailableSpaceAsync)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum RemovalOptions: u32 {
    None = 0, PreserveApplicationData = 4096, RemoveForAllUsers = 524288,
}}
pub mod preview { // Windows.Management.Deployment.Preview
use crate::prelude::*;
RT_CLASS!{static class ClassicAppManager}
impl RtActivatable<IClassicAppManagerStatics> for ClassicAppManager {}
impl ClassicAppManager {
    #[inline] pub fn find_installed_app(appUninstallKey: &HStringArg) -> Result<Option<InstalledClassicAppInfo>> {
        <Self as RtActivatable<IClassicAppManagerStatics>>::get_activation_factory().find_installed_app(appUninstallKey)
    }
}
DEFINE_CLSID!(ClassicAppManager(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,68,101,112,108,111,121,109,101,110,116,46,80,114,101,118,105,101,119,46,67,108,97,115,115,105,99,65,112,112,77,97,110,97,103,101,114,0]) [CLSID_ClassicAppManager]);
DEFINE_IID!(IID_IClassicAppManagerStatics, 3808089704, 34860, 20275, 176, 53, 13, 247, 185, 13, 103, 230);
RT_INTERFACE!{static interface IClassicAppManagerStatics(IClassicAppManagerStaticsVtbl, IClassicAppManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IClassicAppManagerStatics] {
    fn FindInstalledApp(&self, appUninstallKey: HSTRING, out: *mut <InstalledClassicAppInfo as RtType>::Abi) -> HRESULT
}}
impl IClassicAppManagerStatics {
    #[inline] pub fn find_installed_app(&self, appUninstallKey: &HStringArg) -> Result<Option<InstalledClassicAppInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).FindInstalledApp)(self.0.as_abi() as *const _ as *mut _, appUninstallKey.get(), &mut out);
        if hr == S_OK { Ok(InstalledClassicAppInfo::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IInstalledClassicAppInfo, 175979939, 26064, 16518, 128, 214, 6, 16, 215, 96, 32, 125);
RT_INTERFACE!{interface IInstalledClassicAppInfo(IInstalledClassicAppInfoVtbl, IInstalledClassicAppInfo_Abi): IInspectable(IInspectableVtbl) [IID_IInstalledClassicAppInfo] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IInstalledClassicAppInfo {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_DisplayName)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_DisplayVersion)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class InstalledClassicAppInfo: IInstalledClassicAppInfo}
} // Windows.Management.Deployment.Preview
} // Windows.Management.Deployment
pub mod policies { // Windows.Management.Policies
use crate::prelude::*;
RT_CLASS!{static class NamedPolicy}
impl RtActivatable<INamedPolicyStatics> for NamedPolicy {}
impl NamedPolicy {
    #[inline] pub fn get_policy_from_path(area: &HStringArg, name: &HStringArg) -> Result<Option<NamedPolicyData>> {
        <Self as RtActivatable<INamedPolicyStatics>>::get_activation_factory().get_policy_from_path(area, name)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_policy_from_path_for_user(user: &super::super::system::User, area: &HStringArg, name: &HStringArg) -> Result<Option<NamedPolicyData>> {
        <Self as RtActivatable<INamedPolicyStatics>>::get_activation_factory().get_policy_from_path_for_user(user, area, name)
    }
}
DEFINE_CLSID!(NamedPolicy(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,80,111,108,105,99,105,101,115,46,78,97,109,101,100,80,111,108,105,99,121,0]) [CLSID_NamedPolicy]);
DEFINE_IID!(IID_INamedPolicyData, 953987480, 38316, 16503, 166, 67, 128, 120, 202, 226, 100, 0);
RT_INTERFACE!{interface INamedPolicyData(INamedPolicyDataVtbl, INamedPolicyData_Abi): IInspectable(IInspectableVtbl) [IID_INamedPolicyData] {
    fn get_Area(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut NamedPolicyKind) -> HRESULT,
    fn get_IsManaged(&self, out: *mut bool) -> HRESULT,
    fn get_IsUserPolicy(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-system"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut <super::super::system::User as RtType>::Abi) -> HRESULT,
    fn GetBoolean(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetBinary(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn GetInt32(&self, out: *mut i32) -> HRESULT,
    fn GetInt64(&self, out: *mut i64) -> HRESULT,
    fn GetString(&self, out: *mut HSTRING) -> HRESULT,
    fn add_Changed(&self, changedHandler: <foundation::TypedEventHandler<NamedPolicyData, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Changed(&self, cookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl INamedPolicyData {
    #[inline] pub fn get_area(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Area)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Name)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<NamedPolicyKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_Kind)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_managed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsManaged)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_user_policy(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsUserPolicy)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<super::super::system::User>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_User)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::system::User::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).GetBoolean)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_binary(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetBinary)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_int32(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).GetInt32)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_int64(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).GetInt64)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetString)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_changed(&self, changedHandler: &foundation::TypedEventHandler<NamedPolicyData, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).add_Changed)(self.0.as_abi() as *const _ as *mut _, get_abi(changedHandler) as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).remove_Changed)(self.0.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class NamedPolicyData: INamedPolicyData}
RT_ENUM! { enum NamedPolicyKind: i32 {
    Invalid = 0, Binary = 1, Boolean = 2, Int32 = 3, Int64 = 4, String = 5,
}}
DEFINE_IID!(IID_INamedPolicyStatics, 2138651623, 30404, 16472, 140, 173, 103, 102, 44, 208, 95, 13);
RT_INTERFACE!{static interface INamedPolicyStatics(INamedPolicyStaticsVtbl, INamedPolicyStatics_Abi): IInspectable(IInspectableVtbl) [IID_INamedPolicyStatics] {
    fn GetPolicyFromPath(&self, area: HSTRING, name: HSTRING, out: *mut <NamedPolicyData as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetPolicyFromPathForUser(&self, user: <super::super::system::User as RtType>::Abi, area: HSTRING, name: HSTRING, out: *mut <NamedPolicyData as RtType>::Abi) -> HRESULT
}}
impl INamedPolicyStatics {
    #[inline] pub fn get_policy_from_path(&self, area: &HStringArg, name: &HStringArg) -> Result<Option<NamedPolicyData>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetPolicyFromPath)(self.0.as_abi() as *const _ as *mut _, area.get(), name.get(), &mut out);
        if hr == S_OK { Ok(NamedPolicyData::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_policy_from_path_for_user(&self, user: &super::super::system::User, area: &HStringArg, name: &HStringArg) -> Result<Option<NamedPolicyData>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetPolicyFromPathForUser)(self.0.as_abi() as *const _ as *mut _, get_abi(user) as *const _ as *mut _, area.get(), name.get(), &mut out);
        if hr == S_OK { Ok(NamedPolicyData::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Management.Policies
pub mod update { // Windows.Management.Update
use crate::prelude::*;
DEFINE_IID!(IID_IPreviewBuildsManager, 4194819425, 32335, 23031, 124, 159, 222, 249, 5, 28, 95, 98);
RT_INTERFACE!{interface IPreviewBuildsManager(IPreviewBuildsManagerVtbl, IPreviewBuildsManager_Abi): IInspectable(IInspectableVtbl) [IID_IPreviewBuildsManager] {
    fn get_ArePreviewBuildsAllowed(&self, out: *mut bool) -> HRESULT,
    fn put_ArePreviewBuildsAllowed(&self, value: bool) -> HRESULT,
    fn GetCurrentState(&self, out: *mut <PreviewBuildsState as RtType>::Abi) -> HRESULT,
    fn SyncAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IPreviewBuildsManager {
    #[inline] pub fn get_are_preview_builds_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_ArePreviewBuildsAllowed)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_are_preview_builds_allowed(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.0.as_abi().lpVtbl).put_ArePreviewBuildsAllowed)(self.0.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_current_state(&self) -> Result<Option<PreviewBuildsState>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetCurrentState)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PreviewBuildsState::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn sync_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).SyncAsync)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PreviewBuildsManager: IPreviewBuildsManager}
impl RtActivatable<IPreviewBuildsManagerStatics> for PreviewBuildsManager {}
impl PreviewBuildsManager {
    #[inline] pub fn get_default() -> Result<Option<PreviewBuildsManager>> {
        <Self as RtActivatable<IPreviewBuildsManagerStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IPreviewBuildsManagerStatics>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(PreviewBuildsManager(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,85,112,100,97,116,101,46,80,114,101,118,105,101,119,66,117,105,108,100,115,77,97,110,97,103,101,114,0]) [CLSID_PreviewBuildsManager]);
DEFINE_IID!(IID_IPreviewBuildsManagerStatics, 1044523143, 45330, 23152, 125, 161, 151, 215, 141, 50, 170, 41);
RT_INTERFACE!{static interface IPreviewBuildsManagerStatics(IPreviewBuildsManagerStaticsVtbl, IPreviewBuildsManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPreviewBuildsManagerStatics] {
    fn GetDefault(&self, out: *mut <PreviewBuildsManager as RtType>::Abi) -> HRESULT,
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IPreviewBuildsManagerStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<PreviewBuildsManager>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).GetDefault)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PreviewBuildsManager::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).IsSupported)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPreviewBuildsState, 2733805630, 45603, 24419, 117, 70, 62, 142, 172, 7, 10, 46);
RT_INTERFACE!{interface IPreviewBuildsState(IPreviewBuildsStateVtbl, IPreviewBuildsState_Abi): IInspectable(IInspectableVtbl) [IID_IPreviewBuildsState] {
    fn get_Properties(&self, out: *mut <foundation::collections::ValueSet as RtType>::Abi) -> HRESULT
}}
impl IPreviewBuildsState {
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::ValueSet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.0.as_abi().lpVtbl).get_Properties)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::ValueSet::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PreviewBuildsState: IPreviewBuildsState}
} // Windows.Management.Update
pub mod workplace { // Windows.Management.Workplace
use crate::prelude::*;
DEFINE_IID!(IID_IMdmAllowPolicyStatics, 3281455591, 29724, 16882, 164, 182, 49, 76, 49, 80, 37, 134);
RT_INTERFACE!{static interface IMdmAllowPolicyStatics(IMdmAllowPolicyStaticsVtbl, IMdmAllowPolicyStatics_Abi): IInspectable(IInspectableVtbl) [IID_IMdmAllowPolicyStatics] {
    fn IsBrowserAllowed(&self, out: *mut bool) -> HRESULT,
    fn IsCameraAllowed(&self, out: *mut bool) -> HRESULT,
    fn IsMicrosoftAccountAllowed(&self, out: *mut bool) -> HRESULT,
    fn IsStoreAllowed(&self, out: *mut bool) -> HRESULT
}}
impl IMdmAllowPolicyStatics {
    #[inline] pub fn is_browser_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).IsBrowserAllowed)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_camera_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).IsCameraAllowed)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_microsoft_account_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).IsMicrosoftAccountAllowed)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_store_allowed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).IsStoreAllowed)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class MdmPolicy}
impl RtActivatable<IMdmAllowPolicyStatics> for MdmPolicy {}
impl RtActivatable<IMdmPolicyStatics2> for MdmPolicy {}
impl MdmPolicy {
    #[inline] pub fn is_browser_allowed() -> Result<bool> {
        <Self as RtActivatable<IMdmAllowPolicyStatics>>::get_activation_factory().is_browser_allowed()
    }
    #[inline] pub fn is_camera_allowed() -> Result<bool> {
        <Self as RtActivatable<IMdmAllowPolicyStatics>>::get_activation_factory().is_camera_allowed()
    }
    #[inline] pub fn is_microsoft_account_allowed() -> Result<bool> {
        <Self as RtActivatable<IMdmAllowPolicyStatics>>::get_activation_factory().is_microsoft_account_allowed()
    }
    #[inline] pub fn is_store_allowed() -> Result<bool> {
        <Self as RtActivatable<IMdmAllowPolicyStatics>>::get_activation_factory().is_store_allowed()
    }
    #[inline] pub fn get_messaging_sync_policy() -> Result<MessagingSyncPolicy> {
        <Self as RtActivatable<IMdmPolicyStatics2>>::get_activation_factory().get_messaging_sync_policy()
    }
}
DEFINE_CLSID!(MdmPolicy(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,87,111,114,107,112,108,97,99,101,46,77,100,109,80,111,108,105,99,121,0]) [CLSID_MdmPolicy]);
DEFINE_IID!(IID_IMdmPolicyStatics2, 3382474022, 980, 18937, 169, 147, 67, 239, 204, 210, 101, 196);
RT_INTERFACE!{static interface IMdmPolicyStatics2(IMdmPolicyStatics2Vtbl, IMdmPolicyStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IMdmPolicyStatics2] {
    fn GetMessagingSyncPolicy(&self, out: *mut MessagingSyncPolicy) -> HRESULT
}}
impl IMdmPolicyStatics2 {
    #[inline] pub fn get_messaging_sync_policy(&self) -> Result<MessagingSyncPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).GetMessagingSyncPolicy)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum MessagingSyncPolicy: i32 {
    Disallowed = 0, Allowed = 1, Required = 2,
}}
RT_CLASS!{static class WorkplaceSettings}
impl RtActivatable<IWorkplaceSettingsStatics> for WorkplaceSettings {}
impl WorkplaceSettings {
    #[inline] pub fn get_is_microsoft_account_optional() -> Result<bool> {
        <Self as RtActivatable<IWorkplaceSettingsStatics>>::get_activation_factory().get_is_microsoft_account_optional()
    }
}
DEFINE_CLSID!(WorkplaceSettings(&[87,105,110,100,111,119,115,46,77,97,110,97,103,101,109,101,110,116,46,87,111,114,107,112,108,97,99,101,46,87,111,114,107,112,108,97,99,101,83,101,116,116,105,110,103,115,0]) [CLSID_WorkplaceSettings]);
DEFINE_IID!(IID_IWorkplaceSettingsStatics, 3831984125, 11666, 19464, 186, 212, 246, 89, 11, 84, 166, 211);
RT_INTERFACE!{static interface IWorkplaceSettingsStatics(IWorkplaceSettingsStaticsVtbl, IWorkplaceSettingsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IWorkplaceSettingsStatics] {
    fn get_IsMicrosoftAccountOptional(&self, out: *mut bool) -> HRESULT
}}
impl IWorkplaceSettingsStatics {
    #[inline] pub fn get_is_microsoft_account_optional(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.0.as_abi().lpVtbl).get_IsMicrosoftAccountOptional)(self.0.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Management.Workplace
