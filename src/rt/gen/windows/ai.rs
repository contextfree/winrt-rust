pub mod machinelearning { // Windows.AI.MachineLearning
use crate::prelude::*;
DEFINE_IID!(IID_IImageFeatureDescriptor, 911574437, 5914, 18986, 152, 95, 38, 81, 89, 211, 137, 90);
RT_INTERFACE!{interface IImageFeatureDescriptor(IImageFeatureDescriptorVtbl): IInspectable(IInspectableVtbl) [IID_IImageFeatureDescriptor] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapPixelFormat(&self, out: *mut super::super::graphics::imaging::BitmapPixelFormat) -> HRESULT,
    #[cfg(not(feature="windows-graphics"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapAlphaMode(&self, out: *mut super::super::graphics::imaging::BitmapAlphaMode) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT
}}
impl ComPtr<IImageFeatureDescriptor> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<super::super::graphics::imaging::BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BitmapPixelFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<super::super::graphics::imaging::BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BitmapAlphaMode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Width)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Height)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageFeatureDescriptor: IImageFeatureDescriptor}
DEFINE_IID!(IID_IImageFeatureValue, 4030812121, 51626, 17413, 183, 251, 148, 248, 124, 138, 48, 55);
RT_INTERFACE!{interface IImageFeatureValue(IImageFeatureValueVtbl): IInspectable(IInspectableVtbl) [IID_IImageFeatureValue] {
    #[cfg(feature="windows-media")] fn get_VideoFrame(&self, out: *mut *mut super::super::media::VideoFrame) -> HRESULT
}}
impl ComPtr<IImageFeatureValue> {
    #[cfg(feature="windows-media")] #[inline] pub fn get_video_frame(&self) -> Result<Option<ComPtr<super::super::media::VideoFrame>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_VideoFrame)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class ImageFeatureValue: IImageFeatureValue}
impl RtActivatable<IImageFeatureValueStatics> for ImageFeatureValue {}
impl ImageFeatureValue {
    #[cfg(feature="windows-media")] #[inline] pub fn create_from_video_frame(image: &ComPtr<super::super::media::VideoFrame>) -> Result<Option<ComPtr<ImageFeatureValue>>> {
        <Self as RtActivatable<IImageFeatureValueStatics>>::get_activation_factory().create_from_video_frame(image)
    }
}
DEFINE_CLSID!(ImageFeatureValue(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,73,109,97,103,101,70,101,97,116,117,114,101,86,97,108,117,101,0]) [CLSID_ImageFeatureValue]);
DEFINE_IID!(IID_IImageFeatureValueStatics, 465770493, 9163, 17936, 176, 133, 200, 225, 200, 126, 186, 160);
RT_INTERFACE!{static interface IImageFeatureValueStatics(IImageFeatureValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IImageFeatureValueStatics] {
    #[cfg(feature="windows-media")] fn CreateFromVideoFrame(&self, image: *mut super::super::media::VideoFrame, out: *mut *mut ImageFeatureValue) -> HRESULT
}}
impl ComPtr<IImageFeatureValueStatics> {
    #[cfg(feature="windows-media")] #[inline] pub fn create_from_video_frame(&self, image: &ComPtr<super::super::media::VideoFrame>) -> Result<Option<ComPtr<ImageFeatureValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromVideoFrame)(self.as_abi() as *const _ as *mut _, image.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModel, 1536051488, 18591, 20102, 145, 40, 38, 90, 50, 123, 120, 250);
RT_INTERFACE!{interface ILearningModel(ILearningModelVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModel] {
    fn get_Author(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Domain(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Version(&self, out: *mut i64) -> HRESULT,
    fn get_Metadata(&self, out: *mut *mut foundation::collections::IMapView<HString, HString>) -> HRESULT,
    fn get_InputFeatures(&self, out: *mut *mut foundation::collections::IVectorView<ILearningModelFeatureDescriptor>) -> HRESULT,
    fn get_OutputFeatures(&self, out: *mut *mut foundation::collections::IVectorView<ILearningModelFeatureDescriptor>) -> HRESULT
}}
impl ComPtr<ILearningModel> {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Author)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Domain)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Version)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Metadata)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_features(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ILearningModelFeatureDescriptor>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InputFeatures)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_features(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<ILearningModelFeatureDescriptor>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_OutputFeatures)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModel: ILearningModel}
impl RtActivatable<ILearningModelStatics> for LearningModel {}
impl LearningModel {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_async(modelFile: &ComPtr<super::super::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_storage_file_async(modelFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_async(modelStream)
    }
    #[inline] pub fn load_from_file_path(filePath: &HStringArg) -> Result<Option<ComPtr<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_file_path(filePath)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream(modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>) -> Result<Option<ComPtr<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream(modelStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_with_operator_provider_async(modelFile: &ComPtr<super::super::storage::IStorageFile>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_storage_file_with_operator_provider_async(modelFile, operatorProvider)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider_async(modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_with_operator_provider_async(modelStream, operatorProvider)
    }
    #[inline] pub fn load_from_file_path_with_operator_provider(filePath: &HStringArg, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<Option<ComPtr<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_file_path_with_operator_provider(filePath, operatorProvider)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider(modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<Option<ComPtr<LearningModel>>> {
        <Self as RtActivatable<ILearningModelStatics>>::get_activation_factory().load_from_stream_with_operator_provider(modelStream, operatorProvider)
    }
}
DEFINE_CLSID!(LearningModel(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,0]) [CLSID_LearningModel]);
DEFINE_IID!(IID_ILearningModelBinding, 3929091872, 5775, 20364, 148, 254, 46, 122, 195, 27, 74, 168);
RT_INTERFACE!{interface ILearningModelBinding(ILearningModelBindingVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelBinding] {
    fn Bind(&self, name: HSTRING, value: *mut IInspectable) -> HRESULT,
    fn BindWithProperties(&self, name: HSTRING, value: *mut IInspectable, props: *mut foundation::collections::IPropertySet) -> HRESULT,
    fn Clear(&self) -> HRESULT
}}
impl ComPtr<ILearningModelBinding> {
    #[inline] pub fn bind(&self, name: &HStringArg, value: &ComPtr<IInspectable>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Bind)(self.as_abi() as *const _ as *mut _, name.get(), value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn bind_with_properties(&self, name: &HStringArg, value: &ComPtr<IInspectable>, props: &ComPtr<foundation::collections::IPropertySet>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).BindWithProperties)(self.as_abi() as *const _ as *mut _, name.get(), value.as_abi() as *const _ as *mut _, props.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Clear)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelBinding: ILearningModelBinding}
impl RtActivatable<ILearningModelBindingFactory> for LearningModelBinding {}
impl LearningModelBinding {
    #[inline] pub fn create_from_session(session: &ComPtr<LearningModelSession>) -> Result<ComPtr<LearningModelBinding>> {
        <Self as RtActivatable<ILearningModelBindingFactory>>::get_activation_factory().create_from_session(session)
    }
}
DEFINE_CLSID!(LearningModelBinding(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,66,105,110,100,105,110,103,0]) [CLSID_LearningModelBinding]);
DEFINE_IID!(IID_ILearningModelBindingFactory, 3378477690, 59272, 18270, 137, 23, 35, 170, 56, 31, 175, 11);
RT_INTERFACE!{static interface ILearningModelBindingFactory(ILearningModelBindingFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelBindingFactory] {
    fn CreateFromSession(&self, session: *mut LearningModelSession, out: *mut *mut LearningModelBinding) -> HRESULT
}}
impl ComPtr<ILearningModelBindingFactory> {
    #[inline] pub fn create_from_session(&self, session: &ComPtr<LearningModelSession>) -> Result<ComPtr<LearningModelBinding>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromSession)(self.as_abi() as *const _ as *mut _, session.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelDevice, 4123183358, 16214, 19084, 172, 95, 253, 185, 45, 139, 130, 82);
RT_INTERFACE!{interface ILearningModelDevice(ILearningModelDeviceVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelDevice] {
    #[cfg(feature="windows-graphics")] fn get_AdapterId(&self, out: *mut super::super::graphics::DisplayAdapterId) -> HRESULT,
    #[cfg(feature="windows-graphics")] fn get_Direct3D11Device(&self, out: *mut *mut super::super::graphics::directx::direct3d11::IDirect3DDevice) -> HRESULT
}}
impl ComPtr<ILearningModelDevice> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_adapter_id(&self) -> Result<super::super::graphics::DisplayAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_AdapterId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_direct3d11_device(&self) -> Result<Option<ComPtr<super::super::graphics::directx::direct3d11::IDirect3DDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Direct3D11Device)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelDevice: ILearningModelDevice}
impl RtActivatable<ILearningModelDeviceFactory> for LearningModelDevice {}
impl RtActivatable<ILearningModelDeviceStatics> for LearningModelDevice {}
impl LearningModelDevice {
    #[inline] pub fn create(deviceKind: LearningModelDeviceKind) -> Result<ComPtr<LearningModelDevice>> {
        <Self as RtActivatable<ILearningModelDeviceFactory>>::get_activation_factory().create(deviceKind)
    }
    #[cfg(feature="windows-graphics")] #[inline] pub fn create_from_direct3d11_device(device: &ComPtr<super::super::graphics::directx::direct3d11::IDirect3DDevice>) -> Result<Option<ComPtr<LearningModelDevice>>> {
        <Self as RtActivatable<ILearningModelDeviceStatics>>::get_activation_factory().create_from_direct3d11_device(device)
    }
}
DEFINE_CLSID!(LearningModelDevice(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,68,101,118,105,99,101,0]) [CLSID_LearningModelDevice]);
DEFINE_IID!(IID_ILearningModelDeviceFactory, 2634012493, 45541, 20256, 128, 173, 10, 86, 105, 13, 176, 107);
RT_INTERFACE!{static interface ILearningModelDeviceFactory(ILearningModelDeviceFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelDeviceFactory] {
    fn Create(&self, deviceKind: LearningModelDeviceKind, out: *mut *mut LearningModelDevice) -> HRESULT
}}
impl ComPtr<ILearningModelDeviceFactory> {
    #[inline] pub fn create(&self, deviceKind: LearningModelDeviceKind) -> Result<ComPtr<LearningModelDevice>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, deviceKind, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum LearningModelDeviceKind: i32 {
    Default = 0, Cpu = 1, DirectX = 2, DirectXHighPerformance = 3, DirectXMinPower = 4,
}}
DEFINE_IID!(IID_ILearningModelDeviceStatics, 1240670471, 43199, 17083, 146, 199, 16, 177, 45, 197, 210, 31);
RT_INTERFACE!{static interface ILearningModelDeviceStatics(ILearningModelDeviceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelDeviceStatics] {
    #[cfg(feature="windows-graphics")] fn CreateFromDirect3D11Device(&self, device: *mut super::super::graphics::directx::direct3d11::IDirect3DDevice, out: *mut *mut LearningModelDevice) -> HRESULT
}}
impl ComPtr<ILearningModelDeviceStatics> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn create_from_direct3d11_device(&self, device: &ComPtr<super::super::graphics::directx::direct3d11::IDirect3DDevice>) -> Result<Option<ComPtr<LearningModelDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromDirect3D11Device)(self.as_abi() as *const _ as *mut _, device.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelEvaluationResult, 3002712013, 38414, 18880, 133, 147, 235, 25, 10, 227, 238, 226);
RT_INTERFACE!{interface ILearningModelEvaluationResult(ILearningModelEvaluationResultVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelEvaluationResult] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ErrorStatus(&self, out: *mut i32) -> HRESULT,
    fn get_Succeeded(&self, out: *mut bool) -> HRESULT,
    fn get_Outputs(&self, out: *mut *mut foundation::collections::IMapView<HString, IInspectable>) -> HRESULT
}}
impl ComPtr<ILearningModelEvaluationResult> {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CorrelationId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_error_status(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ErrorStatus)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_succeeded(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Succeeded)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_outputs(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Outputs)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelEvaluationResult: ILearningModelEvaluationResult}
DEFINE_IID!(IID_ILearningModelFeatureDescriptor, 3154694012, 28368, 16388, 151, 186, 185, 162, 238, 205, 43, 79);
RT_INTERFACE!{interface ILearningModelFeatureDescriptor(ILearningModelFeatureDescriptorVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelFeatureDescriptor] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Kind(&self, out: *mut LearningModelFeatureKind) -> HRESULT,
    fn get_IsRequired(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<ILearningModelFeatureDescriptor> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_kind(&self) -> Result<LearningModelFeatureKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsRequired)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum LearningModelFeatureKind: i32 {
    Tensor = 0, Sequence = 1, Map = 2, Image = 3,
}}
DEFINE_IID!(IID_ILearningModelFeatureValue, 4111467995, 16517, 19966, 159, 237, 149, 235, 12, 12, 247, 92);
RT_INTERFACE!{interface ILearningModelFeatureValue(ILearningModelFeatureValueVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelFeatureValue] {
    fn get_Kind(&self, out: *mut LearningModelFeatureKind) -> HRESULT
}}
impl ComPtr<ILearningModelFeatureValue> {
    #[inline] pub fn get_kind(&self) -> Result<LearningModelFeatureKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Kind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelOperatorProvider, 706883165, 44977, 18413, 191, 173, 181, 179, 164, 89, 236, 4);
RT_INTERFACE!{interface ILearningModelOperatorProvider(ILearningModelOperatorProviderVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelOperatorProvider] {
    
}}
DEFINE_IID!(IID_ILearningModelSession, 2388195574, 46983, 19473, 144, 240, 113, 41, 174, 202, 116, 169);
RT_INTERFACE!{interface ILearningModelSession(ILearningModelSessionVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelSession] {
    fn get_Model(&self, out: *mut *mut LearningModel) -> HRESULT,
    fn get_Device(&self, out: *mut *mut LearningModelDevice) -> HRESULT,
    fn get_EvaluationProperties(&self, out: *mut *mut foundation::collections::IPropertySet) -> HRESULT,
    fn EvaluateAsync(&self, bindings: *mut LearningModelBinding, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResult>) -> HRESULT,
    fn EvaluateFeaturesAsync(&self, features: *mut foundation::collections::IMap<HString, IInspectable>, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResult>) -> HRESULT,
    fn Evaluate(&self, bindings: *mut LearningModelBinding, correlationId: HSTRING, out: *mut *mut LearningModelEvaluationResult) -> HRESULT,
    fn EvaluateFeatures(&self, features: *mut foundation::collections::IMap<HString, IInspectable>, correlationId: HSTRING, out: *mut *mut LearningModelEvaluationResult) -> HRESULT
}}
impl ComPtr<ILearningModelSession> {
    #[inline] pub fn get_model(&self) -> Result<Option<ComPtr<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Model)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_device(&self) -> Result<Option<ComPtr<LearningModelDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Device)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_evaluation_properties(&self) -> Result<Option<ComPtr<foundation::collections::IPropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_EvaluationProperties)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_async(&self, bindings: &ComPtr<LearningModelBinding>, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).EvaluateAsync)(self.as_abi() as *const _ as *mut _, bindings.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features_async(&self, features: &ComPtr<foundation::collections::IMap<HString, IInspectable>>, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).EvaluateFeaturesAsync)(self.as_abi() as *const _ as *mut _, features.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate(&self, bindings: &ComPtr<LearningModelBinding>, correlationId: &HStringArg) -> Result<Option<ComPtr<LearningModelEvaluationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Evaluate)(self.as_abi() as *const _ as *mut _, bindings.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features(&self, features: &ComPtr<foundation::collections::IMap<HString, IInspectable>>, correlationId: &HStringArg) -> Result<Option<ComPtr<LearningModelEvaluationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).EvaluateFeatures)(self.as_abi() as *const _ as *mut _, features.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelSession: ILearningModelSession}
impl RtActivatable<ILearningModelSessionFactory> for LearningModelSession {}
impl LearningModelSession {
    #[inline] pub fn create_from_model(model: &ComPtr<LearningModel>) -> Result<ComPtr<LearningModelSession>> {
        <Self as RtActivatable<ILearningModelSessionFactory>>::get_activation_factory().create_from_model(model)
    }
    #[inline] pub fn create_from_model_on_device(model: &ComPtr<LearningModel>, deviceToRunOn: &ComPtr<LearningModelDevice>) -> Result<ComPtr<LearningModelSession>> {
        <Self as RtActivatable<ILearningModelSessionFactory>>::get_activation_factory().create_from_model_on_device(model, deviceToRunOn)
    }
}
DEFINE_CLSID!(LearningModelSession(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,76,101,97,114,110,105,110,103,77,111,100,101,108,83,101,115,115,105,111,110,0]) [CLSID_LearningModelSession]);
DEFINE_IID!(IID_ILearningModelSessionFactory, 258705437, 7323, 18358, 191, 224, 241, 207, 98, 166, 117, 121);
RT_INTERFACE!{static interface ILearningModelSessionFactory(ILearningModelSessionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelSessionFactory] {
    fn CreateFromModel(&self, model: *mut LearningModel, out: *mut *mut LearningModelSession) -> HRESULT,
    fn CreateFromModelOnDevice(&self, model: *mut LearningModel, deviceToRunOn: *mut LearningModelDevice, out: *mut *mut LearningModelSession) -> HRESULT
}}
impl ComPtr<ILearningModelSessionFactory> {
    #[inline] pub fn create_from_model(&self, model: &ComPtr<LearningModel>) -> Result<ComPtr<LearningModelSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromModel)(self.as_abi() as *const _ as *mut _, model.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_model_on_device(&self, model: &ComPtr<LearningModel>, deviceToRunOn: &ComPtr<LearningModelDevice>) -> Result<ComPtr<LearningModelSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromModelOnDevice)(self.as_abi() as *const _ as *mut _, model.as_abi() as *const _ as *mut _, deviceToRunOn.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ILearningModelStatics, 3820582888, 26962, 20039, 142, 244, 31, 127, 7, 137, 124, 109);
RT_INTERFACE!{static interface ILearningModelStatics(ILearningModelStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelStatics] {
    #[cfg(feature="windows-storage")] fn LoadFromStorageFileAsync(&self, modelFile: *mut super::super::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<LearningModel>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamAsync(&self, modelStream: *mut super::super::storage::streams::IRandomAccessStreamReference, out: *mut *mut foundation::IAsyncOperation<LearningModel>) -> HRESULT,
    fn LoadFromFilePath(&self, filePath: HSTRING, out: *mut *mut LearningModel) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStream(&self, modelStream: *mut super::super::storage::streams::IRandomAccessStreamReference, out: *mut *mut LearningModel) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStorageFileWithOperatorProviderAsync(&self, modelFile: *mut super::super::storage::IStorageFile, operatorProvider: *mut ILearningModelOperatorProvider, out: *mut *mut foundation::IAsyncOperation<LearningModel>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithOperatorProviderAsync(&self, modelStream: *mut super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: *mut ILearningModelOperatorProvider, out: *mut *mut foundation::IAsyncOperation<LearningModel>) -> HRESULT,
    fn LoadFromFilePathWithOperatorProvider(&self, filePath: HSTRING, operatorProvider: *mut ILearningModelOperatorProvider, out: *mut *mut LearningModel) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithOperatorProvider(&self, modelStream: *mut super::super::storage::streams::IRandomAccessStreamReference, operatorProvider: *mut ILearningModelOperatorProvider, out: *mut *mut LearningModel) -> HRESULT
}}
impl ComPtr<ILearningModelStatics> {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_async(&self, modelFile: &ComPtr<super::super::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStorageFileAsync)(self.as_abi() as *const _ as *mut _, modelFile.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(&self, modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStreamAsync)(self.as_abi() as *const _ as *mut _, modelStream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_file_path(&self, filePath: &HStringArg) -> Result<Option<ComPtr<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFilePath)(self.as_abi() as *const _ as *mut _, filePath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream(&self, modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>) -> Result<Option<ComPtr<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStream)(self.as_abi() as *const _ as *mut _, modelStream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_storage_file_with_operator_provider_async(&self, modelFile: &ComPtr<super::super::storage::IStorageFile>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStorageFileWithOperatorProviderAsync)(self.as_abi() as *const _ as *mut _, modelFile.as_abi() as *const _ as *mut _, operatorProvider.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider_async(&self, modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStreamWithOperatorProviderAsync)(self.as_abi() as *const _ as *mut _, modelStream.as_abi() as *const _ as *mut _, operatorProvider.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_file_path_with_operator_provider(&self, filePath: &HStringArg, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<Option<ComPtr<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFilePathWithOperatorProvider)(self.as_abi() as *const _ as *mut _, filePath.get(), operatorProvider.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_operator_provider(&self, modelStream: &ComPtr<super::super::storage::streams::IRandomAccessStreamReference>, operatorProvider: &ComPtr<ILearningModelOperatorProvider>) -> Result<Option<ComPtr<LearningModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStreamWithOperatorProvider)(self.as_abi() as *const _ as *mut _, modelStream.as_abi() as *const _ as *mut _, operatorProvider.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IMapFeatureDescriptor, 1392780477, 41559, 17261, 158, 96, 194, 152, 31, 124, 197, 196);
RT_INTERFACE!{interface IMapFeatureDescriptor(IMapFeatureDescriptorVtbl): IInspectable(IInspectableVtbl) [IID_IMapFeatureDescriptor] {
    fn get_KeyKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_ValueDescriptor(&self, out: *mut *mut ILearningModelFeatureDescriptor) -> HRESULT
}}
impl ComPtr<IMapFeatureDescriptor> {
    #[inline] pub fn get_key_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_KeyKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_descriptor(&self) -> Result<Option<ComPtr<ILearningModelFeatureDescriptor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ValueDescriptor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapFeatureDescriptor: IMapFeatureDescriptor}
DEFINE_IID!(IID_ISequenceFeatureDescriptor, 2230752346, 22059, 19810, 168, 81, 115, 154, 206, 217, 102, 104);
RT_INTERFACE!{interface ISequenceFeatureDescriptor(ISequenceFeatureDescriptorVtbl): IInspectable(IInspectableVtbl) [IID_ISequenceFeatureDescriptor] {
    fn get_ElementDescriptor(&self, out: *mut *mut ILearningModelFeatureDescriptor) -> HRESULT
}}
impl ComPtr<ISequenceFeatureDescriptor> {
    #[inline] pub fn get_element_descriptor(&self) -> Result<Option<ComPtr<ILearningModelFeatureDescriptor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ElementDescriptor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SequenceFeatureDescriptor: ISequenceFeatureDescriptor}
DEFINE_IID!(IID_ITensor, 88642963, 41733, 18981, 173, 9, 68, 1, 25, 180, 183, 246);
RT_INTERFACE!{interface ITensor(ITensorVtbl): IInspectable(IInspectableVtbl) [IID_ITensor] {
    fn get_TensorKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_Shape(&self, out: *mut *mut foundation::collections::IVectorView<i64>) -> HRESULT
}}
impl ComPtr<ITensor> {
    #[inline] pub fn get_tensor_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TensorKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Shape)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorBoolean, 1358107117, 10729, 19036, 164, 77, 143, 197, 18, 88, 78, 237);
RT_INTERFACE!{interface ITensorBoolean(ITensorBooleanVtbl): IInspectable(IInspectableVtbl) [IID_ITensorBoolean] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<bool>) -> HRESULT
}}
impl ComPtr<ITensorBoolean> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<bool>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorBoolean: ITensorBoolean}
impl RtActivatable<ITensorBooleanStatics> for TensorBoolean {}
impl TensorBoolean {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorBoolean>>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorBoolean>>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[bool]) -> Result<Option<ComPtr<TensorBoolean>>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<bool>>) -> Result<Option<ComPtr<TensorBoolean>>> {
        <Self as RtActivatable<ITensorBooleanStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorBoolean(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,66,111,111,108,101,97,110,0]) [CLSID_TensorBoolean]);
DEFINE_IID!(IID_ITensorBooleanStatics, 664176172, 9047, 18855, 180, 118, 208, 170, 61, 254, 104, 102);
RT_INTERFACE!{static interface ITensorBooleanStatics(ITensorBooleanStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorBooleanStatics] {
    fn Create(&self, out: *mut *mut TensorBoolean) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorBoolean) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut bool, out: *mut *mut TensorBoolean) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<bool>, out: *mut *mut TensorBoolean) -> HRESULT
}}
impl ComPtr<ITensorBooleanStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorBoolean>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorBoolean>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[bool]) -> Result<Option<ComPtr<TensorBoolean>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<bool>>) -> Result<Option<ComPtr<TensorBoolean>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorDouble, 2447643218, 31375, 20238, 162, 143, 150, 55, 255, 200, 163, 208);
RT_INTERFACE!{interface ITensorDouble(ITensorDoubleVtbl): IInspectable(IInspectableVtbl) [IID_ITensorDouble] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<f64>) -> HRESULT
}}
impl ComPtr<ITensorDouble> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorDouble: ITensorDouble}
impl RtActivatable<ITensorDoubleStatics> for TensorDouble {}
impl TensorDouble {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorDouble>>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorDouble>>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f64]) -> Result<Option<ComPtr<TensorDouble>>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f64>>) -> Result<Option<ComPtr<TensorDouble>>> {
        <Self as RtActivatable<ITensorDoubleStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorDouble(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,68,111,117,98,108,101,0]) [CLSID_TensorDouble]);
