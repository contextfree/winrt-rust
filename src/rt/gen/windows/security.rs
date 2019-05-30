pub mod authentication { // Windows.Security.Authentication
pub mod identity { // Windows.Security.Authentication.Identity
use crate::prelude::*;
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationInfo, 942807756, 26411, 18467, 182, 3, 107, 60, 117, 61, 175, 151);
RT_INTERFACE!{interface IEnterpriseKeyCredentialRegistrationInfo(IEnterpriseKeyCredentialRegistrationInfoVtbl): IInspectable [IID_IEnterpriseKeyCredentialRegistrationInfo] {
    fn get_TenantId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TenantName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationInfo {
    #[inline] pub fn get_tenant_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TenantId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tenant_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TenantName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subject(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Subject)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_key_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_key_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class EnterpriseKeyCredentialRegistrationInfo: IEnterpriseKeyCredentialRegistrationInfo}
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationManager, 2213789247, 41567, 19642, 187, 142, 189, 195, 45, 3, 194, 151);
RT_INTERFACE!{interface IEnterpriseKeyCredentialRegistrationManager(IEnterpriseKeyCredentialRegistrationManagerVtbl): IInspectable [IID_IEnterpriseKeyCredentialRegistrationManager] {
    fn GetRegistrationsAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>> as RtType>::Abi) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationManager {
    #[inline] pub fn get_registrations_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<EnterpriseKeyCredentialRegistrationInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetRegistrationsAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class EnterpriseKeyCredentialRegistrationManager: IEnterpriseKeyCredentialRegistrationManager}
impl RtActivatable<IEnterpriseKeyCredentialRegistrationManagerStatics> for EnterpriseKeyCredentialRegistrationManager {}
impl EnterpriseKeyCredentialRegistrationManager {
    #[inline] pub fn get_current() -> Result<Option<EnterpriseKeyCredentialRegistrationManager>> {
        <Self as RtActivatable<IEnterpriseKeyCredentialRegistrationManagerStatics>>::get_activation_factory().get_current()
    }
}
DEFINE_CLSID!(EnterpriseKeyCredentialRegistrationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,69,110,116,101,114,112,114,105,115,101,75,101,121,67,114,101,100,101,110,116,105,97,108,82,101,103,105,115,116,114,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_EnterpriseKeyCredentialRegistrationManager]);
DEFINE_IID!(IID_IEnterpriseKeyCredentialRegistrationManagerStatics, 2008571550, 44276, 19392, 186, 194, 64, 187, 70, 239, 187, 63);
RT_INTERFACE!{static interface IEnterpriseKeyCredentialRegistrationManagerStatics(IEnterpriseKeyCredentialRegistrationManagerStaticsVtbl): IInspectable [IID_IEnterpriseKeyCredentialRegistrationManagerStatics] {
    fn get_Current(&self, out: *mut <EnterpriseKeyCredentialRegistrationManager as RtType>::Abi) -> HRESULT
}}
impl IEnterpriseKeyCredentialRegistrationManagerStatics {
    #[inline] pub fn get_current(&self) -> Result<Option<EnterpriseKeyCredentialRegistrationManager>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Current)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(EnterpriseKeyCredentialRegistrationManager::wrap(out)) } else { err(hr) }
    }}
}
pub mod core { // Windows.Security.Authentication.Identity.Core
use crate::prelude::*;
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorAuthenticationManager, 265502885, 62836, 17184, 160, 142, 10, 25, 168, 35, 34, 170);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorAuthenticationManager(IMicrosoftAccountMultiFactorAuthenticationManagerVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorAuthenticationManager] {
    fn GetOneTimePassCodeAsync(&self, userAccountId: HSTRING, codeLength: u32, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo> as RtType>::Abi) -> HRESULT,
    fn AddDeviceAsync(&self, userAccountId: HSTRING, authenticationToken: HSTRING, wnsChannelId: HSTRING, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn RemoveDeviceAsync(&self, userAccountId: HSTRING, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn UpdateWnsChannelAsync(&self, userAccountId: HSTRING, channelUri: HSTRING, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn GetSessionsAsync(&self, userAccountIdList: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult> as RtType>::Abi) -> HRESULT,
    fn GetSessionsAndUnregisteredAccountsAsync(&self, userAccountIdList: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> as RtType>::Abi) -> HRESULT,
    fn ApproveSessionUsingAuthSessionInfoAsync(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationSessionInfo: <MicrosoftAccountMultiFactorSessionInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn ApproveSessionAsync(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, userAccountId: HSTRING, sessionId: HSTRING, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn DenySessionUsingAuthSessionInfoAsync(&self, authenticationSessionInfo: <MicrosoftAccountMultiFactorSessionInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT,
    fn DenySessionAsync(&self, userAccountId: HSTRING, sessionId: HSTRING, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType, out: *mut <foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse> as RtType>::Abi) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorAuthenticationManager {
    #[inline] pub fn get_one_time_pass_code_async(&self, userAccountId: &HStringArg, codeLength: u32) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetOneTimePassCodeAsync)(self.get_abi() as *const _ as *mut _, userAccountId.get(), codeLength, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_device_async(&self, userAccountId: &HStringArg, authenticationToken: &HStringArg, wnsChannelId: &HStringArg) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddDeviceAsync)(self.get_abi() as *const _ as *mut _, userAccountId.get(), authenticationToken.get(), wnsChannelId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_device_async(&self, userAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RemoveDeviceAsync)(self.get_abi() as *const _ as *mut _, userAccountId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_wns_channel_async(&self, userAccountId: &HStringArg, channelUri: &HStringArg) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UpdateWnsChannelAsync)(self.get_abi() as *const _ as *mut _, userAccountId.get(), channelUri.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sessions_async(&self, userAccountIdList: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetSessionsAsync)(self.get_abi() as *const _ as *mut _, userAccountIdList.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sessions_and_unregistered_accounts_async(&self, userAccountIdList: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetSessionsAndUnregisteredAccountsAsync)(self.get_abi() as *const _ as *mut _, userAccountIdList.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn approve_session_using_auth_session_info_async(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationSessionInfo: &MicrosoftAccountMultiFactorSessionInfo) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ApproveSessionUsingAuthSessionInfoAsync)(self.get_abi() as *const _ as *mut _, sessionAuthentictionStatus, authenticationSessionInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn approve_session_async(&self, sessionAuthentictionStatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, userAccountId: &HStringArg, sessionId: &HStringArg, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ApproveSessionAsync)(self.get_abi() as *const _ as *mut _, sessionAuthentictionStatus, userAccountId.get(), sessionId.get(), sessionAuthenticationType, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn deny_session_using_auth_session_info_async(&self, authenticationSessionInfo: &MicrosoftAccountMultiFactorSessionInfo) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DenySessionUsingAuthSessionInfoAsync)(self.get_abi() as *const _ as *mut _, authenticationSessionInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn deny_session_async(&self, userAccountId: &HStringArg, sessionId: &HStringArg, sessionAuthenticationType: MicrosoftAccountMultiFactorAuthenticationType) -> Result<foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DenySessionAsync)(self.get_abi() as *const _ as *mut _, userAccountId.get(), sessionId.get(), sessionAuthenticationType, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MicrosoftAccountMultiFactorAuthenticationManager: IMicrosoftAccountMultiFactorAuthenticationManager}
impl RtActivatable<IMicrosoftAccountMultiFactorAuthenticatorStatics> for MicrosoftAccountMultiFactorAuthenticationManager {}
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[inline] pub fn get_current() -> Result<Option<MicrosoftAccountMultiFactorAuthenticationManager>> {
        <Self as RtActivatable<IMicrosoftAccountMultiFactorAuthenticatorStatics>>::get_activation_factory().get_current()
    }
}
DEFINE_CLSID!(MicrosoftAccountMultiFactorAuthenticationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,67,111,114,101,46,77,105,99,114,111,115,111,102,116,65,99,99,111,117,110,116,77,117,108,116,105,70,97,99,116,111,114,65,117,116,104,101,110,116,105,99,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_MicrosoftAccountMultiFactorAuthenticationManager]);
RT_ENUM! { enum MicrosoftAccountMultiFactorAuthenticationType: i32 {
    User = 0, Device = 1,
}}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorAuthenticatorStatics, 3647259366, 62534, 19569, 139, 121, 110, 164, 2, 74, 169, 184);
RT_INTERFACE!{static interface IMicrosoftAccountMultiFactorAuthenticatorStatics(IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorAuthenticatorStatics] {
    fn get_Current(&self, out: *mut <MicrosoftAccountMultiFactorAuthenticationManager as RtType>::Abi) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorAuthenticatorStatics {
    #[inline] pub fn get_current(&self) -> Result<Option<MicrosoftAccountMultiFactorAuthenticationManager>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Current)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(MicrosoftAccountMultiFactorAuthenticationManager::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorGetSessionsResult, 1310960032, 59898, 18810, 149, 222, 109, 87, 71, 191, 151, 76);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorGetSessionsResult(IMicrosoftAccountMultiFactorGetSessionsResultVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorGetSessionsResult] {
    fn get_Sessions(&self, out: *mut <foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo> as RtType>::Abi) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorGetSessionsResult {
    #[inline] pub fn get_sessions(&self) -> Result<Option<foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sessions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ServiceResponse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MicrosoftAccountMultiFactorGetSessionsResult: IMicrosoftAccountMultiFactorGetSessionsResult}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorOneTimeCodedInfo, 2193237579, 55420, 18024, 169, 118, 64, 207, 174, 84, 125, 8);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorOneTimeCodedInfo(IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorOneTimeCodedInfo] {
    fn get_Code(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TimeInterval(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_TimeToLive(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    #[inline] pub fn get_code(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Code)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_time_interval(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TimeInterval)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_time_to_live(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TimeToLive)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ServiceResponse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MicrosoftAccountMultiFactorOneTimeCodedInfo: IMicrosoftAccountMultiFactorOneTimeCodedInfo}
RT_ENUM! { enum MicrosoftAccountMultiFactorServiceResponse: i32 {
    Success = 0, Error = 1, NoNetworkConnection = 2, ServiceUnavailable = 3, TotpSetupDenied = 4, NgcNotSetup = 5, SessionAlreadyDenied = 6, SessionAlreadyApproved = 7, SessionExpired = 8, NgcNonceExpired = 9, InvalidSessionId = 10, InvalidSessionType = 11, InvalidOperation = 12, InvalidStateTransition = 13, DeviceNotFound = 14, FlowDisabled = 15, SessionNotApproved = 16, OperationCanceledByUser = 17, NgcDisabledByServer = 18, NgcKeyNotFoundOnServer = 19, UIRequired = 20, DeviceIdChanged = 21,
}}
RT_ENUM! { enum MicrosoftAccountMultiFactorSessionApprovalStatus: i32 {
    Pending = 0, Approved = 1, Denied = 2,
}}
RT_ENUM! { enum MicrosoftAccountMultiFactorSessionAuthenticationStatus: i32 {
    Authenticated = 0, Unauthenticated = 1,
}}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorSessionInfo, 1602137012, 41592, 17973, 183, 101, 180, 148, 235, 38, 10, 244);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorSessionInfo(IMicrosoftAccountMultiFactorSessionInfoVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorSessionInfo] {
    fn get_UserAccountId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplaySessionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ApprovalStatus(&self, out: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> HRESULT,
    fn get_AuthenticationType(&self, out: *mut MicrosoftAccountMultiFactorAuthenticationType) -> HRESULT,
    fn get_RequestTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ExpirationTime(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorSessionInfo {
    #[inline] pub fn get_user_account_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UserAccountId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SessionId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_session_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplaySessionId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_approval_status(&self) -> Result<MicrosoftAccountMultiFactorSessionApprovalStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ApprovalStatus)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_type(&self) -> Result<MicrosoftAccountMultiFactorAuthenticationType> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AuthenticationType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_request_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RequestTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_expiration_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ExpirationTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MicrosoftAccountMultiFactorSessionInfo: IMicrosoftAccountMultiFactorSessionInfo}
DEFINE_IID!(IID_IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo, 2860434939, 55871, 16520, 162, 13, 86, 24, 175, 173, 178, 229);
RT_INTERFACE!{interface IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl): IInspectable [IID_IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo] {
    fn get_Sessions(&self, out: *mut <foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo> as RtType>::Abi) -> HRESULT,
    fn get_UnregisteredAccounts(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_ServiceResponse(&self, out: *mut MicrosoftAccountMultiFactorServiceResponse) -> HRESULT
}}
impl IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[inline] pub fn get_sessions(&self) -> Result<Option<foundation::collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sessions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_unregistered_accounts(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UnregisteredAccounts)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_service_response(&self) -> Result<MicrosoftAccountMultiFactorServiceResponse> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ServiceResponse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo: IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo}
} // Windows.Security.Authentication.Identity.Core
pub mod provider { // Windows.Security.Authentication.Identity.Provider
use crate::prelude::*;
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthentication, 34215653, 27173, 16547, 140, 0, 80, 160, 35, 246, 25, 209);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthentication(ISecondaryAuthenticationFactorAuthenticationVtbl): IInspectable [IID_ISecondaryAuthenticationFactorAuthentication] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ServiceAuthenticationHmac(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_SessionNonce(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_DeviceNonce(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_DeviceConfigurationData(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn FinishAuthenticationAsync(&self, deviceHmac: <crate::windows::storage::streams::IBuffer as RtType>::Abi, sessionHmac: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus> as RtType>::Abi) -> HRESULT,
    fn AbortAuthenticationAsync(&self, errorLogMessage: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthentication {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_service_authentication_hmac(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ServiceAuthenticationHmac)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_session_nonce(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SessionNonce)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_device_nonce(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceNonce)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_device_configuration_data(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceConfigurationData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn finish_authentication_async(&self, deviceHmac: &crate::windows::storage::streams::IBuffer, sessionHmac: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FinishAuthenticationAsync)(self.get_abi() as *const _ as *mut _, deviceHmac.get_abi() as *const _ as *mut _, sessionHmac.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn abort_authentication_async(&self, errorLogMessage: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AbortAuthenticationAsync)(self.get_abi() as *const _ as *mut _, errorLogMessage.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthentication: ISecondaryAuthenticationFactorAuthentication}
impl RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics> for SecondaryAuthenticationFactorAuthentication {}
impl SecondaryAuthenticationFactorAuthentication {
    #[inline] pub fn show_notification_message_async(deviceName: &HStringArg, message: SecondaryAuthenticationFactorAuthenticationMessage) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().show_notification_message_async(deviceName, message)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn start_authentication_async(deviceId: &HStringArg, serviceAuthenticationNonce: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().start_authentication_async(deviceId, serviceAuthenticationNonce)
    }
    #[inline] pub fn add_authentication_stage_changed(handler: &foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().add_authentication_stage_changed(handler)
    }
    #[inline] pub fn remove_authentication_stage_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().remove_authentication_stage_changed(token)
    }
    #[inline] pub fn get_authentication_stage_info_async() -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorAuthenticationStatics>>::get_activation_factory().get_authentication_stage_info_async()
    }
}
DEFINE_CLSID!(SecondaryAuthenticationFactorAuthentication(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,80,114,111,118,105,100,101,114,46,83,101,99,111,110,100,97,114,121,65,117,116,104,101,110,116,105,99,97,116,105,111,110,70,97,99,116,111,114,65,117,116,104,101,110,116,105,99,97,116,105,111,110,0]) [CLSID_SecondaryAuthenticationFactorAuthentication]);
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationMessage: i32 {
    Invalid = 0, SwipeUpWelcome = 1, TapWelcome = 2, DeviceNeedsAttention = 3, LookingForDevice = 4, LookingForDevicePluggedin = 5, BluetoothIsDisabled = 6, NfcIsDisabled = 7, WiFiIsDisabled = 8, ExtraTapIsRequired = 9, DisabledByPolicy = 10, TapOnDeviceRequired = 11, HoldFinger = 12, ScanFinger = 13, UnauthorizedUser = 14, ReregisterRequired = 15, TryAgain = 16, SayPassphrase = 17, ReadyToSignIn = 18, UseAnotherSignInOption = 19, ConnectionRequired = 20, TimeLimitExceeded = 21, CanceledByUser = 22, CenterHand = 23, MoveHandCloser = 24, MoveHandFarther = 25, PlaceHandAbove = 26, RecognitionFailed = 27, DeviceUnavailable = 28,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationResult, 2629523847, 61293, 19394, 191, 73, 70, 23, 81, 90, 15, 154);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationResult(ISecondaryAuthenticationFactorAuthenticationResultVtbl): IInspectable [IID_ISecondaryAuthenticationFactorAuthenticationResult] {
    fn get_Status(&self, out: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> HRESULT,
    fn get_Authentication(&self, out: *mut <SecondaryAuthenticationFactorAuthentication as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationResult {
    #[inline] pub fn get_status(&self) -> Result<SecondaryAuthenticationFactorAuthenticationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication(&self) -> Result<Option<SecondaryAuthenticationFactorAuthentication>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Authentication)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SecondaryAuthenticationFactorAuthentication::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationResult: ISecondaryAuthenticationFactorAuthenticationResult}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationScenario: i32 {
    SignIn = 0, CredentialPrompt = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationStage: i32 {
    NotStarted = 0, WaitingForUserConfirmation = 1, CollectingCredential = 2, SuspendingAuthentication = 3, CredentialCollected = 4, CredentialAuthenticated = 5, StoppingAuthentication = 6, ReadyForLock = 7, CheckingDevicePresence = 8,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs, 3567644246, 29329, 16499, 188, 31, 204, 184, 245, 175, 223, 150);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs(ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl): IInspectable [IID_ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs] {
    fn get_StageInfo(&self, out: *mut <SecondaryAuthenticationFactorAuthenticationStageInfo as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    #[inline] pub fn get_stage_info(&self) -> Result<Option<SecondaryAuthenticationFactorAuthenticationStageInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_StageInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SecondaryAuthenticationFactorAuthenticationStageInfo::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStageInfo, 1459536523, 59562, 19471, 142, 76, 165, 89, 231, 58, 221, 136);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorAuthenticationStageInfo(ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl): IInspectable [IID_ISecondaryAuthenticationFactorAuthenticationStageInfo] {
    fn get_Stage(&self, out: *mut SecondaryAuthenticationFactorAuthenticationStage) -> HRESULT,
    fn get_Scenario(&self, out: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> HRESULT,
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStageInfo {
    #[inline] pub fn get_stage(&self) -> Result<SecondaryAuthenticationFactorAuthenticationStage> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Stage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_scenario(&self) -> Result<SecondaryAuthenticationFactorAuthenticationScenario> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Scenario)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorAuthenticationStageInfo: ISecondaryAuthenticationFactorAuthenticationStageInfo}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorAuthenticationStatics, 1062741590, 10488, 19983, 174, 140, 88, 152, 185, 174, 36, 105);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorAuthenticationStatics(ISecondaryAuthenticationFactorAuthenticationStaticsVtbl): IInspectable [IID_ISecondaryAuthenticationFactorAuthenticationStatics] {
    fn ShowNotificationMessageAsync(&self, deviceName: HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn StartAuthenticationAsync(&self, deviceId: HSTRING, serviceAuthenticationNonce: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult> as RtType>::Abi) -> HRESULT,
    fn add_AuthenticationStageChanged(&self, handler: <foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AuthenticationStageChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn GetAuthenticationStageInfoAsync(&self, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo> as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorAuthenticationStatics {
    #[inline] pub fn show_notification_message_async(&self, deviceName: &HStringArg, message: SecondaryAuthenticationFactorAuthenticationMessage) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ShowNotificationMessageAsync)(self.get_abi() as *const _ as *mut _, deviceName.get(), message, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn start_authentication_async(&self, deviceId: &HStringArg, serviceAuthenticationNonce: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().StartAuthenticationAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), serviceAuthenticationNonce.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_authentication_stage_changed(&self, handler: &foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_AuthenticationStageChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_authentication_stage_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_AuthenticationStageChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_stage_info_async(&self) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAuthenticationStageInfoAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SecondaryAuthenticationFactorAuthenticationStatus: i32 {
    Failed = 0, Started = 1, UnknownDevice = 2, DisabledByPolicy = 3, InvalidAuthenticationStage = 4,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDeviceCapabilities: u32 {
    None = 0, SecureStorage = 1, StoreKeys = 2, ConfirmUserIntentToAuthenticate = 4, SupportSecureUserPresenceCheck = 8, TransmittedDataIsEncrypted = 16, HMacSha256 = 32, CloseRangeDataTransmission = 64,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDeviceFindScope: i32 {
    User = 0, AllUsers = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresence: i32 {
    Absent = 0, Present = 1,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresenceMonitoringMode: i32 {
    Unsupported = 0, AppManaged = 1, SystemManaged = 2,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics, 2420742681, 32498, 17699, 149, 28, 164, 23, 162, 74, 207, 147);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics(ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl): IInspectable [IID_ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics] {
    fn RegisterDevicePresenceMonitoringAsync(&self, deviceId: HSTRING, deviceInstancePath: HSTRING, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RegisterDevicePresenceMonitoringWithNewDeviceAsync(&self, deviceId: HSTRING, deviceInstancePath: HSTRING, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: HSTRING, deviceModelNumber: HSTRING, deviceConfigurationData: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus> as RtType>::Abi) -> HRESULT,
    fn UnregisterDevicePresenceMonitoringAsync(&self, deviceId: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn IsDevicePresenceMonitoringSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    #[inline] pub fn register_device_presence_monitoring_async(&self, deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RegisterDevicePresenceMonitoringAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), deviceInstancePath.get(), monitoringMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn register_device_presence_monitoring_with_new_device_async(&self, deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceConfigurationData: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RegisterDevicePresenceMonitoringWithNewDeviceAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), deviceInstancePath.get(), monitoringMode, deviceFriendlyName.get(), deviceModelNumber.get(), deviceConfigurationData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn unregister_device_presence_monitoring_async(&self, deviceId: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnregisterDevicePresenceMonitoringAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_device_presence_monitoring_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsDevicePresenceMonitoringSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus: i32 {
    Unsupported = 0, Succeeded = 1, DisabledByPolicy = 2,
}}
RT_ENUM! { enum SecondaryAuthenticationFactorFinishAuthenticationStatus: i32 {
    Failed = 0, Completed = 1, NonceExpired = 2,
}}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorInfo, 506177633, 34099, 20430, 131, 155, 236, 183, 36, 16, 172, 20);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorInfo(ISecondaryAuthenticationFactorInfoVtbl): IInspectable [IID_ISecondaryAuthenticationFactorInfo] {
    fn get_DeviceId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceFriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DeviceModelNumber(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_DeviceConfigurationData(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorInfo {
    #[inline] pub fn get_device_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceFriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device_model_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceModelNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_device_configuration_data(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DeviceConfigurationData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorInfo: ISecondaryAuthenticationFactorInfo}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorInfo2, 349798819, 64550, 20471, 171, 195, 72, 232, 42, 81, 42, 10);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorInfo2(ISecondaryAuthenticationFactorInfo2Vtbl): IInspectable [IID_ISecondaryAuthenticationFactorInfo2] {
    fn get_PresenceMonitoringMode(&self, out: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> HRESULT,
    fn UpdateDevicePresenceAsync(&self, presenceState: SecondaryAuthenticationFactorDevicePresence, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn get_IsAuthenticationSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISecondaryAuthenticationFactorInfo2 {
    #[inline] pub fn get_presence_monitoring_mode(&self) -> Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PresenceMonitoringMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn update_device_presence_async(&self, presenceState: SecondaryAuthenticationFactorDevicePresence) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UpdateDevicePresenceAsync)(self.get_abi() as *const _ as *mut _, presenceState, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_authentication_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsAuthenticationSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistration, 2672606132, 36026, 18608, 132, 13, 219, 178, 42, 84, 198, 120);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorRegistration(ISecondaryAuthenticationFactorRegistrationVtbl): IInspectable [IID_ISecondaryAuthenticationFactorRegistration] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn FinishRegisteringDeviceAsync(&self, deviceConfigurationData: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AbortRegisteringDeviceAsync(&self, errorLogMessage: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistration {
    #[cfg(feature="windows-storage")] #[inline] pub fn finish_registering_device_async(&self, deviceConfigurationData: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FinishRegisteringDeviceAsync)(self.get_abi() as *const _ as *mut _, deviceConfigurationData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn abort_registering_device_async(&self, errorLogMessage: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AbortRegisteringDeviceAsync)(self.get_abi() as *const _ as *mut _, errorLogMessage.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorRegistration: ISecondaryAuthenticationFactorRegistration}
impl RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics> for SecondaryAuthenticationFactorRegistration {}
impl RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics> for SecondaryAuthenticationFactorRegistration {}
impl SecondaryAuthenticationFactorRegistration {
    #[inline] pub fn register_device_presence_monitoring_async(deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().register_device_presence_monitoring_async(deviceId, deviceInstancePath, monitoringMode)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn register_device_presence_monitoring_with_new_device_async(deviceId: &HStringArg, deviceInstancePath: &HStringArg, monitoringMode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceConfigurationData: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().register_device_presence_monitoring_with_new_device_async(deviceId, deviceInstancePath, monitoringMode, deviceFriendlyName, deviceModelNumber, deviceConfigurationData)
    }
    #[inline] pub fn unregister_device_presence_monitoring_async(deviceId: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().unregister_device_presence_monitoring_async(deviceId)
    }
    #[inline] pub fn is_device_presence_monitoring_supported() -> Result<bool> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics>>::get_activation_factory().is_device_presence_monitoring_supported()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn request_start_registering_device_async(deviceId: &HStringArg, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceKey: &crate::windows::storage::streams::IBuffer, mutualAuthenticationKey: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().request_start_registering_device_async(deviceId, capabilities, deviceFriendlyName, deviceModelNumber, deviceKey, mutualAuthenticationKey)
    }
    #[inline] pub fn find_all_registered_device_info_async(queryType: SecondaryAuthenticationFactorDeviceFindScope) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>>> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().find_all_registered_device_info_async(queryType)
    }
    #[inline] pub fn unregister_device_async(deviceId: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().unregister_device_async(deviceId)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn update_device_configuration_data_async(deviceId: &HStringArg, deviceConfigurationData: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ISecondaryAuthenticationFactorRegistrationStatics>>::get_activation_factory().update_device_configuration_data_async(deviceId, deviceConfigurationData)
    }
}
DEFINE_CLSID!(SecondaryAuthenticationFactorRegistration(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,73,100,101,110,116,105,116,121,46,80,114,111,118,105,100,101,114,46,83,101,99,111,110,100,97,114,121,65,117,116,104,101,110,116,105,99,97,116,105,111,110,70,97,99,116,111,114,82,101,103,105,115,116,114,97,116,105,111,110,0]) [CLSID_SecondaryAuthenticationFactorRegistration]);
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistrationResult, 2768123376, 44515, 18817, 175, 107, 236, 25, 89, 33, 104, 42);
RT_INTERFACE!{interface ISecondaryAuthenticationFactorRegistrationResult(ISecondaryAuthenticationFactorRegistrationResultVtbl): IInspectable [IID_ISecondaryAuthenticationFactorRegistrationResult] {
    fn get_Status(&self, out: *mut SecondaryAuthenticationFactorRegistrationStatus) -> HRESULT,
    fn get_Registration(&self, out: *mut <SecondaryAuthenticationFactorRegistration as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistrationResult {
    #[inline] pub fn get_status(&self) -> Result<SecondaryAuthenticationFactorRegistrationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_registration(&self) -> Result<Option<SecondaryAuthenticationFactorRegistration>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Registration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SecondaryAuthenticationFactorRegistration::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SecondaryAuthenticationFactorRegistrationResult: ISecondaryAuthenticationFactorRegistrationResult}
DEFINE_IID!(IID_ISecondaryAuthenticationFactorRegistrationStatics, 450826085, 58295, 16725, 153, 127, 183, 86, 239, 101, 190, 186);
RT_INTERFACE!{static interface ISecondaryAuthenticationFactorRegistrationStatics(ISecondaryAuthenticationFactorRegistrationStaticsVtbl): IInspectable [IID_ISecondaryAuthenticationFactorRegistrationStatics] {
    #[cfg(feature="windows-storage")] fn RequestStartRegisteringDeviceAsync(&self, deviceId: HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: HSTRING, deviceModelNumber: HSTRING, deviceKey: <crate::windows::storage::streams::IBuffer as RtType>::Abi, mutualAuthenticationKey: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult> as RtType>::Abi) -> HRESULT,
    fn FindAllRegisteredDeviceInfoAsync(&self, queryType: SecondaryAuthenticationFactorDeviceFindScope, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>> as RtType>::Abi) -> HRESULT,
    fn UnregisterDeviceAsync(&self, deviceId: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UpdateDeviceConfigurationDataAsync(&self, deviceId: HSTRING, deviceConfigurationData: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ISecondaryAuthenticationFactorRegistrationStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn request_start_registering_device_async(&self, deviceId: &HStringArg, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, deviceFriendlyName: &HStringArg, deviceModelNumber: &HStringArg, deviceKey: &crate::windows::storage::streams::IBuffer, mutualAuthenticationKey: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestStartRegisteringDeviceAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), capabilities, deviceFriendlyName.get(), deviceModelNumber.get(), deviceKey.get_abi() as *const _ as *mut _, mutualAuthenticationKey.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_registered_device_info_async(&self, queryType: SecondaryAuthenticationFactorDeviceFindScope) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<SecondaryAuthenticationFactorInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllRegisteredDeviceInfoAsync)(self.get_abi() as *const _ as *mut _, queryType, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn unregister_device_async(&self, deviceId: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnregisterDeviceAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn update_device_configuration_data_async(&self, deviceId: &HStringArg, deviceConfigurationData: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UpdateDeviceConfigurationDataAsync)(self.get_abi() as *const _ as *mut _, deviceId.get(), deviceConfigurationData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SecondaryAuthenticationFactorRegistrationStatus: i32 {
    Failed = 0, Started = 1, CanceledByUser = 2, PinSetupRequired = 3, DisabledByPolicy = 4,
}}
} // Windows.Security.Authentication.Identity.Provider
} // Windows.Security.Authentication.Identity
pub mod onlineid { // Windows.Security.Authentication.OnlineId
use crate::prelude::*;
RT_ENUM! { enum CredentialPromptType: i32 {
    PromptIfNeeded = 0, RetypeCredentials = 1, DoNotPrompt = 2,
}}
DEFINE_IID!(IID_IOnlineIdAuthenticator, 2684614026, 10667, 18455, 184, 132, 215, 81, 109, 173, 24, 185);
RT_INTERFACE!{interface IOnlineIdAuthenticator(IOnlineIdAuthenticatorVtbl): IInspectable [IID_IOnlineIdAuthenticator] {
    fn AuthenticateUserAsync(&self, request: <OnlineIdServiceTicketRequest as RtType>::Abi, out: *mut <UserAuthenticationOperation as RtType>::Abi) -> HRESULT,
    fn AuthenticateUserAsyncAdvanced(&self, requests: <foundation::collections::IIterable<OnlineIdServiceTicketRequest> as RtType>::Abi, credentialPromptType: CredentialPromptType, out: *mut <UserAuthenticationOperation as RtType>::Abi) -> HRESULT,
    fn SignOutUserAsync(&self, out: *mut <SignOutUserOperation as RtType>::Abi) -> HRESULT,
    fn put_ApplicationId(&self, value: Guid) -> HRESULT,
    fn get_ApplicationId(&self, out: *mut Guid) -> HRESULT,
    fn get_CanSignOut(&self, out: *mut bool) -> HRESULT,
    fn get_AuthenticatedSafeCustomerId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdAuthenticator {
    #[inline] pub fn authenticate_user_async(&self, request: &OnlineIdServiceTicketRequest) -> Result<UserAuthenticationOperation> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateUserAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(UserAuthenticationOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_user_async_advanced(&self, requests: &foundation::collections::IIterable<OnlineIdServiceTicketRequest>, credentialPromptType: CredentialPromptType) -> Result<UserAuthenticationOperation> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateUserAsyncAdvanced)(self.get_abi() as *const _ as *mut _, requests.get_abi() as *const _ as *mut _, credentialPromptType, &mut out);
        if hr == S_OK { Ok(UserAuthenticationOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn sign_out_user_async(&self) -> Result<SignOutUserOperation> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignOutUserAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SignOutUserOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_application_id(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ApplicationId)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_application_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ApplicationId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_can_sign_out(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CanSignOut)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_authenticated_safe_customer_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AuthenticatedSafeCustomerId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdAuthenticator: IOnlineIdAuthenticator}
impl RtActivatable<IActivationFactory> for OnlineIdAuthenticator {}
DEFINE_CLSID!(OnlineIdAuthenticator(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,65,117,116,104,101,110,116,105,99,97,116,111,114,0]) [CLSID_OnlineIdAuthenticator]);
DEFINE_IID!(IID_IOnlineIdServiceTicket, 3378271359, 55169, 19092, 172, 184, 197, 152, 116, 35, 140, 38);
RT_INTERFACE!{interface IOnlineIdServiceTicket(IOnlineIdServiceTicketVtbl): IInspectable [IID_IOnlineIdServiceTicket] {
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Request(&self, out: *mut <OnlineIdServiceTicketRequest as RtType>::Abi) -> HRESULT,
    fn get_ErrorCode(&self, out: *mut i32) -> HRESULT
}}
impl IOnlineIdServiceTicket {
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_request(&self) -> Result<Option<OnlineIdServiceTicketRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Request)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(OnlineIdServiceTicketRequest::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_error_code(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ErrorCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdServiceTicket: IOnlineIdServiceTicket}
DEFINE_IID!(IID_IOnlineIdServiceTicketRequest, 695485907, 64355, 16693, 137, 9, 78, 53, 76, 6, 20, 102);
RT_INTERFACE!{interface IOnlineIdServiceTicketRequest(IOnlineIdServiceTicketRequestVtbl): IInspectable [IID_IOnlineIdServiceTicketRequest] {
    fn get_Service(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Policy(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdServiceTicketRequest {
    #[inline] pub fn get_service(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Service)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_policy(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Policy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdServiceTicketRequest: IOnlineIdServiceTicketRequest}
impl RtActivatable<IOnlineIdServiceTicketRequestFactory> for OnlineIdServiceTicketRequest {}
impl OnlineIdServiceTicketRequest {
    #[inline] pub fn create_online_id_service_ticket_request(service: &HStringArg, policy: &HStringArg) -> Result<OnlineIdServiceTicketRequest> {
        <Self as RtActivatable<IOnlineIdServiceTicketRequestFactory>>::get_activation_factory().create_online_id_service_ticket_request(service, policy)
    }
    #[inline] pub fn create_online_id_service_ticket_request_advanced(service: &HStringArg) -> Result<OnlineIdServiceTicketRequest> {
        <Self as RtActivatable<IOnlineIdServiceTicketRequestFactory>>::get_activation_factory().create_online_id_service_ticket_request_advanced(service)
    }
}
DEFINE_CLSID!(OnlineIdServiceTicketRequest(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,83,101,114,118,105,99,101,84,105,99,107,101,116,82,101,113,117,101,115,116,0]) [CLSID_OnlineIdServiceTicketRequest]);
DEFINE_IID!(IID_IOnlineIdServiceTicketRequestFactory, 3199928840, 40563, 16503, 150, 20, 8, 97, 76, 11, 194, 69);
RT_INTERFACE!{static interface IOnlineIdServiceTicketRequestFactory(IOnlineIdServiceTicketRequestFactoryVtbl): IInspectable [IID_IOnlineIdServiceTicketRequestFactory] {
    fn CreateOnlineIdServiceTicketRequest(&self, service: HSTRING, policy: HSTRING, out: *mut <OnlineIdServiceTicketRequest as RtType>::Abi) -> HRESULT,
    fn CreateOnlineIdServiceTicketRequestAdvanced(&self, service: HSTRING, out: *mut <OnlineIdServiceTicketRequest as RtType>::Abi) -> HRESULT
}}
impl IOnlineIdServiceTicketRequestFactory {
    #[inline] pub fn create_online_id_service_ticket_request(&self, service: &HStringArg, policy: &HStringArg) -> Result<OnlineIdServiceTicketRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateOnlineIdServiceTicketRequest)(self.get_abi() as *const _ as *mut _, service.get(), policy.get(), &mut out);
        if hr == S_OK { Ok(OnlineIdServiceTicketRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_online_id_service_ticket_request_advanced(&self, service: &HStringArg) -> Result<OnlineIdServiceTicketRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateOnlineIdServiceTicketRequestAdvanced)(self.get_abi() as *const _ as *mut _, service.get(), &mut out);
        if hr == S_OK { Ok(OnlineIdServiceTicketRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class OnlineIdSystemAuthenticator}
impl RtActivatable<IOnlineIdSystemAuthenticatorStatics> for OnlineIdSystemAuthenticator {}
impl OnlineIdSystemAuthenticator {
    #[inline] pub fn get_default() -> Result<Option<OnlineIdSystemAuthenticatorForUser>> {
        <Self as RtActivatable<IOnlineIdSystemAuthenticatorStatics>>::get_activation_factory().get_default()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &crate::windows::system::User) -> Result<Option<OnlineIdSystemAuthenticatorForUser>> {
        <Self as RtActivatable<IOnlineIdSystemAuthenticatorStatics>>::get_activation_factory().get_for_user(user)
    }
}
DEFINE_CLSID!(OnlineIdSystemAuthenticator(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,79,110,108,105,110,101,73,100,46,79,110,108,105,110,101,73,100,83,121,115,116,101,109,65,117,116,104,101,110,116,105,99,97,116,111,114,0]) [CLSID_OnlineIdSystemAuthenticator]);
DEFINE_IID!(IID_IOnlineIdSystemAuthenticatorForUser, 1469628155, 7652, 16774, 162, 230, 181, 99, 248, 106, 175, 68);
RT_INTERFACE!{interface IOnlineIdSystemAuthenticatorForUser(IOnlineIdSystemAuthenticatorForUserVtbl): IInspectable [IID_IOnlineIdSystemAuthenticatorForUser] {
    fn GetTicketAsync(&self, request: <OnlineIdServiceTicketRequest as RtType>::Abi, out: *mut <foundation::IAsyncOperation<OnlineIdSystemTicketResult> as RtType>::Abi) -> HRESULT,
    fn put_ApplicationId(&self, value: Guid) -> HRESULT,
    fn get_ApplicationId(&self, out: *mut Guid) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut <crate::windows::system::User as RtType>::Abi) -> HRESULT
}}
impl IOnlineIdSystemAuthenticatorForUser {
    #[inline] pub fn get_ticket_async(&self, request: &OnlineIdServiceTicketRequest) -> Result<foundation::IAsyncOperation<OnlineIdSystemTicketResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetTicketAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_application_id(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ApplicationId)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_application_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ApplicationId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<crate::windows::system::User>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_User)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::system::User::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdSystemAuthenticatorForUser: IOnlineIdSystemAuthenticatorForUser}
DEFINE_IID!(IID_IOnlineIdSystemAuthenticatorStatics, 2231662482, 63028, 16867, 150, 164, 81, 100, 233, 2, 199, 64);
RT_INTERFACE!{static interface IOnlineIdSystemAuthenticatorStatics(IOnlineIdSystemAuthenticatorStaticsVtbl): IInspectable [IID_IOnlineIdSystemAuthenticatorStatics] {
    fn get_Default(&self, out: *mut <OnlineIdSystemAuthenticatorForUser as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: <crate::windows::system::User as RtType>::Abi, out: *mut <OnlineIdSystemAuthenticatorForUser as RtType>::Abi) -> HRESULT
}}
impl IOnlineIdSystemAuthenticatorStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<OnlineIdSystemAuthenticatorForUser>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Default)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(OnlineIdSystemAuthenticatorForUser::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &crate::windows::system::User) -> Result<Option<OnlineIdSystemAuthenticatorForUser>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetForUser)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(OnlineIdSystemAuthenticatorForUser::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IOnlineIdSystemIdentity, 1950142989, 46794, 17229, 129, 36, 83, 234, 18, 104, 83, 7);
RT_INTERFACE!{interface IOnlineIdSystemIdentity(IOnlineIdSystemIdentityVtbl): IInspectable [IID_IOnlineIdSystemIdentity] {
    fn get_Ticket(&self, out: *mut <OnlineIdServiceTicket as RtType>::Abi) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT
}}
impl IOnlineIdSystemIdentity {
    #[inline] pub fn get_ticket(&self) -> Result<Option<OnlineIdServiceTicket>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ticket)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(OnlineIdServiceTicket::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdSystemIdentity: IOnlineIdSystemIdentity}
DEFINE_IID!(IID_IOnlineIdSystemTicketResult, 3674890232, 45208, 19149, 157, 19, 158, 100, 6, 82, 181, 182);
RT_INTERFACE!{interface IOnlineIdSystemTicketResult(IOnlineIdSystemTicketResultVtbl): IInspectable [IID_IOnlineIdSystemTicketResult] {
    fn get_Identity(&self, out: *mut <OnlineIdSystemIdentity as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut OnlineIdSystemTicketStatus) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl IOnlineIdSystemTicketResult {
    #[inline] pub fn get_identity(&self) -> Result<Option<OnlineIdSystemIdentity>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(OnlineIdSystemIdentity::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<OnlineIdSystemTicketStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ExtendedError)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class OnlineIdSystemTicketResult: IOnlineIdSystemTicketResult}
RT_ENUM! { enum OnlineIdSystemTicketStatus: i32 {
    Success = 0, Error = 1, ServiceConnectionError = 2,
}}
RT_CLASS!{class SignOutUserOperation: foundation::IAsyncAction}
RT_CLASS!{class UserAuthenticationOperation: foundation::IAsyncOperation<UserIdentity>}
DEFINE_IID!(IID_IUserIdentity, 558291405, 1858, 19427, 138, 28, 124, 122, 230, 121, 170, 136);
RT_INTERFACE!{interface IUserIdentity(IUserIdentityVtbl): IInspectable [IID_IUserIdentity] {
    fn get_Tickets(&self, out: *mut <foundation::collections::IVectorView<OnlineIdServiceTicket> as RtType>::Abi) -> HRESULT,
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SafeCustomerId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignInName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FirstName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LastName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IsBetaAccount(&self, out: *mut bool) -> HRESULT,
    fn get_IsConfirmedPC(&self, out: *mut bool) -> HRESULT
}}
impl IUserIdentity {
    #[inline] pub fn get_tickets(&self) -> Result<Option<foundation::collections::IVectorView<OnlineIdServiceTicket>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Tickets)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_safe_customer_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SafeCustomerId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sign_in_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SignInName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FirstName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_LastName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_beta_account(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsBetaAccount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_confirmed_pc(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsConfirmedPC)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class UserIdentity: IUserIdentity}
} // Windows.Security.Authentication.OnlineId
pub mod web { // Windows.Security.Authentication.Web
use crate::prelude::*;
RT_ENUM! { enum TokenBindingKeyType: i32 {
    Rsa2048 = 0, EcdsaP256 = 1, AnyExisting = 2,
}}
RT_CLASS!{static class WebAuthenticationBroker}
impl RtActivatable<IWebAuthenticationBrokerStatics> for WebAuthenticationBroker {}
impl RtActivatable<IWebAuthenticationBrokerStatics2> for WebAuthenticationBroker {}
impl WebAuthenticationBroker {
    #[inline] pub fn authenticate_with_callback_uri_async(options: WebAuthenticationOptions, requestUri: &foundation::Uri, callbackUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().authenticate_with_callback_uri_async(options, requestUri, callbackUri)
    }
    #[inline] pub fn authenticate_without_callback_uri_async(options: WebAuthenticationOptions, requestUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().authenticate_without_callback_uri_async(options, requestUri)
    }
    #[inline] pub fn get_current_application_callback_uri() -> Result<Option<foundation::Uri>> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics>>::get_activation_factory().get_current_application_callback_uri()
    }
    #[inline] pub fn authenticate_and_continue(requestUri: &foundation::Uri) -> Result<()> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_and_continue(requestUri)
    }
    #[inline] pub fn authenticate_with_callback_uri_and_continue(requestUri: &foundation::Uri, callbackUri: &foundation::Uri) -> Result<()> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_with_callback_uri_and_continue(requestUri, callbackUri)
    }
    #[inline] pub fn authenticate_with_callback_uri_continuation_data_and_options_and_continue(requestUri: &foundation::Uri, callbackUri: &foundation::Uri, continuationData: &foundation::collections::ValueSet, options: WebAuthenticationOptions) -> Result<()> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_with_callback_uri_continuation_data_and_options_and_continue(requestUri, callbackUri, continuationData, options)
    }
    #[inline] pub fn authenticate_silently_async(requestUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_silently_async(requestUri)
    }
    #[inline] pub fn authenticate_silently_with_options_async(requestUri: &foundation::Uri, options: WebAuthenticationOptions) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> {
        <Self as RtActivatable<IWebAuthenticationBrokerStatics2>>::get_activation_factory().authenticate_silently_with_options_async(requestUri, options)
    }
}
DEFINE_CLSID!(WebAuthenticationBroker(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,87,101,98,65,117,116,104,101,110,116,105,99,97,116,105,111,110,66,114,111,107,101,114,0]) [CLSID_WebAuthenticationBroker]);
DEFINE_IID!(IID_IWebAuthenticationBrokerStatics, 789880602, 58995, 16565, 188, 34, 32, 26, 104, 100, 163, 123);
RT_INTERFACE!{static interface IWebAuthenticationBrokerStatics(IWebAuthenticationBrokerStaticsVtbl): IInspectable [IID_IWebAuthenticationBrokerStatics] {
    fn AuthenticateWithCallbackUriAsync(&self, options: WebAuthenticationOptions, requestUri: <foundation::Uri as RtType>::Abi, callbackUri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebAuthenticationResult> as RtType>::Abi) -> HRESULT,
    fn AuthenticateWithoutCallbackUriAsync(&self, options: WebAuthenticationOptions, requestUri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebAuthenticationResult> as RtType>::Abi) -> HRESULT,
    fn GetCurrentApplicationCallbackUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationBrokerStatics {
    #[inline] pub fn authenticate_with_callback_uri_async(&self, options: WebAuthenticationOptions, requestUri: &foundation::Uri, callbackUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateWithCallbackUriAsync)(self.get_abi() as *const _ as *mut _, options, requestUri.get_abi() as *const _ as *mut _, callbackUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_without_callback_uri_async(&self, options: WebAuthenticationOptions, requestUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateWithoutCallbackUriAsync)(self.get_abi() as *const _ as *mut _, options, requestUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_application_callback_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetCurrentApplicationCallbackUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAuthenticationBrokerStatics2, 1942879134, 5351, 16858, 169, 113, 170, 244, 65, 11, 98, 30);
RT_INTERFACE!{static interface IWebAuthenticationBrokerStatics2(IWebAuthenticationBrokerStatics2Vtbl): IInspectable [IID_IWebAuthenticationBrokerStatics2] {
    fn AuthenticateAndContinue(&self, requestUri: <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn AuthenticateWithCallbackUriAndContinue(&self, requestUri: <foundation::Uri as RtType>::Abi, callbackUri: <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(&self, requestUri: <foundation::Uri as RtType>::Abi, callbackUri: <foundation::Uri as RtType>::Abi, continuationData: <foundation::collections::ValueSet as RtType>::Abi, options: WebAuthenticationOptions) -> HRESULT,
    fn AuthenticateSilentlyAsync(&self, requestUri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebAuthenticationResult> as RtType>::Abi) -> HRESULT,
    fn AuthenticateSilentlyWithOptionsAsync(&self, requestUri: <foundation::Uri as RtType>::Abi, options: WebAuthenticationOptions, out: *mut <foundation::IAsyncOperation<WebAuthenticationResult> as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationBrokerStatics2 {
    #[inline] pub fn authenticate_and_continue(&self, requestUri: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AuthenticateAndContinue)(self.get_abi() as *const _ as *mut _, requestUri.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_with_callback_uri_and_continue(&self, requestUri: &foundation::Uri, callbackUri: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AuthenticateWithCallbackUriAndContinue)(self.get_abi() as *const _ as *mut _, requestUri.get_abi() as *const _ as *mut _, callbackUri.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_with_callback_uri_continuation_data_and_options_and_continue(&self, requestUri: &foundation::Uri, callbackUri: &foundation::Uri, continuationData: &foundation::collections::ValueSet, options: WebAuthenticationOptions) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue)(self.get_abi() as *const _ as *mut _, requestUri.get_abi() as *const _ as *mut _, callbackUri.get_abi() as *const _ as *mut _, continuationData.get_abi() as *const _ as *mut _, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_silently_async(&self, requestUri: &foundation::Uri) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateSilentlyAsync)(self.get_abi() as *const _ as *mut _, requestUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn authenticate_silently_with_options_async(&self, requestUri: &foundation::Uri, options: WebAuthenticationOptions) -> Result<foundation::IAsyncOperation<WebAuthenticationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AuthenticateSilentlyWithOptionsAsync)(self.get_abi() as *const _ as *mut _, requestUri.get_abi() as *const _ as *mut _, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAuthenticationOptions: u32 {
    None = 0, SilentMode = 1, UseTitle = 2, UseHttpPost = 4, UseCorporateNetwork = 8,
}}
DEFINE_IID!(IID_IWebAuthenticationResult, 1677732683, 60905, 18186, 165, 205, 3, 35, 250, 246, 226, 98);
RT_INTERFACE!{interface IWebAuthenticationResult(IWebAuthenticationResultVtbl): IInspectable [IID_IWebAuthenticationResult] {
    fn get_ResponseData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ResponseStatus(&self, out: *mut WebAuthenticationStatus) -> HRESULT,
    fn get_ResponseErrorDetail(&self, out: *mut u32) -> HRESULT
}}
impl IWebAuthenticationResult {
    #[inline] pub fn get_response_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResponseData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_response_status(&self) -> Result<WebAuthenticationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ResponseStatus)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_response_error_detail(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ResponseErrorDetail)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAuthenticationResult: IWebAuthenticationResult}
RT_ENUM! { enum WebAuthenticationStatus: i32 {
    Success = 0, UserCancel = 1, ErrorHttp = 2,
}}
pub mod core { // Windows.Security.Authentication.Web.Core
use crate::prelude::*;
DEFINE_IID!(IID_IFindAllAccountsResult, 2776705885, 46894, 16908, 134, 171, 170, 192, 215, 183, 38, 31);
RT_INTERFACE!{interface IFindAllAccountsResult(IFindAllAccountsResultVtbl): IInspectable [IID_IFindAllAccountsResult] {
    fn get_Accounts(&self, out: *mut <foundation::collections::IVectorView<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut FindAllWebAccountsStatus) -> HRESULT,
    fn get_ProviderError(&self, out: *mut <WebProviderError as RtType>::Abi) -> HRESULT
}}
impl IFindAllAccountsResult {
    #[inline] pub fn get_accounts(&self) -> Result<Option<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Accounts)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<FindAllWebAccountsStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_error(&self) -> Result<Option<WebProviderError>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProviderError)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebProviderError::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FindAllAccountsResult: IFindAllAccountsResult}
RT_ENUM! { enum FindAllWebAccountsStatus: i32 {
    Success = 0, NotAllowedByProvider = 1, NotSupportedByProvider = 2, ProviderError = 3,
}}
DEFINE_IID!(IID_IWebAccountEventArgs, 1874264957, 16974, 17644, 151, 124, 239, 36, 21, 70, 42, 90);
RT_INTERFACE!{interface IWebAccountEventArgs(IWebAccountEventArgsVtbl): IInspectable [IID_IWebAccountEventArgs] {
    fn get_Account(&self, out: *mut <super::super::super::credentials::WebAccount as RtType>::Abi) -> HRESULT
}}
impl IWebAccountEventArgs {
    #[inline] pub fn get_account(&self) -> Result<Option<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Account)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccount::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountEventArgs: IWebAccountEventArgs}
DEFINE_IID!(IID_IWebAccountMonitor, 1950742013, 43677, 17945, 141, 93, 193, 56, 164, 237, 227, 229);
RT_INTERFACE!{interface IWebAccountMonitor(IWebAccountMonitorVtbl): IInspectable [IID_IWebAccountMonitor] {
    fn add_Updated(&self, handler: <foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: <foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DefaultSignInAccountChanged(&self, handler: <foundation::TypedEventHandler<WebAccountMonitor, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DefaultSignInAccountChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IWebAccountMonitor {
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_Updated)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_Updated)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_Removed)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_Removed)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_default_sign_in_account_changed(&self, handler: &foundation::TypedEventHandler<WebAccountMonitor, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_DefaultSignInAccountChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_default_sign_in_account_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_DefaultSignInAccountChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountMonitor: IWebAccountMonitor}
RT_CLASS!{static class WebAuthenticationCoreManager}
impl RtActivatable<IWebAuthenticationCoreManagerStatics> for WebAuthenticationCoreManager {}
impl RtActivatable<IWebAuthenticationCoreManagerStatics2> for WebAuthenticationCoreManager {}
impl RtActivatable<IWebAuthenticationCoreManagerStatics3> for WebAuthenticationCoreManager {}
impl RtActivatable<IWebAuthenticationCoreManagerStatics4> for WebAuthenticationCoreManager {}
impl WebAuthenticationCoreManager {
    #[inline] pub fn get_token_silently_async(request: &WebTokenRequest) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().get_token_silently_async(request)
    }
    #[inline] pub fn get_token_silently_with_web_account_async(request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().get_token_silently_with_web_account_async(request, webAccount)
    }
    #[inline] pub fn request_token_async(request: &WebTokenRequest) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().request_token_async(request)
    }
    #[inline] pub fn request_token_with_web_account_async(request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().request_token_with_web_account_async(request, webAccount)
    }
    #[inline] pub fn find_account_async(provider: &super::super::super::credentials::WebAccountProvider, webAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_async(provider, webAccountId)
    }
    #[inline] pub fn find_account_provider_async(webAccountProviderId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_provider_async(webAccountProviderId)
    }
    #[inline] pub fn find_account_provider_with_authority_async(webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics>>::get_activation_factory().find_account_provider_with_authority_async(webAccountProviderId, authority)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn find_account_provider_with_authority_for_user_async(webAccountProviderId: &HStringArg, authority: &HStringArg, user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics2>>::get_activation_factory().find_account_provider_with_authority_for_user_async(webAccountProviderId, authority, user)
    }
    #[inline] pub fn create_web_account_monitor(webAccounts: &foundation::collections::IIterable<super::super::super::credentials::WebAccount>) -> Result<Option<WebAccountMonitor>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics3>>::get_activation_factory().create_web_account_monitor(webAccounts)
    }
    #[inline] pub fn find_all_accounts_async(provider: &super::super::super::credentials::WebAccountProvider) -> Result<foundation::IAsyncOperation<FindAllAccountsResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics4>>::get_activation_factory().find_all_accounts_async(provider)
    }
    #[inline] pub fn find_all_accounts_with_client_id_async(provider: &super::super::super::credentials::WebAccountProvider, clientId: &HStringArg) -> Result<foundation::IAsyncOperation<FindAllAccountsResult>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics4>>::get_activation_factory().find_all_accounts_with_client_id_async(provider, clientId)
    }
    #[inline] pub fn find_system_account_provider_async(webAccountProviderId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics4>>::get_activation_factory().find_system_account_provider_async(webAccountProviderId)
    }
    #[inline] pub fn find_system_account_provider_with_authority_async(webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics4>>::get_activation_factory().find_system_account_provider_with_authority_async(webAccountProviderId, authority)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn find_system_account_provider_with_authority_for_user_async(webAccountProviderId: &HStringArg, authority: &HStringArg, user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> {
        <Self as RtActivatable<IWebAuthenticationCoreManagerStatics4>>::get_activation_factory().find_system_account_provider_with_authority_for_user_async(webAccountProviderId, authority, user)
    }
}
DEFINE_CLSID!(WebAuthenticationCoreManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,65,117,116,104,101,110,116,105,99,97,116,105,111,110,67,111,114,101,77,97,110,97,103,101,114,0]) [CLSID_WebAuthenticationCoreManager]);
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics, 1791655058, 42369, 17529, 156, 16, 117, 46, 255, 68, 253, 52);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics(IWebAuthenticationCoreManagerStaticsVtbl): IInspectable [IID_IWebAuthenticationCoreManagerStatics] {
    fn GetTokenSilentlyAsync(&self, request: <WebTokenRequest as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebTokenRequestResult> as RtType>::Abi) -> HRESULT,
    fn GetTokenSilentlyWithWebAccountAsync(&self, request: <WebTokenRequest as RtType>::Abi, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebTokenRequestResult> as RtType>::Abi) -> HRESULT,
    fn RequestTokenAsync(&self, request: <WebTokenRequest as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebTokenRequestResult> as RtType>::Abi) -> HRESULT,
    fn RequestTokenWithWebAccountAsync(&self, request: <WebTokenRequest as RtType>::Abi, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncOperation<WebTokenRequestResult> as RtType>::Abi) -> HRESULT,
    fn FindAccountAsync(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, webAccountId: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn FindAccountProviderAsync(&self, webAccountProviderId: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT,
    fn FindAccountProviderWithAuthorityAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics {
    #[inline] pub fn get_token_silently_async(&self, request: &WebTokenRequest) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetTokenSilentlyAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token_silently_with_web_account_async(&self, request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetTokenSilentlyWithWebAccountAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_token_async(&self, request: &WebTokenRequest) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestTokenAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_token_with_web_account_async(&self, request: &WebTokenRequest, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<WebTokenRequestResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestTokenWithWebAccountAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_account_async(&self, provider: &super::super::super::credentials::WebAccountProvider, webAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAccountAsync)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, webAccountId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_account_provider_async(&self, webAccountProviderId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAccountProviderAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_account_provider_with_authority_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAccountProviderWithAuthorityAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), authority.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics2, 4119074890, 35671, 18464, 182, 164, 112, 165, 182, 252, 244, 74);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics2(IWebAuthenticationCoreManagerStatics2Vtbl): IInspectable [IID_IWebAuthenticationCoreManagerStatics2] {
    #[cfg(feature="windows-system")] fn FindAccountProviderWithAuthorityForUserAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, user: <crate::windows::system::User as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn find_account_provider_with_authority_for_user_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg, user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAccountProviderWithAuthorityForUserAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), authority.get(), user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics3, 604303026, 35108, 19859, 171, 58, 153, 104, 139, 65, 157, 86);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics3(IWebAuthenticationCoreManagerStatics3Vtbl): IInspectable [IID_IWebAuthenticationCoreManagerStatics3] {
    fn CreateWebAccountMonitor(&self, webAccounts: <foundation::collections::IIterable<super::super::super::credentials::WebAccount> as RtType>::Abi, out: *mut <WebAccountMonitor as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics3 {
    #[inline] pub fn create_web_account_monitor(&self, webAccounts: &foundation::collections::IIterable<super::super::super::credentials::WebAccount>) -> Result<Option<WebAccountMonitor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWebAccountMonitor)(self.get_abi() as *const _ as *mut _, webAccounts.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebAccountMonitor::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAuthenticationCoreManagerStatics4, 1424372734, 38624, 16872, 152, 50, 18, 152, 137, 124, 42, 175);
RT_INTERFACE!{static interface IWebAuthenticationCoreManagerStatics4(IWebAuthenticationCoreManagerStatics4Vtbl): IInspectable [IID_IWebAuthenticationCoreManagerStatics4] {
    fn FindAllAccountsAsync(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, out: *mut <foundation::IAsyncOperation<FindAllAccountsResult> as RtType>::Abi) -> HRESULT,
    fn FindAllAccountsWithClientIdAsync(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, clientId: HSTRING, out: *mut <foundation::IAsyncOperation<FindAllAccountsResult> as RtType>::Abi) -> HRESULT,
    fn FindSystemAccountProviderAsync(&self, webAccountProviderId: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT,
    fn FindSystemAccountProviderWithAuthorityAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn FindSystemAccountProviderWithAuthorityForUserAsync(&self, webAccountProviderId: HSTRING, authority: HSTRING, user: <crate::windows::system::User as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider> as RtType>::Abi) -> HRESULT
}}
impl IWebAuthenticationCoreManagerStatics4 {
    #[inline] pub fn find_all_accounts_async(&self, provider: &super::super::super::credentials::WebAccountProvider) -> Result<foundation::IAsyncOperation<FindAllAccountsResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllAccountsAsync)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_accounts_with_client_id_async(&self, provider: &super::super::super::credentials::WebAccountProvider, clientId: &HStringArg) -> Result<foundation::IAsyncOperation<FindAllAccountsResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllAccountsWithClientIdAsync)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, clientId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_system_account_provider_async(&self, webAccountProviderId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindSystemAccountProviderAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_system_account_provider_with_authority_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindSystemAccountProviderWithAuthorityAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), authority.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn find_system_account_provider_with_authority_for_user_async(&self, webAccountProviderId: &HStringArg, authority: &HStringArg, user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindSystemAccountProviderWithAuthorityForUserAsync)(self.get_abi() as *const _ as *mut _, webAccountProviderId.get(), authority.get(), user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebProviderError, 3675855793, 20677, 18441, 141, 202, 9, 201, 148, 16, 36, 92);
RT_INTERFACE!{interface IWebProviderError(IWebProviderErrorVtbl): IInspectable [IID_IWebProviderError] {
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_ErrorMessage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IMap<HString, HString> as RtType>::Abi) -> HRESULT
}}
impl IWebProviderError {
    #[inline] pub fn get_error_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ErrorCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_error_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ErrorMessage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IMap<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebProviderError: IWebProviderError}
impl RtActivatable<IWebProviderErrorFactory> for WebProviderError {}
impl WebProviderError {
    #[inline] pub fn create(errorCode: u32, errorMessage: &HStringArg) -> Result<WebProviderError> {
        <Self as RtActivatable<IWebProviderErrorFactory>>::get_activation_factory().create(errorCode, errorMessage)
    }
}
DEFINE_CLSID!(WebProviderError(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,80,114,111,118,105,100,101,114,69,114,114,111,114,0]) [CLSID_WebProviderError]);
DEFINE_IID!(IID_IWebProviderErrorFactory, 3821275693, 35311, 20023, 132, 127, 168, 185, 213, 163, 41, 16);
RT_INTERFACE!{static interface IWebProviderErrorFactory(IWebProviderErrorFactoryVtbl): IInspectable [IID_IWebProviderErrorFactory] {
    fn Create(&self, errorCode: u32, errorMessage: HSTRING, out: *mut <WebProviderError as RtType>::Abi) -> HRESULT
}}
impl IWebProviderErrorFactory {
    #[inline] pub fn create(&self, errorCode: u32, errorMessage: &HStringArg) -> Result<WebProviderError> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, errorCode, errorMessage.get(), &mut out);
        if hr == S_OK { Ok(WebProviderError::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebTokenRequest, 3078311272, 44491, 18035, 179, 100, 12, 247, 179, 92, 175, 151);
RT_INTERFACE!{interface IWebTokenRequest(IWebTokenRequestVtbl): IInspectable [IID_IWebTokenRequest] {
    fn get_WebAccountProvider(&self, out: *mut <super::super::super::credentials::WebAccountProvider as RtType>::Abi) -> HRESULT,
    fn get_Scope(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ClientId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PromptType(&self, out: *mut WebTokenRequestPromptType) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IMap<HString, HString> as RtType>::Abi) -> HRESULT
}}
impl IWebTokenRequest {
    #[inline] pub fn get_web_account_provider(&self) -> Result<Option<super::super::super::credentials::WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccountProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccountProvider::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scope(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Scope)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_client_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ClientId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_prompt_type(&self) -> Result<WebTokenRequestPromptType> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PromptType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IMap<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebTokenRequest: IWebTokenRequest}
impl RtActivatable<IWebTokenRequestFactory> for WebTokenRequest {}
impl WebTokenRequest {
    #[inline] pub fn create(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg) -> Result<WebTokenRequest> {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create(provider, scope, clientId)
    }
    #[inline] pub fn create_with_prompt_type(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg, promptType: WebTokenRequestPromptType) -> Result<WebTokenRequest> {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_prompt_type(provider, scope, clientId, promptType)
    }
    #[inline] pub fn create_with_provider(provider: &super::super::super::credentials::WebAccountProvider) -> Result<WebTokenRequest> {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_provider(provider)
    }
    #[inline] pub fn create_with_scope(provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg) -> Result<WebTokenRequest> {
        <Self as RtActivatable<IWebTokenRequestFactory>>::get_activation_factory().create_with_scope(provider, scope)
    }
}
DEFINE_CLSID!(WebTokenRequest(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,84,111,107,101,110,82,101,113,117,101,115,116,0]) [CLSID_WebTokenRequest]);
DEFINE_IID!(IID_IWebTokenRequest2, 3607150713, 12488, 17303, 150, 84, 150, 28, 59, 232, 184, 85);
RT_INTERFACE!{interface IWebTokenRequest2(IWebTokenRequest2Vtbl): IInspectable [IID_IWebTokenRequest2] {
    fn get_AppProperties(&self, out: *mut <foundation::collections::IMap<HString, HString> as RtType>::Abi) -> HRESULT
}}
impl IWebTokenRequest2 {
    #[inline] pub fn get_app_properties(&self) -> Result<Option<foundation::collections::IMap<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AppProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebTokenRequest3, 1517640529, 15281, 16805, 166, 61, 144, 188, 50, 199, 219, 154);
RT_INTERFACE!{interface IWebTokenRequest3(IWebTokenRequest3Vtbl): IInspectable [IID_IWebTokenRequest3] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CorrelationId(&self, value: HSTRING) -> HRESULT
}}
impl IWebTokenRequest3 {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CorrelationId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_correlation_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CorrelationId)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebTokenRequestFactory, 1827804188, 4080, 19559, 184, 79, 153, 221, 190, 74, 114, 201);
RT_INTERFACE!{static interface IWebTokenRequestFactory(IWebTokenRequestFactoryVtbl): IInspectable [IID_IWebTokenRequestFactory] {
    fn Create(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, scope: HSTRING, clientId: HSTRING, out: *mut <WebTokenRequest as RtType>::Abi) -> HRESULT,
    fn CreateWithPromptType(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, scope: HSTRING, clientId: HSTRING, promptType: WebTokenRequestPromptType, out: *mut <WebTokenRequest as RtType>::Abi) -> HRESULT,
    fn CreateWithProvider(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, out: *mut <WebTokenRequest as RtType>::Abi) -> HRESULT,
    fn CreateWithScope(&self, provider: <super::super::super::credentials::WebAccountProvider as RtType>::Abi, scope: HSTRING, out: *mut <WebTokenRequest as RtType>::Abi) -> HRESULT
}}
impl IWebTokenRequestFactory {
    #[inline] pub fn create(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg) -> Result<WebTokenRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, scope.get(), clientId.get(), &mut out);
        if hr == S_OK { Ok(WebTokenRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_prompt_type(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg, clientId: &HStringArg, promptType: WebTokenRequestPromptType) -> Result<WebTokenRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithPromptType)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, scope.get(), clientId.get(), promptType, &mut out);
        if hr == S_OK { Ok(WebTokenRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_provider(&self, provider: &super::super::super::credentials::WebAccountProvider) -> Result<WebTokenRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithProvider)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebTokenRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_scope(&self, provider: &super::super::super::credentials::WebAccountProvider, scope: &HStringArg) -> Result<WebTokenRequest> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithScope)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _, scope.get(), &mut out);
        if hr == S_OK { Ok(WebTokenRequest::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebTokenRequestPromptType: i32 {
    Default = 0, ForceAuthentication = 1,
}}
DEFINE_IID!(IID_IWebTokenRequestResult, 3240788741, 53752, 17539, 141, 84, 56, 254, 41, 39, 132, 255);
RT_INTERFACE!{interface IWebTokenRequestResult(IWebTokenRequestResultVtbl): IInspectable [IID_IWebTokenRequestResult] {
    fn get_ResponseData(&self, out: *mut <foundation::collections::IVectorView<WebTokenResponse> as RtType>::Abi) -> HRESULT,
    fn get_ResponseStatus(&self, out: *mut WebTokenRequestStatus) -> HRESULT,
    fn get_ResponseError(&self, out: *mut <WebProviderError as RtType>::Abi) -> HRESULT,
    fn InvalidateCacheAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebTokenRequestResult {
    #[inline] pub fn get_response_data(&self) -> Result<Option<foundation::collections::IVectorView<WebTokenResponse>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResponseData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_response_status(&self) -> Result<WebTokenRequestStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ResponseStatus)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_response_error(&self) -> Result<Option<WebProviderError>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ResponseError)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebProviderError::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn invalidate_cache_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().InvalidateCacheAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebTokenRequestResult: IWebTokenRequestResult}
RT_ENUM! { enum WebTokenRequestStatus: i32 {
    Success = 0, UserCancel = 1, AccountSwitch = 2, UserInteractionRequired = 3, AccountProviderNotAvailable = 4, ProviderError = 5,
}}
DEFINE_IID!(IID_IWebTokenResponse, 1739048394, 33782, 17606, 163, 177, 14, 182, 158, 65, 250, 138);
RT_INTERFACE!{interface IWebTokenResponse(IWebTokenResponseVtbl): IInspectable [IID_IWebTokenResponse] {
    fn get_Token(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProviderError(&self, out: *mut <WebProviderError as RtType>::Abi) -> HRESULT,
    fn get_WebAccount(&self, out: *mut <super::super::super::credentials::WebAccount as RtType>::Abi) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IMap<HString, HString> as RtType>::Abi) -> HRESULT
}}
impl IWebTokenResponse {
    #[inline] pub fn get_token(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Token)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_error(&self) -> Result<Option<WebProviderError>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProviderError)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebProviderError::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_web_account(&self) -> Result<Option<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccount::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IMap<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebTokenResponse: IWebTokenResponse}
impl RtActivatable<IWebTokenResponseFactory> for WebTokenResponse {}
impl RtActivatable<IActivationFactory> for WebTokenResponse {}
impl WebTokenResponse {
    #[inline] pub fn create_with_token(token: &HStringArg) -> Result<WebTokenResponse> {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token(token)
    }
    #[inline] pub fn create_with_token_and_account(token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount) -> Result<WebTokenResponse> {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token_and_account(token, webAccount)
    }
    #[inline] pub fn create_with_token_account_and_error(token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount, error: &WebProviderError) -> Result<WebTokenResponse> {
        <Self as RtActivatable<IWebTokenResponseFactory>>::get_activation_factory().create_with_token_account_and_error(token, webAccount, error)
    }
}
DEFINE_CLSID!(WebTokenResponse(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,67,111,114,101,46,87,101,98,84,111,107,101,110,82,101,115,112,111,110,115,101,0]) [CLSID_WebTokenResponse]);
DEFINE_IID!(IID_IWebTokenResponseFactory, 2875979768, 21584, 20214, 151, 247, 5, 43, 4, 49, 192, 240);
RT_INTERFACE!{static interface IWebTokenResponseFactory(IWebTokenResponseFactoryVtbl): IInspectable [IID_IWebTokenResponseFactory] {
    fn CreateWithToken(&self, token: HSTRING, out: *mut <WebTokenResponse as RtType>::Abi) -> HRESULT,
    fn CreateWithTokenAndAccount(&self, token: HSTRING, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <WebTokenResponse as RtType>::Abi) -> HRESULT,
    fn CreateWithTokenAccountAndError(&self, token: HSTRING, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, error: <WebProviderError as RtType>::Abi, out: *mut <WebTokenResponse as RtType>::Abi) -> HRESULT
}}
impl IWebTokenResponseFactory {
    #[inline] pub fn create_with_token(&self, token: &HStringArg) -> Result<WebTokenResponse> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithToken)(self.get_abi() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(WebTokenResponse::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_token_and_account(&self, token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount) -> Result<WebTokenResponse> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithTokenAndAccount)(self.get_abi() as *const _ as *mut _, token.get(), webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebTokenResponse::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_token_account_and_error(&self, token: &HStringArg, webAccount: &super::super::super::credentials::WebAccount, error: &WebProviderError) -> Result<WebTokenResponse> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithTokenAccountAndError)(self.get_abi() as *const _ as *mut _, token.get(), webAccount.get_abi() as *const _ as *mut _, error.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebTokenResponse::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Security.Authentication.Web.Core
pub mod provider { // Windows.Security.Authentication.Web.Provider
use crate::prelude::*;
DEFINE_IID!(IID_IWebAccountClientView, 3887949498, 3015, 19558, 191, 212, 101, 211, 8, 44, 188, 168);
RT_INTERFACE!{interface IWebAccountClientView(IWebAccountClientViewVtbl): IInspectable [IID_IWebAccountClientView] {
    fn get_ApplicationCallbackUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn get_Type(&self, out: *mut WebAccountClientViewType) -> HRESULT,
    fn get_AccountPairwiseId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountClientView {
    #[inline] pub fn get_application_callback_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationCallbackUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<WebAccountClientViewType> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Type)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_account_pairwise_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AccountPairwiseId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountClientView: IWebAccountClientView}
impl RtActivatable<IWebAccountClientViewFactory> for WebAccountClientView {}
impl WebAccountClientView {
    #[inline] pub fn create(viewType: WebAccountClientViewType, applicationCallbackUri: &foundation::Uri) -> Result<WebAccountClientView> {
        <Self as RtActivatable<IWebAccountClientViewFactory>>::get_activation_factory().create(viewType, applicationCallbackUri)
    }
    #[inline] pub fn create_with_pairwise_id(viewType: WebAccountClientViewType, applicationCallbackUri: &foundation::Uri, accountPairwiseId: &HStringArg) -> Result<WebAccountClientView> {
        <Self as RtActivatable<IWebAccountClientViewFactory>>::get_activation_factory().create_with_pairwise_id(viewType, applicationCallbackUri, accountPairwiseId)
    }
}
DEFINE_CLSID!(WebAccountClientView(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,65,99,99,111,117,110,116,67,108,105,101,110,116,86,105,101,119,0]) [CLSID_WebAccountClientView]);
DEFINE_IID!(IID_IWebAccountClientViewFactory, 1634539172, 56866, 18517, 163, 38, 6, 206, 191, 42, 63, 35);
RT_INTERFACE!{static interface IWebAccountClientViewFactory(IWebAccountClientViewFactoryVtbl): IInspectable [IID_IWebAccountClientViewFactory] {
    fn Create(&self, viewType: WebAccountClientViewType, applicationCallbackUri: <foundation::Uri as RtType>::Abi, out: *mut <WebAccountClientView as RtType>::Abi) -> HRESULT,
    fn CreateWithPairwiseId(&self, viewType: WebAccountClientViewType, applicationCallbackUri: <foundation::Uri as RtType>::Abi, accountPairwiseId: HSTRING, out: *mut <WebAccountClientView as RtType>::Abi) -> HRESULT
}}
impl IWebAccountClientViewFactory {
    #[inline] pub fn create(&self, viewType: WebAccountClientViewType, applicationCallbackUri: &foundation::Uri) -> Result<WebAccountClientView> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, viewType, applicationCallbackUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebAccountClientView::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_pairwise_id(&self, viewType: WebAccountClientViewType, applicationCallbackUri: &foundation::Uri, accountPairwiseId: &HStringArg) -> Result<WebAccountClientView> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithPairwiseId)(self.get_abi() as *const _ as *mut _, viewType, applicationCallbackUri.get_abi() as *const _ as *mut _, accountPairwiseId.get(), &mut out);
        if hr == S_OK { Ok(WebAccountClientView::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountClientViewType: i32 {
    IdOnly = 0, IdAndProperties = 1,
}}
RT_CLASS!{static class WebAccountManager}
impl RtActivatable<IWebAccountManagerStatics> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics2> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics3> for WebAccountManager {}
impl RtActivatable<IWebAccountManagerStatics4> for WebAccountManager {}
impl RtActivatable<IWebAccountMapManagerStatics> for WebAccountManager {}
impl RtActivatable<IWebAccountScopeManagerStatics> for WebAccountManager {}
impl WebAccountManager {
    #[inline] pub fn update_web_account_properties_async(webAccount: &super::super::super::credentials::WebAccount, webAccountUserName: &HStringArg, additionalProperties: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().update_web_account_properties_async(webAccount, webAccountUserName, additionalProperties)
    }
    #[inline] pub fn add_web_account_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().add_web_account_async(webAccountId, webAccountUserName, props)
    }
    #[inline] pub fn delete_web_account_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().delete_web_account_async(webAccount)
    }
    #[inline] pub fn find_all_provider_web_accounts_async() -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().find_all_provider_web_accounts_async()
    }
    #[cfg(feature="windows-web")] #[inline] pub fn push_cookies_async(uri: &foundation::Uri, cookies: &foundation::collections::IVectorView<crate::windows::web::http::HttpCookie>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().push_cookies_async(uri, cookies)
    }
    #[inline] pub fn set_view_async(webAccount: &super::super::super::credentials::WebAccount, view: &WebAccountClientView) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().set_view_async(webAccount, view)
    }
    #[inline] pub fn clear_view_async(webAccount: &super::super::super::credentials::WebAccount, applicationCallbackUri: &foundation::Uri) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().clear_view_async(webAccount, applicationCallbackUri)
    }
    #[inline] pub fn get_views_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<WebAccountClientView>>> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().get_views_async(webAccount)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn set_web_account_picture_async(webAccount: &super::super::super::credentials::WebAccount, webAccountPicture: &crate::windows::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().set_web_account_picture_async(webAccount, webAccountPicture)
    }
    #[inline] pub fn clear_web_account_picture_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics>>::get_activation_factory().clear_web_account_picture_async(webAccount)
    }
    #[inline] pub fn pull_cookies_async(uriString: &HStringArg, callerPFN: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics2>>::get_activation_factory().pull_cookies_async(uriString, callerPFN)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn find_all_provider_web_accounts_for_user_async(user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().find_all_provider_web_accounts_for_user_async(user)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_for_user_async(user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_for_user_async(user, webAccountId, webAccountUserName, props)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_for_user_async(user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_with_scope_for_user_async(user, webAccountId, webAccountUserName, props, scope)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_and_map_for_user_async(user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountManagerStatics3>>::get_activation_factory().add_web_account_with_scope_and_map_for_user_async(user, webAccountId, webAccountUserName, props, scope, perUserWebAccountId)
    }
    #[inline] pub fn invalidate_app_cache_for_all_accounts_async() -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics4>>::get_activation_factory().invalidate_app_cache_for_all_accounts_async()
    }
    #[inline] pub fn invalidate_app_cache_for_account_async(webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountManagerStatics4>>::get_activation_factory().invalidate_app_cache_for_account_async(webAccount)
    }
    #[inline] pub fn add_web_account_with_scope_and_map_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().add_web_account_with_scope_and_map_async(webAccountId, webAccountUserName, props, scope, perUserWebAccountId)
    }
    #[inline] pub fn set_per_app_to_per_user_account_async(perAppAccount: &super::super::super::credentials::WebAccount, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().set_per_app_to_per_user_account_async(perAppAccount, perUserWebAccountId)
    }
    #[inline] pub fn get_per_user_from_per_app_account_async(perAppAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().get_per_user_from_per_app_account_async(perAppAccount)
    }
    #[inline] pub fn clear_per_user_from_per_app_account_async(perAppAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountMapManagerStatics>>::get_activation_factory().clear_per_user_from_per_app_account_async(perAppAccount)
    }
    #[inline] pub fn add_web_account_with_scope_async(webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().add_web_account_with_scope_async(webAccountId, webAccountUserName, props, scope)
    }
    #[inline] pub fn set_scope_async(webAccount: &super::super::super::credentials::WebAccount, scope: WebAccountScope) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().set_scope_async(webAccount, scope)
    }
    #[inline] pub fn get_scope(webAccount: &super::super::super::credentials::WebAccount) -> Result<WebAccountScope> {
        <Self as RtActivatable<IWebAccountScopeManagerStatics>>::get_activation_factory().get_scope(webAccount)
    }
}
DEFINE_CLSID!(WebAccountManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,65,99,99,111,117,110,116,77,97,110,97,103,101,114,0]) [CLSID_WebAccountManager]);
DEFINE_IID!(IID_IWebAccountManagerStatics, 3001606566, 54426, 16434, 132, 191, 26, 40, 71, 116, 123, 241);
RT_INTERFACE!{static interface IWebAccountManagerStatics(IWebAccountManagerStaticsVtbl): IInspectable [IID_IWebAccountManagerStatics] {
    fn UpdateWebAccountPropertiesAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, webAccountUserName: HSTRING, additionalProperties: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AddWebAccountAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn DeleteWebAccountAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn FindAllProviderWebAccountsAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-web"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-web")] fn PushCookiesAsync(&self, uri: <foundation::Uri as RtType>::Abi, cookies: <foundation::collections::IVectorView<crate::windows::web::http::HttpCookie> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn SetViewAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, view: <WebAccountClientView as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ClearViewAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, applicationCallbackUri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn GetViewsAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<WebAccountClientView>> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn SetWebAccountPictureAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, webAccountPicture: <crate::windows::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ClearWebAccountPictureAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebAccountManagerStatics {
    #[inline] pub fn update_web_account_properties_async(&self, webAccount: &super::super::super::credentials::WebAccount, webAccountUserName: &HStringArg, additionalProperties: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UpdateWebAccountPropertiesAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, webAccountUserName.get(), additionalProperties.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_web_account_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountAsync)(self.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_web_account_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DeleteWebAccountAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_provider_web_accounts_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllProviderWebAccountsAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn push_cookies_async(&self, uri: &foundation::Uri, cookies: &foundation::collections::IVectorView<crate::windows::web::http::HttpCookie>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PushCookiesAsync)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, cookies.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_view_async(&self, webAccount: &super::super::super::credentials::WebAccount, view: &WebAccountClientView) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SetViewAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, view.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_view_async(&self, webAccount: &super::super::super::credentials::WebAccount, applicationCallbackUri: &foundation::Uri) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ClearViewAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, applicationCallbackUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_views_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<WebAccountClientView>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetViewsAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_web_account_picture_async(&self, webAccount: &super::super::super::credentials::WebAccount, webAccountPicture: &crate::windows::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SetWebAccountPictureAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, webAccountPicture.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_web_account_picture_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ClearWebAccountPictureAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountManagerStatics2, 1755818025, 11615, 18003, 139, 176, 189, 47, 166, 189, 45, 135);
RT_INTERFACE!{static interface IWebAccountManagerStatics2(IWebAccountManagerStatics2Vtbl): IInspectable [IID_IWebAccountManagerStatics2] {
    fn PullCookiesAsync(&self, uriString: HSTRING, callerPFN: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebAccountManagerStatics2 {
    #[inline] pub fn pull_cookies_async(&self, uriString: &HStringArg, callerPFN: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PullCookiesAsync)(self.get_abi() as *const _ as *mut _, uriString.get(), callerPFN.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountManagerStatics3, 3712295846, 35407, 19106, 177, 94, 3, 245, 80, 175, 19, 89);
RT_INTERFACE!{static interface IWebAccountManagerStatics3(IWebAccountManagerStatics3Vtbl): IInspectable [IID_IWebAccountManagerStatics3] {
    #[cfg(feature="windows-system")] fn FindAllProviderWebAccountsForUserAsync(&self, user: <crate::windows::system::User as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountForUserAsync(&self, user: <crate::windows::system::User as RtType>::Abi, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountWithScopeForUserAsync(&self, user: <crate::windows::system::User as RtType>::Abi, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, scope: WebAccountScope, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn AddWebAccountWithScopeAndMapForUserAsync(&self, user: <crate::windows::system::User as RtType>::Abi, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, scope: WebAccountScope, perUserWebAccountId: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT
}}
impl IWebAccountManagerStatics3 {
    #[cfg(feature="windows-system")] #[inline] pub fn find_all_provider_web_accounts_for_user_async(&self, user: &crate::windows::system::User) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllProviderWebAccountsForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_for_user_async(&self, user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_for_user_async(&self, user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountWithScopeForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn add_web_account_with_scope_and_map_for_user_async(&self, user: &crate::windows::system::User, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountWithScopeAndMapForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, scope, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountManagerStatics4, 1508623058, 63451, 16687, 188, 63, 242, 254, 160, 68, 48, 180);
RT_INTERFACE!{static interface IWebAccountManagerStatics4(IWebAccountManagerStatics4Vtbl): IInspectable [IID_IWebAccountManagerStatics4] {
    fn InvalidateAppCacheForAllAccountsAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn InvalidateAppCacheForAccountAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebAccountManagerStatics4 {
    #[inline] pub fn invalidate_app_cache_for_all_accounts_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().InvalidateAppCacheForAllAccountsAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn invalidate_app_cache_for_account_async(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().InvalidateAppCacheForAccountAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountMapManagerStatics, 3908715631, 14875, 18596, 142, 144, 30, 89, 202, 111, 84, 219);
RT_INTERFACE!{static interface IWebAccountMapManagerStatics(IWebAccountMapManagerStaticsVtbl): IInspectable [IID_IWebAccountMapManagerStatics] {
    fn AddWebAccountWithScopeAndMapAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, scope: WebAccountScope, perUserWebAccountId: HSTRING, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn SetPerAppToPerUserAccountAsync(&self, perAppAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, perUserWebAccountId: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn GetPerUserFromPerAppAccountAsync(&self, perAppAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn ClearPerUserFromPerAppAccountAsync(&self, perAppAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebAccountMapManagerStatics {
    #[inline] pub fn add_web_account_with_scope_and_map_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountWithScopeAndMapAsync)(self.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, scope, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_per_app_to_per_user_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount, perUserWebAccountId: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SetPerAppToPerUserAccountAsync)(self.get_abi() as *const _ as *mut _, perAppAccount.get_abi() as *const _ as *mut _, perUserWebAccountId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_per_user_from_per_app_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetPerUserFromPerAppAccountAsync)(self.get_abi() as *const _ as *mut _, perAppAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_per_user_from_per_app_account_async(&self, perAppAccount: &super::super::super::credentials::WebAccount) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ClearPerUserFromPerAppAccountAsync)(self.get_abi() as *const _ as *mut _, perAppAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderAddAccountOperation, 1944837327, 17272, 19577, 147, 53, 165, 215, 171, 129, 89, 78);
RT_INTERFACE!{interface IWebAccountProviderAddAccountOperation(IWebAccountProviderAddAccountOperationVtbl): IInspectable [IID_IWebAccountProviderAddAccountOperation] {
    fn ReportCompleted(&self) -> HRESULT
}}
impl IWebAccountProviderAddAccountOperation {
    #[inline] pub fn report_completed(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportCompleted)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderAddAccountOperation: IWebAccountProviderAddAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderBaseReportOperation, 3148131515, 39227, 19799, 187, 228, 20, 33, 227, 102, 139, 76);
RT_INTERFACE!{interface IWebAccountProviderBaseReportOperation(IWebAccountProviderBaseReportOperationVtbl): IInspectable [IID_IWebAccountProviderBaseReportOperation] {
    fn ReportCompleted(&self) -> HRESULT,
    fn ReportError(&self, value: <super::core::WebProviderError as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderBaseReportOperation {
    #[inline] pub fn report_completed(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportCompleted)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn report_error(&self, value: &super::core::WebProviderError) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportError)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderDeleteAccountOperation, 180046008, 40449, 18889, 163, 85, 125, 72, 202, 247, 214, 202);
RT_INTERFACE!{interface IWebAccountProviderDeleteAccountOperation(IWebAccountProviderDeleteAccountOperationVtbl): IInspectable [IID_IWebAccountProviderDeleteAccountOperation] {
    fn get_WebAccount(&self, out: *mut <super::super::super::credentials::WebAccount as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderDeleteAccountOperation {
    #[inline] pub fn get_web_account(&self) -> Result<Option<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccount::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderDeleteAccountOperation: IWebAccountProviderDeleteAccountOperation}
RT_CLASS!{class WebAccountProviderGetTokenSilentOperation: IWebAccountProviderTokenOperation}
DEFINE_IID!(IID_IWebAccountProviderManageAccountOperation, 3978353756, 53787, 17982, 169, 183, 193, 253, 14, 218, 233, 120);
RT_INTERFACE!{interface IWebAccountProviderManageAccountOperation(IWebAccountProviderManageAccountOperationVtbl): IInspectable [IID_IWebAccountProviderManageAccountOperation] {
    fn get_WebAccount(&self, out: *mut <super::super::super::credentials::WebAccount as RtType>::Abi) -> HRESULT,
    fn ReportCompleted(&self) -> HRESULT
}}
impl IWebAccountProviderManageAccountOperation {
    #[inline] pub fn get_web_account(&self) -> Result<Option<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccount::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn report_completed(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportCompleted)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderManageAccountOperation: IWebAccountProviderManageAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderOperation, 1834820646, 4273, 16794, 164, 78, 249, 197, 22, 21, 116, 230);
RT_INTERFACE!{interface IWebAccountProviderOperation(IWebAccountProviderOperationVtbl): IInspectable [IID_IWebAccountProviderOperation] {
    fn get_Kind(&self, out: *mut WebAccountProviderOperationKind) -> HRESULT
}}
impl IWebAccountProviderOperation {
    #[inline] pub fn get_kind(&self) -> Result<WebAccountProviderOperationKind> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Kind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountProviderOperationKind: i32 {
    RequestToken = 0, GetTokenSilently = 1, AddAccount = 2, ManageAccount = 3, DeleteAccount = 4, RetrieveCookies = 5, SignOutAccount = 6,
}}
RT_CLASS!{class WebAccountProviderRequestTokenOperation: IWebAccountProviderTokenOperation}
DEFINE_IID!(IID_IWebAccountProviderRetrieveCookiesOperation, 1510212673, 4003, 19121, 160, 28, 32, 177, 16, 53, 133, 148);
RT_INTERFACE!{interface IWebAccountProviderRetrieveCookiesOperation(IWebAccountProviderRetrieveCookiesOperationVtbl): IInspectable [IID_IWebAccountProviderRetrieveCookiesOperation] {
    fn get_Context(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-web"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-web")] fn get_Cookies(&self, out: *mut <foundation::collections::IVector<crate::windows::web::http::HttpCookie> as RtType>::Abi) -> HRESULT,
    fn put_Uri(&self, uri: <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn get_Uri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderRetrieveCookiesOperation {
    #[inline] pub fn get_context(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Context)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-web")] #[inline] pub fn get_cookies(&self) -> Result<Option<foundation::collections::IVector<crate::windows::web::http::HttpCookie>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Cookies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_uri(&self, uri: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Uri)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Uri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_application_callback_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationCallbackUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderRetrieveCookiesOperation: IWebAccountProviderRetrieveCookiesOperation}
DEFINE_IID!(IID_IWebAccountProviderSignOutAccountOperation, 3096502813, 3157, 18364, 140, 114, 4, 166, 252, 124, 172, 7);
RT_INTERFACE!{interface IWebAccountProviderSignOutAccountOperation(IWebAccountProviderSignOutAccountOperationVtbl): IInspectable [IID_IWebAccountProviderSignOutAccountOperation] {
    fn get_WebAccount(&self, out: *mut <super::super::super::credentials::WebAccount as RtType>::Abi) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn get_ClientId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountProviderSignOutAccountOperation {
    #[inline] pub fn get_web_account(&self) -> Result<Option<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::super::credentials::WebAccount::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_application_callback_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationCallbackUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_client_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ClientId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderSignOutAccountOperation: IWebAccountProviderSignOutAccountOperation}
DEFINE_IID!(IID_IWebAccountProviderSilentReportOperation, 3769976312, 15119, 17626, 146, 76, 123, 24, 186, 170, 98, 169);
RT_INTERFACE!{interface IWebAccountProviderSilentReportOperation(IWebAccountProviderSilentReportOperationVtbl): IInspectable [IID_IWebAccountProviderSilentReportOperation] {
    fn ReportUserInteractionRequired(&self) -> HRESULT,
    fn ReportUserInteractionRequiredWithError(&self, value: <super::core::WebProviderError as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderSilentReportOperation {
    #[inline] pub fn report_user_interaction_required(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportUserInteractionRequired)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn report_user_interaction_required_with_error(&self, value: &super::core::WebProviderError) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportUserInteractionRequiredWithError)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderTokenObjects, 1083123787, 4904, 17115, 137, 164, 11, 206, 122, 113, 125, 142);
RT_INTERFACE!{interface IWebAccountProviderTokenObjects(IWebAccountProviderTokenObjectsVtbl): IInspectable [IID_IWebAccountProviderTokenObjects] {
    fn get_Operation(&self, out: *mut <IWebAccountProviderOperation as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderTokenObjects {
    #[inline] pub fn get_operation(&self) -> Result<Option<IWebAccountProviderOperation>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Operation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IWebAccountProviderOperation::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderTokenObjects2, 270579859, 23717, 20479, 149, 251, 184, 32, 39, 63, 195, 149);
RT_INTERFACE!{interface IWebAccountProviderTokenObjects2(IWebAccountProviderTokenObjects2Vtbl): IInspectable [IID_IWebAccountProviderTokenObjects2] {
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut <crate::windows::system::User as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderTokenObjects2 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<crate::windows::system::User>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_User)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::system::User::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderTokenOperation, 2512786366, 8244, 19512, 148, 52, 210, 108, 20, 178, 180, 178);
RT_INTERFACE!{interface IWebAccountProviderTokenOperation(IWebAccountProviderTokenOperationVtbl): IInspectable [IID_IWebAccountProviderTokenOperation] {
    fn get_ProviderRequest(&self, out: *mut <WebProviderTokenRequest as RtType>::Abi) -> HRESULT,
    fn get_ProviderResponses(&self, out: *mut <foundation::collections::IVector<WebProviderTokenResponse> as RtType>::Abi) -> HRESULT,
    fn put_CacheExpirationTime(&self, value: foundation::DateTime) -> HRESULT,
    fn get_CacheExpirationTime(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl IWebAccountProviderTokenOperation {
    #[inline] pub fn get_provider_request(&self) -> Result<Option<WebProviderTokenRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProviderRequest)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebProviderTokenRequest::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_provider_responses(&self) -> Result<Option<foundation::collections::IVector<WebProviderTokenResponse>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProviderResponses)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_cache_expiration_time(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CacheExpirationTime)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_cache_expiration_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CacheExpirationTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProviderTriggerDetails: IWebAccountProviderTokenObjects}
DEFINE_IID!(IID_IWebAccountProviderUIReportOperation, 687837907, 36736, 17147, 148, 79, 178, 16, 123, 189, 66, 230);
RT_INTERFACE!{interface IWebAccountProviderUIReportOperation(IWebAccountProviderUIReportOperationVtbl): IInspectable [IID_IWebAccountProviderUIReportOperation] {
    fn ReportUserCanceled(&self) -> HRESULT
}}
impl IWebAccountProviderUIReportOperation {
    #[inline] pub fn report_user_canceled(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ReportUserCanceled)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountScope: i32 {
    PerUser = 0, PerApplication = 1,
}}
DEFINE_IID!(IID_IWebAccountScopeManagerStatics, 1550639996, 4786, 16954, 191, 61, 133, 184, 215, 229, 54, 86);
RT_INTERFACE!{static interface IWebAccountScopeManagerStatics(IWebAccountScopeManagerStaticsVtbl): IInspectable [IID_IWebAccountScopeManagerStatics] {
    fn AddWebAccountWithScopeAsync(&self, webAccountId: HSTRING, webAccountUserName: HSTRING, props: <foundation::collections::IMapView<HString, HString> as RtType>::Abi, scope: WebAccountScope, out: *mut <foundation::IAsyncOperation<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn SetScopeAsync(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, scope: WebAccountScope, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn GetScope(&self, webAccount: <super::super::super::credentials::WebAccount as RtType>::Abi, out: *mut WebAccountScope) -> HRESULT
}}
impl IWebAccountScopeManagerStatics {
    #[inline] pub fn add_web_account_with_scope_async(&self, webAccountId: &HStringArg, webAccountUserName: &HStringArg, props: &foundation::collections::IMapView<HString, HString>, scope: WebAccountScope) -> Result<foundation::IAsyncOperation<super::super::super::credentials::WebAccount>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().AddWebAccountWithScopeAsync)(self.get_abi() as *const _ as *mut _, webAccountId.get(), webAccountUserName.get(), props.get_abi() as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_scope_async(&self, webAccount: &super::super::super::credentials::WebAccount, scope: WebAccountScope) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SetScopeAsync)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, scope, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scope(&self, webAccount: &super::super::super::credentials::WebAccount) -> Result<WebAccountScope> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetScope)(self.get_abi() as *const _ as *mut _, webAccount.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountSelectionOptions: u32 {
    Default = 0, New = 1,
}}
DEFINE_IID!(IID_IWebProviderTokenRequest, 504919947, 34821, 17739, 159, 17, 70, 141, 42, 241, 9, 90);
RT_INTERFACE!{interface IWebProviderTokenRequest(IWebProviderTokenRequestVtbl): IInspectable [IID_IWebProviderTokenRequest] {
    fn get_ClientRequest(&self, out: *mut <super::core::WebTokenRequest as RtType>::Abi) -> HRESULT,
    fn get_WebAccounts(&self, out: *mut <foundation::collections::IVectorView<super::super::super::credentials::WebAccount> as RtType>::Abi) -> HRESULT,
    fn get_WebAccountSelectionOptions(&self, out: *mut WebAccountSelectionOptions) -> HRESULT,
    fn get_ApplicationCallbackUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn GetApplicationTokenBindingKeyAsync(&self, keyType: super::TokenBindingKeyType, target: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::super::cryptography::core::CryptographicKey> as RtType>::Abi) -> HRESULT
}}
impl IWebProviderTokenRequest {
    #[inline] pub fn get_client_request(&self) -> Result<Option<super::core::WebTokenRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ClientRequest)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::core::WebTokenRequest::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_web_accounts(&self) -> Result<Option<foundation::collections::IVectorView<super::super::super::credentials::WebAccount>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccounts)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_web_account_selection_options(&self) -> Result<WebAccountSelectionOptions> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_WebAccountSelectionOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_application_callback_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationCallbackUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_application_token_binding_key_async(&self, keyType: super::TokenBindingKeyType, target: &foundation::Uri) -> Result<foundation::IAsyncOperation<super::super::super::cryptography::core::CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetApplicationTokenBindingKeyAsync)(self.get_abi() as *const _ as *mut _, keyType, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebProviderTokenRequest: IWebProviderTokenRequest}
DEFINE_IID!(IID_IWebProviderTokenRequest2, 3050778188, 4273, 19110, 136, 177, 11, 108, 158, 12, 30, 70);
RT_INTERFACE!{interface IWebProviderTokenRequest2(IWebProviderTokenRequest2Vtbl): IInspectable [IID_IWebProviderTokenRequest2] {
    #[cfg(feature="windows-storage")] fn GetApplicationTokenBindingKeyIdAsync(&self, keyType: super::TokenBindingKeyType, target: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT
}}
impl IWebProviderTokenRequest2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_application_token_binding_key_id_async(&self, keyType: super::TokenBindingKeyType, target: &foundation::Uri) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetApplicationTokenBindingKeyIdAsync)(self.get_abi() as *const _ as *mut _, keyType, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebProviderTokenRequest3, 455546538, 17033, 17518, 146, 86, 218, 251, 111, 102, 165, 30);
RT_INTERFACE!{interface IWebProviderTokenRequest3(IWebProviderTokenRequest3Vtbl): IInspectable [IID_IWebProviderTokenRequest3] {
    fn get_ApplicationPackageFamilyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ApplicationProcessName(&self, out: *mut HSTRING) -> HRESULT,
    fn CheckApplicationForCapabilityAsync(&self, capabilityName: HSTRING, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IWebProviderTokenRequest3 {
    #[inline] pub fn get_application_package_family_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationPackageFamilyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_application_process_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ApplicationProcessName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn check_application_for_capability_async(&self, capabilityName: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CheckApplicationForCapabilityAsync)(self.get_abi() as *const _ as *mut _, capabilityName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebProviderTokenResponse, 4011931539, 61269, 16774, 183, 206, 140, 178, 231, 249, 132, 158);
RT_INTERFACE!{interface IWebProviderTokenResponse(IWebProviderTokenResponseVtbl): IInspectable [IID_IWebProviderTokenResponse] {
    fn get_ClientResponse(&self, out: *mut <super::core::WebTokenResponse as RtType>::Abi) -> HRESULT
}}
impl IWebProviderTokenResponse {
    #[inline] pub fn get_client_response(&self) -> Result<Option<super::core::WebTokenResponse>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ClientResponse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::core::WebTokenResponse::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebProviderTokenResponse: IWebProviderTokenResponse}
impl RtActivatable<IWebProviderTokenResponseFactory> for WebProviderTokenResponse {}
impl WebProviderTokenResponse {
    #[inline] pub fn create(webTokenResponse: &super::core::WebTokenResponse) -> Result<WebProviderTokenResponse> {
        <Self as RtActivatable<IWebProviderTokenResponseFactory>>::get_activation_factory().create(webTokenResponse)
    }
}
DEFINE_CLSID!(WebProviderTokenResponse(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,65,117,116,104,101,110,116,105,99,97,116,105,111,110,46,87,101,98,46,80,114,111,118,105,100,101,114,46,87,101,98,80,114,111,118,105,100,101,114,84,111,107,101,110,82,101,115,112,111,110,115,101,0]) [CLSID_WebProviderTokenResponse]);
DEFINE_IID!(IID_IWebProviderTokenResponseFactory, 4199143834, 9658, 16503, 156, 250, 157, 180, 222, 167, 183, 26);
RT_INTERFACE!{static interface IWebProviderTokenResponseFactory(IWebProviderTokenResponseFactoryVtbl): IInspectable [IID_IWebProviderTokenResponseFactory] {
    fn Create(&self, webTokenResponse: <super::core::WebTokenResponse as RtType>::Abi, out: *mut <WebProviderTokenResponse as RtType>::Abi) -> HRESULT
}}
impl IWebProviderTokenResponseFactory {
    #[inline] pub fn create(&self, webTokenResponse: &super::core::WebTokenResponse) -> Result<WebProviderTokenResponse> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, webTokenResponse.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebProviderTokenResponse::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Security.Authentication.Web.Provider
} // Windows.Security.Authentication.Web
} // Windows.Security.Authentication
pub mod credentials { // Windows.Security.Credentials
use crate::prelude::*;
DEFINE_IID!(IID_ICredentialFactory, 1424954273, 48934, 18357, 151, 221, 222, 119, 155, 124, 173, 88);
RT_INTERFACE!{static interface ICredentialFactory(ICredentialFactoryVtbl): IInspectable [IID_ICredentialFactory] {
    fn CreatePasswordCredential(&self, resource: HSTRING, userName: HSTRING, password: HSTRING, out: *mut <PasswordCredential as RtType>::Abi) -> HRESULT
}}
impl ICredentialFactory {
    #[inline] pub fn create_password_credential(&self, resource: &HStringArg, userName: &HStringArg, password: &HStringArg) -> Result<PasswordCredential> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreatePasswordCredential)(self.get_abi() as *const _ as *mut _, resource.get(), userName.get(), password.get(), &mut out);
        if hr == S_OK { Ok(PasswordCredential::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyCredential, 2508582797, 17787, 18503, 177, 26, 250, 150, 11, 189, 177, 56);
RT_INTERFACE!{interface IKeyCredential(IKeyCredentialVtbl): IInspectable [IID_IKeyCredential] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrievePublicKeyWithDefaultBlobType(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn RetrievePublicKeyWithBlobType(&self, blobType: super::cryptography::core::CryptographicPublicKeyBlobType, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestSignAsync(&self, data: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<KeyCredentialOperationResult> as RtType>::Abi) -> HRESULT,
    fn GetAttestationAsync(&self, out: *mut <foundation::IAsyncOperation<KeyCredentialAttestationResult> as RtType>::Abi) -> HRESULT
}}
impl IKeyCredential {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn retrieve_public_key_with_default_blob_type(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RetrievePublicKeyWithDefaultBlobType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn retrieve_public_key_with_blob_type(&self, blobType: super::cryptography::core::CryptographicPublicKeyBlobType) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RetrievePublicKeyWithBlobType)(self.get_abi() as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_sign_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<KeyCredentialOperationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestSignAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attestation_async(&self) -> Result<foundation::IAsyncOperation<KeyCredentialAttestationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAttestationAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyCredential: IKeyCredential}
DEFINE_IID!(IID_IKeyCredentialAttestationResult, 2024453025, 41921, 16643, 182, 204, 71, 44, 68, 23, 28, 187);
RT_INTERFACE!{interface IKeyCredentialAttestationResult(IKeyCredentialAttestationResultVtbl): IInspectable [IID_IKeyCredentialAttestationResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_CertificateChainBuffer(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_AttestationBuffer(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialAttestationStatus) -> HRESULT
}}
impl IKeyCredentialAttestationResult {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_certificate_chain_buffer(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CertificateChainBuffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_attestation_buffer(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AttestationBuffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<KeyCredentialAttestationStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyCredentialAttestationResult: IKeyCredentialAttestationResult}
RT_ENUM! { enum KeyCredentialAttestationStatus: i32 {
    Success = 0, UnknownError = 1, NotSupported = 2, TemporaryFailure = 3,
}}
RT_ENUM! { enum KeyCredentialCreationOption: i32 {
    ReplaceExisting = 0, FailIfExists = 1,
}}
RT_CLASS!{static class KeyCredentialManager}
impl RtActivatable<IKeyCredentialManagerStatics> for KeyCredentialManager {}
impl KeyCredentialManager {
    #[inline] pub fn is_supported_async() -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().is_supported_async()
    }
    #[inline] pub fn renew_attestation_async() -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().renew_attestation_async()
    }
    #[inline] pub fn request_create_async(name: &HStringArg, option: KeyCredentialCreationOption) -> Result<foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().request_create_async(name, option)
    }
    #[inline] pub fn open_async(name: &HStringArg) -> Result<foundation::IAsyncOperation<KeyCredentialRetrievalResult>> {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().open_async(name)
    }
    #[inline] pub fn delete_async(name: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IKeyCredentialManagerStatics>>::get_activation_factory().delete_async(name)
    }
}
DEFINE_CLSID!(KeyCredentialManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,75,101,121,67,114,101,100,101,110,116,105,97,108,77,97,110,97,103,101,114,0]) [CLSID_KeyCredentialManager]);
DEFINE_IID!(IID_IKeyCredentialManagerStatics, 1789675147, 3825, 19680, 130, 144, 65, 6, 218, 106, 99, 181);
RT_INTERFACE!{static interface IKeyCredentialManagerStatics(IKeyCredentialManagerStaticsVtbl): IInspectable [IID_IKeyCredentialManagerStatics] {
    fn IsSupportedAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn RenewAttestationAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn RequestCreateAsync(&self, name: HSTRING, option: KeyCredentialCreationOption, out: *mut <foundation::IAsyncOperation<KeyCredentialRetrievalResult> as RtType>::Abi) -> HRESULT,
    fn OpenAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncOperation<KeyCredentialRetrievalResult> as RtType>::Abi) -> HRESULT,
    fn DeleteAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IKeyCredentialManagerStatics {
    #[inline] pub fn is_supported_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().IsSupportedAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn renew_attestation_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RenewAttestationAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_create_async(&self, name: &HStringArg, option: KeyCredentialCreationOption) -> Result<foundation::IAsyncOperation<KeyCredentialRetrievalResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestCreateAsync)(self.get_abi() as *const _ as *mut _, name.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_async(&self, name: &HStringArg) -> Result<foundation::IAsyncOperation<KeyCredentialRetrievalResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_async(&self, name: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DeleteAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyCredentialOperationResult, 4114056897, 21089, 19677, 151, 109, 204, 144, 154, 199, 22, 32);
RT_INTERFACE!{interface IKeyCredentialOperationResult(IKeyCredentialOperationResultVtbl): IInspectable [IID_IKeyCredentialOperationResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Result(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialStatus) -> HRESULT
}}
impl IKeyCredentialOperationResult {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_result(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Result)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<KeyCredentialStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyCredentialOperationResult: IKeyCredentialOperationResult}
DEFINE_IID!(IID_IKeyCredentialRetrievalResult, 1489860355, 36231, 16969, 155, 88, 246, 89, 140, 201, 100, 78);
RT_INTERFACE!{interface IKeyCredentialRetrievalResult(IKeyCredentialRetrievalResultVtbl): IInspectable [IID_IKeyCredentialRetrievalResult] {
    fn get_Credential(&self, out: *mut <KeyCredential as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut KeyCredentialStatus) -> HRESULT
}}
impl IKeyCredentialRetrievalResult {
    #[inline] pub fn get_credential(&self) -> Result<Option<KeyCredential>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Credential)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(KeyCredential::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<KeyCredentialStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyCredentialRetrievalResult: IKeyCredentialRetrievalResult}
RT_ENUM! { enum KeyCredentialStatus: i32 {
    Success = 0, UnknownError = 1, NotFound = 2, UserCanceled = 3, UserPrefersPassword = 4, CredentialAlreadyExists = 5, SecurityDeviceLocked = 6,
}}
DEFINE_IID!(IID_IPasswordCredential, 1790019977, 50976, 16807, 166, 193, 254, 173, 179, 99, 41, 160);
RT_INTERFACE!{interface IPasswordCredential(IPasswordCredentialVtbl): IInspectable [IID_IPasswordCredential] {
    fn get_Resource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Resource(&self, resource: HSTRING) -> HRESULT,
    fn get_UserName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserName(&self, userName: HSTRING) -> HRESULT,
    fn get_Password(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Password(&self, password: HSTRING) -> HRESULT,
    fn RetrievePassword(&self) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT
}}
impl IPasswordCredential {
    #[inline] pub fn get_resource(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Resource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_resource(&self, resource: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Resource)(self.get_abi() as *const _ as *mut _, resource.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_user_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UserName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_user_name(&self, userName: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_UserName)(self.get_abi() as *const _ as *mut _, userName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_password(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Password)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_password(&self, password: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Password)(self.get_abi() as *const _ as *mut _, password.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn retrieve_password(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().RetrievePassword)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IPropertySet::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PasswordCredential: IPasswordCredential}
impl RtActivatable<ICredentialFactory> for PasswordCredential {}
impl RtActivatable<IActivationFactory> for PasswordCredential {}
impl PasswordCredential {
    #[inline] pub fn create_password_credential(resource: &HStringArg, userName: &HStringArg, password: &HStringArg) -> Result<PasswordCredential> {
        <Self as RtActivatable<ICredentialFactory>>::get_activation_factory().create_password_credential(resource, userName, password)
    }
}
DEFINE_CLSID!(PasswordCredential(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,67,114,101,100,101,110,116,105,97,108,0]) [CLSID_PasswordCredential]);
RT_CLASS!{class PasswordCredentialPropertyStore: foundation::collections::IPropertySet}
impl RtActivatable<IActivationFactory> for PasswordCredentialPropertyStore {}
DEFINE_CLSID!(PasswordCredentialPropertyStore(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,67,114,101,100,101,110,116,105,97,108,80,114,111,112,101,114,116,121,83,116,111,114,101,0]) [CLSID_PasswordCredentialPropertyStore]);
DEFINE_IID!(IID_IPasswordVault, 1643981835, 51412, 18625, 165, 79, 188, 90, 100, 32, 90, 242);
RT_INTERFACE!{interface IPasswordVault(IPasswordVaultVtbl): IInspectable [IID_IPasswordVault] {
    fn Add(&self, credential: <PasswordCredential as RtType>::Abi) -> HRESULT,
    fn Remove(&self, credential: <PasswordCredential as RtType>::Abi) -> HRESULT,
    fn Retrieve(&self, resource: HSTRING, userName: HSTRING, out: *mut <PasswordCredential as RtType>::Abi) -> HRESULT,
    fn FindAllByResource(&self, resource: HSTRING, out: *mut <foundation::collections::IVectorView<PasswordCredential> as RtType>::Abi) -> HRESULT,
    fn FindAllByUserName(&self, userName: HSTRING, out: *mut <foundation::collections::IVectorView<PasswordCredential> as RtType>::Abi) -> HRESULT,
    fn RetrieveAll(&self, out: *mut <foundation::collections::IVectorView<PasswordCredential> as RtType>::Abi) -> HRESULT
}}
impl IPasswordVault {
    #[inline] pub fn add(&self, credential: &PasswordCredential) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Add)(self.get_abi() as *const _ as *mut _, credential.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove(&self, credential: &PasswordCredential) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Remove)(self.get_abi() as *const _ as *mut _, credential.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn retrieve(&self, resource: &HStringArg, userName: &HStringArg) -> Result<Option<PasswordCredential>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Retrieve)(self.get_abi() as *const _ as *mut _, resource.get(), userName.get(), &mut out);
        if hr == S_OK { Ok(PasswordCredential::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_by_resource(&self, resource: &HStringArg) -> Result<Option<foundation::collections::IVectorView<PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllByResource)(self.get_abi() as *const _ as *mut _, resource.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_by_user_name(&self, userName: &HStringArg) -> Result<Option<foundation::collections::IVectorView<PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllByUserName)(self.get_abi() as *const _ as *mut _, userName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn retrieve_all(&self) -> Result<Option<foundation::collections::IVectorView<PasswordCredential>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RetrieveAll)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PasswordVault: IPasswordVault}
impl RtActivatable<IActivationFactory> for PasswordVault {}
DEFINE_CLSID!(PasswordVault(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,80,97,115,115,119,111,114,100,86,97,117,108,116,0]) [CLSID_PasswordVault]);
DEFINE_IID!(IID_IWebAccount, 1766276786, 32817, 18878, 128, 187, 150, 203, 70, 217, 154, 186);
RT_INTERFACE!{interface IWebAccount(IWebAccountVtbl): IInspectable [IID_IWebAccount] {
    fn get_WebAccountProvider(&self, out: *mut <WebAccountProvider as RtType>::Abi) -> HRESULT,
    fn get_UserName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_State(&self, out: *mut WebAccountState) -> HRESULT
}}
impl IWebAccount {
    #[inline] pub fn get_web_account_provider(&self) -> Result<Option<WebAccountProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_WebAccountProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebAccountProvider::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UserName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<WebAccountState> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_State)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccount: IWebAccount}
impl RtActivatable<IWebAccountFactory> for WebAccount {}
impl WebAccount {
    #[inline] pub fn create_web_account(webAccountProvider: &WebAccountProvider, userName: &HStringArg, state: WebAccountState) -> Result<WebAccount> {
        <Self as RtActivatable<IWebAccountFactory>>::get_activation_factory().create_web_account(webAccountProvider, userName, state)
    }
}
DEFINE_CLSID!(WebAccount(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,87,101,98,65,99,99,111,117,110,116,0]) [CLSID_WebAccount]);
DEFINE_IID!(IID_IWebAccount2, 2069288696, 39179, 20149, 148, 167, 86, 33, 243, 168, 184, 36);
RT_INTERFACE!{interface IWebAccount2(IWebAccount2Vtbl): IInspectable [IID_IWebAccount2] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IMapView<HString, HString> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetPictureAsync(&self, desizedSize: WebAccountPictureSize, out: *mut <foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn SignOutAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn SignOutWithClientIdAsync(&self, clientId: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IWebAccount2 {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IMapView<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_picture_async(&self, desizedSize: WebAccountPictureSize) -> Result<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetPictureAsync)(self.get_abi() as *const _ as *mut _, desizedSize, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn sign_out_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignOutAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn sign_out_with_client_id_async(&self, clientId: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignOutWithClientIdAsync)(self.get_abi() as *const _ as *mut _, clientId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountFactory, 2895838009, 7657, 20114, 183, 143, 5, 129, 168, 127, 110, 92);
RT_INTERFACE!{static interface IWebAccountFactory(IWebAccountFactoryVtbl): IInspectable [IID_IWebAccountFactory] {
    fn CreateWebAccount(&self, webAccountProvider: <WebAccountProvider as RtType>::Abi, userName: HSTRING, state: WebAccountState, out: *mut <WebAccount as RtType>::Abi) -> HRESULT
}}
impl IWebAccountFactory {
    #[inline] pub fn create_web_account(&self, webAccountProvider: &WebAccountProvider, userName: &HStringArg, state: WebAccountState) -> Result<WebAccount> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWebAccount)(self.get_abi() as *const _ as *mut _, webAccountProvider.get_abi() as *const _ as *mut _, userName.get(), state, &mut out);
        if hr == S_OK { Ok(WebAccount::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountPictureSize: i32 {
    Size64x64 = 64, Size208x208 = 208, Size424x424 = 424, Size1080x1080 = 1080,
}}
DEFINE_IID!(IID_IWebAccountProvider, 702335171, 31417, 19068, 163, 54, 185, 66, 249, 219, 247, 199);
RT_INTERFACE!{interface IWebAccountProvider(IWebAccountProviderVtbl): IInspectable [IID_IWebAccountProvider] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IconUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProvider {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_icon_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IconUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WebAccountProvider: IWebAccountProvider}
impl RtActivatable<IWebAccountProviderFactory> for WebAccountProvider {}
impl WebAccountProvider {
    #[inline] pub fn create_web_account_provider(id: &HStringArg, displayName: &HStringArg, iconUri: &foundation::Uri) -> Result<WebAccountProvider> {
        <Self as RtActivatable<IWebAccountProviderFactory>>::get_activation_factory().create_web_account_provider(id, displayName, iconUri)
    }
}
DEFINE_CLSID!(WebAccountProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,87,101,98,65,99,99,111,117,110,116,80,114,111,118,105,100,101,114,0]) [CLSID_WebAccountProvider]);
DEFINE_IID!(IID_IWebAccountProvider2, 1241639685, 20034, 16852, 181, 24, 224, 8, 165, 22, 54, 20);
RT_INTERFACE!{interface IWebAccountProvider2(IWebAccountProvider2Vtbl): IInspectable [IID_IWebAccountProvider2] {
    fn get_DisplayPurpose(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Authority(&self, out: *mut HSTRING) -> HRESULT
}}
impl IWebAccountProvider2 {
    #[inline] pub fn get_display_purpose(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DisplayPurpose)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_authority(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Authority)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProvider3, 3659288971, 38669, 19785, 130, 92, 242, 112, 111, 140, 167, 254);
RT_INTERFACE!{interface IWebAccountProvider3(IWebAccountProvider3Vtbl): IInspectable [IID_IWebAccountProvider3] {
    #[cfg(feature="windows-system")] fn get_User(&self, out: *mut <super::super::system::User as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProvider3 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_user(&self) -> Result<Option<super::super::system::User>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_User)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::system::User::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProvider4, 1905252571, 59286, 16912, 183, 78, 132, 210, 152, 148, 176, 128);
RT_INTERFACE!{interface IWebAccountProvider4(IWebAccountProvider4Vtbl): IInspectable [IID_IWebAccountProvider4] {
    fn get_IsSystemProvider(&self, out: *mut bool) -> HRESULT
}}
impl IWebAccountProvider4 {
    #[inline] pub fn get_is_system_provider(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsSystemProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWebAccountProviderFactory, 494304753, 57825, 19354, 167, 116, 92, 124, 126, 59, 243, 113);
RT_INTERFACE!{static interface IWebAccountProviderFactory(IWebAccountProviderFactoryVtbl): IInspectable [IID_IWebAccountProviderFactory] {
    fn CreateWebAccountProvider(&self, id: HSTRING, displayName: HSTRING, iconUri: <foundation::Uri as RtType>::Abi, out: *mut <WebAccountProvider as RtType>::Abi) -> HRESULT
}}
impl IWebAccountProviderFactory {
    #[inline] pub fn create_web_account_provider(&self, id: &HStringArg, displayName: &HStringArg, iconUri: &foundation::Uri) -> Result<WebAccountProvider> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWebAccountProvider)(self.get_abi() as *const _ as *mut _, id.get(), displayName.get(), iconUri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WebAccountProvider::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum WebAccountState: i32 {
    None = 0, Connected = 1, Error = 2,
}}
pub mod ui { // Windows.Security.Credentials.UI
use crate::prelude::*;
RT_ENUM! { enum AuthenticationProtocol: i32 {
    Basic = 0, Digest = 1, Ntlm = 2, Kerberos = 3, Negotiate = 4, CredSsp = 5, Custom = 6,
}}
RT_CLASS!{static class CredentialPicker}
impl RtActivatable<ICredentialPickerStatics> for CredentialPicker {}
impl CredentialPicker {
    #[inline] pub fn pick_with_options_async(options: &CredentialPickerOptions) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_options_async(options)
    }
    #[inline] pub fn pick_with_message_async(targetName: &HStringArg, message: &HStringArg) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_message_async(targetName, message)
    }
    #[inline] pub fn pick_with_caption_async(targetName: &HStringArg, message: &HStringArg, caption: &HStringArg) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> {
        <Self as RtActivatable<ICredentialPickerStatics>>::get_activation_factory().pick_with_caption_async(targetName, message, caption)
    }
}
DEFINE_CLSID!(CredentialPicker(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,67,114,101,100,101,110,116,105,97,108,80,105,99,107,101,114,0]) [CLSID_CredentialPicker]);
DEFINE_IID!(IID_ICredentialPickerOptions, 2522483532, 38394, 18047, 153, 43, 11, 34, 229, 133, 155, 246);
RT_INTERFACE!{interface ICredentialPickerOptions(ICredentialPickerOptionsVtbl): IInspectable [IID_ICredentialPickerOptions] {
    fn put_Caption(&self, value: HSTRING) -> HRESULT,
    fn get_Caption(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Message(&self, value: HSTRING) -> HRESULT,
    fn get_Message(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ErrorCode(&self, value: u32) -> HRESULT,
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn put_TargetName(&self, value: HSTRING) -> HRESULT,
    fn get_TargetName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_AuthenticationProtocol(&self, value: AuthenticationProtocol) -> HRESULT,
    fn get_AuthenticationProtocol(&self, out: *mut AuthenticationProtocol) -> HRESULT,
    fn put_CustomAuthenticationProtocol(&self, value: HSTRING) -> HRESULT,
    fn get_CustomAuthenticationProtocol(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_PreviousCredential(&self, value: <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_PreviousCredential(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn put_AlwaysDisplayDialog(&self, value: bool) -> HRESULT,
    fn get_AlwaysDisplayDialog(&self, out: *mut bool) -> HRESULT,
    fn put_CallerSavesCredential(&self, value: bool) -> HRESULT,
    fn get_CallerSavesCredential(&self, out: *mut bool) -> HRESULT,
    fn put_CredentialSaveOption(&self, value: CredentialSaveOption) -> HRESULT,
    fn get_CredentialSaveOption(&self, out: *mut CredentialSaveOption) -> HRESULT
}}
impl ICredentialPickerOptions {
    #[inline] pub fn set_caption(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Caption)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_caption(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Caption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_message(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Message)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Message)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_error_code(&self, value: u32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ErrorCode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_error_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ErrorCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_target_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_TargetName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_target_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TargetName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_authentication_protocol(&self, value: AuthenticationProtocol) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_AuthenticationProtocol)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_authentication_protocol(&self) -> Result<AuthenticationProtocol> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AuthenticationProtocol)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_custom_authentication_protocol(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CustomAuthenticationProtocol)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_authentication_protocol(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CustomAuthenticationProtocol)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_previous_credential(&self, value: &crate::windows::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_PreviousCredential)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_previous_credential(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PreviousCredential)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_always_display_dialog(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_AlwaysDisplayDialog)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_always_display_dialog(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AlwaysDisplayDialog)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_caller_saves_credential(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CallerSavesCredential)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_caller_saves_credential(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CallerSavesCredential)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_credential_save_option(&self, value: CredentialSaveOption) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CredentialSaveOption)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_save_option(&self) -> Result<CredentialSaveOption> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CredentialSaveOption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class CredentialPickerOptions: ICredentialPickerOptions}
impl RtActivatable<IActivationFactory> for CredentialPickerOptions {}
DEFINE_CLSID!(CredentialPickerOptions(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,67,114,101,100,101,110,116,105,97,108,80,105,99,107,101,114,79,112,116,105,111,110,115,0]) [CLSID_CredentialPickerOptions]);
DEFINE_IID!(IID_ICredentialPickerResults, 424212890, 52272, 16652, 156, 56, 204, 8, 132, 197, 179, 215);
RT_INTERFACE!{interface ICredentialPickerResults(ICredentialPickerResultsVtbl): IInspectable [IID_ICredentialPickerResults] {
    fn get_ErrorCode(&self, out: *mut u32) -> HRESULT,
    fn get_CredentialSaveOption(&self, out: *mut CredentialSaveOption) -> HRESULT,
    fn get_CredentialSaved(&self, out: *mut bool) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Credential(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_CredentialDomainName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CredentialUserName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CredentialPassword(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICredentialPickerResults {
    #[inline] pub fn get_error_code(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ErrorCode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_save_option(&self) -> Result<CredentialSaveOption> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CredentialSaveOption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_saved(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CredentialSaved)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_credential(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Credential)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_domain_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CredentialDomainName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_user_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CredentialUserName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_credential_password(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CredentialPassword)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CredentialPickerResults: ICredentialPickerResults}
DEFINE_IID!(IID_ICredentialPickerStatics, 2855951475, 51690, 18306, 153, 251, 230, 215, 233, 56, 225, 45);
RT_INTERFACE!{static interface ICredentialPickerStatics(ICredentialPickerStaticsVtbl): IInspectable [IID_ICredentialPickerStatics] {
    fn PickWithOptionsAsync(&self, options: <CredentialPickerOptions as RtType>::Abi, out: *mut <foundation::IAsyncOperation<CredentialPickerResults> as RtType>::Abi) -> HRESULT,
    fn PickWithMessageAsync(&self, targetName: HSTRING, message: HSTRING, out: *mut <foundation::IAsyncOperation<CredentialPickerResults> as RtType>::Abi) -> HRESULT,
    fn PickWithCaptionAsync(&self, targetName: HSTRING, message: HSTRING, caption: HSTRING, out: *mut <foundation::IAsyncOperation<CredentialPickerResults> as RtType>::Abi) -> HRESULT
}}
impl ICredentialPickerStatics {
    #[inline] pub fn pick_with_options_async(&self, options: &CredentialPickerOptions) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PickWithOptionsAsync)(self.get_abi() as *const _ as *mut _, options.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_with_message_async(&self, targetName: &HStringArg, message: &HStringArg) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PickWithMessageAsync)(self.get_abi() as *const _ as *mut _, targetName.get(), message.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_with_caption_async(&self, targetName: &HStringArg, message: &HStringArg, caption: &HStringArg) -> Result<foundation::IAsyncOperation<CredentialPickerResults>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().PickWithCaptionAsync)(self.get_abi() as *const _ as *mut _, targetName.get(), message.get(), caption.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum CredentialSaveOption: i32 {
    Unselected = 0, Selected = 1, Hidden = 2,
}}
RT_ENUM! { enum UserConsentVerificationResult: i32 {
    Verified = 0, DeviceNotPresent = 1, NotConfiguredForUser = 2, DisabledByPolicy = 3, DeviceBusy = 4, RetriesExhausted = 5, Canceled = 6,
}}
RT_CLASS!{static class UserConsentVerifier}
impl RtActivatable<IUserConsentVerifierStatics> for UserConsentVerifier {}
impl UserConsentVerifier {
    #[inline] pub fn check_availability_async() -> Result<foundation::IAsyncOperation<UserConsentVerifierAvailability>> {
        <Self as RtActivatable<IUserConsentVerifierStatics>>::get_activation_factory().check_availability_async()
    }
    #[inline] pub fn request_verification_async(message: &HStringArg) -> Result<foundation::IAsyncOperation<UserConsentVerificationResult>> {
        <Self as RtActivatable<IUserConsentVerifierStatics>>::get_activation_factory().request_verification_async(message)
    }
}
DEFINE_CLSID!(UserConsentVerifier(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,101,100,101,110,116,105,97,108,115,46,85,73,46,85,115,101,114,67,111,110,115,101,110,116,86,101,114,105,102,105,101,114,0]) [CLSID_UserConsentVerifier]);
RT_ENUM! { enum UserConsentVerifierAvailability: i32 {
    Available = 0, DeviceNotPresent = 1, NotConfiguredForUser = 2, DisabledByPolicy = 3, DeviceBusy = 4,
}}
DEFINE_IID!(IID_IUserConsentVerifierStatics, 2941206417, 22092, 19932, 184, 181, 151, 52, 71, 98, 124, 101);
RT_INTERFACE!{static interface IUserConsentVerifierStatics(IUserConsentVerifierStaticsVtbl): IInspectable [IID_IUserConsentVerifierStatics] {
    fn CheckAvailabilityAsync(&self, out: *mut <foundation::IAsyncOperation<UserConsentVerifierAvailability> as RtType>::Abi) -> HRESULT,
    fn RequestVerificationAsync(&self, message: HSTRING, out: *mut <foundation::IAsyncOperation<UserConsentVerificationResult> as RtType>::Abi) -> HRESULT
}}
impl IUserConsentVerifierStatics {
    #[inline] pub fn check_availability_async(&self) -> Result<foundation::IAsyncOperation<UserConsentVerifierAvailability>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CheckAvailabilityAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_verification_async(&self, message: &HStringArg) -> Result<foundation::IAsyncOperation<UserConsentVerificationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestVerificationAsync)(self.get_abi() as *const _ as *mut _, message.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Security.Credentials.UI
} // Windows.Security.Credentials
pub mod cryptography { // Windows.Security.Cryptography
use crate::prelude::*;
RT_ENUM! { enum BinaryStringEncoding: i32 {
    Utf8 = 0, Utf16LE = 1, Utf16BE = 2,
}}
RT_CLASS!{static class CryptographicBuffer}
impl RtActivatable<ICryptographicBufferStatics> for CryptographicBuffer {}
impl CryptographicBuffer {
    #[cfg(feature="windows-storage")] #[inline] pub fn compare(object1: &super::super::storage::streams::IBuffer, object2: &super::super::storage::streams::IBuffer) -> Result<bool> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().compare(object1, object2)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_random(length: u32) -> Result<Option<super::super::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().generate_random(length)
    }
    #[inline] pub fn generate_random_number() -> Result<u32> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().generate_random_number()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_byte_array(value: &[u8]) -> Result<Option<super::super::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().create_from_byte_array(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_to_byte_array(buffer: &super::super::storage::streams::IBuffer) -> Result<ComArray<u8>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().copy_to_byte_array(buffer)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_hex_string(value: &HStringArg) -> Result<Option<super::super::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().decode_from_hex_string(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_hex_string(buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().encode_to_hex_string(buffer)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_base64_string(value: &HStringArg) -> Result<Option<super::super::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().decode_from_base64_string(value)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_base64_string(buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().encode_to_base64_string(buffer)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_string_to_binary(value: &HStringArg, encoding: BinaryStringEncoding) -> Result<Option<super::super::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().convert_string_to_binary(value, encoding)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_binary_to_string(encoding: BinaryStringEncoding, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> {
        <Self as RtActivatable<ICryptographicBufferStatics>>::get_activation_factory().convert_binary_to_string(encoding, buffer)
    }
}
DEFINE_CLSID!(CryptographicBuffer(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,114,121,112,116,111,103,114,97,112,104,105,99,66,117,102,102,101,114,0]) [CLSID_CryptographicBuffer]);
DEFINE_IID!(IID_ICryptographicBufferStatics, 839613986, 15536, 19679, 134, 99, 29, 40, 145, 0, 101, 235);
RT_INTERFACE!{static interface ICryptographicBufferStatics(ICryptographicBufferStaticsVtbl): IInspectable [IID_ICryptographicBufferStatics] {
    #[cfg(feature="windows-storage")] fn Compare(&self, object1: <super::super::storage::streams::IBuffer as RtType>::Abi, object2: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GenerateRandom(&self, length: u32, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn GenerateRandomNumber(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateFromByteArray(&self, valueSize: u32, value: *mut u8, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyToByteArray(&self, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi, valueSize: *mut u32, value: *mut *mut u8) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecodeFromHexString(&self, value: HSTRING, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncodeToHexString(&self, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecodeFromBase64String(&self, value: HSTRING, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncodeToBase64String(&self, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ConvertStringToBinary(&self, value: HSTRING, encoding: BinaryStringEncoding, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ConvertBinaryToString(&self, encoding: BinaryStringEncoding, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut HSTRING) -> HRESULT
}}
impl ICryptographicBufferStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn compare(&self, object1: &super::super::storage::streams::IBuffer, object2: &super::super::storage::streams::IBuffer) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().Compare)(self.get_abi() as *const _ as *mut _, object1.get_abi() as *const _ as *mut _, object2.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_random(&self, length: u32) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GenerateRandom)(self.get_abi() as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn generate_random_number(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GenerateRandomNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_from_byte_array(&self, value: &[u8]) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateFromByteArray)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_to_byte_array(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<ComArray<u8>> { unsafe { 
        let mut valueSize = 0; let mut value = null_mut();
        let hr = (self.get_vtbl().CopyToByteArray)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut valueSize, &mut value);
        if hr == S_OK { Ok(ComArray::from_raw(valueSize, value)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_hex_string(&self, value: &HStringArg) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecodeFromHexString)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_hex_string(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EncodeToHexString)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decode_from_base64_string(&self, value: &HStringArg) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecodeFromBase64String)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encode_to_base64_string(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EncodeToBase64String)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_string_to_binary(&self, value: &HStringArg, encoding: BinaryStringEncoding) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ConvertStringToBinary)(self.get_abi() as *const _ as *mut _, value.get(), encoding, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn convert_binary_to_string(&self, encoding: BinaryStringEncoding, buffer: &super::super::storage::streams::IBuffer) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ConvertBinaryToString)(self.get_abi() as *const _ as *mut _, encoding, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
pub mod certificates { // Windows.Security.Cryptography.Certificates
use crate::prelude::*;
DEFINE_IID!(IID_ICertificate, 859796492, 1240, 17331, 178, 120, 140, 95, 204, 155, 229, 160);
RT_INTERFACE!{interface ICertificate(ICertificateVtbl): IInspectable [IID_ICertificate] {
    fn BuildChainAsync(&self, certificates: <foundation::collections::IIterable<Certificate> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<CertificateChain> as RtType>::Abi) -> HRESULT,
    fn BuildChainWithParametersAsync(&self, certificates: <foundation::collections::IIterable<Certificate> as RtType>::Abi, parameters: <ChainBuildingParameters as RtType>::Abi, out: *mut <foundation::IAsyncOperation<CertificateChain> as RtType>::Abi) -> HRESULT,
    fn get_SerialNumber(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn GetHashValue(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn GetHashValueWithAlgorithm(&self, hashAlgorithmName: HSTRING, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetCertificateBlob(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Issuer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HasPrivateKey(&self, out: *mut bool) -> HRESULT,
    fn get_IsStronglyProtected(&self, out: *mut bool) -> HRESULT,
    fn get_ValidFrom(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ValidTo(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_EnhancedKeyUsages(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificate {
    #[inline] pub fn build_chain_async(&self, certificates: &foundation::collections::IIterable<Certificate>) -> Result<foundation::IAsyncOperation<CertificateChain>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildChainAsync)(self.get_abi() as *const _ as *mut _, certificates.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn build_chain_with_parameters_async(&self, certificates: &foundation::collections::IIterable<Certificate>, parameters: &ChainBuildingParameters) -> Result<foundation::IAsyncOperation<CertificateChain>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildChainWithParametersAsync)(self.get_abi() as *const _ as *mut _, certificates.get_abi() as *const _ as *mut _, parameters.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_serial_number(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().get_SerialNumber)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hash_value(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().GetHashValue)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hash_value_with_algorithm(&self, hashAlgorithmName: &HStringArg) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().GetHashValueWithAlgorithm)(self.get_abi() as *const _ as *mut _, hashAlgorithmName.get(), &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_certificate_blob(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetCertificateBlob)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subject(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Subject)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_issuer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Issuer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_has_private_key(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_HasPrivateKey)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_strongly_protected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsStronglyProtected)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_from(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ValidFrom)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_to(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ValidTo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_enhanced_key_usages(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EnhancedKeyUsages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_friendly_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_FriendlyName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Certificate: ICertificate}
impl RtActivatable<ICertificateFactory> for Certificate {}
impl Certificate {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_certificate(certBlob: &crate::windows::storage::streams::IBuffer) -> Result<Certificate> {
        <Self as RtActivatable<ICertificateFactory>>::get_activation_factory().create_certificate(certBlob)
    }
}
DEFINE_CLSID!(Certificate(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,0]) [CLSID_Certificate]);
DEFINE_IID!(IID_ICertificate2, 397948748, 35365, 19862, 164, 146, 143, 194, 154, 196, 253, 166);
RT_INTERFACE!{interface ICertificate2(ICertificate2Vtbl): IInspectable [IID_ICertificate2] {
    fn get_IsSecurityDeviceBound(&self, out: *mut bool) -> HRESULT,
    fn get_KeyUsages(&self, out: *mut <CertificateKeyUsages as RtType>::Abi) -> HRESULT,
    fn get_KeyAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignatureAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SignatureHashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SubjectAlternativeName(&self, out: *mut <SubjectAlternativeNameInfo as RtType>::Abi) -> HRESULT
}}
impl ICertificate2 {
    #[inline] pub fn get_is_security_device_bound(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsSecurityDeviceBound)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_key_usages(&self) -> Result<Option<CertificateKeyUsages>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyUsages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CertificateKeyUsages::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_key_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signature_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SignatureAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signature_hash_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SignatureHashAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subject_alternative_name(&self) -> Result<Option<SubjectAlternativeNameInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SubjectAlternativeName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SubjectAlternativeNameInfo::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificate3, 3193022822, 44639, 18002, 172, 231, 198, 215, 231, 114, 76, 243);
RT_INTERFACE!{interface ICertificate3(ICertificate3Vtbl): IInspectable [IID_ICertificate3] {
    fn get_IsPerUser(&self, out: *mut bool) -> HRESULT,
    fn get_StoreName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificate3 {
    #[inline] pub fn get_is_per_user(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsPerUser)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_store_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_StoreName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_key_storage_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyStorageProviderName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateChain, 549409669, 13969, 17665, 166, 44, 253, 151, 39, 139, 49, 238);
RT_INTERFACE!{interface ICertificateChain(ICertificateChainVtbl): IInspectable [IID_ICertificateChain] {
    fn Validate(&self, out: *mut ChainValidationResult) -> HRESULT,
    fn ValidateWithParameters(&self, parameter: <ChainValidationParameters as RtType>::Abi, out: *mut ChainValidationResult) -> HRESULT,
    fn GetCertificates(&self, includeRoot: bool, out: *mut <foundation::collections::IVectorView<Certificate> as RtType>::Abi) -> HRESULT
}}
impl ICertificateChain {
    #[inline] pub fn validate(&self) -> Result<ChainValidationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().Validate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn validate_with_parameters(&self, parameter: &ChainValidationParameters) -> Result<ChainValidationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().ValidateWithParameters)(self.get_abi() as *const _ as *mut _, parameter.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_certificates(&self, includeRoot: bool) -> Result<Option<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetCertificates)(self.get_abi() as *const _ as *mut _, includeRoot, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateChain: ICertificateChain}
RT_ENUM! { enum CertificateChainPolicy: i32 {
    Base = 0, Ssl = 1, NTAuthentication = 2, MicrosoftRoot = 3,
}}
RT_CLASS!{static class CertificateEnrollmentManager}
impl RtActivatable<ICertificateEnrollmentManagerStatics> for CertificateEnrollmentManager {}
impl RtActivatable<ICertificateEnrollmentManagerStatics2> for CertificateEnrollmentManager {}
impl RtActivatable<ICertificateEnrollmentManagerStatics3> for CertificateEnrollmentManager {}
impl CertificateEnrollmentManager {
    #[inline] pub fn create_request_async(request: &CertificateRequestProperties) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().create_request_async(request)
    }
    #[inline] pub fn install_certificate_async(certificate: &HStringArg, installOption: InstallOptions) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().install_certificate_async(certificate, installOption)
    }
    #[inline] pub fn import_pfx_data_async(pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics>>::get_activation_factory().import_pfx_data_async(pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName)
    }
    #[inline] pub fn get_user_certificate_enrollment_manager() -> Result<Option<UserCertificateEnrollmentManager>> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics2>>::get_activation_factory().get_user_certificate_enrollment_manager()
    }
    #[inline] pub fn import_pfx_data_to_ksp_async(pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics2>>::get_activation_factory().import_pfx_data_to_ksp_async(pfxData, password, exportable, keyProtectionLevel, installOption, friendlyName, keyStorageProvider)
    }
    #[inline] pub fn import_pfx_data_to_ksp_with_parameters_async(pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<ICertificateEnrollmentManagerStatics3>>::get_activation_factory().import_pfx_data_to_ksp_with_parameters_async(pfxData, password, pfxImportParameters)
    }
}
DEFINE_CLSID!(CertificateEnrollmentManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,69,110,114,111,108,108,109,101,110,116,77,97,110,97,103,101,114,0]) [CLSID_CertificateEnrollmentManager]);
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics, 2286350143, 43398, 18683, 159, 215, 154, 236, 6, 147, 91, 241);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics(ICertificateEnrollmentManagerStaticsVtbl): IInspectable [IID_ICertificateEnrollmentManagerStatics] {
    fn CreateRequestAsync(&self, request: <CertificateRequestProperties as RtType>::Abi, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn InstallCertificateAsync(&self, certificate: HSTRING, installOption: InstallOptions, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ImportPfxDataAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics {
    #[inline] pub fn create_request_async(&self, request: &CertificateRequestProperties) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateRequestAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn install_certificate_async(&self, certificate: &HStringArg, installOption: InstallOptions) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().InstallCertificateAsync)(self.get_abi() as *const _ as *mut _, certificate.get(), installOption, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_pfx_data_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics2, 3696958515, 25641, 16404, 153, 156, 93, 151, 53, 128, 45, 29);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics2(ICertificateEnrollmentManagerStatics2Vtbl): IInspectable [IID_ICertificateEnrollmentManagerStatics2] {
    fn get_UserCertificateEnrollmentManager(&self, out: *mut <UserCertificateEnrollmentManager as RtType>::Abi) -> HRESULT,
    fn ImportPfxDataToKspAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, keyStorageProvider: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics2 {
    #[inline] pub fn get_user_certificate_enrollment_manager(&self) -> Result<Option<UserCertificateEnrollmentManager>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_UserCertificateEnrollmentManager)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(UserCertificateEnrollmentManager::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_pfx_data_to_ksp_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataToKspAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), keyStorageProvider.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateEnrollmentManagerStatics3, 4260135614, 24956, 16986, 183, 45, 57, 139, 38, 172, 114, 100);
RT_INTERFACE!{static interface ICertificateEnrollmentManagerStatics3(ICertificateEnrollmentManagerStatics3Vtbl): IInspectable [IID_ICertificateEnrollmentManagerStatics3] {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxData: HSTRING, password: HSTRING, pfxImportParameters: <PfxImportParameters as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl ICertificateEnrollmentManagerStatics3 {
    #[inline] pub fn import_pfx_data_to_ksp_with_parameters_async(&self, pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataToKspWithParametersAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), pfxImportParameters.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateExtension, 2228160086, 43494, 17741, 142, 69, 46, 167, 196, 188, 213, 59);
RT_INTERFACE!{interface ICertificateExtension(ICertificateExtensionVtbl): IInspectable [IID_ICertificateExtension] {
    fn get_ObjectId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ObjectId(&self, value: HSTRING) -> HRESULT,
    fn get_IsCritical(&self, out: *mut bool) -> HRESULT,
    fn put_IsCritical(&self, value: bool) -> HRESULT,
    fn EncodeValue(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_Value(&self, valueSize: u32, value: *mut u8) -> HRESULT
}}
impl ICertificateExtension {
    #[inline] pub fn get_object_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ObjectId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_object_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ObjectId)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_critical(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsCritical)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_critical(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IsCritical)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn encode_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().EncodeValue)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().get_Value)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &[u8]) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Value)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateExtension: ICertificateExtension}
impl RtActivatable<IActivationFactory> for CertificateExtension {}
DEFINE_CLSID!(CertificateExtension(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,69,120,116,101,110,115,105,111,110,0]) [CLSID_CertificateExtension]);
DEFINE_IID!(IID_ICertificateFactory, 397681180, 19375, 17570, 150, 8, 4, 251, 98, 177, 105, 66);
RT_INTERFACE!{static interface ICertificateFactory(ICertificateFactoryVtbl): IInspectable [IID_ICertificateFactory] {
    #[cfg(feature="windows-storage")] fn CreateCertificate(&self, certBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <Certificate as RtType>::Abi) -> HRESULT
}}
impl ICertificateFactory {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_certificate(&self, certBlob: &crate::windows::storage::streams::IBuffer) -> Result<Certificate> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCertificate)(self.get_abi() as *const _ as *mut _, certBlob.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Certificate::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateKeyUsages, 1791369327, 57807, 18538, 180, 133, 166, 156, 131, 228, 111, 209);
RT_INTERFACE!{interface ICertificateKeyUsages(ICertificateKeyUsagesVtbl): IInspectable [IID_ICertificateKeyUsages] {
    fn get_EncipherOnly(&self, out: *mut bool) -> HRESULT,
    fn put_EncipherOnly(&self, value: bool) -> HRESULT,
    fn get_CrlSign(&self, out: *mut bool) -> HRESULT,
    fn put_CrlSign(&self, value: bool) -> HRESULT,
    fn get_KeyCertificateSign(&self, out: *mut bool) -> HRESULT,
    fn put_KeyCertificateSign(&self, value: bool) -> HRESULT,
    fn get_KeyAgreement(&self, out: *mut bool) -> HRESULT,
    fn put_KeyAgreement(&self, value: bool) -> HRESULT,
    fn get_DataEncipherment(&self, out: *mut bool) -> HRESULT,
    fn put_DataEncipherment(&self, value: bool) -> HRESULT,
    fn get_KeyEncipherment(&self, out: *mut bool) -> HRESULT,
    fn put_KeyEncipherment(&self, value: bool) -> HRESULT,
    fn get_NonRepudiation(&self, out: *mut bool) -> HRESULT,
    fn put_NonRepudiation(&self, value: bool) -> HRESULT,
    fn get_DigitalSignature(&self, out: *mut bool) -> HRESULT,
    fn put_DigitalSignature(&self, value: bool) -> HRESULT
}}
impl ICertificateKeyUsages {
    #[inline] pub fn get_encipher_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_EncipherOnly)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_encipher_only(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_EncipherOnly)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_crl_sign(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CrlSign)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_crl_sign(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CrlSign)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_certificate_sign(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyCertificateSign)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_certificate_sign(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyCertificateSign)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_agreement(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyAgreement)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_agreement(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyAgreement)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_data_encipherment(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DataEncipherment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_data_encipherment(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_DataEncipherment)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_encipherment(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyEncipherment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_encipherment(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyEncipherment)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_non_repudiation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NonRepudiation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_non_repudiation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NonRepudiation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_digital_signature(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DigitalSignature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_digital_signature(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_DigitalSignature)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateKeyUsages: ICertificateKeyUsages}
impl RtActivatable<IActivationFactory> for CertificateKeyUsages {}
DEFINE_CLSID!(CertificateKeyUsages(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,75,101,121,85,115,97,103,101,115,0]) [CLSID_CertificateKeyUsages]);
DEFINE_IID!(IID_ICertificateQuery, 1527261745, 42792, 18710, 181, 238, 255, 203, 138, 207, 36, 23);
RT_INTERFACE!{interface ICertificateQuery(ICertificateQueryVtbl): IInspectable [IID_ICertificateQuery] {
    fn get_EnhancedKeyUsages(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_IssuerName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IssuerName(&self, value: HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_Thumbprint(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_Thumbprint(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn get_HardwareOnly(&self, out: *mut bool) -> HRESULT,
    fn put_HardwareOnly(&self, value: bool) -> HRESULT
}}
impl ICertificateQuery {
    #[inline] pub fn get_enhanced_key_usages(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EnhancedKeyUsages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_issuer_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IssuerName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_issuer_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IssuerName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_friendly_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_FriendlyName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbprint(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().get_Thumbprint)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbprint(&self, value: &[u8]) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Thumbprint)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hardware_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_HardwareOnly)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hardware_only(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_HardwareOnly)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateQuery: ICertificateQuery}
impl RtActivatable<IActivationFactory> for CertificateQuery {}
DEFINE_CLSID!(CertificateQuery(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,81,117,101,114,121,0]) [CLSID_CertificateQuery]);
DEFINE_IID!(IID_ICertificateQuery2, 2472151799, 3033, 20341, 184, 194, 226, 122, 127, 116, 238, 205);
RT_INTERFACE!{interface ICertificateQuery2(ICertificateQuery2Vtbl): IInspectable [IID_ICertificateQuery2] {
    fn get_IncludeDuplicates(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeDuplicates(&self, value: bool) -> HRESULT,
    fn get_IncludeExpiredCertificates(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeExpiredCertificates(&self, value: bool) -> HRESULT,
    fn get_StoreName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_StoreName(&self, value: HSTRING) -> HRESULT
}}
impl ICertificateQuery2 {
    #[inline] pub fn get_include_duplicates(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeDuplicates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_include_duplicates(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IncludeDuplicates)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_include_expired_certificates(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeExpiredCertificates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_include_expired_certificates(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IncludeExpiredCertificates)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_store_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_StoreName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_store_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_StoreName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateRequestProperties, 1216251126, 38114, 19918, 136, 51, 26, 112, 10, 55, 162, 154);
RT_INTERFACE!{interface ICertificateRequestProperties(ICertificateRequestPropertiesVtbl): IInspectable [IID_ICertificateRequestProperties] {
    fn get_Subject(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subject(&self, value: HSTRING) -> HRESULT,
    fn get_KeyAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_KeySize(&self, out: *mut u32) -> HRESULT,
    fn put_KeySize(&self, value: u32) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_HashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_HashAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_Exportable(&self, out: *mut ExportOption) -> HRESULT,
    fn put_Exportable(&self, value: ExportOption) -> HRESULT,
    fn get_KeyUsages(&self, out: *mut EnrollKeyUsages) -> HRESULT,
    fn put_KeyUsages(&self, value: EnrollKeyUsages) -> HRESULT,
    fn get_KeyProtectionLevel(&self, out: *mut KeyProtectionLevel) -> HRESULT,
    fn put_KeyProtectionLevel(&self, value: KeyProtectionLevel) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyStorageProviderName(&self, value: HSTRING) -> HRESULT
}}
impl ICertificateRequestProperties {
    #[inline] pub fn get_subject(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Subject)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_subject(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Subject)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_key_algorithm_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyAlgorithmName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeySize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_size(&self, value: u32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeySize)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_friendly_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_FriendlyName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hash_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HashAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_hash_algorithm_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_HashAlgorithmName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_exportable(&self) -> Result<ExportOption> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Exportable)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_exportable(&self, value: ExportOption) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Exportable)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_usages(&self) -> Result<EnrollKeyUsages> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyUsages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_usages(&self, value: EnrollKeyUsages) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyUsages)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_protection_level(&self) -> Result<KeyProtectionLevel> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyProtectionLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_protection_level(&self, value: KeyProtectionLevel) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyProtectionLevel)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_storage_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyStorageProviderName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_key_storage_provider_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyStorageProviderName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateRequestProperties: ICertificateRequestProperties}
impl RtActivatable<IActivationFactory> for CertificateRequestProperties {}
DEFINE_CLSID!(CertificateRequestProperties(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,82,101,113,117,101,115,116,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_CertificateRequestProperties]);
DEFINE_IID!(IID_ICertificateRequestProperties2, 1033947476, 55103, 20467, 160, 166, 6, 119, 192, 173, 160, 91);
RT_INTERFACE!{interface ICertificateRequestProperties2(ICertificateRequestProperties2Vtbl): IInspectable [IID_ICertificateRequestProperties2] {
    fn get_SmartcardReaderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SmartcardReaderName(&self, value: HSTRING) -> HRESULT,
    fn get_SigningCertificate(&self, out: *mut <Certificate as RtType>::Abi) -> HRESULT,
    fn put_SigningCertificate(&self, value: <Certificate as RtType>::Abi) -> HRESULT,
    fn get_AttestationCredentialCertificate(&self, out: *mut <Certificate as RtType>::Abi) -> HRESULT,
    fn put_AttestationCredentialCertificate(&self, value: <Certificate as RtType>::Abi) -> HRESULT
}}
impl ICertificateRequestProperties2 {
    #[inline] pub fn get_smartcard_reader_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SmartcardReaderName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_smartcard_reader_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_SmartcardReaderName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_signing_certificate(&self) -> Result<Option<Certificate>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SigningCertificate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Certificate::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_signing_certificate(&self, value: &Certificate) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_SigningCertificate)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attestation_credential_certificate(&self) -> Result<Option<Certificate>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AttestationCredentialCertificate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Certificate::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attestation_credential_certificate(&self, value: &Certificate) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_AttestationCredentialCertificate)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateRequestProperties3, 3867670038, 29517, 18097, 157, 76, 110, 223, 219, 252, 132, 91);
RT_INTERFACE!{interface ICertificateRequestProperties3(ICertificateRequestProperties3Vtbl): IInspectable [IID_ICertificateRequestProperties3] {
    fn get_CurveName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CurveName(&self, value: HSTRING) -> HRESULT,
    fn get_CurveParameters(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn put_CurveParameters(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn get_ContainerNamePrefix(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerNamePrefix(&self, value: HSTRING) -> HRESULT,
    fn get_ContainerName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerName(&self, value: HSTRING) -> HRESULT,
    fn get_UseExistingKey(&self, out: *mut bool) -> HRESULT,
    fn put_UseExistingKey(&self, value: bool) -> HRESULT
}}
impl ICertificateRequestProperties3 {
    #[inline] pub fn get_curve_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CurveName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_curve_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CurveName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_curve_parameters(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().get_CurveParameters)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn set_curve_parameters(&self, value: &[u8]) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CurveParameters)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_container_name_prefix(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ContainerNamePrefix)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_container_name_prefix(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ContainerNamePrefix)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_container_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ContainerName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_container_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ContainerName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_use_existing_key(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_UseExistingKey)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_use_existing_key(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_UseExistingKey)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateRequestProperties4, 1312987858, 7265, 20458, 184, 254, 19, 95, 177, 156, 220, 228);
RT_INTERFACE!{interface ICertificateRequestProperties4(ICertificateRequestProperties4Vtbl): IInspectable [IID_ICertificateRequestProperties4] {
    fn get_SuppressedDefaults(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_SubjectAlternativeName(&self, out: *mut <SubjectAlternativeNameInfo as RtType>::Abi) -> HRESULT,
    fn get_Extensions(&self, out: *mut <foundation::collections::IVector<CertificateExtension> as RtType>::Abi) -> HRESULT
}}
impl ICertificateRequestProperties4 {
    #[inline] pub fn get_suppressed_defaults(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SuppressedDefaults)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subject_alternative_name(&self) -> Result<Option<SubjectAlternativeNameInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SubjectAlternativeName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SubjectAlternativeNameInfo::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extensions(&self) -> Result<Option<foundation::collections::IVector<CertificateExtension>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Extensions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateStore, 2965370656, 13390, 17201, 175, 20, 167, 247, 167, 235, 201, 58);
RT_INTERFACE!{interface ICertificateStore(ICertificateStoreVtbl): IInspectable [IID_ICertificateStore] {
    fn Add(&self, certificate: <Certificate as RtType>::Abi) -> HRESULT,
    fn Delete(&self, certificate: <Certificate as RtType>::Abi) -> HRESULT
}}
impl ICertificateStore {
    #[inline] pub fn add(&self, certificate: &Certificate) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Add)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn delete(&self, certificate: &Certificate) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Delete)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class CertificateStore: ICertificateStore}
DEFINE_IID!(IID_ICertificateStore2, 3353775690, 16765, 19738, 186, 189, 21, 104, 126, 84, 153, 116);
RT_INTERFACE!{interface ICertificateStore2(ICertificateStore2Vtbl): IInspectable [IID_ICertificateStore2] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT
}}
impl ICertificateStore2 {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class CertificateStores}
impl RtActivatable<ICertificateStoresStatics> for CertificateStores {}
impl RtActivatable<ICertificateStoresStatics2> for CertificateStores {}
impl CertificateStores {
    #[inline] pub fn find_all_async() -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>>> {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().find_all_async()
    }
    #[inline] pub fn find_all_with_query_async(query: &CertificateQuery) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>>> {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().find_all_with_query_async(query)
    }
    #[inline] pub fn get_trusted_root_certification_authorities() -> Result<Option<CertificateStore>> {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_trusted_root_certification_authorities()
    }
    #[inline] pub fn get_intermediate_certification_authorities() -> Result<Option<CertificateStore>> {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_intermediate_certification_authorities()
    }
    #[inline] pub fn get_store_by_name(storeName: &HStringArg) -> Result<Option<CertificateStore>> {
        <Self as RtActivatable<ICertificateStoresStatics>>::get_activation_factory().get_store_by_name(storeName)
    }
    #[inline] pub fn get_user_store_by_name(storeName: &HStringArg) -> Result<Option<UserCertificateStore>> {
        <Self as RtActivatable<ICertificateStoresStatics2>>::get_activation_factory().get_user_store_by_name(storeName)
    }
}
DEFINE_CLSID!(CertificateStores(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,101,114,116,105,102,105,99,97,116,101,83,116,111,114,101,115,0]) [CLSID_CertificateStores]);
DEFINE_IID!(IID_ICertificateStoresStatics, 4226598713, 50942, 19943, 153, 207, 116, 195, 229, 150, 224, 50);
RT_INTERFACE!{static interface ICertificateStoresStatics(ICertificateStoresStaticsVtbl): IInspectable [IID_ICertificateStoresStatics] {
    fn FindAllAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>> as RtType>::Abi) -> HRESULT,
    fn FindAllWithQueryAsync(&self, query: <CertificateQuery as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>> as RtType>::Abi) -> HRESULT,
    fn get_TrustedRootCertificationAuthorities(&self, out: *mut <CertificateStore as RtType>::Abi) -> HRESULT,
    fn get_IntermediateCertificationAuthorities(&self, out: *mut <CertificateStore as RtType>::Abi) -> HRESULT,
    fn GetStoreByName(&self, storeName: HSTRING, out: *mut <CertificateStore as RtType>::Abi) -> HRESULT
}}
impl ICertificateStoresStatics {
    #[inline] pub fn find_all_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_all_with_query_async(&self, query: &CertificateQuery) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FindAllWithQueryAsync)(self.get_abi() as *const _ as *mut _, query.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_trusted_root_certification_authorities(&self) -> Result<Option<CertificateStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TrustedRootCertificationAuthorities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CertificateStore::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_intermediate_certification_authorities(&self) -> Result<Option<CertificateStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IntermediateCertificationAuthorities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CertificateStore::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_store_by_name(&self, storeName: &HStringArg) -> Result<Option<CertificateStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetStoreByName)(self.get_abi() as *const _ as *mut _, storeName.get(), &mut out);
        if hr == S_OK { Ok(CertificateStore::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICertificateStoresStatics2, 4203744121, 41172, 19340, 188, 85, 192, 163, 126, 177, 65, 237);
RT_INTERFACE!{static interface ICertificateStoresStatics2(ICertificateStoresStatics2Vtbl): IInspectable [IID_ICertificateStoresStatics2] {
    fn GetUserStoreByName(&self, storeName: HSTRING, out: *mut <UserCertificateStore as RtType>::Abi) -> HRESULT
}}
impl ICertificateStoresStatics2 {
    #[inline] pub fn get_user_store_by_name(&self, storeName: &HStringArg) -> Result<Option<UserCertificateStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetUserStoreByName)(self.get_abi() as *const _ as *mut _, storeName.get(), &mut out);
        if hr == S_OK { Ok(UserCertificateStore::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IChainBuildingParameters, 1110157602, 31885, 18359, 181, 155, 177, 39, 3, 115, 58, 195);
RT_INTERFACE!{interface IChainBuildingParameters(IChainBuildingParametersVtbl): IInspectable [IID_IChainBuildingParameters] {
    fn get_EnhancedKeyUsages(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_ValidationTimestamp(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn put_ValidationTimestamp(&self, value: foundation::DateTime) -> HRESULT,
    fn get_RevocationCheckEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_RevocationCheckEnabled(&self, value: bool) -> HRESULT,
    fn get_NetworkRetrievalEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_NetworkRetrievalEnabled(&self, value: bool) -> HRESULT,
    fn get_AuthorityInformationAccessEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_AuthorityInformationAccessEnabled(&self, value: bool) -> HRESULT,
    fn get_CurrentTimeValidationEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_CurrentTimeValidationEnabled(&self, value: bool) -> HRESULT,
    fn get_ExclusiveTrustRoots(&self, out: *mut <foundation::collections::IVector<Certificate> as RtType>::Abi) -> HRESULT
}}
impl IChainBuildingParameters {
    #[inline] pub fn get_enhanced_key_usages(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EnhancedKeyUsages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_validation_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ValidationTimestamp)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_validation_timestamp(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ValidationTimestamp)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_revocation_check_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RevocationCheckEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_revocation_check_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RevocationCheckEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_network_retrieval_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_NetworkRetrievalEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_network_retrieval_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_NetworkRetrievalEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_authority_information_access_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AuthorityInformationAccessEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_authority_information_access_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_AuthorityInformationAccessEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_current_time_validation_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CurrentTimeValidationEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_current_time_validation_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CurrentTimeValidationEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_exclusive_trust_roots(&self) -> Result<Option<foundation::collections::IVector<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ExclusiveTrustRoots)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ChainBuildingParameters: IChainBuildingParameters}
impl RtActivatable<IActivationFactory> for ChainBuildingParameters {}
DEFINE_CLSID!(ChainBuildingParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,104,97,105,110,66,117,105,108,100,105,110,103,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_ChainBuildingParameters]);
DEFINE_IID!(IID_IChainValidationParameters, 3295951690, 32432, 19286, 160, 64, 185, 200, 230, 85, 221, 243);
RT_INTERFACE!{interface IChainValidationParameters(IChainValidationParametersVtbl): IInspectable [IID_IChainValidationParameters] {
    fn get_CertificateChainPolicy(&self, out: *mut CertificateChainPolicy) -> HRESULT,
    fn put_CertificateChainPolicy(&self, value: CertificateChainPolicy) -> HRESULT,
    #[cfg(feature="windows-networking")] fn get_ServerDnsName(&self, out: *mut <crate::windows::networking::HostName as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-networking")] fn put_ServerDnsName(&self, value: <crate::windows::networking::HostName as RtType>::Abi) -> HRESULT
}}
impl IChainValidationParameters {
    #[inline] pub fn get_certificate_chain_policy(&self) -> Result<CertificateChainPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_CertificateChainPolicy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_certificate_chain_policy(&self, value: CertificateChainPolicy) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_CertificateChainPolicy)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_server_dns_name(&self) -> Result<Option<crate::windows::networking::HostName>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ServerDnsName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::networking::HostName::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn set_server_dns_name(&self, value: &crate::windows::networking::HostName) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ServerDnsName)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ChainValidationParameters: IChainValidationParameters}
impl RtActivatable<IActivationFactory> for ChainValidationParameters {}
DEFINE_CLSID!(ChainValidationParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,104,97,105,110,86,97,108,105,100,97,116,105,111,110,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_ChainValidationParameters]);
RT_ENUM! { enum ChainValidationResult: i32 {
    Success = 0, Untrusted = 1, Revoked = 2, Expired = 3, IncompleteChain = 4, InvalidSignature = 5, WrongUsage = 6, InvalidName = 7, InvalidCertificateAuthorityPolicy = 8, BasicConstraintsError = 9, UnknownCriticalExtension = 10, RevocationInformationMissing = 11, RevocationFailure = 12, OtherErrors = 13,
}}
DEFINE_IID!(IID_ICmsAttachedSignature, 1636408733, 14167, 20171, 189, 220, 12, 163, 87, 215, 169, 54);
RT_INTERFACE!{interface ICmsAttachedSignature(ICmsAttachedSignatureVtbl): IInspectable [IID_ICmsAttachedSignature] {
    fn get_Certificates(&self, out: *mut <foundation::collections::IVectorView<Certificate> as RtType>::Abi) -> HRESULT,
    fn get_Content(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT,
    fn get_Signers(&self, out: *mut <foundation::collections::IVectorView<CmsSignerInfo> as RtType>::Abi) -> HRESULT,
    fn VerifySignature(&self, out: *mut SignatureValidationResult) -> HRESULT
}}
impl ICmsAttachedSignature {
    #[inline] pub fn get_certificates(&self) -> Result<Option<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Certificates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().get_Content)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signers(&self) -> Result<Option<foundation::collections::IVectorView<CmsSignerInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Signers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn verify_signature(&self) -> Result<SignatureValidationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().VerifySignature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class CmsAttachedSignature: ICmsAttachedSignature}
impl RtActivatable<ICmsAttachedSignatureFactory> for CmsAttachedSignature {}
impl RtActivatable<ICmsAttachedSignatureStatics> for CmsAttachedSignature {}
impl CmsAttachedSignature {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_attached_signature(inputBlob: &crate::windows::storage::streams::IBuffer) -> Result<CmsAttachedSignature> {
        <Self as RtActivatable<ICmsAttachedSignatureFactory>>::get_activation_factory().create_cms_attached_signature(inputBlob)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(data: &crate::windows::storage::streams::IBuffer, signers: &foundation::collections::IIterable<CmsSignerInfo>, certificates: &foundation::collections::IIterable<Certificate>) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICmsAttachedSignatureStatics>>::get_activation_factory().generate_signature_async(data, signers, certificates)
    }
}
DEFINE_CLSID!(CmsAttachedSignature(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,65,116,116,97,99,104,101,100,83,105,103,110,97,116,117,114,101,0]) [CLSID_CmsAttachedSignature]);
DEFINE_IID!(IID_ICmsAttachedSignatureFactory, 3502832661, 63319, 19556, 163, 98, 82, 204, 28, 119, 207, 251);
RT_INTERFACE!{static interface ICmsAttachedSignatureFactory(ICmsAttachedSignatureFactoryVtbl): IInspectable [IID_ICmsAttachedSignatureFactory] {
    #[cfg(feature="windows-storage")] fn CreateCmsAttachedSignature(&self, inputBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CmsAttachedSignature as RtType>::Abi) -> HRESULT
}}
impl ICmsAttachedSignatureFactory {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_attached_signature(&self, inputBlob: &crate::windows::storage::streams::IBuffer) -> Result<CmsAttachedSignature> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCmsAttachedSignature)(self.get_abi() as *const _ as *mut _, inputBlob.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CmsAttachedSignature::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICmsAttachedSignatureStatics, 2274925710, 45229, 18829, 167, 245, 120, 181, 155, 206, 75, 54);
RT_INTERFACE!{static interface ICmsAttachedSignatureStatics(ICmsAttachedSignatureStaticsVtbl): IInspectable [IID_ICmsAttachedSignatureStatics] {
    #[cfg(feature="windows-storage")] fn GenerateSignatureAsync(&self, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, signers: <foundation::collections::IIterable<CmsSignerInfo> as RtType>::Abi, certificates: <foundation::collections::IIterable<Certificate> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT
}}
impl ICmsAttachedSignatureStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(&self, data: &crate::windows::storage::streams::IBuffer, signers: &foundation::collections::IIterable<CmsSignerInfo>, certificates: &foundation::collections::IIterable<Certificate>) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GenerateSignatureAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, signers.get_abi() as *const _ as *mut _, certificates.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICmsDetachedSignature, 253686100, 63070, 17718, 131, 57, 89, 68, 8, 29, 178, 202);
RT_INTERFACE!{interface ICmsDetachedSignature(ICmsDetachedSignatureVtbl): IInspectable [IID_ICmsDetachedSignature] {
    fn get_Certificates(&self, out: *mut <foundation::collections::IVectorView<Certificate> as RtType>::Abi) -> HRESULT,
    fn get_Signers(&self, out: *mut <foundation::collections::IVectorView<CmsSignerInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignatureAsync(&self, data: <crate::windows::storage::streams::IInputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SignatureValidationResult> as RtType>::Abi) -> HRESULT
}}
impl ICmsDetachedSignature {
    #[inline] pub fn get_certificates(&self) -> Result<Option<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Certificates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_signers(&self) -> Result<Option<foundation::collections::IVectorView<CmsSignerInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Signers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature_async(&self, data: &crate::windows::storage::streams::IInputStream) -> Result<foundation::IAsyncOperation<SignatureValidationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().VerifySignatureAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CmsDetachedSignature: ICmsDetachedSignature}
impl RtActivatable<ICmsDetachedSignatureFactory> for CmsDetachedSignature {}
impl RtActivatable<ICmsDetachedSignatureStatics> for CmsDetachedSignature {}
impl CmsDetachedSignature {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_detached_signature(inputBlob: &crate::windows::storage::streams::IBuffer) -> Result<CmsDetachedSignature> {
        <Self as RtActivatable<ICmsDetachedSignatureFactory>>::get_activation_factory().create_cms_detached_signature(inputBlob)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(data: &crate::windows::storage::streams::IInputStream, signers: &foundation::collections::IIterable<CmsSignerInfo>, certificates: &foundation::collections::IIterable<Certificate>) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICmsDetachedSignatureStatics>>::get_activation_factory().generate_signature_async(data, signers, certificates)
    }
}
DEFINE_CLSID!(CmsDetachedSignature(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,68,101,116,97,99,104,101,100,83,105,103,110,97,116,117,114,101,0]) [CLSID_CmsDetachedSignature]);
DEFINE_IID!(IID_ICmsDetachedSignatureFactory, 3299554563, 44671, 17287, 173, 25, 0, 241, 80, 228, 142, 187);
RT_INTERFACE!{static interface ICmsDetachedSignatureFactory(ICmsDetachedSignatureFactoryVtbl): IInspectable [IID_ICmsDetachedSignatureFactory] {
    #[cfg(feature="windows-storage")] fn CreateCmsDetachedSignature(&self, inputBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CmsDetachedSignature as RtType>::Abi) -> HRESULT
}}
impl ICmsDetachedSignatureFactory {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_cms_detached_signature(&self, inputBlob: &crate::windows::storage::streams::IBuffer) -> Result<CmsDetachedSignature> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCmsDetachedSignature)(self.get_abi() as *const _ as *mut _, inputBlob.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CmsDetachedSignature::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICmsDetachedSignatureStatics, 1024543997, 49051, 18050, 155, 230, 145, 245, 124, 5, 56, 8);
RT_INTERFACE!{static interface ICmsDetachedSignatureStatics(ICmsDetachedSignatureStaticsVtbl): IInspectable [IID_ICmsDetachedSignatureStatics] {
    #[cfg(feature="windows-storage")] fn GenerateSignatureAsync(&self, data: <crate::windows::storage::streams::IInputStream as RtType>::Abi, signers: <foundation::collections::IIterable<CmsSignerInfo> as RtType>::Abi, certificates: <foundation::collections::IIterable<Certificate> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT
}}
impl ICmsDetachedSignatureStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn generate_signature_async(&self, data: &crate::windows::storage::streams::IInputStream, signers: &foundation::collections::IIterable<CmsSignerInfo>, certificates: &foundation::collections::IIterable<Certificate>) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GenerateSignatureAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, signers.get_abi() as *const _ as *mut _, certificates.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICmsSignerInfo, 1355817179, 7471, 19482, 181, 197, 208, 24, 143, 249, 31, 71);
RT_INTERFACE!{interface ICmsSignerInfo(ICmsSignerInfoVtbl): IInspectable [IID_ICmsSignerInfo] {
    fn get_Certificate(&self, out: *mut <Certificate as RtType>::Abi) -> HRESULT,
    fn put_Certificate(&self, value: <Certificate as RtType>::Abi) -> HRESULT,
    fn get_HashAlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_HashAlgorithmName(&self, value: HSTRING) -> HRESULT,
    fn get_TimestampInfo(&self, out: *mut <CmsTimestampInfo as RtType>::Abi) -> HRESULT
}}
impl ICmsSignerInfo {
    #[inline] pub fn get_certificate(&self) -> Result<Option<Certificate>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Certificate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Certificate::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_certificate(&self, value: &Certificate) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Certificate)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hash_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HashAlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_hash_algorithm_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_HashAlgorithmName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_timestamp_info(&self) -> Result<Option<CmsTimestampInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TimestampInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CmsTimestampInfo::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CmsSignerInfo: ICmsSignerInfo}
impl RtActivatable<IActivationFactory> for CmsSignerInfo {}
DEFINE_CLSID!(CmsSignerInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,67,109,115,83,105,103,110,101,114,73,110,102,111,0]) [CLSID_CmsSignerInfo]);
DEFINE_IID!(IID_ICmsTimestampInfo, 794755314, 11288, 20360, 132, 53, 197, 52, 8, 96, 118, 245);
RT_INTERFACE!{interface ICmsTimestampInfo(ICmsTimestampInfoVtbl): IInspectable [IID_ICmsTimestampInfo] {
    fn get_SigningCertificate(&self, out: *mut <Certificate as RtType>::Abi) -> HRESULT,
    fn get_Certificates(&self, out: *mut <foundation::collections::IVectorView<Certificate> as RtType>::Abi) -> HRESULT,
    fn get_Timestamp(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl ICmsTimestampInfo {
    #[inline] pub fn get_signing_certificate(&self) -> Result<Option<Certificate>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SigningCertificate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Certificate::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_certificates(&self) -> Result<Option<foundation::collections::IVectorView<Certificate>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Certificates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_timestamp(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Timestamp)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class CmsTimestampInfo: ICmsTimestampInfo}
RT_ENUM! { enum EnrollKeyUsages: u32 {
    None = 0, Decryption = 1, Signing = 2, KeyAgreement = 4, All = 16777215,
}}
RT_ENUM! { enum ExportOption: i32 {
    NotExportable = 0, Exportable = 1,
}}
RT_ENUM! { enum InstallOptions: u32 {
    None = 0, DeleteExpired = 1,
}}
RT_CLASS!{static class KeyAlgorithmNames}
impl RtActivatable<IKeyAlgorithmNamesStatics> for KeyAlgorithmNames {}
impl RtActivatable<IKeyAlgorithmNamesStatics2> for KeyAlgorithmNames {}
impl KeyAlgorithmNames {
    #[inline] pub fn get_rsa() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_rsa()
    }
    #[inline] pub fn get_dsa() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_dsa()
    }
    #[inline] pub fn get_ecdh_256() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh_256()
    }
    #[inline] pub fn get_ecdh_384() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh_384()
    }
    #[inline] pub fn get_ecdh_521() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdh_521()
    }
    #[inline] pub fn get_ecdsa_256() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_256()
    }
    #[inline] pub fn get_ecdsa_384() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_384()
    }
    #[inline] pub fn get_ecdsa_521() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_521()
    }
    #[inline] pub fn get_ecdsa() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa()
    }
    #[inline] pub fn get_ecdh() -> Result<HString> {
        <Self as RtActivatable<IKeyAlgorithmNamesStatics2>>::get_activation_factory().get_ecdh()
    }
}
DEFINE_CLSID!(KeyAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_KeyAlgorithmNames]);
DEFINE_IID!(IID_IKeyAlgorithmNamesStatics, 1200645591, 31431, 17793, 140, 59, 208, 112, 39, 20, 4, 72);
RT_INTERFACE!{static interface IKeyAlgorithmNamesStatics(IKeyAlgorithmNamesStaticsVtbl): IInspectable [IID_IKeyAlgorithmNamesStatics] {
    fn get_Rsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Dsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh521(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdsa521(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAlgorithmNamesStatics {
    #[inline] pub fn get_rsa(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rsa)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dsa(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Dsa)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdh_256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdh256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdh_384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdh384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdh_521(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdh521)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdsa256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdsa384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_521(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdsa521)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyAlgorithmNamesStatics2, 3382400646, 57853, 19018, 137, 61, 162, 111, 51, 221, 139, 180);
RT_INTERFACE!{static interface IKeyAlgorithmNamesStatics2(IKeyAlgorithmNamesStatics2Vtbl): IInspectable [IID_IKeyAlgorithmNamesStatics2] {
    fn get_Ecdsa(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ecdh(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAlgorithmNamesStatics2 {
    #[inline] pub fn get_ecdsa(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdsa)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdh(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ecdh)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class KeyAttestationHelper}
impl RtActivatable<IKeyAttestationHelperStatics> for KeyAttestationHelper {}
impl RtActivatable<IKeyAttestationHelperStatics2> for KeyAttestationHelper {}
impl KeyAttestationHelper {
    #[inline] pub fn decrypt_tpm_attestation_credential_async(credential: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IKeyAttestationHelperStatics>>::get_activation_factory().decrypt_tpm_attestation_credential_async(credential)
    }
    #[inline] pub fn get_tpm_attestation_credential_id(credential: &HStringArg) -> Result<HString> {
        <Self as RtActivatable<IKeyAttestationHelperStatics>>::get_activation_factory().get_tpm_attestation_credential_id(credential)
    }
    #[inline] pub fn decrypt_tpm_attestation_credential_with_container_name_async(credential: &HStringArg, containerName: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IKeyAttestationHelperStatics2>>::get_activation_factory().decrypt_tpm_attestation_credential_with_container_name_async(credential, containerName)
    }
}
DEFINE_CLSID!(KeyAttestationHelper(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,65,116,116,101,115,116,97,116,105,111,110,72,101,108,112,101,114,0]) [CLSID_KeyAttestationHelper]);
DEFINE_IID!(IID_IKeyAttestationHelperStatics, 373875270, 63044, 17190, 136, 190, 58, 241, 2, 211, 14, 12);
RT_INTERFACE!{static interface IKeyAttestationHelperStatics(IKeyAttestationHelperStaticsVtbl): IInspectable [IID_IKeyAttestationHelperStatics] {
    fn DecryptTpmAttestationCredentialAsync(&self, credential: HSTRING, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn GetTpmAttestationCredentialId(&self, credential: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IKeyAttestationHelperStatics {
    #[inline] pub fn decrypt_tpm_attestation_credential_async(&self, credential: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecryptTpmAttestationCredentialAsync)(self.get_abi() as *const _ as *mut _, credential.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tpm_attestation_credential_id(&self, credential: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetTpmAttestationCredentialId)(self.get_abi() as *const _ as *mut _, credential.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyAttestationHelperStatics2, 2623081260, 42694, 19038, 158, 100, 232, 93, 82, 121, 223, 151);
RT_INTERFACE!{static interface IKeyAttestationHelperStatics2(IKeyAttestationHelperStatics2Vtbl): IInspectable [IID_IKeyAttestationHelperStatics2] {
    fn DecryptTpmAttestationCredentialWithContainerNameAsync(&self, credential: HSTRING, containerName: HSTRING, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT
}}
impl IKeyAttestationHelperStatics2 {
    #[inline] pub fn decrypt_tpm_attestation_credential_with_container_name_async(&self, credential: &HStringArg, containerName: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecryptTpmAttestationCredentialWithContainerNameAsync)(self.get_abi() as *const _ as *mut _, credential.get(), containerName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum KeyProtectionLevel: i32 {
    NoConsent = 0, ConsentOnly = 1, ConsentWithPassword = 2, ConsentWithFingerprint = 3,
}}
RT_ENUM! { enum KeySize: i32 {
    Invalid = 0, Rsa2048 = 2048, Rsa4096 = 4096,
}}
RT_CLASS!{static class KeyStorageProviderNames}
impl RtActivatable<IKeyStorageProviderNamesStatics> for KeyStorageProviderNames {}
impl RtActivatable<IKeyStorageProviderNamesStatics2> for KeyStorageProviderNames {}
impl KeyStorageProviderNames {
    #[inline] pub fn get_software_key_storage_provider() -> Result<HString> {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_software_key_storage_provider()
    }
    #[inline] pub fn get_smartcard_key_storage_provider() -> Result<HString> {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_smartcard_key_storage_provider()
    }
    #[inline] pub fn get_platform_key_storage_provider() -> Result<HString> {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics>>::get_activation_factory().get_platform_key_storage_provider()
    }
    #[inline] pub fn get_passport_key_storage_provider() -> Result<HString> {
        <Self as RtActivatable<IKeyStorageProviderNamesStatics2>>::get_activation_factory().get_passport_key_storage_provider()
    }
}
DEFINE_CLSID!(KeyStorageProviderNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,75,101,121,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,78,97,109,101,115,0]) [CLSID_KeyStorageProviderNames]);
DEFINE_IID!(IID_IKeyStorageProviderNamesStatics, 2937613024, 21801, 17922, 189, 148, 10, 171, 145, 149, 123, 92);
RT_INTERFACE!{static interface IKeyStorageProviderNamesStatics(IKeyStorageProviderNamesStaticsVtbl): IInspectable [IID_IKeyStorageProviderNamesStatics] {
    fn get_SoftwareKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SmartcardKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PlatformKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyStorageProviderNamesStatics {
    #[inline] pub fn get_software_key_storage_provider(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SoftwareKeyStorageProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_smartcard_key_storage_provider(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SmartcardKeyStorageProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_platform_key_storage_provider(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PlatformKeyStorageProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyStorageProviderNamesStatics2, 640513085, 39982, 16844, 136, 18, 196, 217, 113, 221, 124, 96);
RT_INTERFACE!{static interface IKeyStorageProviderNamesStatics2(IKeyStorageProviderNamesStatics2Vtbl): IInspectable [IID_IKeyStorageProviderNamesStatics2] {
    fn get_PassportKeyStorageProvider(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyStorageProviderNamesStatics2 {
    #[inline] pub fn get_passport_key_storage_provider(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PassportKeyStorageProvider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPfxImportParameters, 1745696017, 39432, 18376, 134, 74, 46, 221, 77, 142, 180, 108);
RT_INTERFACE!{interface IPfxImportParameters(IPfxImportParametersVtbl): IInspectable [IID_IPfxImportParameters] {
    fn get_Exportable(&self, out: *mut ExportOption) -> HRESULT,
    fn put_Exportable(&self, value: ExportOption) -> HRESULT,
    fn get_KeyProtectionLevel(&self, out: *mut KeyProtectionLevel) -> HRESULT,
    fn put_KeyProtectionLevel(&self, value: KeyProtectionLevel) -> HRESULT,
    fn get_InstallOptions(&self, out: *mut InstallOptions) -> HRESULT,
    fn put_InstallOptions(&self, value: InstallOptions) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_FriendlyName(&self, value: HSTRING) -> HRESULT,
    fn get_KeyStorageProviderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_KeyStorageProviderName(&self, value: HSTRING) -> HRESULT,
    fn get_ContainerNamePrefix(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContainerNamePrefix(&self, value: HSTRING) -> HRESULT,
    fn get_ReaderName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ReaderName(&self, value: HSTRING) -> HRESULT
}}
impl IPfxImportParameters {
    #[inline] pub fn get_exportable(&self) -> Result<ExportOption> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Exportable)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_exportable(&self, value: ExportOption) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Exportable)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_protection_level(&self) -> Result<KeyProtectionLevel> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeyProtectionLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_key_protection_level(&self, value: KeyProtectionLevel) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyProtectionLevel)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_install_options(&self) -> Result<InstallOptions> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_InstallOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_install_options(&self, value: InstallOptions) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_InstallOptions)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_friendly_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_FriendlyName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_key_storage_provider_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KeyStorageProviderName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_key_storage_provider_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KeyStorageProviderName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_container_name_prefix(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ContainerNamePrefix)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_container_name_prefix(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ContainerNamePrefix)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_reader_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ReaderName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_reader_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ReaderName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PfxImportParameters: IPfxImportParameters}
impl RtActivatable<IActivationFactory> for PfxImportParameters {}
DEFINE_CLSID!(PfxImportParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,80,102,120,73,109,112,111,114,116,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_PfxImportParameters]);
RT_ENUM! { enum SignatureValidationResult: i32 {
    Success = 0, InvalidParameter = 1, BadMessage = 2, InvalidSignature = 3, OtherErrors = 4,
}}
RT_CLASS!{static class StandardCertificateStoreNames}
impl RtActivatable<IStandardCertificateStoreNamesStatics> for StandardCertificateStoreNames {}
impl StandardCertificateStoreNames {
    #[inline] pub fn get_personal() -> Result<HString> {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_personal()
    }
    #[inline] pub fn get_trusted_root_certification_authorities() -> Result<HString> {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_trusted_root_certification_authorities()
    }
    #[inline] pub fn get_intermediate_certification_authorities() -> Result<HString> {
        <Self as RtActivatable<IStandardCertificateStoreNamesStatics>>::get_activation_factory().get_intermediate_certification_authorities()
    }
}
DEFINE_CLSID!(StandardCertificateStoreNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,83,116,97,110,100,97,114,100,67,101,114,116,105,102,105,99,97,116,101,83,116,111,114,101,78,97,109,101,115,0]) [CLSID_StandardCertificateStoreNames]);
DEFINE_IID!(IID_IStandardCertificateStoreNamesStatics, 202722011, 42134, 16888, 143, 229, 158, 150, 243, 110, 251, 248);
RT_INTERFACE!{static interface IStandardCertificateStoreNamesStatics(IStandardCertificateStoreNamesStaticsVtbl): IInspectable [IID_IStandardCertificateStoreNamesStatics] {
    fn get_Personal(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TrustedRootCertificationAuthorities(&self, out: *mut HSTRING) -> HRESULT,
    fn get_IntermediateCertificationAuthorities(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardCertificateStoreNamesStatics {
    #[inline] pub fn get_personal(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Personal)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_trusted_root_certification_authorities(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TrustedRootCertificationAuthorities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_intermediate_certification_authorities(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IntermediateCertificationAuthorities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISubjectAlternativeNameInfo, 1479039473, 22173, 19488, 190, 123, 78, 28, 154, 11, 197, 43);
RT_INTERFACE!{interface ISubjectAlternativeNameInfo(ISubjectAlternativeNameInfoVtbl): IInspectable [IID_ISubjectAlternativeNameInfo] {
    fn get_EmailName(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_IPAddress(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_Url(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_DnsName(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_DistinguishedName(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_PrincipalName(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl ISubjectAlternativeNameInfo {
    #[inline] pub fn get_email_name(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EmailName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ip_address(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IPAddress)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_url(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Url)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dns_name(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DnsName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_distinguished_name(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DistinguishedName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_principal_name(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PrincipalName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SubjectAlternativeNameInfo: ISubjectAlternativeNameInfo}
impl RtActivatable<IActivationFactory> for SubjectAlternativeNameInfo {}
DEFINE_CLSID!(SubjectAlternativeNameInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,101,114,116,105,102,105,99,97,116,101,115,46,83,117,98,106,101,99,116,65,108,116,101,114,110,97,116,105,118,101,78,97,109,101,73,110,102,111,0]) [CLSID_SubjectAlternativeNameInfo]);
DEFINE_IID!(IID_ISubjectAlternativeNameInfo2, 1132099782, 7249, 16874, 179, 74, 61, 101, 67, 152, 163, 112);
RT_INTERFACE!{interface ISubjectAlternativeNameInfo2(ISubjectAlternativeNameInfo2Vtbl): IInspectable [IID_ISubjectAlternativeNameInfo2] {
    fn get_EmailNames(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_IPAddresses(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Urls(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_DnsNames(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_DistinguishedNames(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_PrincipalNames(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Extension(&self, out: *mut <CertificateExtension as RtType>::Abi) -> HRESULT
}}
impl ISubjectAlternativeNameInfo2 {
    #[inline] pub fn get_email_names(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EmailNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ip_addresses(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_IPAddresses)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_urls(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Urls)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dns_names(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DnsNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_distinguished_names(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DistinguishedNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_principal_names(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PrincipalNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_extension(&self) -> Result<Option<CertificateExtension>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Extension)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CertificateExtension::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUserCertificateEnrollmentManager, 2519807768, 8929, 18457, 178, 11, 171, 70, 166, 236, 160, 110);
RT_INTERFACE!{interface IUserCertificateEnrollmentManager(IUserCertificateEnrollmentManagerVtbl): IInspectable [IID_IUserCertificateEnrollmentManager] {
    fn CreateRequestAsync(&self, request: <CertificateRequestProperties as RtType>::Abi, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn InstallCertificateAsync(&self, certificate: HSTRING, installOption: InstallOptions, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ImportPfxDataAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ImportPfxDataToKspAsync(&self, pfxData: HSTRING, password: HSTRING, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: HSTRING, keyStorageProvider: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IUserCertificateEnrollmentManager {
    #[inline] pub fn create_request_async(&self, request: &CertificateRequestProperties) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateRequestAsync)(self.get_abi() as *const _ as *mut _, request.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn install_certificate_async(&self, certificate: &HStringArg, installOption: InstallOptions) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().InstallCertificateAsync)(self.get_abi() as *const _ as *mut _, certificate.get(), installOption, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_pfx_data_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_pfx_data_to_ksp_async(&self, pfxData: &HStringArg, password: &HStringArg, exportable: ExportOption, keyProtectionLevel: KeyProtectionLevel, installOption: InstallOptions, friendlyName: &HStringArg, keyStorageProvider: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataToKspAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), exportable, keyProtectionLevel, installOption, friendlyName.get(), keyStorageProvider.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserCertificateEnrollmentManager: IUserCertificateEnrollmentManager}
DEFINE_IID!(IID_IUserCertificateEnrollmentManager2, 229481649, 26078, 18730, 184, 109, 252, 92, 72, 44, 55, 71);
RT_INTERFACE!{interface IUserCertificateEnrollmentManager2(IUserCertificateEnrollmentManager2Vtbl): IInspectable [IID_IUserCertificateEnrollmentManager2] {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxData: HSTRING, password: HSTRING, pfxImportParameters: <PfxImportParameters as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IUserCertificateEnrollmentManager2 {
    #[inline] pub fn import_pfx_data_to_ksp_with_parameters_async(&self, pfxData: &HStringArg, password: &HStringArg, pfxImportParameters: &PfxImportParameters) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPfxDataToKspWithParametersAsync)(self.get_abi() as *const _ as *mut _, pfxData.get(), password.get(), pfxImportParameters.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IUserCertificateStore, 3388677507, 30879, 19278, 145, 128, 4, 90, 117, 122, 172, 109);
RT_INTERFACE!{interface IUserCertificateStore(IUserCertificateStoreVtbl): IInspectable [IID_IUserCertificateStore] {
    fn RequestAddAsync(&self, certificate: <Certificate as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn RequestDeleteAsync(&self, certificate: <Certificate as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT
}}
impl IUserCertificateStore {
    #[inline] pub fn request_add_async(&self, certificate: &Certificate) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAddAsync)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_delete_async(&self, certificate: &Certificate) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestDeleteAsync)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserCertificateStore: IUserCertificateStore}
} // Windows.Security.Cryptography.Certificates
pub mod core { // Windows.Security.Cryptography.Core
use crate::prelude::*;
RT_CLASS!{static class AsymmetricAlgorithmNames}
impl RtActivatable<IAsymmetricAlgorithmNamesStatics> for AsymmetricAlgorithmNames {}
impl RtActivatable<IAsymmetricAlgorithmNamesStatics2> for AsymmetricAlgorithmNames {}
impl AsymmetricAlgorithmNames {
    #[inline] pub fn get_rsa_pkcs1() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_pkcs1()
    }
    #[inline] pub fn get_rsa_oaep_sha1() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha1()
    }
    #[inline] pub fn get_rsa_oaep_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha256()
    }
    #[inline] pub fn get_rsa_oaep_sha384() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha384()
    }
    #[inline] pub fn get_rsa_oaep_sha512() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_oaep_sha512()
    }
    #[inline] pub fn get_ecdsa_p256_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p256_sha256()
    }
    #[inline] pub fn get_ecdsa_p384_sha384() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p384_sha384()
    }
    #[inline] pub fn get_ecdsa_p521_sha512() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_ecdsa_p521_sha512()
    }
    #[inline] pub fn get_dsa_sha1() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_dsa_sha1()
    }
    #[inline] pub fn get_dsa_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_dsa_sha256()
    }
    #[inline] pub fn get_rsa_sign_pkcs1_sha1() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha1()
    }
    #[inline] pub fn get_rsa_sign_pkcs1_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha256()
    }
    #[inline] pub fn get_rsa_sign_pkcs1_sha384() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha384()
    }
    #[inline] pub fn get_rsa_sign_pkcs1_sha512() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pkcs1_sha512()
    }
    #[inline] pub fn get_rsa_sign_pss_sha1() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha1()
    }
    #[inline] pub fn get_rsa_sign_pss_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha256()
    }
    #[inline] pub fn get_rsa_sign_pss_sha384() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha384()
    }
    #[inline] pub fn get_rsa_sign_pss_sha512() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rsa_sign_pss_sha512()
    }
    #[inline] pub fn get_ecdsa_sha256() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha256()
    }
    #[inline] pub fn get_ecdsa_sha384() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha384()
    }
    #[inline] pub fn get_ecdsa_sha512() -> Result<HString> {
        <Self as RtActivatable<IAsymmetricAlgorithmNamesStatics2>>::get_activation_factory().get_ecdsa_sha512()
    }
}
DEFINE_CLSID!(AsymmetricAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,65,115,121,109,109,101,116,114,105,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_AsymmetricAlgorithmNames]);
DEFINE_IID!(IID_IAsymmetricAlgorithmNamesStatics, 3405184228, 26560, 18090, 132, 249, 117, 46, 119, 68, 159, 155);
RT_INTERFACE!{static interface IAsymmetricAlgorithmNamesStatics(IAsymmetricAlgorithmNamesStaticsVtbl): IInspectable [IID_IAsymmetricAlgorithmNamesStatics] {
    fn get_RsaPkcs1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaOaepSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP256Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP384Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaP521Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DsaSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DsaSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPkcs1Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RsaSignPssSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAsymmetricAlgorithmNamesStatics {
    #[inline] pub fn get_rsa_pkcs1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaPkcs1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_oaep_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaOaepSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_oaep_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaOaepSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_oaep_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaOaepSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_oaep_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaOaepSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_p256_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaP256Sha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_p384_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaP384Sha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_p521_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaP521Sha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dsa_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DsaSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_dsa_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DsaSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPkcs1Sha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPkcs1Sha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPkcs1Sha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pkcs1_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPkcs1Sha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pss_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPssSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pss_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPssSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pss_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPssSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rsa_sign_pss_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RsaSignPssSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAsymmetricAlgorithmNamesStatics2, 4047618262, 19455, 20259, 186, 102, 96, 69, 177, 55, 213, 223);
RT_INTERFACE!{static interface IAsymmetricAlgorithmNamesStatics2(IAsymmetricAlgorithmNamesStatics2Vtbl): IInspectable [IID_IAsymmetricAlgorithmNamesStatics2] {
    fn get_EcdsaSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_EcdsaSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAsymmetricAlgorithmNamesStatics2 {
    #[inline] pub fn get_ecdsa_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ecdsa_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EcdsaSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProvider, 3906142007, 25177, 20104, 183, 224, 148, 25, 31, 222, 105, 158);
RT_INTERFACE!{interface IAsymmetricKeyAlgorithmProvider(IAsymmetricKeyAlgorithmProviderVtbl): IInspectable [IID_IAsymmetricKeyAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn CreateKeyPair(&self, keySize: u32, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportDefaultPrivateKeyBlob(&self, keyBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportKeyPairWithBlobType(&self, keyBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, blobType: CryptographicPrivateKeyBlobType, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportDefaultPublicKeyBlob(&self, keyBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ImportPublicKeyWithBlobType(&self, keyBlob: <crate::windows::storage::streams::IBuffer as RtType>::Abi, blobType: CryptographicPublicKeyBlobType, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProvider {
    #[inline] pub fn get_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_key_pair(&self, keySize: u32) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateKeyPair)(self.get_abi() as *const _ as *mut _, keySize, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn import_default_private_key_blob(&self, keyBlob: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportDefaultPrivateKeyBlob)(self.get_abi() as *const _ as *mut _, keyBlob.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn import_key_pair_with_blob_type(&self, keyBlob: &crate::windows::storage::streams::IBuffer, blobType: CryptographicPrivateKeyBlobType) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportKeyPairWithBlobType)(self.get_abi() as *const _ as *mut _, keyBlob.get_abi() as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn import_default_public_key_blob(&self, keyBlob: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportDefaultPublicKeyBlob)(self.get_abi() as *const _ as *mut _, keyBlob.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn import_public_key_with_blob_type(&self, keyBlob: &crate::windows::storage::streams::IBuffer, blobType: CryptographicPublicKeyBlobType) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ImportPublicKeyWithBlobType)(self.get_abi() as *const _ as *mut _, keyBlob.get_abi() as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AsymmetricKeyAlgorithmProvider: IAsymmetricKeyAlgorithmProvider}
impl RtActivatable<IAsymmetricKeyAlgorithmProviderStatics> for AsymmetricKeyAlgorithmProvider {}
impl AsymmetricKeyAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<Option<AsymmetricKeyAlgorithmProvider>> {
        <Self as RtActivatable<IAsymmetricKeyAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }
}
DEFINE_CLSID!(AsymmetricKeyAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,65,115,121,109,109,101,116,114,105,99,75,101,121,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_AsymmetricKeyAlgorithmProvider]);
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProvider2, 1311910526, 31821, 18839, 172, 79, 27, 132, 139, 54, 48, 110);
RT_INTERFACE!{interface IAsymmetricKeyAlgorithmProvider2(IAsymmetricKeyAlgorithmProvider2Vtbl): IInspectable [IID_IAsymmetricKeyAlgorithmProvider2] {
    fn CreateKeyPairWithCurveName(&self, curveName: HSTRING, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT,
    fn CreateKeyPairWithCurveParameters(&self, parametersSize: u32, parameters: *mut u8, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProvider2 {
    #[inline] pub fn create_key_pair_with_curve_name(&self, curveName: &HStringArg) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateKeyPairWithCurveName)(self.get_abi() as *const _ as *mut _, curveName.get(), &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_key_pair_with_curve_parameters(&self, parameters: &[u8]) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateKeyPairWithCurveParameters)(self.get_abi() as *const _ as *mut _, parameters.len() as u32, parameters.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IAsymmetricKeyAlgorithmProviderStatics, 1113316888, 42995, 18342, 168, 210, 196, 141, 96, 51, 166, 92);
RT_INTERFACE!{static interface IAsymmetricKeyAlgorithmProviderStatics(IAsymmetricKeyAlgorithmProviderStaticsVtbl): IInspectable [IID_IAsymmetricKeyAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut <AsymmetricKeyAlgorithmProvider as RtType>::Abi) -> HRESULT
}}
impl IAsymmetricKeyAlgorithmProviderStatics {
    #[inline] pub fn open_algorithm(&self, algorithm: &HStringArg) -> Result<Option<AsymmetricKeyAlgorithmProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAlgorithm)(self.get_abi() as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(AsymmetricKeyAlgorithmProvider::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum Capi1KdfTargetAlgorithm: i32 {
    NotAes = 0, Aes = 1,
}}
RT_CLASS!{static class CryptographicEngine}
impl RtActivatable<ICryptographicEngineStatics> for CryptographicEngine {}
impl RtActivatable<ICryptographicEngineStatics2> for CryptographicEngine {}
impl CryptographicEngine {
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().encrypt(key, data, iv)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().decrypt(key, data, iv)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt_and_authenticate(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, nonce: &crate::windows::storage::streams::IBuffer, authenticatedData: &crate::windows::storage::streams::IBuffer) -> Result<Option<EncryptedAndAuthenticatedData>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().encrypt_and_authenticate(key, data, nonce, authenticatedData)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_and_authenticate(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, nonce: &crate::windows::storage::streams::IBuffer, authenticationTag: &crate::windows::storage::streams::IBuffer, authenticatedData: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().decrypt_and_authenticate(key, data, nonce, authenticationTag, authenticatedData)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn sign(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().sign(key, data)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, signature: &crate::windows::storage::streams::IBuffer) -> Result<bool> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().verify_signature(key, data, signature)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn derive_key_material(key: &CryptographicKey, parameters: &KeyDerivationParameters, desiredKeySize: u32) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics>>::get_activation_factory().derive_key_material(key, parameters, desiredKeySize)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_hashed_data(key, data)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature_with_hash_input(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, signature: &crate::windows::storage::streams::IBuffer) -> Result<bool> {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().verify_signature_with_hash_input(key, data, signature)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_async(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().decrypt_async(key, data, iv)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_async(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_async(key, data)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data_async(key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> {
        <Self as RtActivatable<ICryptographicEngineStatics2>>::get_activation_factory().sign_hashed_data_async(key, data)
    }
}
DEFINE_CLSID!(CryptographicEngine(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,67,114,121,112,116,111,103,114,97,112,104,105,99,69,110,103,105,110,101,0]) [CLSID_CryptographicEngine]);
DEFINE_IID!(IID_ICryptographicEngineStatics, 2682914361, 28663, 19589, 160, 149, 149, 235, 49, 113, 94, 185);
RT_INTERFACE!{static interface ICryptographicEngineStatics(ICryptographicEngineStaticsVtbl): IInspectable [IID_ICryptographicEngineStatics] {
    #[cfg(feature="windows-storage")] fn Encrypt(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, iv: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn Decrypt(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, iv: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn EncryptAndAuthenticate(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, nonce: <crate::windows::storage::streams::IBuffer as RtType>::Abi, authenticatedData: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <EncryptedAndAuthenticatedData as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecryptAndAuthenticate(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, nonce: <crate::windows::storage::streams::IBuffer as RtType>::Abi, authenticationTag: <crate::windows::storage::streams::IBuffer as RtType>::Abi, authenticatedData: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn Sign(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignature(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, signature: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DeriveKeyMaterial(&self, key: <CryptographicKey as RtType>::Abi, parameters: <KeyDerivationParameters as RtType>::Abi, desiredKeySize: u32, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl ICryptographicEngineStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Encrypt)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, iv.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Decrypt)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, iv.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn encrypt_and_authenticate(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, nonce: &crate::windows::storage::streams::IBuffer, authenticatedData: &crate::windows::storage::streams::IBuffer) -> Result<Option<EncryptedAndAuthenticatedData>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().EncryptAndAuthenticate)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, nonce.get_abi() as *const _ as *mut _, authenticatedData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(EncryptedAndAuthenticatedData::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_and_authenticate(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, nonce: &crate::windows::storage::streams::IBuffer, authenticationTag: &crate::windows::storage::streams::IBuffer, authenticatedData: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecryptAndAuthenticate)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, nonce.get_abi() as *const _ as *mut _, authenticationTag.get_abi() as *const _ as *mut _, authenticatedData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Sign)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, signature: &crate::windows::storage::streams::IBuffer) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().VerifySignature)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, signature.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn derive_key_material(&self, key: &CryptographicKey, parameters: &KeyDerivationParameters, desiredKeySize: u32) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DeriveKeyMaterial)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, parameters.get_abi() as *const _ as *mut _, desiredKeySize, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICryptographicEngineStatics2, 1733904638, 57247, 16785, 146, 199, 108, 230, 245, 132, 32, 224);
RT_INTERFACE!{static interface ICryptographicEngineStatics2(ICryptographicEngineStatics2Vtbl): IInspectable [IID_ICryptographicEngineStatics2] {
    #[cfg(feature="windows-storage")] fn SignHashedData(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn VerifySignatureWithHashInput(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, signature: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-storage")] fn DecryptAsync(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, iv: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SignAsync(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SignHashedDataAsync(&self, key: <CryptographicKey as RtType>::Abi, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT
}}
impl ICryptographicEngineStatics2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignHashedData)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn verify_signature_with_hash_input(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, signature: &crate::windows::storage::streams::IBuffer) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().VerifySignatureWithHashInput)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, signature.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn decrypt_async(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer, iv: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().DecryptAsync)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, iv.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_async(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignAsync)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn sign_hashed_data_async(&self, key: &CryptographicKey, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SignHashedDataAsync)(self.get_abi() as *const _ as *mut _, key.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CryptographicHash: IHashComputation}
DEFINE_IID!(IID_ICryptographicKey, 3978967920, 36475, 16393, 132, 1, 255, 209, 166, 46, 235, 39);
RT_INTERFACE!{interface ICryptographicKey(ICryptographicKeyVtbl): IInspectable [IID_ICryptographicKey] {
    fn get_KeySize(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportDefaultPrivateKeyBlobType(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportPrivateKeyWithBlobType(&self, blobType: CryptographicPrivateKeyBlobType, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportDefaultPublicKeyBlobType(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ExportPublicKeyWithBlobType(&self, blobType: CryptographicPublicKeyBlobType, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl ICryptographicKey {
    #[inline] pub fn get_key_size(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_KeySize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn export_default_private_key_blob_type(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ExportDefaultPrivateKeyBlobType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn export_private_key_with_blob_type(&self, blobType: CryptographicPrivateKeyBlobType) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ExportPrivateKeyWithBlobType)(self.get_abi() as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn export_default_public_key_blob_type(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ExportDefaultPublicKeyBlobType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn export_public_key_with_blob_type(&self, blobType: CryptographicPublicKeyBlobType) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ExportPublicKeyWithBlobType)(self.get_abi() as *const _ as *mut _, blobType, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class CryptographicKey: ICryptographicKey}
RT_ENUM! { enum CryptographicPadding: i32 {
    None = 0, RsaOaep = 1, RsaPkcs1V15 = 2, RsaPss = 3,
}}
RT_ENUM! { enum CryptographicPrivateKeyBlobType: i32 {
    Pkcs8RawPrivateKeyInfo = 0, Pkcs1RsaPrivateKey = 1, BCryptPrivateKey = 2, Capi1PrivateKey = 3, BCryptEccFullPrivateKey = 4,
}}
RT_ENUM! { enum CryptographicPublicKeyBlobType: i32 {
    X509SubjectPublicKeyInfo = 0, Pkcs1RsaPublicKey = 1, BCryptPublicKey = 2, Capi1PublicKey = 3, BCryptEccFullPublicKey = 4,
}}
RT_CLASS!{static class EccCurveNames}
impl RtActivatable<IEccCurveNamesStatics> for EccCurveNames {}
impl EccCurveNames {
    #[inline] pub fn get_brainpool_p160r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p160r1()
    }
    #[inline] pub fn get_brainpool_p160t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p160t1()
    }
    #[inline] pub fn get_brainpool_p192r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p192r1()
    }
    #[inline] pub fn get_brainpool_p192t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p192t1()
    }
    #[inline] pub fn get_brainpool_p224r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p224r1()
    }
    #[inline] pub fn get_brainpool_p224t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p224t1()
    }
    #[inline] pub fn get_brainpool_p256r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p256r1()
    }
    #[inline] pub fn get_brainpool_p256t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p256t1()
    }
    #[inline] pub fn get_brainpool_p320r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p320r1()
    }
    #[inline] pub fn get_brainpool_p320t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p320t1()
    }
    #[inline] pub fn get_brainpool_p384r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p384r1()
    }
    #[inline] pub fn get_brainpool_p384t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p384t1()
    }
    #[inline] pub fn get_brainpool_p512r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p512r1()
    }
    #[inline] pub fn get_brainpool_p512t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_brainpool_p512t1()
    }
    #[inline] pub fn get_curve_25519() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_curve_25519()
    }
    #[inline] pub fn get_ec192wapi() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_ec192wapi()
    }
    #[inline] pub fn get_nist_p192() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p192()
    }
    #[inline] pub fn get_nist_p224() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p224()
    }
    #[inline] pub fn get_nist_p256() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p256()
    }
    #[inline] pub fn get_nist_p384() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p384()
    }
    #[inline] pub fn get_nist_p521() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nist_p521()
    }
    #[inline] pub fn get_nums_p256t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p256t1()
    }
    #[inline] pub fn get_nums_p384t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p384t1()
    }
    #[inline] pub fn get_nums_p512t1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_nums_p512t1()
    }
    #[inline] pub fn get_sec_p160k1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160k1()
    }
    #[inline] pub fn get_sec_p160r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160r1()
    }
    #[inline] pub fn get_sec_p160r2() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p160r2()
    }
    #[inline] pub fn get_sec_p192k1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p192k1()
    }
    #[inline] pub fn get_sec_p192r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p192r1()
    }
    #[inline] pub fn get_sec_p224k1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p224k1()
    }
    #[inline] pub fn get_sec_p224r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p224r1()
    }
    #[inline] pub fn get_sec_p256k1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p256k1()
    }
    #[inline] pub fn get_sec_p256r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p256r1()
    }
    #[inline] pub fn get_sec_p384r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p384r1()
    }
    #[inline] pub fn get_sec_p521r1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_sec_p521r1()
    }
    #[inline] pub fn get_wtls7() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls7()
    }
    #[inline] pub fn get_wtls9() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls9()
    }
    #[inline] pub fn get_wtls12() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_wtls12()
    }
    #[inline] pub fn get_x962p192v1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p192v1()
    }
    #[inline] pub fn get_x962p192v2() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p192v2()
    }
    #[inline] pub fn get_x962p192v3() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p192v3()
    }
    #[inline] pub fn get_x962p239v1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p239v1()
    }
    #[inline] pub fn get_x962p239v2() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p239v2()
    }
    #[inline] pub fn get_x962p239v3() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p239v3()
    }
    #[inline] pub fn get_x962p256v1() -> Result<HString> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_x962p256v1()
    }
    #[inline] pub fn get_all_ecc_curve_names() -> Result<Option<foundation::collections::IVectorView<HString>>> {
        <Self as RtActivatable<IEccCurveNamesStatics>>::get_activation_factory().get_all_ecc_curve_names()
    }
}
DEFINE_CLSID!(EccCurveNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,69,99,99,67,117,114,118,101,78,97,109,101,115,0]) [CLSID_EccCurveNames]);
DEFINE_IID!(IID_IEccCurveNamesStatics, 3019870988, 44779, 16542, 183, 212, 155, 149, 41, 90, 174, 207);
RT_INTERFACE!{static interface IEccCurveNamesStatics(IEccCurveNamesStaticsVtbl): IInspectable [IID_IEccCurveNamesStatics] {
    fn get_BrainpoolP160r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP160t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP192r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP192t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP224r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP224t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP256r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP256t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP320r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP320t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP384r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP384t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP512r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BrainpoolP512t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Curve25519(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Ec192wapi(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP192(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP224(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NistP521(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP256t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP384t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NumsP512t1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP160r2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP192k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP192r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP224k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP224r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP256k1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP256r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP384r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SecP521r1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls9(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Wtls12(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P192v3(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v2(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P239v3(&self, out: *mut HSTRING) -> HRESULT,
    fn get_X962P256v1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AllEccCurveNames(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IEccCurveNamesStatics {
    #[inline] pub fn get_brainpool_p160r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP160r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p160t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP160t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p192r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP192r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p192t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP192t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p224r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP224r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p224t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP224t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p256r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP256r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p256t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP256t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p320r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP320r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p320t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP320t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p384r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP384r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p384t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP384t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p512r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP512r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_brainpool_p512t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_BrainpoolP512t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_curve_25519(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Curve25519)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_ec192wapi(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Ec192wapi)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nist_p192(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NistP192)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nist_p224(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NistP224)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nist_p256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NistP256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nist_p384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NistP384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nist_p521(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NistP521)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nums_p256t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumsP256t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nums_p384t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumsP384t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_nums_p512t1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_NumsP512t1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p160k1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP160k1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p160r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP160r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p160r2(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP160r2)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p192k1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP192k1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p192r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP192r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p224k1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP224k1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p224r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP224r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p256k1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP256k1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p256r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP256r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p384r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP384r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sec_p521r1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SecP521r1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_wtls7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Wtls7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_wtls9(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Wtls9)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_wtls12(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Wtls12)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p192v1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P192v1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p192v2(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P192v2)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p192v3(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P192v3)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p239v1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P239v1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p239v2(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P239v2)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p239v3(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P239v3)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_x962p256v1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_X962P256v1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_all_ecc_curve_names(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AllEccCurveNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IEncryptedAndAuthenticatedData, 1873031143, 7883, 19200, 190, 165, 96, 184, 63, 134, 47, 23);
RT_INTERFACE!{interface IEncryptedAndAuthenticatedData(IEncryptedAndAuthenticatedDataVtbl): IInspectable [IID_IEncryptedAndAuthenticatedData] {
    #[cfg(feature="windows-storage")] fn get_EncryptedData(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_AuthenticationTag(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl IEncryptedAndAuthenticatedData {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_encrypted_data(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_EncryptedData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_authentication_tag(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AuthenticationTag)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class EncryptedAndAuthenticatedData: IEncryptedAndAuthenticatedData}
RT_CLASS!{static class HashAlgorithmNames}
impl RtActivatable<IHashAlgorithmNamesStatics> for HashAlgorithmNames {}
impl HashAlgorithmNames {
    #[inline] pub fn get_md5() -> Result<HString> {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_md5()
    }
    #[inline] pub fn get_sha1() -> Result<HString> {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha1()
    }
    #[inline] pub fn get_sha256() -> Result<HString> {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha256()
    }
    #[inline] pub fn get_sha384() -> Result<HString> {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha384()
    }
    #[inline] pub fn get_sha512() -> Result<HString> {
        <Self as RtActivatable<IHashAlgorithmNamesStatics>>::get_activation_factory().get_sha512()
    }
}
DEFINE_CLSID!(HashAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,72,97,115,104,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_HashAlgorithmNames]);
DEFINE_IID!(IID_IHashAlgorithmNamesStatics, 1801323798, 56982, 20234, 141, 87, 220, 201, 218, 227, 108, 118);
RT_INTERFACE!{static interface IHashAlgorithmNamesStatics(IHashAlgorithmNamesStaticsVtbl): IInspectable [IID_IHashAlgorithmNamesStatics] {
    fn get_Md5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IHashAlgorithmNamesStatics {
    #[inline] pub fn get_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Md5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHashAlgorithmProvider, 3197841536, 45763, 16939, 188, 225, 236, 144, 239, 181, 215, 181);
RT_INTERFACE!{interface IHashAlgorithmProvider(IHashAlgorithmProviderVtbl): IInspectable [IID_IHashAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HashLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn HashData(&self, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn CreateHash(&self, out: *mut <CryptographicHash as RtType>::Abi) -> HRESULT
}}
impl IHashAlgorithmProvider {
    #[inline] pub fn get_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hash_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_HashLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn hash_data(&self, data: &crate::windows::storage::streams::IBuffer) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().HashData)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_hash(&self) -> Result<Option<CryptographicHash>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateHash)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicHash::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HashAlgorithmProvider: IHashAlgorithmProvider}
impl RtActivatable<IHashAlgorithmProviderStatics> for HashAlgorithmProvider {}
impl HashAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<Option<HashAlgorithmProvider>> {
        <Self as RtActivatable<IHashAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }
}
DEFINE_CLSID!(HashAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,72,97,115,104,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_HashAlgorithmProvider]);
DEFINE_IID!(IID_IHashAlgorithmProviderStatics, 2678888257, 23748, 17206, 174, 56, 98, 18, 183, 90, 145, 90);
RT_INTERFACE!{static interface IHashAlgorithmProviderStatics(IHashAlgorithmProviderStaticsVtbl): IInspectable [IID_IHashAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut <HashAlgorithmProvider as RtType>::Abi) -> HRESULT
}}
impl IHashAlgorithmProviderStatics {
    #[inline] pub fn open_algorithm(&self, algorithm: &HStringArg) -> Result<Option<HashAlgorithmProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAlgorithm)(self.get_abi() as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(HashAlgorithmProvider::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHashComputation, 1493488054, 44337, 17923, 163, 164, 177, 189, 169, 142, 37, 98);
RT_INTERFACE!{interface IHashComputation(IHashComputationVtbl): IInspectable [IID_IHashComputation] {
    #[cfg(feature="windows-storage")] fn Append(&self, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetValueAndReset(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl IHashComputation {
    #[cfg(feature="windows-storage")] #[inline] pub fn append(&self, data: &crate::windows::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Append)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_value_and_reset(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetValueAndReset)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class KeyDerivationAlgorithmNames}
impl RtActivatable<IKeyDerivationAlgorithmNamesStatics> for KeyDerivationAlgorithmNames {}
impl RtActivatable<IKeyDerivationAlgorithmNamesStatics2> for KeyDerivationAlgorithmNames {}
impl KeyDerivationAlgorithmNames {
    #[inline] pub fn get_pbkdf2_md5() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_md5()
    }
    #[inline] pub fn get_pbkdf2_sha1() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha1()
    }
    #[inline] pub fn get_pbkdf2_sha256() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha256()
    }
    #[inline] pub fn get_pbkdf2_sha384() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha384()
    }
    #[inline] pub fn get_pbkdf2_sha512() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_pbkdf2_sha512()
    }
    #[inline] pub fn get_sp800108_ctr_hmac_md5() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_md5()
    }
    #[inline] pub fn get_sp800108_ctr_hmac_sha1() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha1()
    }
    #[inline] pub fn get_sp800108_ctr_hmac_sha256() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha256()
    }
    #[inline] pub fn get_sp800108_ctr_hmac_sha384() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha384()
    }
    #[inline] pub fn get_sp800108_ctr_hmac_sha512() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp800108_ctr_hmac_sha512()
    }
    #[inline] pub fn get_sp80056a_concat_md5() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_md5()
    }
    #[inline] pub fn get_sp80056a_concat_sha1() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha1()
    }
    #[inline] pub fn get_sp80056a_concat_sha256() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha256()
    }
    #[inline] pub fn get_sp80056a_concat_sha384() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha384()
    }
    #[inline] pub fn get_sp80056a_concat_sha512() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics>>::get_activation_factory().get_sp80056a_concat_sha512()
    }
    #[inline] pub fn get_capi_kdf_md5() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_md5()
    }
    #[inline] pub fn get_capi_kdf_sha1() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha1()
    }
    #[inline] pub fn get_capi_kdf_sha256() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha256()
    }
    #[inline] pub fn get_capi_kdf_sha384() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha384()
    }
    #[inline] pub fn get_capi_kdf_sha512() -> Result<HString> {
        <Self as RtActivatable<IKeyDerivationAlgorithmNamesStatics2>>::get_activation_factory().get_capi_kdf_sha512()
    }
}
DEFINE_CLSID!(KeyDerivationAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_KeyDerivationAlgorithmNames]);
DEFINE_IID!(IID_IKeyDerivationAlgorithmNamesStatics, 2070820414, 38098, 18233, 165, 123, 2, 46, 12, 58, 64, 42);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmNamesStatics(IKeyDerivationAlgorithmNamesStaticsVtbl): IInspectable [IID_IKeyDerivationAlgorithmNamesStatics] {
    fn get_Pbkdf2Md5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pbkdf2Sha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp800108CtrHmacSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Sp80056aConcatSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyDerivationAlgorithmNamesStatics {
    #[inline] pub fn get_pbkdf2_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Pbkdf2Md5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pbkdf2_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Pbkdf2Sha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pbkdf2_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Pbkdf2Sha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pbkdf2_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Pbkdf2Sha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pbkdf2_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Pbkdf2Sha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp800108CtrHmacMd5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp800108CtrHmacSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp800108CtrHmacSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp800108CtrHmacSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp800108_ctr_hmac_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp800108CtrHmacSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp80056a_concat_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp80056aConcatMd5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp80056a_concat_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp80056aConcatSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp80056a_concat_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp80056aConcatSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp80056a_concat_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp80056aConcatSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sp80056a_concat_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Sp80056aConcatSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyDerivationAlgorithmNamesStatics2, 1469398955, 24644, 18031, 151, 244, 51, 123, 120, 8, 56, 77);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmNamesStatics2(IKeyDerivationAlgorithmNamesStatics2Vtbl): IInspectable [IID_IKeyDerivationAlgorithmNamesStatics2] {
    fn get_CapiKdfMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CapiKdfSha512(&self, out: *mut HSTRING) -> HRESULT
}}
impl IKeyDerivationAlgorithmNamesStatics2 {
    #[inline] pub fn get_capi_kdf_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CapiKdfMd5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capi_kdf_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CapiKdfSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capi_kdf_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CapiKdfSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capi_kdf_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CapiKdfSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capi_kdf_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CapiKdfSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyDerivationAlgorithmProvider, 3791366203, 18033, 17335, 145, 88, 118, 58, 170, 152, 182, 191);
RT_INTERFACE!{interface IKeyDerivationAlgorithmProvider(IKeyDerivationAlgorithmProviderVtbl): IInspectable [IID_IKeyDerivationAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateKey(&self, keyMaterial: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl IKeyDerivationAlgorithmProvider {
    #[inline] pub fn get_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_key(&self, keyMaterial: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateKey)(self.get_abi() as *const _ as *mut _, keyMaterial.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyDerivationAlgorithmProvider: IKeyDerivationAlgorithmProvider}
impl RtActivatable<IKeyDerivationAlgorithmProviderStatics> for KeyDerivationAlgorithmProvider {}
impl KeyDerivationAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<Option<KeyDerivationAlgorithmProvider>> {
        <Self as RtActivatable<IKeyDerivationAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }
}
DEFINE_CLSID!(KeyDerivationAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_KeyDerivationAlgorithmProvider]);
DEFINE_IID!(IID_IKeyDerivationAlgorithmProviderStatics, 170002810, 2588, 17467, 148, 24, 185, 73, 138, 235, 22, 3);
RT_INTERFACE!{static interface IKeyDerivationAlgorithmProviderStatics(IKeyDerivationAlgorithmProviderStaticsVtbl): IInspectable [IID_IKeyDerivationAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut <KeyDerivationAlgorithmProvider as RtType>::Abi) -> HRESULT
}}
impl IKeyDerivationAlgorithmProviderStatics {
    #[inline] pub fn open_algorithm(&self, algorithm: &HStringArg) -> Result<Option<KeyDerivationAlgorithmProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAlgorithm)(self.get_abi() as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(KeyDerivationAlgorithmProvider::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyDerivationParameters, 2079349095, 1147, 19084, 150, 74, 70, 159, 253, 85, 34, 226);
RT_INTERFACE!{interface IKeyDerivationParameters(IKeyDerivationParametersVtbl): IInspectable [IID_IKeyDerivationParameters] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_KdfGenericBinary(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_KdfGenericBinary(&self, value: <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_IterationCount(&self, out: *mut u32) -> HRESULT
}}
impl IKeyDerivationParameters {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_kdf_generic_binary(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_KdfGenericBinary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_kdf_generic_binary(&self, value: &crate::windows::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_KdfGenericBinary)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_iteration_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IterationCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class KeyDerivationParameters: IKeyDerivationParameters}
impl RtActivatable<IKeyDerivationParametersStatics> for KeyDerivationParameters {}
impl RtActivatable<IKeyDerivationParametersStatics2> for KeyDerivationParameters {}
impl KeyDerivationParameters {
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_pbkdf2(pbkdf2Salt: &crate::windows::storage::streams::IBuffer, iterationCount: u32) -> Result<Option<KeyDerivationParameters>> {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_pbkdf2(pbkdf2Salt, iterationCount)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp800108(label: &crate::windows::storage::streams::IBuffer, context: &crate::windows::storage::streams::IBuffer) -> Result<Option<KeyDerivationParameters>> {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_sp800108(label, context)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp80056a(algorithmId: &crate::windows::storage::streams::IBuffer, partyUInfo: &crate::windows::storage::streams::IBuffer, partyVInfo: &crate::windows::storage::streams::IBuffer, suppPubInfo: &crate::windows::storage::streams::IBuffer, suppPrivInfo: &crate::windows::storage::streams::IBuffer) -> Result<Option<KeyDerivationParameters>> {
        <Self as RtActivatable<IKeyDerivationParametersStatics>>::get_activation_factory().build_for_sp80056a(algorithmId, partyUInfo, partyVInfo, suppPubInfo, suppPrivInfo)
    }
    #[inline] pub fn build_for_capi1_kdf(capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm) -> Result<Option<KeyDerivationParameters>> {
        <Self as RtActivatable<IKeyDerivationParametersStatics2>>::get_activation_factory().build_for_capi1_kdf(capi1KdfTargetAlgorithm)
    }
}
DEFINE_CLSID!(KeyDerivationParameters(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,75,101,121,68,101,114,105,118,97,116,105,111,110,80,97,114,97,109,101,116,101,114,115,0]) [CLSID_KeyDerivationParameters]);
DEFINE_IID!(IID_IKeyDerivationParameters2, 3443615441, 16766, 20300, 182, 102, 192, 216, 121, 243, 248, 224);
RT_INTERFACE!{interface IKeyDerivationParameters2(IKeyDerivationParameters2Vtbl): IInspectable [IID_IKeyDerivationParameters2] {
    fn get_Capi1KdfTargetAlgorithm(&self, out: *mut Capi1KdfTargetAlgorithm) -> HRESULT,
    fn put_Capi1KdfTargetAlgorithm(&self, value: Capi1KdfTargetAlgorithm) -> HRESULT
}}
impl IKeyDerivationParameters2 {
    #[inline] pub fn get_capi1_kdf_target_algorithm(&self) -> Result<Capi1KdfTargetAlgorithm> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Capi1KdfTargetAlgorithm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_capi1_kdf_target_algorithm(&self, value: Capi1KdfTargetAlgorithm) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Capi1KdfTargetAlgorithm)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyDerivationParametersStatics, 3935707070, 62335, 16710, 157, 254, 164, 86, 241, 115, 95, 75);
RT_INTERFACE!{static interface IKeyDerivationParametersStatics(IKeyDerivationParametersStaticsVtbl): IInspectable [IID_IKeyDerivationParametersStatics] {
    #[cfg(feature="windows-storage")] fn BuildForPbkdf2(&self, pbkdf2Salt: <crate::windows::storage::streams::IBuffer as RtType>::Abi, iterationCount: u32, out: *mut <KeyDerivationParameters as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn BuildForSP800108(&self, label: <crate::windows::storage::streams::IBuffer as RtType>::Abi, context: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <KeyDerivationParameters as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn BuildForSP80056a(&self, algorithmId: <crate::windows::storage::streams::IBuffer as RtType>::Abi, partyUInfo: <crate::windows::storage::streams::IBuffer as RtType>::Abi, partyVInfo: <crate::windows::storage::streams::IBuffer as RtType>::Abi, suppPubInfo: <crate::windows::storage::streams::IBuffer as RtType>::Abi, suppPrivInfo: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <KeyDerivationParameters as RtType>::Abi) -> HRESULT
}}
impl IKeyDerivationParametersStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_pbkdf2(&self, pbkdf2Salt: &crate::windows::storage::streams::IBuffer, iterationCount: u32) -> Result<Option<KeyDerivationParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildForPbkdf2)(self.get_abi() as *const _ as *mut _, pbkdf2Salt.get_abi() as *const _ as *mut _, iterationCount, &mut out);
        if hr == S_OK { Ok(KeyDerivationParameters::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp800108(&self, label: &crate::windows::storage::streams::IBuffer, context: &crate::windows::storage::streams::IBuffer) -> Result<Option<KeyDerivationParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildForSP800108)(self.get_abi() as *const _ as *mut _, label.get_abi() as *const _ as *mut _, context.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(KeyDerivationParameters::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn build_for_sp80056a(&self, algorithmId: &crate::windows::storage::streams::IBuffer, partyUInfo: &crate::windows::storage::streams::IBuffer, partyVInfo: &crate::windows::storage::streams::IBuffer, suppPubInfo: &crate::windows::storage::streams::IBuffer, suppPrivInfo: &crate::windows::storage::streams::IBuffer) -> Result<Option<KeyDerivationParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildForSP80056a)(self.get_abi() as *const _ as *mut _, algorithmId.get_abi() as *const _ as *mut _, partyUInfo.get_abi() as *const _ as *mut _, partyVInfo.get_abi() as *const _ as *mut _, suppPubInfo.get_abi() as *const _ as *mut _, suppPrivInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(KeyDerivationParameters::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKeyDerivationParametersStatics2, 2776120789, 22755, 20219, 178, 131, 161, 101, 49, 38, 225, 190);
RT_INTERFACE!{static interface IKeyDerivationParametersStatics2(IKeyDerivationParametersStatics2Vtbl): IInspectable [IID_IKeyDerivationParametersStatics2] {
    fn BuildForCapi1Kdf(&self, capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm, out: *mut <KeyDerivationParameters as RtType>::Abi) -> HRESULT
}}
impl IKeyDerivationParametersStatics2 {
    #[inline] pub fn build_for_capi1_kdf(&self, capi1KdfTargetAlgorithm: Capi1KdfTargetAlgorithm) -> Result<Option<KeyDerivationParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().BuildForCapi1Kdf)(self.get_abi() as *const _ as *mut _, capi1KdfTargetAlgorithm, &mut out);
        if hr == S_OK { Ok(KeyDerivationParameters::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class MacAlgorithmNames}
impl RtActivatable<IMacAlgorithmNamesStatics> for MacAlgorithmNames {}
impl MacAlgorithmNames {
    #[inline] pub fn get_hmac_md5() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_md5()
    }
    #[inline] pub fn get_hmac_sha1() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha1()
    }
    #[inline] pub fn get_hmac_sha256() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha256()
    }
    #[inline] pub fn get_hmac_sha384() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha384()
    }
    #[inline] pub fn get_hmac_sha512() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_hmac_sha512()
    }
    #[inline] pub fn get_aes_cmac() -> Result<HString> {
        <Self as RtActivatable<IMacAlgorithmNamesStatics>>::get_activation_factory().get_aes_cmac()
    }
}
DEFINE_CLSID!(MacAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,77,97,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_MacAlgorithmNames]);
DEFINE_IID!(IID_IMacAlgorithmNamesStatics, 1094788728, 64286, 17316, 137, 94, 169, 2, 110, 67, 144, 163);
RT_INTERFACE!{static interface IMacAlgorithmNamesStatics(IMacAlgorithmNamesStaticsVtbl): IInspectable [IID_IMacAlgorithmNamesStatics] {
    fn get_HmacMd5(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha1(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha256(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha384(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HmacSha512(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCmac(&self, out: *mut HSTRING) -> HRESULT
}}
impl IMacAlgorithmNamesStatics {
    #[inline] pub fn get_hmac_md5(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HmacMd5)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hmac_sha1(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HmacSha1)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hmac_sha256(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HmacSha256)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hmac_sha384(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HmacSha384)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hmac_sha512(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_HmacSha512)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_cmac(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesCmac)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMacAlgorithmProvider, 1245693379, 7357, 16846, 160, 146, 170, 11, 197, 210, 210, 245);
RT_INTERFACE!{interface IMacAlgorithmProvider(IMacAlgorithmProviderVtbl): IInspectable [IID_IMacAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MacLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateKey(&self, keyMaterial: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl IMacAlgorithmProvider {
    #[inline] pub fn get_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mac_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MacLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_key(&self, keyMaterial: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateKey)(self.get_abi() as *const _ as *mut _, keyMaterial.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MacAlgorithmProvider: IMacAlgorithmProvider}
impl RtActivatable<IMacAlgorithmProviderStatics> for MacAlgorithmProvider {}
impl MacAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<Option<MacAlgorithmProvider>> {
        <Self as RtActivatable<IMacAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }
}
DEFINE_CLSID!(MacAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,77,97,99,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_MacAlgorithmProvider]);
DEFINE_IID!(IID_IMacAlgorithmProvider2, 1839409685, 55601, 17133, 142, 126, 195, 1, 202, 238, 17, 156);
RT_INTERFACE!{interface IMacAlgorithmProvider2(IMacAlgorithmProvider2Vtbl): IInspectable [IID_IMacAlgorithmProvider2] {
    #[cfg(feature="windows-storage")] fn CreateHash(&self, keyMaterial: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicHash as RtType>::Abi) -> HRESULT
}}
impl IMacAlgorithmProvider2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn create_hash(&self, keyMaterial: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicHash>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateHash)(self.get_abi() as *const _ as *mut _, keyMaterial.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicHash::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMacAlgorithmProviderStatics, 3384656199, 52343, 19952, 158, 78, 185, 33, 224, 128, 100, 76);
RT_INTERFACE!{static interface IMacAlgorithmProviderStatics(IMacAlgorithmProviderStaticsVtbl): IInspectable [IID_IMacAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut <MacAlgorithmProvider as RtType>::Abi) -> HRESULT
}}
impl IMacAlgorithmProviderStatics {
    #[inline] pub fn open_algorithm(&self, algorithm: &HStringArg) -> Result<Option<MacAlgorithmProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAlgorithm)(self.get_abi() as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(MacAlgorithmProvider::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class PersistedKeyProvider}
impl RtActivatable<IPersistedKeyProviderStatics> for PersistedKeyProvider {}
impl PersistedKeyProvider {
    #[inline] pub fn open_key_pair_from_certificate_async(certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<foundation::IAsyncOperation<CryptographicKey>> {
        <Self as RtActivatable<IPersistedKeyProviderStatics>>::get_activation_factory().open_key_pair_from_certificate_async(certificate, hashAlgorithmName, padding)
    }
    #[inline] pub fn open_public_key_from_certificate(certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<Option<CryptographicKey>> {
        <Self as RtActivatable<IPersistedKeyProviderStatics>>::get_activation_factory().open_public_key_from_certificate(certificate, hashAlgorithmName, padding)
    }
}
DEFINE_CLSID!(PersistedKeyProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,80,101,114,115,105,115,116,101,100,75,101,121,80,114,111,118,105,100,101,114,0]) [CLSID_PersistedKeyProvider]);
DEFINE_IID!(IID_IPersistedKeyProviderStatics, 1999063060, 55764, 19701, 182, 104, 224, 69, 125, 243, 8, 148);
RT_INTERFACE!{static interface IPersistedKeyProviderStatics(IPersistedKeyProviderStaticsVtbl): IInspectable [IID_IPersistedKeyProviderStatics] {
    fn OpenKeyPairFromCertificateAsync(&self, certificate: <super::certificates::Certificate as RtType>::Abi, hashAlgorithmName: HSTRING, padding: CryptographicPadding, out: *mut <foundation::IAsyncOperation<CryptographicKey> as RtType>::Abi) -> HRESULT,
    fn OpenPublicKeyFromCertificate(&self, certificate: <super::certificates::Certificate as RtType>::Abi, hashAlgorithmName: HSTRING, padding: CryptographicPadding, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl IPersistedKeyProviderStatics {
    #[inline] pub fn open_key_pair_from_certificate_async(&self, certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<foundation::IAsyncOperation<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenKeyPairFromCertificateAsync)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _, hashAlgorithmName.get(), padding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_public_key_from_certificate(&self, certificate: &super::certificates::Certificate, hashAlgorithmName: &HStringArg, padding: CryptographicPadding) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenPublicKeyFromCertificate)(self.get_abi() as *const _ as *mut _, certificate.get_abi() as *const _ as *mut _, hashAlgorithmName.get(), padding, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class SymmetricAlgorithmNames}
impl RtActivatable<ISymmetricAlgorithmNamesStatics> for SymmetricAlgorithmNames {}
impl SymmetricAlgorithmNames {
    #[inline] pub fn get_des_cbc() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_cbc()
    }
    #[inline] pub fn get_des_ecb() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_ecb()
    }
    #[inline] pub fn get_triple_des_cbc() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_cbc()
    }
    #[inline] pub fn get_triple_des_ecb() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_ecb()
    }
    #[inline] pub fn get_rc2_cbc() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_cbc()
    }
    #[inline] pub fn get_rc2_ecb() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_ecb()
    }
    #[inline] pub fn get_aes_cbc() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_cbc()
    }
    #[inline] pub fn get_aes_ecb() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ecb()
    }
    #[inline] pub fn get_aes_gcm() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_gcm()
    }
    #[inline] pub fn get_aes_ccm() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ccm()
    }
    #[inline] pub fn get_aes_cbc_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_cbc_pkcs7()
    }
    #[inline] pub fn get_aes_ecb_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_aes_ecb_pkcs7()
    }
    #[inline] pub fn get_des_cbc_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_cbc_pkcs7()
    }
    #[inline] pub fn get_des_ecb_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_des_ecb_pkcs7()
    }
    #[inline] pub fn get_triple_des_cbc_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_cbc_pkcs7()
    }
    #[inline] pub fn get_triple_des_ecb_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_triple_des_ecb_pkcs7()
    }
    #[inline] pub fn get_rc2_cbc_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_cbc_pkcs7()
    }
    #[inline] pub fn get_rc2_ecb_pkcs7() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc2_ecb_pkcs7()
    }
    #[inline] pub fn get_rc4() -> Result<HString> {
        <Self as RtActivatable<ISymmetricAlgorithmNamesStatics>>::get_activation_factory().get_rc4()
    }
}
DEFINE_CLSID!(SymmetricAlgorithmNames(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,83,121,109,109,101,116,114,105,99,65,108,103,111,114,105,116,104,109,78,97,109,101,115,0]) [CLSID_SymmetricAlgorithmNames]);
DEFINE_IID!(IID_ISymmetricAlgorithmNamesStatics, 1752199803, 51606, 20142, 132, 215, 121, 178, 174, 183, 59, 156);
RT_INTERFACE!{static interface ISymmetricAlgorithmNamesStatics(ISymmetricAlgorithmNamesStaticsVtbl): IInspectable [IID_ISymmetricAlgorithmNamesStatics] {
    fn get_DesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2Cbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2Ecb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCbc(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesEcb(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesGcm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCcm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesCbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TripleDesEcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2CbcPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc2EcbPkcs7(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rc4(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISymmetricAlgorithmNamesStatics {
    #[inline] pub fn get_des_cbc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DesCbc)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_des_ecb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DesEcb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triple_des_cbc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TripleDesCbc)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triple_des_ecb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TripleDesEcb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rc2_cbc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rc2Cbc)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rc2_ecb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rc2Ecb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_cbc(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesCbc)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_ecb(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesEcb)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_gcm(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesGcm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_ccm(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesCcm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_cbc_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesCbcPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_aes_ecb_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AesEcbPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_des_cbc_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DesCbcPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_des_ecb_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DesEcbPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triple_des_cbc_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TripleDesCbcPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triple_des_ecb_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TripleDesEcbPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rc2_cbc_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rc2CbcPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rc2_ecb_pkcs7(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rc2EcbPkcs7)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rc4(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Rc4)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISymmetricKeyAlgorithmProvider, 1031686707, 15312, 18690, 138, 200, 71, 13, 80, 210, 19, 118);
RT_INTERFACE!{interface ISymmetricKeyAlgorithmProvider(ISymmetricKeyAlgorithmProviderVtbl): IInspectable [IID_ISymmetricKeyAlgorithmProvider] {
    fn get_AlgorithmName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_BlockLength(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateSymmetricKey(&self, keyMaterial: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <CryptographicKey as RtType>::Abi) -> HRESULT
}}
impl ISymmetricKeyAlgorithmProvider {
    #[inline] pub fn get_algorithm_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_AlgorithmName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_block_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_BlockLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_symmetric_key(&self, keyMaterial: &crate::windows::storage::streams::IBuffer) -> Result<Option<CryptographicKey>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateSymmetricKey)(self.get_abi() as *const _ as *mut _, keyMaterial.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(CryptographicKey::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SymmetricKeyAlgorithmProvider: ISymmetricKeyAlgorithmProvider}
impl RtActivatable<ISymmetricKeyAlgorithmProviderStatics> for SymmetricKeyAlgorithmProvider {}
impl SymmetricKeyAlgorithmProvider {
    #[inline] pub fn open_algorithm(algorithm: &HStringArg) -> Result<Option<SymmetricKeyAlgorithmProvider>> {
        <Self as RtActivatable<ISymmetricKeyAlgorithmProviderStatics>>::get_activation_factory().open_algorithm(algorithm)
    }
}
DEFINE_CLSID!(SymmetricKeyAlgorithmProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,67,111,114,101,46,83,121,109,109,101,116,114,105,99,75,101,121,65,108,103,111,114,105,116,104,109,80,114,111,118,105,100,101,114,0]) [CLSID_SymmetricKeyAlgorithmProvider]);
DEFINE_IID!(IID_ISymmetricKeyAlgorithmProviderStatics, 2369463078, 7991, 18719, 182, 14, 245, 67, 27, 38, 180, 131);
RT_INTERFACE!{static interface ISymmetricKeyAlgorithmProviderStatics(ISymmetricKeyAlgorithmProviderStaticsVtbl): IInspectable [IID_ISymmetricKeyAlgorithmProviderStatics] {
    fn OpenAlgorithm(&self, algorithm: HSTRING, out: *mut <SymmetricKeyAlgorithmProvider as RtType>::Abi) -> HRESULT
}}
impl ISymmetricKeyAlgorithmProviderStatics {
    #[inline] pub fn open_algorithm(&self, algorithm: &HStringArg) -> Result<Option<SymmetricKeyAlgorithmProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().OpenAlgorithm)(self.get_abi() as *const _ as *mut _, algorithm.get(), &mut out);
        if hr == S_OK { Ok(SymmetricKeyAlgorithmProvider::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Security.Cryptography.Core
pub mod dataprotection { // Windows.Security.Cryptography.DataProtection
use crate::prelude::*;
DEFINE_IID!(IID_IDataProtectionProvider, 157522248, 60706, 17008, 189, 28, 109, 114, 192, 15, 135, 135);
RT_INTERFACE!{interface IDataProtectionProvider(IDataProtectionProviderVtbl): IInspectable [IID_IDataProtectionProvider] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, data: <crate::windows::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ProtectStreamAsync(&self, src: <crate::windows::storage::streams::IInputStream as RtType>::Abi, dest: <crate::windows::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectStreamAsync(&self, src: <crate::windows::storage::streams::IInputStream as RtType>::Abi, dest: <crate::windows::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IDataProtectionProvider {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(&self, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(&self, data: &crate::windows::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_stream_async(&self, src: &crate::windows::storage::streams::IInputStream, dest: &crate::windows::storage::streams::IOutputStream) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectStreamAsync)(self.get_abi() as *const _ as *mut _, src.get_abi() as *const _ as *mut _, dest.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_stream_async(&self, src: &crate::windows::storage::streams::IInputStream, dest: &crate::windows::storage::streams::IOutputStream) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectStreamAsync)(self.get_abi() as *const _ as *mut _, src.get_abi() as *const _ as *mut _, dest.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataProtectionProvider: IDataProtectionProvider}
impl RtActivatable<IDataProtectionProviderFactory> for DataProtectionProvider {}
impl RtActivatable<IActivationFactory> for DataProtectionProvider {}
impl DataProtectionProvider {
    #[inline] pub fn create_overload_explicit(protectionDescriptor: &HStringArg) -> Result<DataProtectionProvider> {
        <Self as RtActivatable<IDataProtectionProviderFactory>>::get_activation_factory().create_overload_explicit(protectionDescriptor)
    }
}
DEFINE_CLSID!(DataProtectionProvider(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,67,114,121,112,116,111,103,114,97,112,104,121,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,80,114,111,118,105,100,101,114,0]) [CLSID_DataProtectionProvider]);
DEFINE_IID!(IID_IDataProtectionProviderFactory, 2918399404, 18738, 19679, 172, 65, 114, 20, 51, 53, 20, 202);
RT_INTERFACE!{static interface IDataProtectionProviderFactory(IDataProtectionProviderFactoryVtbl): IInspectable [IID_IDataProtectionProviderFactory] {
    fn CreateOverloadExplicit(&self, protectionDescriptor: HSTRING, out: *mut <DataProtectionProvider as RtType>::Abi) -> HRESULT
}}
impl IDataProtectionProviderFactory {
    #[inline] pub fn create_overload_explicit(&self, protectionDescriptor: &HStringArg) -> Result<DataProtectionProvider> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateOverloadExplicit)(self.get_abi() as *const _ as *mut _, protectionDescriptor.get(), &mut out);
        if hr == S_OK { Ok(DataProtectionProvider::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Security.Cryptography.DataProtection
} // Windows.Security.Cryptography
pub mod enterprisedata { // Windows.Security.EnterpriseData
use crate::prelude::*;
DEFINE_IID!(IID_IBufferProtectUnprotectResult, 1201233628, 27884, 20026, 178, 81, 158, 116, 133, 215, 158, 122);
RT_INTERFACE!{interface IBufferProtectUnprotectResult(IBufferProtectUnprotectResultVtbl): IInspectable [IID_IBufferProtectUnprotectResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Buffer(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_ProtectionInfo(&self, out: *mut <DataProtectionInfo as RtType>::Abi) -> HRESULT
}}
impl IBufferProtectUnprotectResult {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_buffer(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Buffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_protection_info(&self) -> Result<Option<DataProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProtectionInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DataProtectionInfo::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BufferProtectUnprotectResult: IBufferProtectUnprotectResult}
DEFINE_IID!(IID_IDataProtectionInfo, 2216734913, 24113, 17413, 149, 64, 63, 148, 58, 240, 203, 38);
RT_INTERFACE!{interface IDataProtectionInfo(IDataProtectionInfoVtbl): IInspectable [IID_IDataProtectionInfo] {
    fn get_Status(&self, out: *mut DataProtectionStatus) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IDataProtectionInfo {
    #[inline] pub fn get_status(&self) -> Result<DataProtectionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_identity(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataProtectionInfo: IDataProtectionInfo}
RT_CLASS!{static class DataProtectionManager}
impl RtActivatable<IDataProtectionManagerStatics> for DataProtectionManager {}
impl DataProtectionManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(data: &super::super::storage::streams::IBuffer, identity: &HStringArg) -> Result<foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().protect_async(data, identity)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(data: &super::super::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<BufferProtectUnprotectResult>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().unprotect_async(data)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_stream_async(unprotectedStream: &super::super::storage::streams::IInputStream, identity: &HStringArg, protectedStream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().protect_stream_async(unprotectedStream, identity, protectedStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_stream_async(protectedStream: &super::super::storage::streams::IInputStream, unprotectedStream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().unprotect_stream_async(protectedStream, unprotectedStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(protectedData: &super::super::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().get_protection_info_async(protectedData)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_stream_protection_info_async(protectedStream: &super::super::storage::streams::IInputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> {
        <Self as RtActivatable<IDataProtectionManagerStatics>>::get_activation_factory().get_stream_protection_info_async(protectedStream)
    }
}
DEFINE_CLSID!(DataProtectionManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,68,97,116,97,80,114,111,116,101,99,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_DataProtectionManager]);
DEFINE_IID!(IID_IDataProtectionManagerStatics, 3054803828, 37188, 20196, 138, 138, 48, 181, 243, 97, 67, 14);
RT_INTERFACE!{static interface IDataProtectionManagerStatics(IDataProtectionManagerStaticsVtbl): IInspectable [IID_IDataProtectionManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, data: <super::super::storage::streams::IBuffer as RtType>::Abi, identity: HSTRING, out: *mut <foundation::IAsyncOperation<BufferProtectUnprotectResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, data: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BufferProtectUnprotectResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn ProtectStreamAsync(&self, unprotectedStream: <super::super::storage::streams::IInputStream as RtType>::Abi, identity: HSTRING, protectedStream: <super::super::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<DataProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectStreamAsync(&self, protectedStream: <super::super::storage::streams::IInputStream as RtType>::Abi, unprotectedStream: <super::super::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<DataProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetProtectionInfoAsync(&self, protectedData: <super::super::storage::streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperation<DataProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetStreamProtectionInfoAsync(&self, protectedStream: <super::super::storage::streams::IInputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<DataProtectionInfo> as RtType>::Abi) -> HRESULT
}}
impl IDataProtectionManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(&self, data: &super::super::storage::streams::IBuffer, identity: &HStringArg) -> Result<foundation::IAsyncOperation<BufferProtectUnprotectResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(&self, data: &super::super::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<BufferProtectUnprotectResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectAsync)(self.get_abi() as *const _ as *mut _, data.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_stream_async(&self, unprotectedStream: &super::super::storage::streams::IInputStream, identity: &HStringArg, protectedStream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectStreamAsync)(self.get_abi() as *const _ as *mut _, unprotectedStream.get_abi() as *const _ as *mut _, identity.get(), protectedStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_stream_async(&self, protectedStream: &super::super::storage::streams::IInputStream, unprotectedStream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectStreamAsync)(self.get_abi() as *const _ as *mut _, protectedStream.get_abi() as *const _ as *mut _, unprotectedStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(&self, protectedData: &super::super::storage::streams::IBuffer) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetProtectionInfoAsync)(self.get_abi() as *const _ as *mut _, protectedData.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_stream_protection_info_async(&self, protectedStream: &super::super::storage::streams::IInputStream) -> Result<foundation::IAsyncOperation<DataProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetStreamProtectionInfoAsync)(self.get_abi() as *const _ as *mut _, protectedStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum DataProtectionStatus: i32 {
    ProtectedToOtherIdentity = 0, Protected = 1, Revoked = 2, Unprotected = 3, LicenseExpired = 4, AccessSuspended = 5,
}}
RT_ENUM! { enum EnforcementLevel: i32 {
    NoProtection = 0, Silent = 1, Override = 2, Block = 3,
}}
DEFINE_IID!(IID_IFileProtectionInfo, 1323918470, 5246, 19920, 143, 175, 82, 83, 237, 145, 173, 12);
RT_INTERFACE!{interface IFileProtectionInfo(IFileProtectionInfoVtbl): IInspectable [IID_IFileProtectionInfo] {
    fn get_Status(&self, out: *mut FileProtectionStatus) -> HRESULT,
    fn get_IsRoamable(&self, out: *mut bool) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IFileProtectionInfo {
    #[inline] pub fn get_status(&self) -> Result<FileProtectionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_roamable(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsRoamable)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_identity(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileProtectionInfo: IFileProtectionInfo}
DEFINE_IID!(IID_IFileProtectionInfo2, 2182232652, 21882, 18829, 142, 148, 148, 76, 213, 131, 100, 50);
RT_INTERFACE!{interface IFileProtectionInfo2(IFileProtectionInfo2Vtbl): IInspectable [IID_IFileProtectionInfo2] {
    fn get_IsProtectWhileOpenSupported(&self, out: *mut bool) -> HRESULT
}}
impl IFileProtectionInfo2 {
    #[inline] pub fn get_is_protect_while_open_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsProtectWhileOpenSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class FileProtectionManager}
impl RtActivatable<IFileProtectionManagerStatics> for FileProtectionManager {}
impl RtActivatable<IFileProtectionManagerStatics2> for FileProtectionManager {}
impl RtActivatable<IFileProtectionManagerStatics3> for FileProtectionManager {}
impl FileProtectionManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().protect_async(target, identity)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(source: &super::super::storage::IStorageItem, target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().copy_protection_async(source, target)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(source: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().get_protection_info_async(source)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_async(protectedFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().save_file_as_container_async(protectedFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_async(containerFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().load_file_from_container_async(containerFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_async(containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().load_file_from_container_with_target_async(containerFile, target)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_protected_and_open_async(parentFolder: &super::super::storage::IStorageFolder, desiredName: &HStringArg, identity: &HStringArg, collisionOption: super::super::storage::CreationCollisionOption) -> Result<foundation::IAsyncOperation<ProtectedFileCreateResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics>>::get_activation_factory().create_protected_and_open_async(parentFolder, desiredName, identity, collisionOption)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn is_container_async(file: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().is_container_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_and_name_collision_option_async(containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem, collisionOption: super::super::storage::NameCollisionOption) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().load_file_from_container_with_target_and_name_collision_option_async(containerFile, target, collisionOption)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_with_sharing_async(protectedFile: &super::super::storage::IStorageFile, sharedWithIdentities: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<ProtectedContainerExportResult>> {
        <Self as RtActivatable<IFileProtectionManagerStatics2>>::get_activation_factory().save_file_as_container_with_sharing_async(protectedFile, sharedWithIdentities)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> {
        <Self as RtActivatable<IFileProtectionManagerStatics3>>::get_activation_factory().unprotect_async(target)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_with_options_async(target: &super::super::storage::IStorageItem, options: &FileUnprotectOptions) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> {
        <Self as RtActivatable<IFileProtectionManagerStatics3>>::get_activation_factory().unprotect_with_options_async(target, options)
    }
}
DEFINE_CLSID!(FileProtectionManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,80,114,111,116,101,99,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_FileProtectionManager]);
DEFINE_IID!(IID_IFileProtectionManagerStatics, 1481047195, 58899, 17003, 187, 56, 136, 203, 161, 220, 154, 219);
RT_INTERFACE!{static interface IFileProtectionManagerStatics(IFileProtectionManagerStaticsVtbl): IInspectable [IID_IFileProtectionManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, target: <super::super::storage::IStorageItem as RtType>::Abi, identity: HSTRING, out: *mut <foundation::IAsyncOperation<FileProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyProtectionAsync(&self, source: <super::super::storage::IStorageItem as RtType>::Abi, target: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetProtectionInfoAsync(&self, source: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<FileProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveFileAsContainerAsync(&self, protectedFile: <super::super::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectedContainerExportResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerAsync(&self, containerFile: <super::super::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectedContainerImportResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerWithTargetAsync(&self, containerFile: <super::super::storage::IStorageFile as RtType>::Abi, target: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectedContainerImportResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateProtectedAndOpenAsync(&self, parentFolder: <super::super::storage::IStorageFolder as RtType>::Abi, desiredName: HSTRING, identity: HSTRING, collisionOption: super::super::storage::CreationCollisionOption, out: *mut <foundation::IAsyncOperation<ProtectedFileCreateResult> as RtType>::Abi) -> HRESULT
}}
impl IFileProtectionManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(&self, target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectAsync)(self.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(&self, source: &super::super::storage::IStorageItem, target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CopyProtectionAsync)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_protection_info_async(&self, source: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetProtectionInfoAsync)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_async(&self, protectedFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<ProtectedContainerExportResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SaveFileAsContainerAsync)(self.get_abi() as *const _ as *mut _, protectedFile.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_async(&self, containerFile: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFileFromContainerAsync)(self.get_abi() as *const _ as *mut _, containerFile.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_async(&self, containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFileFromContainerWithTargetAsync)(self.get_abi() as *const _ as *mut _, containerFile.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_protected_and_open_async(&self, parentFolder: &super::super::storage::IStorageFolder, desiredName: &HStringArg, identity: &HStringArg, collisionOption: super::super::storage::CreationCollisionOption) -> Result<foundation::IAsyncOperation<ProtectedFileCreateResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateProtectedAndOpenAsync)(self.get_abi() as *const _ as *mut _, parentFolder.get_abi() as *const _ as *mut _, desiredName.get(), identity.get(), collisionOption, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileProtectionManagerStatics2, 2211620677, 1155, 16811, 178, 213, 188, 127, 35, 215, 78, 187);
RT_INTERFACE!{static interface IFileProtectionManagerStatics2(IFileProtectionManagerStatics2Vtbl): IInspectable [IID_IFileProtectionManagerStatics2] {
    #[cfg(feature="windows-storage")] fn IsContainerAsync(&self, file: <super::super::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFileFromContainerWithTargetAndNameCollisionOptionAsync(&self, containerFile: <super::super::storage::IStorageFile as RtType>::Abi, target: <super::super::storage::IStorageItem as RtType>::Abi, collisionOption: super::super::storage::NameCollisionOption, out: *mut <foundation::IAsyncOperation<ProtectedContainerImportResult> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveFileAsContainerWithSharingAsync(&self, protectedFile: <super::super::storage::IStorageFile as RtType>::Abi, sharedWithIdentities: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectedContainerExportResult> as RtType>::Abi) -> HRESULT
}}
impl IFileProtectionManagerStatics2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn is_container_async(&self, file: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().IsContainerAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_file_from_container_with_target_and_name_collision_option_async(&self, containerFile: &super::super::storage::IStorageFile, target: &super::super::storage::IStorageItem, collisionOption: super::super::storage::NameCollisionOption) -> Result<foundation::IAsyncOperation<ProtectedContainerImportResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().LoadFileFromContainerWithTargetAndNameCollisionOptionAsync)(self.get_abi() as *const _ as *mut _, containerFile.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, collisionOption, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_file_as_container_with_sharing_async(&self, protectedFile: &super::super::storage::IStorageFile, sharedWithIdentities: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<ProtectedContainerExportResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SaveFileAsContainerWithSharingAsync)(self.get_abi() as *const _ as *mut _, protectedFile.get_abi() as *const _ as *mut _, sharedWithIdentities.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileProtectionManagerStatics3, 1763214490, 25167, 18134, 178, 65, 233, 205, 95, 223, 62, 63);
RT_INTERFACE!{static interface IFileProtectionManagerStatics3(IFileProtectionManagerStatics3Vtbl): IInspectable [IID_IFileProtectionManagerStatics3] {
    #[cfg(feature="windows-storage")] fn UnprotectAsync(&self, target: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<FileProtectionInfo> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn UnprotectWithOptionsAsync(&self, target: <super::super::storage::IStorageItem as RtType>::Abi, options: <FileUnprotectOptions as RtType>::Abi, out: *mut <foundation::IAsyncOperation<FileProtectionInfo> as RtType>::Abi) -> HRESULT
}}
impl IFileProtectionManagerStatics3 {
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_async(&self, target: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectAsync)(self.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn unprotect_with_options_async(&self, target: &super::super::storage::IStorageItem, options: &FileUnprotectOptions) -> Result<foundation::IAsyncOperation<FileProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().UnprotectWithOptionsAsync)(self.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, options.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum FileProtectionStatus: i32 {
    Undetermined = 0, Unknown = 0, Unprotected = 1, Revoked = 2, Protected = 3, ProtectedByOtherUser = 4, ProtectedToOtherEnterprise = 5, NotProtectable = 6, ProtectedToOtherIdentity = 7, LicenseExpired = 8, AccessSuspended = 9, FileInUse = 10,
}}
RT_CLASS!{static class FileRevocationManager}
impl RtActivatable<IFileRevocationManagerStatics> for FileRevocationManager {}
impl FileRevocationManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(storageItem: &super::super::storage::IStorageItem, enterpriseIdentity: &HStringArg) -> Result<foundation::IAsyncOperation<FileProtectionStatus>> {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().protect_async(storageItem, enterpriseIdentity)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(sourceStorageItem: &super::super::storage::IStorageItem, targetStorageItem: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().copy_protection_async(sourceStorageItem, targetStorageItem)
    }
    #[inline] pub fn revoke(enterpriseIdentity: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().revoke(enterpriseIdentity)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_status_async(storageItem: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionStatus>> {
        <Self as RtActivatable<IFileRevocationManagerStatics>>::get_activation_factory().get_status_async(storageItem)
    }
}
DEFINE_CLSID!(FileRevocationManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,82,101,118,111,99,97,116,105,111,110,77,97,110,97,103,101,114,0]) [CLSID_FileRevocationManager]);
DEFINE_IID!(IID_IFileRevocationManagerStatics, 627817533, 7261, 16992, 140, 117, 145, 68, 207, 183, 139, 169);
RT_INTERFACE!{static interface IFileRevocationManagerStatics(IFileRevocationManagerStaticsVtbl): IInspectable [IID_IFileRevocationManagerStatics] {
    #[cfg(feature="windows-storage")] fn ProtectAsync(&self, storageItem: <super::super::storage::IStorageItem as RtType>::Abi, enterpriseIdentity: HSTRING, out: *mut <foundation::IAsyncOperation<FileProtectionStatus> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CopyProtectionAsync(&self, sourceStorageItem: <super::super::storage::IStorageItem as RtType>::Abi, targetStorageItem: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn Revoke(&self, enterpriseIdentity: HSTRING) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetStatusAsync(&self, storageItem: <super::super::storage::IStorageItem as RtType>::Abi, out: *mut <foundation::IAsyncOperation<FileProtectionStatus> as RtType>::Abi) -> HRESULT
}}
impl IFileRevocationManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn protect_async(&self, storageItem: &super::super::storage::IStorageItem, enterpriseIdentity: &HStringArg) -> Result<foundation::IAsyncOperation<FileProtectionStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ProtectAsync)(self.get_abi() as *const _ as *mut _, storageItem.get_abi() as *const _ as *mut _, enterpriseIdentity.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_protection_async(&self, sourceStorageItem: &super::super::storage::IStorageItem, targetStorageItem: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CopyProtectionAsync)(self.get_abi() as *const _ as *mut _, sourceStorageItem.get_abi() as *const _ as *mut _, targetStorageItem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn revoke(&self, enterpriseIdentity: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Revoke)(self.get_abi() as *const _ as *mut _, enterpriseIdentity.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_status_async(&self, storageItem: &super::super::storage::IStorageItem) -> Result<foundation::IAsyncOperation<FileProtectionStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetStatusAsync)(self.get_abi() as *const _ as *mut _, storageItem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileUnprotectOptions, 2098402033, 15117, 19928, 161, 248, 30, 197, 56, 34, 226, 243);
RT_INTERFACE!{interface IFileUnprotectOptions(IFileUnprotectOptionsVtbl): IInspectable [IID_IFileUnprotectOptions] {
    fn put_Audit(&self, value: bool) -> HRESULT,
    fn get_Audit(&self, out: *mut bool) -> HRESULT
}}
impl IFileUnprotectOptions {
    #[inline] pub fn set_audit(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Audit)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_audit(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Audit)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class FileUnprotectOptions: IFileUnprotectOptions}
impl RtActivatable<IFileUnprotectOptionsFactory> for FileUnprotectOptions {}
impl FileUnprotectOptions {
    #[inline] pub fn create(audit: bool) -> Result<FileUnprotectOptions> {
        <Self as RtActivatable<IFileUnprotectOptionsFactory>>::get_activation_factory().create(audit)
    }
}
DEFINE_CLSID!(FileUnprotectOptions(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,70,105,108,101,85,110,112,114,111,116,101,99,116,79,112,116,105,111,110,115,0]) [CLSID_FileUnprotectOptions]);
DEFINE_IID!(IID_IFileUnprotectOptionsFactory, 1370403740, 55948, 19519, 155, 251, 203, 115, 167, 204, 224, 221);
RT_INTERFACE!{static interface IFileUnprotectOptionsFactory(IFileUnprotectOptionsFactoryVtbl): IInspectable [IID_IFileUnprotectOptionsFactory] {
    fn Create(&self, audit: bool, out: *mut <FileUnprotectOptions as RtType>::Abi) -> HRESULT
}}
impl IFileUnprotectOptionsFactory {
    #[inline] pub fn create(&self, audit: bool) -> Result<FileUnprotectOptions> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, audit, &mut out);
        if hr == S_OK { Ok(FileUnprotectOptions::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProtectedAccessResumedEventArgs, 2890779225, 23936, 20117, 140, 95, 133, 57, 69, 14, 235, 224);
RT_INTERFACE!{interface IProtectedAccessResumedEventArgs(IProtectedAccessResumedEventArgsVtbl): IInspectable [IID_IProtectedAccessResumedEventArgs] {
    fn get_Identities(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IProtectedAccessResumedEventArgs {
    #[inline] pub fn get_identities(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedAccessResumedEventArgs: IProtectedAccessResumedEventArgs}
DEFINE_IID!(IID_IProtectedAccessSuspendingEventArgs, 1973523424, 41796, 17055, 185, 117, 4, 252, 31, 136, 193, 133);
RT_INTERFACE!{interface IProtectedAccessSuspendingEventArgs(IProtectedAccessSuspendingEventArgsVtbl): IInspectable [IID_IProtectedAccessSuspendingEventArgs] {
    fn get_Identities(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IProtectedAccessSuspendingEventArgs {
    #[inline] pub fn get_identities(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Deadline)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedAccessSuspendingEventArgs: IProtectedAccessSuspendingEventArgs}
DEFINE_IID!(IID_IProtectedContainerExportResult, 961081237, 63483, 19266, 175, 176, 223, 112, 180, 21, 67, 193);
RT_INTERFACE!{interface IProtectedContainerExportResult(IProtectedContainerExportResultVtbl): IInspectable [IID_IProtectedContainerExportResult] {
    fn get_Status(&self, out: *mut ProtectedImportExportStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut <super::super::storage::StorageFile as RtType>::Abi) -> HRESULT
}}
impl IProtectedContainerExportResult {
    #[inline] pub fn get_status(&self) -> Result<ProtectedImportExportStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_file(&self) -> Result<Option<super::super::storage::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_File)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::StorageFile::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedContainerExportResult: IProtectedContainerExportResult}
DEFINE_IID!(IID_IProtectedContainerImportResult, 3451355345, 59323, 19738, 147, 57, 52, 220, 65, 20, 159, 155);
RT_INTERFACE!{interface IProtectedContainerImportResult(IProtectedContainerImportResultVtbl): IInspectable [IID_IProtectedContainerImportResult] {
    fn get_Status(&self, out: *mut ProtectedImportExportStatus) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut <super::super::storage::StorageFile as RtType>::Abi) -> HRESULT
}}
impl IProtectedContainerImportResult {
    #[inline] pub fn get_status(&self) -> Result<ProtectedImportExportStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_file(&self) -> Result<Option<super::super::storage::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_File)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::StorageFile::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedContainerImportResult: IProtectedContainerImportResult}
DEFINE_IID!(IID_IProtectedContentRevokedEventArgs, 1667786785, 22713, 18414, 147, 217, 240, 247, 65, 207, 67, 240);
RT_INTERFACE!{interface IProtectedContentRevokedEventArgs(IProtectedContentRevokedEventArgsVtbl): IInspectable [IID_IProtectedContentRevokedEventArgs] {
    fn get_Identities(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IProtectedContentRevokedEventArgs {
    #[inline] pub fn get_identities(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedContentRevokedEventArgs: IProtectedContentRevokedEventArgs}
DEFINE_IID!(IID_IProtectedFileCreateResult, 686026090, 59879, 18947, 159, 83, 189, 177, 97, 114, 105, 155);
RT_INTERFACE!{interface IProtectedFileCreateResult(IProtectedFileCreateResultVtbl): IInspectable [IID_IProtectedFileCreateResult] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_File(&self, out: *mut <super::super::storage::StorageFile as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_Stream(&self, out: *mut <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn get_ProtectionInfo(&self, out: *mut <FileProtectionInfo as RtType>::Abi) -> HRESULT
}}
impl IProtectedFileCreateResult {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_file(&self) -> Result<Option<super::super::storage::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_File)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::StorageFile::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_stream(&self) -> Result<Option<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Stream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_protection_info(&self) -> Result<Option<FileProtectionInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_ProtectionInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(FileProtectionInfo::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectedFileCreateResult: IProtectedFileCreateResult}
RT_ENUM! { enum ProtectedImportExportStatus: i32 {
    Ok = 0, Undetermined = 1, Unprotected = 2, Revoked = 3, NotRoamable = 4, ProtectedToOtherIdentity = 5, LicenseExpired = 6, AccessSuspended = 7,
}}
RT_ENUM! { enum ProtectionPolicyAuditAction: i32 {
    Decrypt = 0, CopyToLocation = 1, SendToRecipient = 2, Other = 3,
}}
DEFINE_IID!(IID_IProtectionPolicyAuditInfo, 1113241572, 65207, 17660, 179, 187, 195, 196, 215, 236, 190, 187);
RT_INTERFACE!{interface IProtectionPolicyAuditInfo(IProtectionPolicyAuditInfoVtbl): IInspectable [IID_IProtectionPolicyAuditInfo] {
    fn put_Action(&self, value: ProtectionPolicyAuditAction) -> HRESULT,
    fn get_Action(&self, out: *mut ProtectionPolicyAuditAction) -> HRESULT,
    fn put_DataDescription(&self, value: HSTRING) -> HRESULT,
    fn get_DataDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SourceDescription(&self, value: HSTRING) -> HRESULT,
    fn get_SourceDescription(&self, out: *mut HSTRING) -> HRESULT,
    fn put_TargetDescription(&self, value: HSTRING) -> HRESULT,
    fn get_TargetDescription(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyAuditInfo {
    #[inline] pub fn set_action(&self, value: ProtectionPolicyAuditAction) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Action)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_action(&self) -> Result<ProtectionPolicyAuditAction> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Action)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_data_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_DataDescription)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_data_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_DataDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_source_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_SourceDescription)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_source_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SourceDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_target_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_TargetDescription)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_target_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TargetDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectionPolicyAuditInfo: IProtectionPolicyAuditInfo}
impl RtActivatable<IProtectionPolicyAuditInfoFactory> for ProtectionPolicyAuditInfo {}
impl ProtectionPolicyAuditInfo {
    #[inline] pub fn create(action: ProtectionPolicyAuditAction, dataDescription: &HStringArg, sourceDescription: &HStringArg, targetDescription: &HStringArg) -> Result<ProtectionPolicyAuditInfo> {
        <Self as RtActivatable<IProtectionPolicyAuditInfoFactory>>::get_activation_factory().create(action, dataDescription, sourceDescription, targetDescription)
    }
    #[inline] pub fn create_with_action_and_data_description(action: ProtectionPolicyAuditAction, dataDescription: &HStringArg) -> Result<ProtectionPolicyAuditInfo> {
        <Self as RtActivatable<IProtectionPolicyAuditInfoFactory>>::get_activation_factory().create_with_action_and_data_description(action, dataDescription)
    }
}
DEFINE_CLSID!(ProtectionPolicyAuditInfo(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,80,114,111,116,101,99,116,105,111,110,80,111,108,105,99,121,65,117,100,105,116,73,110,102,111,0]) [CLSID_ProtectionPolicyAuditInfo]);
DEFINE_IID!(IID_IProtectionPolicyAuditInfoFactory, 2127829003, 37608, 17109, 131, 212, 37, 68, 11, 66, 53, 73);
RT_INTERFACE!{static interface IProtectionPolicyAuditInfoFactory(IProtectionPolicyAuditInfoFactoryVtbl): IInspectable [IID_IProtectionPolicyAuditInfoFactory] {
    fn Create(&self, action: ProtectionPolicyAuditAction, dataDescription: HSTRING, sourceDescription: HSTRING, targetDescription: HSTRING, out: *mut <ProtectionPolicyAuditInfo as RtType>::Abi) -> HRESULT,
    fn CreateWithActionAndDataDescription(&self, action: ProtectionPolicyAuditAction, dataDescription: HSTRING, out: *mut <ProtectionPolicyAuditInfo as RtType>::Abi) -> HRESULT
}}
impl IProtectionPolicyAuditInfoFactory {
    #[inline] pub fn create(&self, action: ProtectionPolicyAuditAction, dataDescription: &HStringArg, sourceDescription: &HStringArg, targetDescription: &HStringArg) -> Result<ProtectionPolicyAuditInfo> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().Create)(self.get_abi() as *const _ as *mut _, action, dataDescription.get(), sourceDescription.get(), targetDescription.get(), &mut out);
        if hr == S_OK { Ok(ProtectionPolicyAuditInfo::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_action_and_data_description(&self, action: ProtectionPolicyAuditAction, dataDescription: &HStringArg) -> Result<ProtectionPolicyAuditInfo> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithActionAndDataDescription)(self.get_abi() as *const _ as *mut _, action, dataDescription.get(), &mut out);
        if hr == S_OK { Ok(ProtectionPolicyAuditInfo::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ProtectionPolicyEvaluationResult: i32 {
    Allowed = 0, Blocked = 1, ConsentRequired = 2,
}}
DEFINE_IID!(IID_IProtectionPolicyManager, 3580902936, 41101, 18406, 162, 64, 153, 52, 215, 22, 94, 181);
RT_INTERFACE!{interface IProtectionPolicyManager(IProtectionPolicyManagerVtbl): IInspectable [IID_IProtectionPolicyManager] {
    fn put_Identity(&self, value: HSTRING) -> HRESULT,
    fn get_Identity(&self, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyManager {
    #[inline] pub fn set_identity(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Identity)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_identity(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Identity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ProtectionPolicyManager: IProtectionPolicyManager}
impl RtActivatable<IProtectionPolicyManagerStatics> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics2> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics3> for ProtectionPolicyManager {}
impl RtActivatable<IProtectionPolicyManagerStatics4> for ProtectionPolicyManager {}
impl ProtectionPolicyManager {
    #[inline] pub fn is_identity_managed(identity: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().is_identity_managed(identity)
    }
    #[inline] pub fn try_apply_process_ui_policy(identity: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().try_apply_process_ui_policy(identity)
    }
    #[inline] pub fn clear_process_ui_policy() -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().clear_process_ui_policy()
    }
    #[inline] pub fn create_current_thread_network_context(identity: &HStringArg) -> Result<Option<ThreadNetworkContext>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().create_current_thread_network_context(identity)
    }
    #[cfg(feature="windows-networking")] #[inline] pub fn get_primary_managed_identity_for_network_endpoint_async(endpointHost: &super::super::networking::HostName) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().get_primary_managed_identity_for_network_endpoint_async(endpointHost)
    }
    #[inline] pub fn revoke_content(identity: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().revoke_content(identity)
    }
    #[inline] pub fn get_for_current_view() -> Result<Option<ProtectionPolicyManager>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn add_protected_access_suspending(handler: &foundation::EventHandler<ProtectedAccessSuspendingEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_access_suspending(handler)
    }
    #[inline] pub fn remove_protected_access_suspending(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_access_suspending(token)
    }
    #[inline] pub fn add_protected_access_resumed(handler: &foundation::EventHandler<ProtectedAccessResumedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_access_resumed(handler)
    }
    #[inline] pub fn remove_protected_access_resumed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_access_resumed(token)
    }
    #[inline] pub fn add_protected_content_revoked(handler: &foundation::EventHandler<ProtectedContentRevokedEventArgs>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().add_protected_content_revoked(handler)
    }
    #[inline] pub fn remove_protected_content_revoked(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().remove_protected_content_revoked(token)
    }
    #[inline] pub fn check_access(sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().check_access(sourceIdentity, targetIdentity)
    }
    #[inline] pub fn request_access_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics>>::get_activation_factory().request_access_async(sourceIdentity, targetIdentity)
    }
    #[inline] pub fn has_content_been_revoked_since(identity: &HStringArg, since: foundation::DateTime) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().has_content_been_revoked_since(identity, since)
    }
    #[inline] pub fn check_access_for_app(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().check_access_for_app(sourceIdentity, appPackageFamilyName)
    }
    #[inline] pub fn request_access_for_app_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().request_access_for_app_async(sourceIdentity, appPackageFamilyName)
    }
    #[inline] pub fn get_enforcement_level(identity: &HStringArg) -> Result<EnforcementLevel> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().get_enforcement_level(identity)
    }
    #[inline] pub fn is_user_decryption_allowed(identity: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().is_user_decryption_allowed(identity)
    }
    #[inline] pub fn is_protection_under_lock_required(identity: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().is_protection_under_lock_required(identity)
    }
    #[inline] pub fn add_policy_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().add_policy_changed(handler)
    }
    #[inline] pub fn remove_policy_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().remove_policy_changed(token)
    }
    #[inline] pub fn get_is_protection_enabled() -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics2>>::get_activation_factory().get_is_protection_enabled()
    }
    #[inline] pub fn request_access_with_auditing_info_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_with_auditing_info_async(sourceIdentity, targetIdentity, auditInfo)
    }
    #[inline] pub fn request_access_with_message_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_with_message_async(sourceIdentity, targetIdentity, auditInfo, messageFromApp)
    }
    #[inline] pub fn request_access_for_app_with_auditing_info_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_for_app_with_auditing_info_async(sourceIdentity, appPackageFamilyName, auditInfo)
    }
    #[inline] pub fn request_access_for_app_with_message_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().request_access_for_app_with_message_async(sourceIdentity, appPackageFamilyName, auditInfo, messageFromApp)
    }
    #[inline] pub fn log_audit_event(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<()> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics3>>::get_activation_factory().log_audit_event(sourceIdentity, targetIdentity, auditInfo)
    }
    #[inline] pub fn is_roamable_protection_enabled(identity: &HStringArg) -> Result<bool> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_roamable_protection_enabled(identity)
    }
    #[inline] pub fn request_access_with_behavior_async(sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_with_behavior_async(sourceIdentity, targetIdentity, auditInfo, messageFromApp, behavior)
    }
    #[inline] pub fn request_access_for_app_with_behavior_async(sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_for_app_with_behavior_async(sourceIdentity, appPackageFamilyName, auditInfo, messageFromApp, behavior)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_async(sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_app_async(sourceItemList, appPackageFamilyName, auditInfo)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_with_message_and_behavior_async(sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_app_with_message_and_behavior_async(sourceItemList, appPackageFamilyName, auditInfo, messageFromApp, behavior)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_async(sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_process_async(sourceItemList, processId, auditInfo)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_with_message_and_behavior_async(sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().request_access_to_files_for_process_with_message_and_behavior_async(sourceItemList, processId, auditInfo, messageFromApp, behavior)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_async(target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_file_protection_required_async(target, identity)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_for_new_file_async(parentFolder: &super::super::storage::IStorageFolder, identity: &HStringArg, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().is_file_protection_required_for_new_file_async(parentFolder, identity, desiredName)
    }
    #[inline] pub fn get_primary_managed_identity() -> Result<HString> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().get_primary_managed_identity()
    }
    #[inline] pub fn get_primary_managed_identity_for_identity(identity: &HStringArg) -> Result<HString> {
        <Self as RtActivatable<IProtectionPolicyManagerStatics4>>::get_activation_factory().get_primary_managed_identity_for_identity(identity)
    }
}
DEFINE_CLSID!(ProtectionPolicyManager(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,110,116,101,114,112,114,105,115,101,68,97,116,97,46,80,114,111,116,101,99,116,105,111,110,80,111,108,105,99,121,77,97,110,97,103,101,114,0]) [CLSID_ProtectionPolicyManager]);
DEFINE_IID!(IID_IProtectionPolicyManager2, 2885112442, 33845, 16767, 153, 182, 81, 190, 175, 54, 88, 136);
RT_INTERFACE!{interface IProtectionPolicyManager2(IProtectionPolicyManager2Vtbl): IInspectable [IID_IProtectionPolicyManager2] {
    fn put_ShowEnterpriseIndicator(&self, value: bool) -> HRESULT,
    fn get_ShowEnterpriseIndicator(&self, out: *mut bool) -> HRESULT
}}
impl IProtectionPolicyManager2 {
    #[inline] pub fn set_show_enterprise_indicator(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_ShowEnterpriseIndicator)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_show_enterprise_indicator(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ShowEnterpriseIndicator)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics, 3233807462, 35901, 19798, 136, 4, 198, 143, 10, 211, 46, 197);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics(IProtectionPolicyManagerStaticsVtbl): IInspectable [IID_IProtectionPolicyManagerStatics] {
    fn IsIdentityManaged(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn TryApplyProcessUIPolicy(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn ClearProcessUIPolicy(&self) -> HRESULT,
    fn CreateCurrentThreadNetworkContext(&self, identity: HSTRING, out: *mut <ThreadNetworkContext as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-networking"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-networking")] fn GetPrimaryManagedIdentityForNetworkEndpointAsync(&self, endpointHost: <super::super::networking::HostName as RtType>::Abi, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn RevokeContent(&self, identity: HSTRING) -> HRESULT,
    fn GetForCurrentView(&self, out: *mut <ProtectionPolicyManager as RtType>::Abi) -> HRESULT,
    fn add_ProtectedAccessSuspending(&self, handler: <foundation::EventHandler<ProtectedAccessSuspendingEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedAccessSuspending(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ProtectedAccessResumed(&self, handler: <foundation::EventHandler<ProtectedAccessResumedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedAccessResumed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_ProtectedContentRevoked(&self, handler: <foundation::EventHandler<ProtectedContentRevokedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ProtectedContentRevoked(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CheckAccess(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, out: *mut ProtectionPolicyEvaluationResult) -> HRESULT,
    fn RequestAccessAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT
}}
impl IProtectionPolicyManagerStatics {
    #[inline] pub fn is_identity_managed(&self, identity: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsIdentityManaged)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_apply_process_ui_policy(&self, identity: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().TryApplyProcessUIPolicy)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn clear_process_ui_policy(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().ClearProcessUIPolicy)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_current_thread_network_context(&self, identity: &HStringArg) -> Result<Option<ThreadNetworkContext>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCurrentThreadNetworkContext)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(ThreadNetworkContext::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-networking")] #[inline] pub fn get_primary_managed_identity_for_network_endpoint_async(&self, endpointHost: &super::super::networking::HostName) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetPrimaryManagedIdentityForNetworkEndpointAsync)(self.get_abi() as *const _ as *mut _, endpointHost.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn revoke_content(&self, identity: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().RevokeContent)(self.get_abi() as *const _ as *mut _, identity.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ProtectionPolicyManager>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ProtectionPolicyManager::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_protected_access_suspending(&self, handler: &foundation::EventHandler<ProtectedAccessSuspendingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_ProtectedAccessSuspending)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_protected_access_suspending(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_ProtectedAccessSuspending)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_protected_access_resumed(&self, handler: &foundation::EventHandler<ProtectedAccessResumedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_ProtectedAccessResumed)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_protected_access_resumed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_ProtectedAccessResumed)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_protected_content_revoked(&self, handler: &foundation::EventHandler<ProtectedContentRevokedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_ProtectedContentRevoked)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_protected_content_revoked(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_ProtectedContentRevoked)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn check_access(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().CheckAccess)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics2, 3062864524, 14816, 17993, 178, 228, 7, 10, 184, 165, 121, 179);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics2(IProtectionPolicyManagerStatics2Vtbl): IInspectable [IID_IProtectionPolicyManagerStatics2] {
    fn HasContentBeenRevokedSince(&self, identity: HSTRING, since: foundation::DateTime, out: *mut bool) -> HRESULT,
    fn CheckAccessForApp(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, out: *mut ProtectionPolicyEvaluationResult) -> HRESULT,
    fn RequestAccessForAppAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn GetEnforcementLevel(&self, identity: HSTRING, out: *mut EnforcementLevel) -> HRESULT,
    fn IsUserDecryptionAllowed(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn IsProtectionUnderLockRequired(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn add_PolicyChanged(&self, handler: <foundation::EventHandler<IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PolicyChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_IsProtectionEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IProtectionPolicyManagerStatics2 {
    #[inline] pub fn has_content_been_revoked_since(&self, identity: &HStringArg, since: foundation::DateTime) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().HasContentBeenRevokedSince)(self.get_abi() as *const _ as *mut _, identity.get(), since, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn check_access_for_app(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<ProtectionPolicyEvaluationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().CheckAccessForApp)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn request_access_for_app_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessForAppAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_enforcement_level(&self, identity: &HStringArg) -> Result<EnforcementLevel> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().GetEnforcementLevel)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_user_decryption_allowed(&self, identity: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsUserDecryptionAllowed)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_protection_under_lock_required(&self, identity: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsProtectionUnderLockRequired)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_policy_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_PolicyChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_policy_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_PolicyChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_protection_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsProtectionEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics3, 1224711820, 27247, 19871, 188, 237, 24, 171, 83, 122, 160, 21);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics3(IProtectionPolicyManagerStatics3Vtbl): IInspectable [IID_IProtectionPolicyManagerStatics3] {
    fn RequestAccessWithAuditingInfoAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn RequestAccessWithMessageAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn RequestAccessForAppWithAuditingInfoAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn RequestAccessForAppWithMessageAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn LogAuditEvent(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi) -> HRESULT
}}
impl IProtectionPolicyManagerStatics3 {
    #[inline] pub fn request_access_with_auditing_info_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessWithAuditingInfoAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_with_message_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessWithMessageAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_for_app_with_auditing_info_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessForAppWithAuditingInfoAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_for_app_with_message_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessForAppWithMessageAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn log_audit_event(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().LogAuditEvent)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IProtectionPolicyManagerStatics4, 548902107, 52413, 18703, 140, 131, 73, 204, 183, 122, 234, 108);
RT_INTERFACE!{static interface IProtectionPolicyManagerStatics4(IProtectionPolicyManagerStatics4Vtbl): IInspectable [IID_IProtectionPolicyManagerStatics4] {
    fn IsRoamableProtectionEnabled(&self, identity: HSTRING, out: *mut bool) -> HRESULT,
    fn RequestAccessWithBehaviorAsync(&self, sourceIdentity: HSTRING, targetIdentity: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    fn RequestAccessForAppWithBehaviorAsync(&self, sourceIdentity: HSTRING, appPackageFamilyName: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForAppAsync(&self, sourceItemList: <foundation::collections::IIterable<super::super::storage::IStorageItem> as RtType>::Abi, appPackageFamilyName: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForAppWithMessageAndBehaviorAsync(&self, sourceItemList: <foundation::collections::IIterable<super::super::storage::IStorageItem> as RtType>::Abi, appPackageFamilyName: HSTRING, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForProcessAsync(&self, sourceItemList: <foundation::collections::IIterable<super::super::storage::IStorageItem> as RtType>::Abi, processId: u32, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-storage")] fn RequestAccessToFilesForProcessWithMessageAndBehaviorAsync(&self, sourceItemList: <foundation::collections::IIterable<super::super::storage::IStorageItem> as RtType>::Abi, processId: u32, auditInfo: <ProtectionPolicyAuditInfo as RtType>::Abi, messageFromApp: HSTRING, behavior: ProtectionPolicyRequestAccessBehavior, out: *mut <foundation::IAsyncOperation<ProtectionPolicyEvaluationResult> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-storage")] fn IsFileProtectionRequiredAsync(&self, target: <super::super::storage::IStorageItem as RtType>::Abi, identity: HSTRING, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn IsFileProtectionRequiredForNewFileAsync(&self, parentFolder: <super::super::storage::IStorageFolder as RtType>::Abi, identity: HSTRING, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn get_PrimaryManagedIdentity(&self, out: *mut HSTRING) -> HRESULT,
    fn GetPrimaryManagedIdentityForIdentity(&self, identity: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IProtectionPolicyManagerStatics4 {
    #[inline] pub fn is_roamable_protection_enabled(&self, identity: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsRoamableProtectionEnabled)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn request_access_with_behavior_async(&self, sourceIdentity: &HStringArg, targetIdentity: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessWithBehaviorAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), targetIdentity.get(), auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_for_app_with_behavior_async(&self, sourceIdentity: &HStringArg, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessForAppWithBehaviorAsync)(self.get_abi() as *const _ as *mut _, sourceIdentity.get(), appPackageFamilyName.get(), auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_async(&self, sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessToFilesForAppAsync)(self.get_abi() as *const _ as *mut _, sourceItemList.get_abi() as *const _ as *mut _, appPackageFamilyName.get(), auditInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_app_with_message_and_behavior_async(&self, sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, appPackageFamilyName: &HStringArg, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessToFilesForAppWithMessageAndBehaviorAsync)(self.get_abi() as *const _ as *mut _, sourceItemList.get_abi() as *const _ as *mut _, appPackageFamilyName.get(), auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_async(&self, sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessToFilesForProcessAsync)(self.get_abi() as *const _ as *mut _, sourceItemList.get_abi() as *const _ as *mut _, processId, auditInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn request_access_to_files_for_process_with_message_and_behavior_async(&self, sourceItemList: &foundation::collections::IIterable<super::super::storage::IStorageItem>, processId: u32, auditInfo: &ProtectionPolicyAuditInfo, messageFromApp: &HStringArg, behavior: ProtectionPolicyRequestAccessBehavior) -> Result<foundation::IAsyncOperation<ProtectionPolicyEvaluationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessToFilesForProcessWithMessageAndBehaviorAsync)(self.get_abi() as *const _ as *mut _, sourceItemList.get_abi() as *const _ as *mut _, processId, auditInfo.get_abi() as *const _ as *mut _, messageFromApp.get(), behavior, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_async(&self, target: &super::super::storage::IStorageItem, identity: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().IsFileProtectionRequiredAsync)(self.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn is_file_protection_required_for_new_file_async(&self, parentFolder: &super::super::storage::IStorageFolder, identity: &HStringArg, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().IsFileProtectionRequiredForNewFileAsync)(self.get_abi() as *const _ as *mut _, parentFolder.get_abi() as *const _ as *mut _, identity.get(), desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_primary_managed_identity(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_PrimaryManagedIdentity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_primary_managed_identity_for_identity(&self, identity: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetPrimaryManagedIdentityForIdentity)(self.get_abi() as *const _ as *mut _, identity.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ProtectionPolicyRequestAccessBehavior: i32 {
    Decrypt = 0, TreatOverridePolicyAsBlock = 1,
}}
DEFINE_IID!(IID_IThreadNetworkContext, 4199459049, 61203, 16474, 177, 44, 215, 52, 140, 111, 65, 252);
RT_INTERFACE!{interface IThreadNetworkContext(IThreadNetworkContextVtbl): IInspectable [IID_IThreadNetworkContext] {
    
}}
RT_CLASS!{class ThreadNetworkContext: IThreadNetworkContext}
} // Windows.Security.EnterpriseData
pub mod exchangeactivesyncprovisioning { // Windows.Security.ExchangeActiveSyncProvisioning
use crate::prelude::*;
DEFINE_IID!(IID_IEasClientDeviceInformation, 1423956353, 6504, 19619, 185, 88, 229, 149, 209, 101, 5, 235);
RT_INTERFACE!{interface IEasClientDeviceInformation(IEasClientDeviceInformationVtbl): IInspectable [IID_IEasClientDeviceInformation] {
    fn get_Id(&self, out: *mut Guid) -> HRESULT,
    fn get_OperatingSystem(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemProductName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemSku(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEasClientDeviceInformation {
    #[inline] pub fn get_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_operating_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_OperatingSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_manufacturer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SystemManufacturer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_product_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SystemProductName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_sku(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SystemSku)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class EasClientDeviceInformation: IEasClientDeviceInformation}
impl RtActivatable<IActivationFactory> for EasClientDeviceInformation {}
DEFINE_CLSID!(EasClientDeviceInformation(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,120,99,104,97,110,103,101,65,99,116,105,118,101,83,121,110,99,80,114,111,118,105,115,105,111,110,105,110,103,46,69,97,115,67,108,105,101,110,116,68,101,118,105,99,101,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_EasClientDeviceInformation]);
DEFINE_IID!(IID_IEasClientDeviceInformation2, 4289943843, 47910, 19818, 129, 188, 22, 90, 238, 10, 215, 84);
RT_INTERFACE!{interface IEasClientDeviceInformation2(IEasClientDeviceInformation2Vtbl): IInspectable [IID_IEasClientDeviceInformation2] {
    fn get_SystemHardwareVersion(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemFirmwareVersion(&self, out: *mut HSTRING) -> HRESULT
}}
impl IEasClientDeviceInformation2 {
    #[inline] pub fn get_system_hardware_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SystemHardwareVersion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_firmware_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SystemFirmwareVersion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IEasClientSecurityPolicy, 1169630050, 57274, 19099, 172, 237, 111, 226, 173, 203, 100, 32);
RT_INTERFACE!{interface IEasClientSecurityPolicy(IEasClientSecurityPolicyVtbl): IInspectable [IID_IEasClientSecurityPolicy] {
    fn get_RequireEncryption(&self, out: *mut bool) -> HRESULT,
    fn put_RequireEncryption(&self, value: bool) -> HRESULT,
    fn get_MinPasswordLength(&self, out: *mut u8) -> HRESULT,
    fn put_MinPasswordLength(&self, value: u8) -> HRESULT,
    fn get_DisallowConvenienceLogon(&self, out: *mut bool) -> HRESULT,
    fn put_DisallowConvenienceLogon(&self, value: bool) -> HRESULT,
    fn get_MinPasswordComplexCharacters(&self, out: *mut u8) -> HRESULT,
    fn put_MinPasswordComplexCharacters(&self, value: u8) -> HRESULT,
    fn get_PasswordExpiration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn put_PasswordExpiration(&self, value: foundation::TimeSpan) -> HRESULT,
    fn get_PasswordHistory(&self, out: *mut u32) -> HRESULT,
    fn put_PasswordHistory(&self, value: u32) -> HRESULT,
    fn get_MaxPasswordFailedAttempts(&self, out: *mut u8) -> HRESULT,
    fn put_MaxPasswordFailedAttempts(&self, value: u8) -> HRESULT,
    fn get_MaxInactivityTimeLock(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn put_MaxInactivityTimeLock(&self, value: foundation::TimeSpan) -> HRESULT,
    fn CheckCompliance(&self, out: *mut <EasComplianceResults as RtType>::Abi) -> HRESULT,
    fn ApplyAsync(&self, out: *mut <foundation::IAsyncOperation<EasComplianceResults> as RtType>::Abi) -> HRESULT
}}
impl IEasClientSecurityPolicy {
    #[inline] pub fn get_require_encryption(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RequireEncryption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_require_encryption(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RequireEncryption)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_min_password_length(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MinPasswordLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_min_password_length(&self, value: u8) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MinPasswordLength)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_disallow_convenience_logon(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DisallowConvenienceLogon)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_disallow_convenience_logon(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_DisallowConvenienceLogon)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_min_password_complex_characters(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MinPasswordComplexCharacters)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_min_password_complex_characters(&self, value: u8) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MinPasswordComplexCharacters)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_password_expiration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PasswordExpiration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_password_expiration(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_PasswordExpiration)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_password_history(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PasswordHistory)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_password_history(&self, value: u32) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_PasswordHistory)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_password_failed_attempts(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MaxPasswordFailedAttempts)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_password_failed_attempts(&self, value: u8) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MaxPasswordFailedAttempts)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_inactivity_time_lock(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MaxInactivityTimeLock)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_inactivity_time_lock(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_MaxInactivityTimeLock)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn check_compliance(&self) -> Result<Option<EasComplianceResults>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CheckCompliance)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(EasComplianceResults::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn apply_async(&self) -> Result<foundation::IAsyncOperation<EasComplianceResults>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().ApplyAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class EasClientSecurityPolicy: IEasClientSecurityPolicy}
impl RtActivatable<IActivationFactory> for EasClientSecurityPolicy {}
DEFINE_CLSID!(EasClientSecurityPolicy(&[87,105,110,100,111,119,115,46,83,101,99,117,114,105,116,121,46,69,120,99,104,97,110,103,101,65,99,116,105,118,101,83,121,110,99,80,114,111,118,105,115,105,111,110,105,110,103,46,69,97,115,67,108,105,101,110,116,83,101,99,117,114,105,116,121,80,111,108,105,99,121,0]) [CLSID_EasClientSecurityPolicy]);
DEFINE_IID!(IID_IEasComplianceResults, 1178347932, 32537, 19558, 180, 3, 203, 69, 221, 87, 162, 179);
RT_INTERFACE!{interface IEasComplianceResults(IEasComplianceResultsVtbl): IInspectable [IID_IEasComplianceResults] {
    fn get_Compliant(&self, out: *mut bool) -> HRESULT,
    fn get_RequireEncryptionResult(&self, out: *mut EasRequireEncryptionResult) -> HRESULT,
    fn get_MinPasswordLengthResult(&self, out: *mut EasMinPasswordLengthResult) -> HRESULT,
    fn get_DisallowConvenienceLogonResult(&self, out: *mut EasDisallowConvenienceLogonResult) -> HRESULT,
    fn get_MinPasswordComplexCharactersResult(&self, out: *mut EasMinPasswordComplexCharactersResult) -> HRESULT,
    fn get_PasswordExpirationResult(&self, out: *mut EasPasswordExpirationResult) -> HRESULT,
    fn get_PasswordHistoryResult(&self, out: *mut EasPasswordHistoryResult) -> HRESULT,
    fn get_MaxPasswordFailedAttemptsResult(&self, out: *mut EasMaxPasswordFailedAttemptsResult) -> HRESULT,
    fn get_MaxInactivityTimeLockResult(&self, out: *mut EasMaxInactivityTimeLockResult) -> HRESULT
}}
impl IEasComplianceResults {
    #[inline] pub fn get_compliant(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Compliant)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_require_encryption_result(&self) -> Result<EasRequireEncryptionResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RequireEncryptionResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_password_length_result(&self) -> Result<EasMinPasswordLengthResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MinPasswordLengthResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_disallow_convenience_logon_result(&self) -> Result<EasDisallowConvenienceLogonResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_DisallowConvenienceLogonResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_password_complex_characters_result(&self) -> Result<EasMinPasswordComplexCharactersResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MinPasswordComplexCharactersResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_password_expiration_result(&self) -> Result<EasPasswordExpirationResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PasswordExpirationResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_password_history_result(&self) -> Result<EasPasswordHistoryResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PasswordHistoryResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_password_failed_attempts_result(&self) -> Result<EasMaxPasswordFailedAttemptsResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MaxPasswordFailedAttemptsResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_inactivity_time_lock_result(&self) -> Result<EasMaxInactivityTimeLockResult> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MaxInactivityTimeLockResult)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class EasComplianceResults: IEasComplianceResults}
DEFINE_IID!(IID_IEasComplianceResults2, 801005769, 6824, 18421, 136, 187, 203, 62, 240, 191, 251, 21);
RT_INTERFACE!{interface IEasComplianceResults2(IEasComplianceResults2Vtbl): IInspectable [IID_IEasComplianceResults2] {
    fn get_EncryptionProviderType(&self, out: *mut EasEncryptionProviderType) -> HRESULT
}}
impl IEasComplianceResults2 {
    #[inline] pub fn get_encryption_provider_type(&self) -> Result<EasEncryptionProviderType> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_EncryptionProviderType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum EasDisallowConvenienceLogonResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3,
}}
RT_ENUM! { enum EasEncryptionProviderType: i32 {
    NotEvaluated = 0, WindowsEncryption = 1, OtherEncryption = 2,
}}
RT_ENUM! { enum EasMaxInactivityTimeLockResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, InvalidParameter = 4,
}}
RT_ENUM! { enum EasMaxPasswordFailedAttemptsResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, InvalidParameter = 4,
}}
RT_ENUM! { enum EasMinPasswordComplexCharactersResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, RequestedPolicyNotEnforceable = 4, InvalidParameter = 5, CurrentUserHasBlankPassword = 6, AdminsHaveBlankPassword = 7, UserCannotChangePassword = 8, AdminsCannotChangePassword = 9, LocalControlledUsersCannotChangePassword = 10, ConnectedAdminsProviderPolicyIsWeak = 11, ConnectedUserProviderPolicyIsWeak = 12, ChangeConnectedAdminsPassword = 13, ChangeConnectedUserPassword = 14,
}}
RT_ENUM! { enum EasMinPasswordLengthResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, RequestedPolicyNotEnforceable = 4, InvalidParameter = 5, CurrentUserHasBlankPassword = 6, AdminsHaveBlankPassword = 7, UserCannotChangePassword = 8, AdminsCannotChangePassword = 9, LocalControlledUsersCannotChangePassword = 10, ConnectedAdminsProviderPolicyIsWeak = 11, ConnectedUserProviderPolicyIsWeak = 12, ChangeConnectedAdminsPassword = 13, ChangeConnectedUserPassword = 14,
}}
RT_ENUM! { enum EasPasswordExpirationResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, RequestedExpirationIncompatible = 4, InvalidParameter = 5, UserCannotChangePassword = 6, AdminsCannotChangePassword = 7, LocalControlledUsersCannotChangePassword = 8,
}}
RT_ENUM! { enum EasPasswordHistoryResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, RequestedPolicyIsStricter = 3, InvalidParameter = 4,
}}
RT_ENUM! { enum EasRequireEncryptionResult: i32 {
    NotEvaluated = 0, Compliant = 1, CanBeCompliant = 2, NotProvisionedOnAllVolumes = 3, DeFixedDataNotSupported = 4, FixedDataNotSupported = 4, DeHardwareNotCompliant = 5, HardwareNotCompliant = 5, DeWinReNotConfigured = 6, LockNotConfigured = 6, DeProtectionSuspended = 7, ProtectionSuspended = 7, DeOsVolumeNotProtected = 8, OsVolumeNotProtected = 8, DeProtectionNotYetEnabled = 9, ProtectionNotYetEnabled = 9, NoFeatureLicense = 10, OsNotProtected = 11, UnexpectedFailure = 12,
}}
} // Windows.Security.ExchangeActiveSyncProvisioning
