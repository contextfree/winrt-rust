use crate::prelude::*;
DEFINE_IID!(IID_IPerceptionTimestamp, 2277656580, 41518, 19163, 186, 38, 215, 142, 246, 57, 188, 244);
RT_INTERFACE!{interface IPerceptionTimestamp(IPerceptionTimestampVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestamp] {
    fn get_TargetTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_PredictionAmount(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl IPerceptionTimestamp {
    #[inline] pub fn get_target_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TargetTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_prediction_amount(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_PredictionAmount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PerceptionTimestamp: IPerceptionTimestamp}
DEFINE_IID!(IID_IPerceptionTimestamp2, 3813980141, 11217, 16823, 158, 208, 116, 161, 92, 53, 69, 55);
RT_INTERFACE!{interface IPerceptionTimestamp2(IPerceptionTimestamp2Vtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestamp2] {
    fn get_SystemRelativeTargetTime(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl IPerceptionTimestamp2 {
    #[inline] pub fn get_system_relative_target_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_SystemRelativeTargetTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class PerceptionTimestampHelper}
impl RtActivatable<IPerceptionTimestampHelperStatics> for PerceptionTimestampHelper {}
impl RtActivatable<IPerceptionTimestampHelperStatics2> for PerceptionTimestampHelper {}
impl PerceptionTimestampHelper {
    #[inline] pub fn from_historical_target_time(targetTime: foundation::DateTime) -> Result<Option<PerceptionTimestamp>> {
        <Self as RtActivatable<IPerceptionTimestampHelperStatics>>::get_activation_factory().from_historical_target_time(targetTime)
    }
    #[inline] pub fn from_system_relative_target_time(targetTime: foundation::TimeSpan) -> Result<Option<PerceptionTimestamp>> {
        <Self as RtActivatable<IPerceptionTimestampHelperStatics2>>::get_activation_factory().from_system_relative_target_time(targetTime)
    }
}
DEFINE_CLSID!(PerceptionTimestampHelper(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,80,101,114,99,101,112,116,105,111,110,84,105,109,101,115,116,97,109,112,72,101,108,112,101,114,0]) [CLSID_PerceptionTimestampHelper]);
DEFINE_IID!(IID_IPerceptionTimestampHelperStatics, 1202065876, 43487, 20188, 133, 93, 244, 211, 57, 217, 103, 172);
RT_INTERFACE!{static interface IPerceptionTimestampHelperStatics(IPerceptionTimestampHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestampHelperStatics] {
    fn FromHistoricalTargetTime(&self, targetTime: foundation::DateTime, out: *mut <PerceptionTimestamp as RtType>::Abi) -> HRESULT
}}
impl IPerceptionTimestampHelperStatics {
    #[inline] pub fn from_historical_target_time(&self, targetTime: foundation::DateTime) -> Result<Option<PerceptionTimestamp>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromHistoricalTargetTime)(self.get_abi() as *const _ as *mut _, targetTime, &mut out);
        if hr == S_OK { Ok(PerceptionTimestamp::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPerceptionTimestampHelperStatics2, 1943119870, 16313, 17777, 135, 212, 60, 146, 10, 94, 134, 235);
RT_INTERFACE!{static interface IPerceptionTimestampHelperStatics2(IPerceptionTimestampHelperStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestampHelperStatics2] {
    fn FromSystemRelativeTargetTime(&self, targetTime: foundation::TimeSpan, out: *mut <PerceptionTimestamp as RtType>::Abi) -> HRESULT
}}
impl IPerceptionTimestampHelperStatics2 {
    #[inline] pub fn from_system_relative_target_time(&self, targetTime: foundation::TimeSpan) -> Result<Option<PerceptionTimestamp>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromSystemRelativeTargetTime)(self.get_abi() as *const _ as *mut _, targetTime, &mut out);
        if hr == S_OK { Ok(PerceptionTimestamp::wrap(out)) } else { err(hr) }
    }}
}
pub mod automation { // Windows.Perception.Automation
pub mod core { // Windows.Perception.Automation.Core
use crate::prelude::*;
RT_CLASS!{static class CorePerceptionAutomation}
impl RtActivatable<ICorePerceptionAutomationStatics> for CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[inline] pub fn set_activation_factory_provider(provider: &foundation::IGetActivationFactory) -> Result<()> {
        <Self as RtActivatable<ICorePerceptionAutomationStatics>>::get_activation_factory().set_activation_factory_provider(provider)
    }
}
DEFINE_CLSID!(CorePerceptionAutomation(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,65,117,116,111,109,97,116,105,111,110,46,67,111,114,101,46,67,111,114,101,80,101,114,99,101,112,116,105,111,110,65,117,116,111,109,97,116,105,111,110,0]) [CLSID_CorePerceptionAutomation]);
DEFINE_IID!(IID_ICorePerceptionAutomationStatics, 196101441, 19682, 18723, 154, 118, 129, 135, 236, 197, 145, 18);
RT_INTERFACE!{static interface ICorePerceptionAutomationStatics(ICorePerceptionAutomationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICorePerceptionAutomationStatics] {
    fn SetActivationFactoryProvider(&self, provider: <foundation::IGetActivationFactory as RtType>::Abi) -> HRESULT
}}
impl ICorePerceptionAutomationStatics {
    #[inline] pub fn set_activation_factory_provider(&self, provider: &foundation::IGetActivationFactory) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetActivationFactoryProvider)(self.get_abi() as *const _ as *mut _, provider.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
} // Windows.Perception.Automation.Core
} // Windows.Perception.Automation
pub mod people { // Windows.Perception.People
use crate::prelude::*;
DEFINE_IID!(IID_IHeadPose, 2136655269, 18907, 14239, 148, 41, 50, 162, 250, 243, 79, 166);
RT_INTERFACE!{interface IHeadPose(IHeadPoseVtbl): IInspectable(IInspectableVtbl) [IID_IHeadPose] {
    fn get_Position(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_ForwardDirection(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_UpDirection(&self, out: *mut foundation::numerics::Vector3) -> HRESULT
}}
impl IHeadPose {
    #[inline] pub fn get_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Position)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_forward_direction(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ForwardDirection)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_up_direction(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_UpDirection)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HeadPose: IHeadPose}
} // Windows.Perception.People
pub mod spatial { // Windows.Perception.Spatial
use crate::prelude::*;
DEFINE_IID!(IID_ISpatialAnchor, 86631886, 7476, 14082, 188, 236, 234, 191, 245, 120, 168, 105);
RT_INTERFACE!{interface ISpatialAnchor(ISpatialAnchorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor] {
    fn get_CoordinateSystem(&self, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn get_RawCoordinateSystem(&self, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn add_RawCoordinateSystemAdjusted(&self, handler: <foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawCoordinateSystemAdjusted(&self, cookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl ISpatialAnchor {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CoordinateSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_coordinate_system(&self) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_RawCoordinateSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_raw_coordinate_system_adjusted(&self, handler: &foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_RawCoordinateSystemAdjusted)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_raw_coordinate_system_adjusted(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_RawCoordinateSystemAdjusted)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchor: ISpatialAnchor}
impl RtActivatable<ISpatialAnchorStatics> for SpatialAnchor {}
impl SpatialAnchor {
    #[inline] pub fn try_create_relative_to(coordinateSystem: &SpatialCoordinateSystem) -> Result<Option<SpatialAnchor>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_relative_to(coordinateSystem)
    }
    #[inline] pub fn try_create_with_position_relative_to(coordinateSystem: &SpatialCoordinateSystem, position: foundation::numerics::Vector3) -> Result<Option<SpatialAnchor>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_relative_to(coordinateSystem, position)
    }
    #[inline] pub fn try_create_with_position_and_orientation_relative_to(coordinateSystem: &SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<Option<SpatialAnchor>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_and_orientation_relative_to(coordinateSystem, position, orientation)
    }
}
DEFINE_CLSID!(SpatialAnchor(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,0]) [CLSID_SpatialAnchor]);
DEFINE_IID!(IID_ISpatialAnchor2, 3977758984, 42645, 19702, 146, 253, 151, 38, 59, 167, 16, 71);
RT_INTERFACE!{interface ISpatialAnchor2(ISpatialAnchor2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor2] {
    fn get_RemovedByUser(&self, out: *mut bool) -> HRESULT
}}
impl ISpatialAnchor2 {
    #[inline] pub fn get_removed_by_user(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RemovedByUser)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorExporter, 2586460984, 9467, 17001, 137, 197, 136, 48, 74, 238, 242, 15);
RT_INTERFACE!{interface ISpatialAnchorExporter(ISpatialAnchorExporterVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorExporter] {
    fn GetAnchorExportSufficiencyAsync(&self, anchor: <SpatialAnchor as RtType>::Abi, purpose: SpatialAnchorExportPurpose, out: *mut <foundation::IAsyncOperation<SpatialAnchorExportSufficiency> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn TryExportAnchorAsync(&self, anchor: <SpatialAnchor as RtType>::Abi, purpose: SpatialAnchorExportPurpose, stream: <super::super::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl ISpatialAnchorExporter {
    #[inline] pub fn get_anchor_export_sufficiency_async(&self, anchor: &SpatialAnchor, purpose: SpatialAnchorExportPurpose) -> Result<foundation::IAsyncOperation<SpatialAnchorExportSufficiency>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAnchorExportSufficiencyAsync)(self.get_abi() as *const _ as *mut _, anchor.get_abi() as *const _ as *mut _, purpose, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchor_async(&self, anchor: &SpatialAnchor, purpose: SpatialAnchorExportPurpose, stream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryExportAnchorAsync)(self.get_abi() as *const _ as *mut _, anchor.get_abi() as *const _ as *mut _, purpose, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorExporter: ISpatialAnchorExporter}
impl RtActivatable<ISpatialAnchorExporterStatics> for SpatialAnchorExporter {}
impl SpatialAnchorExporter {
    #[inline] pub fn get_default() -> Result<Option<SpatialAnchorExporter>> {
        <Self as RtActivatable<ISpatialAnchorExporterStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn request_access_async() -> Result<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        <Self as RtActivatable<ISpatialAnchorExporterStatics>>::get_activation_factory().request_access_async()
    }
}
DEFINE_CLSID!(SpatialAnchorExporter(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,69,120,112,111,114,116,101,114,0]) [CLSID_SpatialAnchorExporter]);
DEFINE_IID!(IID_ISpatialAnchorExporterStatics, 3978627000, 9333, 17308, 133, 255, 127, 237, 52, 31, 220, 136);
RT_INTERFACE!{static interface ISpatialAnchorExporterStatics(ISpatialAnchorExporterStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorExporterStatics] {
    fn GetDefault(&self, out: *mut <SpatialAnchorExporter as RtType>::Abi) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut <foundation::IAsyncOperation<SpatialPerceptionAccessStatus> as RtType>::Abi) -> HRESULT
}}
impl ISpatialAnchorExporterStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<SpatialAnchorExporter>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialAnchorExporter::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SpatialAnchorExportPurpose: i32 {
    Relocalization = 0, Sharing = 1,
}}
DEFINE_IID!(IID_ISpatialAnchorExportSufficiency, 2009226027, 13321, 16520, 185, 27, 253, 253, 5, 209, 100, 143);
RT_INTERFACE!{interface ISpatialAnchorExportSufficiency(ISpatialAnchorExportSufficiencyVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorExportSufficiency] {
    fn get_IsMinimallySufficient(&self, out: *mut bool) -> HRESULT,
    fn get_SufficiencyLevel(&self, out: *mut f64) -> HRESULT,
    fn get_RecommendedSufficiencyLevel(&self, out: *mut f64) -> HRESULT
}}
impl ISpatialAnchorExportSufficiency {
    #[inline] pub fn get_is_minimally_sufficient(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsMinimallySufficient)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sufficiency_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_SufficiencyLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recommended_sufficiency_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RecommendedSufficiencyLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorExportSufficiency: ISpatialAnchorExportSufficiency}
RT_CLASS!{static class SpatialAnchorManager}
impl RtActivatable<ISpatialAnchorManagerStatics> for SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[inline] pub fn request_store_async() -> Result<foundation::IAsyncOperation<SpatialAnchorStore>> {
        <Self as RtActivatable<ISpatialAnchorManagerStatics>>::get_activation_factory().request_store_async()
    }
}
DEFINE_CLSID!(SpatialAnchorManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorManager]);
DEFINE_IID!(IID_ISpatialAnchorManagerStatics, 2296581803, 62391, 16907, 176, 134, 138, 128, 192, 125, 145, 13);
RT_INTERFACE!{static interface ISpatialAnchorManagerStatics(ISpatialAnchorManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorManagerStatics] {
    fn RequestStoreAsync(&self, out: *mut <foundation::IAsyncOperation<SpatialAnchorStore> as RtType>::Abi) -> HRESULT
}}
impl ISpatialAnchorManagerStatics {
    #[inline] pub fn request_store_async(&self) -> Result<foundation::IAsyncOperation<SpatialAnchorStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestStoreAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs, 2716343992, 22215, 12567, 162, 228, 129, 224, 252, 242, 142, 0);
RT_INTERFACE!{interface ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(ISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs] {
    fn get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self, out: *mut foundation::numerics::Matrix4x4) -> HRESULT
}}
impl ISpatialAnchorRawCoordinateSystemAdjustedEventArgs {
    #[inline] pub fn get_old_raw_coordinate_system_to_new_raw_coordinate_system_transform(&self) -> Result<foundation::numerics::Matrix4x4> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorRawCoordinateSystemAdjustedEventArgs: ISpatialAnchorRawCoordinateSystemAdjustedEventArgs}
DEFINE_IID!(IID_ISpatialAnchorStatics, 2844952130, 372, 12572, 174, 121, 14, 81, 7, 102, 159, 22);
RT_INTERFACE!{static interface ISpatialAnchorStatics(ISpatialAnchorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStatics] {
    fn TryCreateRelativeTo(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, out: *mut <SpatialAnchor as RtType>::Abi) -> HRESULT,
    fn TryCreateWithPositionRelativeTo(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3, out: *mut <SpatialAnchor as RtType>::Abi) -> HRESULT,
    fn TryCreateWithPositionAndOrientationRelativeTo(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion, out: *mut <SpatialAnchor as RtType>::Abi) -> HRESULT
}}
impl ISpatialAnchorStatics {
    #[inline] pub fn try_create_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem) -> Result<Option<SpatialAnchor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryCreateRelativeTo)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialAnchor::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_with_position_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem, position: foundation::numerics::Vector3) -> Result<Option<SpatialAnchor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryCreateWithPositionRelativeTo)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(SpatialAnchor::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_with_position_and_orientation_relative_to(&self, coordinateSystem: &SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<Option<SpatialAnchor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryCreateWithPositionAndOrientationRelativeTo)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position, orientation, &mut out);
        if hr == S_OK { Ok(SpatialAnchor::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorStore, 2965124662, 18538, 15536, 158, 111, 18, 69, 22, 92, 77, 182);
RT_INTERFACE!{interface ISpatialAnchorStore(ISpatialAnchorStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStore] {
    fn GetAllSavedAnchors(&self, out: *mut <foundation::collections::IMapView<HString, SpatialAnchor> as RtType>::Abi) -> HRESULT,
    fn TrySave(&self, id: HSTRING, anchor: <SpatialAnchor as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn Remove(&self, id: HSTRING) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ISpatialAnchorStore {
    #[inline] pub fn get_all_saved_anchors(&self) -> Result<Option<foundation::collections::IMapView<HString, SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetAllSavedAnchors)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_save(&self, id: &HStringArg, anchor: &SpatialAnchor) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().TrySave)(self.get_abi() as *const _ as *mut _, id.get(), anchor.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove(&self, id: &HStringArg) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Remove)(self.get_abi() as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Clear)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorStore: ISpatialAnchorStore}
RT_CLASS!{static class SpatialAnchorTransferManager}
impl RtActivatable<ISpatialAnchorTransferManagerStatics> for SpatialAnchorTransferManager {}
impl SpatialAnchorTransferManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_import_anchors_async(stream: &super::super::storage::streams::IInputStream) -> Result<foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_import_anchors_async(stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchors_async(anchors: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_export_anchors_async(anchors, stream)
    }
    #[inline] pub fn request_access_async() -> Result<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().request_access_async()
    }
}
DEFINE_CLSID!(SpatialAnchorTransferManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,84,114,97,110,115,102,101,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorTransferManager]);
DEFINE_IID!(IID_ISpatialAnchorTransferManagerStatics, 62650809, 4824, 19406, 136, 53, 197, 223, 58, 192, 173, 171);
RT_INTERFACE!{static interface ISpatialAnchorTransferManagerStatics(ISpatialAnchorTransferManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorTransferManagerStatics] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryImportAnchorsAsync(&self, stream: <super::super::storage::streams::IInputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryExportAnchorsAsync(&self, anchors: <foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>> as RtType>::Abi, stream: <super::super::storage::streams::IOutputStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut <foundation::IAsyncOperation<SpatialPerceptionAccessStatus> as RtType>::Abi) -> HRESULT
}}
impl ISpatialAnchorTransferManagerStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_import_anchors_async(&self, stream: &super::super::storage::streams::IInputStream) -> Result<foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryImportAnchorsAsync)(self.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchors_async(&self, anchors: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: &super::super::storage::streams::IOutputStream) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryExportAnchorsAsync)(self.get_abi() as *const _ as *mut _, anchors.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct SpatialBoundingBox {
    Center: foundation::numerics::Vector3, Extents: foundation::numerics::Vector3,
}}
RT_STRUCT! { struct SpatialBoundingFrustum {
    Near: foundation::numerics::Plane, Far: foundation::numerics::Plane, Right: foundation::numerics::Plane, Left: foundation::numerics::Plane, Top: foundation::numerics::Plane, Bottom: foundation::numerics::Plane,
}}
RT_STRUCT! { struct SpatialBoundingOrientedBox {
    Center: foundation::numerics::Vector3, Extents: foundation::numerics::Vector3, Orientation: foundation::numerics::Quaternion,
}}
RT_STRUCT! { struct SpatialBoundingSphere {
    Center: foundation::numerics::Vector3, Radius: f32,
}}
DEFINE_IID!(IID_ISpatialBoundingVolume, 4213204442, 26819, 13279, 183, 175, 76, 120, 114, 7, 153, 156);
RT_INTERFACE!{interface ISpatialBoundingVolume(ISpatialBoundingVolumeVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialBoundingVolume] {
    
}}
RT_CLASS!{class SpatialBoundingVolume: ISpatialBoundingVolume}
impl RtActivatable<ISpatialBoundingVolumeStatics> for SpatialBoundingVolume {}
impl SpatialBoundingVolume {
    #[inline] pub fn from_box(coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingBox) -> Result<Option<SpatialBoundingVolume>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_box(coordinateSystem, box_)
    }
    #[inline] pub fn from_oriented_box(coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox) -> Result<Option<SpatialBoundingVolume>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_oriented_box(coordinateSystem, box_)
    }
    #[inline] pub fn from_sphere(coordinateSystem: &SpatialCoordinateSystem, sphere: SpatialBoundingSphere) -> Result<Option<SpatialBoundingVolume>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_sphere(coordinateSystem, sphere)
    }
    #[inline] pub fn from_frustum(coordinateSystem: &SpatialCoordinateSystem, frustum: SpatialBoundingFrustum) -> Result<Option<SpatialBoundingVolume>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_frustum(coordinateSystem, frustum)
    }
}
DEFINE_CLSID!(SpatialBoundingVolume(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,66,111,117,110,100,105,110,103,86,111,108,117,109,101,0]) [CLSID_SpatialBoundingVolume]);
DEFINE_IID!(IID_ISpatialBoundingVolumeStatics, 92836119, 46049, 14040, 176, 23, 86, 97, 129, 165, 177, 150);
RT_INTERFACE!{static interface ISpatialBoundingVolumeStatics(ISpatialBoundingVolumeStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialBoundingVolumeStatics] {
    fn FromBox(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, box_: SpatialBoundingBox, out: *mut <SpatialBoundingVolume as RtType>::Abi) -> HRESULT,
    fn FromOrientedBox(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, box_: SpatialBoundingOrientedBox, out: *mut <SpatialBoundingVolume as RtType>::Abi) -> HRESULT,
    fn FromSphere(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, sphere: SpatialBoundingSphere, out: *mut <SpatialBoundingVolume as RtType>::Abi) -> HRESULT,
    fn FromFrustum(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, frustum: SpatialBoundingFrustum, out: *mut <SpatialBoundingVolume as RtType>::Abi) -> HRESULT
}}
impl ISpatialBoundingVolumeStatics {
    #[inline] pub fn from_box(&self, coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingBox) -> Result<Option<SpatialBoundingVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromBox)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(SpatialBoundingVolume::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_oriented_box(&self, coordinateSystem: &SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox) -> Result<Option<SpatialBoundingVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromOrientedBox)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(SpatialBoundingVolume::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_sphere(&self, coordinateSystem: &SpatialCoordinateSystem, sphere: SpatialBoundingSphere) -> Result<Option<SpatialBoundingVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromSphere)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, sphere, &mut out);
        if hr == S_OK { Ok(SpatialBoundingVolume::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_frustum(&self, coordinateSystem: &SpatialCoordinateSystem, frustum: SpatialBoundingFrustum) -> Result<Option<SpatialBoundingVolume>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().FromFrustum)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, frustum, &mut out);
        if hr == S_OK { Ok(SpatialBoundingVolume::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialCoordinateSystem, 1777060427, 24739, 13702, 166, 83, 89, 167, 189, 103, 109, 7);
RT_INTERFACE!{interface ISpatialCoordinateSystem(ISpatialCoordinateSystemVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialCoordinateSystem] {
    fn TryGetTransformTo(&self, target: <SpatialCoordinateSystem as RtType>::Abi, out: *mut <foundation::IReference<foundation::numerics::Matrix4x4> as RtType>::Abi) -> HRESULT
}}
impl ISpatialCoordinateSystem {
    #[inline] pub fn try_get_transform_to(&self, target: &SpatialCoordinateSystem) -> Result<Option<foundation::IReference<foundation::numerics::Matrix4x4>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryGetTransformTo)(self.get_abi() as *const _ as *mut _, target.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialCoordinateSystem: ISpatialCoordinateSystem}
DEFINE_IID!(IID_ISpatialEntity, 376301909, 57835, 17740, 186, 8, 230, 192, 102, 141, 220, 101);
RT_INTERFACE!{interface ISpatialEntity(ISpatialEntityVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntity] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Anchor(&self, out: *mut <SpatialAnchor as RtType>::Abi) -> HRESULT,
    fn get_Properties(&self, out: *mut <foundation::collections::ValueSet as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntity {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_anchor(&self) -> Result<Option<SpatialAnchor>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Anchor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialAnchor::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<foundation::collections::ValueSet>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::ValueSet::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntity: ISpatialEntity}
impl RtActivatable<ISpatialEntityFactory> for SpatialEntity {}
impl SpatialEntity {
    #[inline] pub fn create_with_spatial_anchor(spatialAnchor: &SpatialAnchor) -> Result<SpatialEntity> {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor(spatialAnchor)
    }
    #[inline] pub fn create_with_spatial_anchor_and_properties(spatialAnchor: &SpatialAnchor, propertySet: &foundation::collections::ValueSet) -> Result<SpatialEntity> {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor_and_properties(spatialAnchor, propertySet)
    }
}
DEFINE_CLSID!(SpatialEntity(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,0]) [CLSID_SpatialEntity]);
DEFINE_IID!(IID_ISpatialEntityAddedEventArgs, 2744644763, 5482, 18183, 172, 44, 211, 29, 87, 14, 211, 153);
RT_INTERFACE!{interface ISpatialEntityAddedEventArgs(ISpatialEntityAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityAddedEventArgs] {
    fn get_Entity(&self, out: *mut <SpatialEntity as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityAddedEventArgs {
    #[inline] pub fn get_entity(&self) -> Result<Option<SpatialEntity>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Entity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntity::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityAddedEventArgs: ISpatialEntityAddedEventArgs}
DEFINE_IID!(IID_ISpatialEntityFactory, 3790725925, 13471, 16933, 162, 243, 75, 1, 193, 95, 224, 86);
RT_INTERFACE!{static interface ISpatialEntityFactory(ISpatialEntityFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityFactory] {
    fn CreateWithSpatialAnchor(&self, spatialAnchor: <SpatialAnchor as RtType>::Abi, out: *mut <SpatialEntity as RtType>::Abi) -> HRESULT,
    fn CreateWithSpatialAnchorAndProperties(&self, spatialAnchor: <SpatialAnchor as RtType>::Abi, propertySet: <foundation::collections::ValueSet as RtType>::Abi, out: *mut <SpatialEntity as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityFactory {
    #[inline] pub fn create_with_spatial_anchor(&self, spatialAnchor: &SpatialAnchor) -> Result<SpatialEntity> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithSpatialAnchor)(self.get_abi() as *const _ as *mut _, spatialAnchor.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntity::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_spatial_anchor_and_properties(&self, spatialAnchor: &SpatialAnchor, propertySet: &foundation::collections::ValueSet) -> Result<SpatialEntity> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateWithSpatialAnchorAndProperties)(self.get_abi() as *const _ as *mut _, spatialAnchor.get_abi() as *const _ as *mut _, propertySet.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntity::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialEntityRemovedEventArgs, 2440304640, 21357, 20127, 171, 246, 65, 91, 84, 68, 214, 81);
RT_INTERFACE!{interface ISpatialEntityRemovedEventArgs(ISpatialEntityRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityRemovedEventArgs] {
    fn get_Entity(&self, out: *mut <SpatialEntity as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityRemovedEventArgs {
    #[inline] pub fn get_entity(&self) -> Result<Option<SpatialEntity>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Entity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntity::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityRemovedEventArgs: ISpatialEntityRemovedEventArgs}
DEFINE_IID!(IID_ISpatialEntityStore, 848791738, 58643, 20230, 136, 157, 27, 227, 14, 207, 67, 230);
RT_INTERFACE!{interface ISpatialEntityStore(ISpatialEntityStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStore] {
    fn SaveAsync(&self, entity: <SpatialEntity as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn RemoveAsync(&self, entity: <SpatialEntity as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn CreateEntityWatcher(&self, out: *mut <SpatialEntityWatcher as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityStore {
    #[inline] pub fn save_async(&self, entity: &SpatialEntity) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().SaveAsync)(self.get_abi() as *const _ as *mut _, entity.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_async(&self, entity: &SpatialEntity) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RemoveAsync)(self.get_abi() as *const _ as *mut _, entity.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_entity_watcher(&self) -> Result<Option<SpatialEntityWatcher>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateEntityWatcher)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntityWatcher::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityStore: ISpatialEntityStore}
impl RtActivatable<ISpatialEntityStoreStatics> for SpatialEntityStore {}
impl SpatialEntityStore {
    #[inline] pub fn get_is_supported() -> Result<bool> {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().get_is_supported()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn try_get_for_remote_system_session(session: &super::super::system::remotesystems::RemoteSystemSession) -> Result<Option<SpatialEntityStore>> {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().try_get_for_remote_system_session(session)
    }
}
DEFINE_CLSID!(SpatialEntityStore(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,83,116,111,114,101,0]) [CLSID_SpatialEntityStore]);
DEFINE_IID!(IID_ISpatialEntityStoreStatics, 1800091806, 31824, 20114, 138, 98, 77, 29, 75, 124, 205, 62);
RT_INTERFACE!{static interface ISpatialEntityStoreStatics(ISpatialEntityStoreStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStoreStatics] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-system")] fn TryGetForRemoteSystemSession(&self, session: <super::super::system::remotesystems::RemoteSystemSession as RtType>::Abi, out: *mut <SpatialEntityStore as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityStoreStatics {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn try_get_for_remote_system_session(&self, session: &super::super::system::remotesystems::RemoteSystemSession) -> Result<Option<SpatialEntityStore>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryGetForRemoteSystemSession)(self.get_abi() as *const _ as *mut _, session.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntityStore::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialEntityUpdatedEventArgs, 3848738662, 25211, 17355, 164, 159, 179, 190, 109, 71, 222, 237);
RT_INTERFACE!{interface ISpatialEntityUpdatedEventArgs(ISpatialEntityUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityUpdatedEventArgs] {
    fn get_Entity(&self, out: *mut <SpatialEntity as RtType>::Abi) -> HRESULT
}}
impl ISpatialEntityUpdatedEventArgs {
    #[inline] pub fn get_entity(&self) -> Result<Option<SpatialEntity>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Entity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialEntity::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityUpdatedEventArgs: ISpatialEntityUpdatedEventArgs}
DEFINE_IID!(IID_ISpatialEntityWatcher, 3015204768, 27998, 19388, 128, 93, 95, 229, 185, 186, 25, 89);
RT_INTERFACE!{interface ISpatialEntityWatcher(ISpatialEntityWatcherVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityWatcher] {
    fn get_Status(&self, out: *mut SpatialEntityWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: <foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: <foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: <foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: <foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl ISpatialEntityWatcher {
    #[inline] pub fn get_status(&self) -> Result<SpatialEntityWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_added(&self, handler: &foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_Added)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_Added)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_Updated)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_Updated)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_Removed)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_Removed)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_EnumerationCompleted)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_EnumerationCompleted)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Start)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().Stop)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityWatcher: ISpatialEntityWatcher}
RT_ENUM! { enum SpatialEntityWatcherStatus: i32 {
    Created = 0, Started = 1, EnumerationCompleted = 2, Stopping = 3, Stopped = 4, Aborted = 5,
}}
RT_ENUM! { enum SpatialLocatability: i32 {
    Unavailable = 0, OrientationOnly = 1, PositionalTrackingActivating = 2, PositionalTrackingActive = 3, PositionalTrackingInhibited = 4,
}}
DEFINE_IID!(IID_ISpatialLocation, 495047325, 9377, 14293, 143, 161, 57, 180, 249, 173, 103, 226);
RT_INTERFACE!{interface ISpatialLocation(ISpatialLocationVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocation] {
    fn get_Position(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_Orientation(&self, out: *mut foundation::numerics::Quaternion) -> HRESULT,
    fn get_AbsoluteLinearVelocity(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteLinearAcceleration(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteAngularVelocity(&self, out: *mut foundation::numerics::Quaternion) -> HRESULT,
    fn get_AbsoluteAngularAcceleration(&self, out: *mut foundation::numerics::Quaternion) -> HRESULT
}}
impl ISpatialLocation {
    #[inline] pub fn get_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Position)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_linear_velocity(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteLinearVelocity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_linear_acceleration(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteLinearAcceleration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_velocity(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteAngularVelocity)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_acceleration(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteAngularAcceleration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocation: ISpatialLocation}
DEFINE_IID!(IID_ISpatialLocation2, 293544982, 14503, 18968, 180, 4, 171, 143, 171, 225, 215, 139);
RT_INTERFACE!{interface ISpatialLocation2(ISpatialLocation2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocation2] {
    fn get_AbsoluteAngularVelocityAxisAngle(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteAngularAccelerationAxisAngle(&self, out: *mut foundation::numerics::Vector3) -> HRESULT
}}
impl ISpatialLocation2 {
    #[inline] pub fn get_absolute_angular_velocity_axis_angle(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteAngularVelocityAxisAngle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_acceleration_axis_angle(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_AbsoluteAngularAccelerationAxisAngle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialLocator, 4131883301, 40460, 15286, 153, 126, 182, 78, 204, 162, 76, 244);
RT_INTERFACE!{interface ISpatialLocator(ISpatialLocatorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocator] {
    fn get_Locatability(&self, out: *mut SpatialLocatability) -> HRESULT,
    fn add_LocatabilityChanged(&self, handler: <foundation::TypedEventHandler<SpatialLocator, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LocatabilityChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_PositionalTrackingDeactivating(&self, handler: <foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PositionalTrackingDeactivating(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn TryLocateAtTimestamp(&self, timestamp: <super::PerceptionTimestamp as RtType>::Abi, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, out: *mut <SpatialLocation as RtType>::Abi) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self, out: *mut <SpatialLocatorAttachedFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativePosition: foundation::numerics::Vector3, out: *mut <SpatialLocatorAttachedFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut <SpatialLocatorAttachedFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut <SpatialLocatorAttachedFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self, out: *mut <SpatialStationaryFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativePosition: foundation::numerics::Vector3, out: *mut <SpatialStationaryFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut <SpatialStationaryFrameOfReference as RtType>::Abi) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut <SpatialStationaryFrameOfReference as RtType>::Abi) -> HRESULT
}}
impl ISpatialLocator {
    #[inline] pub fn get_locatability(&self) -> Result<SpatialLocatability> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Locatability)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_locatability_changed(&self, handler: &foundation::TypedEventHandler<SpatialLocator, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_LocatabilityChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_locatability_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_LocatabilityChanged)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_positional_tracking_deactivating(&self, handler: &foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_PositionalTrackingDeactivating)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_positional_tracking_deactivating(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_PositionalTrackingDeactivating)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_locate_at_timestamp(&self, timestamp: &super::PerceptionTimestamp, coordinateSystem: &SpatialCoordinateSystem) -> Result<Option<SpatialLocation>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryLocateAtTimestamp)(self.get_abi() as *const _ as *mut _, timestamp.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialLocation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading(&self) -> Result<Option<SpatialLocatorAttachedFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateAttachedFrameOfReferenceAtCurrentHeading)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialLocatorAttachedFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position(&self, relativePosition: foundation::numerics::Vector3) -> Result<Option<SpatialLocatorAttachedFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(self.get_abi() as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(SpatialLocatorAttachedFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<SpatialLocatorAttachedFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(self.get_abi() as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(SpatialLocatorAttachedFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation_and_relative_heading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<Option<SpatialLocatorAttachedFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(self.get_abi() as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(SpatialLocatorAttachedFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location(&self) -> Result<Option<SpatialStationaryFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateStationaryFrameOfReferenceAtCurrentLocation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialStationaryFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position(&self, relativePosition: foundation::numerics::Vector3) -> Result<Option<SpatialStationaryFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(self.get_abi() as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(SpatialStationaryFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<SpatialStationaryFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(self.get_abi() as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(SpatialStationaryFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation_and_relative_heading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<Option<SpatialStationaryFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(self.get_abi() as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(SpatialStationaryFrameOfReference::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocator: ISpatialLocator}
impl RtActivatable<ISpatialLocatorStatics> for SpatialLocator {}
impl SpatialLocator {
    #[inline] pub fn get_default() -> Result<Option<SpatialLocator>> {
        <Self as RtActivatable<ISpatialLocatorStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(SpatialLocator(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,76,111,99,97,116,111,114,0]) [CLSID_SpatialLocator]);
DEFINE_IID!(IID_ISpatialLocatorAttachedFrameOfReference, 3782692598, 8015, 18844, 150, 37, 239, 94, 110, 215, 160, 72);
RT_INTERFACE!{interface ISpatialLocatorAttachedFrameOfReference(ISpatialLocatorAttachedFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorAttachedFrameOfReference] {
    fn get_RelativePosition(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn put_RelativePosition(&self, value: foundation::numerics::Vector3) -> HRESULT,
    fn get_RelativeOrientation(&self, out: *mut foundation::numerics::Quaternion) -> HRESULT,
    fn put_RelativeOrientation(&self, value: foundation::numerics::Quaternion) -> HRESULT,
    fn AdjustHeading(&self, headingOffsetInRadians: f64) -> HRESULT,
    fn GetStationaryCoordinateSystemAtTimestamp(&self, timestamp: <super::PerceptionTimestamp as RtType>::Abi, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn TryGetRelativeHeadingAtTimestamp(&self, timestamp: <super::PerceptionTimestamp as RtType>::Abi, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT
}}
impl ISpatialLocatorAttachedFrameOfReference {
    #[inline] pub fn get_relative_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RelativePosition)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_relative_position(&self, value: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RelativePosition)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_relative_orientation(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_RelativeOrientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_relative_orientation(&self, value: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_RelativeOrientation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn adjust_heading(&self, headingOffsetInRadians: f64) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().AdjustHeading)(self.get_abi() as *const _ as *mut _, headingOffsetInRadians);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stationary_coordinate_system_at_timestamp(&self, timestamp: &super::PerceptionTimestamp) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetStationaryCoordinateSystemAtTimestamp)(self.get_abi() as *const _ as *mut _, timestamp.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_get_relative_heading_at_timestamp(&self, timestamp: &super::PerceptionTimestamp) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryGetRelativeHeadingAtTimestamp)(self.get_abi() as *const _ as *mut _, timestamp.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocatorAttachedFrameOfReference: ISpatialLocatorAttachedFrameOfReference}
DEFINE_IID!(IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs, 3098034275, 58356, 13963, 144, 97, 158, 169, 209, 214, 204, 22);
RT_INTERFACE!{interface ISpatialLocatorPositionalTrackingDeactivatingEventArgs(ISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs] {
    fn get_Canceled(&self, out: *mut bool) -> HRESULT,
    fn put_Canceled(&self, value: bool) -> HRESULT
}}
impl ISpatialLocatorPositionalTrackingDeactivatingEventArgs {
    #[inline] pub fn get_canceled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Canceled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_canceled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_Canceled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocatorPositionalTrackingDeactivatingEventArgs: ISpatialLocatorPositionalTrackingDeactivatingEventArgs}
DEFINE_IID!(IID_ISpatialLocatorStatics, 3077452608, 42946, 13851, 187, 130, 86, 233, 59, 137, 177, 187);
RT_INTERFACE!{static interface ISpatialLocatorStatics(ISpatialLocatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorStatics] {
    fn GetDefault(&self, out: *mut <SpatialLocator as RtType>::Abi) -> HRESULT
}}
impl ISpatialLocatorStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<SpatialLocator>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialLocator::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum SpatialLookDirectionRange: i32 {
    ForwardOnly = 0, Omnidirectional = 1,
}}
RT_ENUM! { enum SpatialMovementRange: i32 {
    NoMovement = 0, Bounded = 1,
}}
RT_ENUM! { enum SpatialPerceptionAccessStatus: i32 {
    Unspecified = 0, Allowed = 1, DeniedByUser = 2, DeniedBySystem = 3,
}}
DEFINE_IID!(IID_ISpatialStageFrameOfReference, 2055877732, 44301, 17808, 171, 134, 51, 6, 43, 103, 73, 38);
RT_INTERFACE!{interface ISpatialStageFrameOfReference(ISpatialStageFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStageFrameOfReference] {
    fn get_CoordinateSystem(&self, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn get_MovementRange(&self, out: *mut SpatialMovementRange) -> HRESULT,
    fn get_LookDirectionRange(&self, out: *mut SpatialLookDirectionRange) -> HRESULT,
    fn GetCoordinateSystemAtCurrentLocation(&self, locator: <SpatialLocator as RtType>::Abi, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn TryGetMovementBounds(&self, coordinateSystem: <SpatialCoordinateSystem as RtType>::Abi, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector3) -> HRESULT
}}
impl ISpatialStageFrameOfReference {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CoordinateSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_movement_range(&self) -> Result<SpatialMovementRange> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_MovementRange)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_look_direction_range(&self) -> Result<SpatialLookDirectionRange> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_LookDirectionRange)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_coordinate_system_at_current_location(&self, locator: &SpatialLocator) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetCoordinateSystemAtCurrentLocation)(self.get_abi() as *const _ as *mut _, locator.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_get_movement_bounds(&self, coordinateSystem: &SpatialCoordinateSystem) -> Result<ComArray<foundation::numerics::Vector3>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = (self.get_vtbl().TryGetMovementBounds)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialStageFrameOfReference: ISpatialStageFrameOfReference}
impl RtActivatable<ISpatialStageFrameOfReferenceStatics> for SpatialStageFrameOfReference {}
impl SpatialStageFrameOfReference {
    #[inline] pub fn get_current() -> Result<Option<SpatialStageFrameOfReference>> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().get_current()
    }
    #[inline] pub fn add_current_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().add_current_changed(handler)
    }
    #[inline] pub fn remove_current_changed(cookie: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().remove_current_changed(cookie)
    }
    #[inline] pub fn request_new_stage_async() -> Result<foundation::IAsyncOperation<SpatialStageFrameOfReference>> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().request_new_stage_async()
    }
}
DEFINE_CLSID!(SpatialStageFrameOfReference(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,83,116,97,103,101,70,114,97,109,101,79,102,82,101,102,101,114,101,110,99,101,0]) [CLSID_SpatialStageFrameOfReference]);
DEFINE_IID!(IID_ISpatialStageFrameOfReferenceStatics, 4153236557, 41124, 18844, 141, 145, 168, 201, 101, 212, 6, 84);
RT_INTERFACE!{static interface ISpatialStageFrameOfReferenceStatics(ISpatialStageFrameOfReferenceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStageFrameOfReferenceStatics] {
    fn get_Current(&self, out: *mut <SpatialStageFrameOfReference as RtType>::Abi) -> HRESULT,
    fn add_CurrentChanged(&self, handler: <foundation::EventHandler<IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CurrentChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn RequestNewStageAsync(&self, out: *mut <foundation::IAsyncOperation<SpatialStageFrameOfReference> as RtType>::Abi) -> HRESULT
}}
impl ISpatialStageFrameOfReferenceStatics {
    #[inline] pub fn get_current(&self) -> Result<Option<SpatialStageFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Current)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialStageFrameOfReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_current_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_CurrentChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_current_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_CurrentChanged)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn request_new_stage_async(&self) -> Result<foundation::IAsyncOperation<SpatialStageFrameOfReference>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestNewStageAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialStationaryFrameOfReference, 165399737, 48376, 15999, 190, 126, 126, 220, 203, 177, 120, 168);
RT_INTERFACE!{interface ISpatialStationaryFrameOfReference(ISpatialStationaryFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStationaryFrameOfReference] {
    fn get_CoordinateSystem(&self, out: *mut <SpatialCoordinateSystem as RtType>::Abi) -> HRESULT
}}
impl ISpatialStationaryFrameOfReference {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CoordinateSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialStationaryFrameOfReference: ISpatialStationaryFrameOfReference}
pub mod preview { // Windows.Perception.Spatial.Preview
use crate::prelude::*;
RT_CLASS!{static class SpatialGraphInteropPreview}
impl RtActivatable<ISpatialGraphInteropPreviewStatics> for SpatialGraphInteropPreview {}
impl SpatialGraphInteropPreview {
    #[inline] pub fn create_coordinate_system_for_node(nodeId: Guid) -> Result<Option<super::SpatialCoordinateSystem>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node(nodeId)
    }
    #[inline] pub fn create_coordinate_system_for_node_with_position(nodeId: Guid, relativePosition: foundation::numerics::Vector3) -> Result<Option<super::SpatialCoordinateSystem>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node_with_position(nodeId, relativePosition)
    }
    #[inline] pub fn create_coordinate_system_for_node_with_position_and_orientation(nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<super::SpatialCoordinateSystem>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node_with_position_and_orientation(nodeId, relativePosition, relativeOrientation)
    }
    #[inline] pub fn create_locator_for_node(nodeId: Guid) -> Result<Option<super::SpatialLocator>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_locator_for_node(nodeId)
    }
}
DEFINE_CLSID!(SpatialGraphInteropPreview(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,80,114,101,118,105,101,119,46,83,112,97,116,105,97,108,71,114,97,112,104,73,110,116,101,114,111,112,80,114,101,118,105,101,119,0]) [CLSID_SpatialGraphInteropPreview]);
DEFINE_IID!(IID_ISpatialGraphInteropPreviewStatics, 3225576524, 8408, 20176, 174, 247, 104, 5, 184, 229, 63, 85);
RT_INTERFACE!{static interface ISpatialGraphInteropPreviewStatics(ISpatialGraphInteropPreviewStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialGraphInteropPreviewStatics] {
    fn CreateCoordinateSystemForNode(&self, nodeId: Guid, out: *mut <super::SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn CreateCoordinateSystemForNodeWithPosition(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, out: *mut <super::SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn CreateCoordinateSystemForNodeWithPositionAndOrientation(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut <super::SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn CreateLocatorForNode(&self, nodeId: Guid, out: *mut <super::SpatialLocator as RtType>::Abi) -> HRESULT
}}
impl ISpatialGraphInteropPreviewStatics {
    #[inline] pub fn create_coordinate_system_for_node(&self, nodeId: Guid) -> Result<Option<super::SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCoordinateSystemForNode)(self.get_abi() as *const _ as *mut _, nodeId, &mut out);
        if hr == S_OK { Ok(super::SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_coordinate_system_for_node_with_position(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3) -> Result<Option<super::SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCoordinateSystemForNodeWithPosition)(self.get_abi() as *const _ as *mut _, nodeId, relativePosition, &mut out);
        if hr == S_OK { Ok(super::SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_coordinate_system_for_node_with_position_and_orientation(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<super::SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateCoordinateSystemForNodeWithPositionAndOrientation)(self.get_abi() as *const _ as *mut _, nodeId, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(super::SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_locator_for_node(&self, nodeId: Guid) -> Result<Option<super::SpatialLocator>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().CreateLocatorForNode)(self.get_abi() as *const _ as *mut _, nodeId, &mut out);
        if hr == S_OK { Ok(super::SpatialLocator::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Perception.Spatial.Preview
pub mod surfaces { // Windows.Perception.Spatial.Surfaces
use crate::prelude::*;
DEFINE_IID!(IID_ISpatialSurfaceInfo, 4176079847, 14775, 14690, 187, 3, 87, 245, 110, 31, 176, 161);
RT_INTERFACE!{interface ISpatialSurfaceInfo(ISpatialSurfaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceInfo] {
    fn get_Id(&self, out: *mut Guid) -> HRESULT,
    fn get_UpdateTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn TryGetBounds(&self, coordinateSystem: <super::SpatialCoordinateSystem as RtType>::Abi, out: *mut <foundation::IReference<super::SpatialBoundingOrientedBox> as RtType>::Abi) -> HRESULT,
    fn TryComputeLatestMeshAsync(&self, maxTrianglesPerCubicMeter: f64, out: *mut <foundation::IAsyncOperation<SpatialSurfaceMesh> as RtType>::Abi) -> HRESULT,
    fn TryComputeLatestMeshWithOptionsAsync(&self, maxTrianglesPerCubicMeter: f64, options: <SpatialSurfaceMeshOptions as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SpatialSurfaceMesh> as RtType>::Abi) -> HRESULT
}}
impl ISpatialSurfaceInfo {
    #[inline] pub fn get_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_update_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_UpdateTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_get_bounds(&self, coordinateSystem: &super::SpatialCoordinateSystem) -> Result<Option<foundation::IReference<super::SpatialBoundingOrientedBox>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryGetBounds)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_compute_latest_mesh_async(&self, maxTrianglesPerCubicMeter: f64) -> Result<foundation::IAsyncOperation<SpatialSurfaceMesh>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryComputeLatestMeshAsync)(self.get_abi() as *const _ as *mut _, maxTrianglesPerCubicMeter, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_compute_latest_mesh_with_options_async(&self, maxTrianglesPerCubicMeter: f64, options: &SpatialSurfaceMeshOptions) -> Result<foundation::IAsyncOperation<SpatialSurfaceMesh>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().TryComputeLatestMeshWithOptionsAsync)(self.get_abi() as *const _ as *mut _, maxTrianglesPerCubicMeter, options.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceInfo: ISpatialSurfaceInfo}
DEFINE_IID!(IID_ISpatialSurfaceMesh, 277829593, 57101, 14672, 160, 253, 249, 114, 199, 124, 39, 180);
RT_INTERFACE!{interface ISpatialSurfaceMesh(ISpatialSurfaceMeshVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMesh] {
    fn get_SurfaceInfo(&self, out: *mut <SpatialSurfaceInfo as RtType>::Abi) -> HRESULT,
    fn get_CoordinateSystem(&self, out: *mut <super::SpatialCoordinateSystem as RtType>::Abi) -> HRESULT,
    fn get_TriangleIndices(&self, out: *mut <SpatialSurfaceMeshBuffer as RtType>::Abi) -> HRESULT,
    fn get_VertexPositions(&self, out: *mut <SpatialSurfaceMeshBuffer as RtType>::Abi) -> HRESULT,
    fn get_VertexPositionScale(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_VertexNormals(&self, out: *mut <SpatialSurfaceMeshBuffer as RtType>::Abi) -> HRESULT
}}
impl ISpatialSurfaceMesh {
    #[inline] pub fn get_surface_info(&self) -> Result<Option<SpatialSurfaceInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SurfaceInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialSurfaceInfo::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<super::SpatialCoordinateSystem>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_CoordinateSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::SpatialCoordinateSystem::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_indices(&self) -> Result<Option<SpatialSurfaceMeshBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_TriangleIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialSurfaceMeshBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_positions(&self) -> Result<Option<SpatialSurfaceMeshBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VertexPositions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialSurfaceMeshBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_position_scale(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_VertexPositionScale)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_normals(&self) -> Result<Option<SpatialSurfaceMeshBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_VertexNormals)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SpatialSurfaceMeshBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceMesh: ISpatialSurfaceMesh}
DEFINE_IID!(IID_ISpatialSurfaceMeshBuffer, 2479839712, 34591, 13304, 152, 178, 3, 209, 1, 69, 143, 111);
RT_INTERFACE!{interface ISpatialSurfaceMeshBuffer(ISpatialSurfaceMeshBufferVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshBuffer] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_Format(&self, out: *mut crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    fn get_Stride(&self, out: *mut u32) -> HRESULT,
    fn get_ElementCount(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT
}}
impl ISpatialSurfaceMeshBuffer {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Format)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_stride(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_Stride)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_element_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_ElementCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<crate::windows::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_Data)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceMeshBuffer: ISpatialSurfaceMeshBuffer}
DEFINE_IID!(IID_ISpatialSurfaceMeshOptions, 3530923913, 13682, 15661, 161, 13, 95, 238, 147, 148, 170, 55);
RT_INTERFACE!{interface ISpatialSurfaceMeshOptions(ISpatialSurfaceMeshOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshOptions] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_VertexPositionFormat(&self, out: *mut crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_VertexPositionFormat(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_TriangleIndexFormat(&self, out: *mut crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_TriangleIndexFormat(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_VertexNormalFormat(&self, out: *mut crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-graphics")] fn put_VertexNormalFormat(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    fn get_IncludeVertexNormals(&self, out: *mut bool) -> HRESULT,
    fn put_IncludeVertexNormals(&self, value: bool) -> HRESULT
}}
impl ISpatialSurfaceMeshOptions {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_vertex_position_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_VertexPositionFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_vertex_position_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_VertexPositionFormat)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_triangle_index_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_TriangleIndexFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_triangle_index_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_TriangleIndexFormat)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_vertex_normal_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_VertexNormalFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_vertex_normal_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_VertexNormalFormat)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_include_vertex_normals(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().get_IncludeVertexNormals)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_include_vertex_normals(&self, value: bool) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().put_IncludeVertexNormals)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceMeshOptions: ISpatialSurfaceMeshOptions}
impl RtActivatable<ISpatialSurfaceMeshOptionsStatics> for SpatialSurfaceMeshOptions {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceMeshOptions {}
impl SpatialSurfaceMeshOptions {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_position_formats() -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_position_formats()
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_triangle_index_formats() -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_triangle_index_formats()
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_normal_formats() -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_normal_formats()
    }
}
DEFINE_CLSID!(SpatialSurfaceMeshOptions(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,77,101,115,104,79,112,116,105,111,110,115,0]) [CLSID_SpatialSurfaceMeshOptions]);
DEFINE_IID!(IID_ISpatialSurfaceMeshOptionsStatics, 2603879103, 38785, 17669, 137, 53, 1, 53, 117, 202, 174, 94);
RT_INTERFACE!{static interface ISpatialSurfaceMeshOptionsStatics(ISpatialSurfaceMeshOptionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshOptionsStatics] {
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexPositionFormats(&self, out: *mut <foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedTriangleIndexFormats(&self, out: *mut <foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexNormalFormats(&self, out: *mut <foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat> as RtType>::Abi) -> HRESULT
}}
impl ISpatialSurfaceMeshOptionsStatics {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_position_formats(&self) -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SupportedVertexPositionFormats)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_triangle_index_formats(&self) -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SupportedTriangleIndexFormats)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_normal_formats(&self) -> Result<Option<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().get_SupportedVertexNormalFormats)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialSurfaceObserver, 280401945, 56778, 13443, 172, 58, 116, 143, 232, 200, 109, 245);
RT_INTERFACE!{interface ISpatialSurfaceObserver(ISpatialSurfaceObserverVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserver] {
    fn GetObservedSurfaces(&self, out: *mut <foundation::collections::IMapView<Guid, SpatialSurfaceInfo> as RtType>::Abi) -> HRESULT,
    fn SetBoundingVolume(&self, bounds: <super::SpatialBoundingVolume as RtType>::Abi) -> HRESULT,
    fn SetBoundingVolumes(&self, bounds: <foundation::collections::IIterable<super::SpatialBoundingVolume> as RtType>::Abi) -> HRESULT,
    fn add_ObservedSurfacesChanged(&self, handler: <foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ObservedSurfacesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ISpatialSurfaceObserver {
    #[inline] pub fn get_observed_surfaces(&self) -> Result<Option<foundation::collections::IMapView<Guid, SpatialSurfaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().GetObservedSurfaces)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_bounding_volume(&self, bounds: &super::SpatialBoundingVolume) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetBoundingVolume)(self.get_abi() as *const _ as *mut _, bounds.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_bounding_volumes(&self, bounds: &foundation::collections::IIterable<super::SpatialBoundingVolume>) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().SetBoundingVolumes)(self.get_abi() as *const _ as *mut _, bounds.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_observed_surfaces_changed(&self, handler: &foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().add_ObservedSurfacesChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_observed_surfaces_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = (self.get_vtbl().remove_ObservedSurfacesChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceObserver: ISpatialSurfaceObserver}
impl RtActivatable<ISpatialSurfaceObserverStatics> for SpatialSurfaceObserver {}
impl RtActivatable<ISpatialSurfaceObserverStatics2> for SpatialSurfaceObserver {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceObserver {}
impl SpatialSurfaceObserver {
    #[inline] pub fn request_access_async() -> Result<foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics>>::get_activation_factory().request_access_async()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics2>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(SpatialSurfaceObserver(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,79,98,115,101,114,118,101,114,0]) [CLSID_SpatialSurfaceObserver]);
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics, 374952429, 8456, 16744, 145, 117, 135, 224, 39, 188, 146, 133);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics(ISpatialSurfaceObserverStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics] {
    fn RequestAccessAsync(&self, out: *mut <foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus> as RtType>::Abi) -> HRESULT
}}
impl ISpatialSurfaceObserverStatics {
    #[inline] pub fn request_access_async(&self) -> Result<foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>> { unsafe { 
        let mut out = null_mut();
        let hr = (self.get_vtbl().RequestAccessAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics2, 257114721, 50525, 20075, 168, 149, 161, 157, 230, 154, 66, 227);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics2(ISpatialSurfaceObserverStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics2] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl ISpatialSurfaceObserverStatics2 {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = (self.get_vtbl().IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Perception.Spatial.Surfaces
} // Windows.Perception.Spatial