DEFINE_IID!(IID_ITensorDoubleStatics, 2825294789, 38200, 17639, 163, 202, 93, 243, 116, 165, 167, 12);
RT_INTERFACE!{static interface ITensorDoubleStatics(ITensorDoubleStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorDoubleStatics] {
    fn Create(&self, out: *mut *mut TensorDouble) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorDouble) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut f64, out: *mut *mut TensorDouble) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<f64>, out: *mut *mut TensorDouble) -> HRESULT
}}
impl ComPtr<ITensorDoubleStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorDouble>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorDouble>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f64]) -> Result<Option<ComPtr<TensorDouble>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f64>>) -> Result<Option<ComPtr<TensorDouble>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorFeatureDescriptor, 1950702720, 37994, 17168, 161, 156, 238, 10, 240, 40, 252, 228);
RT_INTERFACE!{interface ITensorFeatureDescriptor(ITensorFeatureDescriptorVtbl): IInspectable(IInspectableVtbl) [IID_ITensorFeatureDescriptor] {
    fn get_TensorKind(&self, out: *mut TensorKind) -> HRESULT,
    fn get_Shape(&self, out: *mut *mut foundation::collections::IVectorView<i64>) -> HRESULT
}}
impl ComPtr<ITensorFeatureDescriptor> {
    #[inline] pub fn get_tensor_kind(&self) -> Result<TensorKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TensorKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Shape)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFeatureDescriptor: ITensorFeatureDescriptor}
DEFINE_IID!(IID_ITensorFloat, 4062719362, 43522, 17096, 160, 200, 223, 30, 252, 150, 118, 225);
RT_INTERFACE!{interface ITensorFloat(ITensorFloatVtbl): IInspectable(IInspectableVtbl) [IID_ITensorFloat] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<f32>) -> HRESULT
}}
impl ComPtr<ITensorFloat> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<f32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFloat: ITensorFloat}
impl RtActivatable<ITensorFloatStatics> for TensorFloat {}
impl TensorFloat {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorFloat>>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorFloat>>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f32]) -> Result<Option<ComPtr<TensorFloat>>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f32>>) -> Result<Option<ComPtr<TensorFloat>>> {
        <Self as RtActivatable<ITensorFloatStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorFloat(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,70,108,111,97,116,0]) [CLSID_TensorFloat]);
