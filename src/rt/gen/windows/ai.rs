pub mod machinelearning { // Windows.AI.MachineLearning
pub mod preview { // Windows.AI.MachineLearning.Preview
use ::prelude::*;
RT_ENUM! { enum FeatureElementKindPreview: i32 {
    Undefined (FeatureElementKindPreview_Undefined) = 0, Float (FeatureElementKindPreview_Float) = 1, UInt8 (FeatureElementKindPreview_UInt8) = 2, Int8 (FeatureElementKindPreview_Int8) = 3, UInt16 (FeatureElementKindPreview_UInt16) = 4, Int16 (FeatureElementKindPreview_Int16) = 5, Int32 (FeatureElementKindPreview_Int32) = 6, Int64 (FeatureElementKindPreview_Int64) = 7, String (FeatureElementKindPreview_String) = 8, Boolean (FeatureElementKindPreview_Boolean) = 9, Float16 (FeatureElementKindPreview_Float16) = 10, Double (FeatureElementKindPreview_Double) = 11, UInt32 (FeatureElementKindPreview_UInt32) = 12, UInt64 (FeatureElementKindPreview_UInt64) = 13, Complex64 (FeatureElementKindPreview_Complex64) = 14, Complex128 (FeatureElementKindPreview_Complex128) = 15,
}}
DEFINE_IID!(IID_IImageVariableDescriptorPreview, 2061630066, 670, 19909, 162, 248, 95, 183, 99, 21, 65, 80);
RT_INTERFACE!{interface IImageVariableDescriptorPreview(IImageVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_IImageVariableDescriptorPreview] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapPixelFormat(&self, out: *mut ::rt::gen::windows::graphics::imaging::BitmapPixelFormat) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT
}}
impl IImageVariableDescriptorPreview {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<::rt::gen::windows::graphics::imaging::BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapPixelFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Width)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Height)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageVariableDescriptorPreview: IImageVariableDescriptorPreview}
DEFINE_IID!(IID_IInferencingOptionsPreview, 1203536389, 19766, 18345, 143, 104, 255, 203, 51, 157, 208, 252);
RT_INTERFACE!{interface IInferencingOptionsPreview(IInferencingOptionsPreviewVtbl): IInspectable(IInspectableVtbl) [IID_IInferencingOptionsPreview] {
    fn get_PreferredDeviceKind(&self, out: *mut LearningModelDeviceKindPreview) -> HRESULT,
    fn put_PreferredDeviceKind(&self, value: LearningModelDeviceKindPreview) -> HRESULT,
    fn get_IsTracingEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsTracingEnabled(&self, value: bool) -> HRESULT,
    fn get_MaxBatchSize(&self, out: *mut i32) -> HRESULT,
    fn put_MaxBatchSize(&self, value: i32) -> HRESULT,
    fn get_MinimizeMemoryAllocation(&self, out: *mut bool) -> HRESULT,
    fn put_MinimizeMemoryAllocation(&self, value: bool) -> HRESULT,
    fn get_ReclaimMemoryAfterEvaluation(&self, out: *mut bool) -> HRESULT,
    fn put_ReclaimMemoryAfterEvaluation(&self, value: bool) -> HRESULT
}}
impl IInferencingOptionsPreview {
    #[inline] pub fn get_preferred_device_kind(&self) -> Result<LearningModelDeviceKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PreferredDeviceKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_preferred_device_kind(&self, value: LearningModelDeviceKindPreview) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PreferredDeviceKind)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_tracing_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsTracingEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_tracing_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsTracingEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_batch_size(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxBatchSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_batch_size(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxBatchSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_minimize_memory_allocation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinimizeMemoryAllocation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_minimize_memory_allocation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MinimizeMemoryAllocation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_reclaim_memory_after_evaluation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReclaimMemoryAfterEvaluation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_reclaim_memory_after_evaluation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ReclaimMemoryAfterEvaluation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class InferencingOptionsPreview: IInferencingOptionsPreview}
DEFINE_IID!(IID_ILearningModelBindingPreview, 2479423976, 27768, 19279, 174, 193, 166, 187, 158, 105, 22, 36);
RT_INTERFACE!{interface ILearningModelBindingPreview(ILearningModelBindingPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelBindingPreview] {
    fn Bind(&self, name: HSTRING, value: *mut IInspectable) -> HRESULT,
    fn BindWithProperties(&self, name: HSTRING, value: *mut IInspectable, metadata: *mut foundation::collections::IPropertySet) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ILearningModelBindingPreview {
    #[inline] pub fn bind(&self, name: &HStringArg, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Bind)(self as *const _ as *mut _, name.get(), value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn bind_with_properties(&self, name: &HStringArg, value: &IInspectable, metadata: &foundation::collections::IPropertySet) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).BindWithProperties)(self as *const _ as *mut _, name.get(), value as *const _ as *mut _, metadata as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Clear)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelBindingPreview: ILearningModelBindingPreview}
impl RtActivatable<ILearningModelBindingPreviewFactory> for LearningModelBindingPreview {}
impl LearningModelBindingPreview {
    #[inline] pub fn create_from_model(model: &LearningModelPreview) -> Result<ComPtr<LearningModelBindingPreview>> {
        <Self as RtActivatable<ILearningModelBindingPreviewFactory>>::get_activation_factory().create_from_model(model)
    }
}
DEFINE_CLSID!(LearningModelBindingPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,66,105,110,100,105,110,103,80,114,101,118,105,101,119,0]) [CLSID_LearningModelBindingPreview]);
DEFINE_IID!(IID_ILearningModelBindingPreviewFactory, 1220026783, 7761, 19831, 174, 80, 62, 193, 100, 173, 52, 128);
RT_INTERFACE!{static interface ILearningModelBindingPreviewFactory(ILearningModelBindingPreviewFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelBindingPreviewFactory] {
    fn CreateFromModel(&self, model: *mut LearningModelPreview, out: *mut *mut LearningModelBindingPreview) -> HRESULT
}}
impl ILearningModelBindingPreviewFactory {
    #[inline] pub fn create_from_model(&self, model: &LearningModelPreview) -> Result<ComPtr<LearningModelBindingPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromModel)(self as *const _ as *mut _, model as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelDescriptionPreview, 4113304006, 34321, 16557, 142, 89, 222, 63, 215, 3, 10, 64);
RT_INTERFACE!{interface ILearningModelDescriptionPreview(ILearningModelDescriptionPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelDescriptionPreview] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut i64) -> HRESULT,
    fn get_Metadata(&self, out: *mut *mut foundation::collections::IMapView<HString, HString>) -> HRESULT,
    fn get_InputFeatures(&self, out: *mut *mut foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>) -> HRESULT,
    fn get_OutputFeatures(&self, out: *mut *mut foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>) -> HRESULT
}}
impl ILearningModelDescriptionPreview {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Author)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Domain)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Metadata)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_features(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InputFeatures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_features(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OutputFeatures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelDescriptionPreview: ILearningModelDescriptionPreview}
RT_ENUM! { enum LearningModelDeviceKindPreview: i32 {
    LearningDeviceAny (LearningModelDeviceKindPreview_LearningDeviceAny) = 0, LearningDeviceCpu (LearningModelDeviceKindPreview_LearningDeviceCpu) = 1, LearningDeviceGpu (LearningModelDeviceKindPreview_LearningDeviceGpu) = 2, LearningDeviceNpu (LearningModelDeviceKindPreview_LearningDeviceNpu) = 3, LearningDeviceDsp (LearningModelDeviceKindPreview_LearningDeviceDsp) = 4, LearningDeviceFpga (LearningModelDeviceKindPreview_LearningDeviceFpga) = 5,
}}
DEFINE_IID!(IID_ILearningModelEvaluationResultPreview, 3743804063, 39011, 16520, 132, 152, 135, 161, 244, 104, 111, 146);
RT_INTERFACE!{interface ILearningModelEvaluationResultPreview(ILearningModelEvaluationResultPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelEvaluationResultPreview] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Outputs(&self, out: *mut *mut foundation::collections::IMapView<HString, IInspectable>) -> HRESULT
}}
impl ILearningModelEvaluationResultPreview {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CorrelationId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_outputs(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Outputs)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelEvaluationResultPreview: ILearningModelEvaluationResultPreview}
RT_ENUM! { enum LearningModelFeatureKindPreview: i32 {
    Undefined (LearningModelFeatureKindPreview_Undefined) = 0, Tensor (LearningModelFeatureKindPreview_Tensor) = 1, Sequence (LearningModelFeatureKindPreview_Sequence) = 2, Map (LearningModelFeatureKindPreview_Map) = 3, Image (LearningModelFeatureKindPreview_Image) = 4,
}}
DEFINE_IID!(IID_ILearningModelPreview, 77342314, 37812, 18316, 174, 184, 112, 21, 123, 240, 255, 148);
RT_INTERFACE!{interface ILearningModelPreview(ILearningModelPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelPreview] {
    fn EvaluateAsync(&self, binding: *mut LearningModelBindingPreview, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResultPreview>) -> HRESULT,
    fn EvaluateFeaturesAsync(&self, features: *mut foundation::collections::IMap<HString, IInspectable>, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResultPreview>) -> HRESULT,
    fn get_Description(&self, out: *mut *mut LearningModelDescriptionPreview) -> HRESULT,
    fn get_InferencingOptions(&self, out: *mut *mut InferencingOptionsPreview) -> HRESULT,
    fn put_InferencingOptions(&self, value: *mut InferencingOptionsPreview) -> HRESULT
}}
impl ILearningModelPreview {
    #[inline] pub fn evaluate_async(&self, binding: &LearningModelBindingPreview, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EvaluateAsync)(self as *const _ as *mut _, binding as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features_async(&self, features: &foundation::collections::IMap<HString, IInspectable>, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).EvaluateFeaturesAsync)(self as *const _ as *mut _, features as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<Option<ComPtr<LearningModelDescriptionPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inferencing_options(&self) -> Result<Option<ComPtr<InferencingOptionsPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InferencingOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_inferencing_options(&self, value: &InferencingOptionsPreview) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InferencingOptions)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelPreview: ILearningModelPreview}
impl RtActivatable<ILearningModelPreviewStatics> for LearningModelPreview {}
impl LearningModelPreview {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(modelFile: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_storage_file_async(modelFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(modelStream: &::rt::gen::windows::storage::streams::IRandomAccessStreamReference) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_stream_async(modelStream)
    }
}
DEFINE_CLSID!(LearningModelPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,80,114,101,118,105,101,119,0]) [CLSID_LearningModelPreview]);
DEFINE_IID!(IID_ILearningModelPreviewStatics, 374061920, 33893, 18310, 139, 147, 44, 22, 168, 146, 137, 215);
RT_INTERFACE!{static interface ILearningModelPreviewStatics(ILearningModelPreviewStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelPreviewStatics] {
    #[cfg(feature="windows-storage")] fn LoadModelFromStorageFileAsync(&self, modelFile: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<LearningModelPreview>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadModelFromStreamAsync(&self, modelStream: *mut ::rt::gen::windows::storage::streams::IRandomAccessStreamReference, out: *mut *mut foundation::IAsyncOperation<LearningModelPreview>) -> HRESULT
}}
impl ILearningModelPreviewStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(&self, modelFile: &::rt::gen::windows::storage::IStorageFile) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadModelFromStorageFileAsync)(self as *const _ as *mut _, modelFile as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(&self, modelStream: &::rt::gen::windows::storage::streams::IRandomAccessStreamReference) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadModelFromStreamAsync)(self as *const _ as *mut _, modelStream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelVariableDescriptorPreview, 2973628034, 64560, 18731, 142, 160, 237, 31, 83, 192, 176, 56);
RT_INTERFACE!{interface ILearningModelVariableDescriptorPreview(ILearningModelVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelVariableDescriptorPreview] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ModelFeatureKind(&self, out: *mut LearningModelFeatureKindPreview) -> HRESULT,
    fn get_IsRequired(&self, out: *mut bool) -> HRESULT
}}
impl ILearningModelVariableDescriptorPreview {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model_feature_kind(&self) -> Result<LearningModelFeatureKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ModelFeatureKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsRequired)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelVariableDescriptorPreview: ILearningModelVariableDescriptorPreview}
DEFINE_IID!(IID_IMapVariableDescriptorPreview, 1018397552, 49195, 16950, 179, 232, 107, 220, 164, 156, 49, 41);
RT_INTERFACE!{interface IMapVariableDescriptorPreview(IMapVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_IMapVariableDescriptorPreview] {
    fn get_KeyKind(&self, out: *mut FeatureElementKindPreview) -> HRESULT,
    fn get_ValidStringKeys(&self, out: *mut *mut foundation::collections::IIterable<HString>) -> HRESULT,
    fn get_ValidIntegerKeys(&self, out: *mut *mut foundation::collections::IIterable<i64>) -> HRESULT,
    fn get_Fields(&self, out: *mut *mut ILearningModelVariableDescriptorPreview) -> HRESULT
}}
impl IMapVariableDescriptorPreview {
    #[inline] pub fn get_key_kind(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_KeyKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_string_keys(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ValidStringKeys)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_integer_keys(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ValidIntegerKeys)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fields(&self) -> Result<Option<ComPtr<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Fields)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapVariableDescriptorPreview: IMapVariableDescriptorPreview}
DEFINE_IID!(IID_ISequenceVariableDescriptorPreview, 2631463570, 39090, 17712, 161, 182, 45, 237, 95, 236, 188, 38);
RT_INTERFACE!{interface ISequenceVariableDescriptorPreview(ISequenceVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ISequenceVariableDescriptorPreview] {
    fn get_ElementType(&self, out: *mut *mut ILearningModelVariableDescriptorPreview) -> HRESULT
}}
impl ISequenceVariableDescriptorPreview {
    #[inline] pub fn get_element_type(&self) -> Result<Option<ComPtr<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ElementType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SequenceVariableDescriptorPreview: ISequenceVariableDescriptorPreview}
DEFINE_IID!(IID_ITensorVariableDescriptorPreview, 2819575834, 39596, 16947, 151, 132, 172, 234, 249, 37, 16, 181);
RT_INTERFACE!{interface ITensorVariableDescriptorPreview(ITensorVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ITensorVariableDescriptorPreview] {
    fn get_DataType(&self, out: *mut FeatureElementKindPreview) -> HRESULT,
    fn get_Shape(&self, out: *mut *mut foundation::collections::IIterable<i64>) -> HRESULT
}}
impl ITensorVariableDescriptorPreview {
    #[inline] pub fn get_data_type(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Shape)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorVariableDescriptorPreview: ITensorVariableDescriptorPreview}
} // Windows.AI.MachineLearning.Preview
} // Windows.AI.MachineLearning
