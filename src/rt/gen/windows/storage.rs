use crate::prelude::*;
DEFINE_IID!(IID_IAppDataPaths, 1929500170, 31138, 18633, 158, 192, 63, 218, 9, 47, 121, 225);
RT_INTERFACE!{interface IAppDataPaths(IAppDataPathsVtbl, IAppDataPaths_Abi): IInspectable(IInspectableVtbl) [IID_IAppDataPaths] {
    fn get_Cookies(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Documents(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Favorites(&self, out: *mut HSTRING) -> HRESULT,
    fn get_History(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InternetCache(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProgramData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RoamingAppData(&self, out: *mut HSTRING) -> HRESULT
}}
impl IAppDataPaths {
    #[inline] pub fn get_cookies(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Cookies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_desktop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Desktop)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_documents(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Documents)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_favorites(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Favorites)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_history(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_History)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_internet_cache(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_InternetCache)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_app_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalAppData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_program_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ProgramData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming_app_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RoamingAppData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class AppDataPaths: IAppDataPaths}
impl RtActivatable<IAppDataPathsStatics> for AppDataPaths {}
impl AppDataPaths {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::system::User) -> Result<Option<AppDataPaths>> {
        <Self as RtActivatable<IAppDataPathsStatics>>::get_activation_factory().get_for_user(user)
    }
    #[inline] pub fn get_default() -> Result<Option<AppDataPaths>> {
        <Self as RtActivatable<IAppDataPathsStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(AppDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,68,97,116,97,80,97,116,104,115,0]) [CLSID_AppDataPaths]);
DEFINE_IID!(IID_IAppDataPathsStatics, 3639290622, 43481, 19220, 185, 153, 227, 146, 19, 121, 217, 3);
RT_INTERFACE!{static interface IAppDataPathsStatics(IAppDataPathsStaticsVtbl, IAppDataPathsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IAppDataPathsStatics] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: <super::system::User as RtType>::Abi, out: *mut <AppDataPaths as RtType>::Abi) -> HRESULT,
    fn GetDefault(&self, out: *mut <AppDataPaths as RtType>::Abi) -> HRESULT
}}
impl IAppDataPathsStatics {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &super::system::User) -> Result<Option<AppDataPaths>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForUser)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(AppDataPaths::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default(&self) -> Result<Option<AppDataPaths>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(AppDataPaths::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IApplicationData, 3285872567, 46916, 19269, 176, 184, 34, 58, 9, 56, 208, 220);
RT_INTERFACE!{interface IApplicationData(IApplicationDataVtbl, IApplicationData_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationData] {
    fn get_Version(&self, out: *mut u32) -> HRESULT,
    fn SetVersionAsync(&self, desiredVersion: u32, handler: <ApplicationDataSetVersionHandler as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ClearAllAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ClearAsync(&self, locality: ApplicationDataLocality, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn get_LocalSettings(&self, out: *mut <ApplicationDataContainer as RtType>::Abi) -> HRESULT,
    fn get_RoamingSettings(&self, out: *mut <ApplicationDataContainer as RtType>::Abi) -> HRESULT,
    fn get_LocalFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_RoamingFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_TemporaryFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn add_DataChanged(&self, handler: <foundation::TypedEventHandler<ApplicationData, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DataChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn SignalDataChanged(&self) -> HRESULT,
    fn get_RoamingStorageQuota(&self, out: *mut u64) -> HRESULT
}}
impl IApplicationData {
    #[inline] pub fn get_version(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Version)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_version_async(&self, desiredVersion: u32, handler: &ApplicationDataSetVersionHandler) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetVersionAsync)(self.get_abi() as *const _ as *mut _, desiredVersion, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_all_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ClearAllAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_async(&self, locality: ApplicationDataLocality) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ClearAsync)(self.get_abi() as *const _ as *mut _, locality, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_settings(&self) -> Result<Option<ApplicationDataContainer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalSettings)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ApplicationDataContainer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming_settings(&self) -> Result<Option<ApplicationDataContainer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RoamingSettings)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ApplicationDataContainer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RoamingFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_temporary_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TemporaryFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_data_changed(&self, handler: &foundation::TypedEventHandler<ApplicationData, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DataChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_data_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DataChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn signal_data_changed(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SignalDataChanged)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming_storage_quota(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RoamingStorageQuota)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ApplicationData: IApplicationData}
impl RtActivatable<IApplicationDataStatics> for ApplicationData {}
impl RtActivatable<IApplicationDataStatics2> for ApplicationData {}
impl ApplicationData {
    #[inline] pub fn get_current() -> Result<Option<ApplicationData>> {
        <Self as RtActivatable<IApplicationDataStatics>>::get_activation_factory().get_current()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(user: &super::system::User) -> Result<foundation::IAsyncOperation<ApplicationData>> {
        <Self as RtActivatable<IApplicationDataStatics2>>::get_activation_factory().get_for_user_async(user)
    }
}
DEFINE_CLSID!(ApplicationData(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,108,105,99,97,116,105,111,110,68,97,116,97,0]) [CLSID_ApplicationData]);
DEFINE_IID!(IID_IApplicationData2, 2657471849, 2979, 20018, 190, 41, 176, 45, 230, 96, 118, 56);
RT_INTERFACE!{interface IApplicationData2(IApplicationData2Vtbl, IApplicationData2_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationData2] {
    fn get_LocalCacheFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IApplicationData2 {
    #[inline] pub fn get_local_cache_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalCacheFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IApplicationData3, 3693227252, 10098, 19485, 170, 44, 201, 247, 67, 173, 232, 209);
RT_INTERFACE!{interface IApplicationData3(IApplicationData3Vtbl, IApplicationData3_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationData3] {
    fn GetPublisherCacheFolder(&self, folderName: HSTRING, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn ClearPublisherCacheFolderAsync(&self, folderName: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn get_SharedLocalFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IApplicationData3 {
    #[inline] pub fn get_publisher_cache_folder(&self, folderName: &HStringArg) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPublisherCacheFolder)(self.get_abi() as *const _ as *mut _, folderName.get(), &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn clear_publisher_cache_folder_async(&self, folderName: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ClearPublisherCacheFolderAsync)(self.get_abi() as *const _ as *mut _, folderName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_shared_local_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SharedLocalFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ApplicationDataCompositeValue: foundation::collections::IPropertySet}
impl RtActivatable<IActivationFactory> for ApplicationDataCompositeValue {}
DEFINE_CLSID!(ApplicationDataCompositeValue(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,112,112,108,105,99,97,116,105,111,110,68,97,116,97,67,111,109,112,111,115,105,116,101,86,97,108,117,101,0]) [CLSID_ApplicationDataCompositeValue]);
DEFINE_IID!(IID_IApplicationDataContainer, 3316579614, 62567, 16570, 133, 102, 171, 100, 10, 68, 30, 29);
RT_INTERFACE!{interface IApplicationDataContainer(IApplicationDataContainerVtbl, IApplicationDataContainer_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationDataContainer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Locality(&self, out: *mut ApplicationDataLocality) -> HRESULT,
    fn get_Values(&self, out: *mut <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn get_Containers(&self, out: *mut <foundation::collections::IMapView<HString, ApplicationDataContainer> as RtType>::Abi) -> HRESULT,
    fn CreateContainer(&self, name: HSTRING, disposition: ApplicationDataCreateDisposition, out: *mut <ApplicationDataContainer as RtType>::Abi) -> HRESULT,
    fn DeleteContainer(&self, name: HSTRING) -> HRESULT
}}
impl IApplicationDataContainer {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_locality(&self) -> Result<ApplicationDataLocality> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Locality)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_values(&self) -> Result<Option<foundation::collections::IPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Values)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IPropertySet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_containers(&self) -> Result<Option<foundation::collections::IMapView<HString, ApplicationDataContainer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Containers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_container(&self, name: &HStringArg, disposition: ApplicationDataCreateDisposition) -> Result<Option<ApplicationDataContainer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateContainer)(self.get_abi() as *const _ as *mut _, name.get(), disposition, &mut out);
        if hr == S_OK { Ok(ApplicationDataContainer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_container(&self, name: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).DeleteContainer)(self.get_abi() as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ApplicationDataContainer: IApplicationDataContainer}
RT_CLASS!{class ApplicationDataContainerSettings: foundation::collections::IPropertySet}
RT_ENUM! { enum ApplicationDataCreateDisposition: i32 {
    Always = 0, Existing = 1,
}}
RT_ENUM! { enum ApplicationDataLocality: i32 {
    Local = 0, Roaming = 1, Temporary = 2, LocalCache = 3,
}}
DEFINE_IID!(IID_ApplicationDataSetVersionHandler, 2690093542, 52383, 18055, 172, 171, 163, 100, 253, 120, 84, 99);
RT_DELEGATE!{delegate ApplicationDataSetVersionHandler(ApplicationDataSetVersionHandlerVtbl, ApplicationDataSetVersionHandler_Abi, ApplicationDataSetVersionHandlerImpl) [IID_ApplicationDataSetVersionHandler] {
    fn Invoke(&self, setVersionRequest: <SetVersionRequest as RtType>::Abi) -> HRESULT
}}
impl ApplicationDataSetVersionHandler {
    #[inline] pub fn invoke(&self, setVersionRequest: &SetVersionRequest) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, setVersionRequest.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IApplicationDataStatics, 1444025467, 59459, 17891, 148, 216, 6, 22, 158, 60, 142, 23);
RT_INTERFACE!{static interface IApplicationDataStatics(IApplicationDataStaticsVtbl, IApplicationDataStatics_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationDataStatics] {
    fn get_Current(&self, out: *mut <ApplicationData as RtType>::Abi) -> HRESULT
}}
impl IApplicationDataStatics {
    #[inline] pub fn get_current(&self) -> Result<Option<ApplicationData>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Current)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ApplicationData::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IApplicationDataStatics2, 3445645841, 53065, 16548, 164, 124, 199, 240, 219, 186, 129, 7);
RT_INTERFACE!{static interface IApplicationDataStatics2(IApplicationDataStatics2Vtbl, IApplicationDataStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IApplicationDataStatics2] {
    #[cfg(feature="windows-system")] fn GetForUserAsync(&self, user: <super::system::User as RtType>::Abi, out: *mut <foundation::IAsyncOperation<ApplicationData> as RtType>::Abi) -> HRESULT
}}
impl IApplicationDataStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user_async(&self, user: &super::system::User) -> Result<foundation::IAsyncOperation<ApplicationData>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class CachedFileManager}
impl RtActivatable<ICachedFileManagerStatics> for CachedFileManager {}
impl CachedFileManager {
    #[inline] pub fn defer_updates(file: &IStorageFile) -> Result<()> {
        <Self as RtActivatable<ICachedFileManagerStatics>>::get_activation_factory().defer_updates(file)
    }
    #[inline] pub fn complete_updates_async(file: &IStorageFile) -> Result<foundation::IAsyncOperation<provider::FileUpdateStatus>> {
        <Self as RtActivatable<ICachedFileManagerStatics>>::get_activation_factory().complete_updates_async(file)
    }
}
DEFINE_CLSID!(CachedFileManager(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,97,99,104,101,100,70,105,108,101,77,97,110,97,103,101,114,0]) [CLSID_CachedFileManager]);
DEFINE_IID!(IID_ICachedFileManagerStatics, 2415665738, 59266, 18781, 182, 20, 101, 76, 79, 11, 35, 112);
RT_INTERFACE!{static interface ICachedFileManagerStatics(ICachedFileManagerStaticsVtbl, ICachedFileManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_ICachedFileManagerStatics] {
    fn DeferUpdates(&self, file: <IStorageFile as RtType>::Abi) -> HRESULT,
    fn CompleteUpdatesAsync(&self, file: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<provider::FileUpdateStatus> as RtType>::Abi) -> HRESULT
}}
impl ICachedFileManagerStatics {
    #[inline] pub fn defer_updates(&self, file: &IStorageFile) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).DeferUpdates)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn complete_updates_async(&self, file: &IStorageFile) -> Result<foundation::IAsyncOperation<provider::FileUpdateStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CompleteUpdatesAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum CreationCollisionOption: i32 {
    GenerateUniqueName = 0, ReplaceExisting = 1, FailIfExists = 2, OpenIfExists = 3,
}}
RT_CLASS!{static class DownloadsFolder}
impl RtActivatable<IDownloadsFolderStatics> for DownloadsFolder {}
impl RtActivatable<IDownloadsFolderStatics2> for DownloadsFolder {}
impl DownloadsFolder {
    #[inline] pub fn create_file_async(desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_file_async(desiredName)
    }
    #[inline] pub fn create_folder_async(desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_folder_async(desiredName)
    }
    #[inline] pub fn create_file_with_collision_option_async(desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_file_with_collision_option_async(desiredName, option)
    }
    #[inline] pub fn create_folder_with_collision_option_async(desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IDownloadsFolderStatics>>::get_activation_factory().create_folder_with_collision_option_async(desiredName, option)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_async(user: &super::system::User, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_file_for_user_async(user, desiredName)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_async(user: &super::system::User, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_folder_for_user_async(user, desiredName)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_with_collision_option_async(user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_file_for_user_with_collision_option_async(user, desiredName, option)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_with_collision_option_async(user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IDownloadsFolderStatics2>>::get_activation_factory().create_folder_for_user_with_collision_option_async(user, desiredName, option)
    }
}
DEFINE_CLSID!(DownloadsFolder(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,68,111,119,110,108,111,97,100,115,70,111,108,100,101,114,0]) [CLSID_DownloadsFolder]);
DEFINE_IID!(IID_IDownloadsFolderStatics, 663105232, 16462, 18399, 161, 226, 227, 115, 8, 190, 123, 55);
RT_INTERFACE!{static interface IDownloadsFolderStatics(IDownloadsFolderStaticsVtbl, IDownloadsFolderStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDownloadsFolderStatics] {
    fn CreateFileAsync(&self, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateFolderAsync(&self, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn CreateFileWithCollisionOptionAsync(&self, desiredName: HSTRING, option: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateFolderWithCollisionOptionAsync(&self, desiredName: HSTRING, option: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT
}}
impl IDownloadsFolderStatics {
    #[inline] pub fn create_file_async(&self, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_async(&self, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_file_with_collision_option_async(&self, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileWithCollisionOptionAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_with_collision_option_async(&self, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderWithCollisionOptionAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDownloadsFolderStatics2, 3912254909, 36600, 20366, 141, 21, 172, 14, 38, 95, 57, 13);
RT_INTERFACE!{static interface IDownloadsFolderStatics2(IDownloadsFolderStatics2Vtbl, IDownloadsFolderStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IDownloadsFolderStatics2] {
    #[cfg(feature="windows-system")] fn CreateFileForUserAsync(&self, user: <super::system::User as RtType>::Abi, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFolderForUserAsync(&self, user: <super::system::User as RtType>::Abi, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFileForUserWithCollisionOptionAsync(&self, user: <super::system::User as RtType>::Abi, desiredName: HSTRING, option: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn CreateFolderForUserWithCollisionOptionAsync(&self, user: <super::system::User as RtType>::Abi, desiredName: HSTRING, option: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT
}}
impl IDownloadsFolderStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_async(&self, user: &super::system::User, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_async(&self, user: &super::system::User, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_file_for_user_with_collision_option_async(&self, user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileForUserWithCollisionOptionAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn create_folder_for_user_with_collision_option_async(&self, user: &super::system::User, desiredName: &HStringArg, option: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderForUserWithCollisionOptionAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum FileAccessMode: i32 {
    Read = 0, ReadWrite = 1,
}}
RT_ENUM! { enum FileAttributes: u32 {
    Normal = 0, ReadOnly = 1, Directory = 16, Archive = 32, Temporary = 256, LocallyIncomplete = 512,
}}
RT_CLASS!{static class FileIO}
impl RtActivatable<IFileIOStatics> for FileIO {}
impl FileIO {
    #[inline] pub fn read_text_async(file: &IStorageFile) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_text_async(file)
    }
    #[inline] pub fn read_text_with_encoding_async(file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_text_with_encoding_async(file, encoding)
    }
    #[inline] pub fn write_text_async(file: &IStorageFile, contents: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_text_async(file, contents)
    }
    #[inline] pub fn write_text_with_encoding_async(file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_text_with_encoding_async(file, contents, encoding)
    }
    #[inline] pub fn append_text_async(file: &IStorageFile, contents: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_text_async(file, contents)
    }
    #[inline] pub fn append_text_with_encoding_async(file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_text_with_encoding_async(file, contents, encoding)
    }
    #[inline] pub fn read_lines_async(file: &IStorageFile) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_lines_async(file)
    }
    #[inline] pub fn read_lines_with_encoding_async(file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_lines_with_encoding_async(file, encoding)
    }
    #[inline] pub fn write_lines_async(file: &IStorageFile, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_lines_async(file, lines)
    }
    #[inline] pub fn write_lines_with_encoding_async(file: &IStorageFile, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_lines_with_encoding_async(file, lines, encoding)
    }
    #[inline] pub fn append_lines_async(file: &IStorageFile, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_lines_async(file, lines)
    }
    #[inline] pub fn append_lines_with_encoding_async(file: &IStorageFile, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().append_lines_with_encoding_async(file, lines, encoding)
    }
    #[inline] pub fn read_buffer_async(file: &IStorageFile) -> Result<foundation::IAsyncOperation<streams::IBuffer>> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().read_buffer_async(file)
    }
    #[inline] pub fn write_buffer_async(file: &IStorageFile, buffer: &streams::IBuffer) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_buffer_async(file, buffer)
    }
    #[inline] pub fn write_bytes_async(file: &IStorageFile, buffer: &[u8]) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IFileIOStatics>>::get_activation_factory().write_bytes_async(file, buffer)
    }
}
DEFINE_CLSID!(FileIO(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,70,105,108,101,73,79,0]) [CLSID_FileIO]);
DEFINE_IID!(IID_IFileIOStatics, 2289308139, 32596, 18226, 165, 240, 94, 67, 227, 184, 194, 245);
RT_INTERFACE!{static interface IFileIOStatics(IFileIOStaticsVtbl, IFileIOStatics_Abi): IInspectable(IInspectableVtbl) [IID_IFileIOStatics] {
    fn ReadTextAsync(&self, file: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn ReadTextWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn WriteTextAsync(&self, file: <IStorageFile as RtType>::Abi, contents: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteTextWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendTextAsync(&self, file: <IStorageFile as RtType>::Abi, contents: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendTextWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ReadLinesAsync(&self, file: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVector<HString>> as RtType>::Abi) -> HRESULT,
    fn ReadLinesWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncOperation<foundation::collections::IVector<HString>> as RtType>::Abi) -> HRESULT,
    fn WriteLinesAsync(&self, file: <IStorageFile as RtType>::Abi, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteLinesWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendLinesAsync(&self, file: <IStorageFile as RtType>::Abi, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendLinesWithEncodingAsync(&self, file: <IStorageFile as RtType>::Abi, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ReadBufferAsync(&self, file: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<streams::IBuffer> as RtType>::Abi) -> HRESULT,
    fn WriteBufferAsync(&self, file: <IStorageFile as RtType>::Abi, buffer: <streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteBytesAsync(&self, file: <IStorageFile as RtType>::Abi, bufferSize: u32, buffer: *mut u8, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IFileIOStatics {
    #[inline] pub fn read_text_async(&self, file: &IStorageFile) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadTextAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_text_with_encoding_async(&self, file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_text_async(&self, file: &IStorageFile, contents: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteTextAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, contents.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_text_with_encoding_async(&self, file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_text_async(&self, file: &IStorageFile, contents: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendTextAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, contents.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_text_with_encoding_async(&self, file: &IStorageFile, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_lines_async(&self, file: &IStorageFile) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadLinesAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_lines_with_encoding_async(&self, file: &IStorageFile, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_lines_async(&self, file: &IStorageFile, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteLinesAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, lines.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_lines_with_encoding_async(&self, file: &IStorageFile, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, lines.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_lines_async(&self, file: &IStorageFile, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendLinesAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, lines.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_lines_with_encoding_async(&self, file: &IStorageFile, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, lines.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_buffer_async(&self, file: &IStorageFile) -> Result<foundation::IAsyncOperation<streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadBufferAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_buffer_async(&self, file: &IStorageFile, buffer: &streams::IBuffer) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteBufferAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_bytes_async(&self, file: &IStorageFile, buffer: &[u8]) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteBytesAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, buffer.len() as u32, buffer.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum KnownFolderId: i32 {
    AppCaptures = 0, CameraRoll = 1, DocumentsLibrary = 2, HomeGroup = 3, MediaServerDevices = 4, MusicLibrary = 5, Objects3D = 6, PicturesLibrary = 7, Playlists = 8, RecordedCalls = 9, RemovableDevices = 10, SavedPictures = 11, Screenshots = 12, VideosLibrary = 13, AllAppMods = 14, CurrentAppMods = 15,
}}
RT_CLASS!{static class KnownFolders}
impl RtActivatable<IKnownFoldersCameraRollStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersPlaylistsStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersSavedPicturesStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics2> for KnownFolders {}
impl RtActivatable<IKnownFoldersStatics3> for KnownFolders {}
impl KnownFolders {
    #[inline] pub fn get_camera_roll() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersCameraRollStatics>>::get_activation_factory().get_camera_roll()
    }
    #[inline] pub fn get_playlists() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersPlaylistsStatics>>::get_activation_factory().get_playlists()
    }
    #[inline] pub fn get_saved_pictures() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersSavedPicturesStatics>>::get_activation_factory().get_saved_pictures()
    }
    #[inline] pub fn get_music_library() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_music_library()
    }
    #[inline] pub fn get_pictures_library() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_pictures_library()
    }
    #[inline] pub fn get_videos_library() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_videos_library()
    }
    #[inline] pub fn get_documents_library() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_documents_library()
    }
    #[inline] pub fn get_home_group() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_home_group()
    }
    #[inline] pub fn get_removable_devices() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_removable_devices()
    }
    #[inline] pub fn get_media_server_devices() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics>>::get_activation_factory().get_media_server_devices()
    }
    #[inline] pub fn get_objects_3d() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_objects_3d()
    }
    #[inline] pub fn get_app_captures() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_app_captures()
    }
    #[inline] pub fn get_recorded_calls() -> Result<Option<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics2>>::get_activation_factory().get_recorded_calls()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_folder_for_user_async(user: &super::system::User, folderId: KnownFolderId) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IKnownFoldersStatics3>>::get_activation_factory().get_folder_for_user_async(user, folderId)
    }
}
DEFINE_CLSID!(KnownFolders(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,75,110,111,119,110,70,111,108,100,101,114,115,0]) [CLSID_KnownFolders]);
DEFINE_IID!(IID_IKnownFoldersCameraRollStatics, 1561419366, 10216, 18735, 184, 229, 47, 144, 137, 108, 212, 205);
RT_INTERFACE!{static interface IKnownFoldersCameraRollStatics(IKnownFoldersCameraRollStaticsVtbl, IKnownFoldersCameraRollStatics_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersCameraRollStatics] {
    fn get_CameraRoll(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersCameraRollStatics {
    #[inline] pub fn get_camera_roll(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraRoll)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKnownFoldersPlaylistsStatics, 3671452886, 12399, 19818, 180, 150, 70, 186, 142, 177, 6, 206);
RT_INTERFACE!{static interface IKnownFoldersPlaylistsStatics(IKnownFoldersPlaylistsStaticsVtbl, IKnownFoldersPlaylistsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersPlaylistsStatics] {
    fn get_Playlists(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersPlaylistsStatics {
    #[inline] pub fn get_playlists(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Playlists)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKnownFoldersSavedPicturesStatics, 89953258, 9533, 18044, 182, 202, 169, 125, 161, 233, 161, 141);
RT_INTERFACE!{static interface IKnownFoldersSavedPicturesStatics(IKnownFoldersSavedPicturesStaticsVtbl, IKnownFoldersSavedPicturesStatics_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersSavedPicturesStatics] {
    fn get_SavedPictures(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersSavedPicturesStatics {
    #[inline] pub fn get_saved_pictures(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SavedPictures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKnownFoldersStatics, 1512731936, 18434, 17709, 154, 217, 67, 81, 173, 167, 236, 53);
RT_INTERFACE!{static interface IKnownFoldersStatics(IKnownFoldersStaticsVtbl, IKnownFoldersStatics_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics] {
    fn get_MusicLibrary(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_PicturesLibrary(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_VideosLibrary(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_DocumentsLibrary(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_HomeGroup(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_RemovableDevices(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_MediaServerDevices(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersStatics {
    #[inline] pub fn get_music_library(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MusicLibrary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pictures_library(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PicturesLibrary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_videos_library(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_VideosLibrary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_documents_library(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentsLibrary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_home_group(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_HomeGroup)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_removable_devices(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RemovableDevices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_media_server_devices(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MediaServerDevices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKnownFoldersStatics2, 424399053, 53102, 19719, 157, 83, 233, 22, 58, 37, 54, 233);
RT_INTERFACE!{static interface IKnownFoldersStatics2(IKnownFoldersStatics2Vtbl, IKnownFoldersStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics2] {
    fn get_Objects3D(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_AppCaptures(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_RecordedCalls(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersStatics2 {
    #[inline] pub fn get_objects_3d(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Objects3D)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_app_captures(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AppCaptures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_recorded_calls(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RecordedCalls)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IKnownFoldersStatics3, 3306767169, 38722, 20181, 130, 61, 252, 20, 1, 20, 135, 100);
RT_INTERFACE!{static interface IKnownFoldersStatics3(IKnownFoldersStatics3Vtbl, IKnownFoldersStatics3_Abi): IInspectable(IInspectableVtbl) [IID_IKnownFoldersStatics3] {
    #[cfg(feature="windows-system")] fn GetFolderForUserAsync(&self, user: <super::system::User as RtType>::Abi, folderId: KnownFolderId, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT
}}
impl IKnownFoldersStatics3 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_folder_for_user_async(&self, user: &super::system::User, folderId: KnownFolderId) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFolderForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, folderId, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum KnownLibraryId: i32 {
    Music = 0, Pictures = 1, Videos = 2, Documents = 3,
}}
RT_ENUM! { enum NameCollisionOption: i32 {
    GenerateUniqueName = 0, ReplaceExisting = 1, FailIfExists = 2,
}}
RT_CLASS!{static class PathIO}
impl RtActivatable<IPathIOStatics> for PathIO {}
impl PathIO {
    #[inline] pub fn read_text_async(absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_text_async(absolutePath)
    }
    #[inline] pub fn read_text_with_encoding_async(absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<HString>> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_text_with_encoding_async(absolutePath, encoding)
    }
    #[inline] pub fn write_text_async(absolutePath: &HStringArg, contents: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_text_async(absolutePath, contents)
    }
    #[inline] pub fn write_text_with_encoding_async(absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_text_with_encoding_async(absolutePath, contents, encoding)
    }
    #[inline] pub fn append_text_async(absolutePath: &HStringArg, contents: &HStringArg) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_text_async(absolutePath, contents)
    }
    #[inline] pub fn append_text_with_encoding_async(absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_text_with_encoding_async(absolutePath, contents, encoding)
    }
    #[inline] pub fn read_lines_async(absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_lines_async(absolutePath)
    }
    #[inline] pub fn read_lines_with_encoding_async(absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_lines_with_encoding_async(absolutePath, encoding)
    }
    #[inline] pub fn write_lines_async(absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_lines_async(absolutePath, lines)
    }
    #[inline] pub fn write_lines_with_encoding_async(absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_lines_with_encoding_async(absolutePath, lines, encoding)
    }
    #[inline] pub fn append_lines_async(absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_lines_async(absolutePath, lines)
    }
    #[inline] pub fn append_lines_with_encoding_async(absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().append_lines_with_encoding_async(absolutePath, lines, encoding)
    }
    #[inline] pub fn read_buffer_async(absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<streams::IBuffer>> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().read_buffer_async(absolutePath)
    }
    #[inline] pub fn write_buffer_async(absolutePath: &HStringArg, buffer: &streams::IBuffer) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_buffer_async(absolutePath, buffer)
    }
    #[inline] pub fn write_bytes_async(absolutePath: &HStringArg, buffer: &[u8]) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IPathIOStatics>>::get_activation_factory().write_bytes_async(absolutePath, buffer)
    }
}
DEFINE_CLSID!(PathIO(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,97,116,104,73,79,0]) [CLSID_PathIO]);
DEFINE_IID!(IID_IPathIOStatics, 254752600, 36551, 17281, 146, 43, 143, 108, 7, 210, 136, 243);
RT_INTERFACE!{static interface IPathIOStatics(IPathIOStaticsVtbl, IPathIOStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPathIOStatics] {
    fn ReadTextAsync(&self, absolutePath: HSTRING, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn ReadTextWithEncodingAsync(&self, absolutePath: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT,
    fn WriteTextAsync(&self, absolutePath: HSTRING, contents: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteTextWithEncodingAsync(&self, absolutePath: HSTRING, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendTextAsync(&self, absolutePath: HSTRING, contents: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendTextWithEncodingAsync(&self, absolutePath: HSTRING, contents: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ReadLinesAsync(&self, absolutePath: HSTRING, out: *mut <foundation::IAsyncOperation<foundation::collections::IVector<HString>> as RtType>::Abi) -> HRESULT,
    fn ReadLinesWithEncodingAsync(&self, absolutePath: HSTRING, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncOperation<foundation::collections::IVector<HString>> as RtType>::Abi) -> HRESULT,
    fn WriteLinesAsync(&self, absolutePath: HSTRING, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteLinesWithEncodingAsync(&self, absolutePath: HSTRING, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendLinesAsync(&self, absolutePath: HSTRING, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn AppendLinesWithEncodingAsync(&self, absolutePath: HSTRING, lines: <foundation::collections::IIterable<HString> as RtType>::Abi, encoding: streams::UnicodeEncoding, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ReadBufferAsync(&self, absolutePath: HSTRING, out: *mut <foundation::IAsyncOperation<streams::IBuffer> as RtType>::Abi) -> HRESULT,
    fn WriteBufferAsync(&self, absolutePath: HSTRING, buffer: <streams::IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn WriteBytesAsync(&self, absolutePath: HSTRING, bufferSize: u32, buffer: *mut u8, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IPathIOStatics {
    #[inline] pub fn read_text_async(&self, absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadTextAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_text_with_encoding_async(&self, absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_text_async(&self, absolutePath: &HStringArg, contents: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteTextAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), contents.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_text_with_encoding_async(&self, absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_text_async(&self, absolutePath: &HStringArg, contents: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendTextAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), contents.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_text_with_encoding_async(&self, absolutePath: &HStringArg, contents: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendTextWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), contents.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_lines_async(&self, absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadLinesAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_lines_with_encoding_async(&self, absolutePath: &HStringArg, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncOperation<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_lines_async(&self, absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteLinesAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), lines.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_lines_with_encoding_async(&self, absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), lines.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_lines_async(&self, absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendLinesAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), lines.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_lines_with_encoding_async(&self, absolutePath: &HStringArg, lines: &foundation::collections::IIterable<HString>, encoding: streams::UnicodeEncoding) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendLinesWithEncodingAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), lines.get_abi() as *const _ as *mut _, encoding, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_buffer_async(&self, absolutePath: &HStringArg) -> Result<foundation::IAsyncOperation<streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadBufferAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_buffer_async(&self, absolutePath: &HStringArg, buffer: &streams::IBuffer) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteBufferAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn write_bytes_async(&self, absolutePath: &HStringArg, buffer: &[u8]) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteBytesAsync)(self.get_abi() as *const _ as *mut _, absolutePath.get(), buffer.len() as u32, buffer.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISetVersionDeferral, 53807266, 30746, 17274, 176, 120, 63, 50, 186, 220, 254, 71);
RT_INTERFACE!{interface ISetVersionDeferral(ISetVersionDeferralVtbl, ISetVersionDeferral_Abi): IInspectable(IInspectableVtbl) [IID_ISetVersionDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl ISetVersionDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SetVersionDeferral: ISetVersionDeferral}
DEFINE_IID!(IID_ISetVersionRequest, 3116854171, 4182, 20073, 131, 48, 22, 38, 25, 149, 111, 155);
RT_INTERFACE!{interface ISetVersionRequest(ISetVersionRequestVtbl, ISetVersionRequest_Abi): IInspectable(IInspectableVtbl) [IID_ISetVersionRequest] {
    fn get_CurrentVersion(&self, out: *mut u32) -> HRESULT,
    fn get_DesiredVersion(&self, out: *mut u32) -> HRESULT,
    fn GetDeferral(&self, out: *mut <SetVersionDeferral as RtType>::Abi) -> HRESULT
}}
impl ISetVersionRequest {
    #[inline] pub fn get_current_version(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CurrentVersion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_desired_version(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DesiredVersion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<SetVersionDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SetVersionDeferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SetVersionRequest: ISetVersionRequest}
RT_ENUM! { enum StorageDeleteOption: i32 {
    Default = 0, PermanentDelete = 1,
}}
DEFINE_IID!(IID_IStorageFile, 4198457734, 16916, 17036, 166, 76, 20, 201, 172, 115, 21, 234);
RT_INTERFACE!{interface IStorageFile(IStorageFileVtbl, IStorageFile_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFile] {
    fn get_FileType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn OpenAsync(&self, accessMode: FileAccessMode, out: *mut <foundation::IAsyncOperation<streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn OpenTransactedWriteAsync(&self, out: *mut <foundation::IAsyncOperation<StorageStreamTransaction> as RtType>::Abi) -> HRESULT,
    fn CopyOverloadDefaultNameAndOptions(&self, destinationFolder: <IStorageFolder as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CopyOverloadDefaultOptions(&self, destinationFolder: <IStorageFolder as RtType>::Abi, desiredNewName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CopyOverload(&self, destinationFolder: <IStorageFolder as RtType>::Abi, desiredNewName: HSTRING, option: NameCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CopyAndReplaceAsync(&self, fileToReplace: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn MoveOverloadDefaultNameAndOptions(&self, destinationFolder: <IStorageFolder as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn MoveOverloadDefaultOptions(&self, destinationFolder: <IStorageFolder as RtType>::Abi, desiredNewName: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn MoveOverload(&self, destinationFolder: <IStorageFolder as RtType>::Abi, desiredNewName: HSTRING, option: NameCollisionOption, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn MoveAndReplaceAsync(&self, fileToReplace: <IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IStorageFile {
    #[inline] pub fn get_file_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_content_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContentType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_async(&self, accessMode: FileAccessMode) -> Result<foundation::IAsyncOperation<streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenAsync)(self.get_abi() as *const _ as *mut _, accessMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_transacted_write_async(&self) -> Result<foundation::IAsyncOperation<StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_overload_default_name_and_options(&self, destinationFolder: &IStorageFolder) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyOverloadDefaultNameAndOptions)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_overload_default_options(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, desiredNewName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_overload(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg, option: NameCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyOverload)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, desiredNewName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_and_replace_async(&self, fileToReplace: &IStorageFile) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyAndReplaceAsync)(self.get_abi() as *const _ as *mut _, fileToReplace.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn move_overload_default_name_and_options(&self, destinationFolder: &IStorageFolder) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).MoveOverloadDefaultNameAndOptions)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn move_overload_default_options(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).MoveOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, desiredNewName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn move_overload(&self, destinationFolder: &IStorageFolder, desiredNewName: &HStringArg, option: NameCollisionOption) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).MoveOverload)(self.get_abi() as *const _ as *mut _, destinationFolder.get_abi() as *const _ as *mut _, desiredNewName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn move_and_replace_async(&self, fileToReplace: &IStorageFile) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).MoveAndReplaceAsync)(self.get_abi() as *const _ as *mut _, fileToReplace.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageFile: IStorageFile}
impl RtActivatable<IStorageFileStatics> for StorageFile {}
impl StorageFile {
    #[inline] pub fn get_file_from_path_async(path: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().get_file_from_path_async(path)
    }
    #[inline] pub fn get_file_from_application_uri_async(uri: &foundation::Uri) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().get_file_from_application_uri_async(uri)
    }
    #[inline] pub fn create_streamed_file_async(displayNameWithExtension: &HStringArg, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().create_streamed_file_async(displayNameWithExtension, dataRequested, thumbnail)
    }
    #[inline] pub fn replace_with_streamed_file_async(fileToReplace: &IStorageFile, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().replace_with_streamed_file_async(fileToReplace, dataRequested, thumbnail)
    }
    #[inline] pub fn create_streamed_file_from_uri_async(displayNameWithExtension: &HStringArg, uri: &foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().create_streamed_file_from_uri_async(displayNameWithExtension, uri, thumbnail)
    }
    #[inline] pub fn replace_with_streamed_file_from_uri_async(fileToReplace: &IStorageFile, uri: &foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> {
        <Self as RtActivatable<IStorageFileStatics>>::get_activation_factory().replace_with_streamed_file_from_uri_async(fileToReplace, uri, thumbnail)
    }
}
DEFINE_CLSID!(StorageFile(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,70,105,108,101,0]) [CLSID_StorageFile]);
DEFINE_IID!(IID_IStorageFile2, 2504936399, 2679, 17147, 183, 119, 194, 237, 88, 165, 46, 68);
RT_INTERFACE!{interface IStorageFile2(IStorageFile2Vtbl, IStorageFile2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFile2] {
    fn OpenWithOptionsAsync(&self, accessMode: FileAccessMode, options: StorageOpenOptions, out: *mut <foundation::IAsyncOperation<streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn OpenTransactedWriteWithOptionsAsync(&self, options: StorageOpenOptions, out: *mut <foundation::IAsyncOperation<StorageStreamTransaction> as RtType>::Abi) -> HRESULT
}}
impl IStorageFile2 {
    #[inline] pub fn open_with_options_async(&self, accessMode: FileAccessMode, options: StorageOpenOptions) -> Result<foundation::IAsyncOperation<streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenWithOptionsAsync)(self.get_abi() as *const _ as *mut _, accessMode, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_transacted_write_with_options_async(&self, options: StorageOpenOptions) -> Result<foundation::IAsyncOperation<StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteWithOptionsAsync)(self.get_abi() as *const _ as *mut _, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFilePropertiesWithAvailability, 2949365403, 22571, 16691, 150, 72, 228, 76, 164, 110, 228, 145);
RT_INTERFACE!{interface IStorageFilePropertiesWithAvailability(IStorageFilePropertiesWithAvailabilityVtbl, IStorageFilePropertiesWithAvailability_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFilePropertiesWithAvailability] {
    fn get_IsAvailable(&self, out: *mut bool) -> HRESULT
}}
impl IStorageFilePropertiesWithAvailability {
    #[inline] pub fn get_is_available(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsAvailable)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFileStatics, 1501873936, 56050, 17352, 139, 180, 164, 211, 234, 207, 208, 63);
RT_INTERFACE!{static interface IStorageFileStatics(IStorageFileStaticsVtbl, IStorageFileStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFileStatics] {
    fn GetFileFromPathAsync(&self, path: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn GetFileFromApplicationUriAsync(&self, uri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateStreamedFileAsync(&self, displayNameWithExtension: HSTRING, dataRequested: <StreamedFileDataRequestedHandler as RtType>::Abi, thumbnail: <streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn ReplaceWithStreamedFileAsync(&self, fileToReplace: <IStorageFile as RtType>::Abi, dataRequested: <StreamedFileDataRequestedHandler as RtType>::Abi, thumbnail: <streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateStreamedFileFromUriAsync(&self, displayNameWithExtension: HSTRING, uri: <foundation::Uri as RtType>::Abi, thumbnail: <streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn ReplaceWithStreamedFileFromUriAsync(&self, fileToReplace: <IStorageFile as RtType>::Abi, uri: <foundation::Uri as RtType>::Abi, thumbnail: <streams::IRandomAccessStreamReference as RtType>::Abi, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT
}}
impl IStorageFileStatics {
    #[inline] pub fn get_file_from_path_async(&self, path: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFileFromPathAsync)(self.get_abi() as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file_from_application_uri_async(&self, uri: &foundation::Uri) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFileFromApplicationUriAsync)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_streamed_file_async(&self, displayNameWithExtension: &HStringArg, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateStreamedFileAsync)(self.get_abi() as *const _ as *mut _, displayNameWithExtension.get(), dataRequested.get_abi() as *const _ as *mut _, thumbnail.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn replace_with_streamed_file_async(&self, fileToReplace: &IStorageFile, dataRequested: &StreamedFileDataRequestedHandler, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReplaceWithStreamedFileAsync)(self.get_abi() as *const _ as *mut _, fileToReplace.get_abi() as *const _ as *mut _, dataRequested.get_abi() as *const _ as *mut _, thumbnail.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_streamed_file_from_uri_async(&self, displayNameWithExtension: &HStringArg, uri: &foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateStreamedFileFromUriAsync)(self.get_abi() as *const _ as *mut _, displayNameWithExtension.get(), uri.get_abi() as *const _ as *mut _, thumbnail.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn replace_with_streamed_file_from_uri_async(&self, fileToReplace: &IStorageFile, uri: &foundation::Uri, thumbnail: &streams::IRandomAccessStreamReference) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReplaceWithStreamedFileFromUriAsync)(self.get_abi() as *const _ as *mut _, fileToReplace.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, thumbnail.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFolder, 1926351736, 46063, 20341, 168, 11, 111, 217, 218, 226, 148, 75);
RT_INTERFACE!{interface IStorageFolder(IStorageFolderVtbl, IStorageFolder_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolder] {
    fn CreateFileAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateFileAsync(&self, desiredName: HSTRING, options: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn CreateFolderAsync(&self, desiredName: HSTRING, options: CreationCollisionOption, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn GetFileAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFile> as RtType>::Abi) -> HRESULT,
    fn GetFolderAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn GetItemAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncOperation<IStorageItem> as RtType>::Abi) -> HRESULT,
    fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<StorageFile>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<StorageFolder>> as RtType>::Abi) -> HRESULT,
    fn GetItemsAsyncOverloadDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItem>> as RtType>::Abi) -> HRESULT
}}
impl IStorageFolder {
    #[inline] pub fn create_file_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_file_async(&self, desiredName: &HStringArg, options: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_async(&self, desiredName: &HStringArg, options: CreationCollisionOption) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file_async(&self, name: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFileAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder_async(&self, name: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFolderAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_async(&self, name: &HStringArg) -> Result<foundation::IAsyncOperation<IStorageItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async_overload_default_options_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async_overload_default_options_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_items_async_overload_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsyncOverloadDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageFolder: IStorageFolder}
impl RtActivatable<IStorageFolderStatics> for StorageFolder {}
impl StorageFolder {
    #[inline] pub fn get_folder_from_path_async(path: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> {
        <Self as RtActivatable<IStorageFolderStatics>>::get_activation_factory().get_folder_from_path_async(path)
    }
}
DEFINE_CLSID!(StorageFolder(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,70,111,108,100,101,114,0]) [CLSID_StorageFolder]);
DEFINE_IID!(IID_IStorageFolder2, 3894929593, 2265, 19086, 160, 172, 254, 94, 211, 203, 187, 211);
RT_INTERFACE!{interface IStorageFolder2(IStorageFolder2Vtbl, IStorageFolder2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolder2] {
    fn TryGetItemAsync(&self, name: HSTRING, out: *mut <foundation::IAsyncOperation<IStorageItem> as RtType>::Abi) -> HRESULT
}}
impl IStorageFolder2 {
    #[inline] pub fn try_get_item_async(&self, name: &HStringArg) -> Result<foundation::IAsyncOperation<IStorageItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetItemAsync)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFolder3, 2673965209, 48609, 16676, 174, 179, 176, 106, 217, 111, 152, 212);
RT_INTERFACE!{interface IStorageFolder3(IStorageFolder3Vtbl, IStorageFolder3_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolder3] {
    fn TryGetChangeTracker(&self, out: *mut <StorageLibraryChangeTracker as RtType>::Abi) -> HRESULT
}}
impl IStorageFolder3 {
    #[inline] pub fn try_get_change_tracker(&self) -> Result<Option<StorageLibraryChangeTracker>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetChangeTracker)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageLibraryChangeTracker::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFolderStatics, 150153215, 34261, 18617, 174, 233, 40, 81, 30, 51, 159, 159);
RT_INTERFACE!{static interface IStorageFolderStatics(IStorageFolderStaticsVtbl, IStorageFolderStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolderStatics] {
    fn GetFolderFromPathAsync(&self, path: HSTRING, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT
}}
impl IStorageFolderStatics {
    #[inline] pub fn get_folder_from_path_async(&self, path: &HStringArg) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFolderFromPathAsync)(self.get_abi() as *const _ as *mut _, path.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItem, 1107798422, 51759, 17143, 189, 232, 139, 16, 69, 122, 127, 48);
RT_INTERFACE!{interface IStorageItem(IStorageItemVtbl, IStorageItem_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItem] {
    fn RenameAsyncOverloadDefaultOptions(&self, desiredName: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn RenameAsync(&self, desiredName: HSTRING, option: NameCollisionOption, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn DeleteAsyncOverloadDefaultOptions(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn DeleteAsync(&self, option: StorageDeleteOption, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn GetBasicPropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<fileproperties::BasicProperties> as RtType>::Abi) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Attributes(&self, out: *mut FileAttributes) -> HRESULT,
    fn get_DateCreated(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn IsOfType(&self, type_: StorageItemTypes, out: *mut bool) -> HRESULT
}}
impl IStorageItem {
    #[inline] pub fn rename_async_overload_default_options(&self, desiredName: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RenameAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, desiredName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn rename_async(&self, desiredName: &HStringArg, option: NameCollisionOption) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RenameAsync)(self.get_abi() as *const _ as *mut _, desiredName.get(), option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_async_overload_default_options(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DeleteAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_async(&self, option: StorageDeleteOption) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DeleteAsync)(self.get_abi() as *const _ as *mut _, option, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_basic_properties_async(&self) -> Result<foundation::IAsyncOperation<fileproperties::BasicProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetBasicPropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Path)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attributes(&self) -> Result<FileAttributes> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Attributes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_date_created(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DateCreated)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_of_type(&self, type_: StorageItemTypes) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsOfType)(self.get_abi() as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItem2, 1408837330, 2108, 17027, 180, 91, 129, 192, 7, 35, 126, 68);
RT_INTERFACE!{interface IStorageItem2(IStorageItem2Vtbl, IStorageItem2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItem2] {
    fn GetParentAsync(&self, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn IsEqual(&self, item: <IStorageItem as RtType>::Abi, out: *mut bool) -> HRESULT
}}
impl IStorageItem2 {
    #[inline] pub fn get_parent_async(&self) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetParentAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_equal(&self, item: &IStorageItem) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsEqual)(self.get_abi() as *const _ as *mut _, item.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItemProperties, 2254849144, 32809, 18174, 167, 137, 28, 47, 62, 47, 251, 92);
RT_INTERFACE!{interface IStorageItemProperties(IStorageItemPropertiesVtbl, IStorageItemProperties_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemProperties] {
    fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: fileproperties::ThumbnailMode, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT,
    fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT,
    fn GetThumbnailAsync(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FolderRelativeId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut <fileproperties::StorageItemContentProperties as RtType>::Abi) -> HRESULT
}}
impl IStorageItemProperties {
    #[inline] pub fn get_thumbnail_async_overload_default_size_default_options(&self, mode: fileproperties::ThumbnailMode) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(self.get_abi() as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail_async_overload_default_options(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetThumbnailAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, mode, requestedSize, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail_async(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetThumbnailAsync)(self.get_abi() as *const _ as *mut _, mode, requestedSize, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder_relative_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FolderRelativeId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<fileproperties::StorageItemContentProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(fileproperties::StorageItemContentProperties::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItemProperties2, 2391189841, 1209, 19410, 146, 157, 254, 243, 247, 22, 33, 208);
RT_INTERFACE!{interface IStorageItemProperties2(IStorageItemProperties2Vtbl, IStorageItemProperties2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemProperties2] {
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: fileproperties::ThumbnailMode, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT,
    fn GetScaledImageAsThumbnailAsyncOverloadDefaultOptions(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT,
    fn GetScaledImageAsThumbnailAsync(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions, out: *mut <foundation::IAsyncOperation<fileproperties::StorageItemThumbnail> as RtType>::Abi) -> HRESULT
}}
impl IStorageItemProperties2 {
    #[inline] pub fn get_scaled_image_as_thumbnail_async_overload_default_size_default_options(&self, mode: fileproperties::ThumbnailMode) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetScaledImageAsThumbnailAsyncOverloadDefaultSizeDefaultOptions)(self.get_abi() as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scaled_image_as_thumbnail_async_overload_default_options(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetScaledImageAsThumbnailAsyncOverloadDefaultOptions)(self.get_abi() as *const _ as *mut _, mode, requestedSize, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scaled_image_as_thumbnail_async(&self, mode: fileproperties::ThumbnailMode, requestedSize: u32, options: fileproperties::ThumbnailOptions) -> Result<foundation::IAsyncOperation<fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetScaledImageAsThumbnailAsync)(self.get_abi() as *const _ as *mut _, mode, requestedSize, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItemPropertiesWithProvider, 2249978779, 25448, 19950, 180, 14, 116, 104, 74, 92, 231, 20);
RT_INTERFACE!{interface IStorageItemPropertiesWithProvider(IStorageItemPropertiesWithProviderVtbl, IStorageItemPropertiesWithProvider_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemPropertiesWithProvider] {
    fn get_Provider(&self, out: *mut <StorageProvider as RtType>::Abi) -> HRESULT
}}
impl IStorageItemPropertiesWithProvider {
    #[inline] pub fn get_provider(&self) -> Result<Option<StorageProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Provider)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageProvider::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorageItemTypes: u32 {
    None = 0, File = 1, Folder = 2,
}}
DEFINE_IID!(IID_IStorageLibrary, 517828867, 3678, 19820, 181, 232, 147, 24, 152, 61, 106, 3);
RT_INTERFACE!{interface IStorageLibrary(IStorageLibraryVtbl, IStorageLibrary_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibrary] {
    fn RequestAddFolderAsync(&self, out: *mut <foundation::IAsyncOperation<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn RequestRemoveFolderAsync(&self, folder: <StorageFolder as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn get_Folders(&self, out: *mut <foundation::collections::IObservableVector<StorageFolder> as RtType>::Abi) -> HRESULT,
    fn get_SaveFolder(&self, out: *mut <StorageFolder as RtType>::Abi) -> HRESULT,
    fn add_DefinitionChanged(&self, handler: <foundation::TypedEventHandler<StorageLibrary, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DefinitionChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageLibrary {
    #[inline] pub fn request_add_folder_async(&self) -> Result<foundation::IAsyncOperation<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RequestAddFolderAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_remove_folder_async(&self, folder: &StorageFolder) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RequestRemoveFolderAsync)(self.get_abi() as *const _ as *mut _, folder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders(&self) -> Result<Option<foundation::collections::IObservableVector<StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Folders)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IObservableVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_save_folder(&self) -> Result<Option<StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SaveFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_definition_changed(&self, handler: &foundation::TypedEventHandler<StorageLibrary, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DefinitionChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_definition_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DefinitionChanged)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibrary: IStorageLibrary}
impl RtActivatable<IStorageLibraryStatics> for StorageLibrary {}
impl RtActivatable<IStorageLibraryStatics2> for StorageLibrary {}
impl StorageLibrary {
    #[inline] pub fn get_library_async(libraryId: KnownLibraryId) -> Result<foundation::IAsyncOperation<StorageLibrary>> {
        <Self as RtActivatable<IStorageLibraryStatics>>::get_activation_factory().get_library_async(libraryId)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn get_library_for_user_async(user: &super::system::User, libraryId: KnownLibraryId) -> Result<foundation::IAsyncOperation<StorageLibrary>> {
        <Self as RtActivatable<IStorageLibraryStatics2>>::get_activation_factory().get_library_for_user_async(user, libraryId)
    }
}
DEFINE_CLSID!(StorageLibrary(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,111,114,97,103,101,76,105,98,114,97,114,121,0]) [CLSID_StorageLibrary]);
DEFINE_IID!(IID_IStorageLibrary2, 1527571272, 64691, 16433, 175, 176, 166, 141, 123, 212, 69, 52);
RT_INTERFACE!{interface IStorageLibrary2(IStorageLibrary2Vtbl, IStorageLibrary2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibrary2] {
    fn get_ChangeTracker(&self, out: *mut <StorageLibraryChangeTracker as RtType>::Abi) -> HRESULT
}}
impl IStorageLibrary2 {
    #[inline] pub fn get_change_tracker(&self) -> Result<Option<StorageLibraryChangeTracker>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ChangeTracker)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageLibraryChangeTracker::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageLibrary3, 2317882001, 8532, 16897, 129, 19, 210, 192, 92, 225, 173, 35);
RT_INTERFACE!{interface IStorageLibrary3(IStorageLibrary3Vtbl, IStorageLibrary3_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibrary3] {
    fn AreFolderSuggestionsAvailableAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IStorageLibrary3 {
    #[inline] pub fn are_folder_suggestions_available_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AreFolderSuggestionsAvailableAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageLibraryChange, 9964323, 11234, 18697, 170, 72, 21, 159, 82, 3, 165, 30);
RT_INTERFACE!{interface IStorageLibraryChange(IStorageLibraryChangeVtbl, IStorageLibraryChange_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChange] {
    fn get_ChangeType(&self, out: *mut StorageLibraryChangeType) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PreviousPath(&self, out: *mut HSTRING) -> HRESULT,
    fn IsOfType(&self, type_: StorageItemTypes, out: *mut bool) -> HRESULT,
    fn GetStorageItemAsync(&self, out: *mut <foundation::IAsyncOperation<IStorageItem> as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryChange {
    #[inline] pub fn get_change_type(&self) -> Result<StorageLibraryChangeType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ChangeType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Path)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_previous_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PreviousPath)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn is_of_type(&self, type_: StorageItemTypes) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsOfType)(self.get_abi() as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_storage_item_async(&self) -> Result<foundation::IAsyncOperation<IStorageItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetStorageItemAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibraryChange: IStorageLibraryChange}
DEFINE_IID!(IID_IStorageLibraryChangeReader, 4060462211, 64674, 16889, 137, 84, 238, 46, 153, 30, 185, 111);
RT_INTERFACE!{interface IStorageLibraryChangeReader(IStorageLibraryChangeReaderVtbl, IStorageLibraryChangeReader_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChangeReader] {
    fn ReadBatchAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<StorageLibraryChange>> as RtType>::Abi) -> HRESULT,
    fn AcceptChangesAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryChangeReader {
    #[inline] pub fn read_batch_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<StorageLibraryChange>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadBatchAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn accept_changes_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AcceptChangesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibraryChangeReader: IStorageLibraryChangeReader}
DEFINE_IID!(IID_IStorageLibraryChangeTracker, 2652205846, 24691, 17654, 150, 129, 116, 146, 209, 40, 108, 144);
RT_INTERFACE!{interface IStorageLibraryChangeTracker(IStorageLibraryChangeTrackerVtbl, IStorageLibraryChangeTracker_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChangeTracker] {
    fn GetChangeReader(&self, out: *mut <StorageLibraryChangeReader as RtType>::Abi) -> HRESULT,
    fn Enable(&self) -> HRESULT,
    fn Reset(&self) -> HRESULT
}}
impl IStorageLibraryChangeTracker {
    #[inline] pub fn get_change_reader(&self) -> Result<Option<StorageLibraryChangeReader>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetChangeReader)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageLibraryChangeReader::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn enable(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Enable)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn reset(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Reset)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibraryChangeTracker: IStorageLibraryChangeTracker}
RT_ENUM! { enum StorageLibraryChangeType: i32 {
    Created = 0, Deleted = 1, MovedOrRenamed = 2, ContentsChanged = 3, MovedOutOfLibrary = 4, MovedIntoLibrary = 5, ContentsReplaced = 6, IndexingStatusChanged = 7, EncryptionChanged = 8, ChangeTrackingLost = 9,
}}
DEFINE_IID!(IID_IStorageLibraryStatics, 1107863259, 26698, 18886, 158, 89, 144, 18, 30, 224, 80, 214);
RT_INTERFACE!{static interface IStorageLibraryStatics(IStorageLibraryStaticsVtbl, IStorageLibraryStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryStatics] {
    fn GetLibraryAsync(&self, libraryId: KnownLibraryId, out: *mut <foundation::IAsyncOperation<StorageLibrary> as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryStatics {
    #[inline] pub fn get_library_async(&self, libraryId: KnownLibraryId) -> Result<foundation::IAsyncOperation<StorageLibrary>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetLibraryAsync)(self.get_abi() as *const _ as *mut _, libraryId, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageLibraryStatics2, 4289760732, 64117, 18069, 185, 209, 127, 129, 249, 120, 50, 227);
RT_INTERFACE!{static interface IStorageLibraryStatics2(IStorageLibraryStatics2Vtbl, IStorageLibraryStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryStatics2] {
    #[cfg(feature="windows-system")] fn GetLibraryForUserAsync(&self, user: <super::system::User as RtType>::Abi, libraryId: KnownLibraryId, out: *mut <foundation::IAsyncOperation<StorageLibrary> as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryStatics2 {
    #[cfg(feature="windows-system")] #[inline] pub fn get_library_for_user_async(&self, user: &super::system::User, libraryId: KnownLibraryId) -> Result<foundation::IAsyncOperation<StorageLibrary>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetLibraryForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, libraryId, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorageOpenOptions: u32 {
    None = 0, AllowOnlyReaders = 1, AllowReadersAndWriters = 2,
}}
DEFINE_IID!(IID_IStorageProvider, 3875925716, 54392, 18390, 186, 70, 26, 142, 190, 17, 74, 32);
RT_INTERFACE!{interface IStorageProvider(IStorageProviderVtbl, IStorageProvider_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProvider] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStorageProvider {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProvider: IStorageProvider}
DEFINE_IID!(IID_IStorageProvider2, 17635607, 13316, 16715, 159, 215, 205, 68, 71, 46, 170, 57);
RT_INTERFACE!{interface IStorageProvider2(IStorageProvider2Vtbl, IStorageProvider2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProvider2] {
    fn IsPropertySupportedForPartialFileAsync(&self, propertyCanonicalName: HSTRING, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IStorageProvider2 {
    #[inline] pub fn is_property_supported_for_partial_file_async(&self, propertyCanonicalName: &HStringArg) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).IsPropertySupportedForPartialFileAsync)(self.get_abi() as *const _ as *mut _, propertyCanonicalName.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageStreamTransaction, 4135383907, 42301, 19860, 174, 44, 103, 35, 45, 147, 172, 221);
RT_INTERFACE!{interface IStorageStreamTransaction(IStorageStreamTransactionVtbl, IStorageStreamTransaction_Abi): IInspectable(IInspectableVtbl) [IID_IStorageStreamTransaction] {
    fn get_Stream(&self, out: *mut <streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn CommitAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IStorageStreamTransaction {
    #[inline] pub fn get_stream(&self) -> Result<Option<streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Stream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn commit_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CommitAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageStreamTransaction: IStorageStreamTransaction}
DEFINE_IID!(IID_IStreamedFileDataRequest, 376700110, 55997, 19792, 190, 238, 24, 11, 138, 129, 145, 182);
RT_INTERFACE!{interface IStreamedFileDataRequest(IStreamedFileDataRequestVtbl, IStreamedFileDataRequest_Abi): IInspectable(IInspectableVtbl) [IID_IStreamedFileDataRequest] {
    fn FailAndClose(&self, failureMode: StreamedFileFailureMode) -> HRESULT
}}
impl IStreamedFileDataRequest {
    #[inline] pub fn fail_and_close(&self, failureMode: StreamedFileFailureMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).FailAndClose)(self.get_abi() as *const _ as *mut _, failureMode);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StreamedFileDataRequest: streams::IOutputStream}
DEFINE_IID!(IID_StreamedFileDataRequestedHandler, 4277577764, 12257, 19719, 163, 91, 183, 124, 80, 181, 244, 204);
RT_DELEGATE!{delegate StreamedFileDataRequestedHandler(StreamedFileDataRequestedHandlerVtbl, StreamedFileDataRequestedHandler_Abi, StreamedFileDataRequestedHandlerImpl) [IID_StreamedFileDataRequestedHandler] {
    fn Invoke(&self, stream: <StreamedFileDataRequest as RtType>::Abi) -> HRESULT
}}
impl StreamedFileDataRequestedHandler {
    #[inline] pub fn invoke(&self, stream: &StreamedFileDataRequest) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum StreamedFileFailureMode: i32 {
    Failed = 0, CurrentlyUnavailable = 1, Incomplete = 2,
}}
DEFINE_IID!(IID_ISystemAudioProperties, 1066350775, 12428, 18401, 146, 77, 134, 69, 52, 142, 93, 183);
RT_INTERFACE!{interface ISystemAudioProperties(ISystemAudioPropertiesVtbl, ISystemAudioProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemAudioProperties] {
    fn get_EncodingBitrate(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemAudioProperties {
    #[inline] pub fn get_encoding_bitrate(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_EncodingBitrate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemAudioProperties: ISystemAudioProperties}
DEFINE_IID!(IID_ISystemDataPaths, 3811229552, 55546, 17900, 169, 66, 210, 226, 111, 182, 11, 165);
RT_INTERFACE!{interface ISystemDataPaths(ISystemDataPathsVtbl, ISystemDataPaths_Abi): IInspectable(IInspectableVtbl) [IID_ISystemDataPaths] {
    fn get_Fonts(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ProgramData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Public(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDesktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDocuments(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicDownloads(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicMusic(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicPictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PublicVideos(&self, out: *mut HSTRING) -> HRESULT,
    fn get_System(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemHost(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemX86(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemX64(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SystemArm(&self, out: *mut HSTRING) -> HRESULT,
    fn get_UserProfiles(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Windows(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemDataPaths {
    #[inline] pub fn get_fonts(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Fonts)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_program_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ProgramData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Public)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_desktop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicDesktop)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_documents(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicDocuments)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_downloads(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicDownloads)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_music(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicMusic)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_pictures(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicPictures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_public_videos(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicVideos)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_System)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_host(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemHost)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_x86(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemX86)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_x64(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemX64)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_arm(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemArm)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_user_profiles(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_UserProfiles)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_windows(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Windows)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemDataPaths: ISystemDataPaths}
impl RtActivatable<ISystemDataPathsStatics> for SystemDataPaths {}
impl SystemDataPaths {
    #[inline] pub fn get_default() -> Result<Option<SystemDataPaths>> {
        <Self as RtActivatable<ISystemDataPathsStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(SystemDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,121,115,116,101,109,68,97,116,97,80,97,116,104,115,0]) [CLSID_SystemDataPaths]);
DEFINE_IID!(IID_ISystemDataPathsStatics, 3774443472, 39200, 19402, 179, 121, 249, 111, 223, 124, 170, 216);
RT_INTERFACE!{static interface ISystemDataPathsStatics(ISystemDataPathsStaticsVtbl, ISystemDataPathsStatics_Abi): IInspectable(IInspectableVtbl) [IID_ISystemDataPathsStatics] {
    fn GetDefault(&self, out: *mut <SystemDataPaths as RtType>::Abi) -> HRESULT
}}
impl ISystemDataPathsStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<SystemDataPaths>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemDataPaths::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISystemGPSProperties, 3237244596, 49524, 18458, 188, 37, 146, 25, 134, 246, 166, 243);
RT_INTERFACE!{interface ISystemGPSProperties(ISystemGPSPropertiesVtbl, ISystemGPSProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemGPSProperties] {
    fn get_LatitudeDecimal(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LongitudeDecimal(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemGPSProperties {
    #[inline] pub fn get_latitude_decimal(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LatitudeDecimal)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_longitude_decimal(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LongitudeDecimal)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemGPSProperties: ISystemGPSProperties}
DEFINE_IID!(IID_ISystemImageProperties, 18558512, 35641, 17160, 190, 161, 232, 170, 97, 228, 120, 38);
RT_INTERFACE!{interface ISystemImageProperties(ISystemImagePropertiesVtbl, ISystemImageProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemImageProperties] {
    fn get_HorizontalSize(&self, out: *mut HSTRING) -> HRESULT,
    fn get_VerticalSize(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemImageProperties {
    #[inline] pub fn get_horizontal_size(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_HorizontalSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vertical_size(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_VerticalSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemImageProperties: ISystemImageProperties}
DEFINE_IID!(IID_ISystemMediaProperties, 2754294550, 33813, 16604, 140, 68, 152, 54, 29, 35, 84, 48);
RT_INTERFACE!{interface ISystemMediaProperties(ISystemMediaPropertiesVtbl, ISystemMediaProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemMediaProperties] {
    fn get_Duration(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Producer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SubTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Writer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Year(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemMediaProperties {
    #[inline] pub fn get_duration(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Duration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_producer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Producer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_publisher(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Publisher)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sub_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SubTitle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_writer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Writer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_year(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Year)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemMediaProperties: ISystemMediaProperties}
DEFINE_IID!(IID_ISystemMusicProperties, 3027863765, 26543, 19395, 141, 57, 91, 137, 2, 32, 38, 161);
RT_INTERFACE!{interface ISystemMusicProperties(ISystemMusicPropertiesVtbl, ISystemMusicProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemMusicProperties] {
    fn get_AlbumArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_AlbumTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Artist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Composer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Conductor(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DisplayArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Genre(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TrackNumber(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemMusicProperties {
    #[inline] pub fn get_album_artist(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AlbumArtist)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_album_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AlbumTitle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_artist(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Artist)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_composer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Composer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_conductor(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Conductor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_artist(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayArtist)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_genre(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Genre)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_track_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TrackNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemMusicProperties: ISystemMusicProperties}
DEFINE_IID!(IID_ISystemPhotoProperties, 1194654781, 43809, 17444, 183, 53, 244, 53, 58, 86, 200, 252);
RT_INTERFACE!{interface ISystemPhotoProperties(ISystemPhotoPropertiesVtbl, ISystemPhotoProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemPhotoProperties] {
    fn get_CameraManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn get_CameraModel(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DateTaken(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PeopleNames(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemPhotoProperties {
    #[inline] pub fn get_camera_manufacturer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraManufacturer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_camera_model(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraModel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_date_taken(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DateTaken)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_people_names(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PeopleNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemPhotoProperties: ISystemPhotoProperties}
DEFINE_IID!(IID_ISystemProperties, 2440720833, 34291, 19921, 176, 1, 165, 11, 253, 33, 200, 210);
RT_INTERFACE!{static interface ISystemProperties(ISystemPropertiesVtbl, ISystemProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemProperties] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ItemNameDisplay(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Keywords(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Rating(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Audio(&self, out: *mut <SystemAudioProperties as RtType>::Abi) -> HRESULT,
    fn get_GPS(&self, out: *mut <SystemGPSProperties as RtType>::Abi) -> HRESULT,
    fn get_Media(&self, out: *mut <SystemMediaProperties as RtType>::Abi) -> HRESULT,
    fn get_Music(&self, out: *mut <SystemMusicProperties as RtType>::Abi) -> HRESULT,
    fn get_Photo(&self, out: *mut <SystemPhotoProperties as RtType>::Abi) -> HRESULT,
    fn get_Video(&self, out: *mut <SystemVideoProperties as RtType>::Abi) -> HRESULT,
    fn get_Image(&self, out: *mut <SystemImageProperties as RtType>::Abi) -> HRESULT
}}
impl ISystemProperties {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Author)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_comment(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Comment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_name_display(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ItemNameDisplay)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_keywords(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Keywords)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rating(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Rating)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_audio(&self) -> Result<Option<SystemAudioProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Audio)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemAudioProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_gps(&self) -> Result<Option<SystemGPSProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_GPS)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemGPSProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_media(&self) -> Result<Option<SystemMediaProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Media)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemMediaProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_music(&self) -> Result<Option<SystemMusicProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Music)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemMusicProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_photo(&self) -> Result<Option<SystemPhotoProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Photo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemPhotoProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_video(&self) -> Result<Option<SystemVideoProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Video)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemVideoProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image(&self) -> Result<Option<SystemImageProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Image)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SystemImageProperties::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class SystemProperties}
impl RtActivatable<ISystemProperties> for SystemProperties {}
impl SystemProperties {
    #[inline] pub fn get_author() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_author()
    }
    #[inline] pub fn get_comment() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_comment()
    }
    #[inline] pub fn get_item_name_display() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_item_name_display()
    }
    #[inline] pub fn get_keywords() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_keywords()
    }
    #[inline] pub fn get_rating() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_rating()
    }
    #[inline] pub fn get_title() -> Result<HString> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_title()
    }
    #[inline] pub fn get_audio() -> Result<Option<SystemAudioProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_audio()
    }
    #[inline] pub fn get_gps() -> Result<Option<SystemGPSProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_gps()
    }
    #[inline] pub fn get_media() -> Result<Option<SystemMediaProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_media()
    }
    #[inline] pub fn get_music() -> Result<Option<SystemMusicProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_music()
    }
    #[inline] pub fn get_photo() -> Result<Option<SystemPhotoProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_photo()
    }
    #[inline] pub fn get_video() -> Result<Option<SystemVideoProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_video()
    }
    #[inline] pub fn get_image() -> Result<Option<SystemImageProperties>> {
        <Self as RtActivatable<ISystemProperties>>::get_activation_factory().get_image()
    }
}
DEFINE_CLSID!(SystemProperties(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,121,115,116,101,109,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_SystemProperties]);
DEFINE_IID!(IID_ISystemVideoProperties, 541128469, 26616, 17186, 155, 128, 79, 169, 254, 251, 131, 232);
RT_INTERFACE!{interface ISystemVideoProperties(ISystemVideoPropertiesVtbl, ISystemVideoProperties_Abi): IInspectable(IInspectableVtbl) [IID_ISystemVideoProperties] {
    fn get_Director(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrameHeight(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FrameWidth(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_TotalBitrate(&self, out: *mut HSTRING) -> HRESULT
}}
impl ISystemVideoProperties {
    #[inline] pub fn get_director(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Director)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_height(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FrameHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_width(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FrameWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_total_bitrate(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TotalBitrate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SystemVideoProperties: ISystemVideoProperties}
DEFINE_IID!(IID_IUserDataPaths, 4190451986, 43972, 18175, 138, 43, 220, 157, 127, 166, 229, 47);
RT_INTERFACE!{interface IUserDataPaths(IUserDataPathsVtbl, IUserDataPaths_Abi): IInspectable(IInspectableVtbl) [IID_IUserDataPaths] {
    fn get_CameraRoll(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Cookies(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Desktop(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Documents(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Downloads(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Favorites(&self, out: *mut HSTRING) -> HRESULT,
    fn get_History(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InternetCache(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LocalAppDataLow(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Music(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Profile(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Recent(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RoamingAppData(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SavedPictures(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Screenshots(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Templates(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Videos(&self, out: *mut HSTRING) -> HRESULT
}}
impl IUserDataPaths {
    #[inline] pub fn get_camera_roll(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraRoll)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_cookies(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Cookies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_desktop(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Desktop)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_documents(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Documents)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_downloads(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Downloads)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_favorites(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Favorites)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_history(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_History)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_internet_cache(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_InternetCache)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_app_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalAppData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_app_data_low(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalAppDataLow)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_music(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Music)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pictures(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Pictures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_profile(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Profile)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_recent(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Recent)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_roaming_app_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RoamingAppData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_saved_pictures(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SavedPictures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_screenshots(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Screenshots)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_templates(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Templates)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_videos(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Videos)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class UserDataPaths: IUserDataPaths}
impl RtActivatable<IUserDataPathsStatics> for UserDataPaths {}
impl UserDataPaths {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(user: &super::system::User) -> Result<Option<UserDataPaths>> {
        <Self as RtActivatable<IUserDataPathsStatics>>::get_activation_factory().get_for_user(user)
    }
    #[inline] pub fn get_default() -> Result<Option<UserDataPaths>> {
        <Self as RtActivatable<IUserDataPathsStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(UserDataPaths(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,85,115,101,114,68,97,116,97,80,97,116,104,115,0]) [CLSID_UserDataPaths]);
DEFINE_IID!(IID_IUserDataPathsStatics, 28483055, 57442, 18593, 139, 12, 242, 199, 169, 202, 86, 192);
RT_INTERFACE!{static interface IUserDataPathsStatics(IUserDataPathsStaticsVtbl, IUserDataPathsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IUserDataPathsStatics] {
    #[cfg(not(feature="windows-system"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-system")] fn GetForUser(&self, user: <super::system::User as RtType>::Abi, out: *mut <UserDataPaths as RtType>::Abi) -> HRESULT,
    fn GetDefault(&self, out: *mut <UserDataPaths as RtType>::Abi) -> HRESULT
}}
impl IUserDataPathsStatics {
    #[cfg(feature="windows-system")] #[inline] pub fn get_for_user(&self, user: &super::system::User) -> Result<Option<UserDataPaths>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForUser)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(UserDataPaths::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default(&self) -> Result<Option<UserDataPaths>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(UserDataPaths::wrap(out)) } else { err(hr) }
    }}
}
pub mod accesscache { // Windows.Storage.AccessCache
use crate::prelude::*;
RT_ENUM! { enum AccessCacheOptions: u32 {
    None = 0, DisallowUserInput = 1, FastLocationsOnly = 2, UseReadOnlyCachedCopy = 4, SuppressAccessTimeUpdate = 8,
}}
RT_STRUCT! { struct AccessListEntry {
    Token: HSTRING, Metadata: HSTRING,
}}
RT_CLASS!{class AccessListEntryView: foundation::collections::IVectorView<AccessListEntry>}
DEFINE_IID!(IID_IItemRemovedEventArgs, 1499954780, 21950, 19558, 186, 102, 94, 174, 167, 157, 38, 49);
RT_INTERFACE!{interface IItemRemovedEventArgs(IItemRemovedEventArgsVtbl, IItemRemovedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IItemRemovedEventArgs] {
    fn get_RemovedEntry(&self, out: *mut AccessListEntry) -> HRESULT
}}
impl IItemRemovedEventArgs {
    #[inline] pub fn get_removed_entry(&self) -> Result<AccessListEntry> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RemovedEntry)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ItemRemovedEventArgs: IItemRemovedEventArgs}
RT_ENUM! { enum RecentStorageItemVisibility: i32 {
    AppOnly = 0, AppAndSystem = 1,
}}
RT_CLASS!{static class StorageApplicationPermissions}
impl RtActivatable<IStorageApplicationPermissionsStatics> for StorageApplicationPermissions {}
impl StorageApplicationPermissions {
    #[inline] pub fn get_future_access_list() -> Result<Option<StorageItemAccessList>> {
        <Self as RtActivatable<IStorageApplicationPermissionsStatics>>::get_activation_factory().get_future_access_list()
    }
    #[inline] pub fn get_most_recently_used_list() -> Result<Option<StorageItemMostRecentlyUsedList>> {
        <Self as RtActivatable<IStorageApplicationPermissionsStatics>>::get_activation_factory().get_most_recently_used_list()
    }
}
DEFINE_CLSID!(StorageApplicationPermissions(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,65,99,99,101,115,115,67,97,99,104,101,46,83,116,111,114,97,103,101,65,112,112,108,105,99,97,116,105,111,110,80,101,114,109,105,115,115,105,111,110,115,0]) [CLSID_StorageApplicationPermissions]);
DEFINE_IID!(IID_IStorageApplicationPermissionsStatics, 1133633450, 53299, 18681, 128, 96, 62, 200, 71, 210, 227, 241);
RT_INTERFACE!{static interface IStorageApplicationPermissionsStatics(IStorageApplicationPermissionsStaticsVtbl, IStorageApplicationPermissionsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageApplicationPermissionsStatics] {
    fn get_FutureAccessList(&self, out: *mut <StorageItemAccessList as RtType>::Abi) -> HRESULT,
    fn get_MostRecentlyUsedList(&self, out: *mut <StorageItemMostRecentlyUsedList as RtType>::Abi) -> HRESULT
}}
impl IStorageApplicationPermissionsStatics {
    #[inline] pub fn get_future_access_list(&self) -> Result<Option<StorageItemAccessList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FutureAccessList)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageItemAccessList::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_most_recently_used_list(&self) -> Result<Option<StorageItemMostRecentlyUsedList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MostRecentlyUsedList)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageItemMostRecentlyUsedList::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageItemAccessList, 749729453, 56976, 18421, 178, 195, 221, 54, 201, 253, 212, 83);
RT_INTERFACE!{interface IStorageItemAccessList(IStorageItemAccessListVtbl, IStorageItemAccessList_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemAccessList] {
    fn AddOverloadDefaultMetadata(&self, file: <super::IStorageItem as RtType>::Abi, out: *mut HSTRING) -> HRESULT,
    fn Add(&self, file: <super::IStorageItem as RtType>::Abi, metadata: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn AddOrReplaceOverloadDefaultMetadata(&self, token: HSTRING, file: <super::IStorageItem as RtType>::Abi) -> HRESULT,
    fn AddOrReplace(&self, token: HSTRING, file: <super::IStorageItem as RtType>::Abi, metadata: HSTRING) -> HRESULT,
    fn GetItemAsync(&self, token: HSTRING, out: *mut <foundation::IAsyncOperation<super::IStorageItem> as RtType>::Abi) -> HRESULT,
    fn GetFileAsync(&self, token: HSTRING, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT,
    fn GetFolderAsync(&self, token: HSTRING, out: *mut <foundation::IAsyncOperation<super::StorageFolder> as RtType>::Abi) -> HRESULT,
    fn GetItemWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut <foundation::IAsyncOperation<super::IStorageItem> as RtType>::Abi) -> HRESULT,
    fn GetFileWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT,
    fn GetFolderWithOptionsAsync(&self, token: HSTRING, options: AccessCacheOptions, out: *mut <foundation::IAsyncOperation<super::StorageFolder> as RtType>::Abi) -> HRESULT,
    fn Remove(&self, token: HSTRING) -> HRESULT,
    fn ContainsItem(&self, token: HSTRING, out: *mut bool) -> HRESULT,
    fn Clear(&self) -> HRESULT,
    fn CheckAccess(&self, file: <super::IStorageItem as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn get_Entries(&self, out: *mut <AccessListEntryView as RtType>::Abi) -> HRESULT,
    fn get_MaximumItemsAllowed(&self, out: *mut u32) -> HRESULT
}}
impl IStorageItemAccessList {
    #[inline] pub fn add_overload_default_metadata(&self, file: &super::IStorageItem) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AddOverloadDefaultMetadata)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add(&self, file: &super::IStorageItem, metadata: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Add)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, metadata.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_or_replace_overload_default_metadata(&self, token: &HStringArg, file: &super::IStorageItem) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AddOrReplaceOverloadDefaultMetadata)(self.get_abi() as *const _ as *mut _, token.get(), file.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_or_replace(&self, token: &HStringArg, file: &super::IStorageItem, metadata: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AddOrReplace)(self.get_abi() as *const _ as *mut _, token.get(), file.get_abi() as *const _ as *mut _, metadata.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_item_async(&self, token: &HStringArg) -> Result<foundation::IAsyncOperation<super::IStorageItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemAsync)(self.get_abi() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file_async(&self, token: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFileAsync)(self.get_abi() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder_async(&self, token: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFolderAsync)(self.get_abi() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_item_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<foundation::IAsyncOperation<super::IStorageItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemWithOptionsAsync)(self.get_abi() as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFileWithOptionsAsync)(self.get_abi() as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder_with_options_async(&self, token: &HStringArg, options: AccessCacheOptions) -> Result<foundation::IAsyncOperation<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFolderWithOptionsAsync)(self.get_abi() as *const _ as *mut _, token.get(), options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove(&self, token: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Remove)(self.get_abi() as *const _ as *mut _, token.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn contains_item(&self, token: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ContainsItem)(self.get_abi() as *const _ as *mut _, token.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Clear)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn check_access(&self, file: &super::IStorageItem) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).CheckAccess)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_entries(&self) -> Result<Option<AccessListEntryView>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Entries)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(AccessListEntryView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_maximum_items_allowed(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaximumItemsAllowed)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageItemAccessList: IStorageItemAccessList}
DEFINE_IID!(IID_IStorageItemMostRecentlyUsedList, 23214549, 20749, 16670, 140, 241, 195, 209, 239, 250, 76, 51);
RT_INTERFACE!{interface IStorageItemMostRecentlyUsedList(IStorageItemMostRecentlyUsedListVtbl, IStorageItemMostRecentlyUsedList_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemMostRecentlyUsedList] {
    fn add_ItemRemoved(&self, handler: <foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ItemRemoved(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageItemMostRecentlyUsedList {
    #[inline] pub fn add_item_removed(&self, handler: &foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_ItemRemoved)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_item_removed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_ItemRemoved)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageItemMostRecentlyUsedList: IStorageItemMostRecentlyUsedList}
DEFINE_IID!(IID_IStorageItemMostRecentlyUsedList2, 3662159520, 60813, 18225, 161, 219, 228, 78, 226, 32, 64, 147);
RT_INTERFACE!{interface IStorageItemMostRecentlyUsedList2(IStorageItemMostRecentlyUsedList2Vtbl, IStorageItemMostRecentlyUsedList2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemMostRecentlyUsedList2] {
    fn AddWithMetadataAndVisibility(&self, file: <super::IStorageItem as RtType>::Abi, metadata: HSTRING, visibility: RecentStorageItemVisibility, out: *mut HSTRING) -> HRESULT,
    fn AddOrReplaceWithMetadataAndVisibility(&self, token: HSTRING, file: <super::IStorageItem as RtType>::Abi, metadata: HSTRING, visibility: RecentStorageItemVisibility) -> HRESULT
}}
impl IStorageItemMostRecentlyUsedList2 {
    #[inline] pub fn add_with_metadata_and_visibility(&self, file: &super::IStorageItem, metadata: &HStringArg, visibility: RecentStorageItemVisibility) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AddWithMetadataAndVisibility)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, metadata.get(), visibility, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_or_replace_with_metadata_and_visibility(&self, token: &HStringArg, file: &super::IStorageItem, metadata: &HStringArg, visibility: RecentStorageItemVisibility) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AddOrReplaceWithMetadataAndVisibility)(self.get_abi() as *const _ as *mut _, token.get(), file.get_abi() as *const _ as *mut _, metadata.get(), visibility);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Storage.AccessCache
pub mod bulkaccess { // Windows.Storage.BulkAccess
use crate::prelude::*;
RT_CLASS!{class FileInformation: IStorageItemInformation}
DEFINE_IID!(IID_IFileInformationFactory, 1075677374, 38415, 19821, 167, 208, 26, 56, 97, 231, 108, 131);
RT_INTERFACE!{interface IFileInformationFactory(IFileInformationFactoryVtbl, IFileInformationFactory_Abi): IInspectable(IInspectableVtbl) [IID_IFileInformationFactory] {
    fn GetItemsAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItemInformation>> as RtType>::Abi) -> HRESULT,
    fn GetItemsAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItemInformation>> as RtType>::Abi) -> HRESULT,
    fn GetFilesAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<FileInformation>> as RtType>::Abi) -> HRESULT,
    fn GetFilesAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<FileInformation>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<FolderInformation>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<FolderInformation>> as RtType>::Abi) -> HRESULT,
    fn GetVirtualizedItemsVector(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn GetVirtualizedFilesVector(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn GetVirtualizedFoldersVector(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IFileInformationFactory {
    #[inline] pub fn get_items_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItemInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_items_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<IStorageItemInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<FileInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<FileInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<FolderInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<FolderInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_virtualized_items_vector(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVirtualizedItemsVector)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_virtualized_files_vector(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVirtualizedFilesVector)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_virtualized_folders_vector(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVirtualizedFoldersVector)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileInformationFactory: IFileInformationFactory}
impl RtActivatable<IFileInformationFactoryFactory> for FileInformationFactory {}
impl FileInformationFactory {
    #[inline] pub fn create_with_mode(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode) -> Result<FileInformationFactory> {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode(queryResult, mode)
    }
    #[inline] pub fn create_with_mode_and_size(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32) -> Result<FileInformationFactory> {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size(queryResult, mode, requestedThumbnailSize)
    }
    #[inline] pub fn create_with_mode_and_size_and_options(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions) -> Result<FileInformationFactory> {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size_and_options(queryResult, mode, requestedThumbnailSize, thumbnailOptions)
    }
    #[inline] pub fn create_with_mode_and_size_and_options_and_flags(queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool) -> Result<FileInformationFactory> {
        <Self as RtActivatable<IFileInformationFactoryFactory>>::get_activation_factory().create_with_mode_and_size_and_options_and_flags(queryResult, mode, requestedThumbnailSize, thumbnailOptions, delayLoad)
    }
}
DEFINE_CLSID!(FileInformationFactory(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,66,117,108,107,65,99,99,101,115,115,46,70,105,108,101,73,110,102,111,114,109,97,116,105,111,110,70,97,99,116,111,114,121,0]) [CLSID_FileInformationFactory]);
DEFINE_IID!(IID_IFileInformationFactoryFactory, 2229931645, 58530, 20224, 138, 250, 175, 94, 15, 130, 107, 213);
RT_INTERFACE!{static interface IFileInformationFactoryFactory(IFileInformationFactoryFactoryVtbl, IFileInformationFactoryFactory_Abi): IInspectable(IInspectableVtbl) [IID_IFileInformationFactoryFactory] {
    fn CreateWithMode(&self, queryResult: <super::search::IStorageQueryResultBase as RtType>::Abi, mode: super::fileproperties::ThumbnailMode, out: *mut <FileInformationFactory as RtType>::Abi) -> HRESULT,
    fn CreateWithModeAndSize(&self, queryResult: <super::search::IStorageQueryResultBase as RtType>::Abi, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, out: *mut <FileInformationFactory as RtType>::Abi) -> HRESULT,
    fn CreateWithModeAndSizeAndOptions(&self, queryResult: <super::search::IStorageQueryResultBase as RtType>::Abi, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, out: *mut <FileInformationFactory as RtType>::Abi) -> HRESULT,
    fn CreateWithModeAndSizeAndOptionsAndFlags(&self, queryResult: <super::search::IStorageQueryResultBase as RtType>::Abi, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool, out: *mut <FileInformationFactory as RtType>::Abi) -> HRESULT
}}
impl IFileInformationFactoryFactory {
    #[inline] pub fn create_with_mode(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode) -> Result<FileInformationFactory> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithMode)(self.get_abi() as *const _ as *mut _, queryResult.get_abi() as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(FileInformationFactory::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_mode_and_size(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32) -> Result<FileInformationFactory> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithModeAndSize)(self.get_abi() as *const _ as *mut _, queryResult.get_abi() as *const _ as *mut _, mode, requestedThumbnailSize, &mut out);
        if hr == S_OK { Ok(FileInformationFactory::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_mode_and_size_and_options(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions) -> Result<FileInformationFactory> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithModeAndSizeAndOptions)(self.get_abi() as *const _ as *mut _, queryResult.get_abi() as *const _ as *mut _, mode, requestedThumbnailSize, thumbnailOptions, &mut out);
        if hr == S_OK { Ok(FileInformationFactory::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_mode_and_size_and_options_and_flags(&self, queryResult: &super::search::IStorageQueryResultBase, mode: super::fileproperties::ThumbnailMode, requestedThumbnailSize: u32, thumbnailOptions: super::fileproperties::ThumbnailOptions, delayLoad: bool) -> Result<FileInformationFactory> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithModeAndSizeAndOptionsAndFlags)(self.get_abi() as *const _ as *mut _, queryResult.get_abi() as *const _ as *mut _, mode, requestedThumbnailSize, thumbnailOptions, delayLoad, &mut out);
        if hr == S_OK { Ok(FileInformationFactory::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FolderInformation: IStorageItemInformation}
DEFINE_IID!(IID_IStorageItemInformation, 2275789707, 35186, 20288, 141, 224, 216, 111, 177, 121, 216, 250);
RT_INTERFACE!{interface IStorageItemInformation(IStorageItemInformationVtbl, IStorageItemInformation_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemInformation] {
    fn get_MusicProperties(&self, out: *mut <super::fileproperties::MusicProperties as RtType>::Abi) -> HRESULT,
    fn get_VideoProperties(&self, out: *mut <super::fileproperties::VideoProperties as RtType>::Abi) -> HRESULT,
    fn get_ImageProperties(&self, out: *mut <super::fileproperties::ImageProperties as RtType>::Abi) -> HRESULT,
    fn get_DocumentProperties(&self, out: *mut <super::fileproperties::DocumentProperties as RtType>::Abi) -> HRESULT,
    fn get_BasicProperties(&self, out: *mut <super::fileproperties::BasicProperties as RtType>::Abi) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut <super::fileproperties::StorageItemThumbnail as RtType>::Abi) -> HRESULT,
    fn add_ThumbnailUpdated(&self, changedHandler: <foundation::TypedEventHandler<IStorageItemInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ThumbnailUpdated(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_PropertiesUpdated(&self, changedHandler: <foundation::TypedEventHandler<IStorageItemInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PropertiesUpdated(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IStorageItemInformation {
    #[inline] pub fn get_music_properties(&self) -> Result<Option<super::fileproperties::MusicProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MusicProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::MusicProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_video_properties(&self) -> Result<Option<super::fileproperties::VideoProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_VideoProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::VideoProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_properties(&self) -> Result<Option<super::fileproperties::ImageProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ImageProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::ImageProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_properties(&self) -> Result<Option<super::fileproperties::DocumentProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::DocumentProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_basic_properties(&self) -> Result<Option<super::fileproperties::BasicProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BasicProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::BasicProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail(&self) -> Result<Option<super::fileproperties::StorageItemThumbnail>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Thumbnail)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::fileproperties::StorageItemThumbnail::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_thumbnail_updated(&self, changedHandler: &foundation::TypedEventHandler<IStorageItemInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_ThumbnailUpdated)(self.get_abi() as *const _ as *mut _, changedHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_thumbnail_updated(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_ThumbnailUpdated)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_properties_updated(&self, changedHandler: &foundation::TypedEventHandler<IStorageItemInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_PropertiesUpdated)(self.get_abi() as *const _ as *mut _, changedHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_properties_updated(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_PropertiesUpdated)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Storage.BulkAccess
pub mod compression { // Windows.Storage.Compression
use crate::prelude::*;
RT_ENUM! { enum CompressAlgorithm: i32 {
    InvalidAlgorithm = 0, NullAlgorithm = 1, Mszip = 2, Xpress = 3, XpressHuff = 4, Lzms = 5,
}}
DEFINE_IID!(IID_ICompressor, 180577370, 22444, 20193, 183, 2, 132, 211, 157, 84, 36, 224);
RT_INTERFACE!{interface ICompressor(ICompressorVtbl, ICompressor_Abi): IInspectable(IInspectableVtbl) [IID_ICompressor] {
    fn FinishAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn DetachStream(&self, out: *mut <super::streams::IOutputStream as RtType>::Abi) -> HRESULT
}}
impl ICompressor {
    #[inline] pub fn finish_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FinishAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn detach_stream(&self) -> Result<Option<super::streams::IOutputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::streams::IOutputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Compressor: ICompressor}
impl RtActivatable<ICompressorFactory> for Compressor {}
impl Compressor {
    #[inline] pub fn create_compressor(underlyingStream: &super::streams::IOutputStream) -> Result<Compressor> {
        <Self as RtActivatable<ICompressorFactory>>::get_activation_factory().create_compressor(underlyingStream)
    }
    #[inline] pub fn create_compressor_ex(underlyingStream: &super::streams::IOutputStream, algorithm: CompressAlgorithm, blockSize: u32) -> Result<Compressor> {
        <Self as RtActivatable<ICompressorFactory>>::get_activation_factory().create_compressor_ex(underlyingStream, algorithm, blockSize)
    }
}
DEFINE_CLSID!(Compressor(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,111,109,112,114,101,115,115,105,111,110,46,67,111,109,112,114,101,115,115,111,114,0]) [CLSID_Compressor]);
DEFINE_IID!(IID_ICompressorFactory, 1597871780, 11515, 17452, 168, 186, 215, 209, 27, 3, 157, 160);
RT_INTERFACE!{static interface ICompressorFactory(ICompressorFactoryVtbl, ICompressorFactory_Abi): IInspectable(IInspectableVtbl) [IID_ICompressorFactory] {
    fn CreateCompressor(&self, underlyingStream: <super::streams::IOutputStream as RtType>::Abi, out: *mut <Compressor as RtType>::Abi) -> HRESULT,
    fn CreateCompressorEx(&self, underlyingStream: <super::streams::IOutputStream as RtType>::Abi, algorithm: CompressAlgorithm, blockSize: u32, out: *mut <Compressor as RtType>::Abi) -> HRESULT
}}
impl ICompressorFactory {
    #[inline] pub fn create_compressor(&self, underlyingStream: &super::streams::IOutputStream) -> Result<Compressor> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCompressor)(self.get_abi() as *const _ as *mut _, underlyingStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Compressor::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_compressor_ex(&self, underlyingStream: &super::streams::IOutputStream, algorithm: CompressAlgorithm, blockSize: u32) -> Result<Compressor> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCompressorEx)(self.get_abi() as *const _ as *mut _, underlyingStream.get_abi() as *const _ as *mut _, algorithm, blockSize, &mut out);
        if hr == S_OK { Ok(Compressor::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDecompressor, 3095658054, 54922, 19595, 173, 160, 78, 232, 19, 252, 82, 131);
RT_INTERFACE!{interface IDecompressor(IDecompressorVtbl, IDecompressor_Abi): IInspectable(IInspectableVtbl) [IID_IDecompressor] {
    fn DetachStream(&self, out: *mut <super::streams::IInputStream as RtType>::Abi) -> HRESULT
}}
impl IDecompressor {
    #[inline] pub fn detach_stream(&self) -> Result<Option<super::streams::IInputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::streams::IInputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Decompressor: IDecompressor}
impl RtActivatable<IDecompressorFactory> for Decompressor {}
impl Decompressor {
    #[inline] pub fn create_decompressor(underlyingStream: &super::streams::IInputStream) -> Result<Decompressor> {
        <Self as RtActivatable<IDecompressorFactory>>::get_activation_factory().create_decompressor(underlyingStream)
    }
}
DEFINE_CLSID!(Decompressor(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,67,111,109,112,114,101,115,115,105,111,110,46,68,101,99,111,109,112,114,101,115,115,111,114,0]) [CLSID_Decompressor]);
DEFINE_IID!(IID_IDecompressorFactory, 1396171346, 7586, 17121, 136, 52, 3, 121, 210, 141, 116, 47);
RT_INTERFACE!{static interface IDecompressorFactory(IDecompressorFactoryVtbl, IDecompressorFactory_Abi): IInspectable(IInspectableVtbl) [IID_IDecompressorFactory] {
    fn CreateDecompressor(&self, underlyingStream: <super::streams::IInputStream as RtType>::Abi, out: *mut <Decompressor as RtType>::Abi) -> HRESULT
}}
impl IDecompressorFactory {
    #[inline] pub fn create_decompressor(&self, underlyingStream: &super::streams::IInputStream) -> Result<Decompressor> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateDecompressor)(self.get_abi() as *const _ as *mut _, underlyingStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Decompressor::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Storage.Compression
pub mod fileproperties { // Windows.Storage.FileProperties
use crate::prelude::*;
DEFINE_IID!(IID_IBasicProperties, 3495777755, 30814, 19046, 190, 2, 155, 238, 197, 138, 234, 129);
RT_INTERFACE!{interface IBasicProperties(IBasicPropertiesVtbl, IBasicProperties_Abi): IInspectable(IInspectableVtbl) [IID_IBasicProperties] {
    fn get_Size(&self, out: *mut u64) -> HRESULT,
    fn get_DateModified(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_ItemDate(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl IBasicProperties {
    #[inline] pub fn get_size(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Size)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_date_modified(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DateModified)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_item_date(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ItemDate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BasicProperties: IBasicProperties}
DEFINE_IID!(IID_IDocumentProperties, 2125142460, 6177, 18723, 180, 169, 10, 234, 64, 77, 0, 112);
RT_INTERFACE!{interface IDocumentProperties(IDocumentPropertiesVtbl, IDocumentProperties_Abi): IInspectable(IInspectableVtbl) [IID_IDocumentProperties] {
    fn get_Author(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Keywords(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Comment(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Comment(&self, value: HSTRING) -> HRESULT
}}
impl IDocumentProperties {
    #[inline] pub fn get_author(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Author)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keywords(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Keywords)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_comment(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Comment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_comment(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Comment)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DocumentProperties: IDocumentProperties}
RT_CLASS!{static class GeotagHelper}
impl RtActivatable<IGeotagHelperStatics> for GeotagHelper {}
impl GeotagHelper {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_geotag_async(file: &super::IStorageFile) -> Result<foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint>> {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().get_geotag_async(file)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_from_geolocator_async(file: &super::IStorageFile, geolocator: &super::super::devices::geolocation::Geolocator) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().set_geotag_from_geolocator_async(file, geolocator)
    }
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_async(file: &super::IStorageFile, geopoint: &super::super::devices::geolocation::Geopoint) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IGeotagHelperStatics>>::get_activation_factory().set_geotag_async(file, geopoint)
    }
}
DEFINE_CLSID!(GeotagHelper(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,70,105,108,101,80,114,111,112,101,114,116,105,101,115,46,71,101,111,116,97,103,72,101,108,112,101,114,0]) [CLSID_GeotagHelper]);
DEFINE_IID!(IID_IGeotagHelperStatics, 1095316036, 9508, 18005, 134, 166, 237, 22, 245, 252, 113, 107);
RT_INTERFACE!{static interface IGeotagHelperStatics(IGeotagHelperStaticsVtbl, IGeotagHelperStatics_Abi): IInspectable(IInspectableVtbl) [IID_IGeotagHelperStatics] {
    #[cfg(feature="windows-devices")] fn GetGeotagAsync(&self, file: <super::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-devices")] fn SetGeotagFromGeolocatorAsync(&self, file: <super::IStorageFile as RtType>::Abi, geolocator: <super::super::devices::geolocation::Geolocator as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-devices")] fn SetGeotagAsync(&self, file: <super::IStorageFile as RtType>::Abi, geopoint: <super::super::devices::geolocation::Geopoint as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IGeotagHelperStatics {
    #[cfg(feature="windows-devices")] #[inline] pub fn get_geotag_async(&self, file: &super::IStorageFile) -> Result<foundation::IAsyncOperation<super::super::devices::geolocation::Geopoint>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetGeotagAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_from_geolocator_async(&self, file: &super::IStorageFile, geolocator: &super::super::devices::geolocation::Geolocator) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetGeotagFromGeolocatorAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, geolocator.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-devices")] #[inline] pub fn set_geotag_async(&self, file: &super::IStorageFile, geopoint: &super::super::devices::geolocation::Geopoint) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetGeotagAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, geopoint.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IImageProperties, 1379701796, 64767, 17013, 175, 238, 236, 219, 154, 180, 121, 115);
RT_INTERFACE!{interface IImageProperties(IImagePropertiesVtbl, IImageProperties_Abi): IInspectable(IInspectableVtbl) [IID_IImageProperties] {
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Keywords(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_DateTaken(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn put_DateTaken(&self, value: foundation::DateTime) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Latitude(&self, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT,
    fn get_Longitude(&self, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT,
    fn get_CameraManufacturer(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CameraManufacturer(&self, value: HSTRING) -> HRESULT,
    fn get_CameraModel(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CameraModel(&self, value: HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut PhotoOrientation) -> HRESULT,
    fn get_PeopleNames(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IImageProperties {
    #[inline] pub fn get_rating(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Rating)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rating(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Rating)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keywords(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Keywords)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_date_taken(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DateTaken)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_date_taken(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DateTaken)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Width)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Height)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_latitude(&self) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Latitude)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_longitude(&self) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Longitude)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_camera_manufacturer(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraManufacturer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_camera_manufacturer(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_CameraManufacturer)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_camera_model(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraModel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_camera_model(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_CameraModel)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<PhotoOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_people_names(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PeopleNames)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageProperties: IImageProperties}
DEFINE_IID!(IID_IMusicProperties, 3163204450, 26348, 16794, 188, 93, 202, 101, 164, 203, 70, 218);
RT_INTERFACE!{interface IMusicProperties(IMusicPropertiesVtbl, IMusicProperties_Abi): IInspectable(IInspectableVtbl) [IID_IMusicProperties] {
    fn get_Album(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Album(&self, value: HSTRING) -> HRESULT,
    fn get_Artist(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Artist(&self, value: HSTRING) -> HRESULT,
    fn get_Genre(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_TrackNumber(&self, out: *mut u32) -> HRESULT,
    fn put_TrackNumber(&self, value: u32) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Duration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Bitrate(&self, out: *mut u32) -> HRESULT,
    fn get_AlbumArtist(&self, out: *mut HSTRING) -> HRESULT,
    fn put_AlbumArtist(&self, value: HSTRING) -> HRESULT,
    fn get_Composers(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Conductors(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Subtitle(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subtitle(&self, value: HSTRING) -> HRESULT,
    fn get_Producers(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Publisher(&self, value: HSTRING) -> HRESULT,
    fn get_Writers(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Year(&self, out: *mut u32) -> HRESULT,
    fn put_Year(&self, value: u32) -> HRESULT
}}
impl IMusicProperties {
    #[inline] pub fn get_album(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Album)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_album(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Album)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_artist(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Artist)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_artist(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Artist)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_genre(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Genre)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_track_number(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TrackNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_track_number(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TrackNumber)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_rating(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Rating)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rating(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Rating)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Duration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bitrate(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Bitrate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_album_artist(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AlbumArtist)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_album_artist(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AlbumArtist)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_composers(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Composers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_conductors(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Conductors)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_subtitle(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Subtitle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_subtitle(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Subtitle)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_producers(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Producers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_publisher(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Publisher)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_publisher(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Publisher)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_writers(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Writers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_year(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Year)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_year(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Year)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class MusicProperties: IMusicProperties}
RT_ENUM! { enum PhotoOrientation: i32 {
    Unspecified = 0, Normal = 1, FlipHorizontal = 2, Rotate180 = 3, FlipVertical = 4, Transpose = 5, Rotate270 = 6, Transverse = 7, Rotate90 = 8,
}}
RT_ENUM! { enum PropertyPrefetchOptions: u32 {
    None = 0, MusicProperties = 1, VideoProperties = 2, ImageProperties = 4, DocumentProperties = 8, BasicProperties = 16,
}}
DEFINE_IID!(IID_IStorageItemContentProperties, 86592429, 48184, 18623, 133, 215, 119, 14, 14, 42, 224, 186);
RT_INTERFACE!{interface IStorageItemContentProperties(IStorageItemContentPropertiesVtbl, IStorageItemContentProperties_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemContentProperties] {
    fn GetMusicPropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<MusicProperties> as RtType>::Abi) -> HRESULT,
    fn GetVideoPropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<VideoProperties> as RtType>::Abi) -> HRESULT,
    fn GetImagePropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<ImageProperties> as RtType>::Abi) -> HRESULT,
    fn GetDocumentPropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<DocumentProperties> as RtType>::Abi) -> HRESULT
}}
impl IStorageItemContentProperties {
    #[inline] pub fn get_music_properties_async(&self) -> Result<foundation::IAsyncOperation<MusicProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetMusicPropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_video_properties_async(&self) -> Result<foundation::IAsyncOperation<VideoProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVideoPropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_image_properties_async(&self) -> Result<foundation::IAsyncOperation<ImageProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetImagePropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_properties_async(&self) -> Result<foundation::IAsyncOperation<DocumentProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDocumentPropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageItemContentProperties: IStorageItemContentProperties}
DEFINE_IID!(IID_IStorageItemExtraProperties, 3309527474, 21709, 17195, 189, 188, 75, 25, 196, 180, 112, 215);
RT_INTERFACE!{interface IStorageItemExtraProperties(IStorageItemExtraPropertiesVtbl, IStorageItemExtraProperties_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemExtraProperties] {
    fn RetrievePropertiesAsync(&self, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IMap<HString, IInspectable>> as RtType>::Abi) -> HRESULT,
    fn SavePropertiesAsync(&self, propertiesToSave: <foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, IInspectable>> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn SavePropertiesAsyncOverloadDefault(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IStorageItemExtraProperties {
    #[inline] pub fn retrieve_properties_async(&self, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<foundation::collections::IMap<HString, IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RetrievePropertiesAsync)(self.get_abi() as *const _ as *mut _, propertiesToRetrieve.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_properties_async(&self, propertiesToSave: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, IInspectable>>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SavePropertiesAsync)(self.get_abi() as *const _ as *mut _, propertiesToSave.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_properties_async_overload_default(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SavePropertiesAsyncOverloadDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageItemThumbnail: super::streams::IRandomAccessStreamWithContentType}
RT_ENUM! { enum ThumbnailMode: i32 {
    PicturesView = 0, VideosView = 1, MusicView = 2, DocumentsView = 3, ListView = 4, SingleItem = 5,
}}
RT_ENUM! { enum ThumbnailOptions: u32 {
    None = 0, ReturnOnlyIfCached = 1, ResizeThumbnail = 2, UseCurrentScale = 4,
}}
DEFINE_IID!(IID_IThumbnailProperties, 1765659695, 56295, 18869, 179, 179, 40, 147, 172, 93, 52, 35);
RT_INTERFACE!{interface IThumbnailProperties(IThumbnailPropertiesVtbl, IThumbnailProperties_Abi): IInspectable(IInspectableVtbl) [IID_IThumbnailProperties] {
    fn get_OriginalWidth(&self, out: *mut u32) -> HRESULT,
    fn get_OriginalHeight(&self, out: *mut u32) -> HRESULT,
    fn get_ReturnedSmallerCachedSize(&self, out: *mut bool) -> HRESULT,
    fn get_Type(&self, out: *mut ThumbnailType) -> HRESULT
}}
impl IThumbnailProperties {
    #[inline] pub fn get_original_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_OriginalWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_original_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_OriginalHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_returned_smaller_cached_size(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ReturnedSmallerCachedSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<ThumbnailType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Type)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum ThumbnailType: i32 {
    Image = 0, Icon = 1,
}}
RT_ENUM! { enum VideoOrientation: i32 {
    Normal = 0, Rotate90 = 90, Rotate180 = 180, Rotate270 = 270,
}}
DEFINE_IID!(IID_IVideoProperties, 1905976583, 26846, 19896, 151, 222, 73, 153, 140, 5, 159, 47);
RT_INTERFACE!{interface IVideoProperties(IVideoPropertiesVtbl, IVideoProperties_Abi): IInspectable(IInspectableVtbl) [IID_IVideoProperties] {
    fn get_Rating(&self, out: *mut u32) -> HRESULT,
    fn put_Rating(&self, value: u32) -> HRESULT,
    fn get_Keywords(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT,
    fn get_Duration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_Latitude(&self, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT,
    fn get_Longitude(&self, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_Subtitle(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Subtitle(&self, value: HSTRING) -> HRESULT,
    fn get_Producers(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Publisher(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Publisher(&self, value: HSTRING) -> HRESULT,
    fn get_Writers(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Year(&self, out: *mut u32) -> HRESULT,
    fn put_Year(&self, value: u32) -> HRESULT,
    fn get_Bitrate(&self, out: *mut u32) -> HRESULT,
    fn get_Directors(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Orientation(&self, out: *mut VideoOrientation) -> HRESULT
}}
impl IVideoProperties {
    #[inline] pub fn get_rating(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Rating)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rating(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Rating)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_keywords(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Keywords)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Width)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Height)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Duration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_latitude(&self) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Latitude)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_longitude(&self) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Longitude)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_subtitle(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Subtitle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_subtitle(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Subtitle)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_producers(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Producers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_publisher(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Publisher)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_publisher(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Publisher)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_writers(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Writers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_year(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Year)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_year(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Year)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bitrate(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Bitrate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_directors(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Directors)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<VideoOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class VideoProperties: IVideoProperties}
} // Windows.Storage.FileProperties
pub mod pickers { // Windows.Storage.Pickers
use crate::prelude::*;
RT_CLASS!{class FileExtensionVector: foundation::collections::IVector<HString>}
DEFINE_IID!(IID_IFileOpenPicker, 749217674, 4805, 19551, 137, 119, 148, 84, 119, 147, 194, 65);
RT_INTERFACE!{interface IFileOpenPicker(IFileOpenPickerVtbl, IFileOpenPicker_Abi): IInspectable(IInspectableVtbl) [IID_IFileOpenPicker] {
    fn get_ViewMode(&self, out: *mut PickerViewMode) -> HRESULT,
    fn put_ViewMode(&self, value: PickerViewMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeFilter(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn PickSingleFileAsync(&self, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT,
    fn PickMultipleFilesAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>> as RtType>::Abi) -> HRESULT
}}
impl IFileOpenPicker {
    #[inline] pub fn get_view_mode(&self) -> Result<PickerViewMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ViewMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_view_mode(&self, value: PickerViewMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ViewMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_settings_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_start_location(&self) -> Result<PickerLocationId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_commit_button_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CommitButtonText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_CommitButtonText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_file_type_filter(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileTypeFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_single_file_async(&self) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickSingleFileAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_multiple_files_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickMultipleFilesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileOpenPicker: IFileOpenPicker}
impl RtActivatable<IFileOpenPickerStatics> for FileOpenPicker {}
impl RtActivatable<IActivationFactory> for FileOpenPicker {}
impl FileOpenPicker {
    #[inline] pub fn resume_pick_single_file_async() -> Result<foundation::IAsyncOperation<super::StorageFile>> {
        <Self as RtActivatable<IFileOpenPickerStatics>>::get_activation_factory().resume_pick_single_file_async()
    }
}
DEFINE_CLSID!(FileOpenPicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,105,108,101,79,112,101,110,80,105,99,107,101,114,0]) [CLSID_FileOpenPicker]);
DEFINE_IID!(IID_IFileOpenPicker2, 2364239058, 46150, 18167, 178, 101, 144, 248, 229, 90, 214, 80);
RT_INTERFACE!{interface IFileOpenPicker2(IFileOpenPicker2Vtbl, IFileOpenPicker2_Abi): IInspectable(IInspectableVtbl) [IID_IFileOpenPicker2] {
    fn get_ContinuationData(&self, out: *mut <foundation::collections::ValueSet as RtType>::Abi) -> HRESULT,
    fn PickSingleFileAndContinue(&self) -> HRESULT,
    fn PickMultipleFilesAndContinue(&self) -> HRESULT
}}
impl IFileOpenPicker2 {
    #[inline] pub fn get_continuation_data(&self) -> Result<Option<foundation::collections::ValueSet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContinuationData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::ValueSet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_single_file_and_continue(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).PickSingleFileAndContinue)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn pick_multiple_files_and_continue(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).PickMultipleFilesAndContinue)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileOpenPickerStatics, 1747015483, 12034, 18483, 150, 212, 171, 191, 173, 114, 182, 123);
RT_INTERFACE!{static interface IFileOpenPickerStatics(IFileOpenPickerStaticsVtbl, IFileOpenPickerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerStatics] {
    fn ResumePickSingleFileAsync(&self, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT
}}
impl IFileOpenPickerStatics {
    #[inline] pub fn resume_pick_single_file_async(&self) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ResumePickSingleFileAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileOpenPickerWithOperationId, 1062712681, 9506, 19621, 170, 115, 161, 85, 9, 241, 252, 191);
RT_INTERFACE!{interface IFileOpenPickerWithOperationId(IFileOpenPickerWithOperationIdVtbl, IFileOpenPickerWithOperationId_Abi): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerWithOperationId] {
    fn PickSingleFileAsync(&self, pickerOperationId: HSTRING, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT
}}
impl IFileOpenPickerWithOperationId {
    #[inline] pub fn pick_single_file_async(&self, pickerOperationId: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickSingleFileAsync)(self.get_abi() as *const _ as *mut _, pickerOperationId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FilePickerFileTypesOrderedMap: foundation::collections::IMap<HString, foundation::collections::IVector<HString>>}
RT_CLASS!{class FilePickerSelectedFilesArray: foundation::collections::IVectorView<super::StorageFile>}
DEFINE_IID!(IID_IFileSavePicker, 847708107, 24959, 19653, 175, 106, 179, 253, 242, 154, 209, 69);
RT_INTERFACE!{interface IFileSavePicker(IFileSavePickerVtbl, IFileSavePicker_Abi): IInspectable(IInspectableVtbl) [IID_IFileSavePicker] {
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeChoices(&self, out: *mut <foundation::collections::IMap<HString, foundation::collections::IVector<HString>> as RtType>::Abi) -> HRESULT,
    fn get_DefaultFileExtension(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DefaultFileExtension(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedSaveFile(&self, out: *mut <super::StorageFile as RtType>::Abi) -> HRESULT,
    fn put_SuggestedSaveFile(&self, value: <super::StorageFile as RtType>::Abi) -> HRESULT,
    fn get_SuggestedFileName(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SuggestedFileName(&self, value: HSTRING) -> HRESULT,
    fn PickSaveFileAsync(&self, out: *mut <foundation::IAsyncOperation<super::StorageFile> as RtType>::Abi) -> HRESULT
}}
impl IFileSavePicker {
    #[inline] pub fn get_settings_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_start_location(&self) -> Result<PickerLocationId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_commit_button_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CommitButtonText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_CommitButtonText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_file_type_choices(&self) -> Result<Option<foundation::collections::IMap<HString, foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileTypeChoices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_default_file_extension(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DefaultFileExtension)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_default_file_extension(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DefaultFileExtension)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_save_file(&self) -> Result<Option<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SuggestedSaveFile)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFile::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_save_file(&self, value: &super::StorageFile) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SuggestedSaveFile)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_file_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SuggestedFileName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_file_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SuggestedFileName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn pick_save_file_async(&self) -> Result<foundation::IAsyncOperation<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickSaveFileAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileSavePicker: IFileSavePicker}
impl RtActivatable<IActivationFactory> for FileSavePicker {}
DEFINE_CLSID!(FileSavePicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,105,108,101,83,97,118,101,80,105,99,107,101,114,0]) [CLSID_FileSavePicker]);
DEFINE_IID!(IID_IFileSavePicker2, 247665570, 53835, 17562, 129, 151, 232, 145, 4, 253, 66, 204);
RT_INTERFACE!{interface IFileSavePicker2(IFileSavePicker2Vtbl, IFileSavePicker2_Abi): IInspectable(IInspectableVtbl) [IID_IFileSavePicker2] {
    fn get_ContinuationData(&self, out: *mut <foundation::collections::ValueSet as RtType>::Abi) -> HRESULT,
    fn PickSaveFileAndContinue(&self) -> HRESULT
}}
impl IFileSavePicker2 {
    #[inline] pub fn get_continuation_data(&self) -> Result<Option<foundation::collections::ValueSet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContinuationData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::ValueSet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_save_file_and_continue(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).PickSaveFileAndContinue)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileSavePicker3, 1770712169, 47676, 20049, 189, 144, 74, 188, 187, 244, 207, 175);
RT_INTERFACE!{interface IFileSavePicker3(IFileSavePicker3Vtbl, IFileSavePicker3_Abi): IInspectable(IInspectableVtbl) [IID_IFileSavePicker3] {
    fn get_EnterpriseId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_EnterpriseId(&self, value: HSTRING) -> HRESULT
}}
impl IFileSavePicker3 {
    #[inline] pub fn get_enterprise_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_EnterpriseId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_enterprise_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_EnterpriseId)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFolderPicker, 139425689, 62459, 16394, 153, 177, 123, 74, 119, 47, 214, 13);
RT_INTERFACE!{interface IFolderPicker(IFolderPickerVtbl, IFolderPicker_Abi): IInspectable(IInspectableVtbl) [IID_IFolderPicker] {
    fn get_ViewMode(&self, out: *mut PickerViewMode) -> HRESULT,
    fn put_ViewMode(&self, value: PickerViewMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn put_SettingsIdentifier(&self, value: HSTRING) -> HRESULT,
    fn get_SuggestedStartLocation(&self, out: *mut PickerLocationId) -> HRESULT,
    fn put_SuggestedStartLocation(&self, value: PickerLocationId) -> HRESULT,
    fn get_CommitButtonText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_CommitButtonText(&self, value: HSTRING) -> HRESULT,
    fn get_FileTypeFilter(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn PickSingleFolderAsync(&self, out: *mut <foundation::IAsyncOperation<super::StorageFolder> as RtType>::Abi) -> HRESULT
}}
impl IFolderPicker {
    #[inline] pub fn get_view_mode(&self) -> Result<PickerViewMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ViewMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_view_mode(&self, value: PickerViewMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ViewMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_settings_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_settings_identifier(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_suggested_start_location(&self) -> Result<PickerLocationId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_suggested_start_location(&self, value: PickerLocationId) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SuggestedStartLocation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_commit_button_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CommitButtonText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_commit_button_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_CommitButtonText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_file_type_filter(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileTypeFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_single_folder_async(&self) -> Result<foundation::IAsyncOperation<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickSingleFolderAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FolderPicker: IFolderPicker}
impl RtActivatable<IActivationFactory> for FolderPicker {}
DEFINE_CLSID!(FolderPicker(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,105,99,107,101,114,115,46,70,111,108,100,101,114,80,105,99,107,101,114,0]) [CLSID_FolderPicker]);
DEFINE_IID!(IID_IFolderPicker2, 2394143383, 56453, 17942, 190, 148, 150, 96, 136, 31, 47, 93);
RT_INTERFACE!{interface IFolderPicker2(IFolderPicker2Vtbl, IFolderPicker2_Abi): IInspectable(IInspectableVtbl) [IID_IFolderPicker2] {
    fn get_ContinuationData(&self, out: *mut <foundation::collections::ValueSet as RtType>::Abi) -> HRESULT,
    fn PickFolderAndContinue(&self) -> HRESULT
}}
impl IFolderPicker2 {
    #[inline] pub fn get_continuation_data(&self) -> Result<Option<foundation::collections::ValueSet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContinuationData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::ValueSet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn pick_folder_and_continue(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).PickFolderAndContinue)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum PickerLocationId: i32 {
    DocumentsLibrary = 0, ComputerFolder = 1, Desktop = 2, Downloads = 3, HomeGroup = 4, MusicLibrary = 5, PicturesLibrary = 6, VideosLibrary = 7, Objects3D = 8, Unspecified = 9,
}}
RT_ENUM! { enum PickerViewMode: i32 {
    List = 0, Thumbnail = 1,
}}
pub mod provider { // Windows.Storage.Pickers.Provider
use crate::prelude::*;
RT_ENUM! { enum AddFileResult: i32 {
    Added = 0, AlreadyAdded = 1, NotAllowed = 2, Unavailable = 3,
}}
DEFINE_IID!(IID_IFileOpenPickerUI, 3718535696, 63956, 16580, 138, 245, 197, 182, 181, 166, 29, 29);
RT_INTERFACE!{interface IFileOpenPickerUI(IFileOpenPickerUIVtbl, IFileOpenPickerUI_Abi): IInspectable(IInspectableVtbl) [IID_IFileOpenPickerUI] {
    fn AddFile(&self, id: HSTRING, file: <super::super::IStorageFile as RtType>::Abi, out: *mut AddFileResult) -> HRESULT,
    fn RemoveFile(&self, id: HSTRING) -> HRESULT,
    fn ContainsFile(&self, id: HSTRING, out: *mut bool) -> HRESULT,
    fn CanAddFile(&self, file: <super::super::IStorageFile as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn get_AllowedFileTypes(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_SelectionMode(&self, out: *mut FileSelectionMode) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn add_FileRemoved(&self, handler: <foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileRemoved(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Closing(&self, handler: <foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closing(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IFileOpenPickerUI {
    #[inline] pub fn add_file(&self, id: &HStringArg, file: &super::super::IStorageFile) -> Result<AddFileResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).AddFile)(self.get_abi() as *const _ as *mut _, id.get(), file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_file(&self, id: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).RemoveFile)(self.get_abi() as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn contains_file(&self, id: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ContainsFile)(self.get_abi() as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn can_add_file(&self, file: &super::super::IStorageFile) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).CanAddFile)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_allowed_file_types(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AllowedFileTypes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_selection_mode(&self) -> Result<FileSelectionMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SelectionMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_settings_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_file_removed(&self, handler: &foundation::TypedEventHandler<FileOpenPickerUI, FileRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_FileRemoved)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_file_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_FileRemoved)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_closing(&self, handler: &foundation::TypedEventHandler<FileOpenPickerUI, PickerClosingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Closing)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closing(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Closing)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class FileOpenPickerUI: IFileOpenPickerUI}
DEFINE_IID!(IID_IFileRemovedEventArgs, 319045031, 32714, 19499, 158, 202, 104, 144, 249, 240, 1, 133);
RT_INTERFACE!{interface IFileRemovedEventArgs(IFileRemovedEventArgsVtbl, IFileRemovedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IFileRemovedEventArgs] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT
}}
impl IFileRemovedEventArgs {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileRemovedEventArgs: IFileRemovedEventArgs}
DEFINE_IID!(IID_IFileSavePickerUI, 2522268135, 15958, 17356, 138, 57, 51, 199, 61, 157, 84, 43);
RT_INTERFACE!{interface IFileSavePickerUI(IFileSavePickerUIVtbl, IFileSavePickerUI_Abi): IInspectable(IInspectableVtbl) [IID_IFileSavePickerUI] {
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_AllowedFileTypes(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_SettingsIdentifier(&self, out: *mut HSTRING) -> HRESULT,
    fn get_FileName(&self, out: *mut HSTRING) -> HRESULT,
    fn TrySetFileName(&self, value: HSTRING, out: *mut SetFileNameResult) -> HRESULT,
    fn add_FileNameChanged(&self, handler: <foundation::TypedEventHandler<FileSavePickerUI, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileNameChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_TargetFileRequested(&self, handler: <foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TargetFileRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IFileSavePickerUI {
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allowed_file_types(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AllowedFileTypes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_settings_identifier(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SettingsIdentifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_set_file_name(&self, value: &HStringArg) -> Result<SetFileNameResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).TrySetFileName)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_file_name_changed(&self, handler: &foundation::TypedEventHandler<FileSavePickerUI, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_FileNameChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_file_name_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_FileNameChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_target_file_requested(&self, handler: &foundation::TypedEventHandler<FileSavePickerUI, TargetFileRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_TargetFileRequested)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_target_file_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_TargetFileRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class FileSavePickerUI: IFileSavePickerUI}
RT_ENUM! { enum FileSelectionMode: i32 {
    Single = 0, Multiple = 1,
}}
DEFINE_IID!(IID_IPickerClosingDeferral, 2063071006, 6759, 18993, 174, 128, 233, 7, 112, 138, 97, 155);
RT_INTERFACE!{interface IPickerClosingDeferral(IPickerClosingDeferralVtbl, IPickerClosingDeferral_Abi): IInspectable(IInspectableVtbl) [IID_IPickerClosingDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPickerClosingDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PickerClosingDeferral: IPickerClosingDeferral}
DEFINE_IID!(IID_IPickerClosingEventArgs, 2119823908, 45874, 20242, 139, 159, 168, 194, 240, 107, 50, 205);
RT_INTERFACE!{interface IPickerClosingEventArgs(IPickerClosingEventArgsVtbl, IPickerClosingEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPickerClosingEventArgs] {
    fn get_ClosingOperation(&self, out: *mut <PickerClosingOperation as RtType>::Abi) -> HRESULT,
    fn get_IsCanceled(&self, out: *mut bool) -> HRESULT
}}
impl IPickerClosingEventArgs {
    #[inline] pub fn get_closing_operation(&self) -> Result<Option<PickerClosingOperation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ClosingOperation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PickerClosingOperation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_canceled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsCanceled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PickerClosingEventArgs: IPickerClosingEventArgs}
DEFINE_IID!(IID_IPickerClosingOperation, 1290402692, 48878, 20025, 167, 115, 252, 95, 14, 174, 50, 141);
RT_INTERFACE!{interface IPickerClosingOperation(IPickerClosingOperationVtbl, IPickerClosingOperation_Abi): IInspectable(IInspectableVtbl) [IID_IPickerClosingOperation] {
    fn GetDeferral(&self, out: *mut <PickerClosingDeferral as RtType>::Abi) -> HRESULT,
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT
}}
impl IPickerClosingOperation {
    #[inline] pub fn get_deferral(&self) -> Result<Option<PickerClosingDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PickerClosingDeferral::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Deadline)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PickerClosingOperation: IPickerClosingOperation}
RT_ENUM! { enum SetFileNameResult: i32 {
    Succeeded = 0, NotAllowed = 1, Unavailable = 2,
}}
DEFINE_IID!(IID_ITargetFileRequest, 1119695701, 32648, 18315, 142, 129, 105, 11, 32, 52, 6, 120);
RT_INTERFACE!{interface ITargetFileRequest(ITargetFileRequestVtbl, ITargetFileRequest_Abi): IInspectable(IInspectableVtbl) [IID_ITargetFileRequest] {
    fn get_TargetFile(&self, out: *mut <super::super::IStorageFile as RtType>::Abi) -> HRESULT,
    fn put_TargetFile(&self, value: <super::super::IStorageFile as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <TargetFileRequestDeferral as RtType>::Abi) -> HRESULT
}}
impl ITargetFileRequest {
    #[inline] pub fn get_target_file(&self) -> Result<Option<super::super::IStorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TargetFile)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::IStorageFile::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_target_file(&self, value: &super::super::IStorageFile) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TargetFile)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<TargetFileRequestDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TargetFileRequestDeferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetFileRequest: ITargetFileRequest}
DEFINE_IID!(IID_ITargetFileRequestDeferral, 1257151889, 48917, 19881, 149, 246, 246, 183, 213, 88, 34, 91);
RT_INTERFACE!{interface ITargetFileRequestDeferral(ITargetFileRequestDeferralVtbl, ITargetFileRequestDeferral_Abi): IInspectable(IInspectableVtbl) [IID_ITargetFileRequestDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl ITargetFileRequestDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetFileRequestDeferral: ITargetFileRequestDeferral}
DEFINE_IID!(IID_ITargetFileRequestedEventArgs, 2976111553, 6993, 19593, 165, 145, 15, 212, 11, 60, 87, 201);
RT_INTERFACE!{interface ITargetFileRequestedEventArgs(ITargetFileRequestedEventArgsVtbl, ITargetFileRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_ITargetFileRequestedEventArgs] {
    fn get_Request(&self, out: *mut <TargetFileRequest as RtType>::Abi) -> HRESULT
}}
impl ITargetFileRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<TargetFileRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Request)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(TargetFileRequest::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TargetFileRequestedEventArgs: ITargetFileRequestedEventArgs}
} // Windows.Storage.Pickers.Provider
} // Windows.Storage.Pickers
pub mod provider { // Windows.Storage.Provider
use crate::prelude::*;
RT_ENUM! { enum CachedFileOptions: u32 {
    None = 0, RequireUpdateOnAccess = 1, UseCachedFileWhenOffline = 2, DenyAccessWhenOffline = 4,
}}
RT_ENUM! { enum CachedFileTarget: i32 {
    Local = 0, Remote = 1,
}}
RT_CLASS!{static class CachedFileUpdater}
impl RtActivatable<ICachedFileUpdaterStatics> for CachedFileUpdater {}
impl CachedFileUpdater {
    #[inline] pub fn set_update_information(file: &super::IStorageFile, contentId: &HStringArg, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> Result<()> {
        <Self as RtActivatable<ICachedFileUpdaterStatics>>::get_activation_factory().set_update_information(file, contentId, readMode, writeMode, options)
    }
}
DEFINE_CLSID!(CachedFileUpdater(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,67,97,99,104,101,100,70,105,108,101,85,112,100,97,116,101,114,0]) [CLSID_CachedFileUpdater]);
DEFINE_IID!(IID_ICachedFileUpdaterStatics, 2680752416, 31695, 18568, 168, 30, 16, 45, 112, 52, 215, 206);
RT_INTERFACE!{static interface ICachedFileUpdaterStatics(ICachedFileUpdaterStaticsVtbl, ICachedFileUpdaterStatics_Abi): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterStatics] {
    fn SetUpdateInformation(&self, file: <super::IStorageFile as RtType>::Abi, contentId: HSTRING, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> HRESULT
}}
impl ICachedFileUpdaterStatics {
    #[inline] pub fn set_update_information(&self, file: &super::IStorageFile, contentId: &HStringArg, readMode: ReadActivationMode, writeMode: WriteActivationMode, options: CachedFileOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetUpdateInformation)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, contentId.get(), readMode, writeMode, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ICachedFileUpdaterUI, 2658091494, 47858, 19095, 182, 0, 147, 51, 245, 223, 128, 253);
RT_INTERFACE!{interface ICachedFileUpdaterUI(ICachedFileUpdaterUIVtbl, ICachedFileUpdaterUI_Abi): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterUI] {
    fn get_Title(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Title(&self, value: HSTRING) -> HRESULT,
    fn get_UpdateTarget(&self, out: *mut CachedFileTarget) -> HRESULT,
    fn add_FileUpdateRequested(&self, handler: <foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FileUpdateRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_UIRequested(&self, handler: <foundation::TypedEventHandler<CachedFileUpdaterUI, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UIRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_UIStatus(&self, out: *mut UIStatus) -> HRESULT
}}
impl ICachedFileUpdaterUI {
    #[inline] pub fn get_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Title)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_title(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Title)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_update_target(&self) -> Result<CachedFileTarget> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UpdateTarget)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_file_update_requested(&self, handler: &foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_FileUpdateRequested)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_file_update_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_FileUpdateRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_ui_requested(&self, handler: &foundation::TypedEventHandler<CachedFileUpdaterUI, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_UIRequested)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_ui_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_UIRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_ui_status(&self) -> Result<UIStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UIStatus)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class CachedFileUpdaterUI: ICachedFileUpdaterUI}
DEFINE_IID!(IID_ICachedFileUpdaterUI2, 2287378972, 34457, 17216, 159, 73, 247, 202, 215, 254, 137, 145);
RT_INTERFACE!{interface ICachedFileUpdaterUI2(ICachedFileUpdaterUI2Vtbl, ICachedFileUpdaterUI2_Abi): IInspectable(IInspectableVtbl) [IID_ICachedFileUpdaterUI2] {
    fn get_UpdateRequest(&self, out: *mut <FileUpdateRequest as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <FileUpdateRequestDeferral as RtType>::Abi) -> HRESULT
}}
impl ICachedFileUpdaterUI2 {
    #[inline] pub fn get_update_request(&self) -> Result<Option<FileUpdateRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_UpdateRequest)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(FileUpdateRequest::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<FileUpdateRequestDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(FileUpdateRequestDeferral::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileUpdateRequest, 1086858550, 49662, 19859, 167, 146, 30, 115, 107, 199, 8, 55);
RT_INTERFACE!{interface IFileUpdateRequest(IFileUpdateRequestVtbl, IFileUpdateRequest_Abi): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequest] {
    fn get_ContentId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_File(&self, out: *mut <super::StorageFile as RtType>::Abi) -> HRESULT,
    fn get_Status(&self, out: *mut FileUpdateStatus) -> HRESULT,
    fn put_Status(&self, value: FileUpdateStatus) -> HRESULT,
    fn GetDeferral(&self, out: *mut <FileUpdateRequestDeferral as RtType>::Abi) -> HRESULT,
    fn UpdateLocalFile(&self, value: <super::IStorageFile as RtType>::Abi) -> HRESULT
}}
impl IFileUpdateRequest {
    #[inline] pub fn get_content_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContentId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_file(&self) -> Result<Option<super::StorageFile>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_File)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFile::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<FileUpdateStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_status(&self, value: FileUpdateStatus) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Status)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<FileUpdateRequestDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(FileUpdateRequestDeferral::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_local_file(&self, value: &super::IStorageFile) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateLocalFile)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class FileUpdateRequest: IFileUpdateRequest}
DEFINE_IID!(IID_IFileUpdateRequest2, 2185774664, 48574, 17531, 162, 238, 122, 254, 106, 3, 42, 148);
RT_INTERFACE!{interface IFileUpdateRequest2(IFileUpdateRequest2Vtbl, IFileUpdateRequest2_Abi): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequest2] {
    fn get_UserInputNeededMessage(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserInputNeededMessage(&self, value: HSTRING) -> HRESULT
}}
impl IFileUpdateRequest2 {
    #[inline] pub fn get_user_input_needed_message(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_UserInputNeededMessage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_user_input_needed_message(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_UserInputNeededMessage)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IFileUpdateRequestDeferral, 4291746603, 35550, 17573, 187, 0, 22, 76, 78, 114, 241, 58);
RT_INTERFACE!{interface IFileUpdateRequestDeferral(IFileUpdateRequestDeferralVtbl, IFileUpdateRequestDeferral_Abi): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequestDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IFileUpdateRequestDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class FileUpdateRequestDeferral: IFileUpdateRequestDeferral}
DEFINE_IID!(IID_IFileUpdateRequestedEventArgs, 2064290626, 14597, 17293, 170, 239, 120, 174, 38, 95, 141, 210);
RT_INTERFACE!{interface IFileUpdateRequestedEventArgs(IFileUpdateRequestedEventArgsVtbl, IFileUpdateRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IFileUpdateRequestedEventArgs] {
    fn get_Request(&self, out: *mut <FileUpdateRequest as RtType>::Abi) -> HRESULT
}}
impl IFileUpdateRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<FileUpdateRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Request)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(FileUpdateRequest::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class FileUpdateRequestedEventArgs: IFileUpdateRequestedEventArgs}
RT_ENUM! { enum FileUpdateStatus: i32 {
    Incomplete = 0, Complete = 1, UserInputNeeded = 2, CurrentlyUnavailable = 3, Failed = 4, CompleteAndRenamed = 5,
}}
RT_ENUM! { enum ReadActivationMode: i32 {
    NotNeeded = 0, BeforeAccess = 1,
}}
DEFINE_IID!(IID_IStorageProviderGetContentInfoForPathResult, 627339549, 43657, 19730, 130, 227, 247, 42, 146, 227, 57, 102);
RT_INTERFACE!{interface IStorageProviderGetContentInfoForPathResult(IStorageProviderGetContentInfoForPathResultVtbl, IStorageProviderGetContentInfoForPathResult_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderGetContentInfoForPathResult] {
    fn get_Status(&self, out: *mut StorageProviderUriSourceStatus) -> HRESULT,
    fn put_Status(&self, value: StorageProviderUriSourceStatus) -> HRESULT,
    fn get_ContentUri(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContentUri(&self, value: HSTRING) -> HRESULT,
    fn get_ContentId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ContentId(&self, value: HSTRING) -> HRESULT
}}
impl IStorageProviderGetContentInfoForPathResult {
    #[inline] pub fn get_status(&self) -> Result<StorageProviderUriSourceStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_status(&self, value: StorageProviderUriSourceStatus) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Status)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContentUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_uri(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ContentUri)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContentId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_content_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ContentId)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProviderGetContentInfoForPathResult: IStorageProviderGetContentInfoForPathResult}
impl RtActivatable<IActivationFactory> for StorageProviderGetContentInfoForPathResult {}
DEFINE_CLSID!(StorageProviderGetContentInfoForPathResult(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,71,101,116,67,111,110,116,101,110,116,73,110,102,111,70,111,114,80,97,116,104,82,101,115,117,108,116,0]) [CLSID_StorageProviderGetContentInfoForPathResult]);
DEFINE_IID!(IID_IStorageProviderGetPathForContentUriResult, 1668356765, 16664, 17830, 172, 182, 34, 196, 157, 1, 159, 64);
RT_INTERFACE!{interface IStorageProviderGetPathForContentUriResult(IStorageProviderGetPathForContentUriResultVtbl, IStorageProviderGetPathForContentUriResult_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderGetPathForContentUriResult] {
    fn get_Status(&self, out: *mut StorageProviderUriSourceStatus) -> HRESULT,
    fn put_Status(&self, value: StorageProviderUriSourceStatus) -> HRESULT,
    fn get_Path(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Path(&self, value: HSTRING) -> HRESULT
}}
impl IStorageProviderGetPathForContentUriResult {
    #[inline] pub fn get_status(&self) -> Result<StorageProviderUriSourceStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_status(&self, value: StorageProviderUriSourceStatus) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Status)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Path)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_path(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Path)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProviderGetPathForContentUriResult: IStorageProviderGetPathForContentUriResult}
impl RtActivatable<IActivationFactory> for StorageProviderGetPathForContentUriResult {}
DEFINE_CLSID!(StorageProviderGetPathForContentUriResult(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,71,101,116,80,97,116,104,70,111,114,67,111,110,116,101,110,116,85,114,105,82,101,115,117,108,116,0]) [CLSID_StorageProviderGetPathForContentUriResult]);
RT_ENUM! { enum StorageProviderHardlinkPolicy: u32 {
    None = 0, Allowed = 1,
}}
RT_ENUM! { enum StorageProviderHydrationPolicy: i32 {
    Partial = 0, Progressive = 1, Full = 2, AlwaysFull = 3,
}}
RT_ENUM! { enum StorageProviderHydrationPolicyModifier: u32 {
    None = 0, ValidationRequired = 1, StreamingAllowed = 2, AutoDehydrationAllowed = 4,
}}
RT_ENUM! { enum StorageProviderInSyncPolicy: u32 {
    Default = 0, FileCreationTime = 1, FileReadOnlyAttribute = 2, FileHiddenAttribute = 4, FileSystemAttribute = 8, DirectoryCreationTime = 16, DirectoryReadOnlyAttribute = 32, DirectoryHiddenAttribute = 64, DirectorySystemAttribute = 128, FileLastWriteTime = 256, DirectoryLastWriteTime = 512, PreserveInsyncForSyncEngine = 2147483648,
}}
RT_CLASS!{static class StorageProviderItemProperties}
impl RtActivatable<IStorageProviderItemPropertiesStatics> for StorageProviderItemProperties {}
impl StorageProviderItemProperties {
    #[inline] pub fn set_async(item: &super::IStorageItem, itemProperties: &foundation::collections::IIterable<StorageProviderItemProperty>) -> Result<foundation::IAsyncAction> {
        <Self as RtActivatable<IStorageProviderItemPropertiesStatics>>::get_activation_factory().set_async(item, itemProperties)
    }
}
DEFINE_CLSID!(StorageProviderItemProperties(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_StorageProviderItemProperties]);
DEFINE_IID!(IID_IStorageProviderItemPropertiesStatics, 757865623, 9988, 18217, 143, 169, 126, 107, 142, 21, 140, 47);
RT_INTERFACE!{static interface IStorageProviderItemPropertiesStatics(IStorageProviderItemPropertiesStaticsVtbl, IStorageProviderItemPropertiesStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertiesStatics] {
    fn SetAsync(&self, item: <super::IStorageItem as RtType>::Abi, itemProperties: <foundation::collections::IIterable<StorageProviderItemProperty> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IStorageProviderItemPropertiesStatics {
    #[inline] pub fn set_async(&self, item: &super::IStorageItem, itemProperties: &foundation::collections::IIterable<StorageProviderItemProperty>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetAsync)(self.get_abi() as *const _ as *mut _, item.get_abi() as *const _ as *mut _, itemProperties.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageProviderItemProperty, 1198306648, 29451, 16776, 183, 181, 99, 183, 22, 237, 71, 109);
RT_INTERFACE!{interface IStorageProviderItemProperty(IStorageProviderItemPropertyVtbl, IStorageProviderItemProperty_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemProperty] {
    fn put_Id(&self, value: i32) -> HRESULT,
    fn get_Id(&self, out: *mut i32) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IconResource(&self, value: HSTRING) -> HRESULT,
    fn get_IconResource(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStorageProviderItemProperty {
    #[inline] pub fn set_id(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Id)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Value)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_icon_resource(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IconResource)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_icon_resource(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_IconResource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProviderItemProperty: IStorageProviderItemProperty}
impl RtActivatable<IActivationFactory> for StorageProviderItemProperty {}
DEFINE_CLSID!(StorageProviderItemProperty(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,121,0]) [CLSID_StorageProviderItemProperty]);
DEFINE_IID!(IID_IStorageProviderItemPropertyDefinition, 3316876219, 65311, 17048, 131, 30, 255, 28, 8, 8, 150, 144);
RT_INTERFACE!{interface IStorageProviderItemPropertyDefinition(IStorageProviderItemPropertyDefinitionVtbl, IStorageProviderItemPropertyDefinition_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertyDefinition] {
    fn get_Id(&self, out: *mut i32) -> HRESULT,
    fn put_Id(&self, value: i32) -> HRESULT,
    fn get_DisplayNameResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayNameResource(&self, value: HSTRING) -> HRESULT
}}
impl IStorageProviderItemPropertyDefinition {
    #[inline] pub fn get_id(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_id(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Id)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name_resource(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayNameResource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_display_name_resource(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DisplayNameResource)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProviderItemPropertyDefinition: IStorageProviderItemPropertyDefinition}
impl RtActivatable<IActivationFactory> for StorageProviderItemPropertyDefinition {}
DEFINE_CLSID!(StorageProviderItemPropertyDefinition(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,73,116,101,109,80,114,111,112,101,114,116,121,68,101,102,105,110,105,116,105,111,110,0]) [CLSID_StorageProviderItemPropertyDefinition]);
DEFINE_IID!(IID_IStorageProviderItemPropertySource, 2406456382, 63026, 19099, 141, 153, 210, 215, 161, 29, 245, 106);
RT_INTERFACE!{interface IStorageProviderItemPropertySource(IStorageProviderItemPropertySourceVtbl, IStorageProviderItemPropertySource_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderItemPropertySource] {
    fn GetItemProperties(&self, itemPath: HSTRING, out: *mut <foundation::collections::IIterable<StorageProviderItemProperty> as RtType>::Abi) -> HRESULT
}}
impl IStorageProviderItemPropertySource {
    #[inline] pub fn get_item_properties(&self, itemPath: &HStringArg) -> Result<Option<foundation::collections::IIterable<StorageProviderItemProperty>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemProperties)(self.get_abi() as *const _ as *mut _, itemPath.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IIterable::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorageProviderPopulationPolicy: i32 {
    Full = 1, AlwaysFull = 2,
}}
DEFINE_IID!(IID_IStorageProviderPropertyCapabilities, 1703751438, 25527, 17767, 172, 249, 81, 171, 227, 1, 221, 165);
RT_INTERFACE!{interface IStorageProviderPropertyCapabilities(IStorageProviderPropertyCapabilitiesVtbl, IStorageProviderPropertyCapabilities_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderPropertyCapabilities] {
    fn IsPropertySupported(&self, propertyCanonicalName: HSTRING, out: *mut bool) -> HRESULT
}}
impl IStorageProviderPropertyCapabilities {
    #[inline] pub fn is_property_supported(&self, propertyCanonicalName: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsPropertySupported)(self.get_abi() as *const _ as *mut _, propertyCanonicalName.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorageProviderProtectionMode: i32 {
    Unknown = 0, Personal = 1,
}}
DEFINE_IID!(IID_IStorageProviderSyncRootInfo, 2081621444, 39417, 16812, 137, 4, 171, 5, 93, 101, 73, 38);
RT_INTERFACE!{interface IStorageProviderSyncRootInfo(IStorageProviderSyncRootInfoVtbl, IStorageProviderSyncRootInfo_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderSyncRootInfo] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Context(&self, out: *mut <super::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn put_Context(&self, value: <super::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn get_Path(&self, out: *mut <super::IStorageFolder as RtType>::Abi) -> HRESULT,
    fn put_Path(&self, value: <super::IStorageFolder as RtType>::Abi) -> HRESULT,
    fn get_DisplayNameResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_DisplayNameResource(&self, value: HSTRING) -> HRESULT,
    fn get_IconResource(&self, out: *mut HSTRING) -> HRESULT,
    fn put_IconResource(&self, value: HSTRING) -> HRESULT,
    fn get_HydrationPolicy(&self, out: *mut StorageProviderHydrationPolicy) -> HRESULT,
    fn put_HydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> HRESULT,
    fn get_HydrationPolicyModifier(&self, out: *mut StorageProviderHydrationPolicyModifier) -> HRESULT,
    fn put_HydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> HRESULT,
    fn get_PopulationPolicy(&self, out: *mut StorageProviderPopulationPolicy) -> HRESULT,
    fn put_PopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> HRESULT,
    fn get_InSyncPolicy(&self, out: *mut StorageProviderInSyncPolicy) -> HRESULT,
    fn put_InSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> HRESULT,
    fn get_HardlinkPolicy(&self, out: *mut StorageProviderHardlinkPolicy) -> HRESULT,
    fn put_HardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> HRESULT,
    fn get_ShowSiblingsAsGroup(&self, out: *mut bool) -> HRESULT,
    fn put_ShowSiblingsAsGroup(&self, value: bool) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT,
    fn get_ProtectionMode(&self, out: *mut StorageProviderProtectionMode) -> HRESULT,
    fn put_ProtectionMode(&self, value: StorageProviderProtectionMode) -> HRESULT,
    fn get_AllowPinning(&self, out: *mut bool) -> HRESULT,
    fn put_AllowPinning(&self, value: bool) -> HRESULT,
    fn get_StorageProviderItemPropertyDefinitions(&self, out: *mut <foundation::collections::IVector<StorageProviderItemPropertyDefinition> as RtType>::Abi) -> HRESULT,
    fn get_RecycleBinUri(&self, out: *mut <foundation::Uri as RtType>::Abi) -> HRESULT,
    fn put_RecycleBinUri(&self, value: <foundation::Uri as RtType>::Abi) -> HRESULT
}}
impl IStorageProviderSyncRootInfo {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Id)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_context(&self) -> Result<Option<super::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Context)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_context(&self, value: &super::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Context)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_path(&self) -> Result<Option<super::IStorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Path)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::IStorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_path(&self, value: &super::IStorageFolder) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Path)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name_resource(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayNameResource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_display_name_resource(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DisplayNameResource)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_icon_resource(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_IconResource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_icon_resource(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IconResource)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hydration_policy(&self) -> Result<StorageProviderHydrationPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HydrationPolicy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hydration_policy(&self, value: StorageProviderHydrationPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_HydrationPolicy)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hydration_policy_modifier(&self) -> Result<StorageProviderHydrationPolicyModifier> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HydrationPolicyModifier)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hydration_policy_modifier(&self, value: StorageProviderHydrationPolicyModifier) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_HydrationPolicyModifier)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_population_policy(&self) -> Result<StorageProviderPopulationPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PopulationPolicy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_population_policy(&self, value: StorageProviderPopulationPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_PopulationPolicy)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_in_sync_policy(&self) -> Result<StorageProviderInSyncPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_InSyncPolicy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_in_sync_policy(&self, value: StorageProviderInSyncPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_InSyncPolicy)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hardlink_policy(&self) -> Result<StorageProviderHardlinkPolicy> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HardlinkPolicy)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hardlink_policy(&self, value: StorageProviderHardlinkPolicy) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_HardlinkPolicy)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_show_siblings_as_group(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ShowSiblingsAsGroup)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_show_siblings_as_group(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ShowSiblingsAsGroup)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Version)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_version(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Version)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_protection_mode(&self) -> Result<StorageProviderProtectionMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ProtectionMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_protection_mode(&self, value: StorageProviderProtectionMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ProtectionMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_pinning(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AllowPinning)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_pinning(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AllowPinning)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_storage_provider_item_property_definitions(&self) -> Result<Option<foundation::collections::IVector<StorageProviderItemPropertyDefinition>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_StorageProviderItemPropertyDefinitions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_recycle_bin_uri(&self) -> Result<Option<foundation::Uri>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RecycleBinUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Uri::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_recycle_bin_uri(&self, value: &foundation::Uri) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_RecycleBinUri)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageProviderSyncRootInfo: IStorageProviderSyncRootInfo}
impl RtActivatable<IActivationFactory> for StorageProviderSyncRootInfo {}
DEFINE_CLSID!(StorageProviderSyncRootInfo(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,83,121,110,99,82,111,111,116,73,110,102,111,0]) [CLSID_StorageProviderSyncRootInfo]);
DEFINE_IID!(IID_IStorageProviderSyncRootInfo2, 3478237219, 31985, 20838, 189, 186, 239, 217, 95, 82, 158, 49);
RT_INTERFACE!{interface IStorageProviderSyncRootInfo2(IStorageProviderSyncRootInfo2Vtbl, IStorageProviderSyncRootInfo2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderSyncRootInfo2] {
    fn get_ProviderId(&self, out: *mut Guid) -> HRESULT,
    fn put_ProviderId(&self, value: Guid) -> HRESULT
}}
impl IStorageProviderSyncRootInfo2 {
    #[inline] pub fn get_provider_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ProviderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_provider_id(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ProviderId)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{static class StorageProviderSyncRootManager}
impl RtActivatable<IStorageProviderSyncRootManagerStatics> for StorageProviderSyncRootManager {}
impl StorageProviderSyncRootManager {
    #[inline] pub fn register(syncRootInformation: &StorageProviderSyncRootInfo) -> Result<()> {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().register(syncRootInformation)
    }
    #[inline] pub fn unregister(id: &HStringArg) -> Result<()> {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().unregister(id)
    }
    #[inline] pub fn get_sync_root_information_for_folder(folder: &super::IStorageFolder) -> Result<Option<StorageProviderSyncRootInfo>> {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_sync_root_information_for_folder(folder)
    }
    #[inline] pub fn get_sync_root_information_for_id(id: &HStringArg) -> Result<Option<StorageProviderSyncRootInfo>> {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_sync_root_information_for_id(id)
    }
    #[inline] pub fn get_current_sync_roots() -> Result<Option<foundation::collections::IVectorView<StorageProviderSyncRootInfo>>> {
        <Self as RtActivatable<IStorageProviderSyncRootManagerStatics>>::get_activation_factory().get_current_sync_roots()
    }
}
DEFINE_CLSID!(StorageProviderSyncRootManager(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,80,114,111,118,105,100,101,114,46,83,116,111,114,97,103,101,80,114,111,118,105,100,101,114,83,121,110,99,82,111,111,116,77,97,110,97,103,101,114,0]) [CLSID_StorageProviderSyncRootManager]);
DEFINE_IID!(IID_IStorageProviderSyncRootManagerStatics, 1050278847, 36835, 19264, 171, 199, 246, 252, 61, 116, 201, 142);
RT_INTERFACE!{static interface IStorageProviderSyncRootManagerStatics(IStorageProviderSyncRootManagerStaticsVtbl, IStorageProviderSyncRootManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderSyncRootManagerStatics] {
    fn Register(&self, syncRootInformation: <StorageProviderSyncRootInfo as RtType>::Abi) -> HRESULT,
    fn Unregister(&self, id: HSTRING) -> HRESULT,
    fn GetSyncRootInformationForFolder(&self, folder: <super::IStorageFolder as RtType>::Abi, out: *mut <StorageProviderSyncRootInfo as RtType>::Abi) -> HRESULT,
    fn GetSyncRootInformationForId(&self, id: HSTRING, out: *mut <StorageProviderSyncRootInfo as RtType>::Abi) -> HRESULT,
    fn GetCurrentSyncRoots(&self, out: *mut <foundation::collections::IVectorView<StorageProviderSyncRootInfo> as RtType>::Abi) -> HRESULT
}}
impl IStorageProviderSyncRootManagerStatics {
    #[inline] pub fn register(&self, syncRootInformation: &StorageProviderSyncRootInfo) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Register)(self.get_abi() as *const _ as *mut _, syncRootInformation.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn unregister(&self, id: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Unregister)(self.get_abi() as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_sync_root_information_for_folder(&self, folder: &super::IStorageFolder) -> Result<Option<StorageProviderSyncRootInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSyncRootInformationForFolder)(self.get_abi() as *const _ as *mut _, folder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageProviderSyncRootInfo::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_sync_root_information_for_id(&self, id: &HStringArg) -> Result<Option<StorageProviderSyncRootInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSyncRootInformationForId)(self.get_abi() as *const _ as *mut _, id.get(), &mut out);
        if hr == S_OK { Ok(StorageProviderSyncRootInfo::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_sync_roots(&self) -> Result<Option<foundation::collections::IVectorView<StorageProviderSyncRootInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCurrentSyncRoots)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageProviderUriSource, 2996307665, 35808, 18786, 139, 182, 13, 76, 46, 20, 212, 122);
RT_INTERFACE!{interface IStorageProviderUriSource(IStorageProviderUriSourceVtbl, IStorageProviderUriSource_Abi): IInspectable(IInspectableVtbl) [IID_IStorageProviderUriSource] {
    fn GetPathForContentUri(&self, contentUri: HSTRING, result: <StorageProviderGetPathForContentUriResult as RtType>::Abi) -> HRESULT,
    fn GetContentInfoForPath(&self, path: HSTRING, result: <StorageProviderGetContentInfoForPathResult as RtType>::Abi) -> HRESULT
}}
impl IStorageProviderUriSource {
    #[inline] pub fn get_path_for_content_uri(&self, contentUri: &HStringArg, result: &StorageProviderGetPathForContentUriResult) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).GetPathForContentUri)(self.get_abi() as *const _ as *mut _, contentUri.get(), result.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_content_info_for_path(&self, path: &HStringArg, result: &StorageProviderGetContentInfoForPathResult) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).GetContentInfoForPath)(self.get_abi() as *const _ as *mut _, path.get(), result.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum StorageProviderUriSourceStatus: i32 {
    Success = 0, NoSyncRoot = 1, FileNotFound = 2,
}}
RT_ENUM! { enum UIStatus: i32 {
    Unavailable = 0, Hidden = 1, Visible = 2, Complete = 3,
}}
RT_ENUM! { enum WriteActivationMode: i32 {
    ReadOnly = 0, NotNeeded = 1, AfterWrite = 2,
}}
} // Windows.Storage.Provider
pub mod search { // Windows.Storage.Search
use crate::prelude::*;
RT_ENUM! { enum CommonFileQuery: i32 {
    DefaultQuery = 0, OrderByName = 1, OrderByTitle = 2, OrderByMusicProperties = 3, OrderBySearchRank = 4, OrderByDate = 5,
}}
RT_ENUM! { enum CommonFolderQuery: i32 {
    DefaultQuery = 0, GroupByYear = 100, GroupByMonth = 101, GroupByArtist = 102, GroupByAlbum = 103, GroupByAlbumArtist = 104, GroupByComposer = 105, GroupByGenre = 106, GroupByPublishedYear = 107, GroupByRating = 108, GroupByTag = 109, GroupByAuthor = 110, GroupByType = 111,
}}
DEFINE_IID!(IID_IContentIndexer, 2977333133, 63128, 18818, 176, 95, 58, 110, 140, 171, 1, 162);
RT_INTERFACE!{interface IContentIndexer(IContentIndexerVtbl, IContentIndexer_Abi): IInspectable(IInspectableVtbl) [IID_IContentIndexer] {
    fn AddAsync(&self, indexableContent: <IIndexableContent as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn UpdateAsync(&self, indexableContent: <IIndexableContent as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn DeleteAsync(&self, contentId: HSTRING, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn DeleteMultipleAsync(&self, contentIds: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn DeleteAllAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn RetrievePropertiesAsync(&self, contentId: HSTRING, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IMapView<HString, IInspectable>> as RtType>::Abi) -> HRESULT,
    fn get_Revision(&self, out: *mut u64) -> HRESULT
}}
impl IContentIndexer {
    #[inline] pub fn add_async(&self, indexableContent: &IIndexableContent) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AddAsync)(self.get_abi() as *const _ as *mut _, indexableContent.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_async(&self, indexableContent: &IIndexableContent) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).UpdateAsync)(self.get_abi() as *const _ as *mut _, indexableContent.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_async(&self, contentId: &HStringArg) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DeleteAsync)(self.get_abi() as *const _ as *mut _, contentId.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_multiple_async(&self, contentIds: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DeleteMultipleAsync)(self.get_abi() as *const _ as *mut _, contentIds.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn delete_all_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DeleteAllAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn retrieve_properties_async(&self, contentId: &HStringArg, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<foundation::collections::IMapView<HString, IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RetrievePropertiesAsync)(self.get_abi() as *const _ as *mut _, contentId.get(), propertiesToRetrieve.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_revision(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Revision)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ContentIndexer: IContentIndexer}
impl RtActivatable<IContentIndexerStatics> for ContentIndexer {}
impl ContentIndexer {
    #[inline] pub fn get_indexer_with_name(indexName: &HStringArg) -> Result<Option<ContentIndexer>> {
        <Self as RtActivatable<IContentIndexerStatics>>::get_activation_factory().get_indexer_with_name(indexName)
    }
    #[inline] pub fn get_indexer() -> Result<Option<ContentIndexer>> {
        <Self as RtActivatable<IContentIndexerStatics>>::get_activation_factory().get_indexer()
    }
}
DEFINE_CLSID!(ContentIndexer(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,67,111,110,116,101,110,116,73,110,100,101,120,101,114,0]) [CLSID_ContentIndexer]);
DEFINE_IID!(IID_IContentIndexerQuery, 1893970168, 19452, 17034, 136, 137, 204, 81, 218, 154, 123, 157);
RT_INTERFACE!{interface IContentIndexerQuery(IContentIndexerQueryVtbl, IContentIndexerQuery_Abi): IInspectable(IInspectableVtbl) [IID_IContentIndexerQuery] {
    fn GetCountAsync(&self, out: *mut <foundation::IAsyncOperation<u32> as RtType>::Abi) -> HRESULT,
    fn GetPropertiesAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<foundation::collections::IMapView<HString, IInspectable>>> as RtType>::Abi) -> HRESULT,
    fn GetPropertiesRangeAsync(&self, startIndex: u32, maxItems: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<foundation::collections::IMapView<HString, IInspectable>>> as RtType>::Abi) -> HRESULT,
    fn GetAsync(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<IIndexableContent>> as RtType>::Abi) -> HRESULT,
    fn GetRangeAsync(&self, startIndex: u32, maxItems: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<IIndexableContent>> as RtType>::Abi) -> HRESULT,
    fn get_QueryFolder(&self, out: *mut <super::StorageFolder as RtType>::Abi) -> HRESULT
}}
impl IContentIndexerQuery {
    #[inline] pub fn get_count_async(&self) -> Result<foundation::IAsyncOperation<u32>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCountAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPropertiesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties_range_async(&self, startIndex: u32, maxItems: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPropertiesRangeAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItems, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_async(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<IIndexableContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_range_async(&self, startIndex: u32, maxItems: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<IIndexableContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetRangeAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItems, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_query_folder(&self) -> Result<Option<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_QueryFolder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFolder::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ContentIndexerQuery: IContentIndexerQuery}
DEFINE_IID!(IID_IContentIndexerQueryOperations, 679624208, 18310, 17137, 151, 48, 121, 43, 53, 102, 177, 80);
RT_INTERFACE!{interface IContentIndexerQueryOperations(IContentIndexerQueryOperationsVtbl, IContentIndexerQueryOperations_Abi): IInspectable(IInspectableVtbl) [IID_IContentIndexerQueryOperations] {
    fn CreateQueryWithSortOrderAndLanguage(&self, searchFilter: HSTRING, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, sortOrder: <foundation::collections::IIterable<SortEntry> as RtType>::Abi, searchFilterLanguage: HSTRING, out: *mut <ContentIndexerQuery as RtType>::Abi) -> HRESULT,
    fn CreateQueryWithSortOrder(&self, searchFilter: HSTRING, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, sortOrder: <foundation::collections::IIterable<SortEntry> as RtType>::Abi, out: *mut <ContentIndexerQuery as RtType>::Abi) -> HRESULT,
    fn CreateQuery(&self, searchFilter: HSTRING, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <ContentIndexerQuery as RtType>::Abi) -> HRESULT
}}
impl IContentIndexerQueryOperations {
    #[inline] pub fn create_query_with_sort_order_and_language(&self, searchFilter: &HStringArg, propertiesToRetrieve: &foundation::collections::IIterable<HString>, sortOrder: &foundation::collections::IIterable<SortEntry>, searchFilterLanguage: &HStringArg) -> Result<Option<ContentIndexerQuery>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateQueryWithSortOrderAndLanguage)(self.get_abi() as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve.get_abi() as *const _ as *mut _, sortOrder.get_abi() as *const _ as *mut _, searchFilterLanguage.get(), &mut out);
        if hr == S_OK { Ok(ContentIndexerQuery::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_query_with_sort_order(&self, searchFilter: &HStringArg, propertiesToRetrieve: &foundation::collections::IIterable<HString>, sortOrder: &foundation::collections::IIterable<SortEntry>) -> Result<Option<ContentIndexerQuery>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateQueryWithSortOrder)(self.get_abi() as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve.get_abi() as *const _ as *mut _, sortOrder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ContentIndexerQuery::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_query(&self, searchFilter: &HStringArg, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<Option<ContentIndexerQuery>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateQuery)(self.get_abi() as *const _ as *mut _, searchFilter.get(), propertiesToRetrieve.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ContentIndexerQuery::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IContentIndexerStatics, 2353562485, 45950, 19552, 155, 168, 183, 96, 253, 163, 229, 157);
RT_INTERFACE!{static interface IContentIndexerStatics(IContentIndexerStaticsVtbl, IContentIndexerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IContentIndexerStatics] {
    fn GetIndexerWithName(&self, indexName: HSTRING, out: *mut <ContentIndexer as RtType>::Abi) -> HRESULT,
    fn GetIndexer(&self, out: *mut <ContentIndexer as RtType>::Abi) -> HRESULT
}}
impl IContentIndexerStatics {
    #[inline] pub fn get_indexer_with_name(&self, indexName: &HStringArg) -> Result<Option<ContentIndexer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetIndexerWithName)(self.get_abi() as *const _ as *mut _, indexName.get(), &mut out);
        if hr == S_OK { Ok(ContentIndexer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_indexer(&self) -> Result<Option<ContentIndexer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetIndexer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ContentIndexer::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum DateStackOption: i32 {
    None = 0, Year = 1, Month = 2,
}}
RT_ENUM! { enum FolderDepth: i32 {
    Shallow = 0, Deep = 1,
}}
DEFINE_IID!(IID_IIndexableContent, 3438387295, 54453, 18490, 176, 110, 224, 219, 30, 196, 32, 228);
RT_INTERFACE!{interface IIndexableContent(IIndexableContentVtbl, IIndexableContent_Abi): IInspectable(IInspectableVtbl) [IID_IIndexableContent] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Id(&self, value: HSTRING) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::IMap<HString, IInspectable> as RtType>::Abi) -> HRESULT,
    fn get_Stream(&self, out: *mut <super::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn put_Stream(&self, value: <super::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn get_StreamContentType(&self, out: *mut HSTRING) -> HRESULT,
    fn put_StreamContentType(&self, value: HSTRING) -> HRESULT
}}
impl IIndexableContent {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_id(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Id)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::IMap<HString, IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_stream(&self) -> Result<Option<super::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Stream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_stream(&self, value: &super::streams::IRandomAccessStream) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Stream)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stream_content_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_StreamContentType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_stream_content_type(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_StreamContentType)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class IndexableContent: IIndexableContent}
impl RtActivatable<IActivationFactory> for IndexableContent {}
DEFINE_CLSID!(IndexableContent(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,73,110,100,101,120,97,98,108,101,67,111,110,116,101,110,116,0]) [CLSID_IndexableContent]);
RT_ENUM! { enum IndexedState: i32 {
    Unknown = 0, NotIndexed = 1, PartiallyIndexed = 2, FullyIndexed = 3,
}}
RT_ENUM! { enum IndexerOption: i32 {
    UseIndexerWhenAvailable = 0, OnlyUseIndexer = 1, DoNotUseIndexer = 2, OnlyUseIndexerAndOptimizeForIndexedProperties = 3,
}}
DEFINE_IID!(IID_IQueryOptions, 509495022, 3909, 18488, 168, 233, 208, 71, 157, 68, 108, 48);
RT_INTERFACE!{interface IQueryOptions(IQueryOptionsVtbl, IQueryOptions_Abi): IInspectable(IInspectableVtbl) [IID_IQueryOptions] {
    fn get_FileTypeFilter(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_FolderDepth(&self, out: *mut FolderDepth) -> HRESULT,
    fn put_FolderDepth(&self, value: FolderDepth) -> HRESULT,
    fn get_ApplicationSearchFilter(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ApplicationSearchFilter(&self, value: HSTRING) -> HRESULT,
    fn get_UserSearchFilter(&self, out: *mut HSTRING) -> HRESULT,
    fn put_UserSearchFilter(&self, value: HSTRING) -> HRESULT,
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_IndexerOption(&self, out: *mut IndexerOption) -> HRESULT,
    fn put_IndexerOption(&self, value: IndexerOption) -> HRESULT,
    fn get_SortOrder(&self, out: *mut <foundation::collections::IVector<SortEntry> as RtType>::Abi) -> HRESULT,
    fn get_GroupPropertyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_DateStackOption(&self, out: *mut DateStackOption) -> HRESULT,
    fn SaveToString(&self, out: *mut HSTRING) -> HRESULT,
    fn LoadFromString(&self, value: HSTRING) -> HRESULT,
    fn SetThumbnailPrefetch(&self, mode: super::fileproperties::ThumbnailMode, requestedSize: u32, options: super::fileproperties::ThumbnailOptions) -> HRESULT,
    fn SetPropertyPrefetch(&self, options: super::fileproperties::PropertyPrefetchOptions, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi) -> HRESULT
}}
impl IQueryOptions {
    #[inline] pub fn get_file_type_filter(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileTypeFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder_depth(&self) -> Result<FolderDepth> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_FolderDepth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_folder_depth(&self, value: FolderDepth) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_FolderDepth)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_application_search_filter(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ApplicationSearchFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_application_search_filter(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ApplicationSearchFilter)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_user_search_filter(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_UserSearchFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_user_search_filter(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_UserSearchFilter)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Language)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_language(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Language)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_indexer_option(&self) -> Result<IndexerOption> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IndexerOption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_indexer_option(&self, value: IndexerOption) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IndexerOption)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_sort_order(&self) -> Result<Option<foundation::collections::IVector<SortEntry>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SortOrder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_group_property_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_GroupPropertyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_date_stack_option(&self) -> Result<DateStackOption> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DateStackOption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn save_to_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SaveToString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_string(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).LoadFromString)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbnail_prefetch(&self, mode: super::fileproperties::ThumbnailMode, requestedSize: u32, options: super::fileproperties::ThumbnailOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetThumbnailPrefetch)(self.get_abi() as *const _ as *mut _, mode, requestedSize, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_property_prefetch(&self, options: super::fileproperties::PropertyPrefetchOptions, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetPropertyPrefetch)(self.get_abi() as *const _ as *mut _, options, propertiesToRetrieve.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class QueryOptions: IQueryOptions}
impl RtActivatable<IQueryOptionsFactory> for QueryOptions {}
impl RtActivatable<IActivationFactory> for QueryOptions {}
impl QueryOptions {
    #[inline] pub fn create_common_file_query(query: CommonFileQuery, fileTypeFilter: &foundation::collections::IIterable<HString>) -> Result<QueryOptions> {
        <Self as RtActivatable<IQueryOptionsFactory>>::get_activation_factory().create_common_file_query(query, fileTypeFilter)
    }
    #[inline] pub fn create_common_folder_query(query: CommonFolderQuery) -> Result<QueryOptions> {
        <Self as RtActivatable<IQueryOptionsFactory>>::get_activation_factory().create_common_folder_query(query)
    }
}
DEFINE_CLSID!(QueryOptions(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,81,117,101,114,121,79,112,116,105,111,110,115,0]) [CLSID_QueryOptions]);
DEFINE_IID!(IID_IQueryOptionsFactory, 53354380, 43457, 20081, 128, 17, 13, 238, 157, 72, 17, 163);
RT_INTERFACE!{static interface IQueryOptionsFactory(IQueryOptionsFactoryVtbl, IQueryOptionsFactory_Abi): IInspectable(IInspectableVtbl) [IID_IQueryOptionsFactory] {
    fn CreateCommonFileQuery(&self, query: CommonFileQuery, fileTypeFilter: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <QueryOptions as RtType>::Abi) -> HRESULT,
    fn CreateCommonFolderQuery(&self, query: CommonFolderQuery, out: *mut <QueryOptions as RtType>::Abi) -> HRESULT
}}
impl IQueryOptionsFactory {
    #[inline] pub fn create_common_file_query(&self, query: CommonFileQuery, fileTypeFilter: &foundation::collections::IIterable<HString>) -> Result<QueryOptions> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCommonFileQuery)(self.get_abi() as *const _ as *mut _, query, fileTypeFilter.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(QueryOptions::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_common_folder_query(&self, query: CommonFolderQuery) -> Result<QueryOptions> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCommonFolderQuery)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(QueryOptions::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IQueryOptionsWithProviderFilter, 1537019942, 5572, 17629, 184, 154, 71, 165, 155, 125, 124, 79);
RT_INTERFACE!{interface IQueryOptionsWithProviderFilter(IQueryOptionsWithProviderFilterVtbl, IQueryOptionsWithProviderFilter_Abi): IInspectable(IInspectableVtbl) [IID_IQueryOptionsWithProviderFilter] {
    fn get_StorageProviderIdFilter(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT
}}
impl IQueryOptionsWithProviderFilter {
    #[inline] pub fn get_storage_provider_id_filter(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_StorageProviderIdFilter)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct SortEntry {
    PropertyName: HSTRING, AscendingOrder: bool,
}}
RT_CLASS!{class SortEntryVector: foundation::collections::IVector<SortEntry>}
DEFINE_IID!(IID_IStorageFileQueryResult, 1392354375, 11178, 16684, 178, 159, 212, 177, 119, 142, 250, 30);
RT_INTERFACE!{interface IStorageFileQueryResult(IStorageFileQueryResultVtbl, IStorageFileQueryResult_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFileQueryResult] {
    fn GetFilesAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>> as RtType>::Abi) -> HRESULT,
    fn GetFilesAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>> as RtType>::Abi) -> HRESULT
}}
impl IStorageFileQueryResult {
    #[inline] pub fn get_files_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageFileQueryResult: IStorageFileQueryResult}
DEFINE_IID!(IID_IStorageFileQueryResult2, 1314765277, 28993, 18116, 139, 227, 233, 220, 158, 39, 39, 92);
RT_INTERFACE!{interface IStorageFileQueryResult2(IStorageFileQueryResult2Vtbl, IStorageFileQueryResult2_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFileQueryResult2] {
    #[cfg(feature="windows-data")] fn GetMatchingPropertiesWithRanges(&self, file: <super::StorageFile as RtType>::Abi, out: *mut <foundation::collections::IMap<HString, foundation::collections::IVectorView<super::super::data::text::TextSegment>> as RtType>::Abi) -> HRESULT
}}
impl IStorageFileQueryResult2 {
    #[cfg(feature="windows-data")] #[inline] pub fn get_matching_properties_with_ranges(&self, file: &super::StorageFile) -> Result<Option<foundation::collections::IMap<HString, foundation::collections::IVectorView<super::super::data::text::TextSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetMatchingPropertiesWithRanges)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFolderQueryOperations, 3410218185, 17515, 19023, 190, 151, 117, 119, 113, 190, 82, 3);
RT_INTERFACE!{interface IStorageFolderQueryOperations(IStorageFolderQueryOperationsVtbl, IStorageFolderQueryOperations_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolderQueryOperations] {
    fn GetIndexedStateAsync(&self, out: *mut <foundation::IAsyncOperation<IndexedState> as RtType>::Abi) -> HRESULT,
    fn CreateFileQueryOverloadDefault(&self, out: *mut <StorageFileQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateFileQuery(&self, query: CommonFileQuery, out: *mut <StorageFileQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateFileQueryWithOptions(&self, queryOptions: <QueryOptions as RtType>::Abi, out: *mut <StorageFileQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateFolderQueryOverloadDefault(&self, out: *mut <StorageFolderQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateFolderQuery(&self, query: CommonFolderQuery, out: *mut <StorageFolderQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateFolderQueryWithOptions(&self, queryOptions: <QueryOptions as RtType>::Abi, out: *mut <StorageFolderQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateItemQuery(&self, out: *mut <StorageItemQueryResult as RtType>::Abi) -> HRESULT,
    fn CreateItemQueryWithOptions(&self, queryOptions: <QueryOptions as RtType>::Abi, out: *mut <StorageItemQueryResult as RtType>::Abi) -> HRESULT,
    fn GetFilesAsync(&self, query: CommonFileQuery, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>> as RtType>::Abi) -> HRESULT,
    fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsync(&self, query: CommonFolderQuery, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>> as RtType>::Abi) -> HRESULT,
    fn GetItemsAsync(&self, startIndex: u32, maxItemsToRetrieve: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>> as RtType>::Abi) -> HRESULT,
    fn AreQueryOptionsSupported(&self, queryOptions: <QueryOptions as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery, out: *mut bool) -> HRESULT,
    fn IsCommonFileQuerySupported(&self, query: CommonFileQuery, out: *mut bool) -> HRESULT
}}
impl IStorageFolderQueryOperations {
    #[inline] pub fn get_indexed_state_async(&self) -> Result<foundation::IAsyncOperation<IndexedState>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetIndexedStateAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_file_query_overload_default(&self) -> Result<Option<StorageFileQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileQueryOverloadDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFileQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_file_query(&self, query: CommonFileQuery) -> Result<Option<StorageFileQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileQuery)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(StorageFileQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_file_query_with_options(&self, queryOptions: &QueryOptions) -> Result<Option<StorageFileQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFileQueryWithOptions)(self.get_abi() as *const _ as *mut _, queryOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFileQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_query_overload_default(&self) -> Result<Option<StorageFolderQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderQueryOverloadDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolderQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_query(&self, query: CommonFolderQuery) -> Result<Option<StorageFolderQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderQuery)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(StorageFolderQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_folder_query_with_options(&self, queryOptions: &QueryOptions) -> Result<Option<StorageFolderQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFolderQueryWithOptions)(self.get_abi() as *const _ as *mut _, queryOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageFolderQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_item_query(&self) -> Result<Option<StorageItemQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateItemQuery)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageItemQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_item_query_with_options(&self, queryOptions: &QueryOptions) -> Result<Option<StorageItemQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateItemQueryWithOptions)(self.get_abi() as *const _ as *mut _, queryOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(StorageItemQueryResult::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async(&self, query: CommonFileQuery, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsync)(self.get_abi() as *const _ as *mut _, query, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_files_async_overload_default_start_and_count(&self, query: CommonFileQuery) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFile>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFilesAsyncOverloadDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async(&self, query: CommonFolderQuery, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsync)(self.get_abi() as *const _ as *mut _, query, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async_overload_default_start_and_count(&self, query: CommonFolderQuery) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsyncOverloadDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_items_async(&self, startIndex: u32, maxItemsToRetrieve: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxItemsToRetrieve, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn are_query_options_supported(&self, queryOptions: &QueryOptions) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).AreQueryOptionsSupported)(self.get_abi() as *const _ as *mut _, queryOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_common_folder_query_supported(&self, query: CommonFolderQuery) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsCommonFolderQuerySupported)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_common_file_query_supported(&self, query: CommonFileQuery) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsCommonFileQuerySupported)(self.get_abi() as *const _ as *mut _, query, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStorageFolderQueryResult, 1716832529, 32102, 18170, 174, 207, 228, 164, 186, 169, 58, 184);
RT_INTERFACE!{interface IStorageFolderQueryResult(IStorageFolderQueryResultVtbl, IStorageFolderQueryResult_Abi): IInspectable(IInspectableVtbl) [IID_IStorageFolderQueryResult] {
    fn GetFoldersAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>> as RtType>::Abi) -> HRESULT,
    fn GetFoldersAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>> as RtType>::Abi) -> HRESULT
}}
impl IStorageFolderQueryResult {
    #[inline] pub fn get_folders_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folders_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::StorageFolder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFoldersAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageFolderQueryResult: IStorageFolderQueryResult}
DEFINE_IID!(IID_IStorageItemQueryResult, 3902046329, 40280, 18360, 178, 178, 65, 176, 127, 71, 149, 249);
RT_INTERFACE!{interface IStorageItemQueryResult(IStorageItemQueryResultVtbl, IStorageItemQueryResult_Abi): IInspectable(IInspectableVtbl) [IID_IStorageItemQueryResult] {
    fn GetItemsAsync(&self, startIndex: u32, maxNumberOfItems: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>> as RtType>::Abi) -> HRESULT,
    fn GetItemsAsyncDefaultStartAndCount(&self, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>> as RtType>::Abi) -> HRESULT
}}
impl IStorageItemQueryResult {
    #[inline] pub fn get_items_async(&self, startIndex: u32, maxNumberOfItems: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsync)(self.get_abi() as *const _ as *mut _, startIndex, maxNumberOfItems, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_items_async_default_start_and_count(&self) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<super::IStorageItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemsAsyncDefaultStartAndCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageItemQueryResult: IStorageItemQueryResult}
DEFINE_IID!(IID_IStorageLibraryChangeTrackerTriggerDetails, 499622761, 47011, 19954, 157, 97, 235, 168, 90, 3, 67, 210);
RT_INTERFACE!{interface IStorageLibraryChangeTrackerTriggerDetails(IStorageLibraryChangeTrackerTriggerDetailsVtbl, IStorageLibraryChangeTrackerTriggerDetails_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryChangeTrackerTriggerDetails] {
    fn get_Folder(&self, out: *mut <super::StorageFolder as RtType>::Abi) -> HRESULT,
    fn get_ChangeTracker(&self, out: *mut <super::StorageLibraryChangeTracker as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryChangeTrackerTriggerDetails {
    #[inline] pub fn get_folder(&self) -> Result<Option<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Folder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_change_tracker(&self) -> Result<Option<super::StorageLibraryChangeTracker>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ChangeTracker)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageLibraryChangeTracker::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibraryChangeTrackerTriggerDetails: IStorageLibraryChangeTrackerTriggerDetails}
DEFINE_IID!(IID_IStorageLibraryContentChangedTriggerDetails, 708254071, 43967, 19997, 138, 165, 99, 133, 216, 136, 71, 153);
RT_INTERFACE!{interface IStorageLibraryContentChangedTriggerDetails(IStorageLibraryContentChangedTriggerDetailsVtbl, IStorageLibraryContentChangedTriggerDetails_Abi): IInspectable(IInspectableVtbl) [IID_IStorageLibraryContentChangedTriggerDetails] {
    fn get_Folder(&self, out: *mut <super::StorageFolder as RtType>::Abi) -> HRESULT,
    fn CreateModifiedSinceQuery(&self, lastQueryTime: foundation::DateTime, out: *mut <StorageItemQueryResult as RtType>::Abi) -> HRESULT
}}
impl IStorageLibraryContentChangedTriggerDetails {
    #[inline] pub fn get_folder(&self) -> Result<Option<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Folder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_modified_since_query(&self, lastQueryTime: foundation::DateTime) -> Result<Option<StorageItemQueryResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateModifiedSinceQuery)(self.get_abi() as *const _ as *mut _, lastQueryTime, &mut out);
        if hr == S_OK { Ok(StorageItemQueryResult::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class StorageLibraryContentChangedTriggerDetails: IStorageLibraryContentChangedTriggerDetails}
DEFINE_IID!(IID_IStorageQueryResultBase, 3264730893, 29523, 18347, 186, 88, 140, 97, 66, 93, 197, 75);
RT_INTERFACE!{interface IStorageQueryResultBase(IStorageQueryResultBaseVtbl, IStorageQueryResultBase_Abi): IInspectable(IInspectableVtbl) [IID_IStorageQueryResultBase] {
    fn GetItemCountAsync(&self, out: *mut <foundation::IAsyncOperation<u32> as RtType>::Abi) -> HRESULT,
    fn get_Folder(&self, out: *mut <super::StorageFolder as RtType>::Abi) -> HRESULT,
    fn add_ContentsChanged(&self, handler: <foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ContentsChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_OptionsChanged(&self, changedHandler: <foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OptionsChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn FindStartIndexAsync(&self, value: <IInspectable as RtType>::Abi, out: *mut <foundation::IAsyncOperation<u32> as RtType>::Abi) -> HRESULT,
    fn GetCurrentQueryOptions(&self, out: *mut <QueryOptions as RtType>::Abi) -> HRESULT,
    fn ApplyNewQueryOptions(&self, newQueryOptions: <QueryOptions as RtType>::Abi) -> HRESULT
}}
impl IStorageQueryResultBase {
    #[inline] pub fn get_item_count_async(&self) -> Result<foundation::IAsyncOperation<u32>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetItemCountAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_folder(&self) -> Result<Option<super::StorageFolder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Folder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::StorageFolder::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_contents_changed(&self, handler: &foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_ContentsChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_contents_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_ContentsChanged)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_options_changed(&self, changedHandler: &foundation::TypedEventHandler<IStorageQueryResultBase, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_OptionsChanged)(self.get_abi() as *const _ as *mut _, changedHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_options_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_OptionsChanged)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn find_start_index_async(&self, value: &IInspectable) -> Result<foundation::IAsyncOperation<u32>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FindStartIndexAsync)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_query_options(&self) -> Result<Option<QueryOptions>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCurrentQueryOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(QueryOptions::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn apply_new_query_options(&self, newQueryOptions: &QueryOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).ApplyNewQueryOptions)(self.get_abi() as *const _ as *mut _, newQueryOptions.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IValueAndLanguage, 3113306241, 41454, 19396, 146, 165, 70, 105, 104, 227, 4, 54);
RT_INTERFACE!{interface IValueAndLanguage(IValueAndLanguageVtbl, IValueAndLanguage_Abi): IInspectable(IInspectableVtbl) [IID_IValueAndLanguage] {
    fn get_Language(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Language(&self, value: HSTRING) -> HRESULT,
    fn get_Value(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn put_Value(&self, value: <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IValueAndLanguage {
    #[inline] pub fn get_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Language)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_language(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Language)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Value)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class ValueAndLanguage: IValueAndLanguage}
impl RtActivatable<IActivationFactory> for ValueAndLanguage {}
DEFINE_CLSID!(ValueAndLanguage(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,101,97,114,99,104,46,86,97,108,117,101,65,110,100,76,97,110,103,117,97,103,101,0]) [CLSID_ValueAndLanguage]);
} // Windows.Storage.Search
pub mod streams { // Windows.Storage.Streams
use crate::prelude::*;
DEFINE_IID!(IID_IBuffer, 2421821408, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IBuffer(IBufferVtbl, IBuffer_Abi): IInspectable(IInspectableVtbl) [IID_IBuffer] {
    fn get_Capacity(&self, out: *mut u32) -> HRESULT,
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn put_Length(&self, value: u32) -> HRESULT
}}
impl IBuffer {
    #[inline] pub fn get_capacity(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Capacity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Length)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_length(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Length)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Buffer: IBuffer}
impl RtActivatable<IBufferFactory> for Buffer {}
impl RtActivatable<IBufferStatics> for Buffer {}
impl Buffer {
    #[inline] pub fn create(capacity: u32) -> Result<Buffer> {
        <Self as RtActivatable<IBufferFactory>>::get_activation_factory().create(capacity)
    }
    #[inline] pub fn create_copy_from_memory_buffer(input: &foundation::IMemoryBuffer) -> Result<Option<Buffer>> {
        <Self as RtActivatable<IBufferStatics>>::get_activation_factory().create_copy_from_memory_buffer(input)
    }
    #[inline] pub fn create_memory_buffer_over_ibuffer(input: &IBuffer) -> Result<Option<foundation::MemoryBuffer>> {
        <Self as RtActivatable<IBufferStatics>>::get_activation_factory().create_memory_buffer_over_ibuffer(input)
    }
}
DEFINE_CLSID!(Buffer(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,66,117,102,102,101,114,0]) [CLSID_Buffer]);
DEFINE_IID!(IID_IBufferFactory, 1907331405, 49423, 18507, 188, 80, 20, 188, 98, 59, 58, 39);
RT_INTERFACE!{static interface IBufferFactory(IBufferFactoryVtbl, IBufferFactory_Abi): IInspectable(IInspectableVtbl) [IID_IBufferFactory] {
    fn Create(&self, capacity: u32, out: *mut <Buffer as RtType>::Abi) -> HRESULT
}}
impl IBufferFactory {
    #[inline] pub fn create(&self, capacity: u32) -> Result<Buffer> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, capacity, &mut out);
        if hr == S_OK { Ok(Buffer::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBufferStatics, 3909215835, 55062, 18266, 169, 10, 175, 114, 41, 177, 231, 65);
RT_INTERFACE!{static interface IBufferStatics(IBufferStaticsVtbl, IBufferStatics_Abi): IInspectable(IInspectableVtbl) [IID_IBufferStatics] {
    fn CreateCopyFromMemoryBuffer(&self, input: <foundation::IMemoryBuffer as RtType>::Abi, out: *mut <Buffer as RtType>::Abi) -> HRESULT,
    fn CreateMemoryBufferOverIBuffer(&self, input: <IBuffer as RtType>::Abi, out: *mut <foundation::MemoryBuffer as RtType>::Abi) -> HRESULT
}}
impl IBufferStatics {
    #[inline] pub fn create_copy_from_memory_buffer(&self, input: &foundation::IMemoryBuffer) -> Result<Option<Buffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCopyFromMemoryBuffer)(self.get_abi() as *const _ as *mut _, input.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Buffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_memory_buffer_over_ibuffer(&self, input: &IBuffer) -> Result<Option<foundation::MemoryBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateMemoryBufferOverIBuffer)(self.get_abi() as *const _ as *mut _, input.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::MemoryBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ByteOrder: i32 {
    LittleEndian = 0, BigEndian = 1,
}}
DEFINE_IID!(IID_IContentTypeProvider, 2547030181, 15257, 19945, 136, 165, 225, 29, 47, 80, 199, 149);
RT_INTERFACE!{interface IContentTypeProvider(IContentTypeProviderVtbl, IContentTypeProvider_Abi): IInspectable(IInspectableVtbl) [IID_IContentTypeProvider] {
    fn get_ContentType(&self, out: *mut HSTRING) -> HRESULT
}}
impl IContentTypeProvider {
    #[inline] pub fn get_content_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ContentType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDataReader, 3803512873, 46273, 17172, 164, 184, 251, 129, 58, 47, 39, 94);
RT_INTERFACE!{interface IDataReader(IDataReaderVtbl, IDataReader_Abi): IInspectable(IInspectableVtbl) [IID_IDataReader] {
    fn get_UnconsumedBufferLength(&self, out: *mut u32) -> HRESULT,
    fn get_UnicodeEncoding(&self, out: *mut UnicodeEncoding) -> HRESULT,
    fn put_UnicodeEncoding(&self, value: UnicodeEncoding) -> HRESULT,
    fn get_ByteOrder(&self, out: *mut ByteOrder) -> HRESULT,
    fn put_ByteOrder(&self, value: ByteOrder) -> HRESULT,
    fn get_InputStreamOptions(&self, out: *mut InputStreamOptions) -> HRESULT,
    fn put_InputStreamOptions(&self, value: InputStreamOptions) -> HRESULT,
    fn ReadByte(&self, out: *mut u8) -> HRESULT,
    fn ReadBytes(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn ReadBuffer(&self, length: u32, out: *mut <IBuffer as RtType>::Abi) -> HRESULT,
    fn ReadBoolean(&self, out: *mut bool) -> HRESULT,
    fn ReadGuid(&self, out: *mut Guid) -> HRESULT,
    fn ReadInt16(&self, out: *mut i16) -> HRESULT,
    fn ReadInt32(&self, out: *mut i32) -> HRESULT,
    fn ReadInt64(&self, out: *mut i64) -> HRESULT,
    fn ReadUInt16(&self, out: *mut u16) -> HRESULT,
    fn ReadUInt32(&self, out: *mut u32) -> HRESULT,
    fn ReadUInt64(&self, out: *mut u64) -> HRESULT,
    fn ReadSingle(&self, out: *mut f32) -> HRESULT,
    fn ReadDouble(&self, out: *mut f64) -> HRESULT,
    fn ReadString(&self, codeUnitCount: u32, out: *mut HSTRING) -> HRESULT,
    fn ReadDateTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn ReadTimeSpan(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn LoadAsync(&self, count: u32, out: *mut <DataReaderLoadOperation as RtType>::Abi) -> HRESULT,
    fn DetachBuffer(&self, out: *mut <IBuffer as RtType>::Abi) -> HRESULT,
    fn DetachStream(&self, out: *mut <IInputStream as RtType>::Abi) -> HRESULT
}}
impl IDataReader {
    #[inline] pub fn get_unconsumed_buffer_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UnconsumedBufferLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_unicode_encoding(&self) -> Result<UnicodeEncoding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UnicodeEncoding)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_unicode_encoding(&self, value: UnicodeEncoding) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_UnicodeEncoding)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_byte_order(&self) -> Result<ByteOrder> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ByteOrder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_byte_order(&self, value: ByteOrder) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ByteOrder)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_input_stream_options(&self) -> Result<InputStreamOptions> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_InputStreamOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_input_stream_options(&self, value: InputStreamOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_InputStreamOptions)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn read_byte(&self) -> Result<u8> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadByte)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_bytes(&self, value: &mut [u8]) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).ReadBytes)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_mut_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn read_buffer(&self, length: u32) -> Result<Option<IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadBuffer)(self.get_abi() as *const _ as *mut _, length, &mut out);
        if hr == S_OK { Ok(IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_boolean(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadBoolean)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_guid(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadGuid)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_int16(&self) -> Result<i16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadInt16)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_int32(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadInt32)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_int64(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadInt64)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_uint16(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadUInt16)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_uint32(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadUInt32)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_uint64(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadUInt64)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_single(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadSingle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_double(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadDouble)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_string(&self, codeUnitCount: u32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadString)(self.get_abi() as *const _ as *mut _, codeUnitCount, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn read_date_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadDateTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn read_time_span(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).ReadTimeSpan)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn load_async(&self, count: u32) -> Result<DataReaderLoadOperation> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadAsync)(self.get_abi() as *const _ as *mut _, count, &mut out);
        if hr == S_OK { Ok(DataReaderLoadOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn detach_buffer(&self) -> Result<Option<IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachBuffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn detach_stream(&self) -> Result<Option<IInputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataReader: IDataReader}
impl RtActivatable<IDataReaderFactory> for DataReader {}
impl RtActivatable<IDataReaderStatics> for DataReader {}
impl DataReader {
    #[inline] pub fn create_data_reader(inputStream: &IInputStream) -> Result<DataReader> {
        <Self as RtActivatable<IDataReaderFactory>>::get_activation_factory().create_data_reader(inputStream)
    }
    #[inline] pub fn from_buffer(buffer: &IBuffer) -> Result<Option<DataReader>> {
        <Self as RtActivatable<IDataReaderStatics>>::get_activation_factory().from_buffer(buffer)
    }
}
DEFINE_CLSID!(DataReader(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,68,97,116,97,82,101,97,100,101,114,0]) [CLSID_DataReader]);
DEFINE_IID!(IID_IDataReaderFactory, 3612506183, 22490, 19989, 145, 76, 6, 128, 102, 153, 160, 152);
RT_INTERFACE!{static interface IDataReaderFactory(IDataReaderFactoryVtbl, IDataReaderFactory_Abi): IInspectable(IInspectableVtbl) [IID_IDataReaderFactory] {
    fn CreateDataReader(&self, inputStream: <IInputStream as RtType>::Abi, out: *mut <DataReader as RtType>::Abi) -> HRESULT
}}
impl IDataReaderFactory {
    #[inline] pub fn create_data_reader(&self, inputStream: &IInputStream) -> Result<DataReader> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateDataReader)(self.get_abi() as *const _ as *mut _, inputStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DataReader::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataReaderLoadOperation: foundation::IAsyncOperation<u32>}
DEFINE_IID!(IID_IDataReaderStatics, 301776840, 63802, 18203, 177, 33, 243, 121, 227, 73, 49, 60);
RT_INTERFACE!{static interface IDataReaderStatics(IDataReaderStaticsVtbl, IDataReaderStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDataReaderStatics] {
    fn FromBuffer(&self, buffer: <IBuffer as RtType>::Abi, out: *mut <DataReader as RtType>::Abi) -> HRESULT
}}
impl IDataReaderStatics {
    #[inline] pub fn from_buffer(&self, buffer: &IBuffer) -> Result<Option<DataReader>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FromBuffer)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DataReader::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDataWriter, 1689817701, 54081, 18722, 179, 138, 221, 74, 248, 128, 140, 78);
RT_INTERFACE!{interface IDataWriter(IDataWriterVtbl, IDataWriter_Abi): IInspectable(IInspectableVtbl) [IID_IDataWriter] {
    fn get_UnstoredBufferLength(&self, out: *mut u32) -> HRESULT,
    fn get_UnicodeEncoding(&self, out: *mut UnicodeEncoding) -> HRESULT,
    fn put_UnicodeEncoding(&self, value: UnicodeEncoding) -> HRESULT,
    fn get_ByteOrder(&self, out: *mut ByteOrder) -> HRESULT,
    fn put_ByteOrder(&self, value: ByteOrder) -> HRESULT,
    fn WriteByte(&self, value: u8) -> HRESULT,
    fn WriteBytes(&self, valueSize: u32, value: *mut u8) -> HRESULT,
    fn WriteBuffer(&self, buffer: <IBuffer as RtType>::Abi) -> HRESULT,
    fn WriteBufferRange(&self, buffer: <IBuffer as RtType>::Abi, start: u32, count: u32) -> HRESULT,
    fn WriteBoolean(&self, value: bool) -> HRESULT,
    fn WriteGuid(&self, value: Guid) -> HRESULT,
    fn WriteInt16(&self, value: i16) -> HRESULT,
    fn WriteInt32(&self, value: i32) -> HRESULT,
    fn WriteInt64(&self, value: i64) -> HRESULT,
    fn WriteUInt16(&self, value: u16) -> HRESULT,
    fn WriteUInt32(&self, value: u32) -> HRESULT,
    fn WriteUInt64(&self, value: u64) -> HRESULT,
    fn WriteSingle(&self, value: f32) -> HRESULT,
    fn WriteDouble(&self, value: f64) -> HRESULT,
    fn WriteDateTime(&self, value: foundation::DateTime) -> HRESULT,
    fn WriteTimeSpan(&self, value: foundation::TimeSpan) -> HRESULT,
    fn WriteString(&self, value: HSTRING, out: *mut u32) -> HRESULT,
    fn MeasureString(&self, value: HSTRING, out: *mut u32) -> HRESULT,
    fn StoreAsync(&self, out: *mut <DataWriterStoreOperation as RtType>::Abi) -> HRESULT,
    fn FlushAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn DetachBuffer(&self, out: *mut <IBuffer as RtType>::Abi) -> HRESULT,
    fn DetachStream(&self, out: *mut <IOutputStream as RtType>::Abi) -> HRESULT
}}
impl IDataWriter {
    #[inline] pub fn get_unstored_buffer_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UnstoredBufferLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_unicode_encoding(&self) -> Result<UnicodeEncoding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UnicodeEncoding)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_unicode_encoding(&self, value: UnicodeEncoding) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_UnicodeEncoding)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_byte_order(&self) -> Result<ByteOrder> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ByteOrder)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_byte_order(&self, value: ByteOrder) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ByteOrder)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_byte(&self, value: u8) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteByte)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_bytes(&self, value: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteBytes)(self.get_abi() as *const _ as *mut _, value.len() as u32, value.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_buffer(&self, buffer: &IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteBuffer)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_buffer_range(&self, buffer: &IBuffer, start: u32, count: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteBufferRange)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, start, count);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_boolean(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteBoolean)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_guid(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteGuid)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_int16(&self, value: i16) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteInt16)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_int32(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteInt32)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_int64(&self, value: i64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteInt64)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_uint16(&self, value: u16) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteUInt16)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_uint32(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteUInt32)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_uint64(&self, value: u64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteUInt64)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_single(&self, value: f32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteSingle)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_double(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteDouble)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_date_time(&self, value: foundation::DateTime) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteDateTime)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_time_span(&self, value: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WriteTimeSpan)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn write_string(&self, value: &HStringArg) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).WriteString)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn measure_string(&self, value: &HStringArg) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).MeasureString)(self.get_abi() as *const _ as *mut _, value.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn store_async(&self) -> Result<DataWriterStoreOperation> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).StoreAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DataWriterStoreOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn flush_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FlushAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn detach_buffer(&self) -> Result<Option<IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachBuffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn detach_stream(&self) -> Result<Option<IOutputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IOutputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataWriter: IDataWriter}
impl RtActivatable<IDataWriterFactory> for DataWriter {}
impl RtActivatable<IActivationFactory> for DataWriter {}
impl DataWriter {
    #[inline] pub fn create_data_writer(outputStream: &IOutputStream) -> Result<DataWriter> {
        <Self as RtActivatable<IDataWriterFactory>>::get_activation_factory().create_data_writer(outputStream)
    }
}
DEFINE_CLSID!(DataWriter(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,68,97,116,97,87,114,105,116,101,114,0]) [CLSID_DataWriter]);
DEFINE_IID!(IID_IDataWriterFactory, 864839618, 35716, 19499, 156, 80, 123, 135, 103, 132, 122, 31);
RT_INTERFACE!{static interface IDataWriterFactory(IDataWriterFactoryVtbl, IDataWriterFactory_Abi): IInspectable(IInspectableVtbl) [IID_IDataWriterFactory] {
    fn CreateDataWriter(&self, outputStream: <IOutputStream as RtType>::Abi, out: *mut <DataWriter as RtType>::Abi) -> HRESULT
}}
impl IDataWriterFactory {
    #[inline] pub fn create_data_writer(&self, outputStream: &IOutputStream) -> Result<DataWriter> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateDataWriter)(self.get_abi() as *const _ as *mut _, outputStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DataWriter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DataWriterStoreOperation: foundation::IAsyncOperation<u32>}
RT_CLASS!{class FileInputStream: IInputStream}
RT_ENUM! { enum FileOpenDisposition: i32 {
    OpenExisting = 0, OpenAlways = 1, CreateNew = 2, CreateAlways = 3, TruncateExisting = 4,
}}
RT_CLASS!{class FileOutputStream: IOutputStream}
RT_CLASS!{class FileRandomAccessStream: IRandomAccessStream}
impl RtActivatable<IFileRandomAccessStreamStatics> for FileRandomAccessStream {}
impl FileRandomAccessStream {
    #[inline] pub fn open_async(filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_async(filePath, accessMode)
    }
    #[inline] pub fn open_with_options_async(filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_with_options_async(filePath, accessMode, sharingOptions, openDisposition)
    }
    #[inline] pub fn open_transacted_write_async(filePath: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_async(filePath)
    }
    #[inline] pub fn open_transacted_write_with_options_async(filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_with_options_async(filePath, openOptions, openDisposition)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_async(user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_for_user_async(user, filePath, accessMode)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_with_options_async(user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_for_user_with_options_async(user, filePath, accessMode, sharingOptions, openDisposition)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_async(user: &super::super::system::User, filePath: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_for_user_async(user, filePath)
    }
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_with_options_async(user: &super::super::system::User, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        <Self as RtActivatable<IFileRandomAccessStreamStatics>>::get_activation_factory().open_transacted_write_for_user_with_options_async(user, filePath, openOptions, openDisposition)
    }
}
DEFINE_CLSID!(FileRandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,70,105,108,101,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_FileRandomAccessStream]);
DEFINE_IID!(IID_IFileRandomAccessStreamStatics, 1934950663, 15191, 19293, 131, 69, 85, 77, 47, 198, 33, 240);
RT_INTERFACE!{static interface IFileRandomAccessStreamStatics(IFileRandomAccessStreamStaticsVtbl, IFileRandomAccessStreamStatics_Abi): IInspectable(IInspectableVtbl) [IID_IFileRandomAccessStreamStatics] {
    fn OpenAsync(&self, filePath: HSTRING, accessMode: super::FileAccessMode, out: *mut <foundation::IAsyncOperation<IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn OpenWithOptionsAsync(&self, filePath: HSTRING, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut <foundation::IAsyncOperation<IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn OpenTransactedWriteAsync(&self, filePath: HSTRING, out: *mut <foundation::IAsyncOperation<super::StorageStreamTransaction> as RtType>::Abi) -> HRESULT,
    fn OpenTransactedWriteWithOptionsAsync(&self, filePath: HSTRING, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut <foundation::IAsyncOperation<super::StorageStreamTransaction> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenForUserAsync(&self, user: <super::super::system::User as RtType>::Abi, filePath: HSTRING, accessMode: super::FileAccessMode, out: *mut <foundation::IAsyncOperation<IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenForUserWithOptionsAsync(&self, user: <super::super::system::User as RtType>::Abi, filePath: HSTRING, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut <foundation::IAsyncOperation<IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenTransactedWriteForUserAsync(&self, user: <super::super::system::User as RtType>::Abi, filePath: HSTRING, out: *mut <foundation::IAsyncOperation<super::StorageStreamTransaction> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn OpenTransactedWriteForUserWithOptionsAsync(&self, user: <super::super::system::User as RtType>::Abi, filePath: HSTRING, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition, out: *mut <foundation::IAsyncOperation<super::StorageStreamTransaction> as RtType>::Abi) -> HRESULT
}}
impl IFileRandomAccessStreamStatics {
    #[inline] pub fn open_async(&self, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenAsync)(self.get_abi() as *const _ as *mut _, filePath.get(), accessMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_with_options_async(&self, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenWithOptionsAsync)(self.get_abi() as *const _ as *mut _, filePath.get(), accessMode, sharingOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_transacted_write_async(&self, filePath: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteAsync)(self.get_abi() as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn open_transacted_write_with_options_async(&self, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteWithOptionsAsync)(self.get_abi() as *const _ as *mut _, filePath.get(), openOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_async(&self, user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, filePath.get(), accessMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_for_user_with_options_async(&self, user: &super::super::system::User, filePath: &HStringArg, accessMode: super::FileAccessMode, sharingOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenForUserWithOptionsAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, filePath.get(), accessMode, sharingOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_async(&self, user: &super::super::system::User, filePath: &HStringArg) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteForUserAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn open_transacted_write_for_user_with_options_async(&self, user: &super::super::system::User, filePath: &HStringArg, openOptions: super::StorageOpenOptions, openDisposition: FileOpenDisposition) -> Result<foundation::IAsyncOperation<super::StorageStreamTransaction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenTransactedWriteForUserWithOptionsAsync)(self.get_abi() as *const _ as *mut _, user.get_abi() as *const _ as *mut _, filePath.get(), openOptions, openDisposition, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class InMemoryRandomAccessStream: IRandomAccessStream}
impl RtActivatable<IActivationFactory> for InMemoryRandomAccessStream {}
DEFINE_CLSID!(InMemoryRandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,73,110,77,101,109,111,114,121,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_InMemoryRandomAccessStream]);
DEFINE_IID!(IID_IInputStream, 2421821410, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IInputStream(IInputStreamVtbl, IInputStream_Abi): IInspectable(IInspectableVtbl) [IID_IInputStream] {
    fn ReadAsync(&self, buffer: <IBuffer as RtType>::Abi, count: u32, options: InputStreamOptions, out: *mut <foundation::IAsyncOperationWithProgress<IBuffer, u32> as RtType>::Abi) -> HRESULT
}}
impl IInputStream {
    #[inline] pub fn read_async(&self, buffer: &IBuffer, count: u32, options: InputStreamOptions) -> Result<foundation::IAsyncOperationWithProgress<IBuffer, u32>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadAsync)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, count, options, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum InputStreamOptions: u32 {
    None = 0, Partial = 1, ReadAhead = 2,
}}
RT_CLASS!{class InputStreamOverStream: IInputStream}
DEFINE_IID!(IID_IInputStreamReference, 1133681944, 24265, 19290, 145, 156, 66, 5, 176, 200, 4, 182);
RT_INTERFACE!{interface IInputStreamReference(IInputStreamReferenceVtbl, IInputStreamReference_Abi): IInspectable(IInspectableVtbl) [IID_IInputStreamReference] {
    fn OpenSequentialReadAsync(&self, out: *mut <foundation::IAsyncOperation<IInputStream> as RtType>::Abi) -> HRESULT
}}
impl IInputStreamReference {
    #[inline] pub fn open_sequential_read_async(&self) -> Result<foundation::IAsyncOperation<IInputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenSequentialReadAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IOutputStream, 2421821414, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IOutputStream(IOutputStreamVtbl, IOutputStream_Abi): IInspectable(IInspectableVtbl) [IID_IOutputStream] {
    fn WriteAsync(&self, buffer: <IBuffer as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<u32, u32> as RtType>::Abi) -> HRESULT,
    fn FlushAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IOutputStream {
    #[inline] pub fn write_async(&self, buffer: &IBuffer) -> Result<foundation::IAsyncOperationWithProgress<u32, u32>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).WriteAsync)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn flush_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FlushAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class OutputStreamOverStream: IOutputStream}
DEFINE_IID!(IID_IRandomAccessStream, 2421821409, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
RT_INTERFACE!{interface IRandomAccessStream(IRandomAccessStreamVtbl, IRandomAccessStream_Abi): IInspectable(IInspectableVtbl) [IID_IRandomAccessStream] {
    fn get_Size(&self, out: *mut u64) -> HRESULT,
    fn put_Size(&self, value: u64) -> HRESULT,
    fn GetInputStreamAt(&self, position: u64, out: *mut <IInputStream as RtType>::Abi) -> HRESULT,
    fn GetOutputStreamAt(&self, position: u64, out: *mut <IOutputStream as RtType>::Abi) -> HRESULT,
    fn get_Position(&self, out: *mut u64) -> HRESULT,
    fn Seek(&self, position: u64) -> HRESULT,
    fn CloneStream(&self, out: *mut <IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn get_CanRead(&self, out: *mut bool) -> HRESULT,
    fn get_CanWrite(&self, out: *mut bool) -> HRESULT
}}
impl IRandomAccessStream {
    #[inline] pub fn get_size(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Size)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_size(&self, value: u64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Size)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_input_stream_at(&self, position: u64) -> Result<Option<IInputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetInputStreamAt)(self.get_abi() as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(IInputStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_stream_at(&self, position: u64) -> Result<Option<IOutputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetOutputStreamAt)(self.get_abi() as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(IOutputStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_position(&self) -> Result<u64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Position)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn seek(&self, position: u64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Seek)(self.get_abi() as *const _ as *mut _, position);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clone_stream(&self) -> Result<Option<IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CloneStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_can_read(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CanRead)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_can_write(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CanWrite)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class RandomAccessStream}
impl RtActivatable<IRandomAccessStreamStatics> for RandomAccessStream {}
impl RandomAccessStream {
    #[inline] pub fn copy_async(source: &IInputStream, destination: &IOutputStream) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_async(source, destination)
    }
    #[inline] pub fn copy_size_async(source: &IInputStream, destination: &IOutputStream, bytesToCopy: u64) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_size_async(source, destination, bytesToCopy)
    }
    #[inline] pub fn copy_and_close_async(source: &IInputStream, destination: &IOutputStream) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> {
        <Self as RtActivatable<IRandomAccessStreamStatics>>::get_activation_factory().copy_and_close_async(source, destination)
    }
}
DEFINE_CLSID!(RandomAccessStream(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,0]) [CLSID_RandomAccessStream]);
RT_CLASS!{class RandomAccessStreamOverStream: IRandomAccessStream}
DEFINE_IID!(IID_IRandomAccessStreamReference, 871248180, 7638, 20026, 128, 103, 209, 193, 98, 232, 100, 43);
RT_INTERFACE!{interface IRandomAccessStreamReference(IRandomAccessStreamReferenceVtbl, IRandomAccessStreamReference_Abi): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamReference] {
    fn OpenReadAsync(&self, out: *mut <foundation::IAsyncOperation<IRandomAccessStreamWithContentType> as RtType>::Abi) -> HRESULT
}}
impl IRandomAccessStreamReference {
    #[inline] pub fn open_read_async(&self) -> Result<foundation::IAsyncOperation<IRandomAccessStreamWithContentType>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).OpenReadAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class RandomAccessStreamReference: IRandomAccessStreamReference}
impl RtActivatable<IRandomAccessStreamReferenceStatics> for RandomAccessStreamReference {}
impl RandomAccessStreamReference {
    #[inline] pub fn create_from_file(file: &super::IStorageFile) -> Result<Option<RandomAccessStreamReference>> {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_file(file)
    }
    #[inline] pub fn create_from_uri(uri: &foundation::Uri) -> Result<Option<RandomAccessStreamReference>> {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_uri(uri)
    }
    #[inline] pub fn create_from_stream(stream: &IRandomAccessStream) -> Result<Option<RandomAccessStreamReference>> {
        <Self as RtActivatable<IRandomAccessStreamReferenceStatics>>::get_activation_factory().create_from_stream(stream)
    }
}
DEFINE_CLSID!(RandomAccessStreamReference(&[87,105,110,100,111,119,115,46,83,116,111,114,97,103,101,46,83,116,114,101,97,109,115,46,82,97,110,100,111,109,65,99,99,101,115,115,83,116,114,101,97,109,82,101,102,101,114,101,110,99,101,0]) [CLSID_RandomAccessStreamReference]);
DEFINE_IID!(IID_IRandomAccessStreamReferenceStatics, 2238908892, 16319, 20093, 152, 111, 239, 59, 26, 7, 169, 100);
RT_INTERFACE!{static interface IRandomAccessStreamReferenceStatics(IRandomAccessStreamReferenceStaticsVtbl, IRandomAccessStreamReferenceStatics_Abi): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamReferenceStatics] {
    fn CreateFromFile(&self, file: <super::IStorageFile as RtType>::Abi, out: *mut <RandomAccessStreamReference as RtType>::Abi) -> HRESULT,
    fn CreateFromUri(&self, uri: <foundation::Uri as RtType>::Abi, out: *mut <RandomAccessStreamReference as RtType>::Abi) -> HRESULT,
    fn CreateFromStream(&self, stream: <IRandomAccessStream as RtType>::Abi, out: *mut <RandomAccessStreamReference as RtType>::Abi) -> HRESULT
}}
impl IRandomAccessStreamReferenceStatics {
    #[inline] pub fn create_from_file(&self, file: &super::IStorageFile) -> Result<Option<RandomAccessStreamReference>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromFile)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(RandomAccessStreamReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_uri(&self, uri: &foundation::Uri) -> Result<Option<RandomAccessStreamReference>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromUri)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(RandomAccessStreamReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_stream(&self, stream: &IRandomAccessStream) -> Result<Option<RandomAccessStreamReference>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromStream)(self.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(RandomAccessStreamReference::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRandomAccessStreamStatics, 1380773327, 28201, 19685, 149, 115, 107, 117, 61, 182, 108, 58);
RT_INTERFACE!{static interface IRandomAccessStreamStatics(IRandomAccessStreamStaticsVtbl, IRandomAccessStreamStatics_Abi): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamStatics] {
    fn CopyAsync(&self, source: <IInputStream as RtType>::Abi, destination: <IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<u64, u64> as RtType>::Abi) -> HRESULT,
    fn CopySizeAsync(&self, source: <IInputStream as RtType>::Abi, destination: <IOutputStream as RtType>::Abi, bytesToCopy: u64, out: *mut <foundation::IAsyncOperationWithProgress<u64, u64> as RtType>::Abi) -> HRESULT,
    fn CopyAndCloseAsync(&self, source: <IInputStream as RtType>::Abi, destination: <IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<u64, u64> as RtType>::Abi) -> HRESULT
}}
impl IRandomAccessStreamStatics {
    #[inline] pub fn copy_async(&self, source: &IInputStream, destination: &IOutputStream) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyAsync)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, destination.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_size_async(&self, source: &IInputStream, destination: &IOutputStream, bytesToCopy: u64) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopySizeAsync)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, destination.get_abi() as *const _ as *mut _, bytesToCopy, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_and_close_async(&self, source: &IInputStream, destination: &IOutputStream) -> Result<foundation::IAsyncOperationWithProgress<u64, u64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CopyAndCloseAsync)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, destination.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IRandomAccessStreamWithContentType, 3424995367, 19261, 17295, 146, 50, 16, 199, 107, 199, 224, 56);
RT_INTERFACE!{interface IRandomAccessStreamWithContentType(IRandomAccessStreamWithContentTypeVtbl, IRandomAccessStreamWithContentType_Abi): IInspectable(IInspectableVtbl) [IID_IRandomAccessStreamWithContentType] {
    
}}
RT_ENUM! { enum UnicodeEncoding: i32 {
    Utf8 = 0, Utf16LE = 1, Utf16BE = 2,
}}
} // Windows.Storage.Streams