DEFINE_IID!(IID_ITensorFloat16Bit, 179934460, 23433, 19516, 181, 228, 82, 130, 165, 49, 108, 10);
RT_INTERFACE!{interface ITensorFloat16Bit(ITensorFloat16BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorFloat16Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<f32>) -> HRESULT
}}
impl ComPtr<ITensorFloat16Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<f32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorFloat16Bit: ITensorFloat16Bit}
impl RtActivatable<ITensorFloat16BitStatics> for TensorFloat16Bit {}
impl TensorFloat16Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorFloat16Bit>>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorFloat16Bit>>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f32]) -> Result<Option<ComPtr<TensorFloat16Bit>>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f32>>) -> Result<Option<ComPtr<TensorFloat16Bit>>> {
        <Self as RtActivatable<ITensorFloat16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorFloat16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,70,108,111,97,116,49,54,66,105,116,0]) [CLSID_TensorFloat16Bit]);
DEFINE_IID!(IID_ITensorFloat16BitStatics, 2771236597, 12682, 17620, 130, 11, 12, 220, 112, 84, 168, 74);
RT_INTERFACE!{static interface ITensorFloat16BitStatics(ITensorFloat16BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorFloat16BitStatics] {
    fn Create(&self, out: *mut *mut TensorFloat16Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorFloat16Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut f32, out: *mut *mut TensorFloat16Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<f32>, out: *mut *mut TensorFloat16Bit) -> HRESULT
}}
impl ComPtr<ITensorFloat16BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorFloat16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorFloat16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f32]) -> Result<Option<ComPtr<TensorFloat16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f32>>) -> Result<Option<ComPtr<TensorFloat16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorFloatStatics, 3687659867, 15267, 17711, 177, 13, 60, 19, 94, 87, 63, 169);
RT_INTERFACE!{static interface ITensorFloatStatics(ITensorFloatStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorFloatStatics] {
    fn Create(&self, out: *mut *mut TensorFloat) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorFloat) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut f32, out: *mut *mut TensorFloat) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<f32>, out: *mut *mut TensorFloat) -> HRESULT
}}
impl ComPtr<ITensorFloatStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorFloat>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorFloat>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[f32]) -> Result<Option<ComPtr<TensorFloat>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<f32>>) -> Result<Option<ComPtr<TensorFloat>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt16Bit, 2560830777, 59094, 17583, 138, 250, 186, 235, 196, 77, 192, 32);
RT_INTERFACE!{interface ITensorInt16Bit(ITensorInt16BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt16Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<i16>) -> HRESULT
}}
impl ComPtr<ITensorInt16Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<i16>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt16Bit: ITensorInt16Bit}
impl RtActivatable<ITensorInt16BitStatics> for TensorInt16Bit {}
impl TensorInt16Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorInt16Bit>>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt16Bit>>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i16]) -> Result<Option<ComPtr<TensorInt16Bit>>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i16>>) -> Result<Option<ComPtr<TensorInt16Bit>>> {
        <Self as RtActivatable<ITensorInt16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,49,54,66,105,116,0]) [CLSID_TensorInt16Bit]);
