use crate::prelude::*;
DEFINE_IID!(IID_IPerceptionTimestamp, 2277656580, 41518, 19163, 186, 38, 215, 142, 246, 57, 188, 244);
RT_INTERFACE!{interface IPerceptionTimestamp(IPerceptionTimestampVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestamp] {
    fn get_TargetTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn get_PredictionAmount(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl ComPtr<IPerceptionTimestamp> {
    #[inline] pub fn get_target_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TargetTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_prediction_amount(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PredictionAmount)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PerceptionTimestamp: IPerceptionTimestamp}
DEFINE_IID!(IID_IPerceptionTimestamp2, 3813980141, 11217, 16823, 158, 208, 116, 161, 92, 53, 69, 55);
RT_INTERFACE!{interface IPerceptionTimestamp2(IPerceptionTimestamp2Vtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestamp2] {
    fn get_SystemRelativeTargetTime(&self, out: *mut foundation::TimeSpan) -> HRESULT
}}
impl ComPtr<IPerceptionTimestamp2> {
    #[inline] pub fn get_system_relative_target_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SystemRelativeTargetTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class PerceptionTimestampHelper}
impl RtActivatable<IPerceptionTimestampHelperStatics> for PerceptionTimestampHelper {}
impl RtActivatable<IPerceptionTimestampHelperStatics2> for PerceptionTimestampHelper {}
impl PerceptionTimestampHelper {
    #[inline] pub fn from_historical_target_time(targetTime: foundation::DateTime) -> Result<Option<ComPtr<PerceptionTimestamp>>> {
        <Self as RtActivatable<IPerceptionTimestampHelperStatics>>::get_activation_factory().from_historical_target_time(targetTime)
    }
    #[inline] pub fn from_system_relative_target_time(targetTime: foundation::TimeSpan) -> Result<Option<ComPtr<PerceptionTimestamp>>> {
        <Self as RtActivatable<IPerceptionTimestampHelperStatics2>>::get_activation_factory().from_system_relative_target_time(targetTime)
    }
}
DEFINE_CLSID!(PerceptionTimestampHelper(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,80,101,114,99,101,112,116,105,111,110,84,105,109,101,115,116,97,109,112,72,101,108,112,101,114,0]) [CLSID_PerceptionTimestampHelper]);
DEFINE_IID!(IID_IPerceptionTimestampHelperStatics, 1202065876, 43487, 20188, 133, 93, 244, 211, 57, 217, 103, 172);
RT_INTERFACE!{static interface IPerceptionTimestampHelperStatics(IPerceptionTimestampHelperStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestampHelperStatics] {
    fn FromHistoricalTargetTime(&self, targetTime: foundation::DateTime, out: *mut *mut PerceptionTimestamp) -> HRESULT
}}
impl ComPtr<IPerceptionTimestampHelperStatics> {
    #[inline] pub fn from_historical_target_time(&self, targetTime: foundation::DateTime) -> Result<Option<ComPtr<PerceptionTimestamp>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromHistoricalTargetTime)(self.as_abi() as *const _ as *mut _, targetTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPerceptionTimestampHelperStatics2, 1943119870, 16313, 17777, 135, 212, 60, 146, 10, 94, 134, 235);
RT_INTERFACE!{static interface IPerceptionTimestampHelperStatics2(IPerceptionTimestampHelperStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IPerceptionTimestampHelperStatics2] {
    fn FromSystemRelativeTargetTime(&self, targetTime: foundation::TimeSpan, out: *mut *mut PerceptionTimestamp) -> HRESULT
}}
impl ComPtr<IPerceptionTimestampHelperStatics2> {
    #[inline] pub fn from_system_relative_target_time(&self, targetTime: foundation::TimeSpan) -> Result<Option<ComPtr<PerceptionTimestamp>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromSystemRelativeTargetTime)(self.as_abi() as *const _ as *mut _, targetTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
pub mod automation { // Windows.Perception.Automation
pub mod core { // Windows.Perception.Automation.Core
use crate::prelude::*;
RT_CLASS!{static class CorePerceptionAutomation}
impl RtActivatable<ICorePerceptionAutomationStatics> for CorePerceptionAutomation {}
impl CorePerceptionAutomation {
    #[inline] pub fn set_activation_factory_provider(provider: &ComPtr<foundation::IGetActivationFactory>) -> Result<()> {
        <Self as RtActivatable<ICorePerceptionAutomationStatics>>::get_activation_factory().set_activation_factory_provider(provider)
    }
}
DEFINE_CLSID!(CorePerceptionAutomation(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,65,117,116,111,109,97,116,105,111,110,46,67,111,114,101,46,67,111,114,101,80,101,114,99,101,112,116,105,111,110,65,117,116,111,109,97,116,105,111,110,0]) [CLSID_CorePerceptionAutomation]);
DEFINE_IID!(IID_ICorePerceptionAutomationStatics, 196101441, 19682, 18723, 154, 118, 129, 135, 236, 197, 145, 18);
RT_INTERFACE!{static interface ICorePerceptionAutomationStatics(ICorePerceptionAutomationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ICorePerceptionAutomationStatics] {
    fn SetActivationFactoryProvider(&self, provider: *mut foundation::IGetActivationFactory) -> HRESULT
}}
impl ComPtr<ICorePerceptionAutomationStatics> {
    #[inline] pub fn set_activation_factory_provider(&self, provider: &ComPtr<foundation::IGetActivationFactory>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetActivationFactoryProvider)(self.as_abi() as *const _ as *mut _, provider.as_abi() as *const _ as *mut _);
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
impl ComPtr<IHeadPose> {
    #[inline] pub fn get_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Position)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_forward_direction(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ForwardDirection)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_up_direction(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_UpDirection)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HeadPose: IHeadPose}
} // Windows.Perception.People
pub mod spatial { // Windows.Perception.Spatial
use crate::prelude::*;
DEFINE_IID!(IID_ISpatialAnchor, 86631886, 7476, 14082, 188, 236, 234, 191, 245, 120, 168, 105);
RT_INTERFACE!{interface ISpatialAnchor(ISpatialAnchorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor] {
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn get_RawCoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn add_RawCoordinateSystemAdjusted(&self, handler: *mut foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_RawCoordinateSystemAdjusted(&self, cookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<ISpatialAnchor> {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CoordinateSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_coordinate_system(&self) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_RawCoordinateSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_raw_coordinate_system_adjusted(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialAnchor, SpatialAnchorRawCoordinateSystemAdjustedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_RawCoordinateSystemAdjusted)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_raw_coordinate_system_adjusted(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_RawCoordinateSystemAdjusted)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchor: ISpatialAnchor}
impl RtActivatable<ISpatialAnchorStatics> for SpatialAnchor {}
impl SpatialAnchor {
    #[inline] pub fn try_create_relative_to(coordinateSystem: &ComPtr<SpatialCoordinateSystem>) -> Result<Option<ComPtr<SpatialAnchor>>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_relative_to(coordinateSystem)
    }
    #[inline] pub fn try_create_with_position_relative_to(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, position: foundation::numerics::Vector3) -> Result<Option<ComPtr<SpatialAnchor>>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_relative_to(coordinateSystem, position)
    }
    #[inline] pub fn try_create_with_position_and_orientation_relative_to(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<SpatialAnchor>>> {
        <Self as RtActivatable<ISpatialAnchorStatics>>::get_activation_factory().try_create_with_position_and_orientation_relative_to(coordinateSystem, position, orientation)
    }
}
DEFINE_CLSID!(SpatialAnchor(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,0]) [CLSID_SpatialAnchor]);
DEFINE_IID!(IID_ISpatialAnchor2, 3977758984, 42645, 19702, 146, 253, 151, 38, 59, 167, 16, 71);
RT_INTERFACE!{interface ISpatialAnchor2(ISpatialAnchor2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchor2] {
    fn get_RemovedByUser(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<ISpatialAnchor2> {
    #[inline] pub fn get_removed_by_user(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RemovedByUser)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorExporter, 2586460984, 9467, 17001, 137, 197, 136, 48, 74, 238, 242, 15);
RT_INTERFACE!{interface ISpatialAnchorExporter(ISpatialAnchorExporterVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorExporter] {
    fn GetAnchorExportSufficiencyAsync(&self, anchor: *mut SpatialAnchor, purpose: SpatialAnchorExportPurpose, out: *mut *mut foundation::IAsyncOperation<SpatialAnchorExportSufficiency>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn TryExportAnchorAsync(&self, anchor: *mut SpatialAnchor, purpose: SpatialAnchorExportPurpose, stream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl ComPtr<ISpatialAnchorExporter> {
    #[inline] pub fn get_anchor_export_sufficiency_async(&self, anchor: &ComPtr<SpatialAnchor>, purpose: SpatialAnchorExportPurpose) -> Result<ComPtr<foundation::IAsyncOperation<SpatialAnchorExportSufficiency>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAnchorExportSufficiencyAsync)(self.as_abi() as *const _ as *mut _, anchor.as_abi() as *const _ as *mut _, purpose, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchor_async(&self, anchor: &ComPtr<SpatialAnchor>, purpose: SpatialAnchorExportPurpose, stream: &ComPtr<super::super::storage::streams::IOutputStream>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryExportAnchorAsync)(self.as_abi() as *const _ as *mut _, anchor.as_abi() as *const _ as *mut _, purpose, stream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorExporter: ISpatialAnchorExporter}
impl RtActivatable<ISpatialAnchorExporterStatics> for SpatialAnchorExporter {}
impl SpatialAnchorExporter {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<SpatialAnchorExporter>>> {
        <Self as RtActivatable<ISpatialAnchorExporterStatics>>::get_activation_factory().get_default()
    }
    #[inline] pub fn request_access_async() -> Result<ComPtr<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> {
        <Self as RtActivatable<ISpatialAnchorExporterStatics>>::get_activation_factory().request_access_async()
    }
}
DEFINE_CLSID!(SpatialAnchorExporter(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,69,120,112,111,114,116,101,114,0]) [CLSID_SpatialAnchorExporter]);
DEFINE_IID!(IID_ISpatialAnchorExporterStatics, 3978627000, 9333, 17308, 133, 255, 127, 237, 52, 31, 220, 136);
RT_INTERFACE!{static interface ISpatialAnchorExporterStatics(ISpatialAnchorExporterStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorExporterStatics] {
    fn GetDefault(&self, out: *mut *mut SpatialAnchorExporter) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut foundation::IAsyncOperation<SpatialPerceptionAccessStatus>) -> HRESULT
}}
impl ComPtr<ISpatialAnchorExporterStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<SpatialAnchorExporter>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestAccessAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
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
impl ComPtr<ISpatialAnchorExportSufficiency> {
    #[inline] pub fn get_is_minimally_sufficient(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsMinimallySufficient)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sufficiency_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SufficiencyLevel)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_recommended_sufficiency_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RecommendedSufficiencyLevel)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorExportSufficiency: ISpatialAnchorExportSufficiency}
RT_CLASS!{static class SpatialAnchorManager}
impl RtActivatable<ISpatialAnchorManagerStatics> for SpatialAnchorManager {}
impl SpatialAnchorManager {
    #[inline] pub fn request_store_async() -> Result<ComPtr<foundation::IAsyncOperation<SpatialAnchorStore>>> {
        <Self as RtActivatable<ISpatialAnchorManagerStatics>>::get_activation_factory().request_store_async()
    }
}
DEFINE_CLSID!(SpatialAnchorManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorManager]);
DEFINE_IID!(IID_ISpatialAnchorManagerStatics, 2296581803, 62391, 16907, 176, 134, 138, 128, 192, 125, 145, 13);
RT_INTERFACE!{static interface ISpatialAnchorManagerStatics(ISpatialAnchorManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorManagerStatics] {
    fn RequestStoreAsync(&self, out: *mut *mut foundation::IAsyncOperation<SpatialAnchorStore>) -> HRESULT
}}
impl ComPtr<ISpatialAnchorManagerStatics> {
    #[inline] pub fn request_store_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<SpatialAnchorStore>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestStoreAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs, 2716343992, 22215, 12567, 162, 228, 129, 224, 252, 242, 142, 0);
RT_INTERFACE!{interface ISpatialAnchorRawCoordinateSystemAdjustedEventArgs(ISpatialAnchorRawCoordinateSystemAdjustedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorRawCoordinateSystemAdjustedEventArgs] {
    fn get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform(&self, out: *mut foundation::numerics::Matrix4x4) -> HRESULT
}}
impl ComPtr<ISpatialAnchorRawCoordinateSystemAdjustedEventArgs> {
    #[inline] pub fn get_old_raw_coordinate_system_to_new_raw_coordinate_system_transform(&self) -> Result<foundation::numerics::Matrix4x4> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_OldRawCoordinateSystemToNewRawCoordinateSystemTransform)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorRawCoordinateSystemAdjustedEventArgs: ISpatialAnchorRawCoordinateSystemAdjustedEventArgs}
DEFINE_IID!(IID_ISpatialAnchorStatics, 2844952130, 372, 12572, 174, 121, 14, 81, 7, 102, 159, 22);
RT_INTERFACE!{static interface ISpatialAnchorStatics(ISpatialAnchorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStatics] {
    fn TryCreateRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn TryCreateWithPositionRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, position: foundation::numerics::Vector3, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn TryCreateWithPositionAndOrientationRelativeTo(&self, coordinateSystem: *mut SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion, out: *mut *mut SpatialAnchor) -> HRESULT
}}
impl ComPtr<ISpatialAnchorStatics> {
    #[inline] pub fn try_create_relative_to(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>) -> Result<Option<ComPtr<SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryCreateRelativeTo)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_with_position_relative_to(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, position: foundation::numerics::Vector3) -> Result<Option<ComPtr<SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryCreateWithPositionRelativeTo)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, position, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_create_with_position_and_orientation_relative_to(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryCreateWithPositionAndOrientationRelativeTo)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, position, orientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialAnchorStore, 2965124662, 18538, 15536, 158, 111, 18, 69, 22, 92, 77, 182);
RT_INTERFACE!{interface ISpatialAnchorStore(ISpatialAnchorStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorStore] {
    fn GetAllSavedAnchors(&self, out: *mut *mut foundation::collections::IMapView<HString, SpatialAnchor>) -> HRESULT,
    fn TrySave(&self, id: HSTRING, anchor: *mut SpatialAnchor, out: *mut bool) -> HRESULT,
    fn Remove(&self, id: HSTRING) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ComPtr<ISpatialAnchorStore> {
    #[inline] pub fn get_all_saved_anchors(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, SpatialAnchor>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAllSavedAnchors)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_save(&self, id: &HStringArg, anchor: &ComPtr<SpatialAnchor>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).TrySave)(self.as_abi() as *const _ as *mut _, id.get(), anchor.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove(&self, id: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Remove)(self.as_abi() as *const _ as *mut _, id.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Clear)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialAnchorStore: ISpatialAnchorStore}
RT_CLASS!{static class SpatialAnchorTransferManager}
impl RtActivatable<ISpatialAnchorTransferManagerStatics> for SpatialAnchorTransferManager {}
impl SpatialAnchorTransferManager {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_import_anchors_async(stream: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>>>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_import_anchors_async(stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchors_async(anchors: &ComPtr<foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>>>, stream: &ComPtr<super::super::storage::streams::IOutputStream>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().try_export_anchors_async(anchors, stream)
    }
    #[inline] pub fn request_access_async() -> Result<ComPtr<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> {
        <Self as RtActivatable<ISpatialAnchorTransferManagerStatics>>::get_activation_factory().request_access_async()
    }
}
DEFINE_CLSID!(SpatialAnchorTransferManager(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,65,110,99,104,111,114,84,114,97,110,115,102,101,114,77,97,110,97,103,101,114,0]) [CLSID_SpatialAnchorTransferManager]);
DEFINE_IID!(IID_ISpatialAnchorTransferManagerStatics, 62650809, 4824, 19406, 136, 53, 197, 223, 58, 192, 173, 171);
RT_INTERFACE!{static interface ISpatialAnchorTransferManagerStatics(ISpatialAnchorTransferManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialAnchorTransferManagerStatics] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryImportAnchorsAsync(&self, stream: *mut super::super::storage::streams::IInputStream, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn TryExportAnchorsAsync(&self, anchors: *mut foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>>, stream: *mut super::super::storage::streams::IOutputStream, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestAccessAsync(&self, out: *mut *mut foundation::IAsyncOperation<SpatialPerceptionAccessStatus>) -> HRESULT
}}
impl ComPtr<ISpatialAnchorTransferManagerStatics> {
    #[cfg(feature="windows-storage")] #[inline] pub fn try_import_anchors_async(&self, stream: &ComPtr<super::super::storage::streams::IInputStream>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IMapView<HString, SpatialAnchor>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryImportAnchorsAsync)(self.as_abi() as *const _ as *mut _, stream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn try_export_anchors_async(&self, anchors: &ComPtr<foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, SpatialAnchor>>>, stream: &ComPtr<super::super::storage::streams::IOutputStream>) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryExportAnchorsAsync)(self.as_abi() as *const _ as *mut _, anchors.as_abi() as *const _ as *mut _, stream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_access_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<SpatialPerceptionAccessStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestAccessAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
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
    #[inline] pub fn from_box(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, box_: SpatialBoundingBox) -> Result<Option<ComPtr<SpatialBoundingVolume>>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_box(coordinateSystem, box_)
    }
    #[inline] pub fn from_oriented_box(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, box_: SpatialBoundingOrientedBox) -> Result<Option<ComPtr<SpatialBoundingVolume>>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_oriented_box(coordinateSystem, box_)
    }
    #[inline] pub fn from_sphere(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, sphere: SpatialBoundingSphere) -> Result<Option<ComPtr<SpatialBoundingVolume>>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_sphere(coordinateSystem, sphere)
    }
    #[inline] pub fn from_frustum(coordinateSystem: &ComPtr<SpatialCoordinateSystem>, frustum: SpatialBoundingFrustum) -> Result<Option<ComPtr<SpatialBoundingVolume>>> {
        <Self as RtActivatable<ISpatialBoundingVolumeStatics>>::get_activation_factory().from_frustum(coordinateSystem, frustum)
    }
}
DEFINE_CLSID!(SpatialBoundingVolume(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,66,111,117,110,100,105,110,103,86,111,108,117,109,101,0]) [CLSID_SpatialBoundingVolume]);
DEFINE_IID!(IID_ISpatialBoundingVolumeStatics, 92836119, 46049, 14040, 176, 23, 86, 97, 129, 165, 177, 150);
RT_INTERFACE!{static interface ISpatialBoundingVolumeStatics(ISpatialBoundingVolumeStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialBoundingVolumeStatics] {
    fn FromBox(&self, coordinateSystem: *mut SpatialCoordinateSystem, box_: SpatialBoundingBox, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromOrientedBox(&self, coordinateSystem: *mut SpatialCoordinateSystem, box_: SpatialBoundingOrientedBox, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromSphere(&self, coordinateSystem: *mut SpatialCoordinateSystem, sphere: SpatialBoundingSphere, out: *mut *mut SpatialBoundingVolume) -> HRESULT,
    fn FromFrustum(&self, coordinateSystem: *mut SpatialCoordinateSystem, frustum: SpatialBoundingFrustum, out: *mut *mut SpatialBoundingVolume) -> HRESULT
}}
impl ComPtr<ISpatialBoundingVolumeStatics> {
    #[inline] pub fn from_box(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, box_: SpatialBoundingBox) -> Result<Option<ComPtr<SpatialBoundingVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromBox)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_oriented_box(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, box_: SpatialBoundingOrientedBox) -> Result<Option<ComPtr<SpatialBoundingVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromOrientedBox)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, box_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_sphere(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, sphere: SpatialBoundingSphere) -> Result<Option<ComPtr<SpatialBoundingVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromSphere)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, sphere, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn from_frustum(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>, frustum: SpatialBoundingFrustum) -> Result<Option<ComPtr<SpatialBoundingVolume>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FromFrustum)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, frustum, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialCoordinateSystem, 1777060427, 24739, 13702, 166, 83, 89, 167, 189, 103, 109, 7);
RT_INTERFACE!{interface ISpatialCoordinateSystem(ISpatialCoordinateSystemVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialCoordinateSystem] {
    fn TryGetTransformTo(&self, target: *mut SpatialCoordinateSystem, out: *mut *mut foundation::IReference<foundation::numerics::Matrix4x4>) -> HRESULT
}}
impl ComPtr<ISpatialCoordinateSystem> {
    #[inline] pub fn try_get_transform_to(&self, target: &ComPtr<SpatialCoordinateSystem>) -> Result<Option<ComPtr<foundation::IReference<foundation::numerics::Matrix4x4>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryGetTransformTo)(self.as_abi() as *const _ as *mut _, target.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialCoordinateSystem: ISpatialCoordinateSystem}
DEFINE_IID!(IID_ISpatialEntity, 376301909, 57835, 17740, 186, 8, 230, 192, 102, 141, 220, 101);
RT_INTERFACE!{interface ISpatialEntity(ISpatialEntityVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntity] {
    fn get_Id(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Anchor(&self, out: *mut *mut SpatialAnchor) -> HRESULT,
    fn get_Properties(&self, out: *mut *mut foundation::collections::ValueSet) -> HRESULT
}}
impl ComPtr<ISpatialEntity> {
    #[inline] pub fn get_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_anchor(&self) -> Result<Option<ComPtr<SpatialAnchor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Anchor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<foundation::collections::ValueSet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Properties)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntity: ISpatialEntity}
impl RtActivatable<ISpatialEntityFactory> for SpatialEntity {}
impl SpatialEntity {
    #[inline] pub fn create_with_spatial_anchor(spatialAnchor: &ComPtr<SpatialAnchor>) -> Result<ComPtr<SpatialEntity>> {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor(spatialAnchor)
    }
    #[inline] pub fn create_with_spatial_anchor_and_properties(spatialAnchor: &ComPtr<SpatialAnchor>, propertySet: &ComPtr<foundation::collections::ValueSet>) -> Result<ComPtr<SpatialEntity>> {
        <Self as RtActivatable<ISpatialEntityFactory>>::get_activation_factory().create_with_spatial_anchor_and_properties(spatialAnchor, propertySet)
    }
}
DEFINE_CLSID!(SpatialEntity(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,0]) [CLSID_SpatialEntity]);
DEFINE_IID!(IID_ISpatialEntityAddedEventArgs, 2744644763, 5482, 18183, 172, 44, 211, 29, 87, 14, 211, 153);
RT_INTERFACE!{interface ISpatialEntityAddedEventArgs(ISpatialEntityAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityAddedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ComPtr<ISpatialEntityAddedEventArgs> {
    #[inline] pub fn get_entity(&self) -> Result<Option<ComPtr<SpatialEntity>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Entity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityAddedEventArgs: ISpatialEntityAddedEventArgs}
DEFINE_IID!(IID_ISpatialEntityFactory, 3790725925, 13471, 16933, 162, 243, 75, 1, 193, 95, 224, 86);
RT_INTERFACE!{static interface ISpatialEntityFactory(ISpatialEntityFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityFactory] {
    fn CreateWithSpatialAnchor(&self, spatialAnchor: *mut SpatialAnchor, out: *mut *mut SpatialEntity) -> HRESULT,
    fn CreateWithSpatialAnchorAndProperties(&self, spatialAnchor: *mut SpatialAnchor, propertySet: *mut foundation::collections::ValueSet, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ComPtr<ISpatialEntityFactory> {
    #[inline] pub fn create_with_spatial_anchor(&self, spatialAnchor: &ComPtr<SpatialAnchor>) -> Result<ComPtr<SpatialEntity>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithSpatialAnchor)(self.as_abi() as *const _ as *mut _, spatialAnchor.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_spatial_anchor_and_properties(&self, spatialAnchor: &ComPtr<SpatialAnchor>, propertySet: &ComPtr<foundation::collections::ValueSet>) -> Result<ComPtr<SpatialEntity>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithSpatialAnchorAndProperties)(self.as_abi() as *const _ as *mut _, spatialAnchor.as_abi() as *const _ as *mut _, propertySet.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialEntityRemovedEventArgs, 2440304640, 21357, 20127, 171, 246, 65, 91, 84, 68, 214, 81);
RT_INTERFACE!{interface ISpatialEntityRemovedEventArgs(ISpatialEntityRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityRemovedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ComPtr<ISpatialEntityRemovedEventArgs> {
    #[inline] pub fn get_entity(&self) -> Result<Option<ComPtr<SpatialEntity>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Entity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityRemovedEventArgs: ISpatialEntityRemovedEventArgs}
DEFINE_IID!(IID_ISpatialEntityStore, 848791738, 58643, 20230, 136, 157, 27, 227, 14, 207, 67, 230);
RT_INTERFACE!{interface ISpatialEntityStore(ISpatialEntityStoreVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStore] {
    fn SaveAsync(&self, entity: *mut SpatialEntity, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn RemoveAsync(&self, entity: *mut SpatialEntity, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn CreateEntityWatcher(&self, out: *mut *mut SpatialEntityWatcher) -> HRESULT
}}
impl ComPtr<ISpatialEntityStore> {
    #[inline] pub fn save_async(&self, entity: &ComPtr<SpatialEntity>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SaveAsync)(self.as_abi() as *const _ as *mut _, entity.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_async(&self, entity: &ComPtr<SpatialEntity>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RemoveAsync)(self.as_abi() as *const _ as *mut _, entity.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_entity_watcher(&self) -> Result<Option<ComPtr<SpatialEntityWatcher>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateEntityWatcher)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityStore: ISpatialEntityStore}
impl RtActivatable<ISpatialEntityStoreStatics> for SpatialEntityStore {}
impl SpatialEntityStore {
    #[inline] pub fn get_is_supported() -> Result<bool> {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().get_is_supported()
    }
    #[cfg(feature="windows-system")] #[inline] pub fn try_get_for_remote_system_session(session: &ComPtr<super::super::system::remotesystems::RemoteSystemSession>) -> Result<Option<ComPtr<SpatialEntityStore>>> {
        <Self as RtActivatable<ISpatialEntityStoreStatics>>::get_activation_factory().try_get_for_remote_system_session(session)
    }
}
DEFINE_CLSID!(SpatialEntityStore(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,69,110,116,105,116,121,83,116,111,114,101,0]) [CLSID_SpatialEntityStore]);
DEFINE_IID!(IID_ISpatialEntityStoreStatics, 1800091806, 31824, 20114, 138, 98, 77, 29, 75, 124, 205, 62);
RT_INTERFACE!{static interface ISpatialEntityStoreStatics(ISpatialEntityStoreStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityStoreStatics] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    #[cfg(feature="windows-system")] fn TryGetForRemoteSystemSession(&self, session: *mut super::super::system::remotesystems::RemoteSystemSession, out: *mut *mut SpatialEntityStore) -> HRESULT
}}
impl ComPtr<ISpatialEntityStoreStatics> {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsSupported)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn try_get_for_remote_system_session(&self, session: &ComPtr<super::super::system::remotesystems::RemoteSystemSession>) -> Result<Option<ComPtr<SpatialEntityStore>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryGetForRemoteSystemSession)(self.as_abi() as *const _ as *mut _, session.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialEntityUpdatedEventArgs, 3848738662, 25211, 17355, 164, 159, 179, 190, 109, 71, 222, 237);
RT_INTERFACE!{interface ISpatialEntityUpdatedEventArgs(ISpatialEntityUpdatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityUpdatedEventArgs] {
    fn get_Entity(&self, out: *mut *mut SpatialEntity) -> HRESULT
}}
impl ComPtr<ISpatialEntityUpdatedEventArgs> {
    #[inline] pub fn get_entity(&self) -> Result<Option<ComPtr<SpatialEntity>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Entity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialEntityUpdatedEventArgs: ISpatialEntityUpdatedEventArgs}
DEFINE_IID!(IID_ISpatialEntityWatcher, 3015204768, 27998, 19388, 128, 93, 95, 229, 185, 186, 25, 89);
RT_INTERFACE!{interface ISpatialEntityWatcher(ISpatialEntityWatcherVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialEntityWatcher] {
    fn get_Status(&self, out: *mut SpatialEntityWatcherStatus) -> HRESULT,
    fn add_Added(&self, handler: *mut foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Added(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Updated(&self, handler: *mut foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Updated(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Removed(&self, handler: *mut foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Removed(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_EnumerationCompleted(&self, handler: *mut foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_EnumerationCompleted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn Start(&self) -> HRESULT,
    fn Stop(&self) -> HRESULT
}}
impl ComPtr<ISpatialEntityWatcher> {
    #[inline] pub fn get_status(&self) -> Result<SpatialEntityWatcherStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Status)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_added(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityAddedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Added)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_added(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Added)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_updated(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityUpdatedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Updated)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_updated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Updated)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_removed(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialEntityWatcher, SpatialEntityRemovedEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_Removed)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_removed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_Removed)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_enumeration_completed(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialEntityWatcher, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_EnumerationCompleted)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_enumeration_completed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_EnumerationCompleted)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Start)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Stop)(self.as_abi() as *const _ as *mut _);
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
impl ComPtr<ISpatialLocation> {
    #[inline] pub fn get_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Position)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Orientation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_linear_velocity(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteLinearVelocity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_linear_acceleration(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteLinearAcceleration)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_velocity(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteAngularVelocity)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_acceleration(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteAngularAcceleration)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocation: ISpatialLocation}
DEFINE_IID!(IID_ISpatialLocation2, 293544982, 14503, 18968, 180, 4, 171, 143, 171, 225, 215, 139);
RT_INTERFACE!{interface ISpatialLocation2(ISpatialLocation2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocation2] {
    fn get_AbsoluteAngularVelocityAxisAngle(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_AbsoluteAngularAccelerationAxisAngle(&self, out: *mut foundation::numerics::Vector3) -> HRESULT
}}
impl ComPtr<ISpatialLocation2> {
    #[inline] pub fn get_absolute_angular_velocity_axis_angle(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteAngularVelocityAxisAngle)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_absolute_angular_acceleration_axis_angle(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AbsoluteAngularAccelerationAxisAngle)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialLocator, 4131883301, 40460, 15286, 153, 126, 182, 78, 204, 162, 76, 244);
RT_INTERFACE!{interface ISpatialLocator(ISpatialLocatorVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocator] {
    fn get_Locatability(&self, out: *mut SpatialLocatability) -> HRESULT,
    fn add_LocatabilityChanged(&self, handler: *mut foundation::TypedEventHandler<SpatialLocator, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LocatabilityChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_PositionalTrackingDeactivating(&self, handler: *mut foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PositionalTrackingDeactivating(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn TryLocateAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, coordinateSystem: *mut SpatialCoordinateSystem, out: *mut *mut SpatialLocation) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeading(&self, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition(&self, relativePosition: foundation::numerics::Vector3, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut *mut SpatialLocatorAttachedFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocation(&self, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition(&self, relativePosition: foundation::numerics::Vector3, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT,
    fn CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64, out: *mut *mut SpatialStationaryFrameOfReference) -> HRESULT
}}
impl ComPtr<ISpatialLocator> {
    #[inline] pub fn get_locatability(&self) -> Result<SpatialLocatability> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Locatability)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_locatability_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialLocator, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_LocatabilityChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_locatability_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_LocatabilityChanged)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_positional_tracking_deactivating(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialLocator, SpatialLocatorPositionalTrackingDeactivatingEventArgs>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_PositionalTrackingDeactivating)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_positional_tracking_deactivating(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_PositionalTrackingDeactivating)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_locate_at_timestamp(&self, timestamp: &ComPtr<super::PerceptionTimestamp>, coordinateSystem: &ComPtr<SpatialCoordinateSystem>) -> Result<Option<ComPtr<SpatialLocation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryLocateAtTimestamp)(self.as_abi() as *const _ as *mut _, timestamp.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading(&self) -> Result<Option<ComPtr<SpatialLocatorAttachedFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeading)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position(&self, relativePosition: foundation::numerics::Vector3) -> Result<Option<ComPtr<SpatialLocatorAttachedFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPosition)(self.as_abi() as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<SpatialLocatorAttachedFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientation)(self.as_abi() as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attached_frame_of_reference_at_current_heading_with_position_and_orientation_and_relative_heading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<Option<ComPtr<SpatialLocatorAttachedFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttachedFrameOfReferenceAtCurrentHeadingWithPositionAndOrientationAndRelativeHeading)(self.as_abi() as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location(&self) -> Result<Option<ComPtr<SpatialStationaryFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position(&self, relativePosition: foundation::numerics::Vector3) -> Result<Option<ComPtr<SpatialStationaryFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPosition)(self.as_abi() as *const _ as *mut _, relativePosition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<SpatialStationaryFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientation)(self.as_abi() as *const _ as *mut _, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_stationary_frame_of_reference_at_current_location_with_position_and_orientation_and_relative_heading(&self, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, relativeHeadingInRadians: f64) -> Result<Option<ComPtr<SpatialStationaryFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateStationaryFrameOfReferenceAtCurrentLocationWithPositionAndOrientationAndRelativeHeading)(self.as_abi() as *const _ as *mut _, relativePosition, relativeOrientation, relativeHeadingInRadians, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocator: ISpatialLocator}
impl RtActivatable<ISpatialLocatorStatics> for SpatialLocator {}
impl SpatialLocator {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<SpatialLocator>>> {
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
    fn GetStationaryCoordinateSystemAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn TryGetRelativeHeadingAtTimestamp(&self, timestamp: *mut super::PerceptionTimestamp, out: *mut *mut foundation::IReference<f64>) -> HRESULT
}}
impl ComPtr<ISpatialLocatorAttachedFrameOfReference> {
    #[inline] pub fn get_relative_position(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RelativePosition)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_relative_position(&self, value: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_RelativePosition)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_relative_orientation(&self) -> Result<foundation::numerics::Quaternion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_RelativeOrientation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_relative_orientation(&self, value: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_RelativeOrientation)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn adjust_heading(&self, headingOffsetInRadians: f64) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).AdjustHeading)(self.as_abi() as *const _ as *mut _, headingOffsetInRadians);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stationary_coordinate_system_at_timestamp(&self, timestamp: &ComPtr<super::PerceptionTimestamp>) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStationaryCoordinateSystemAtTimestamp)(self.as_abi() as *const _ as *mut _, timestamp.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_get_relative_heading_at_timestamp(&self, timestamp: &ComPtr<super::PerceptionTimestamp>) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryGetRelativeHeadingAtTimestamp)(self.as_abi() as *const _ as *mut _, timestamp.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocatorAttachedFrameOfReference: ISpatialLocatorAttachedFrameOfReference}
DEFINE_IID!(IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs, 3098034275, 58356, 13963, 144, 97, 158, 169, 209, 214, 204, 22);
RT_INTERFACE!{interface ISpatialLocatorPositionalTrackingDeactivatingEventArgs(ISpatialLocatorPositionalTrackingDeactivatingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorPositionalTrackingDeactivatingEventArgs] {
    fn get_Canceled(&self, out: *mut bool) -> HRESULT,
    fn put_Canceled(&self, value: bool) -> HRESULT
}}
impl ComPtr<ISpatialLocatorPositionalTrackingDeactivatingEventArgs> {
    #[inline] pub fn get_canceled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Canceled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_canceled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Canceled)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialLocatorPositionalTrackingDeactivatingEventArgs: ISpatialLocatorPositionalTrackingDeactivatingEventArgs}
DEFINE_IID!(IID_ISpatialLocatorStatics, 3077452608, 42946, 13851, 187, 130, 86, 233, 59, 137, 177, 187);
RT_INTERFACE!{static interface ISpatialLocatorStatics(ISpatialLocatorStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialLocatorStatics] {
    fn GetDefault(&self, out: *mut *mut SpatialLocator) -> HRESULT
}}
impl ComPtr<ISpatialLocatorStatics> {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<SpatialLocator>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetDefault)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
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
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn get_MovementRange(&self, out: *mut SpatialMovementRange) -> HRESULT,
    fn get_LookDirectionRange(&self, out: *mut SpatialLookDirectionRange) -> HRESULT,
    fn GetCoordinateSystemAtCurrentLocation(&self, locator: *mut SpatialLocator, out: *mut *mut SpatialCoordinateSystem) -> HRESULT,
    fn TryGetMovementBounds(&self, coordinateSystem: *mut SpatialCoordinateSystem, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector3) -> HRESULT
}}
impl ComPtr<ISpatialStageFrameOfReference> {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CoordinateSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_movement_range(&self) -> Result<SpatialMovementRange> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MovementRange)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_look_direction_range(&self) -> Result<SpatialLookDirectionRange> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LookDirectionRange)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_coordinate_system_at_current_location(&self, locator: &ComPtr<SpatialLocator>) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCoordinateSystemAtCurrentLocation)(self.as_abi() as *const _ as *mut _, locator.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_get_movement_bounds(&self, coordinateSystem: &ComPtr<SpatialCoordinateSystem>) -> Result<ComArray<foundation::numerics::Vector3>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryGetMovementBounds)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialStageFrameOfReference: ISpatialStageFrameOfReference}
impl RtActivatable<ISpatialStageFrameOfReferenceStatics> for SpatialStageFrameOfReference {}
impl SpatialStageFrameOfReference {
    #[inline] pub fn get_current() -> Result<Option<ComPtr<SpatialStageFrameOfReference>>> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().get_current()
    }
    #[inline] pub fn add_current_changed(handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().add_current_changed(handler)
    }
    #[inline] pub fn remove_current_changed(cookie: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().remove_current_changed(cookie)
    }
    #[inline] pub fn request_new_stage_async() -> Result<ComPtr<foundation::IAsyncOperation<SpatialStageFrameOfReference>>> {
        <Self as RtActivatable<ISpatialStageFrameOfReferenceStatics>>::get_activation_factory().request_new_stage_async()
    }
}
DEFINE_CLSID!(SpatialStageFrameOfReference(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,112,97,116,105,97,108,83,116,97,103,101,70,114,97,109,101,79,102,82,101,102,101,114,101,110,99,101,0]) [CLSID_SpatialStageFrameOfReference]);
DEFINE_IID!(IID_ISpatialStageFrameOfReferenceStatics, 4153236557, 41124, 18844, 141, 145, 168, 201, 101, 212, 6, 84);
RT_INTERFACE!{static interface ISpatialStageFrameOfReferenceStatics(ISpatialStageFrameOfReferenceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStageFrameOfReferenceStatics] {
    fn get_Current(&self, out: *mut *mut SpatialStageFrameOfReference) -> HRESULT,
    fn add_CurrentChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CurrentChanged(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn RequestNewStageAsync(&self, out: *mut *mut foundation::IAsyncOperation<SpatialStageFrameOfReference>) -> HRESULT
}}
impl ComPtr<ISpatialStageFrameOfReferenceStatics> {
    #[inline] pub fn get_current(&self) -> Result<Option<ComPtr<SpatialStageFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Current)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_current_changed(&self, handler: &ComPtr<foundation::EventHandler<IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_CurrentChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_current_changed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_CurrentChanged)(self.as_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn request_new_stage_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<SpatialStageFrameOfReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestNewStageAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialStationaryFrameOfReference, 165399737, 48376, 15999, 190, 126, 126, 220, 203, 177, 120, 168);
RT_INTERFACE!{interface ISpatialStationaryFrameOfReference(ISpatialStationaryFrameOfReferenceVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialStationaryFrameOfReference] {
    fn get_CoordinateSystem(&self, out: *mut *mut SpatialCoordinateSystem) -> HRESULT
}}
impl ComPtr<ISpatialStationaryFrameOfReference> {
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<ComPtr<SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CoordinateSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialStationaryFrameOfReference: ISpatialStationaryFrameOfReference}
pub mod preview { // Windows.Perception.Spatial.Preview
use crate::prelude::*;
RT_CLASS!{static class SpatialGraphInteropPreview}
impl RtActivatable<ISpatialGraphInteropPreviewStatics> for SpatialGraphInteropPreview {}
impl SpatialGraphInteropPreview {
    #[inline] pub fn create_coordinate_system_for_node(nodeId: Guid) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node(nodeId)
    }
    #[inline] pub fn create_coordinate_system_for_node_with_position(nodeId: Guid, relativePosition: foundation::numerics::Vector3) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node_with_position(nodeId, relativePosition)
    }
    #[inline] pub fn create_coordinate_system_for_node_with_position_and_orientation(nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_coordinate_system_for_node_with_position_and_orientation(nodeId, relativePosition, relativeOrientation)
    }
    #[inline] pub fn create_locator_for_node(nodeId: Guid) -> Result<Option<ComPtr<super::SpatialLocator>>> {
        <Self as RtActivatable<ISpatialGraphInteropPreviewStatics>>::get_activation_factory().create_locator_for_node(nodeId)
    }
}
DEFINE_CLSID!(SpatialGraphInteropPreview(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,80,114,101,118,105,101,119,46,83,112,97,116,105,97,108,71,114,97,112,104,73,110,116,101,114,111,112,80,114,101,118,105,101,119,0]) [CLSID_SpatialGraphInteropPreview]);
DEFINE_IID!(IID_ISpatialGraphInteropPreviewStatics, 3225576524, 8408, 20176, 174, 247, 104, 5, 184, 229, 63, 85);
RT_INTERFACE!{static interface ISpatialGraphInteropPreviewStatics(ISpatialGraphInteropPreviewStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialGraphInteropPreviewStatics] {
    fn CreateCoordinateSystemForNode(&self, nodeId: Guid, out: *mut *mut super::SpatialCoordinateSystem) -> HRESULT,
    fn CreateCoordinateSystemForNodeWithPosition(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, out: *mut *mut super::SpatialCoordinateSystem) -> HRESULT,
    fn CreateCoordinateSystemForNodeWithPositionAndOrientation(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion, out: *mut *mut super::SpatialCoordinateSystem) -> HRESULT,
    fn CreateLocatorForNode(&self, nodeId: Guid, out: *mut *mut super::SpatialLocator) -> HRESULT
}}
impl ComPtr<ISpatialGraphInteropPreviewStatics> {
    #[inline] pub fn create_coordinate_system_for_node(&self, nodeId: Guid) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateCoordinateSystemForNode)(self.as_abi() as *const _ as *mut _, nodeId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_coordinate_system_for_node_with_position(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateCoordinateSystemForNodeWithPosition)(self.as_abi() as *const _ as *mut _, nodeId, relativePosition, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_coordinate_system_for_node_with_position_and_orientation(&self, nodeId: Guid, relativePosition: foundation::numerics::Vector3, relativeOrientation: foundation::numerics::Quaternion) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateCoordinateSystemForNodeWithPositionAndOrientation)(self.as_abi() as *const _ as *mut _, nodeId, relativePosition, relativeOrientation, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_locator_for_node(&self, nodeId: Guid) -> Result<Option<ComPtr<super::SpatialLocator>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateLocatorForNode)(self.as_abi() as *const _ as *mut _, nodeId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
} // Windows.Perception.Spatial.Preview
pub mod surfaces { // Windows.Perception.Spatial.Surfaces
use crate::prelude::*;
DEFINE_IID!(IID_ISpatialSurfaceInfo, 4176079847, 14775, 14690, 187, 3, 87, 245, 110, 31, 176, 161);
RT_INTERFACE!{interface ISpatialSurfaceInfo(ISpatialSurfaceInfoVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceInfo] {
    fn get_Id(&self, out: *mut Guid) -> HRESULT,
    fn get_UpdateTime(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn TryGetBounds(&self, coordinateSystem: *mut super::SpatialCoordinateSystem, out: *mut *mut foundation::IReference<super::SpatialBoundingOrientedBox>) -> HRESULT,
    fn TryComputeLatestMeshAsync(&self, maxTrianglesPerCubicMeter: f64, out: *mut *mut foundation::IAsyncOperation<SpatialSurfaceMesh>) -> HRESULT,
    fn TryComputeLatestMeshWithOptionsAsync(&self, maxTrianglesPerCubicMeter: f64, options: *mut SpatialSurfaceMeshOptions, out: *mut *mut foundation::IAsyncOperation<SpatialSurfaceMesh>) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceInfo> {
    #[inline] pub fn get_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Id)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_update_time(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_UpdateTime)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn try_get_bounds(&self, coordinateSystem: &ComPtr<super::SpatialCoordinateSystem>) -> Result<Option<ComPtr<foundation::IReference<super::SpatialBoundingOrientedBox>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryGetBounds)(self.as_abi() as *const _ as *mut _, coordinateSystem.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_compute_latest_mesh_async(&self, maxTrianglesPerCubicMeter: f64) -> Result<ComPtr<foundation::IAsyncOperation<SpatialSurfaceMesh>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryComputeLatestMeshAsync)(self.as_abi() as *const _ as *mut _, maxTrianglesPerCubicMeter, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_compute_latest_mesh_with_options_async(&self, maxTrianglesPerCubicMeter: f64, options: &ComPtr<SpatialSurfaceMeshOptions>) -> Result<ComPtr<foundation::IAsyncOperation<SpatialSurfaceMesh>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TryComputeLatestMeshWithOptionsAsync)(self.as_abi() as *const _ as *mut _, maxTrianglesPerCubicMeter, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceInfo: ISpatialSurfaceInfo}
DEFINE_IID!(IID_ISpatialSurfaceMesh, 277829593, 57101, 14672, 160, 253, 249, 114, 199, 124, 39, 180);
RT_INTERFACE!{interface ISpatialSurfaceMesh(ISpatialSurfaceMeshVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMesh] {
    fn get_SurfaceInfo(&self, out: *mut *mut SpatialSurfaceInfo) -> HRESULT,
    fn get_CoordinateSystem(&self, out: *mut *mut super::SpatialCoordinateSystem) -> HRESULT,
    fn get_TriangleIndices(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT,
    fn get_VertexPositions(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT,
    fn get_VertexPositionScale(&self, out: *mut foundation::numerics::Vector3) -> HRESULT,
    fn get_VertexNormals(&self, out: *mut *mut SpatialSurfaceMeshBuffer) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceMesh> {
    #[inline] pub fn get_surface_info(&self) -> Result<Option<ComPtr<SpatialSurfaceInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SurfaceInfo)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_coordinate_system(&self) -> Result<Option<ComPtr<super::SpatialCoordinateSystem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CoordinateSystem)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_indices(&self) -> Result<Option<ComPtr<SpatialSurfaceMeshBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_TriangleIndices)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_positions(&self) -> Result<Option<ComPtr<SpatialSurfaceMeshBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_VertexPositions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_position_scale(&self) -> Result<foundation::numerics::Vector3> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_VertexPositionScale)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_normals(&self) -> Result<Option<ComPtr<SpatialSurfaceMeshBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_VertexNormals)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceMesh: ISpatialSurfaceMesh}
DEFINE_IID!(IID_ISpatialSurfaceMeshBuffer, 2479839712, 34591, 13304, 152, 178, 3, 209, 1, 69, 143, 111);
RT_INTERFACE!{interface ISpatialSurfaceMeshBuffer(ISpatialSurfaceMeshBufferVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshBuffer] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_Format(&self, out: *mut crate::windows::graphics::directx::DirectXPixelFormat) -> HRESULT,
    fn get_Stride(&self, out: *mut u32) -> HRESULT,
    fn get_ElementCount(&self, out: *mut u32) -> HRESULT,
    #[cfg(feature="windows-storage")] fn get_Data(&self, out: *mut *mut crate::windows::storage::streams::IBuffer) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceMeshBuffer> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Format)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_stride(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Stride)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_element_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ElementCount)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_data(&self) -> Result<Option<ComPtr<crate::windows::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Data)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
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
impl ComPtr<ISpatialSurfaceMeshOptions> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_vertex_position_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_VertexPositionFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_vertex_position_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_VertexPositionFormat)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_triangle_index_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TriangleIndexFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_triangle_index_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_TriangleIndexFormat)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_vertex_normal_format(&self) -> Result<crate::windows::graphics::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_VertexNormalFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn set_vertex_normal_format(&self, value: crate::windows::graphics::directx::DirectXPixelFormat) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_VertexNormalFormat)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_include_vertex_normals(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IncludeVertexNormals)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_include_vertex_normals(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_IncludeVertexNormals)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceMeshOptions: ISpatialSurfaceMeshOptions}
impl RtActivatable<ISpatialSurfaceMeshOptionsStatics> for SpatialSurfaceMeshOptions {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceMeshOptions {}
impl SpatialSurfaceMeshOptions {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_position_formats() -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_position_formats()
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_triangle_index_formats() -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_triangle_index_formats()
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_normal_formats() -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> {
        <Self as RtActivatable<ISpatialSurfaceMeshOptionsStatics>>::get_activation_factory().get_supported_vertex_normal_formats()
    }
}
DEFINE_CLSID!(SpatialSurfaceMeshOptions(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,77,101,115,104,79,112,116,105,111,110,115,0]) [CLSID_SpatialSurfaceMeshOptions]);
DEFINE_IID!(IID_ISpatialSurfaceMeshOptionsStatics, 2603879103, 38785, 17669, 137, 53, 1, 53, 117, 202, 174, 94);
RT_INTERFACE!{static interface ISpatialSurfaceMeshOptionsStatics(ISpatialSurfaceMeshOptionsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceMeshOptionsStatics] {
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexPositionFormats(&self, out: *mut *mut foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedTriangleIndexFormats(&self, out: *mut *mut foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_SupportedVertexNormalFormats(&self, out: *mut *mut foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceMeshOptionsStatics> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_position_formats(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SupportedVertexPositionFormats)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_triangle_index_formats(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SupportedTriangleIndexFormats)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_supported_vertex_normal_formats(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<crate::windows::graphics::directx::DirectXPixelFormat>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SupportedVertexNormalFormats)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialSurfaceObserver, 280401945, 56778, 13443, 172, 58, 116, 143, 232, 200, 109, 245);
RT_INTERFACE!{interface ISpatialSurfaceObserver(ISpatialSurfaceObserverVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserver] {
    fn GetObservedSurfaces(&self, out: *mut *mut foundation::collections::IMapView<Guid, SpatialSurfaceInfo>) -> HRESULT,
    fn SetBoundingVolume(&self, bounds: *mut super::SpatialBoundingVolume) -> HRESULT,
    fn SetBoundingVolumes(&self, bounds: *mut foundation::collections::IIterable<super::SpatialBoundingVolume>) -> HRESULT,
    fn add_ObservedSurfacesChanged(&self, handler: *mut foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ObservedSurfacesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceObserver> {
    #[inline] pub fn get_observed_surfaces(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<Guid, SpatialSurfaceInfo>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetObservedSurfaces)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_bounding_volume(&self, bounds: &ComPtr<super::SpatialBoundingVolume>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetBoundingVolume)(self.as_abi() as *const _ as *mut _, bounds.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_bounding_volumes(&self, bounds: &ComPtr<foundation::collections::IIterable<super::SpatialBoundingVolume>>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetBoundingVolumes)(self.as_abi() as *const _ as *mut _, bounds.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_observed_surfaces_changed(&self, handler: &ComPtr<foundation::TypedEventHandler<SpatialSurfaceObserver, IInspectable>>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).add_ObservedSurfacesChanged)(self.as_abi() as *const _ as *mut _, handler.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_observed_surfaces_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).remove_ObservedSurfacesChanged)(self.as_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SpatialSurfaceObserver: ISpatialSurfaceObserver}
impl RtActivatable<ISpatialSurfaceObserverStatics> for SpatialSurfaceObserver {}
impl RtActivatable<ISpatialSurfaceObserverStatics2> for SpatialSurfaceObserver {}
impl RtActivatable<IActivationFactory> for SpatialSurfaceObserver {}
impl SpatialSurfaceObserver {
    #[inline] pub fn request_access_async() -> Result<ComPtr<foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>> {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics>>::get_activation_factory().request_access_async()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<ISpatialSurfaceObserverStatics2>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(SpatialSurfaceObserver(&[87,105,110,100,111,119,115,46,80,101,114,99,101,112,116,105,111,110,46,83,112,97,116,105,97,108,46,83,117,114,102,97,99,101,115,46,83,112,97,116,105,97,108,83,117,114,102,97,99,101,79,98,115,101,114,118,101,114,0]) [CLSID_SpatialSurfaceObserver]);
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics, 374952429, 8456, 16744, 145, 117, 135, 224, 39, 188, 146, 133);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics(ISpatialSurfaceObserverStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics] {
    fn RequestAccessAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceObserverStatics> {
    #[inline] pub fn request_access_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::SpatialPerceptionAccessStatus>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RequestAccessAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISpatialSurfaceObserverStatics2, 257114721, 50525, 20075, 168, 149, 161, 157, 230, 154, 66, 227);
RT_INTERFACE!{static interface ISpatialSurfaceObserverStatics2(ISpatialSurfaceObserverStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_ISpatialSurfaceObserverStatics2] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<ISpatialSurfaceObserverStatics2> {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsSupported)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Perception.Spatial.Surfaces
} // Windows.Perception.Spatial