DEFINE_IID!(IID_ITensorInt16BitStatics, 2556715667, 9838, 19226, 130, 31, 230, 13, 112, 137, 139, 145);
RT_INTERFACE!{static interface ITensorInt16BitStatics(ITensorInt16BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt16BitStatics] {
    fn Create(&self, out: *mut *mut TensorInt16Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorInt16Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut i16, out: *mut *mut TensorInt16Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<i16>, out: *mut *mut TensorInt16Bit) -> HRESULT
}}
impl ComPtr<ITensorInt16BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i16]) -> Result<Option<ComPtr<TensorInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i16>>) -> Result<Option<ComPtr<TensorInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt32Bit, 738994387, 8316, 17542, 167, 210, 136, 69, 34, 197, 229, 137);
RT_INTERFACE!{interface ITensorInt32Bit(ITensorInt32BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt32Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<i32>) -> HRESULT
}}
impl ComPtr<ITensorInt32Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<i32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt32Bit: ITensorInt32Bit}
impl RtActivatable<ITensorInt32BitStatics> for TensorInt32Bit {}
impl TensorInt32Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorInt32Bit>>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt32Bit>>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i32]) -> Result<Option<ComPtr<TensorInt32Bit>>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i32>>) -> Result<Option<ComPtr<TensorInt32Bit>>> {
        <Self as RtActivatable<ITensorInt32BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt32Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,51,50,66,105,116,0]) [CLSID_TensorInt32Bit]);
DEFINE_IID!(IID_ITensorInt32BitStatics, 1698268747, 21242, 20021, 144, 124, 131, 76, 172, 65, 123, 80);
RT_INTERFACE!{static interface ITensorInt32BitStatics(ITensorInt32BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt32BitStatics] {
    fn Create(&self, out: *mut *mut TensorInt32Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorInt32Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut i32, out: *mut *mut TensorInt32Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<i32>, out: *mut *mut TensorInt32Bit) -> HRESULT
}}
impl ComPtr<ITensorInt32BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i32]) -> Result<Option<ComPtr<TensorInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i32>>) -> Result<Option<ComPtr<TensorInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt64Bit, 1234593210, 8098, 17837, 175, 37, 160, 189, 155, 218, 76, 135);
RT_INTERFACE!{interface ITensorInt64Bit(ITensorInt64BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt64Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<i64>) -> HRESULT
}}
impl ComPtr<ITensorInt64Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt64Bit: ITensorInt64Bit}
impl RtActivatable<ITensorInt64BitStatics> for TensorInt64Bit {}
impl TensorInt64Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorInt64Bit>>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt64Bit>>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i64]) -> Result<Option<ComPtr<TensorInt64Bit>>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt64Bit>>> {
        <Self as RtActivatable<ITensorInt64BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt64Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,54,52,66,105,116,0]) [CLSID_TensorInt64Bit]);
DEFINE_IID!(IID_ITensorInt64BitStatics, 2521345437, 4504, 19828, 149, 23, 120, 58, 182, 43, 156, 194);
RT_INTERFACE!{static interface ITensorInt64BitStatics(ITensorInt64BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt64BitStatics] {
    fn Create(&self, out: *mut *mut TensorInt64Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorInt64Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut i64, out: *mut *mut TensorInt64Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorInt64Bit) -> HRESULT
}}
impl ComPtr<ITensorInt64BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[i64]) -> Result<Option<ComPtr<TensorInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorInt8Bit, 3453851589, 65496, 20463, 174, 251, 48, 225, 164, 133, 178, 238);
RT_INTERFACE!{interface ITensorInt8Bit(ITensorInt8BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt8Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<u8>) -> HRESULT
}}
impl ComPtr<ITensorInt8Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u8>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorInt8Bit: ITensorInt8Bit}
impl RtActivatable<ITensorInt8BitStatics> for TensorInt8Bit {}
impl TensorInt8Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorInt8Bit>>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt8Bit>>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u8]) -> Result<Option<ComPtr<TensorInt8Bit>>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u8>>) -> Result<Option<ComPtr<TensorInt8Bit>>> {
        <Self as RtActivatable<ITensorInt8BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorInt8Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,73,110,116,56,66,105,116,0]) [CLSID_TensorInt8Bit]);
DEFINE_IID!(IID_ITensorInt8BitStatics, 2980127364, 2396, 19574, 166, 97, 172, 76, 238, 31, 62, 139);
RT_INTERFACE!{static interface ITensorInt8BitStatics(ITensorInt8BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorInt8BitStatics] {
    fn Create(&self, out: *mut *mut TensorInt8Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorInt8Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut u8, out: *mut *mut TensorInt8Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<u8>, out: *mut *mut TensorInt8Bit) -> HRESULT
}}
impl ComPtr<ITensorInt8BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u8]) -> Result<Option<ComPtr<TensorInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u8>>) -> Result<Option<ComPtr<TensorInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TensorKind: i32 {
    Undefined = 0, Float = 1, UInt8 = 2, Int8 = 3, UInt16 = 4, Int16 = 5, Int32 = 6, Int64 = 7, String = 8, Boolean = 9, Float16 = 10, Double = 11, UInt32 = 12, UInt64 = 13, Complex64 = 14, Complex128 = 15,
}}
DEFINE_IID!(IID_ITensorString, 1478702536, 48561, 17936, 188, 117, 53, 233, 203, 240, 9, 183);
RT_INTERFACE!{interface ITensorString(ITensorStringVtbl): IInspectable(IInspectableVtbl) [IID_ITensorString] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl ComPtr<ITensorString> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorString: ITensorString}
impl RtActivatable<ITensorStringStatics> for TensorString {}
impl TensorString {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorString>>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorString>>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[&HStringArg]) -> Result<Option<ComPtr<TensorString>>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<Option<ComPtr<TensorString>>> {
        <Self as RtActivatable<ITensorStringStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorString(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,83,116,114,105,110,103,0]) [CLSID_TensorString]);
DEFINE_IID!(IID_ITensorStringStatics, 2204250916, 53030, 20247, 162, 212, 32, 239, 141, 9, 125, 83);
RT_INTERFACE!{static interface ITensorStringStatics(ITensorStringStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorStringStatics] {
    fn Create(&self, out: *mut *mut TensorString) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorString) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut HSTRING, out: *mut *mut TensorString) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<HString>, out: *mut *mut TensorString) -> HRESULT
}}
impl ComPtr<ITensorStringStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[&HStringArg]) -> Result<Option<ComPtr<TensorString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<Option<ComPtr<TensorString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt16Bit, 1746145099, 9152, 17139, 129, 246, 168, 145, 192, 17, 188, 63);
RT_INTERFACE!{interface ITensorUInt16Bit(ITensorUInt16BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt16Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<u16>) -> HRESULT
}}
impl ComPtr<ITensorUInt16Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u16>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt16Bit: ITensorUInt16Bit}
impl RtActivatable<ITensorUInt16BitStatics> for TensorUInt16Bit {}
impl TensorUInt16Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorUInt16Bit>>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt16Bit>>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u16]) -> Result<Option<ComPtr<TensorUInt16Bit>>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u16>>) -> Result<Option<ComPtr<TensorUInt16Bit>>> {
        <Self as RtActivatable<ITensorUInt16BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt16Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,49,54,66,105,116,0]) [CLSID_TensorUInt16Bit]);
DEFINE_IID!(IID_ITensorUInt16BitStatics, 1576486365, 650, 18458, 162, 124, 199, 230, 67, 94, 82, 221);
RT_INTERFACE!{static interface ITensorUInt16BitStatics(ITensorUInt16BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt16BitStatics] {
    fn Create(&self, out: *mut *mut TensorUInt16Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorUInt16Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut u16, out: *mut *mut TensorUInt16Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<u16>, out: *mut *mut TensorUInt16Bit) -> HRESULT
}}
impl ComPtr<ITensorUInt16BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorUInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u16]) -> Result<Option<ComPtr<TensorUInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u16>>) -> Result<Option<ComPtr<TensorUInt16Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt32Bit, 3637101311, 29969, 17827, 191, 172, 195, 143, 55, 13, 34, 55);
RT_INTERFACE!{interface ITensorUInt32Bit(ITensorUInt32BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt32Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl ComPtr<ITensorUInt32Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt32Bit: ITensorUInt32Bit}
impl RtActivatable<ITensorUInt32BitStatics> for TensorUInt32Bit {}
impl TensorUInt32Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorUInt32Bit>>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt32Bit>>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u32]) -> Result<Option<ComPtr<TensorUInt32Bit>>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u32>>) -> Result<Option<ComPtr<TensorUInt32Bit>>> {
        <Self as RtActivatable<ITensorUInt32BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt32Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,51,50,66,105,116,0]) [CLSID_TensorUInt32Bit]);
DEFINE_IID!(IID_ITensorUInt32BitStatics, 1098659895, 59251, 17272, 142, 127, 12, 195, 61, 190, 166, 151);
RT_INTERFACE!{static interface ITensorUInt32BitStatics(ITensorUInt32BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt32BitStatics] {
    fn Create(&self, out: *mut *mut TensorUInt32Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorUInt32Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut u32, out: *mut *mut TensorUInt32Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<u32>, out: *mut *mut TensorUInt32Bit) -> HRESULT
}}
impl ComPtr<ITensorUInt32BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorUInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u32]) -> Result<Option<ComPtr<TensorUInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u32>>) -> Result<Option<ComPtr<TensorUInt32Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt64Bit, 779157421, 1215, 18469, 131, 154, 130, 186, 239, 140, 120, 134);
RT_INTERFACE!{interface ITensorUInt64Bit(ITensorUInt64BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt64Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<u64>) -> HRESULT
}}
impl ComPtr<ITensorUInt64Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt64Bit: ITensorUInt64Bit}
impl RtActivatable<ITensorUInt64BitStatics> for TensorUInt64Bit {}
impl TensorUInt64Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorUInt64Bit>>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt64Bit>>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u64]) -> Result<Option<ComPtr<TensorUInt64Bit>>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u64>>) -> Result<Option<ComPtr<TensorUInt64Bit>>> {
        <Self as RtActivatable<ITensorUInt64BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt64Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,54,52,66,105,116,0]) [CLSID_TensorUInt64Bit]);
DEFINE_IID!(IID_ITensorUInt64BitStatics, 2055086315, 9263, 18379, 169, 198, 246, 2, 236, 251, 254, 228);
RT_INTERFACE!{static interface ITensorUInt64BitStatics(ITensorUInt64BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt64BitStatics] {
    fn Create(&self, out: *mut *mut TensorUInt64Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorUInt64Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut u64, out: *mut *mut TensorUInt64Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<u64>, out: *mut *mut TensorUInt64Bit) -> HRESULT
}}
impl ComPtr<ITensorUInt64BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorUInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u64]) -> Result<Option<ComPtr<TensorUInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u64>>) -> Result<Option<ComPtr<TensorUInt64Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITensorUInt8Bit, 1491185191, 25131, 18659, 190, 34, 216, 103, 174, 209, 218, 172);
RT_INTERFACE!{interface ITensorUInt8Bit(ITensorUInt8BitVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt8Bit] {
    fn GetAsVectorView(&self, out: *mut *mut foundation::collections::IVectorView<u8>) -> HRESULT
}}
impl ComPtr<ITensorUInt8Bit> {
    #[inline] pub fn get_as_vector_view(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u8>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAsVectorView)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorUInt8Bit: ITensorUInt8Bit}
impl RtActivatable<ITensorUInt8BitStatics> for TensorUInt8Bit {}
impl TensorUInt8Bit {
    #[inline] pub fn create() -> Result<Option<ComPtr<TensorUInt8Bit>>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create()
    }
    #[inline] pub fn create2(shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt8Bit>>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create2(shape)
    }
    #[inline] pub fn create_from_array(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u8]) -> Result<Option<ComPtr<TensorUInt8Bit>>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create_from_array(shape, data)
    }
    #[inline] pub fn create_from_iterable(shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u8>>) -> Result<Option<ComPtr<TensorUInt8Bit>>> {
        <Self as RtActivatable<ITensorUInt8BitStatics>>::get_activation_factory().create_from_iterable(shape, data)
    }
}
DEFINE_CLSID!(TensorUInt8Bit(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,84,101,110,115,111,114,85,73,110,116,56,66,105,116,0]) [CLSID_TensorUInt8Bit]);
DEFINE_IID!(IID_ITensorUInt8BitStatics, 100038019, 48164, 16928, 138, 65, 45, 205, 140, 94, 211, 60);
RT_INTERFACE!{static interface ITensorUInt8BitStatics(ITensorUInt8BitStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ITensorUInt8BitStatics] {
    fn Create(&self, out: *mut *mut TensorUInt8Bit) -> HRESULT,
    fn Create2(&self, shape: *mut foundation::collections::IIterable<i64>, out: *mut *mut TensorUInt8Bit) -> HRESULT,
    fn CreateFromArray(&self, shape: *mut foundation::collections::IIterable<i64>, dataSize: u32, data: *mut u8, out: *mut *mut TensorUInt8Bit) -> HRESULT,
    fn CreateFromIterable(&self, shape: *mut foundation::collections::IIterable<i64>, data: *mut foundation::collections::IIterable<u8>, out: *mut *mut TensorUInt8Bit) -> HRESULT
}}
impl ComPtr<ITensorUInt8BitStatics> {
    #[inline] pub fn create(&self) -> Result<Option<ComPtr<TensorUInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create2(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>) -> Result<Option<ComPtr<TensorUInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create2)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_array(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &[u8]) -> Result<Option<ComPtr<TensorUInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromArray)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.len() as u32, data.as_ptr() as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_iterable(&self, shape: &ComPtr<foundation::collections::IIterable<i64>>, data: &ComPtr<foundation::collections::IIterable<u8>>) -> Result<Option<ComPtr<TensorUInt8Bit>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromIterable)(self.as_abi() as *const _ as *mut _, shape.as_abi() as *const _ as *mut _, data.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
pub mod preview { // Windows.AI.MachineLearning.Preview
use crate::prelude::*;
RT_ENUM! { enum FeatureElementKindPreview: i32 {
    Undefined = 0, Float = 1, UInt8 = 2, Int8 = 3, UInt16 = 4, Int16 = 5, Int32 = 6, Int64 = 7, String = 8, Boolean = 9, Float16 = 10, Double = 11, UInt32 = 12, UInt64 = 13, Complex64 = 14, Complex128 = 15,
}}
DEFINE_IID!(IID_IImageVariableDescriptorPreview, 2061630066, 670, 19909, 162, 248, 95, 183, 99, 21, 65, 80);
RT_INTERFACE!{interface IImageVariableDescriptorPreview(IImageVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_IImageVariableDescriptorPreview] {
    #[cfg(not(feature="windows-graphics"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-graphics")] fn get_BitmapPixelFormat(&self, out: *mut crate::windows::graphics::imaging::BitmapPixelFormat) -> HRESULT,
    fn get_Width(&self, out: *mut u32) -> HRESULT,
    fn get_Height(&self, out: *mut u32) -> HRESULT
}}
impl ComPtr<IImageVariableDescriptorPreview> {
    #[cfg(feature="windows-graphics")] #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<crate::windows::graphics::imaging::BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BitmapPixelFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Width)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Height)(self.as_abi() as *const _ as *mut _, &mut out);
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
impl ComPtr<IInferencingOptionsPreview> {
    #[inline] pub fn get_preferred_device_kind(&self) -> Result<LearningModelDeviceKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PreferredDeviceKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_preferred_device_kind(&self, value: LearningModelDeviceKindPreview) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_PreferredDeviceKind)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_tracing_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsTracingEnabled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_tracing_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_IsTracingEnabled)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_batch_size(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MaxBatchSize)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_batch_size(&self, value: i32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_MaxBatchSize)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_minimize_memory_allocation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MinimizeMemoryAllocation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_minimize_memory_allocation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_MinimizeMemoryAllocation)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_reclaim_memory_after_evaluation(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ReclaimMemoryAfterEvaluation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_reclaim_memory_after_evaluation(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ReclaimMemoryAfterEvaluation)(self.as_abi() as *const _ as *mut _, value);
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
impl ComPtr<ILearningModelBindingPreview> {
    #[inline] pub fn bind(&self, name: &HStringArg, value: &ComPtr<IInspectable>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Bind)(self.as_abi() as *const _ as *mut _, name.get(), value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn bind_with_properties(&self, name: &HStringArg, value: &ComPtr<IInspectable>, metadata: &ComPtr<foundation::collections::IPropertySet>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).BindWithProperties)(self.as_abi() as *const _ as *mut _, name.get(), value.as_abi() as *const _ as *mut _, metadata.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn clear(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Clear)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelBindingPreview: ILearningModelBindingPreview}
impl RtActivatable<ILearningModelBindingPreviewFactory> for LearningModelBindingPreview {}
impl LearningModelBindingPreview {
    #[inline] pub fn create_from_model(model: &ComPtr<LearningModelPreview>) -> Result<ComPtr<LearningModelBindingPreview>> {
        <Self as RtActivatable<ILearningModelBindingPreviewFactory>>::get_activation_factory().create_from_model(model)
    }
}
DEFINE_CLSID!(LearningModelBindingPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,66,105,110,100,105,110,103,80,114,101,118,105,101,119,0]) [CLSID_LearningModelBindingPreview]);
DEFINE_IID!(IID_ILearningModelBindingPreviewFactory, 1220026783, 7761, 19831, 174, 80, 62, 193, 100, 173, 52, 128);
RT_INTERFACE!{static interface ILearningModelBindingPreviewFactory(ILearningModelBindingPreviewFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelBindingPreviewFactory] {
    fn CreateFromModel(&self, model: *mut LearningModelPreview, out: *mut *mut LearningModelBindingPreview) -> HRESULT
}}
impl ComPtr<ILearningModelBindingPreviewFactory> {
    #[inline] pub fn create_from_model(&self, model: &ComPtr<LearningModelPreview>) -> Result<ComPtr<LearningModelBindingPreview>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateFromModel)(self.as_abi() as *const _ as *mut _, model.as_abi() as *const _ as *mut _, &mut out);
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
impl ComPtr<ILearningModelDescriptionPreview> {
    #[inline] pub fn get_author(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Author)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_domain(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Domain)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<i64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Version)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Metadata)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_features(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InputFeatures)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_output_features(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<ILearningModelVariableDescriptorPreview>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_OutputFeatures)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelDescriptionPreview: ILearningModelDescriptionPreview}
RT_ENUM! { enum LearningModelDeviceKindPreview: i32 {
    LearningDeviceAny = 0, LearningDeviceCpu = 1, LearningDeviceGpu = 2, LearningDeviceNpu = 3, LearningDeviceDsp = 4, LearningDeviceFpga = 5,
}}
DEFINE_IID!(IID_ILearningModelEvaluationResultPreview, 3743804063, 39011, 16520, 132, 152, 135, 161, 244, 104, 111, 146);
RT_INTERFACE!{interface ILearningModelEvaluationResultPreview(ILearningModelEvaluationResultPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelEvaluationResultPreview] {
    fn get_CorrelationId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Outputs(&self, out: *mut *mut foundation::collections::IMapView<HString, IInspectable>) -> HRESULT
}}
impl ComPtr<ILearningModelEvaluationResultPreview> {
    #[inline] pub fn get_correlation_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_CorrelationId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_outputs(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Outputs)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelEvaluationResultPreview: ILearningModelEvaluationResultPreview}
RT_ENUM! { enum LearningModelFeatureKindPreview: i32 {
    Undefined = 0, Tensor = 1, Sequence = 2, Map = 3, Image = 4,
}}
DEFINE_IID!(IID_ILearningModelPreview, 77342314, 37812, 18316, 174, 184, 112, 21, 123, 240, 255, 148);
RT_INTERFACE!{interface ILearningModelPreview(ILearningModelPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelPreview] {
    fn EvaluateAsync(&self, binding: *mut LearningModelBindingPreview, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResultPreview>) -> HRESULT,
    fn EvaluateFeaturesAsync(&self, features: *mut foundation::collections::IMap<HString, IInspectable>, correlationId: HSTRING, out: *mut *mut foundation::IAsyncOperation<LearningModelEvaluationResultPreview>) -> HRESULT,
    fn get_Description(&self, out: *mut *mut LearningModelDescriptionPreview) -> HRESULT,
    fn get_InferencingOptions(&self, out: *mut *mut InferencingOptionsPreview) -> HRESULT,
    fn put_InferencingOptions(&self, value: *mut InferencingOptionsPreview) -> HRESULT
}}
impl ComPtr<ILearningModelPreview> {
    #[inline] pub fn evaluate_async(&self, binding: &ComPtr<LearningModelBindingPreview>, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).EvaluateAsync)(self.as_abi() as *const _ as *mut _, binding.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn evaluate_features_async(&self, features: &ComPtr<foundation::collections::IMap<HString, IInspectable>>, correlationId: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelEvaluationResultPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).EvaluateFeaturesAsync)(self.as_abi() as *const _ as *mut _, features.as_abi() as *const _ as *mut _, correlationId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<Option<ComPtr<LearningModelDescriptionPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inferencing_options(&self) -> Result<Option<ComPtr<InferencingOptionsPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InferencingOptions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_inferencing_options(&self, value: &ComPtr<InferencingOptionsPreview>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_InferencingOptions)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class LearningModelPreview: ILearningModelPreview}
impl RtActivatable<ILearningModelPreviewStatics> for LearningModelPreview {}
impl LearningModelPreview {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(modelFile: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_storage_file_async(modelFile)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(modelStream: &ComPtr<crate::windows::storage::streams::IRandomAccessStreamReference>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> {
        <Self as RtActivatable<ILearningModelPreviewStatics>>::get_activation_factory().load_model_from_stream_async(modelStream)
    }
}
DEFINE_CLSID!(LearningModelPreview(&[87,105,110,100,111,119,115,46,65,73,46,77,97,99,104,105,110,101,76,101,97,114,110,105,110,103,46,80,114,101,118,105,101,119,46,76,101,97,114,110,105,110,103,77,111,100,101,108,80,114,101,118,105,101,119,0]) [CLSID_LearningModelPreview]);
DEFINE_IID!(IID_ILearningModelPreviewStatics, 374061920, 33893, 18310, 139, 147, 44, 22, 168, 146, 137, 215);
RT_INTERFACE!{static interface ILearningModelPreviewStatics(ILearningModelPreviewStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ILearningModelPreviewStatics] {
    #[cfg(feature="windows-storage")] fn LoadModelFromStorageFileAsync(&self, modelFile: *mut crate::windows::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<LearningModelPreview>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadModelFromStreamAsync(&self, modelStream: *mut crate::windows::storage::streams::IRandomAccessStreamReference, out: *mut *mut foundation::IAsyncOperation<LearningModelPreview>) -> HRESULT
}}
impl ComPtr<ILearningModelPreviewStatics> {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_storage_file_async(&self, modelFile: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadModelFromStorageFileAsync)(self.as_abi() as *const _ as *mut _, modelFile.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_stream_async(&self, modelStream: &ComPtr<crate::windows::storage::streams::IRandomAccessStreamReference>) -> Result<ComPtr<foundation::IAsyncOperation<LearningModelPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadModelFromStreamAsync)(self.as_abi() as *const _ as *mut _, modelStream.as_abi() as *const _ as *mut _, &mut out);
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
impl ComPtr<ILearningModelVariableDescriptorPreview> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Description)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_model_feature_kind(&self) -> Result<LearningModelFeatureKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ModelFeatureKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_required(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsRequired)(self.as_abi() as *const _ as *mut _, &mut out);
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
impl ComPtr<IMapVariableDescriptorPreview> {
    #[inline] pub fn get_key_kind(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_KeyKind)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_string_keys(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ValidStringKeys)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_valid_integer_keys(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ValidIntegerKeys)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_fields(&self) -> Result<Option<ComPtr<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Fields)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class MapVariableDescriptorPreview: IMapVariableDescriptorPreview}
DEFINE_IID!(IID_ISequenceVariableDescriptorPreview, 2631463570, 39090, 17712, 161, 182, 45, 237, 95, 236, 188, 38);
RT_INTERFACE!{interface ISequenceVariableDescriptorPreview(ISequenceVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ISequenceVariableDescriptorPreview] {
    fn get_ElementType(&self, out: *mut *mut ILearningModelVariableDescriptorPreview) -> HRESULT
}}
impl ComPtr<ISequenceVariableDescriptorPreview> {
    #[inline] pub fn get_element_type(&self) -> Result<Option<ComPtr<ILearningModelVariableDescriptorPreview>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ElementType)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SequenceVariableDescriptorPreview: ISequenceVariableDescriptorPreview}
DEFINE_IID!(IID_ITensorVariableDescriptorPreview, 2819575834, 39596, 16947, 151, 132, 172, 234, 249, 37, 16, 181);
RT_INTERFACE!{interface ITensorVariableDescriptorPreview(ITensorVariableDescriptorPreviewVtbl): IInspectable(IInspectableVtbl) [IID_ITensorVariableDescriptorPreview] {
    fn get_DataType(&self, out: *mut FeatureElementKindPreview) -> HRESULT,
    fn get_Shape(&self, out: *mut *mut foundation::collections::IIterable<i64>) -> HRESULT
}}
impl ComPtr<ITensorVariableDescriptorPreview> {
    #[inline] pub fn get_data_type(&self) -> Result<FeatureElementKindPreview> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DataType)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_shape(&self) -> Result<Option<ComPtr<foundation::collections::IIterable<i64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Shape)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TensorVariableDescriptorPreview: ITensorVariableDescriptorPreview}
} // Windows.AI.MachineLearning.Preview
} // Windows.AI.MachineLearning
