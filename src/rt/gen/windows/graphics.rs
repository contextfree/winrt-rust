use crate::prelude::*;
RT_STRUCT! { struct DisplayAdapterId {
    LowPart: u32, HighPart: i32,
}}
DEFINE_IID!(IID_IGeometrySource2D, 3405740290, 26380, 16769, 166, 36, 218, 151, 114, 3, 184, 69);
RT_INTERFACE!{interface IGeometrySource2D(IGeometrySource2DVtbl): IInspectable(IInspectableVtbl) [IID_IGeometrySource2D] {
    
}}
RT_STRUCT! { struct PointInt32 {
    X: i32, Y: i32,
}}
RT_STRUCT! { struct RectInt32 {
    X: i32, Y: i32, Width: i32, Height: i32,
}}
RT_STRUCT! { struct SizeInt32 {
    Width: i32, Height: i32,
}}
pub mod capture { // Windows.Graphics.Capture
use crate::prelude::*;
DEFINE_IID!(IID_IDirect3D11CaptureFrame, 4199597603, 14554, 19250, 172, 243, 250, 151, 52, 173, 128, 14);
RT_INTERFACE!{interface IDirect3D11CaptureFrame(IDirect3D11CaptureFrameVtbl): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFrame] {
    fn get_Surface(&self, out: *mut *mut super::directx::direct3d11::IDirect3DSurface) -> HRESULT,
    fn get_SystemRelativeTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_ContentSize(&self, out: *mut super::SizeInt32) -> HRESULT
}}
impl IDirect3D11CaptureFrame {
    #[inline] pub fn get_surface(&self) -> Result<Option<ComPtr<super::directx::direct3d11::IDirect3DSurface>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Surface)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_relative_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SystemRelativeTime)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_content_size(&self) -> Result<super::SizeInt32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ContentSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Direct3D11CaptureFrame: IDirect3D11CaptureFrame}
DEFINE_IID!(IID_IDirect3D11CaptureFramePool, 619408674, 6517, 16942, 130, 231, 120, 13, 189, 141, 223, 36);
RT_INTERFACE!{interface IDirect3D11CaptureFramePool(IDirect3D11CaptureFramePoolVtbl): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePool] {
    fn Recreate(&self, device: *mut super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> HRESULT,
    fn TryGetNextFrame(&self, out: *mut *mut Direct3D11CaptureFrame) -> HRESULT,
    fn add_FrameArrived(&self, handler: *mut foundation::TypedEventHandler<Direct3D11CaptureFramePool, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameArrived(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateCaptureSession(&self, item: *mut GraphicsCaptureItem, out: *mut *mut GraphicsCaptureSession) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_DispatcherQueue(&self, out: *mut *mut super::super::system::DispatcherQueue) -> HRESULT
}}
impl IDirect3D11CaptureFramePool {
    #[inline] pub fn recreate(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Recreate)(self as *const _ as *mut _, device as *const _ as *mut _, pixelFormat, numberOfBuffers, size);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_get_next_frame(&self) -> Result<Option<ComPtr<Direct3D11CaptureFrame>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetNextFrame)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_arrived(&self, handler: &foundation::TypedEventHandler<Direct3D11CaptureFramePool, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_FrameArrived)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_arrived(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_FrameArrived)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_capture_session(&self, item: &GraphicsCaptureItem) -> Result<Option<ComPtr<GraphicsCaptureSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCaptureSession)(self as *const _ as *mut _, item as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_dispatcher_queue(&self) -> Result<Option<ComPtr<super::super::system::DispatcherQueue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DispatcherQueue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Direct3D11CaptureFramePool: IDirect3D11CaptureFramePool}
impl RtActivatable<IDirect3D11CaptureFramePoolStatics> for Direct3D11CaptureFramePool {}
impl RtActivatable<IDirect3D11CaptureFramePoolStatics2> for Direct3D11CaptureFramePool {}
impl Direct3D11CaptureFramePool {
    #[inline] pub fn create(device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<ComPtr<Direct3D11CaptureFramePool>>> {
        <Self as RtActivatable<IDirect3D11CaptureFramePoolStatics>>::get_activation_factory().create(device, pixelFormat, numberOfBuffers, size)
    }
    #[inline] pub fn create_free_threaded(device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<ComPtr<Direct3D11CaptureFramePool>>> {
        <Self as RtActivatable<IDirect3D11CaptureFramePoolStatics2>>::get_activation_factory().create_free_threaded(device, pixelFormat, numberOfBuffers, size)
    }
}
DEFINE_CLSID!(Direct3D11CaptureFramePool(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,68,105,114,101,99,116,51,68,49,49,67,97,112,116,117,114,101,70,114,97,109,101,80,111,111,108,0]) [CLSID_Direct3D11CaptureFramePool]);
DEFINE_IID!(IID_IDirect3D11CaptureFramePoolStatics, 2005140842, 26538, 19795, 174, 84, 16, 136, 213, 168, 202, 33);
RT_INTERFACE!{static interface IDirect3D11CaptureFramePoolStatics(IDirect3D11CaptureFramePoolStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePoolStatics] {
    fn Create(&self, device: *mut super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32, out: *mut *mut Direct3D11CaptureFramePool) -> HRESULT
}}
impl IDirect3D11CaptureFramePoolStatics {
    #[inline] pub fn create(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<ComPtr<Direct3D11CaptureFramePool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, device as *const _ as *mut _, pixelFormat, numberOfBuffers, size, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDirect3D11CaptureFramePoolStatics2, 1486557247, 27580, 24053, 169, 145, 2, 226, 139, 59, 102, 213);
RT_INTERFACE!{static interface IDirect3D11CaptureFramePoolStatics2(IDirect3D11CaptureFramePoolStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePoolStatics2] {
    fn CreateFreeThreaded(&self, device: *mut super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32, out: *mut *mut Direct3D11CaptureFramePool) -> HRESULT
}}
impl IDirect3D11CaptureFramePoolStatics2 {
    #[inline] pub fn create_free_threaded(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<ComPtr<Direct3D11CaptureFramePool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFreeThreaded)(self as *const _ as *mut _, device as *const _ as *mut _, pixelFormat, numberOfBuffers, size, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsCaptureItem, 2042886491, 12791, 20162, 164, 100, 99, 46, 245, 211, 7, 96);
RT_INTERFACE!{interface IGraphicsCaptureItem(IGraphicsCaptureItemVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureItem] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Size(&self, out: *mut super::SizeInt32) -> HRESULT,
    fn add_Closed(&self, handler: *mut foundation::TypedEventHandler<GraphicsCaptureItem, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IGraphicsCaptureItem {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<super::SizeInt32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_closed(&self, handler: &foundation::TypedEventHandler<GraphicsCaptureItem, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Closed)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Closed)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GraphicsCaptureItem: IGraphicsCaptureItem}
impl RtActivatable<IGraphicsCaptureItemStatics> for GraphicsCaptureItem {}
impl GraphicsCaptureItem {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_from_visual(visual: &super::super::ui::composition::Visual) -> Result<Option<ComPtr<GraphicsCaptureItem>>> {
        <Self as RtActivatable<IGraphicsCaptureItemStatics>>::get_activation_factory().create_from_visual(visual)
    }
}
DEFINE_CLSID!(GraphicsCaptureItem(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,71,114,97,112,104,105,99,115,67,97,112,116,117,114,101,73,116,101,109,0]) [CLSID_GraphicsCaptureItem]);
DEFINE_IID!(IID_IGraphicsCaptureItemStatics, 2826878629, 17788, 22408, 171, 71, 12, 241, 211, 99, 126, 116);
RT_INTERFACE!{static interface IGraphicsCaptureItemStatics(IGraphicsCaptureItemStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureItemStatics] {
    #[cfg(feature="windows-ui")] fn CreateFromVisual(&self, visual: *mut super::super::ui::composition::Visual, out: *mut *mut GraphicsCaptureItem) -> HRESULT
}}
impl IGraphicsCaptureItemStatics {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_from_visual(&self, visual: &super::super::ui::composition::Visual) -> Result<Option<ComPtr<GraphicsCaptureItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromVisual)(self as *const _ as *mut _, visual as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsCapturePicker, 1511461299, 44409, 19274, 147, 54, 19, 24, 253, 222, 53, 57);
RT_INTERFACE!{interface IGraphicsCapturePicker(IGraphicsCapturePickerVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsCapturePicker] {
    fn PickSingleItemAsync(&self, out: *mut *mut foundation::IAsyncOperation<GraphicsCaptureItem>) -> HRESULT
}}
impl IGraphicsCapturePicker {
    #[inline] pub fn pick_single_item_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<GraphicsCaptureItem>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).PickSingleItemAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GraphicsCapturePicker: IGraphicsCapturePicker}
impl RtActivatable<IActivationFactory> for GraphicsCapturePicker {}
DEFINE_CLSID!(GraphicsCapturePicker(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,71,114,97,112,104,105,99,115,67,97,112,116,117,114,101,80,105,99,107,101,114,0]) [CLSID_GraphicsCapturePicker]);
DEFINE_IID!(IID_IGraphicsCaptureSession, 2169389737, 63247, 19159, 147, 155, 253, 220, 198, 235, 136, 13);
RT_INTERFACE!{interface IGraphicsCaptureSession(IGraphicsCaptureSessionVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureSession] {
    fn StartCapture(&self) -> HRESULT
}}
impl IGraphicsCaptureSession {
    #[inline] pub fn start_capture(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartCapture)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GraphicsCaptureSession: IGraphicsCaptureSession}
impl RtActivatable<IGraphicsCaptureSessionStatics> for GraphicsCaptureSession {}
impl GraphicsCaptureSession {
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IGraphicsCaptureSessionStatics>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(GraphicsCaptureSession(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,71,114,97,112,104,105,99,115,67,97,112,116,117,114,101,83,101,115,115,105,111,110,0]) [CLSID_GraphicsCaptureSession]);
DEFINE_IID!(IID_IGraphicsCaptureSessionStatics, 572826944, 22900, 18858, 178, 50, 8, 130, 83, 111, 76, 181);
RT_INTERFACE!{static interface IGraphicsCaptureSessionStatics(IGraphicsCaptureSessionStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureSessionStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IGraphicsCaptureSessionStatics {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Graphics.Capture
pub mod directx { // Windows.Graphics.DirectX
use crate::prelude::*;
RT_ENUM! { enum DirectXAlphaMode: i32 {
    Unspecified = 0, Premultiplied = 1, Straight = 2, Ignore = 3,
}}
RT_ENUM! { enum DirectXColorSpace: i32 {
    RgbFullG22NoneP709 = 0, RgbFullG10NoneP709 = 1, RgbStudioG22NoneP709 = 2, RgbStudioG22NoneP2020 = 3, Reserved = 4, YccFullG22NoneP709X601 = 5, YccStudioG22LeftP601 = 6, YccFullG22LeftP601 = 7, YccStudioG22LeftP709 = 8, YccFullG22LeftP709 = 9, YccStudioG22LeftP2020 = 10, YccFullG22LeftP2020 = 11, RgbFullG2084NoneP2020 = 12, YccStudioG2084LeftP2020 = 13, RgbStudioG2084NoneP2020 = 14, YccStudioG22TopLeftP2020 = 15, YccStudioG2084TopLeftP2020 = 16, RgbFullG22NoneP2020 = 17, YccStudioGHlgTopLeftP2020 = 18, YccFullGHlgTopLeftP2020 = 19, RgbStudioG24NoneP709 = 20, RgbStudioG24NoneP2020 = 21, YccStudioG24LeftP709 = 22, YccStudioG24LeftP2020 = 23, YccStudioG24TopLeftP2020 = 24,
}}
RT_ENUM! { enum DirectXPixelFormat: i32 {
    Unknown = 0, R32G32B32A32Typeless = 1, R32G32B32A32Float = 2, R32G32B32A32UInt = 3, R32G32B32A32Int = 4, R32G32B32Typeless = 5, R32G32B32Float = 6, R32G32B32UInt = 7, R32G32B32Int = 8, R16G16B16A16Typeless = 9, R16G16B16A16Float = 10, R16G16B16A16UIntNormalized = 11, R16G16B16A16UInt = 12, R16G16B16A16IntNormalized = 13, R16G16B16A16Int = 14, R32G32Typeless = 15, R32G32Float = 16, R32G32UInt = 17, R32G32Int = 18, R32G8X24Typeless = 19, D32FloatS8X24UInt = 20, R32FloatX8X24Typeless = 21, X32TypelessG8X24UInt = 22, R10G10B10A2Typeless = 23, R10G10B10A2UIntNormalized = 24, R10G10B10A2UInt = 25, R11G11B10Float = 26, R8G8B8A8Typeless = 27, R8G8B8A8UIntNormalized = 28, R8G8B8A8UIntNormalizedSrgb = 29, R8G8B8A8UInt = 30, R8G8B8A8IntNormalized = 31, R8G8B8A8Int = 32, R16G16Typeless = 33, R16G16Float = 34, R16G16UIntNormalized = 35, R16G16UInt = 36, R16G16IntNormalized = 37, R16G16Int = 38, R32Typeless = 39, D32Float = 40, R32Float = 41, R32UInt = 42, R32Int = 43, R24G8Typeless = 44, D24UIntNormalizedS8UInt = 45, R24UIntNormalizedX8Typeless = 46, X24TypelessG8UInt = 47, R8G8Typeless = 48, R8G8UIntNormalized = 49, R8G8UInt = 50, R8G8IntNormalized = 51, R8G8Int = 52, R16Typeless = 53, R16Float = 54, D16UIntNormalized = 55, R16UIntNormalized = 56, R16UInt = 57, R16IntNormalized = 58, R16Int = 59, R8Typeless = 60, R8UIntNormalized = 61, R8UInt = 62, R8IntNormalized = 63, R8Int = 64, A8UIntNormalized = 65, R1UIntNormalized = 66, R9G9B9E5SharedExponent = 67, R8G8B8G8UIntNormalized = 68, G8R8G8B8UIntNormalized = 69, BC1Typeless = 70, BC1UIntNormalized = 71, BC1UIntNormalizedSrgb = 72, BC2Typeless = 73, BC2UIntNormalized = 74, BC2UIntNormalizedSrgb = 75, BC3Typeless = 76, BC3UIntNormalized = 77, BC3UIntNormalizedSrgb = 78, BC4Typeless = 79, BC4UIntNormalized = 80, BC4IntNormalized = 81, BC5Typeless = 82, BC5UIntNormalized = 83, BC5IntNormalized = 84, B5G6R5UIntNormalized = 85, B5G5R5A1UIntNormalized = 86, B8G8R8A8UIntNormalized = 87, B8G8R8X8UIntNormalized = 88, R10G10B10XRBiasA2UIntNormalized = 89, B8G8R8A8Typeless = 90, B8G8R8A8UIntNormalizedSrgb = 91, B8G8R8X8Typeless = 92, B8G8R8X8UIntNormalizedSrgb = 93, BC6HTypeless = 94, BC6H16UnsignedFloat = 95, BC6H16Float = 96, BC7Typeless = 97, BC7UIntNormalized = 98, BC7UIntNormalizedSrgb = 99, Ayuv = 100, Y410 = 101, Y416 = 102, NV12 = 103, P010 = 104, P016 = 105, Opaque420 = 106, Yuy2 = 107, Y210 = 108, Y216 = 109, NV11 = 110, AI44 = 111, IA44 = 112, P8 = 113, A8P8 = 114, B4G4R4A4UIntNormalized = 115, P208 = 130, V208 = 131, V408 = 132,
}}
pub mod direct3d11 { // Windows.Graphics.DirectX.Direct3D11
use crate::prelude::*;
RT_ENUM! { enum Direct3DBindings: u32 {
    VertexBuffer = 1, IndexBuffer = 2, ConstantBuffer = 4, ShaderResource = 8, StreamOutput = 16, RenderTarget = 32, DepthStencil = 64, UnorderedAccess = 128, Decoder = 512, VideoEncoder = 1024,
}}
DEFINE_IID!(IID_IDirect3DDevice, 2742428843, 36191, 18000, 157, 62, 158, 174, 61, 155, 198, 112);
RT_INTERFACE!{interface IDirect3DDevice(IDirect3DDeviceVtbl): IInspectable(IInspectableVtbl) [IID_IDirect3DDevice] {
    fn Trim(&self) -> HRESULT
}}
impl IDirect3DDevice {
    #[inline] pub fn trim(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Trim)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_STRUCT! { struct Direct3DMultisampleDescription {
    Count: i32, Quality: i32,
}}
DEFINE_IID!(IID_IDirect3DSurface, 200581446, 5057, 18068, 190, 227, 122, 191, 21, 234, 245, 134);
RT_INTERFACE!{interface IDirect3DSurface(IDirect3DSurfaceVtbl): IInspectable(IInspectableVtbl) [IID_IDirect3DSurface] {
    fn get_Description(&self, out: *mut Direct3DSurfaceDescription) -> HRESULT
}}
impl IDirect3DSurface {
    #[inline] pub fn get_description(&self) -> Result<Direct3DSurfaceDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_STRUCT! { struct Direct3DSurfaceDescription {
    Width: i32, Height: i32, Format: super::DirectXPixelFormat, MultisampleDescription: Direct3DMultisampleDescription,
}}
RT_ENUM! { enum Direct3DUsage: i32 {
    Default = 0, Immutable = 1, Dynamic = 2, Staging = 3,
}}
} // Windows.Graphics.DirectX.Direct3D11
} // Windows.Graphics.DirectX
pub mod display { // Windows.Graphics.Display
use crate::prelude::*;
DEFINE_IID!(IID_IAdvancedColorInfo, 2274876667, 45609, 16513, 174, 154, 44, 200, 94, 52, 173, 106);
RT_INTERFACE!{interface IAdvancedColorInfo(IAdvancedColorInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAdvancedColorInfo] {
    fn get_CurrentAdvancedColorKind(&self, out: *mut AdvancedColorKind) -> HRESULT,
    fn get_RedPrimary(&self, out: *mut foundation::Point) -> HRESULT,
    fn get_GreenPrimary(&self, out: *mut foundation::Point) -> HRESULT,
    fn get_BluePrimary(&self, out: *mut foundation::Point) -> HRESULT,
    fn get_WhitePoint(&self, out: *mut foundation::Point) -> HRESULT,
    fn get_MaxLuminanceInNits(&self, out: *mut f32) -> HRESULT,
    fn get_MinLuminanceInNits(&self, out: *mut f32) -> HRESULT,
    fn get_MaxAverageFullFrameLuminanceInNits(&self, out: *mut f32) -> HRESULT,
    fn get_SdrWhiteLevelInNits(&self, out: *mut f32) -> HRESULT,
    fn IsHdrMetadataFormatCurrentlySupported(&self, format: HdrMetadataFormat, out: *mut bool) -> HRESULT,
    fn IsAdvancedColorKindAvailable(&self, kind: AdvancedColorKind, out: *mut bool) -> HRESULT
}}
impl IAdvancedColorInfo {
    #[inline] pub fn get_current_advanced_color_kind(&self) -> Result<AdvancedColorKind> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentAdvancedColorKind)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_red_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RedPrimary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_green_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_GreenPrimary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_blue_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BluePrimary)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_white_point(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WhitePoint)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxLuminanceInNits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinLuminanceInNits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_average_full_frame_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxAverageFullFrameLuminanceInNits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sdr_white_level_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SdrWhiteLevelInNits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_hdr_metadata_format_currently_supported(&self, format: HdrMetadataFormat) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsHdrMetadataFormatCurrentlySupported)(self as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_advanced_color_kind_available(&self, kind: AdvancedColorKind) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsAdvancedColorKindAvailable)(self as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AdvancedColorInfo: IAdvancedColorInfo}
RT_ENUM! { enum AdvancedColorKind: i32 {
    StandardDynamicRange = 0, WideColorGamut = 1, HighDynamicRange = 2,
}}
DEFINE_IID!(IID_IBrightnessOverride, 2529780250, 49475, 17298, 190, 221, 74, 126, 149, 116, 200, 253);
RT_INTERFACE!{interface IBrightnessOverride(IBrightnessOverrideVtbl): IInspectable(IInspectableVtbl) [IID_IBrightnessOverride] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsOverrideActive(&self, out: *mut bool) -> HRESULT,
    fn get_BrightnessLevel(&self, out: *mut f64) -> HRESULT,
    fn SetBrightnessLevel(&self, brightnessLevel: f64, options: DisplayBrightnessOverrideOptions) -> HRESULT,
    fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> HRESULT,
    fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario, out: *mut f64) -> HRESULT,
    fn StartOverride(&self) -> HRESULT,
    fn StopOverride(&self) -> HRESULT,
    fn add_IsSupportedChanged(&self, handler: *mut foundation::TypedEventHandler<BrightnessOverride, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsSupportedChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsOverrideActiveChanged(&self, handler: *mut foundation::TypedEventHandler<BrightnessOverride, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsOverrideActiveChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_BrightnessLevelChanged(&self, handler: *mut foundation::TypedEventHandler<BrightnessOverride, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BrightnessLevelChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IBrightnessOverride {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_override_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOverrideActive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_brightness_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BrightnessLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_level(&self, brightnessLevel: f64, options: DisplayBrightnessOverrideOptions) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetBrightnessLevel)(self as *const _ as *mut _, brightnessLevel, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_scenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetBrightnessScenario)(self as *const _ as *mut _, scenario, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_level_for_scenario(&self, scenario: DisplayBrightnessScenario) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetLevelForScenario)(self as *const _ as *mut _, scenario, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StartOverride)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StopOverride)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_supported_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsSupportedChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_supported_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_IsSupportedChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_override_active_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsOverrideActiveChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_override_active_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_IsOverrideActiveChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_brightness_level_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_BrightnessLevelChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_brightness_level_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_BrightnessLevelChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BrightnessOverride: IBrightnessOverride}
impl RtActivatable<IBrightnessOverrideStatics> for BrightnessOverride {}
impl BrightnessOverride {
    #[inline] pub fn get_default_for_system() -> Result<Option<ComPtr<BrightnessOverride>>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().get_default_for_system()
    }
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<BrightnessOverride>>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn save_for_system_async(value: &BrightnessOverride) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().save_for_system_async(value)
    }
}
DEFINE_CLSID!(BrightnessOverride(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,66,114,105,103,104,116,110,101,115,115,79,118,101,114,114,105,100,101,0]) [CLSID_BrightnessOverride]);
DEFINE_IID!(IID_IBrightnessOverrideSettings, 3507661610, 30212, 19898, 188, 248, 75, 111, 73, 80, 44, 176);
RT_INTERFACE!{interface IBrightnessOverrideSettings(IBrightnessOverrideSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideSettings] {
    fn get_DesiredLevel(&self, out: *mut f64) -> HRESULT,
    fn get_DesiredNits(&self, out: *mut f32) -> HRESULT
}}
impl IBrightnessOverrideSettings {
    #[inline] pub fn get_desired_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredLevel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_desired_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredNits)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BrightnessOverrideSettings: IBrightnessOverrideSettings}
impl RtActivatable<IBrightnessOverrideSettingsStatics> for BrightnessOverrideSettings {}
impl BrightnessOverrideSettings {
    #[inline] pub fn create_from_level(level: f64) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_level(level)
    }
    #[inline] pub fn create_from_nits(nits: f32) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_nits(nits)
    }
    #[inline] pub fn create_from_display_brightness_override_scenario(overrideScenario: DisplayBrightnessOverrideScenario) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_display_brightness_override_scenario(overrideScenario)
    }
}
DEFINE_CLSID!(BrightnessOverrideSettings(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,66,114,105,103,104,116,110,101,115,115,79,118,101,114,114,105,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_BrightnessOverrideSettings]);
DEFINE_IID!(IID_IBrightnessOverrideSettingsStatics, 3565673616, 28532, 17419, 179, 131, 95, 233, 108, 240, 11, 15);
RT_INTERFACE!{static interface IBrightnessOverrideSettingsStatics(IBrightnessOverrideSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideSettingsStatics] {
    fn CreateFromLevel(&self, level: f64, out: *mut *mut BrightnessOverrideSettings) -> HRESULT,
    fn CreateFromNits(&self, nits: f32, out: *mut *mut BrightnessOverrideSettings) -> HRESULT,
    fn CreateFromDisplayBrightnessOverrideScenario(&self, overrideScenario: DisplayBrightnessOverrideScenario, out: *mut *mut BrightnessOverrideSettings) -> HRESULT
}}
impl IBrightnessOverrideSettingsStatics {
    #[inline] pub fn create_from_level(&self, level: f64) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromLevel)(self as *const _ as *mut _, level, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_nits(&self, nits: f32) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromNits)(self as *const _ as *mut _, nits, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_display_brightness_override_scenario(&self, overrideScenario: DisplayBrightnessOverrideScenario) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromDisplayBrightnessOverrideScenario)(self as *const _ as *mut _, overrideScenario, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBrightnessOverrideStatics, 61323757, 57841, 19048, 161, 31, 148, 106, 216, 206, 83, 147);
RT_INTERFACE!{static interface IBrightnessOverrideStatics(IBrightnessOverrideStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideStatics] {
    fn GetDefaultForSystem(&self, out: *mut *mut BrightnessOverride) -> HRESULT,
    fn GetForCurrentView(&self, out: *mut *mut BrightnessOverride) -> HRESULT,
    fn SaveForSystemAsync(&self, value: *mut BrightnessOverride, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IBrightnessOverrideStatics {
    #[inline] pub fn get_default_for_system(&self) -> Result<Option<ComPtr<BrightnessOverride>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefaultForSystem)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<BrightnessOverride>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_for_system_async(&self, value: &BrightnessOverride) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveForSystemAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IColorOverrideSettings, 4226785588, 19073, 19533, 165, 182, 125, 27, 92, 75, 208, 11);
RT_INTERFACE!{interface IColorOverrideSettings(IColorOverrideSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IColorOverrideSettings] {
    fn get_DesiredDisplayColorOverrideScenario(&self, out: *mut DisplayColorOverrideScenario) -> HRESULT
}}
impl IColorOverrideSettings {
    #[inline] pub fn get_desired_display_color_override_scenario(&self) -> Result<DisplayColorOverrideScenario> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DesiredDisplayColorOverrideScenario)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ColorOverrideSettings: IColorOverrideSettings}
impl RtActivatable<IColorOverrideSettingsStatics> for ColorOverrideSettings {}
impl ColorOverrideSettings {
    #[inline] pub fn create_from_display_color_override_scenario(overrideScenario: DisplayColorOverrideScenario) -> Result<Option<ComPtr<ColorOverrideSettings>>> {
        <Self as RtActivatable<IColorOverrideSettingsStatics>>::get_activation_factory().create_from_display_color_override_scenario(overrideScenario)
    }
}
DEFINE_CLSID!(ColorOverrideSettings(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,67,111,108,111,114,79,118,101,114,114,105,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_ColorOverrideSettings]);
DEFINE_IID!(IID_IColorOverrideSettingsStatics, 2959663199, 50207, 19145, 175, 171, 130, 122, 182, 36, 143, 154);
RT_INTERFACE!{static interface IColorOverrideSettingsStatics(IColorOverrideSettingsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IColorOverrideSettingsStatics] {
    fn CreateFromDisplayColorOverrideScenario(&self, overrideScenario: DisplayColorOverrideScenario, out: *mut *mut ColorOverrideSettings) -> HRESULT
}}
impl IColorOverrideSettingsStatics {
    #[inline] pub fn create_from_display_color_override_scenario(&self, overrideScenario: DisplayColorOverrideScenario) -> Result<Option<ComPtr<ColorOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFromDisplayColorOverrideScenario)(self as *const _ as *mut _, overrideScenario, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum DisplayBrightnessOverrideOptions: u32 {
    None = 0, UseDimmedPolicyWhenBatteryIsLow = 1,
}}
RT_ENUM! { enum DisplayBrightnessOverrideScenario: i32 {
    IdleBrightness = 0, BarcodeReadingBrightness = 1, FullBrightness = 2,
}}
RT_ENUM! { enum DisplayBrightnessScenario: i32 {
    DefaultBrightness = 0, IdleBrightness = 1, BarcodeReadingBrightness = 2, FullBrightness = 3,
}}
RT_ENUM! { enum DisplayColorOverrideScenario: i32 {
    Accurate = 0,
}}
DEFINE_IID!(IID_IDisplayEnhancementOverride, 1117099215, 55674, 19202, 164, 40, 92, 66, 146, 247, 245, 34);
RT_INTERFACE!{interface IDisplayEnhancementOverride(IDisplayEnhancementOverrideVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverride] {
    fn get_ColorOverrideSettings(&self, out: *mut *mut ColorOverrideSettings) -> HRESULT,
    fn put_ColorOverrideSettings(&self, value: *mut ColorOverrideSettings) -> HRESULT,
    fn get_BrightnessOverrideSettings(&self, out: *mut *mut BrightnessOverrideSettings) -> HRESULT,
    fn put_BrightnessOverrideSettings(&self, value: *mut BrightnessOverrideSettings) -> HRESULT,
    fn get_CanOverride(&self, out: *mut bool) -> HRESULT,
    fn get_IsOverrideActive(&self, out: *mut bool) -> HRESULT,
    fn GetCurrentDisplayEnhancementOverrideCapabilities(&self, out: *mut *mut DisplayEnhancementOverrideCapabilities) -> HRESULT,
    fn RequestOverride(&self) -> HRESULT,
    fn StopOverride(&self) -> HRESULT,
    fn add_CanOverrideChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CanOverrideChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsOverrideActiveChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsOverrideActiveChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DisplayEnhancementOverrideCapabilitiesChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayEnhancementOverrideCapabilitiesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayEnhancementOverride {
    #[inline] pub fn get_color_override_settings(&self) -> Result<Option<ComPtr<ColorOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ColorOverrideSettings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_color_override_settings(&self, value: &ColorOverrideSettings) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ColorOverrideSettings)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_brightness_override_settings(&self) -> Result<Option<ComPtr<BrightnessOverrideSettings>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BrightnessOverrideSettings)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_override_settings(&self, value: &BrightnessOverrideSettings) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_BrightnessOverrideSettings)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_can_override(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanOverride)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_override_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOverrideActive)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_display_enhancement_override_capabilities(&self) -> Result<Option<ComPtr<DisplayEnhancementOverrideCapabilities>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentDisplayEnhancementOverrideCapabilities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).RequestOverride)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).StopOverride)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_can_override_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CanOverrideChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_can_override_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_CanOverrideChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_override_active_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsOverrideActiveChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_override_active_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_IsOverrideActiveChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_enhancement_override_capabilities_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DisplayEnhancementOverrideCapabilitiesChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_enhancement_override_capabilities_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DisplayEnhancementOverrideCapabilitiesChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverride: IDisplayEnhancementOverride}
impl RtActivatable<IDisplayEnhancementOverrideStatics> for DisplayEnhancementOverride {}
impl DisplayEnhancementOverride {
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<DisplayEnhancementOverride>>> {
        <Self as RtActivatable<IDisplayEnhancementOverrideStatics>>::get_activation_factory().get_for_current_view()
    }
}
DEFINE_CLSID!(DisplayEnhancementOverride(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,69,110,104,97,110,99,101,109,101,110,116,79,118,101,114,114,105,100,101,0]) [CLSID_DisplayEnhancementOverride]);
DEFINE_IID!(IID_IDisplayEnhancementOverrideCapabilities, 1164992734, 61018, 18359, 153, 24, 30, 81, 232, 18, 204, 200);
RT_INTERFACE!{interface IDisplayEnhancementOverrideCapabilities(IDisplayEnhancementOverrideCapabilitiesVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideCapabilities] {
    fn get_IsBrightnessControlSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsBrightnessNitsControlSupported(&self, out: *mut bool) -> HRESULT,
    fn GetSupportedNitRanges(&self, out: *mut *mut foundation::collections::IVectorView<NitRange>) -> HRESULT
}}
impl IDisplayEnhancementOverrideCapabilities {
    #[inline] pub fn get_is_brightness_control_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsBrightnessControlSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_brightness_nits_control_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsBrightnessNitsControlSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_nit_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<NitRange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSupportedNitRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverrideCapabilities: IDisplayEnhancementOverrideCapabilities}
DEFINE_IID!(IID_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs, 3680626276, 5626, 18906, 139, 119, 7, 219, 210, 175, 88, 93);
RT_INTERFACE!{interface IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(IDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs] {
    fn get_Capabilities(&self, out: *mut *mut DisplayEnhancementOverrideCapabilities) -> HRESULT
}}
impl IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    #[inline] pub fn get_capabilities(&self) -> Result<Option<ComPtr<DisplayEnhancementOverrideCapabilities>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Capabilities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverrideCapabilitiesChangedEventArgs: IDisplayEnhancementOverrideCapabilitiesChangedEventArgs}
DEFINE_IID!(IID_IDisplayEnhancementOverrideStatics, 3478879937, 38801, 17491, 176, 19, 41, 182, 247, 120, 229, 25);
RT_INTERFACE!{static interface IDisplayEnhancementOverrideStatics(IDisplayEnhancementOverrideStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideStatics] {
    fn GetForCurrentView(&self, out: *mut *mut DisplayEnhancementOverride) -> HRESULT
}}
impl IDisplayEnhancementOverrideStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<DisplayEnhancementOverride>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation, 3201372846, 44483, 19913, 174, 101, 133, 31, 77, 125, 71, 153);
RT_INTERFACE!{interface IDisplayInformation(IDisplayInformationVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformation] {
    fn get_CurrentOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_NativeOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn add_OrientationChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OrientationChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_ResolutionScale(&self, out: *mut ResolutionScale) -> HRESULT,
    fn get_LogicalDpi(&self, out: *mut f32) -> HRESULT,
    fn get_RawDpiX(&self, out: *mut f32) -> HRESULT,
    fn get_RawDpiY(&self, out: *mut f32) -> HRESULT,
    fn add_DpiChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DpiChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn add_StereoEnabledChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StereoEnabledChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetColorProfileAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>) -> HRESULT,
    fn add_ColorProfileChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ColorProfileChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformation {
    #[inline] pub fn get_current_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentOrientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_native_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NativeOrientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_orientation_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OrientationChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_orientation_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_OrientationChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_scale(&self) -> Result<ResolutionScale> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResolutionScale)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_logical_dpi(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LogicalDpi)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_dpi_x(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RawDpiX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_dpi_y(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RawDpiY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_dpi_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DpiChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_dpi_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DpiChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StereoEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_stereo_enabled_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StereoEnabledChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stereo_enabled_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_StereoEnabledChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetColorProfileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_color_profile_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ColorProfileChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_color_profile_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ColorProfileChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayInformation: IDisplayInformation}
impl RtActivatable<IDisplayInformationStatics> for DisplayInformation {}
impl DisplayInformation {
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<DisplayInformation>>> {
        <Self as RtActivatable<IDisplayInformationStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn get_auto_rotation_preferences() -> Result<DisplayOrientations> {
        <Self as RtActivatable<IDisplayInformationStatics>>::get_activation_factory().get_auto_rotation_preferences()
    }
    #[inline] pub fn set_auto_rotation_preferences(value: DisplayOrientations) -> Result<()> {
        <Self as RtActivatable<IDisplayInformationStatics>>::get_activation_factory().set_auto_rotation_preferences(value)
    }
    #[inline] pub fn add_display_contents_invalidated(handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayInformationStatics>>::get_activation_factory().add_display_contents_invalidated(handler)
    }
    #[inline] pub fn remove_display_contents_invalidated(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayInformationStatics>>::get_activation_factory().remove_display_contents_invalidated(token)
    }
}
DEFINE_CLSID!(DisplayInformation(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_DisplayInformation]);
DEFINE_IID!(IID_IDisplayInformation2, 1305280545, 64209, 19342, 142, 223, 119, 88, 135, 184, 191, 25);
RT_INTERFACE!{interface IDisplayInformation2(IDisplayInformation2Vtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformation2] {
    fn get_RawPixelsPerViewPixel(&self, out: *mut f64) -> HRESULT
}}
impl IDisplayInformation2 {
    #[inline] pub fn get_raw_pixels_per_view_pixel(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RawPixelsPerViewPixel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation3, 3675586845, 3849, 17510, 143, 243, 17, 222, 154, 60, 146, 154);
RT_INTERFACE!{interface IDisplayInformation3(IDisplayInformation3Vtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformation3] {
    fn get_DiagonalSizeInInches(&self, out: *mut *mut foundation::IReference<f64>) -> HRESULT
}}
impl IDisplayInformation3 {
    #[inline] pub fn get_diagonal_size_in_inches(&self) -> Result<Option<ComPtr<foundation::IReference<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DiagonalSizeInInches)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation4, 3379744303, 4674, 18110, 181, 54, 225, 170, 254, 158, 122, 207);
RT_INTERFACE!{interface IDisplayInformation4(IDisplayInformation4Vtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformation4] {
    fn get_ScreenWidthInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_ScreenHeightInRawPixels(&self, out: *mut u32) -> HRESULT
}}
impl IDisplayInformation4 {
    #[inline] pub fn get_screen_width_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScreenWidthInRawPixels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_screen_height_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScreenHeightInRawPixels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation5, 978600668, 11486, 19085, 128, 209, 33, 220, 90, 220, 193, 170);
RT_INTERFACE!{interface IDisplayInformation5(IDisplayInformation5Vtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformation5] {
    fn GetAdvancedColorInfo(&self, out: *mut *mut AdvancedColorInfo) -> HRESULT,
    fn add_AdvancedColorInfoChanged(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AdvancedColorInfoChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformation5 {
    #[inline] pub fn get_advanced_color_info(&self) -> Result<Option<ComPtr<AdvancedColorInfo>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetAdvancedColorInfo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_advanced_color_info_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_AdvancedColorInfoChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_advanced_color_info_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_AdvancedColorInfoChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformationStatics, 3332385388, 54354, 17628, 186, 7, 150, 243, 198, 173, 249, 209);
RT_INTERFACE!{static interface IDisplayInformationStatics(IDisplayInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayInformationStatics] {
    fn GetForCurrentView(&self, out: *mut *mut DisplayInformation) -> HRESULT,
    fn get_AutoRotationPreferences(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn put_AutoRotationPreferences(&self, value: DisplayOrientations) -> HRESULT,
    fn add_DisplayContentsInvalidated(&self, handler: *mut foundation::TypedEventHandler<DisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayContentsInvalidated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformationStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<DisplayInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_auto_rotation_preferences(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AutoRotationPreferences)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_auto_rotation_preferences(&self, value: DisplayOrientations) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AutoRotationPreferences)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_contents_invalidated(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DisplayContentsInvalidated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_contents_invalidated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DisplayContentsInvalidated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum DisplayOrientations: u32 {
    None = 0, Landscape = 1, Portrait = 2, LandscapeFlipped = 4, PortraitFlipped = 8,
}}
RT_CLASS!{static class DisplayProperties}
impl RtActivatable<IDisplayPropertiesStatics> for DisplayProperties {}
impl DisplayProperties {
    #[inline] pub fn get_current_orientation() -> Result<DisplayOrientations> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_current_orientation()
    }
    #[inline] pub fn get_native_orientation() -> Result<DisplayOrientations> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_native_orientation()
    }
    #[inline] pub fn get_auto_rotation_preferences() -> Result<DisplayOrientations> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_auto_rotation_preferences()
    }
    #[inline] pub fn set_auto_rotation_preferences(value: DisplayOrientations) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().set_auto_rotation_preferences(value)
    }
    #[inline] pub fn add_orientation_changed(handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().add_orientation_changed(handler)
    }
    #[inline] pub fn remove_orientation_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().remove_orientation_changed(token)
    }
    #[inline] pub fn get_resolution_scale() -> Result<ResolutionScale> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_resolution_scale()
    }
    #[inline] pub fn get_logical_dpi() -> Result<f32> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_logical_dpi()
    }
    #[inline] pub fn add_logical_dpi_changed(handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().add_logical_dpi_changed(handler)
    }
    #[inline] pub fn remove_logical_dpi_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().remove_logical_dpi_changed(token)
    }
    #[inline] pub fn get_stereo_enabled() -> Result<bool> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_stereo_enabled()
    }
    #[inline] pub fn add_stereo_enabled_changed(handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().add_stereo_enabled_changed(handler)
    }
    #[inline] pub fn remove_stereo_enabled_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().remove_stereo_enabled_changed(token)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async() -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>>> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().get_color_profile_async()
    }
    #[inline] pub fn add_color_profile_changed(handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().add_color_profile_changed(handler)
    }
    #[inline] pub fn remove_color_profile_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().remove_color_profile_changed(token)
    }
    #[inline] pub fn add_display_contents_invalidated(handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().add_display_contents_invalidated(handler)
    }
    #[inline] pub fn remove_display_contents_invalidated(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IDisplayPropertiesStatics>>::get_activation_factory().remove_display_contents_invalidated(token)
    }
}
DEFINE_CLSID!(DisplayProperties(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,80,114,111,112,101,114,116,105,101,115,0]) [CLSID_DisplayProperties]);
DEFINE_IID!(IID_DisplayPropertiesEventHandler, 3688729345, 61857, 18129, 158, 227, 84, 59, 204, 153, 89, 128);
RT_DELEGATE!{delegate DisplayPropertiesEventHandler(DisplayPropertiesEventHandlerVtbl, DisplayPropertiesEventHandlerImpl) [IID_DisplayPropertiesEventHandler] {
    fn Invoke(&self, sender: *mut IInspectable) -> HRESULT
}}
impl DisplayPropertiesEventHandler {
    #[inline] pub fn invoke(&self, sender: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, sender as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayPropertiesStatics, 1765272973, 12522, 19949, 130, 113, 69, 83, 255, 2, 246, 138);
RT_INTERFACE!{static interface IDisplayPropertiesStatics(IDisplayPropertiesStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IDisplayPropertiesStatics] {
    fn get_CurrentOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_NativeOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_AutoRotationPreferences(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn put_AutoRotationPreferences(&self, value: DisplayOrientations) -> HRESULT,
    fn add_OrientationChanged(&self, handler: *mut DisplayPropertiesEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OrientationChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_ResolutionScale(&self, out: *mut ResolutionScale) -> HRESULT,
    fn get_LogicalDpi(&self, out: *mut f32) -> HRESULT,
    fn add_LogicalDpiChanged(&self, handler: *mut DisplayPropertiesEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LogicalDpiChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn add_StereoEnabledChanged(&self, handler: *mut DisplayPropertiesEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StereoEnabledChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetColorProfileAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>) -> HRESULT,
    fn add_ColorProfileChanged(&self, handler: *mut DisplayPropertiesEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ColorProfileChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DisplayContentsInvalidated(&self, handler: *mut DisplayPropertiesEventHandler, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayContentsInvalidated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayPropertiesStatics {
    #[inline] pub fn get_current_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CurrentOrientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_native_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NativeOrientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_auto_rotation_preferences(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AutoRotationPreferences)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_auto_rotation_preferences(&self, value: DisplayOrientations) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AutoRotationPreferences)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_orientation_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OrientationChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_orientation_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_OrientationChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_scale(&self) -> Result<ResolutionScale> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResolutionScale)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_logical_dpi(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LogicalDpi)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_logical_dpi_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_LogicalDpiChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_logical_dpi_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_LogicalDpiChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StereoEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_stereo_enabled_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_StereoEnabledChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stereo_enabled_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_StereoEnabledChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetColorProfileAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_color_profile_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_ColorProfileChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_color_profile_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_ColorProfileChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_contents_invalidated(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DisplayContentsInvalidated)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_contents_invalidated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DisplayContentsInvalidated)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum HdrMetadataFormat: i32 {
    Hdr10 = 0, Hdr10Plus = 1,
}}
RT_STRUCT! { struct NitRange {
    MinNits: f32, MaxNits: f32, StepSizeNits: f32,
}}
RT_ENUM! { enum ResolutionScale: i32 {
    Invalid = 0, Scale100Percent = 100, Scale120Percent = 120, Scale125Percent = 125, Scale140Percent = 140, Scale150Percent = 150, Scale160Percent = 160, Scale175Percent = 175, Scale180Percent = 180, Scale200Percent = 200, Scale225Percent = 225, Scale250Percent = 250, Scale300Percent = 300, Scale350Percent = 350, Scale400Percent = 400, Scale450Percent = 450, Scale500Percent = 500,
}}
pub mod core { // Windows.Graphics.Display.Core
use crate::prelude::*;
RT_ENUM! { enum HdmiDisplayColorSpace: i32 {
    RgbLimited = 0, RgbFull = 1, BT2020 = 2, BT709 = 3,
}}
RT_STRUCT! { struct HdmiDisplayHdr2086Metadata {
    RedPrimaryX: u16, RedPrimaryY: u16, GreenPrimaryX: u16, GreenPrimaryY: u16, BluePrimaryX: u16, BluePrimaryY: u16, WhitePointX: u16, WhitePointY: u16, MaxMasteringLuminance: u16, MinMasteringLuminance: u16, MaxContentLightLevel: u16, MaxFrameAverageLightLevel: u16,
}}
RT_ENUM! { enum HdmiDisplayHdrOption: i32 {
    None = 0, EotfSdr = 1, Eotf2084 = 2, DolbyVisionLowLatency = 3,
}}
DEFINE_IID!(IID_IHdmiDisplayInformation, 319503370, 62821, 18286, 171, 213, 234, 5, 174, 231, 76, 105);
RT_INTERFACE!{interface IHdmiDisplayInformation(IHdmiDisplayInformationVtbl): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayInformation] {
    fn GetSupportedDisplayModes(&self, out: *mut *mut foundation::collections::IVectorView<HdmiDisplayMode>) -> HRESULT,
    fn GetCurrentDisplayMode(&self, out: *mut *mut HdmiDisplayMode) -> HRESULT,
    fn SetDefaultDisplayModeAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn RequestSetCurrentDisplayModeAsync(&self, mode: *mut HdmiDisplayMode, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestSetCurrentDisplayModeWithHdrAsync(&self, mode: *mut HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&self, mode: *mut HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption, hdrMetadata: HdmiDisplayHdr2086Metadata, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn add_DisplayModesChanged(&self, value: *mut foundation::TypedEventHandler<HdmiDisplayInformation, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayModesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IHdmiDisplayInformation {
    #[inline] pub fn get_supported_display_modes(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HdmiDisplayMode>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSupportedDisplayModes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_display_mode(&self) -> Result<Option<ComPtr<HdmiDisplayMode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCurrentDisplayMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_default_display_mode_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetDefaultDisplayModeAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_async(&self, mode: &HdmiDisplayMode) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSetCurrentDisplayModeAsync)(self as *const _ as *mut _, mode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_with_hdr_async(&self, mode: &HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSetCurrentDisplayModeWithHdrAsync)(self as *const _ as *mut _, mode as *const _ as *mut _, hdrOption, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_with_hdr_and_metadata_async(&self, mode: &HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption, hdrMetadata: HdmiDisplayHdr2086Metadata) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(self as *const _ as *mut _, mode as *const _ as *mut _, hdrOption, hdrMetadata, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_display_modes_changed(&self, value: &foundation::TypedEventHandler<HdmiDisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_DisplayModesChanged)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_modes_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_DisplayModesChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HdmiDisplayInformation: IHdmiDisplayInformation}
impl RtActivatable<IHdmiDisplayInformationStatics> for HdmiDisplayInformation {}
impl HdmiDisplayInformation {
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<HdmiDisplayInformation>>> {
        <Self as RtActivatable<IHdmiDisplayInformationStatics>>::get_activation_factory().get_for_current_view()
    }
}
DEFINE_CLSID!(HdmiDisplayInformation(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,67,111,114,101,46,72,100,109,105,68,105,115,112,108,97,121,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_HdmiDisplayInformation]);
DEFINE_IID!(IID_IHdmiDisplayInformationStatics, 1827058272, 62506, 18965, 145, 76, 123, 142, 42, 90, 101, 223);
RT_INTERFACE!{static interface IHdmiDisplayInformationStatics(IHdmiDisplayInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayInformationStatics] {
    fn GetForCurrentView(&self, out: *mut *mut HdmiDisplayInformation) -> HRESULT
}}
impl IHdmiDisplayInformationStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<HdmiDisplayInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHdmiDisplayMode, 201774509, 7056, 20305, 153, 129, 239, 90, 28, 13, 223, 102);
RT_INTERFACE!{interface IHdmiDisplayMode(IHdmiDisplayModeVtbl): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayMode] {
    fn get_ResolutionWidthInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_ResolutionHeightInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_RefreshRate(&self, out: *mut f64) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_BitsPerPixel(&self, out: *mut u16) -> HRESULT,
    fn IsEqual(&self, mode: *mut HdmiDisplayMode, out: *mut bool) -> HRESULT,
    fn get_ColorSpace(&self, out: *mut HdmiDisplayColorSpace) -> HRESULT,
    fn get_PixelEncoding(&self, out: *mut HdmiDisplayPixelEncoding) -> HRESULT,
    fn get_IsSdrLuminanceSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsSmpte2084Supported(&self, out: *mut bool) -> HRESULT,
    fn get_Is2086MetadataSupported(&self, out: *mut bool) -> HRESULT
}}
impl IHdmiDisplayMode {
    #[inline] pub fn get_resolution_width_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResolutionWidthInRawPixels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_height_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ResolutionHeightInRawPixels)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_refresh_rate(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RefreshRate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_StereoEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bits_per_pixel(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitsPerPixel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_equal(&self, mode: &HdmiDisplayMode) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsEqual)(self as *const _ as *mut _, mode as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_color_space(&self) -> Result<HdmiDisplayColorSpace> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ColorSpace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_encoding(&self) -> Result<HdmiDisplayPixelEncoding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelEncoding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_sdr_luminance_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSdrLuminanceSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_smpte_2084_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSmpte2084Supported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_2086_metadata_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Is2086MetadataSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HdmiDisplayMode: IHdmiDisplayMode}
DEFINE_IID!(IID_IHdmiDisplayMode2, 130895519, 19260, 17080, 132, 231, 137, 83, 104, 113, 138, 242);
RT_INTERFACE!{interface IHdmiDisplayMode2(IHdmiDisplayMode2Vtbl): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayMode2] {
    fn get_IsDolbyVisionLowLatencySupported(&self, out: *mut bool) -> HRESULT
}}
impl IHdmiDisplayMode2 {
    #[inline] pub fn get_is_dolby_vision_low_latency_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsDolbyVisionLowLatencySupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum HdmiDisplayPixelEncoding: i32 {
    Rgb444 = 0, Ycc444 = 1, Ycc422 = 2, Ycc420 = 3,
}}
} // Windows.Graphics.Display.Core
} // Windows.Graphics.Display
pub mod effects { // Windows.Graphics.Effects
use crate::prelude::*;
DEFINE_IID!(IID_IGraphicsEffect, 3411132622, 36838, 17974, 178, 2, 134, 31, 170, 7, 216, 243);
RT_INTERFACE!{interface IGraphicsEffect(IGraphicsEffectVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsEffect] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, name: HSTRING) -> HRESULT
}}
impl IGraphicsEffect {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, name: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsEffectSource, 764386780, 17209, 20153, 146, 22, 249, 222, 183, 86, 88, 162);
RT_INTERFACE!{interface IGraphicsEffectSource(IGraphicsEffectSourceVtbl): IInspectable(IInspectableVtbl) [IID_IGraphicsEffectSource] {
    
}}
} // Windows.Graphics.Effects
pub mod holographic { // Windows.Graphics.Holographic
use crate::prelude::*;
RT_STRUCT! { struct HolographicAdapterId {
    LowPart: u32, HighPart: i32,
}}
DEFINE_IID!(IID_IHolographicCamera, 3840508997, 39917, 18816, 155, 160, 232, 118, 128, 209, 203, 116);
RT_INTERFACE!{interface IHolographicCamera(IHolographicCameraVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCamera] {
    fn get_RenderTargetSize(&self, out: *mut foundation::Size) -> HRESULT,
    fn get_ViewportScaleFactor(&self, out: *mut f64) -> HRESULT,
    fn put_ViewportScaleFactor(&self, value: f64) -> HRESULT,
    fn get_IsStereo(&self, out: *mut bool) -> HRESULT,
    fn get_Id(&self, out: *mut u32) -> HRESULT,
    fn SetNearPlaneDistance(&self, value: f64) -> HRESULT,
    fn SetFarPlaneDistance(&self, value: f64) -> HRESULT
}}
impl IHolographicCamera {
    #[inline] pub fn get_render_target_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RenderTargetSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_viewport_scale_factor(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ViewportScaleFactor)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_viewport_scale_factor(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ViewportScaleFactor)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_stereo(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsStereo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Id)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_near_plane_distance(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetNearPlaneDistance)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_far_plane_distance(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetFarPlaneDistance)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCamera: IHolographicCamera}
DEFINE_IID!(IID_IHolographicCamera2, 3042680602, 47756, 20356, 173, 121, 46, 126, 30, 36, 80, 243);
RT_INTERFACE!{interface IHolographicCamera2(IHolographicCamera2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCamera2] {
    fn get_LeftViewportParameters(&self, out: *mut *mut HolographicCameraViewportParameters) -> HRESULT,
    fn get_RightViewportParameters(&self, out: *mut *mut HolographicCameraViewportParameters) -> HRESULT,
    fn get_Display(&self, out: *mut *mut HolographicDisplay) -> HRESULT
}}
impl IHolographicCamera2 {
    #[inline] pub fn get_left_viewport_parameters(&self) -> Result<Option<ComPtr<HolographicCameraViewportParameters>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_LeftViewportParameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_right_viewport_parameters(&self) -> Result<Option<ComPtr<HolographicCameraViewportParameters>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RightViewportParameters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display(&self) -> Result<Option<ComPtr<HolographicDisplay>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Display)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera3, 1168789427, 31577, 21070, 74, 63, 74, 106, 214, 101, 4, 119);
RT_INTERFACE!{interface IHolographicCamera3(IHolographicCamera3Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCamera3] {
    fn get_IsPrimaryLayerEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsPrimaryLayerEnabled(&self, value: bool) -> HRESULT,
    fn get_MaxQuadLayerCount(&self, out: *mut u32) -> HRESULT,
    fn get_QuadLayers(&self, out: *mut *mut foundation::collections::IVector<HolographicQuadLayer>) -> HRESULT
}}
impl IHolographicCamera3 {
    #[inline] pub fn get_is_primary_layer_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPrimaryLayerEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_primary_layer_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsPrimaryLayerEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_quad_layer_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxQuadLayerCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_quad_layers(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HolographicQuadLayer>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_QuadLayers)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera4, 2586128854, 18211, 20281, 169, 165, 157, 5, 24, 29, 155, 68);
RT_INTERFACE!{interface IHolographicCamera4(IHolographicCamera4Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCamera4] {
    fn get_CanOverrideViewport(&self, out: *mut bool) -> HRESULT
}}
impl IHolographicCamera4 {
    #[inline] pub fn get_can_override_viewport(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanOverrideViewport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera5, 580323058, 25229, 20213, 156, 8, 166, 63, 221, 119, 135, 198);
RT_INTERFACE!{interface IHolographicCamera5(IHolographicCamera5Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCamera5] {
    fn get_IsHardwareContentProtectionSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsHardwareContentProtectionEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsHardwareContentProtectionEnabled(&self, value: bool) -> HRESULT
}}
impl IHolographicCamera5 {
    #[inline] pub fn get_is_hardware_content_protection_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsHardwareContentProtectionSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_hardware_content_protection_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsHardwareContentProtectionEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_hardware_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsHardwareContentProtectionEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraPose, 226328112, 4830, 17853, 145, 43, 199, 246, 86, 21, 153, 209);
RT_INTERFACE!{interface IHolographicCameraPose(IHolographicCameraPoseVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraPose] {
    fn get_HolographicCamera(&self, out: *mut *mut HolographicCamera) -> HRESULT,
    fn get_Viewport(&self, out: *mut foundation::Rect) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetViewTransform(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, out: *mut *mut foundation::IReference<HolographicStereoTransform>) -> HRESULT,
    fn get_ProjectionTransform(&self, out: *mut HolographicStereoTransform) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetCullingFrustum(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, out: *mut *mut foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetVisibleFrustum(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, out: *mut *mut foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>) -> HRESULT,
    fn get_NearPlaneDistance(&self, out: *mut f64) -> HRESULT,
    fn get_FarPlaneDistance(&self, out: *mut f64) -> HRESULT
}}
impl IHolographicCameraPose {
    #[inline] pub fn get_holographic_camera(&self) -> Result<Option<ComPtr<HolographicCamera>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HolographicCamera)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_viewport(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Viewport)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_view_transform(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<ComPtr<foundation::IReference<HolographicStereoTransform>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetViewTransform)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_projection_transform(&self) -> Result<HolographicStereoTransform> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ProjectionTransform)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_culling_frustum(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<ComPtr<foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetCullingFrustum)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_visible_frustum(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<ComPtr<foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryGetVisibleFrustum)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_near_plane_distance(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NearPlaneDistance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_far_plane_distance(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FarPlaneDistance)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraPose: IHolographicCameraPose}
DEFINE_IID!(IID_IHolographicCameraPose2, 590078067, 23853, 17760, 129, 78, 38, 151, 196, 252, 225, 107);
RT_INTERFACE!{interface IHolographicCameraPose2(IHolographicCameraPose2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraPose2] {
    #[cfg(not(feature="windows-perception"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-perception")] fn OverrideViewTransform(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, coordinateSystemToViewTransform: HolographicStereoTransform) -> HRESULT,
    fn OverrideProjectionTransform(&self, projectionTransform: HolographicStereoTransform) -> HRESULT,
    fn OverrideViewport(&self, leftViewport: foundation::Rect, rightViewport: foundation::Rect) -> HRESULT
}}
impl IHolographicCameraPose2 {
    #[cfg(feature="windows-perception")] #[inline] pub fn override_view_transform(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, coordinateSystemToViewTransform: HolographicStereoTransform) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).OverrideViewTransform)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, coordinateSystemToViewTransform);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn override_projection_transform(&self, projectionTransform: HolographicStereoTransform) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).OverrideProjectionTransform)(self as *const _ as *mut _, projectionTransform);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn override_viewport(&self, leftViewport: foundation::Rect, rightViewport: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).OverrideViewport)(self as *const _ as *mut _, leftViewport, rightViewport);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters, 2393648849, 23540, 19990, 130, 54, 174, 8, 0, 193, 29, 13);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters(IHolographicCameraRenderingParametersVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters] {
    #[cfg(not(feature="windows-perception"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPoint(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPointWithNormal(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPointWithNormalLinearVelocity(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3, linearVelocity: foundation::numerics::Vector3) -> HRESULT,
    fn get_Direct3D11Device(&self, out: *mut *mut super::directx::direct3d11::IDirect3DDevice) -> HRESULT,
    fn get_Direct3D11BackBuffer(&self, out: *mut *mut super::directx::direct3d11::IDirect3DSurface) -> HRESULT
}}
impl IHolographicCameraRenderingParameters {
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetFocusPoint)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point_with_normal(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetFocusPointWithNormal)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position, normal);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point_with_normal_linear_velocity(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3, linearVelocity: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetFocusPointWithNormalLinearVelocity)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position, normal, linearVelocity);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_direct3d11_device(&self) -> Result<Option<ComPtr<super::directx::direct3d11::IDirect3DDevice>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Direct3D11Device)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_direct3d11_back_buffer(&self) -> Result<Option<ComPtr<super::directx::direct3d11::IDirect3DSurface>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Direct3D11BackBuffer)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraRenderingParameters: IHolographicCameraRenderingParameters}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters2, 638742755, 46742, 17972, 148, 214, 190, 6, 129, 100, 53, 153);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters2(IHolographicCameraRenderingParameters2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters2] {
    fn get_ReprojectionMode(&self, out: *mut HolographicReprojectionMode) -> HRESULT,
    fn put_ReprojectionMode(&self, value: HolographicReprojectionMode) -> HRESULT,
    fn CommitDirect3D11DepthBuffer(&self, value: *mut super::directx::direct3d11::IDirect3DSurface) -> HRESULT
}}
impl IHolographicCameraRenderingParameters2 {
    #[inline] pub fn get_reprojection_mode(&self) -> Result<HolographicReprojectionMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ReprojectionMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_reprojection_mode(&self, value: HolographicReprojectionMode) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ReprojectionMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn commit_direct3d11_depth_buffer(&self, value: &super::directx::direct3d11::IDirect3DSurface) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CommitDirect3D11DepthBuffer)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters3, 2980729151, 4973, 19206, 185, 212, 228, 185, 20, 205, 6, 131);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters3(IHolographicCameraRenderingParameters3Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters3] {
    fn get_IsContentProtectionEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsContentProtectionEnabled(&self, value: bool) -> HRESULT
}}
impl IHolographicCameraRenderingParameters3 {
    #[inline] pub fn get_is_content_protection_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsContentProtectionEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsContentProtectionEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraViewportParameters, 2160980983, 33834, 16865, 147, 237, 86, 146, 171, 31, 187, 16);
RT_INTERFACE!{interface IHolographicCameraViewportParameters(IHolographicCameraViewportParametersVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicCameraViewportParameters] {
    fn get_HiddenAreaMesh(&self, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector2) -> HRESULT,
    fn get_VisibleAreaMesh(&self, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector2) -> HRESULT
}}
impl IHolographicCameraViewportParameters {
    #[inline] pub fn get_hidden_area_mesh(&self) -> Result<ComArray<foundation::numerics::Vector2>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HiddenAreaMesh)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_visible_area_mesh(&self) -> Result<ComArray<foundation::numerics::Vector2>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_VisibleAreaMesh)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraViewportParameters: IHolographicCameraViewportParameters}
DEFINE_IID!(IID_IHolographicDisplay, 2597233684, 7583, 16528, 163, 136, 144, 192, 111, 110, 174, 156);
RT_INTERFACE!{interface IHolographicDisplay(IHolographicDisplayVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicDisplay] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MaxViewportSize(&self, out: *mut foundation::Size) -> HRESULT,
    fn get_IsStereo(&self, out: *mut bool) -> HRESULT,
    fn get_IsOpaque(&self, out: *mut bool) -> HRESULT,
    fn get_AdapterId(&self, out: *mut HolographicAdapterId) -> HRESULT,
    #[cfg(feature="windows-perception")] fn get_SpatialLocator(&self, out: *mut *mut super::super::perception::spatial::SpatialLocator) -> HRESULT
}}
impl IHolographicDisplay {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_viewport_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxViewportSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_stereo(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsStereo)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_opaque(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsOpaque)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_adapter_id(&self) -> Result<HolographicAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AdapterId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn get_spatial_locator(&self) -> Result<Option<ComPtr<super::super::perception::spatial::SpatialLocator>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SpatialLocator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicDisplay: IHolographicDisplay}
impl RtActivatable<IHolographicDisplayStatics> for HolographicDisplay {}
impl HolographicDisplay {
    #[inline] pub fn get_default() -> Result<Option<ComPtr<HolographicDisplay>>> {
        <Self as RtActivatable<IHolographicDisplayStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(HolographicDisplay(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,72,111,108,111,103,114,97,112,104,105,99,46,72,111,108,111,103,114,97,112,104,105,99,68,105,115,112,108,97,121,0]) [CLSID_HolographicDisplay]);
DEFINE_IID!(IID_IHolographicDisplay2, 1974222722, 59221, 17260, 141, 150, 77, 50, 209, 49, 71, 62);
RT_INTERFACE!{interface IHolographicDisplay2(IHolographicDisplay2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicDisplay2] {
    fn get_RefreshRate(&self, out: *mut f64) -> HRESULT
}}
impl IHolographicDisplay2 {
    #[inline] pub fn get_refresh_rate(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RefreshRate)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicDisplayStatics, 3409398147, 59312, 18497, 131, 85, 58, 229, 181, 54, 233, 164);
RT_INTERFACE!{static interface IHolographicDisplayStatics(IHolographicDisplayStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicDisplayStatics] {
    fn GetDefault(&self, out: *mut *mut HolographicDisplay) -> HRESULT
}}
impl IHolographicDisplayStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<ComPtr<HolographicDisplay>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDefault)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicFrame, 3331886774, 43193, 12372, 166, 235, 214, 36, 182, 83, 99, 117);
RT_INTERFACE!{interface IHolographicFrame(IHolographicFrameVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicFrame] {
    fn get_AddedCameras(&self, out: *mut *mut foundation::collections::IVectorView<HolographicCamera>) -> HRESULT,
    fn get_RemovedCameras(&self, out: *mut *mut foundation::collections::IVectorView<HolographicCamera>) -> HRESULT,
    fn GetRenderingParameters(&self, cameraPose: *mut HolographicCameraPose, out: *mut *mut HolographicCameraRenderingParameters) -> HRESULT,
    fn get_Duration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_CurrentPrediction(&self, out: *mut *mut HolographicFramePrediction) -> HRESULT,
    fn UpdateCurrentPrediction(&self) -> HRESULT,
    fn PresentUsingCurrentPrediction(&self, out: *mut HolographicFramePresentResult) -> HRESULT,
    fn PresentUsingCurrentPredictionWithBehavior(&self, waitBehavior: HolographicFramePresentWaitBehavior, out: *mut HolographicFramePresentResult) -> HRESULT,
    fn WaitForFrameToFinish(&self) -> HRESULT
}}
impl IHolographicFrame {
    #[inline] pub fn get_added_cameras(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HolographicCamera>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_AddedCameras)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_removed_cameras(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HolographicCamera>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RemovedCameras)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rendering_parameters(&self, cameraPose: &HolographicCameraPose) -> Result<Option<ComPtr<HolographicCameraRenderingParameters>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetRenderingParameters)(self as *const _ as *mut _, cameraPose as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Duration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_prediction(&self) -> Result<Option<ComPtr<HolographicFramePrediction>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CurrentPrediction)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_current_prediction(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateCurrentPrediction)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn present_using_current_prediction(&self) -> Result<HolographicFramePresentResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PresentUsingCurrentPrediction)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn present_using_current_prediction_with_behavior(&self, waitBehavior: HolographicFramePresentWaitBehavior) -> Result<HolographicFramePresentResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).PresentUsingCurrentPredictionWithBehavior)(self as *const _ as *mut _, waitBehavior, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_frame_to_finish(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).WaitForFrameToFinish)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFrame: IHolographicFrame}
DEFINE_IID!(IID_IHolographicFrame2, 675231679, 15346, 24209, 102, 51, 135, 5, 116, 230, 242, 23);
RT_INTERFACE!{interface IHolographicFrame2(IHolographicFrame2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicFrame2] {
    fn GetQuadLayerUpdateParameters(&self, layer: *mut HolographicQuadLayer, out: *mut *mut HolographicQuadLayerUpdateParameters) -> HRESULT
}}
impl IHolographicFrame2 {
    #[inline] pub fn get_quad_layer_update_parameters(&self, layer: &HolographicQuadLayer) -> Result<Option<ComPtr<HolographicQuadLayerUpdateParameters>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetQuadLayerUpdateParameters)(self as *const _ as *mut _, layer as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicFramePrediction, 1376734689, 23562, 20089, 168, 30, 106, 190, 2, 187, 39, 57);
RT_INTERFACE!{interface IHolographicFramePrediction(IHolographicFramePredictionVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicFramePrediction] {
    fn get_CameraPoses(&self, out: *mut *mut foundation::collections::IVectorView<HolographicCameraPose>) -> HRESULT,
    #[cfg(feature="windows-perception")] fn get_Timestamp(&self, out: *mut *mut super::super::perception::PerceptionTimestamp) -> HRESULT
}}
impl IHolographicFramePrediction {
    #[inline] pub fn get_camera_poses(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HolographicCameraPose>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CameraPoses)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn get_timestamp(&self) -> Result<Option<ComPtr<super::super::perception::PerceptionTimestamp>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Timestamp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFramePrediction: IHolographicFramePrediction}
DEFINE_IID!(IID_IHolographicFramePresentationMonitor, 3397854572, 28590, 17038, 187, 131, 37, 223, 238, 81, 19, 107);
RT_INTERFACE!{interface IHolographicFramePresentationMonitor(IHolographicFramePresentationMonitorVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicFramePresentationMonitor] {
    fn ReadReports(&self, out: *mut *mut foundation::collections::IVectorView<HolographicFramePresentationReport>) -> HRESULT
}}
impl IHolographicFramePresentationMonitor {
    #[inline] pub fn read_reports(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HolographicFramePresentationReport>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ReadReports)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFramePresentationMonitor: IHolographicFramePresentationMonitor}
DEFINE_IID!(IID_IHolographicFramePresentationReport, 2159736340, 62196, 19594, 141, 227, 6, 92, 120, 246, 213, 222);
RT_INTERFACE!{interface IHolographicFramePresentationReport(IHolographicFramePresentationReportVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicFramePresentationReport] {
    fn get_CompositorGpuDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_AppGpuDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_AppGpuOverrun(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_MissedPresentationOpportunityCount(&self, out: *mut u32) -> HRESULT,
    fn get_PresentationCount(&self, out: *mut u32) -> HRESULT
}}
impl IHolographicFramePresentationReport {
    #[inline] pub fn get_compositor_gpu_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CompositorGpuDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_gpu_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppGpuDuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_gpu_overrun(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AppGpuOverrun)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_missed_presentation_opportunity_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MissedPresentationOpportunityCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_presentation_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PresentationCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFramePresentationReport: IHolographicFramePresentationReport}
RT_ENUM! { enum HolographicFramePresentResult: i32 {
    Success = 0, DeviceRemoved = 1,
}}
RT_ENUM! { enum HolographicFramePresentWaitBehavior: i32 {
    WaitForFrameToFinish = 0, DoNotWaitForFrameToFinish = 1,
}}
DEFINE_IID!(IID_IHolographicQuadLayer, 2419351753, 51673, 23900, 65, 172, 162, 213, 171, 15, 211, 49);
RT_INTERFACE!{interface IHolographicQuadLayer(IHolographicQuadLayerVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayer] {
    fn get_PixelFormat(&self, out: *mut super::directx::DirectXPixelFormat) -> HRESULT,
    fn get_Size(&self, out: *mut foundation::Size) -> HRESULT
}}
impl IHolographicQuadLayer {
    #[inline] pub fn get_pixel_format(&self) -> Result<super::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Size)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicQuadLayer: IHolographicQuadLayer}
impl RtActivatable<IHolographicQuadLayerFactory> for HolographicQuadLayer {}
impl HolographicQuadLayer {
    #[inline] pub fn create(size: foundation::Size) -> Result<ComPtr<HolographicQuadLayer>> {
        <Self as RtActivatable<IHolographicQuadLayerFactory>>::get_activation_factory().create(size)
    }
    #[inline] pub fn create_with_pixel_format(size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat) -> Result<ComPtr<HolographicQuadLayer>> {
        <Self as RtActivatable<IHolographicQuadLayerFactory>>::get_activation_factory().create_with_pixel_format(size, pixelFormat)
    }
}
DEFINE_CLSID!(HolographicQuadLayer(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,72,111,108,111,103,114,97,112,104,105,99,46,72,111,108,111,103,114,97,112,104,105,99,81,117,97,100,76,97,121,101,114,0]) [CLSID_HolographicQuadLayer]);
DEFINE_IID!(IID_IHolographicQuadLayerFactory, 2792700147, 23060, 23056, 72, 154, 69, 80, 101, 179, 123, 118);
RT_INTERFACE!{static interface IHolographicQuadLayerFactory(IHolographicQuadLayerFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerFactory] {
    fn Create(&self, size: foundation::Size, out: *mut *mut HolographicQuadLayer) -> HRESULT,
    fn CreateWithPixelFormat(&self, size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat, out: *mut *mut HolographicQuadLayer) -> HRESULT
}}
impl IHolographicQuadLayerFactory {
    #[inline] pub fn create(&self, size: foundation::Size) -> Result<ComPtr<HolographicQuadLayer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, size, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_pixel_format(&self, size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat) -> Result<ComPtr<HolographicQuadLayer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithPixelFormat)(self as *const _ as *mut _, size, pixelFormat, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicQuadLayerUpdateParameters, 722379696, 31117, 23498, 85, 194, 44, 12, 118, 46, 187, 8);
RT_INTERFACE!{interface IHolographicQuadLayerUpdateParameters(IHolographicQuadLayerUpdateParametersVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerUpdateParameters] {
    fn AcquireBufferToUpdateContent(&self, out: *mut *mut super::directx::direct3d11::IDirect3DSurface) -> HRESULT,
    fn UpdateViewport(&self, value: foundation::Rect) -> HRESULT,
    fn UpdateContentProtectionEnabled(&self, value: bool) -> HRESULT,
    fn UpdateExtents(&self, value: foundation::numerics::Vector2) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-perception")] fn UpdateLocationWithStationaryMode(&self, coordinateSystem: *mut super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> HRESULT,
    fn UpdateLocationWithDisplayRelativeMode(&self, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> HRESULT
}}
impl IHolographicQuadLayerUpdateParameters {
    #[inline] pub fn acquire_buffer_to_update_content(&self) -> Result<Option<ComPtr<super::directx::direct3d11::IDirect3DSurface>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AcquireBufferToUpdateContent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_viewport(&self, value: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateViewport)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateContentProtectionEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_extents(&self, value: foundation::numerics::Vector2) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateExtents)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn update_location_with_stationary_mode(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateLocationWithStationaryMode)(self as *const _ as *mut _, coordinateSystem as *const _ as *mut _, position, orientation);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_location_with_display_relative_mode(&self, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).UpdateLocationWithDisplayRelativeMode)(self as *const _ as *mut _, position, orientation);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicQuadLayerUpdateParameters: IHolographicQuadLayerUpdateParameters}
DEFINE_IID!(IID_IHolographicQuadLayerUpdateParameters2, 1328796461, 33473, 18113, 137, 128, 60, 183, 13, 152, 24, 43);
RT_INTERFACE!{interface IHolographicQuadLayerUpdateParameters2(IHolographicQuadLayerUpdateParameters2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerUpdateParameters2] {
    fn get_CanAcquireWithHardwareProtection(&self, out: *mut bool) -> HRESULT,
    fn AcquireBufferToUpdateContentWithHardwareProtection(&self, out: *mut *mut super::directx::direct3d11::IDirect3DSurface) -> HRESULT
}}
impl IHolographicQuadLayerUpdateParameters2 {
    #[inline] pub fn get_can_acquire_with_hardware_protection(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CanAcquireWithHardwareProtection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn acquire_buffer_to_update_content_with_hardware_protection(&self) -> Result<Option<ComPtr<super::directx::direct3d11::IDirect3DSurface>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).AcquireBufferToUpdateContentWithHardwareProtection)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HolographicReprojectionMode: i32 {
    PositionAndOrientation = 0, OrientationOnly = 1, Disabled = 2,
}}
DEFINE_IID!(IID_IHolographicSpace, 1132518310, 24184, 17231, 128, 124, 52, 51, 209, 239, 232, 183);
RT_INTERFACE!{interface IHolographicSpace(IHolographicSpaceVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpace] {
    fn get_PrimaryAdapterId(&self, out: *mut HolographicAdapterId) -> HRESULT,
    fn SetDirect3D11Device(&self, value: *mut super::directx::direct3d11::IDirect3DDevice) -> HRESULT,
    fn add_CameraAdded(&self, handler: *mut foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CameraAdded(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_CameraRemoved(&self, handler: *mut foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CameraRemoved(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateNextFrame(&self, out: *mut *mut HolographicFrame) -> HRESULT
}}
impl IHolographicSpace {
    #[inline] pub fn get_primary_adapter_id(&self) -> Result<HolographicAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrimaryAdapterId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_direct3d11_device(&self, value: &super::directx::direct3d11::IDirect3DDevice) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetDirect3D11Device)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_camera_added(&self, handler: &foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CameraAdded)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_camera_added(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_CameraAdded)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_camera_removed(&self, handler: &foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_CameraRemoved)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_camera_removed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_CameraRemoved)(self as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_next_frame(&self) -> Result<Option<ComPtr<HolographicFrame>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateNextFrame)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpace: IHolographicSpace}
impl RtActivatable<IHolographicSpaceStatics> for HolographicSpace {}
impl RtActivatable<IHolographicSpaceStatics2> for HolographicSpace {}
impl RtActivatable<IHolographicSpaceStatics3> for HolographicSpace {}
impl HolographicSpace {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_for_core_window(window: &super::super::ui::core::CoreWindow) -> Result<Option<ComPtr<HolographicSpace>>> {
        <Self as RtActivatable<IHolographicSpaceStatics>>::get_activation_factory().create_for_core_window(window)
    }
    #[inline] pub fn get_is_supported() -> Result<bool> {
        <Self as RtActivatable<IHolographicSpaceStatics2>>::get_activation_factory().get_is_supported()
    }
    #[inline] pub fn get_is_available() -> Result<bool> {
        <Self as RtActivatable<IHolographicSpaceStatics2>>::get_activation_factory().get_is_available()
    }
    #[inline] pub fn add_is_available_changed(handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> {
        <Self as RtActivatable<IHolographicSpaceStatics2>>::get_activation_factory().add_is_available_changed(handler)
    }
    #[inline] pub fn remove_is_available_changed(token: foundation::EventRegistrationToken) -> Result<()> {
        <Self as RtActivatable<IHolographicSpaceStatics2>>::get_activation_factory().remove_is_available_changed(token)
    }
    #[inline] pub fn get_is_configured() -> Result<bool> {
        <Self as RtActivatable<IHolographicSpaceStatics3>>::get_activation_factory().get_is_configured()
    }
}
DEFINE_CLSID!(HolographicSpace(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,72,111,108,111,103,114,97,112,104,105,99,46,72,111,108,111,103,114,97,112,104,105,99,83,112,97,99,101,0]) [CLSID_HolographicSpace]);
DEFINE_IID!(IID_IHolographicSpace2, 1333897640, 47103, 18563, 152, 39, 125, 103, 114, 135, 234, 112);
RT_INTERFACE!{interface IHolographicSpace2(IHolographicSpace2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpace2] {
    fn get_UserPresence(&self, out: *mut HolographicSpaceUserPresence) -> HRESULT,
    fn add_UserPresenceChanged(&self, handler: *mut foundation::TypedEventHandler<HolographicSpace, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserPresenceChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn WaitForNextFrameReady(&self) -> HRESULT,
    fn WaitForNextFrameReadyWithHeadStart(&self, requestedHeadStartDuration: foundation::TimeSpan) -> HRESULT,
    fn CreateFramePresentationMonitor(&self, maxQueuedReports: u32, out: *mut *mut HolographicFramePresentationMonitor) -> HRESULT
}}
impl IHolographicSpace2 {
    #[inline] pub fn get_user_presence(&self) -> Result<HolographicSpaceUserPresence> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_UserPresence)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_user_presence_changed(&self, handler: &foundation::TypedEventHandler<HolographicSpace, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_UserPresenceChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_presence_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_UserPresenceChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_next_frame_ready(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).WaitForNextFrameReady)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_next_frame_ready_with_head_start(&self, requestedHeadStartDuration: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).WaitForNextFrameReadyWithHeadStart)(self as *const _ as *mut _, requestedHeadStartDuration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_frame_presentation_monitor(&self, maxQueuedReports: u32) -> Result<Option<ComPtr<HolographicFramePresentationMonitor>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateFramePresentationMonitor)(self as *const _ as *mut _, maxQueuedReports, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceCameraAddedEventArgs, 1492245045, 48051, 15503, 153, 61, 108, 128, 231, 254, 185, 159);
RT_INTERFACE!{interface IHolographicSpaceCameraAddedEventArgs(IHolographicSpaceCameraAddedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceCameraAddedEventArgs] {
    fn get_Camera(&self, out: *mut *mut HolographicCamera) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IHolographicSpaceCameraAddedEventArgs {
    #[inline] pub fn get_camera(&self) -> Result<Option<ComPtr<HolographicCamera>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Camera)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpaceCameraAddedEventArgs: IHolographicSpaceCameraAddedEventArgs}
DEFINE_IID!(IID_IHolographicSpaceCameraRemovedEventArgs, 2153006248, 62126, 12846, 141, 169, 131, 106, 10, 149, 164, 193);
RT_INTERFACE!{interface IHolographicSpaceCameraRemovedEventArgs(IHolographicSpaceCameraRemovedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceCameraRemovedEventArgs] {
    fn get_Camera(&self, out: *mut *mut HolographicCamera) -> HRESULT
}}
impl IHolographicSpaceCameraRemovedEventArgs {
    #[inline] pub fn get_camera(&self) -> Result<Option<ComPtr<HolographicCamera>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Camera)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpaceCameraRemovedEventArgs: IHolographicSpaceCameraRemovedEventArgs}
DEFINE_IID!(IID_IHolographicSpaceStatics, 911106148, 51442, 15265, 131, 145, 102, 184, 72, 158, 103, 253);
RT_INTERFACE!{static interface IHolographicSpaceStatics(IHolographicSpaceStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics] {
    #[cfg(feature="windows-ui")] fn CreateForCoreWindow(&self, window: *mut super::super::ui::core::CoreWindow, out: *mut *mut HolographicSpace) -> HRESULT
}}
impl IHolographicSpaceStatics {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_for_core_window(&self, window: &super::super::ui::core::CoreWindow) -> Result<Option<ComPtr<HolographicSpace>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateForCoreWindow)(self as *const _ as *mut _, window as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceStatics2, 242708616, 30204, 18607, 135, 88, 6, 82, 246, 240, 124, 89);
RT_INTERFACE!{static interface IHolographicSpaceStatics2(IHolographicSpaceStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics2] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsAvailable(&self, out: *mut bool) -> HRESULT,
    fn add_IsAvailableChanged(&self, handler: *mut foundation::EventHandler<IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsAvailableChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IHolographicSpaceStatics2 {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_available(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsAvailable)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_is_available_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_IsAvailableChanged)(self as *const _ as *mut _, handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_available_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_IsAvailableChanged)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceStatics3, 989912637, 45475, 19966, 142, 121, 254, 197, 144, 158, 109, 248);
RT_INTERFACE!{static interface IHolographicSpaceStatics3(IHolographicSpaceStatics3Vtbl): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics3] {
    fn get_IsConfigured(&self, out: *mut bool) -> HRESULT
}}
impl IHolographicSpaceStatics3 {
    #[inline] pub fn get_is_configured(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsConfigured)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum HolographicSpaceUserPresence: i32 {
    Absent = 0, PresentPassive = 1, PresentActive = 2,
}}
RT_STRUCT! { struct HolographicStereoTransform {
    Left: foundation::numerics::Matrix4x4, Right: foundation::numerics::Matrix4x4,
}}
} // Windows.Graphics.Holographic
pub mod imaging { // Windows.Graphics.Imaging
use crate::prelude::*;
RT_ENUM! { enum BitmapAlphaMode: i32 {
    Premultiplied = 0, Straight = 1, Ignore = 2,
}}
RT_STRUCT! { struct BitmapBounds {
    X: u32, Y: u32, Width: u32, Height: u32,
}}
DEFINE_IID!(IID_IBitmapBuffer, 2772305092, 14748, 17292, 178, 143, 166, 58, 107, 131, 209, 161);
RT_INTERFACE!{interface IBitmapBuffer(IBitmapBufferVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapBuffer] {
    fn GetPlaneCount(&self, out: *mut i32) -> HRESULT,
    fn GetPlaneDescription(&self, index: i32, out: *mut BitmapPlaneDescription) -> HRESULT
}}
impl IBitmapBuffer {
    #[inline] pub fn get_plane_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetPlaneCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_plane_description(&self, index: i32) -> Result<BitmapPlaneDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetPlaneDescription)(self as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapBuffer: IBitmapBuffer}
RT_ENUM! { enum BitmapBufferAccessMode: i32 {
    Read = 0, ReadWrite = 1, Write = 2,
}}
DEFINE_IID!(IID_IBitmapCodecInformation, 1074572018, 50352, 17298, 163, 176, 111, 111, 155, 169, 92, 180);
RT_INTERFACE!{interface IBitmapCodecInformation(IBitmapCodecInformationVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapCodecInformation] {
    fn get_CodecId(&self, out: *mut Guid) -> HRESULT,
    fn get_FileExtensions(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MimeTypes(&self, out: *mut *mut foundation::collections::IVectorView<HString>) -> HRESULT
}}
impl IBitmapCodecInformation {
    #[inline] pub fn get_codec_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_CodecId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_file_extensions(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FileExtensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_FriendlyName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mime_types(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MimeTypes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapCodecInformation: IBitmapCodecInformation}
DEFINE_IID!(IID_IBitmapDecoder, 2901353146, 7540, 19601, 157, 252, 150, 32, 116, 82, 51, 230);
RT_INTERFACE!{interface IBitmapDecoder(IBitmapDecoderVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapDecoder] {
    fn get_BitmapContainerProperties(&self, out: *mut *mut BitmapPropertiesView) -> HRESULT,
    fn get_DecoderInformation(&self, out: *mut *mut BitmapCodecInformation) -> HRESULT,
    fn get_FrameCount(&self, out: *mut u32) -> HRESULT,
    fn GetPreviewAsync(&self, out: *mut *mut foundation::IAsyncOperation<ImageStream>) -> HRESULT,
    fn GetFrameAsync(&self, frameIndex: u32, out: *mut *mut foundation::IAsyncOperation<BitmapFrame>) -> HRESULT
}}
impl IBitmapDecoder {
    #[inline] pub fn get_bitmap_container_properties(&self) -> Result<Option<ComPtr<BitmapPropertiesView>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BitmapContainerProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_decoder_information(&self) -> Result<Option<ComPtr<BitmapCodecInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DecoderInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FrameCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_preview_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ImageStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPreviewAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_async(&self, frameIndex: u32) -> Result<ComPtr<foundation::IAsyncOperation<BitmapFrame>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFrameAsync)(self as *const _ as *mut _, frameIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapDecoder: IBitmapDecoder}
impl RtActivatable<IBitmapDecoderStatics> for BitmapDecoder {}
impl RtActivatable<IBitmapDecoderStatics2> for BitmapDecoder {}
impl BitmapDecoder {
    #[inline] pub fn get_bmp_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_bmp_decoder_id()
    }
    #[inline] pub fn get_jpeg_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_jpeg_decoder_id()
    }
    #[inline] pub fn get_png_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_png_decoder_id()
    }
    #[inline] pub fn get_tiff_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_tiff_decoder_id()
    }
    #[inline] pub fn get_gif_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_gif_decoder_id()
    }
    #[inline] pub fn get_jpeg_xr_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_jpeg_xr_decoder_id()
    }
    #[inline] pub fn get_ico_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_ico_decoder_id()
    }
    #[inline] pub fn get_decoder_information_enumerator() -> Result<Option<ComPtr<foundation::collections::IVectorView<BitmapCodecInformation>>>> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_decoder_information_enumerator()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapDecoder>>> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().create_async(stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_id_async(decoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapDecoder>>> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().create_with_id_async(decoderId, stream)
    }
    #[inline] pub fn get_heif_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics2>>::get_activation_factory().get_heif_decoder_id()
    }
    #[inline] pub fn get_webp_decoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapDecoderStatics2>>::get_activation_factory().get_webp_decoder_id()
    }
}
DEFINE_CLSID!(BitmapDecoder(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,68,101,99,111,100,101,114,0]) [CLSID_BitmapDecoder]);
DEFINE_IID!(IID_IBitmapDecoderStatics, 1133300518, 48367, 20117, 186, 214, 35, 168, 34, 229, 141, 1);
RT_INTERFACE!{static interface IBitmapDecoderStatics(IBitmapDecoderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapDecoderStatics] {
    fn get_BmpDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_PngDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_TiffDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_GifDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegXRDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_IcoDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn GetDecoderInformationEnumerator(&self, out: *mut *mut foundation::collections::IVectorView<BitmapCodecInformation>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateAsync(&self, stream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<BitmapDecoder>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateWithIdAsync(&self, decoderId: Guid, stream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<BitmapDecoder>) -> HRESULT
}}
impl IBitmapDecoderStatics {
    #[inline] pub fn get_bmp_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BmpDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_JpegDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_png_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PngDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tiff_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TiffDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_gif_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_GifDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_xr_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_JpegXRDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_ico_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IcoDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_decoder_information_enumerator(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<BitmapCodecInformation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDecoderInformationEnumerator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(&self, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapDecoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAsync)(self as *const _ as *mut _, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_id_async(&self, decoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapDecoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithIdAsync)(self as *const _ as *mut _, decoderId, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapDecoderStatics2, 1354393834, 39329, 16580, 128, 217, 174, 240, 218, 250, 108, 63);
RT_INTERFACE!{static interface IBitmapDecoderStatics2(IBitmapDecoderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IBitmapDecoderStatics2] {
    fn get_HeifDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_WebpDecoderId(&self, out: *mut Guid) -> HRESULT
}}
impl IBitmapDecoderStatics2 {
    #[inline] pub fn get_heif_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HeifDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_webp_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_WebpDecoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoder, 734292195, 57848, 19284, 149, 232, 50, 145, 149, 81, 206, 98);
RT_INTERFACE!{interface IBitmapEncoder(IBitmapEncoderVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapEncoder] {
    fn get_EncoderInformation(&self, out: *mut *mut BitmapCodecInformation) -> HRESULT,
    fn get_BitmapProperties(&self, out: *mut *mut BitmapProperties) -> HRESULT,
    fn get_BitmapContainerProperties(&self, out: *mut *mut BitmapProperties) -> HRESULT,
    fn get_IsThumbnailGenerated(&self, out: *mut bool) -> HRESULT,
    fn put_IsThumbnailGenerated(&self, value: bool) -> HRESULT,
    fn get_GeneratedThumbnailWidth(&self, out: *mut u32) -> HRESULT,
    fn put_GeneratedThumbnailWidth(&self, value: u32) -> HRESULT,
    fn get_GeneratedThumbnailHeight(&self, out: *mut u32) -> HRESULT,
    fn put_GeneratedThumbnailHeight(&self, value: u32) -> HRESULT,
    fn get_BitmapTransform(&self, out: *mut *mut BitmapTransform) -> HRESULT,
    fn SetPixelData(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, width: u32, height: u32, dpiX: f64, dpiY: f64, pixelsSize: u32, pixels: *mut u8) -> HRESULT,
    fn GoToNextFrameAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn GoToNextFrameWithEncodingOptionsAsync(&self, encodingOptions: *mut foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn FlushAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IBitmapEncoder {
    #[inline] pub fn get_encoder_information(&self) -> Result<Option<ComPtr<BitmapCodecInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_EncoderInformation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_properties(&self) -> Result<Option<ComPtr<BitmapProperties>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BitmapProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_container_properties(&self) -> Result<Option<ComPtr<BitmapProperties>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BitmapContainerProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_thumbnail_generated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsThumbnailGenerated)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_thumbnail_generated(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsThumbnailGenerated)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_generated_thumbnail_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_GeneratedThumbnailWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_generated_thumbnail_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_GeneratedThumbnailWidth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_generated_thumbnail_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_GeneratedThumbnailHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_generated_thumbnail_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_GeneratedThumbnailHeight)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_transform(&self) -> Result<Option<ComPtr<BitmapTransform>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BitmapTransform)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_pixel_data(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, width: u32, height: u32, dpiX: f64, dpiY: f64, pixels: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetPixelData)(self as *const _ as *mut _, pixelFormat, alphaMode, width, height, dpiX, dpiY, pixels.len() as u32, pixels.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn go_to_next_frame_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GoToNextFrameAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn go_to_next_frame_with_encoding_options_async(&self, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GoToNextFrameWithEncodingOptionsAsync)(self as *const _ as *mut _, encodingOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn flush_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).FlushAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapEncoder: IBitmapEncoder}
impl RtActivatable<IBitmapEncoderStatics> for BitmapEncoder {}
impl RtActivatable<IBitmapEncoderStatics2> for BitmapEncoder {}
impl BitmapEncoder {
    #[inline] pub fn get_bmp_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_bmp_encoder_id()
    }
    #[inline] pub fn get_jpeg_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_jpeg_encoder_id()
    }
    #[inline] pub fn get_png_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_png_encoder_id()
    }
    #[inline] pub fn get_tiff_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_tiff_encoder_id()
    }
    #[inline] pub fn get_gif_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_gif_encoder_id()
    }
    #[inline] pub fn get_jpeg_xr_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_jpeg_xr_encoder_id()
    }
    #[inline] pub fn get_encoder_information_enumerator() -> Result<Option<ComPtr<foundation::collections::IVectorView<BitmapCodecInformation>>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_encoder_information_enumerator()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_async(encoderId, stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_encoding_options_async(encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_with_encoding_options_async(encoderId, stream, encodingOptions)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_transcoding_async(stream: &super::super::storage::streams::IRandomAccessStream, bitmapDecoder: &BitmapDecoder) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_for_transcoding_async(stream, bitmapDecoder)
    }
    #[inline] pub fn create_for_in_place_property_encoding_async(bitmapDecoder: &BitmapDecoder) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_for_in_place_property_encoding_async(bitmapDecoder)
    }
    #[inline] pub fn get_heif_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics2>>::get_activation_factory().get_heif_encoder_id()
    }
}
DEFINE_CLSID!(BitmapEncoder(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,69,110,99,111,100,101,114,0]) [CLSID_BitmapEncoder]);
DEFINE_IID!(IID_IBitmapEncoderStatics, 2806208167, 42212, 20153, 142, 64, 86, 77, 231, 225, 204, 178);
RT_INTERFACE!{static interface IBitmapEncoderStatics(IBitmapEncoderStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderStatics] {
    fn get_BmpEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_PngEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_TiffEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_GifEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegXREncoderId(&self, out: *mut Guid) -> HRESULT,
    fn GetEncoderInformationEnumerator(&self, out: *mut *mut foundation::collections::IVectorView<BitmapCodecInformation>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateAsync(&self, encoderId: Guid, stream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<BitmapEncoder>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateWithEncodingOptionsAsync(&self, encoderId: Guid, stream: *mut super::super::storage::streams::IRandomAccessStream, encodingOptions: *mut foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>, out: *mut *mut foundation::IAsyncOperation<BitmapEncoder>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateForTranscodingAsync(&self, stream: *mut super::super::storage::streams::IRandomAccessStream, bitmapDecoder: *mut BitmapDecoder, out: *mut *mut foundation::IAsyncOperation<BitmapEncoder>) -> HRESULT,
    fn CreateForInPlacePropertyEncodingAsync(&self, bitmapDecoder: *mut BitmapDecoder, out: *mut *mut foundation::IAsyncOperation<BitmapEncoder>) -> HRESULT
}}
impl IBitmapEncoderStatics {
    #[inline] pub fn get_bmp_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BmpEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_JpegEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_png_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PngEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tiff_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TiffEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_gif_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_GifEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_xr_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_JpegXREncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_encoder_information_enumerator(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<BitmapCodecInformation>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetEncoderInformationEnumerator)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(&self, encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateAsync)(self as *const _ as *mut _, encoderId, stream as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_encoding_options_async(&self, encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithEncodingOptionsAsync)(self as *const _ as *mut _, encoderId, stream as *const _ as *mut _, encodingOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_transcoding_async(&self, stream: &super::super::storage::streams::IRandomAccessStream, bitmapDecoder: &BitmapDecoder) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateForTranscodingAsync)(self as *const _ as *mut _, stream as *const _ as *mut _, bitmapDecoder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_for_in_place_property_encoding_async(&self, bitmapDecoder: &BitmapDecoder) -> Result<ComPtr<foundation::IAsyncOperation<BitmapEncoder>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateForInPlacePropertyEncodingAsync)(self as *const _ as *mut _, bitmapDecoder as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoderStatics2, 868991577, 65073, 16817, 184, 18, 8, 109, 33, 232, 126, 22);
RT_INTERFACE!{static interface IBitmapEncoderStatics2(IBitmapEncoderStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderStatics2] {
    fn get_HeifEncoderId(&self, out: *mut Guid) -> HRESULT
}}
impl IBitmapEncoderStatics2 {
    #[inline] pub fn get_heif_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HeifEncoderId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoderWithSoftwareBitmap, 1751962177, 17200, 19575, 172, 228, 3, 52, 150, 139, 23, 104);
RT_INTERFACE!{interface IBitmapEncoderWithSoftwareBitmap(IBitmapEncoderWithSoftwareBitmapVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderWithSoftwareBitmap] {
    fn SetSoftwareBitmap(&self, bitmap: *mut SoftwareBitmap) -> HRESULT
}}
impl IBitmapEncoderWithSoftwareBitmap {
    #[inline] pub fn set_software_bitmap(&self, bitmap: &SoftwareBitmap) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetSoftwareBitmap)(self as *const _ as *mut _, bitmap as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum BitmapFlip: i32 {
    None = 0, Horizontal = 1, Vertical = 2,
}}
DEFINE_IID!(IID_IBitmapFrame, 1923389980, 32897, 17293, 145, 188, 148, 236, 252, 129, 133, 198);
RT_INTERFACE!{interface IBitmapFrame(IBitmapFrameVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapFrame] {
    fn GetThumbnailAsync(&self, out: *mut *mut foundation::IAsyncOperation<ImageStream>) -> HRESULT,
    fn get_BitmapProperties(&self, out: *mut *mut BitmapPropertiesView) -> HRESULT,
    fn get_BitmapPixelFormat(&self, out: *mut BitmapPixelFormat) -> HRESULT,
    fn get_BitmapAlphaMode(&self, out: *mut BitmapAlphaMode) -> HRESULT,
    fn get_DpiX(&self, out: *mut f64) -> HRESULT,
    fn get_DpiY(&self, out: *mut f64) -> HRESULT,
    fn get_PixelWidth(&self, out: *mut u32) -> HRESULT,
    fn get_PixelHeight(&self, out: *mut u32) -> HRESULT,
    fn get_OrientedPixelWidth(&self, out: *mut u32) -> HRESULT,
    fn get_OrientedPixelHeight(&self, out: *mut u32) -> HRESULT,
    fn GetPixelDataAsync(&self, out: *mut *mut foundation::IAsyncOperation<PixelDataProvider>) -> HRESULT,
    fn GetPixelDataTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: *mut BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode, out: *mut *mut foundation::IAsyncOperation<PixelDataProvider>) -> HRESULT
}}
impl IBitmapFrame {
    #[inline] pub fn get_thumbnail_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<ImageStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetThumbnailAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_properties(&self) -> Result<Option<ComPtr<BitmapPropertiesView>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BitmapProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapPixelFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapAlphaMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_oriented_pixel_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OrientedPixelWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_oriented_pixel_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OrientedPixelHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_data_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<PixelDataProvider>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPixelDataAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_data_transformed_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: &BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> Result<ComPtr<foundation::IAsyncOperation<PixelDataProvider>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPixelDataTransformedAsync)(self as *const _ as *mut _, pixelFormat, alphaMode, transform as *const _ as *mut _, exifOrientationMode, colorManagementMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapFrame: IBitmapFrame}
DEFINE_IID!(IID_IBitmapFrameWithSoftwareBitmap, 4264066202, 16908, 18787, 135, 173, 105, 20, 54, 224, 131, 131);
RT_INTERFACE!{interface IBitmapFrameWithSoftwareBitmap(IBitmapFrameWithSoftwareBitmapVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapFrameWithSoftwareBitmap] {
    fn GetSoftwareBitmapAsync(&self, out: *mut *mut foundation::IAsyncOperation<SoftwareBitmap>) -> HRESULT,
    fn GetSoftwareBitmapConvertedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, out: *mut *mut foundation::IAsyncOperation<SoftwareBitmap>) -> HRESULT,
    fn GetSoftwareBitmapTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: *mut BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode, out: *mut *mut foundation::IAsyncOperation<SoftwareBitmap>) -> HRESULT
}}
impl IBitmapFrameWithSoftwareBitmap {
    #[inline] pub fn get_software_bitmap_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSoftwareBitmapAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_software_bitmap_converted_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSoftwareBitmapConvertedAsync)(self as *const _ as *mut _, pixelFormat, alphaMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_software_bitmap_transformed_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: &BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSoftwareBitmapTransformedAsync)(self as *const _ as *mut _, pixelFormat, alphaMode, transform as *const _ as *mut _, exifOrientationMode, colorManagementMode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum BitmapInterpolationMode: i32 {
    NearestNeighbor = 0, Linear = 1, Cubic = 2, Fant = 3,
}}
RT_ENUM! { enum BitmapPixelFormat: i32 {
    Unknown = 0, Rgba16 = 12, Rgba8 = 30, Gray16 = 57, Gray8 = 62, Bgra8 = 87, Nv12 = 103, P010 = 104, Yuy2 = 107,
}}
RT_STRUCT! { struct BitmapPlaneDescription {
    StartIndex: i32, Width: i32, Height: i32, Stride: i32,
}}
DEFINE_IID!(IID_IBitmapProperties, 3936309019, 46341, 17488, 164, 209, 232, 202, 148, 82, 157, 141);
RT_INTERFACE!{interface IBitmapProperties(IBitmapPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapProperties] {
    fn SetPropertiesAsync(&self, propertiesToSet: *mut foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IBitmapProperties {
    #[inline] pub fn set_properties_async(&self, propertiesToSet: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetPropertiesAsync)(self as *const _ as *mut _, propertiesToSet as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapProperties: IBitmapProperties}
DEFINE_IID!(IID_IBitmapPropertiesView, 2114971770, 14960, 18680, 156, 85, 25, 108, 245, 165, 69, 245);
RT_INTERFACE!{interface IBitmapPropertiesView(IBitmapPropertiesViewVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapPropertiesView] {
    fn GetPropertiesAsync(&self, propertiesToRetrieve: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<BitmapPropertySet>) -> HRESULT
}}
impl IBitmapPropertiesView {
    #[inline] pub fn get_properties_async(&self, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<ComPtr<foundation::IAsyncOperation<BitmapPropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertiesAsync)(self as *const _ as *mut _, propertiesToRetrieve as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapPropertiesView: IBitmapPropertiesView}
RT_CLASS!{class BitmapPropertySet: foundation::collections::IMap<HString, BitmapTypedValue>}
impl RtActivatable<IActivationFactory> for BitmapPropertySet {}
DEFINE_CLSID!(BitmapPropertySet(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,80,114,111,112,101,114,116,121,83,101,116,0]) [CLSID_BitmapPropertySet]);
RT_ENUM! { enum BitmapRotation: i32 {
    None = 0, Clockwise90Degrees = 1, Clockwise180Degrees = 2, Clockwise270Degrees = 3,
}}
RT_STRUCT! { struct BitmapSize {
    Width: u32, Height: u32,
}}
DEFINE_IID!(IID_IBitmapTransform, 2926924612, 57960, 19765, 173, 207, 233, 149, 211, 26, 141, 52);
RT_INTERFACE!{interface IBitmapTransform(IBitmapTransformVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapTransform] {
    fn get_ScaledWidth(&self, out: *mut u32) -> HRESULT,
    fn put_ScaledWidth(&self, value: u32) -> HRESULT,
    fn get_ScaledHeight(&self, out: *mut u32) -> HRESULT,
    fn put_ScaledHeight(&self, value: u32) -> HRESULT,
    fn get_InterpolationMode(&self, out: *mut BitmapInterpolationMode) -> HRESULT,
    fn put_InterpolationMode(&self, value: BitmapInterpolationMode) -> HRESULT,
    fn get_Flip(&self, out: *mut BitmapFlip) -> HRESULT,
    fn put_Flip(&self, value: BitmapFlip) -> HRESULT,
    fn get_Rotation(&self, out: *mut BitmapRotation) -> HRESULT,
    fn put_Rotation(&self, value: BitmapRotation) -> HRESULT,
    fn get_Bounds(&self, out: *mut BitmapBounds) -> HRESULT,
    fn put_Bounds(&self, value: BitmapBounds) -> HRESULT
}}
impl IBitmapTransform {
    #[inline] pub fn get_scaled_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScaledWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_scaled_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ScaledWidth)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_scaled_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ScaledHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_scaled_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ScaledHeight)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_interpolation_mode(&self) -> Result<BitmapInterpolationMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_InterpolationMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_interpolation_mode(&self, value: BitmapInterpolationMode) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_InterpolationMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_flip(&self) -> Result<BitmapFlip> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Flip)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_flip(&self, value: BitmapFlip) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Flip)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_rotation(&self) -> Result<BitmapRotation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Rotation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rotation(&self, value: BitmapRotation) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Rotation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bounds(&self) -> Result<BitmapBounds> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Bounds)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bounds(&self, value: BitmapBounds) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Bounds)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapTransform: IBitmapTransform}
impl RtActivatable<IActivationFactory> for BitmapTransform {}
DEFINE_CLSID!(BitmapTransform(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,84,114,97,110,115,102,111,114,109,0]) [CLSID_BitmapTransform]);
DEFINE_IID!(IID_IBitmapTypedValue, 3447735465, 9283, 16384, 176, 205, 121, 49, 108, 86, 245, 137);
RT_INTERFACE!{interface IBitmapTypedValue(IBitmapTypedValueVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapTypedValue] {
    fn get_Value(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_Type(&self, out: *mut foundation::PropertyType) -> HRESULT
}}
impl IBitmapTypedValue {
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<foundation::PropertyType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapTypedValue: IBitmapTypedValue}
impl RtActivatable<IBitmapTypedValueFactory> for BitmapTypedValue {}
impl BitmapTypedValue {
    #[inline] pub fn create(value: &IInspectable, type_: foundation::PropertyType) -> Result<ComPtr<BitmapTypedValue>> {
        <Self as RtActivatable<IBitmapTypedValueFactory>>::get_activation_factory().create(value, type_)
    }
}
DEFINE_CLSID!(BitmapTypedValue(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,84,121,112,101,100,86,97,108,117,101,0]) [CLSID_BitmapTypedValue]);
DEFINE_IID!(IID_IBitmapTypedValueFactory, 2463872409, 52755, 18107, 149, 69, 203, 58, 63, 99, 235, 139);
RT_INTERFACE!{static interface IBitmapTypedValueFactory(IBitmapTypedValueFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IBitmapTypedValueFactory] {
    fn Create(&self, value: *mut IInspectable, type_: foundation::PropertyType, out: *mut *mut BitmapTypedValue) -> HRESULT
}}
impl IBitmapTypedValueFactory {
    #[inline] pub fn create(&self, value: &IInspectable, type_: foundation::PropertyType) -> Result<ComPtr<BitmapTypedValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, value as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum ColorManagementMode: i32 {
    DoNotColorManage = 0, ColorManageToSRgb = 1,
}}
RT_ENUM! { enum ExifOrientationMode: i32 {
    IgnoreExifOrientation = 0, RespectExifOrientation = 1,
}}
#[cfg(feature="windows-storage")] RT_CLASS!{class ImageStream: super::super::storage::streams::IRandomAccessStreamWithContentType}
#[cfg(not(feature="windows-storage"))] RT_CLASS!{class ImageStream: IInspectable}
RT_ENUM! { enum JpegSubsamplingMode: i32 {
    Default = 0, Y4Cb2Cr0 = 1, Y4Cb2Cr2 = 2, Y4Cb4Cr4 = 3,
}}
DEFINE_IID!(IID_IPixelDataProvider, 3716357925, 6236, 17813, 159, 185, 204, 190, 110, 193, 138, 111);
RT_INTERFACE!{interface IPixelDataProvider(IPixelDataProviderVtbl): IInspectable(IInspectableVtbl) [IID_IPixelDataProvider] {
    fn DetachPixelData(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT
}}
impl IPixelDataProvider {
    #[inline] pub fn detach_pixel_data(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.lpVtbl).DetachPixelData)(self as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PixelDataProvider: IPixelDataProvider}
RT_ENUM! { enum PngFilterMode: i32 {
    Automatic = 0, None = 1, Sub = 2, Up = 3, Average = 4, Paeth = 5, Adaptive = 6,
}}
DEFINE_IID!(IID_ISoftwareBitmap, 1755186952, 32495, 18495, 150, 63, 218, 147, 136, 24, 224, 115);
RT_INTERFACE!{interface ISoftwareBitmap(ISoftwareBitmapVtbl): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmap] {
    fn get_BitmapPixelFormat(&self, out: *mut BitmapPixelFormat) -> HRESULT,
    fn get_BitmapAlphaMode(&self, out: *mut BitmapAlphaMode) -> HRESULT,
    fn get_PixelWidth(&self, out: *mut i32) -> HRESULT,
    fn get_PixelHeight(&self, out: *mut i32) -> HRESULT,
    fn get_IsReadOnly(&self, out: *mut bool) -> HRESULT,
    fn put_DpiX(&self, value: f64) -> HRESULT,
    fn get_DpiX(&self, out: *mut f64) -> HRESULT,
    fn put_DpiY(&self, value: f64) -> HRESULT,
    fn get_DpiY(&self, out: *mut f64) -> HRESULT,
    fn LockBuffer(&self, mode: BitmapBufferAccessMode, out: *mut *mut BitmapBuffer) -> HRESULT,
    fn CopyTo(&self, bitmap: *mut SoftwareBitmap) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy11(&self) -> (),
    #[cfg(feature="windows-storage")] fn CopyFromBuffer(&self, buffer: *mut super::super::storage::streams::IBuffer) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-storage")] fn CopyToBuffer(&self, buffer: *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn GetReadOnlyView(&self, out: *mut *mut SoftwareBitmap) -> HRESULT
}}
impl ISoftwareBitmap {
    #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapPixelFormat)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_BitmapAlphaMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_width(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelWidth)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_height(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PixelHeight)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_read_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsReadOnly)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_x(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DpiX)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_y(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DpiY)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn lock_buffer(&self, mode: BitmapBufferAccessMode) -> Result<Option<ComPtr<BitmapBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LockBuffer)(self as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_to(&self, bitmap: &SoftwareBitmap) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CopyTo)(self as *const _ as *mut _, bitmap as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_from_buffer(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CopyFromBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_to_buffer(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CopyToBuffer)(self as *const _ as *mut _, buffer as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_read_only_view(&self) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetReadOnlyView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SoftwareBitmap: ISoftwareBitmap}
impl RtActivatable<ISoftwareBitmapFactory> for SoftwareBitmap {}
impl RtActivatable<ISoftwareBitmapStatics> for SoftwareBitmap {}
impl SoftwareBitmap {
    #[inline] pub fn create(format: BitmapPixelFormat, width: i32, height: i32) -> Result<ComPtr<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapFactory>>::get_activation_factory().create(format, width, height)
    }
    #[inline] pub fn create_with_alpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<ComPtr<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapFactory>>::get_activation_factory().create_with_alpha(format, width, height, alpha)
    }
    #[inline] pub fn copy(source: &SoftwareBitmap) -> Result<Option<ComPtr<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().copy(source)
    }
    #[inline] pub fn convert(source: &SoftwareBitmap, format: BitmapPixelFormat) -> Result<Option<ComPtr<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().convert(source, format)
    }
    #[inline] pub fn convert_with_alpha(source: &SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> Result<Option<ComPtr<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().convert_with_alpha(source, format, alpha)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_from_buffer(source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32) -> Result<Option<ComPtr<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_from_buffer(source, format, width, height)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_with_alpha_from_buffer(source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<Option<ComPtr<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_with_alpha_from_buffer(source, format, width, height, alpha)
    }
    #[inline] pub fn create_copy_from_surface_async(surface: &super::directx::direct3d11::IDirect3DSurface) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_from_surface_async(surface)
    }
    #[inline] pub fn create_copy_with_alpha_from_surface_async(surface: &super::directx::direct3d11::IDirect3DSurface, alpha: BitmapAlphaMode) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_with_alpha_from_surface_async(surface, alpha)
    }
}
DEFINE_CLSID!(SoftwareBitmap(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,83,111,102,116,119,97,114,101,66,105,116,109,97,112,0]) [CLSID_SoftwareBitmap]);
DEFINE_IID!(IID_ISoftwareBitmapFactory, 3382700905, 11618, 19783, 166, 179, 79, 219, 106, 7, 253, 248);
RT_INTERFACE!{static interface ISoftwareBitmapFactory(ISoftwareBitmapFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmapFactory] {
    fn Create(&self, format: BitmapPixelFormat, width: i32, height: i32, out: *mut *mut SoftwareBitmap) -> HRESULT,
    fn CreateWithAlpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, out: *mut *mut SoftwareBitmap) -> HRESULT
}}
impl ISoftwareBitmapFactory {
    #[inline] pub fn create(&self, format: BitmapPixelFormat, width: i32, height: i32) -> Result<ComPtr<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, format, width, height, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_alpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<ComPtr<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithAlpha)(self as *const _ as *mut _, format, width, height, alpha, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISoftwareBitmapStatics, 3741550043, 26415, 19101, 128, 110, 194, 68, 47, 52, 62, 134);
RT_INTERFACE!{static interface ISoftwareBitmapStatics(ISoftwareBitmapStaticsVtbl): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmapStatics] {
    fn Copy(&self, source: *mut SoftwareBitmap, out: *mut *mut SoftwareBitmap) -> HRESULT,
    fn Convert(&self, source: *mut SoftwareBitmap, format: BitmapPixelFormat, out: *mut *mut SoftwareBitmap) -> HRESULT,
    fn ConvertWithAlpha(&self, source: *mut SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode, out: *mut *mut SoftwareBitmap) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateCopyFromBuffer(&self, source: *mut super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, out: *mut *mut SoftwareBitmap) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateCopyWithAlphaFromBuffer(&self, source: *mut super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, out: *mut *mut SoftwareBitmap) -> HRESULT,
    fn CreateCopyFromSurfaceAsync(&self, surface: *mut super::directx::direct3d11::IDirect3DSurface, out: *mut *mut foundation::IAsyncOperation<SoftwareBitmap>) -> HRESULT,
    fn CreateCopyWithAlphaFromSurfaceAsync(&self, surface: *mut super::directx::direct3d11::IDirect3DSurface, alpha: BitmapAlphaMode, out: *mut *mut foundation::IAsyncOperation<SoftwareBitmap>) -> HRESULT
}}
impl ISoftwareBitmapStatics {
    #[inline] pub fn copy(&self, source: &SoftwareBitmap) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Copy)(self as *const _ as *mut _, source as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn convert(&self, source: &SoftwareBitmap, format: BitmapPixelFormat) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Convert)(self as *const _ as *mut _, source as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn convert_with_alpha(&self, source: &SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ConvertWithAlpha)(self as *const _ as *mut _, source as *const _ as *mut _, format, alpha, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_from_buffer(&self, source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCopyFromBuffer)(self as *const _ as *mut _, source as *const _ as *mut _, format, width, height, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_with_alpha_from_buffer(&self, source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<Option<ComPtr<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCopyWithAlphaFromBuffer)(self as *const _ as *mut _, source as *const _ as *mut _, format, width, height, alpha, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_copy_from_surface_async(&self, surface: &super::directx::direct3d11::IDirect3DSurface) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCopyFromSurfaceAsync)(self as *const _ as *mut _, surface as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_copy_with_alpha_from_surface_async(&self, surface: &super::directx::direct3d11::IDirect3DSurface, alpha: BitmapAlphaMode) -> Result<ComPtr<foundation::IAsyncOperation<SoftwareBitmap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateCopyWithAlphaFromSurfaceAsync)(self as *const _ as *mut _, surface as *const _ as *mut _, alpha, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TiffCompressionMode: i32 {
    Automatic = 0, None = 1, Ccitt3 = 2, Ccitt4 = 3, Lzw = 4, Rle = 5, Zip = 6, LzwhDifferencing = 7,
}}
} // Windows.Graphics.Imaging
pub mod printing { // Windows.Graphics.Printing
use crate::prelude::*;
RT_ENUM! { enum PrintBinding: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, None = 3, Bale = 4, BindBottom = 5, BindLeft = 6, BindRight = 7, BindTop = 8, Booklet = 9, EdgeStitchBottom = 10, EdgeStitchLeft = 11, EdgeStitchRight = 12, EdgeStitchTop = 13, Fold = 14, JogOffset = 15, Trim = 16,
}}
RT_ENUM! { enum PrintBordering: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, Bordered = 3, Borderless = 4,
}}
RT_ENUM! { enum PrintCollation: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, Collated = 3, Uncollated = 4,
}}
RT_ENUM! { enum PrintColorMode: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, Color = 3, Grayscale = 4, Monochrome = 5,
}}
DEFINE_IID!(IID_IPrintDocumentSource, 3738962992, 61931, 18399, 170, 230, 237, 84, 39, 81, 31, 1);
RT_INTERFACE!{interface IPrintDocumentSource(IPrintDocumentSourceVtbl): IInspectable(IInspectableVtbl) [IID_IPrintDocumentSource] {
    
}}
RT_ENUM! { enum PrintDuplex: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, OneSided = 3, TwoSidedShortEdge = 4, TwoSidedLongEdge = 5,
}}
RT_ENUM! { enum PrintHolePunch: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, None = 3, LeftEdge = 4, RightEdge = 5, TopEdge = 6, BottomEdge = 7,
}}
DEFINE_IID!(IID_IPrintManager, 4280981140, 35993, 17661, 174, 74, 25, 217, 170, 154, 15, 10);
RT_INTERFACE!{interface IPrintManager(IPrintManagerVtbl): IInspectable(IInspectableVtbl) [IID_IPrintManager] {
    fn add_PrintTaskRequested(&self, eventHandler: *mut foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PrintTaskRequested(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintManager {
    #[inline] pub fn add_print_task_requested(&self, eventHandler: &foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_PrintTaskRequested)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_print_task_requested(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_PrintTaskRequested)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintManager: IPrintManager}
impl RtActivatable<IPrintManagerStatic> for PrintManager {}
impl RtActivatable<IPrintManagerStatic2> for PrintManager {}
impl PrintManager {
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<PrintManager>>> {
        <Self as RtActivatable<IPrintManagerStatic>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn show_print_ui_async() -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<IPrintManagerStatic>>::get_activation_factory().show_print_ui_async()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IPrintManagerStatic2>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(PrintManager(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,77,97,110,97,103,101,114,0]) [CLSID_PrintManager]);
DEFINE_IID!(IID_IPrintManagerStatic, 1477991885, 58932, 18004, 132, 240, 224, 21, 42, 130, 23, 172);
RT_INTERFACE!{static interface IPrintManagerStatic(IPrintManagerStaticVtbl): IInspectable(IInspectableVtbl) [IID_IPrintManagerStatic] {
    fn GetForCurrentView(&self, out: *mut *mut PrintManager) -> HRESULT,
    fn ShowPrintUIAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IPrintManagerStatic {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<PrintManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_print_ui_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ShowPrintUIAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintManagerStatic2, 900307285, 59051, 16697, 154, 189, 184, 106, 114, 155, 53, 152);
RT_INTERFACE!{static interface IPrintManagerStatic2(IPrintManagerStatic2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintManagerStatic2] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IPrintManagerStatic2 {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).IsSupported)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum PrintMediaSize: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, BusinessCard = 3, CreditCard = 4, IsoA0 = 5, IsoA1 = 6, IsoA10 = 7, IsoA2 = 8, IsoA3 = 9, IsoA3Extra = 10, IsoA3Rotated = 11, IsoA4 = 12, IsoA4Extra = 13, IsoA4Rotated = 14, IsoA5 = 15, IsoA5Extra = 16, IsoA5Rotated = 17, IsoA6 = 18, IsoA6Rotated = 19, IsoA7 = 20, IsoA8 = 21, IsoA9 = 22, IsoB0 = 23, IsoB1 = 24, IsoB10 = 25, IsoB2 = 26, IsoB3 = 27, IsoB4 = 28, IsoB4Envelope = 29, IsoB5Envelope = 30, IsoB5Extra = 31, IsoB7 = 32, IsoB8 = 33, IsoB9 = 34, IsoC0 = 35, IsoC1 = 36, IsoC10 = 37, IsoC2 = 38, IsoC3 = 39, IsoC3Envelope = 40, IsoC4 = 41, IsoC4Envelope = 42, IsoC5 = 43, IsoC5Envelope = 44, IsoC6 = 45, IsoC6C5Envelope = 46, IsoC6Envelope = 47, IsoC7 = 48, IsoC8 = 49, IsoC9 = 50, IsoDLEnvelope = 51, IsoDLEnvelopeRotated = 52, IsoSRA3 = 53, Japan2LPhoto = 54, JapanChou3Envelope = 55, JapanChou3EnvelopeRotated = 56, JapanChou4Envelope = 57, JapanChou4EnvelopeRotated = 58, JapanDoubleHagakiPostcard = 59, JapanDoubleHagakiPostcardRotated = 60, JapanHagakiPostcard = 61, JapanHagakiPostcardRotated = 62, JapanKaku2Envelope = 63, JapanKaku2EnvelopeRotated = 64, JapanKaku3Envelope = 65, JapanKaku3EnvelopeRotated = 66, JapanLPhoto = 67, JapanQuadrupleHagakiPostcard = 68, JapanYou1Envelope = 69, JapanYou2Envelope = 70, JapanYou3Envelope = 71, JapanYou4Envelope = 72, JapanYou4EnvelopeRotated = 73, JapanYou6Envelope = 74, JapanYou6EnvelopeRotated = 75, JisB0 = 76, JisB1 = 77, JisB10 = 78, JisB2 = 79, JisB3 = 80, JisB4 = 81, JisB4Rotated = 82, JisB5 = 83, JisB5Rotated = 84, JisB6 = 85, JisB6Rotated = 86, JisB7 = 87, JisB8 = 88, JisB9 = 89, NorthAmerica10x11 = 90, NorthAmerica10x12 = 91, NorthAmerica10x14 = 92, NorthAmerica11x17 = 93, NorthAmerica14x17 = 94, NorthAmerica4x6 = 95, NorthAmerica4x8 = 96, NorthAmerica5x7 = 97, NorthAmerica8x10 = 98, NorthAmerica9x11 = 99, NorthAmericaArchitectureASheet = 100, NorthAmericaArchitectureBSheet = 101, NorthAmericaArchitectureCSheet = 102, NorthAmericaArchitectureDSheet = 103, NorthAmericaArchitectureESheet = 104, NorthAmericaCSheet = 105, NorthAmericaDSheet = 106, NorthAmericaESheet = 107, NorthAmericaExecutive = 108, NorthAmericaGermanLegalFanfold = 109, NorthAmericaGermanStandardFanfold = 110, NorthAmericaLegal = 111, NorthAmericaLegalExtra = 112, NorthAmericaLetter = 113, NorthAmericaLetterExtra = 114, NorthAmericaLetterPlus = 115, NorthAmericaLetterRotated = 116, NorthAmericaMonarchEnvelope = 117, NorthAmericaNote = 118, NorthAmericaNumber10Envelope = 119, NorthAmericaNumber10EnvelopeRotated = 120, NorthAmericaNumber11Envelope = 121, NorthAmericaNumber12Envelope = 122, NorthAmericaNumber14Envelope = 123, NorthAmericaNumber9Envelope = 124, NorthAmericaPersonalEnvelope = 125, NorthAmericaQuarto = 126, NorthAmericaStatement = 127, NorthAmericaSuperA = 128, NorthAmericaSuperB = 129, NorthAmericaTabloid = 130, NorthAmericaTabloidExtra = 131, OtherMetricA3Plus = 132, OtherMetricA4Plus = 133, OtherMetricFolio = 134, OtherMetricInviteEnvelope = 135, OtherMetricItalianEnvelope = 136, Prc10Envelope = 137, Prc10EnvelopeRotated = 138, Prc16K = 139, Prc16KRotated = 140, Prc1Envelope = 141, Prc1EnvelopeRotated = 142, Prc2Envelope = 143, Prc2EnvelopeRotated = 144, Prc32K = 145, Prc32KBig = 146, Prc32KRotated = 147, Prc3Envelope = 148, Prc3EnvelopeRotated = 149, Prc4Envelope = 150, Prc4EnvelopeRotated = 151, Prc5Envelope = 152, Prc5EnvelopeRotated = 153, Prc6Envelope = 154, Prc6EnvelopeRotated = 155, Prc7Envelope = 156, Prc7EnvelopeRotated = 157, Prc8Envelope = 158, Prc8EnvelopeRotated = 159, Prc9Envelope = 160, Prc9EnvelopeRotated = 161, Roll04Inch = 162, Roll06Inch = 163, Roll08Inch = 164, Roll12Inch = 165, Roll15Inch = 166, Roll18Inch = 167, Roll22Inch = 168, Roll24Inch = 169, Roll30Inch = 170, Roll36Inch = 171, Roll54Inch = 172,
}}
RT_ENUM! { enum PrintMediaType: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, AutoSelect = 3, Archival = 4, BackPrintFilm = 5, Bond = 6, CardStock = 7, Continuous = 8, EnvelopePlain = 9, EnvelopeWindow = 10, Fabric = 11, HighResolution = 12, Label = 13, MultiLayerForm = 14, MultiPartForm = 15, Photographic = 16, PhotographicFilm = 17, PhotographicGlossy = 18, PhotographicHighGloss = 19, PhotographicMatte = 20, PhotographicSatin = 21, PhotographicSemiGloss = 22, Plain = 23, Screen = 24, ScreenPaged = 25, Stationery = 26, TabStockFull = 27, TabStockPreCut = 28, Transparency = 29, TShirtTransfer = 30, None = 31,
}}
RT_ENUM! { enum PrintOrientation: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, Portrait = 3, PortraitFlipped = 4, Landscape = 5, LandscapeFlipped = 6,
}}
RT_STRUCT! { struct PrintPageDescription {
    PageSize: foundation::Size, ImageableRect: foundation::Rect, DpiX: u32, DpiY: u32,
}}
DEFINE_IID!(IID_IPrintPageInfo, 3712739785, 42657, 19162, 147, 14, 218, 135, 42, 79, 35, 211);
RT_INTERFACE!{interface IPrintPageInfo(IPrintPageInfoVtbl): IInspectable(IInspectableVtbl) [IID_IPrintPageInfo] {
    fn put_MediaSize(&self, value: PrintMediaSize) -> HRESULT,
    fn get_MediaSize(&self, out: *mut PrintMediaSize) -> HRESULT,
    fn put_PageSize(&self, value: foundation::Size) -> HRESULT,
    fn get_PageSize(&self, out: *mut foundation::Size) -> HRESULT,
    fn put_DpiX(&self, value: u32) -> HRESULT,
    fn get_DpiX(&self, out: *mut u32) -> HRESULT,
    fn put_DpiY(&self, value: u32) -> HRESULT,
    fn get_DpiY(&self, out: *mut u32) -> HRESULT,
    fn put_Orientation(&self, value: PrintOrientation) -> HRESULT,
    fn get_Orientation(&self, out: *mut PrintOrientation) -> HRESULT
}}
impl IPrintPageInfo {
    #[inline] pub fn set_media_size(&self, value: PrintMediaSize) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MediaSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_size(&self) -> Result<PrintMediaSize> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MediaSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_page_size(&self, value: foundation::Size) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PageSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_page_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PageSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_x(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DpiX)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiX)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_y(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DpiY)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DpiY)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_orientation(&self, value: PrintOrientation) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Orientation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<PrintOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageInfo: IPrintPageInfo}
impl RtActivatable<IActivationFactory> for PrintPageInfo {}
DEFINE_CLSID!(PrintPageInfo(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,80,97,103,101,73,110,102,111,0]) [CLSID_PrintPageInfo]);
DEFINE_IID!(IID_IPrintPageRange, 4171263060, 28284, 20933, 87, 253, 6, 96, 194, 215, 21, 19);
RT_INTERFACE!{interface IPrintPageRange(IPrintPageRangeVtbl): IInspectable(IInspectableVtbl) [IID_IPrintPageRange] {
    fn get_FirstPageNumber(&self, out: *mut i32) -> HRESULT,
    fn get_LastPageNumber(&self, out: *mut i32) -> HRESULT
}}
impl IPrintPageRange {
    #[inline] pub fn get_first_page_number(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_FirstPageNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_page_number(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_LastPageNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageRange: IPrintPageRange}
impl RtActivatable<IPrintPageRangeFactory> for PrintPageRange {}
impl PrintPageRange {
    #[inline] pub fn create(firstPage: i32, lastPage: i32) -> Result<ComPtr<PrintPageRange>> {
        <Self as RtActivatable<IPrintPageRangeFactory>>::get_activation_factory().create(firstPage, lastPage)
    }
    #[inline] pub fn create_with_single_page(page: i32) -> Result<ComPtr<PrintPageRange>> {
        <Self as RtActivatable<IPrintPageRangeFactory>>::get_activation_factory().create_with_single_page(page)
    }
}
DEFINE_CLSID!(PrintPageRange(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,80,97,103,101,82,97,110,103,101,0]) [CLSID_PrintPageRange]);
DEFINE_IID!(IID_IPrintPageRangeFactory, 1083167839, 57415, 24453, 113, 41, 251, 8, 90, 79, 173, 20);
RT_INTERFACE!{static interface IPrintPageRangeFactory(IPrintPageRangeFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeFactory] {
    fn Create(&self, firstPage: i32, lastPage: i32, out: *mut *mut PrintPageRange) -> HRESULT,
    fn CreateWithSinglePage(&self, page: i32, out: *mut *mut PrintPageRange) -> HRESULT
}}
impl IPrintPageRangeFactory {
    #[inline] pub fn create(&self, firstPage: i32, lastPage: i32) -> Result<ComPtr<PrintPageRange>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, firstPage, lastPage, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_single_page(&self, page: i32) -> Result<ComPtr<PrintPageRange>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateWithSinglePage)(self as *const _ as *mut _, page, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintPageRangeOptions, 3463296808, 4951, 18098, 169, 35, 121, 249, 149, 244, 72, 252);
RT_INTERFACE!{interface IPrintPageRangeOptions(IPrintPageRangeOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeOptions] {
    fn put_AllowAllPages(&self, value: bool) -> HRESULT,
    fn get_AllowAllPages(&self, out: *mut bool) -> HRESULT,
    fn put_AllowCurrentPage(&self, value: bool) -> HRESULT,
    fn get_AllowCurrentPage(&self, out: *mut bool) -> HRESULT,
    fn put_AllowCustomSetOfPages(&self, value: bool) -> HRESULT,
    fn get_AllowCustomSetOfPages(&self, out: *mut bool) -> HRESULT
}}
impl IPrintPageRangeOptions {
    #[inline] pub fn set_allow_all_pages(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowAllPages)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_all_pages(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowAllPages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_current_page(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowCurrentPage)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_current_page(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowCurrentPage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_custom_set_of_pages(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_AllowCustomSetOfPages)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_custom_set_of_pages(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_AllowCustomSetOfPages)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageRangeOptions: IPrintPageRangeOptions}
RT_ENUM! { enum PrintQuality: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, Automatic = 3, Draft = 4, Fax = 5, High = 6, Normal = 7, Photographic = 8, Text = 9,
}}
RT_ENUM! { enum PrintStaple: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, None = 3, StapleTopLeft = 4, StapleTopRight = 5, StapleBottomLeft = 6, StapleBottomRight = 7, StapleDualLeft = 8, StapleDualRight = 9, StapleDualTop = 10, StapleDualBottom = 11, SaddleStitch = 12,
}}
DEFINE_IID!(IID_IPrintTask, 1641546311, 27894, 20397, 132, 226, 165, 232, 46, 45, 76, 235);
RT_INTERFACE!{interface IPrintTask(IPrintTaskVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTask] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Properties(&self, out: *mut *mut super::super::applicationmodel::datatransfer::DataPackagePropertySet) -> HRESULT,
    fn get_Source(&self, out: *mut *mut IPrintDocumentSource) -> HRESULT,
    fn get_Options(&self, out: *mut *mut PrintTaskOptions) -> HRESULT,
    fn add_Previewing(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTask, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Previewing(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Submitting(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTask, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitting(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Progressing(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Progressing(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Completed(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Completed(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintTask {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_properties(&self) -> Result<Option<ComPtr<super::super::applicationmodel::datatransfer::DataPackagePropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Properties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<Option<ComPtr<IPrintDocumentSource>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self) -> Result<Option<ComPtr<PrintTaskOptions>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Options)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_previewing(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Previewing)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_previewing(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Previewing)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_submitting(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Submitting)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitting(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Submitting)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_progressing(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Progressing)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_progressing(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Progressing)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_completed(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Completed)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_completed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Completed)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTask: IPrintTask}
DEFINE_IID!(IID_IPrintTask2, 908281975, 15955, 19869, 143, 94, 49, 106, 200, 222, 218, 225);
RT_INTERFACE!{interface IPrintTask2(IPrintTask2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintTask2] {
    fn put_IsPreviewEnabled(&self, value: bool) -> HRESULT,
    fn get_IsPreviewEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IPrintTask2 {
    #[inline] pub fn set_is_preview_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsPreviewEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_preview_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPreviewEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskCompletedEventArgs, 1540175023, 9449, 19472, 141, 7, 20, 195, 70, 186, 63, 206);
RT_INTERFACE!{interface IPrintTaskCompletedEventArgs(IPrintTaskCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskCompletedEventArgs] {
    fn get_Completion(&self, out: *mut PrintTaskCompletion) -> HRESULT
}}
impl IPrintTaskCompletedEventArgs {
    #[inline] pub fn get_completion(&self) -> Result<PrintTaskCompletion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Completion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskCompletedEventArgs: IPrintTaskCompletedEventArgs}
RT_ENUM! { enum PrintTaskCompletion: i32 {
    Abandoned = 0, Canceled = 1, Failed = 2, Submitted = 3,
}}
DEFINE_IID!(IID_IPrintTaskOptions, 1510631099, 53897, 16827, 150, 221, 87, 226, 131, 56, 174, 63);
RT_INTERFACE!{interface IPrintTaskOptions(IPrintTaskOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptions] {
    fn put_Bordering(&self, value: PrintBordering) -> HRESULT,
    fn get_Bordering(&self, out: *mut PrintBordering) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetPagePrintTicket(&self, printPageInfo: *mut PrintPageInfo, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT
}}
impl IPrintTaskOptions {
    #[inline] pub fn set_bordering(&self, value: PrintBordering) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Bordering)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bordering(&self) -> Result<PrintBordering> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Bordering)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_page_print_ticket(&self, printPageInfo: &PrintPageInfo) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPagePrintTicket)(self as *const _ as *mut _, printPageInfo as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptions: IPrintTaskOptionsCore}
DEFINE_IID!(IID_IPrintTaskOptions2, 3952809478, 39478, 19289, 134, 23, 178, 23, 132, 146, 98, 225);
RT_INTERFACE!{interface IPrintTaskOptions2(IPrintTaskOptions2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptions2] {
    fn get_PageRangeOptions(&self, out: *mut *mut PrintPageRangeOptions) -> HRESULT,
    fn get_CustomPageRanges(&self, out: *mut *mut foundation::collections::IVector<PrintPageRange>) -> HRESULT
}}
impl IPrintTaskOptions2 {
    #[inline] pub fn get_page_range_options(&self) -> Result<Option<ComPtr<PrintPageRangeOptions>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageRangeOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_page_ranges(&self) -> Result<Option<ComPtr<foundation::collections::IVector<PrintPageRange>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomPageRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCore, 467383412, 20177, 16875, 190, 60, 114, 209, 142, 214, 115, 55);
RT_INTERFACE!{interface IPrintTaskOptionsCore(IPrintTaskOptionsCoreVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCore] {
    fn GetPageDescription(&self, jobPageNumber: u32, out: *mut PrintPageDescription) -> HRESULT
}}
impl IPrintTaskOptionsCore {
    #[inline] pub fn get_page_description(&self, jobPageNumber: u32) -> Result<PrintPageDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetPageDescription)(self as *const _ as *mut _, jobPageNumber, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCoreProperties, 3250001970, 40595, 20053, 129, 75, 51, 38, 165, 158, 252, 225);
RT_INTERFACE!{interface IPrintTaskOptionsCoreProperties(IPrintTaskOptionsCorePropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCoreProperties] {
    fn put_MediaSize(&self, value: PrintMediaSize) -> HRESULT,
    fn get_MediaSize(&self, out: *mut PrintMediaSize) -> HRESULT,
    fn put_MediaType(&self, value: PrintMediaType) -> HRESULT,
    fn get_MediaType(&self, out: *mut PrintMediaType) -> HRESULT,
    fn put_Orientation(&self, value: PrintOrientation) -> HRESULT,
    fn get_Orientation(&self, out: *mut PrintOrientation) -> HRESULT,
    fn put_PrintQuality(&self, value: PrintQuality) -> HRESULT,
    fn get_PrintQuality(&self, out: *mut PrintQuality) -> HRESULT,
    fn put_ColorMode(&self, value: PrintColorMode) -> HRESULT,
    fn get_ColorMode(&self, out: *mut PrintColorMode) -> HRESULT,
    fn put_Duplex(&self, value: PrintDuplex) -> HRESULT,
    fn get_Duplex(&self, out: *mut PrintDuplex) -> HRESULT,
    fn put_Collation(&self, value: PrintCollation) -> HRESULT,
    fn get_Collation(&self, out: *mut PrintCollation) -> HRESULT,
    fn put_Staple(&self, value: PrintStaple) -> HRESULT,
    fn get_Staple(&self, out: *mut PrintStaple) -> HRESULT,
    fn put_HolePunch(&self, value: PrintHolePunch) -> HRESULT,
    fn get_HolePunch(&self, out: *mut PrintHolePunch) -> HRESULT,
    fn put_Binding(&self, value: PrintBinding) -> HRESULT,
    fn get_Binding(&self, out: *mut PrintBinding) -> HRESULT,
    fn get_MinCopies(&self, out: *mut u32) -> HRESULT,
    fn get_MaxCopies(&self, out: *mut u32) -> HRESULT,
    fn put_NumberOfCopies(&self, value: u32) -> HRESULT,
    fn get_NumberOfCopies(&self, out: *mut u32) -> HRESULT
}}
impl IPrintTaskOptionsCoreProperties {
    #[inline] pub fn set_media_size(&self, value: PrintMediaSize) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MediaSize)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_size(&self) -> Result<PrintMediaSize> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MediaSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_media_type(&self, value: PrintMediaType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MediaType)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<PrintMediaType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MediaType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_orientation(&self, value: PrintOrientation) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Orientation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<PrintOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_print_quality(&self, value: PrintQuality) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PrintQuality)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_print_quality(&self) -> Result<PrintQuality> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_PrintQuality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_color_mode(&self, value: PrintColorMode) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ColorMode)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_color_mode(&self) -> Result<PrintColorMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ColorMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_duplex(&self, value: PrintDuplex) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Duplex)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_duplex(&self) -> Result<PrintDuplex> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Duplex)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_collation(&self, value: PrintCollation) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Collation)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_collation(&self) -> Result<PrintCollation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Collation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_staple(&self, value: PrintStaple) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Staple)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_staple(&self) -> Result<PrintStaple> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Staple)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hole_punch(&self, value: PrintHolePunch) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_HolePunch)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hole_punch(&self) -> Result<PrintHolePunch> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_HolePunch)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_binding(&self, value: PrintBinding) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Binding)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_binding(&self) -> Result<PrintBinding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Binding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinCopies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxCopies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_number_of_copies(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_NumberOfCopies)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_NumberOfCopies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCoreUIConfiguration, 1659280931, 39454, 17206, 183, 79, 60, 199, 244, 207, 247, 9);
RT_INTERFACE!{interface IPrintTaskOptionsCoreUIConfiguration(IPrintTaskOptionsCoreUIConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCoreUIConfiguration] {
    fn get_DisplayedOptions(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT
}}
impl IPrintTaskOptionsCoreUIConfiguration {
    #[inline] pub fn get_displayed_options(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayedOptions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskProgressingEventArgs, 2165101515, 46096, 17026, 160, 115, 90, 195, 120, 35, 65, 116);
RT_INTERFACE!{interface IPrintTaskProgressingEventArgs(IPrintTaskProgressingEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskProgressingEventArgs] {
    fn get_DocumentPageCount(&self, out: *mut u32) -> HRESULT
}}
impl IPrintTaskProgressingEventArgs {
    #[inline] pub fn get_document_page_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DocumentPageCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskProgressingEventArgs: IPrintTaskProgressingEventArgs}
DEFINE_IID!(IID_IPrintTaskRequest, 1878400558, 10018, 16960, 166, 124, 243, 100, 132, 154, 23, 243);
RT_INTERFACE!{interface IPrintTaskRequest(IPrintTaskRequestVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequest] {
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn CreatePrintTask(&self, title: HSTRING, handler: *mut PrintTaskSourceRequestedHandler, out: *mut *mut PrintTask) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut PrintTaskRequestedDeferral) -> HRESULT
}}
impl IPrintTaskRequest {
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Deadline)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn create_print_task(&self, title: &HStringArg, handler: &PrintTaskSourceRequestedHandler) -> Result<Option<ComPtr<PrintTask>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreatePrintTask)(self as *const _ as *mut _, title.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<PrintTaskRequestedDeferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequest: IPrintTaskRequest}
DEFINE_IID!(IID_IPrintTaskRequestedDeferral, 3488592880, 52798, 17095, 148, 150, 100, 128, 12, 98, 44, 68);
RT_INTERFACE!{interface IPrintTaskRequestedDeferral(IPrintTaskRequestedDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequestedDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPrintTaskRequestedDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequestedDeferral: IPrintTaskRequestedDeferral}
DEFINE_IID!(IID_IPrintTaskRequestedEventArgs, 3501193508, 41755, 17740, 167, 182, 93, 12, 197, 34, 252, 22);
RT_INTERFACE!{interface IPrintTaskRequestedEventArgs(IPrintTaskRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequestedEventArgs] {
    fn get_Request(&self, out: *mut *mut PrintTaskRequest) -> HRESULT
}}
impl IPrintTaskRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<ComPtr<PrintTaskRequest>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Request)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequestedEventArgs: IPrintTaskRequestedEventArgs}
DEFINE_IID!(IID_IPrintTaskSourceRequestedArgs, 4193281982, 62550, 16880, 156, 152, 92, 231, 62, 133, 20, 16);
RT_INTERFACE!{interface IPrintTaskSourceRequestedArgs(IPrintTaskSourceRequestedArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskSourceRequestedArgs] {
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn SetSource(&self, source: *mut IPrintDocumentSource) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut PrintTaskSourceRequestedDeferral) -> HRESULT
}}
impl IPrintTaskSourceRequestedArgs {
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Deadline)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, source: &IPrintDocumentSource) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetSource)(self as *const _ as *mut _, source as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<PrintTaskSourceRequestedDeferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskSourceRequestedArgs: IPrintTaskSourceRequestedArgs}
DEFINE_IID!(IID_IPrintTaskSourceRequestedDeferral, 1242915025, 27026, 19869, 133, 85, 76, 164, 86, 63, 177, 102);
RT_INTERFACE!{interface IPrintTaskSourceRequestedDeferral(IPrintTaskSourceRequestedDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskSourceRequestedDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPrintTaskSourceRequestedDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskSourceRequestedDeferral: IPrintTaskSourceRequestedDeferral}
DEFINE_IID!(IID_PrintTaskSourceRequestedHandler, 1813028776, 23734, 19258, 134, 99, 243, 156, 176, 45, 201, 180);
RT_DELEGATE!{delegate PrintTaskSourceRequestedHandler(PrintTaskSourceRequestedHandlerVtbl, PrintTaskSourceRequestedHandlerImpl) [IID_PrintTaskSourceRequestedHandler] {
    fn Invoke(&self, args: *mut PrintTaskSourceRequestedArgs) -> HRESULT
}}
impl PrintTaskSourceRequestedHandler {
    #[inline] pub fn invoke(&self, args: &PrintTaskSourceRequestedArgs) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, args as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskTargetDeviceSupport, 693989568, 49867, 19325, 176, 234, 147, 9, 80, 145, 162, 32);
RT_INTERFACE!{interface IPrintTaskTargetDeviceSupport(IPrintTaskTargetDeviceSupportVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskTargetDeviceSupport] {
    fn put_IsPrinterTargetEnabled(&self, value: bool) -> HRESULT,
    fn get_IsPrinterTargetEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_Is3DManufacturingTargetEnabled(&self, value: bool) -> HRESULT,
    fn get_Is3DManufacturingTargetEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IPrintTaskTargetDeviceSupport {
    #[inline] pub fn set_is_printer_target_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IsPrinterTargetEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_printer_target_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsPrinterTargetEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_3d_manufacturing_target_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Is3DManufacturingTargetEnabled)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_3d_manufacturing_target_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Is3DManufacturingTargetEnabled)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{static class StandardPrintTaskOptions}
impl RtActivatable<IStandardPrintTaskOptionsStatic> for StandardPrintTaskOptions {}
impl RtActivatable<IStandardPrintTaskOptionsStatic2> for StandardPrintTaskOptions {}
impl RtActivatable<IStandardPrintTaskOptionsStatic3> for StandardPrintTaskOptions {}
impl StandardPrintTaskOptions {
    #[inline] pub fn get_media_size() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_media_size()
    }
    #[inline] pub fn get_media_type() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_media_type()
    }
    #[inline] pub fn get_orientation() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_orientation()
    }
    #[inline] pub fn get_print_quality() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_print_quality()
    }
    #[inline] pub fn get_color_mode() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_color_mode()
    }
    #[inline] pub fn get_duplex() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_duplex()
    }
    #[inline] pub fn get_collation() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_collation()
    }
    #[inline] pub fn get_staple() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_staple()
    }
    #[inline] pub fn get_hole_punch() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_hole_punch()
    }
    #[inline] pub fn get_binding() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_binding()
    }
    #[inline] pub fn get_copies() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_copies()
    }
    #[inline] pub fn get_n_up() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_n_up()
    }
    #[inline] pub fn get_input_bin() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic>>::get_activation_factory().get_input_bin()
    }
    #[inline] pub fn get_bordering() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic2>>::get_activation_factory().get_bordering()
    }
    #[inline] pub fn get_custom_page_ranges() -> Result<HString> {
        <Self as RtActivatable<IStandardPrintTaskOptionsStatic3>>::get_activation_factory().get_custom_page_ranges()
    }
}
DEFINE_CLSID!(StandardPrintTaskOptions(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,83,116,97,110,100,97,114,100,80,114,105,110,116,84,97,115,107,79,112,116,105,111,110,115,0]) [CLSID_StandardPrintTaskOptions]);
DEFINE_IID!(IID_IStandardPrintTaskOptionsStatic, 3024633126, 3536, 19668, 186, 255, 147, 15, 199, 214, 165, 116);
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic(IStandardPrintTaskOptionsStaticVtbl): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic] {
    fn get_MediaSize(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MediaType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Orientation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_PrintQuality(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ColorMode(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Duplex(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Collation(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Staple(&self, out: *mut HSTRING) -> HRESULT,
    fn get_HolePunch(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Binding(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Copies(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NUp(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InputBin(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardPrintTaskOptionsStatic {
    #[inline] pub fn get_media_size(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaSize)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MediaType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Orientation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_print_quality(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrintQuality)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_color_mode(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ColorMode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_duplex(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Duplex)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Collation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_staple(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Staple)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hole_punch(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_HolePunch)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_binding(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Binding)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_copies(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Copies)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_n_up(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NUp)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_bin(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_InputBin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStandardPrintTaskOptionsStatic2, 1004768244, 31300, 17001, 154, 82, 129, 38, 30, 40, 158, 233);
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic2(IStandardPrintTaskOptionsStatic2Vtbl): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic2] {
    fn get_Bordering(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardPrintTaskOptionsStatic2 {
    #[inline] pub fn get_bordering(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Bordering)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStandardPrintTaskOptionsStatic3, 3153497734, 14424, 16819, 167, 153, 85, 221, 152, 136, 212, 117);
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic3(IStandardPrintTaskOptionsStatic3Vtbl): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic3] {
    fn get_CustomPageRanges(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardPrintTaskOptionsStatic3 {
    #[inline] pub fn get_custom_page_ranges(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CustomPageRanges)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
pub mod optiondetails { // Windows.Graphics.Printing.OptionDetails
use crate::prelude::*;
DEFINE_IID!(IID_IPrintBindingOptionDetails, 3287600280, 38244, 20246, 160, 85, 169, 139, 154, 73, 233, 211);
RT_INTERFACE!{interface IPrintBindingOptionDetails(IPrintBindingOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintBindingOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintBindingOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintBindingOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintBorderingOptionDetails, 1299430543, 64339, 20146, 152, 95, 29, 145, 222, 11, 118, 57);
RT_INTERFACE!{interface IPrintBorderingOptionDetails(IPrintBorderingOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintBorderingOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintBorderingOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintBorderingOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCollationOptionDetails, 3601576294, 42406, 16604, 172, 195, 115, 159, 40, 241, 229, 211);
RT_INTERFACE!{interface IPrintCollationOptionDetails(IPrintCollationOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCollationOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCollationOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCollationOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintColorModeOptionDetails, 3685316356, 61910, 18499, 164, 132, 155, 68, 124, 220, 243, 182);
RT_INTERFACE!{interface IPrintColorModeOptionDetails(IPrintColorModeOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintColorModeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintColorModeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintColorModeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCopiesOptionDetails, 1107636377, 17209, 17219, 137, 141, 44, 71, 181, 224, 195, 65);
RT_INTERFACE!{interface IPrintCopiesOptionDetails(IPrintCopiesOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCopiesOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCopiesOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCopiesOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomItemDetails, 1459926583, 23610, 17562, 170, 54, 179, 41, 27, 17, 146, 253);
RT_INTERFACE!{interface IPrintCustomItemDetails(IPrintCustomItemDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemDetails] {
    fn get_ItemId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ItemDisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_ItemDisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomItemDetails {
    #[inline] pub fn get_item_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_item_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ItemDisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_item_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ItemDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomItemDetails: IPrintCustomItemDetails}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails, 2784689544, 22770, 20157, 185, 15, 81, 228, 242, 148, 76, 93);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails(IPrintCustomItemListOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails] {
    fn AddItem(&self, itemId: HSTRING, displayName: HSTRING) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails {
    #[inline] pub fn add_item(&self, itemId: &HStringArg, displayName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddItem)(self as *const _ as *mut _, itemId.get(), displayName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomItemListOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails2, 3386258749, 25884, 19001, 144, 110, 16, 145, 161, 128, 27, 241);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails2(IPrintCustomItemListOptionDetails2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails2] {
    #[cfg(feature="windows-storage")] fn AddItem(&self, itemId: HSTRING, displayName: HSTRING, description: HSTRING, icon: *mut crate::windows::storage::streams::IRandomAccessStreamWithContentType) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn add_item(&self, itemId: &HStringArg, displayName: &HStringArg, description: &HStringArg, icon: &crate::windows::storage::streams::IRandomAccessStreamWithContentType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).AddItem)(self as *const _ as *mut _, itemId.get(), displayName.get(), description.get(), icon as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails3, 1335997759, 15412, 18536, 164, 7, 252, 94, 171, 37, 155, 33);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails3(IPrintCustomItemListOptionDetails3Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails3] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails3 {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomOptionDetails, 3811302940, 10415, 19344, 149, 218, 163, 172, 243, 32, 185, 41);
RT_INTERFACE!{interface IPrintCustomOptionDetails(IPrintCustomOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomOptionDetails] {
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomOptionDetails {
    #[inline] pub fn set_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_DisplayName)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomTextOptionDetails, 718369272, 51389, 18693, 145, 146, 13, 117, 19, 110, 139, 49);
RT_INTERFACE!{interface IPrintCustomTextOptionDetails(IPrintCustomTextOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomTextOptionDetails] {
    fn put_MaxCharacters(&self, value: u32) -> HRESULT,
    fn get_MaxCharacters(&self, out: *mut u32) -> HRESULT
}}
impl IPrintCustomTextOptionDetails {
    #[inline] pub fn set_max_characters(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxCharacters)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_characters(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxCharacters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomTextOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomTextOptionDetails2, 3467053908, 47479, 18200, 131, 56, 126, 210, 176, 216, 111, 227);
RT_INTERFACE!{interface IPrintCustomTextOptionDetails2(IPrintCustomTextOptionDetails2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomTextOptionDetails2] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomTextOptionDetails2 {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomToggleOptionDetails, 2645873940, 58465, 17928, 142, 233, 219, 111, 94, 208, 115, 198);
RT_INTERFACE!{interface IPrintCustomToggleOptionDetails(IPrintCustomToggleOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintCustomToggleOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomToggleOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomToggleOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintDuplexOptionDetails, 4242097553, 54436, 17658, 179, 254, 66, 224, 186, 40, 213, 173);
RT_INTERFACE!{interface IPrintDuplexOptionDetails(IPrintDuplexOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintDuplexOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintDuplexOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintDuplexOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintHolePunchOptionDetails, 2799574808, 18476, 18007, 157, 113, 141, 221, 219, 234, 30, 30);
RT_INTERFACE!{interface IPrintHolePunchOptionDetails(IPrintHolePunchOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintHolePunchOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintHolePunchOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintHolePunchOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintItemListOptionDetails, 2585941951, 65121, 17368, 162, 79, 163, 246, 171, 115, 32, 231);
RT_INTERFACE!{interface IPrintItemListOptionDetails(IPrintItemListOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintItemListOptionDetails] {
    fn get_Items(&self, out: *mut *mut foundation::collections::IVectorView<IInspectable>) -> HRESULT
}}
impl IPrintItemListOptionDetails {
    #[inline] pub fn get_items(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<IInspectable>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Items)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintMediaSizeOptionDetails, 1821203407, 49343, 18376, 184, 74, 98, 142, 125, 13, 26, 29);
RT_INTERFACE!{interface IPrintMediaSizeOptionDetails(IPrintMediaSizeOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintMediaSizeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintMediaSizeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintMediaSizeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintMediaTypeOptionDetails, 4173791243, 44019, 19132, 142, 134, 34, 171, 197, 116, 74, 67);
RT_INTERFACE!{interface IPrintMediaTypeOptionDetails(IPrintMediaTypeOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintMediaTypeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintMediaTypeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintMediaTypeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintNumberOptionDetails, 1291959215, 25692, 19945, 150, 95, 111, 198, 187, 196, 124, 171);
RT_INTERFACE!{interface IPrintNumberOptionDetails(IPrintNumberOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintNumberOptionDetails] {
    fn get_MinValue(&self, out: *mut u32) -> HRESULT,
    fn get_MaxValue(&self, out: *mut u32) -> HRESULT
}}
impl IPrintNumberOptionDetails {
    #[inline] pub fn get_min_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MinValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxValue)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintOptionDetails, 956729039, 54914, 18783, 173, 254, 215, 51, 63, 92, 24, 8);
RT_INTERFACE!{interface IPrintOptionDetails(IPrintOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintOptionDetails] {
    fn get_OptionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OptionType(&self, out: *mut PrintOptionType) -> HRESULT,
    fn put_ErrorText(&self, value: HSTRING) -> HRESULT,
    fn get_ErrorText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_State(&self, value: PrintOptionStates) -> HRESULT,
    fn get_State(&self, out: *mut PrintOptionStates) -> HRESULT,
    fn get_Value(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn TrySetValue(&self, value: *mut IInspectable, out: *mut bool) -> HRESULT
}}
impl IPrintOptionDetails {
    #[inline] pub fn get_option_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OptionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_option_type(&self) -> Result<PrintOptionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_OptionType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_error_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ErrorText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_error_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ErrorText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_state(&self, value: PrintOptionStates) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_State)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<PrintOptionStates> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_State)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_set_value(&self, value: &IInspectable) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).TrySetValue)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum PrintOptionStates: u32 {
    None = 0, Enabled = 1, Constrained = 2,
}}
RT_ENUM! { enum PrintOptionType: i32 {
    Unknown = 0, Number = 1, Text = 2, ItemList = 3, Toggle = 4,
}}
DEFINE_IID!(IID_IPrintOrientationOptionDetails, 1187219577, 26336, 19872, 135, 180, 210, 84, 87, 130, 78, 183);
RT_INTERFACE!{interface IPrintOrientationOptionDetails(IPrintOrientationOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintOrientationOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintOrientationOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintOrientationOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintPageRangeOptionDetails, 1511646391, 11240, 19111, 158, 165, 222, 251, 232, 113, 59, 78);
RT_INTERFACE!{interface IPrintPageRangeOptionDetails(IPrintPageRangeOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintPageRangeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageRangeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintQualityOptionDetails, 768633761, 52762, 17638, 132, 249, 58, 146, 234, 30, 48, 68);
RT_INTERFACE!{interface IPrintQualityOptionDetails(IPrintQualityOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintQualityOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintQualityOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintQualityOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintStapleOptionDetails, 3560011197, 39947, 17632, 132, 246, 206, 235, 206, 101, 56, 0);
RT_INTERFACE!{interface IPrintStapleOptionDetails(IPrintStapleOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintStapleOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintStapleOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_WarningText)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_WarningText)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Description)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Description)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintStapleOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintTaskOptionChangedEventArgs, 1696169221, 42478, 17159, 148, 7, 154, 202, 209, 71, 103, 156);
RT_INTERFACE!{interface IPrintTaskOptionChangedEventArgs(IPrintTaskOptionChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionChangedEventArgs] {
    fn get_OptionId(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl IPrintTaskOptionChangedEventArgs {
    #[inline] pub fn get_option_id(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_OptionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptionChangedEventArgs: IPrintTaskOptionChangedEventArgs}
DEFINE_IID!(IID_IPrintTaskOptionDetails, 4117891825, 43166, 17062, 129, 175, 248, 224, 16, 179, 138, 104);
RT_INTERFACE!{interface IPrintTaskOptionDetails(IPrintTaskOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetails] {
    fn get_Options(&self, out: *mut *mut foundation::collections::IMapView<HString, IPrintOptionDetails>) -> HRESULT,
    fn CreateItemListOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut *mut PrintCustomItemListOptionDetails) -> HRESULT,
    fn CreateTextOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut *mut PrintCustomTextOptionDetails) -> HRESULT,
    fn add_OptionChanged(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OptionChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_BeginValidation(&self, eventHandler: *mut foundation::TypedEventHandler<PrintTaskOptionDetails, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BeginValidation(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintTaskOptionDetails {
    #[inline] pub fn get_options(&self) -> Result<Option<ComPtr<foundation::collections::IMapView<HString, IPrintOptionDetails>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Options)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_item_list_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<ComPtr<PrintCustomItemListOptionDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateItemListOption)(self as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_text_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<ComPtr<PrintCustomTextOptionDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTextOption)(self as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_option_changed(&self, eventHandler: &foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_OptionChanged)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_option_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_OptionChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_begin_validation(&self, eventHandler: &foundation::TypedEventHandler<PrintTaskOptionDetails, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_BeginValidation)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_begin_validation(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_BeginValidation)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptionDetails: IPrintTaskOptionDetails}
impl RtActivatable<IPrintTaskOptionDetailsStatic> for PrintTaskOptionDetails {}
impl PrintTaskOptionDetails {
    #[inline] pub fn get_from_print_task_options(printTaskOptions: &super::PrintTaskOptions) -> Result<Option<ComPtr<PrintTaskOptionDetails>>> {
        <Self as RtActivatable<IPrintTaskOptionDetailsStatic>>::get_activation_factory().get_from_print_task_options(printTaskOptions)
    }
}
DEFINE_CLSID!(PrintTaskOptionDetails(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,79,112,116,105,111,110,68,101,116,97,105,108,115,46,80,114,105,110,116,84,97,115,107,79,112,116,105,111,110,68,101,116,97,105,108,115,0]) [CLSID_PrintTaskOptionDetails]);
DEFINE_IID!(IID_IPrintTaskOptionDetails2, 1400048137, 63848, 18066, 161, 119, 192, 116, 89, 113, 134, 219);
RT_INTERFACE!{interface IPrintTaskOptionDetails2(IPrintTaskOptionDetails2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetails2] {
    fn CreateToggleOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut *mut PrintCustomToggleOptionDetails) -> HRESULT
}}
impl IPrintTaskOptionDetails2 {
    #[inline] pub fn create_toggle_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<ComPtr<PrintCustomToggleOptionDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateToggleOption)(self as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionDetailsStatic, 324903315, 2401, 19310, 135, 102, 241, 59, 127, 188, 205, 88);
RT_INTERFACE!{static interface IPrintTaskOptionDetailsStatic(IPrintTaskOptionDetailsStaticVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetailsStatic] {
    fn GetFromPrintTaskOptions(&self, printTaskOptions: *mut super::PrintTaskOptions, out: *mut *mut PrintTaskOptionDetails) -> HRESULT
}}
impl IPrintTaskOptionDetailsStatic {
    #[inline] pub fn get_from_print_task_options(&self, printTaskOptions: &super::PrintTaskOptions) -> Result<Option<ComPtr<PrintTaskOptionDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFromPrintTaskOptions)(self as *const _ as *mut _, printTaskOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTextOptionDetails, 2910184803, 23780, 18108, 153, 24, 171, 159, 173, 20, 76, 91);
RT_INTERFACE!{interface IPrintTextOptionDetails(IPrintTextOptionDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTextOptionDetails] {
    fn get_MaxCharacters(&self, out: *mut u32) -> HRESULT
}}
impl IPrintTextOptionDetails {
    #[inline] pub fn get_max_characters(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxCharacters)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Graphics.Printing.OptionDetails
pub mod printticket { // Windows.Graphics.Printing.PrintTicket
use crate::prelude::*;
DEFINE_IID!(IID_IPrintTicketCapabilities, 2353352843, 48092, 16982, 161, 66, 47, 214, 21, 236, 180, 22);
RT_INTERFACE!{interface IPrintTicketCapabilities(IPrintTicketCapabilitiesVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketCapabilities] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn get_DocumentBindingFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentCollateFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentDuplexFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentHolePunchFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentInputBinFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentNUpFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentStapleFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_JobPasscodeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageBorderlessFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageMediaSizeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageMediaTypeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOrientationFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOutputColorFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOutputQualityFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageResolutionFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn GetFeature(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn GetParameterDefinition(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketParameterDefinition) -> HRESULT
}}
impl IPrintTicketCapabilities {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_binding_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentBindingFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_collate_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentCollateFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_duplex_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentDuplexFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_hole_punch_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentHolePunchFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_input_bin_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentInputBinFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_n_up_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentNUpFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_staple_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentStapleFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_passcode_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JobPasscodeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_borderless_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageBorderlessFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_size_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageMediaSizeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_type_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageMediaTypeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_orientation_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOrientationFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_color_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOutputColorFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_quality_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOutputQualityFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_resolution_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageResolutionFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_feature(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFeature)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parameter_definition(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketParameterDefinition>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetParameterDefinition)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketCapabilities: IPrintTicketCapabilities}
DEFINE_IID!(IID_IPrintTicketFeature, 3881860458, 23029, 16643, 136, 88, 185, 119, 16, 150, 61, 57);
RT_INTERFACE!{interface IPrintTicketFeature(IPrintTicketFeatureVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketFeature] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetOption(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketOption) -> HRESULT,
    fn get_Options(&self, out: *mut *mut foundation::collections::IVectorView<PrintTicketOption>) -> HRESULT,
    fn GetSelectedOption(&self, out: *mut *mut PrintTicketOption) -> HRESULT,
    fn SetSelectedOption(&self, value: *mut PrintTicketOption) -> HRESULT,
    fn get_SelectionType(&self, out: *mut PrintTicketFeatureSelectionType) -> HRESULT
}}
impl IPrintTicketFeature {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_option(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketOption>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOption)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<PrintTicketOption>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Options)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_selected_option(&self) -> Result<Option<ComPtr<PrintTicketOption>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSelectedOption)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_selected_option(&self, value: &PrintTicketOption) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetSelectedOption)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_selection_type(&self) -> Result<PrintTicketFeatureSelectionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_SelectionType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketFeature: IPrintTicketFeature}
RT_ENUM! { enum PrintTicketFeatureSelectionType: i32 {
    PickOne = 0, PickMany = 1,
}}
DEFINE_IID!(IID_IPrintTicketOption, 2961624976, 45927, 20043, 189, 72, 156, 120, 160, 187, 49, 206);
RT_INTERFACE!{interface IPrintTicketOption(IPrintTicketOptionVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketOption] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-data")] fn GetPropertyNode(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-data")] fn GetScoredPropertyNode(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn GetPropertyValue(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketValue) -> HRESULT,
    fn GetScoredPropertyValue(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketValue) -> HRESULT
}}
impl IPrintTicketOption {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_property_node(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertyNode)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_scored_property_node(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetScoredPropertyNode)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_property_value(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetPropertyValue)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scored_property_value(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetScoredPropertyValue)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketOption: IPrintTicketOption}
RT_ENUM! { enum PrintTicketParameterDataType: i32 {
    Integer = 0, NumericString = 1, String = 2,
}}
DEFINE_IID!(IID_IPrintTicketParameterDefinition, 3602560228, 10594, 19457, 183, 243, 154, 146, 148, 235, 131, 53);
RT_INTERFACE!{interface IPrintTicketParameterDefinition(IPrintTicketParameterDefinitionVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketParameterDefinition] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn get_DataType(&self, out: *mut PrintTicketParameterDataType) -> HRESULT,
    fn get_UnitType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RangeMin(&self, out: *mut i32) -> HRESULT,
    fn get_RangeMax(&self, out: *mut i32) -> HRESULT
}}
impl IPrintTicketParameterDefinition {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_type(&self) -> Result<PrintTicketParameterDataType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_DataType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_unit_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_UnitType)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_range_min(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RangeMin)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_range_max(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_RangeMax)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketParameterDefinition: IPrintTicketParameterDefinition}
DEFINE_IID!(IID_IPrintTicketParameterInitializer, 1580414395, 41125, 18609, 157, 92, 7, 17, 109, 220, 89, 122);
RT_INTERFACE!{interface IPrintTicketParameterInitializer(IPrintTicketParameterInitializerVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketParameterInitializer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn put_Value(&self, value: *mut PrintTicketValue) -> HRESULT,
    fn get_Value(&self, out: *mut *mut PrintTicketValue) -> HRESULT
}}
impl IPrintTicketParameterInitializer {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &PrintTicketValue) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<ComPtr<PrintTicketValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketParameterInitializer: IPrintTicketParameterInitializer}
DEFINE_IID!(IID_IPrintTicketValue, 1723009586, 9293, 20002, 169, 139, 187, 60, 241, 242, 221, 145);
RT_INTERFACE!{interface IPrintTicketValue(IPrintTicketValueVtbl): IInspectable(IInspectableVtbl) [IID_IPrintTicketValue] {
    fn get_Type(&self, out: *mut PrintTicketValueType) -> HRESULT,
    fn GetValueAsInteger(&self, out: *mut i32) -> HRESULT,
    fn GetValueAsString(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintTicketValue {
    #[inline] pub fn get_type(&self) -> Result<PrintTicketValueType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_as_integer(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).GetValueAsInteger)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetValueAsString)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketValue: IPrintTicketValue}
RT_ENUM! { enum PrintTicketValueType: i32 {
    Integer = 0, String = 1, Unknown = 2,
}}
DEFINE_IID!(IID_IWorkflowPrintTicket, 1104487045, 13800, 17550, 168, 197, 228, 182, 162, 207, 130, 108);
RT_INTERFACE!{interface IWorkflowPrintTicket(IWorkflowPrintTicketVtbl): IInspectable(IInspectableVtbl) [IID_IWorkflowPrintTicket] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut *mut crate::windows::data::xml::dom::IXmlNode) -> HRESULT,
    fn GetCapabilities(&self, out: *mut *mut PrintTicketCapabilities) -> HRESULT,
    fn get_DocumentBindingFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentCollateFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentDuplexFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentHolePunchFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentInputBinFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentNUpFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_DocumentStapleFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_JobPasscodeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageBorderlessFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageMediaSizeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageMediaTypeFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOrientationFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOutputColorFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageOutputQualityFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn get_PageResolutionFeature(&self, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn GetFeature(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketFeature) -> HRESULT,
    fn NotifyXmlChangedAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn ValidateAsync(&self, out: *mut *mut foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>) -> HRESULT,
    fn GetParameterInitializer(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut *mut PrintTicketParameterInitializer) -> HRESULT,
    fn SetParameterInitializerAsInteger(&self, name: HSTRING, xmlNamespace: HSTRING, integerValue: i32, out: *mut *mut PrintTicketParameterInitializer) -> HRESULT,
    fn SetParameterInitializerAsString(&self, name: HSTRING, xmlNamespace: HSTRING, stringValue: HSTRING, out: *mut *mut PrintTicketParameterInitializer) -> HRESULT,
    fn MergeAndValidateTicket(&self, deltaShemaTicket: *mut WorkflowPrintTicket, out: *mut *mut WorkflowPrintTicket) -> HRESULT
}}
impl IWorkflowPrintTicket {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNamespace)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<ComPtr<crate::windows::data::xml::dom::IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XmlNode)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capabilities(&self) -> Result<Option<ComPtr<PrintTicketCapabilities>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetCapabilities)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_binding_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentBindingFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_collate_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentCollateFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_duplex_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentDuplexFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_hole_punch_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentHolePunchFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_input_bin_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentInputBinFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_n_up_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentNUpFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_staple_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_DocumentStapleFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_passcode_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JobPasscodeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_borderless_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageBorderlessFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_size_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageMediaSizeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_type_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageMediaTypeFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_orientation_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOrientationFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_color_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOutputColorFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_quality_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageOutputQualityFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_resolution_feature(&self) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PageResolutionFeature)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_feature(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketFeature>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetFeature)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn notify_xml_changed_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).NotifyXmlChangedAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn validate_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ValidateAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parameter_initializer(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<ComPtr<PrintTicketParameterInitializer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetParameterInitializer)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_parameter_initializer_as_integer(&self, name: &HStringArg, xmlNamespace: &HStringArg, integerValue: i32) -> Result<Option<ComPtr<PrintTicketParameterInitializer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetParameterInitializerAsInteger)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), integerValue, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_parameter_initializer_as_string(&self, name: &HStringArg, xmlNamespace: &HStringArg, stringValue: &HStringArg) -> Result<Option<ComPtr<PrintTicketParameterInitializer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SetParameterInitializerAsString)(self as *const _ as *mut _, name.get(), xmlNamespace.get(), stringValue.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn merge_and_validate_ticket(&self, deltaShemaTicket: &WorkflowPrintTicket) -> Result<Option<ComPtr<WorkflowPrintTicket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).MergeAndValidateTicket)(self as *const _ as *mut _, deltaShemaTicket as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WorkflowPrintTicket: IWorkflowPrintTicket}
DEFINE_IID!(IID_IWorkflowPrintTicketValidationResult, 181531538, 55931, 18998, 191, 54, 106, 153, 166, 46, 32, 89);
RT_INTERFACE!{interface IWorkflowPrintTicketValidationResult(IWorkflowPrintTicketValidationResultVtbl): IInspectable(IInspectableVtbl) [IID_IWorkflowPrintTicketValidationResult] {
    fn get_Validated(&self, out: *mut bool) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl IWorkflowPrintTicketValidationResult {
    #[inline] pub fn get_validated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Validated)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedError)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WorkflowPrintTicketValidationResult: IWorkflowPrintTicketValidationResult}
} // Windows.Graphics.Printing.PrintTicket
pub mod workflow { // Windows.Graphics.Printing.Workflow
use crate::prelude::*;
DEFINE_IID!(IID_IPrintWorkflowBackgroundSession, 1534661562, 3166, 21130, 116, 88, 134, 164, 108, 189, 220, 69);
RT_INTERFACE!{interface IPrintWorkflowBackgroundSession(IPrintWorkflowBackgroundSessionVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowBackgroundSession] {
    fn add_SetupRequested(&self, setupEventHandler: *mut foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SetupRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Submitted(&self, submittedEventHandler: *mut foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut PrintWorkflowSessionStatus) -> HRESULT,
    fn Start(&self) -> HRESULT
}}
impl IPrintWorkflowBackgroundSession {
    #[inline] pub fn add_setup_requested(&self, setupEventHandler: &foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_SetupRequested)(self as *const _ as *mut _, setupEventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_setup_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_SetupRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_submitted(&self, submittedEventHandler: &foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Submitted)(self as *const _ as *mut _, submittedEventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitted(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Submitted)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<PrintWorkflowSessionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowBackgroundSession: IPrintWorkflowBackgroundSession}
DEFINE_IID!(IID_IPrintWorkflowBackgroundSetupRequestedEventArgs, 1139372866, 5968, 22985, 97, 251, 56, 55, 72, 162, 3, 98);
RT_INTERFACE!{interface IPrintWorkflowBackgroundSetupRequestedEventArgs(IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowBackgroundSetupRequestedEventArgs] {
    fn GetUserPrintTicketAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>) -> HRESULT,
    fn get_Configuration(&self, out: *mut *mut PrintWorkflowConfiguration) -> HRESULT,
    fn SetRequiresUI(&self) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IPrintWorkflowBackgroundSetupRequestedEventArgs {
    #[inline] pub fn get_user_print_ticket_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUserPrintTicketAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<ComPtr<PrintWorkflowConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Configuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_requires_ui(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetRequiresUI)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowBackgroundSetupRequestedEventArgs: IPrintWorkflowBackgroundSetupRequestedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowConfiguration, 3500852461, 64843, 24053, 75, 182, 141, 13, 21, 158, 190, 63);
RT_INTERFACE!{interface IPrintWorkflowConfiguration(IPrintWorkflowConfigurationVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowConfiguration] {
    fn get_SourceAppDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JobTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintWorkflowConfiguration {
    #[inline] pub fn get_source_app_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SourceAppDisplayName)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_JobTitle)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_SessionId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowConfiguration: IPrintWorkflowConfiguration}
DEFINE_IID!(IID_IPrintWorkflowForegroundSession, 3348849616, 63724, 19691, 149, 58, 200, 135, 97, 87, 221, 51);
RT_INTERFACE!{interface IPrintWorkflowForegroundSession(IPrintWorkflowForegroundSessionVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowForegroundSession] {
    fn add_SetupRequested(&self, setupEventHandler: *mut foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SetupRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_XpsDataAvailable(&self, xpsDataAvailableEventHandler: *mut foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_XpsDataAvailable(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut PrintWorkflowSessionStatus) -> HRESULT,
    fn Start(&self) -> HRESULT
}}
impl IPrintWorkflowForegroundSession {
    #[inline] pub fn add_setup_requested(&self, setupEventHandler: &foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_SetupRequested)(self as *const _ as *mut _, setupEventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_setup_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_SetupRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_xps_data_available(&self, xpsDataAvailableEventHandler: &foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_XpsDataAvailable)(self as *const _ as *mut _, xpsDataAvailableEventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_xps_data_available(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_XpsDataAvailable)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<PrintWorkflowSessionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Status)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Start)(self as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowForegroundSession: IPrintWorkflowForegroundSession}
DEFINE_IID!(IID_IPrintWorkflowForegroundSetupRequestedEventArgs, 3152249415, 39963, 19923, 155, 43, 200, 4, 104, 217, 65, 179);
RT_INTERFACE!{interface IPrintWorkflowForegroundSetupRequestedEventArgs(IPrintWorkflowForegroundSetupRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowForegroundSetupRequestedEventArgs] {
    fn GetUserPrintTicketAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>) -> HRESULT,
    fn get_Configuration(&self, out: *mut *mut PrintWorkflowConfiguration) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IPrintWorkflowForegroundSetupRequestedEventArgs {
    #[inline] pub fn get_user_print_ticket_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetUserPrintTicketAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<ComPtr<PrintWorkflowConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Configuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowForegroundSetupRequestedEventArgs: IPrintWorkflowForegroundSetupRequestedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowObjectModelSourceFileContent, 3278670442, 35370, 16794, 179, 195, 32, 144, 230, 191, 171, 47);
RT_INTERFACE!{interface IPrintWorkflowObjectModelSourceFileContent(IPrintWorkflowObjectModelSourceFileContentVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowObjectModelSourceFileContent] {
    
}}
RT_CLASS!{class PrintWorkflowObjectModelSourceFileContent: IPrintWorkflowObjectModelSourceFileContent}
DEFINE_IID!(IID_IPrintWorkflowObjectModelTargetPackage, 2107030644, 39764, 19617, 173, 58, 151, 156, 61, 68, 221, 172);
RT_INTERFACE!{interface IPrintWorkflowObjectModelTargetPackage(IPrintWorkflowObjectModelTargetPackageVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowObjectModelTargetPackage] {
    
}}
RT_CLASS!{class PrintWorkflowObjectModelTargetPackage: IPrintWorkflowObjectModelTargetPackage}
RT_ENUM! { enum PrintWorkflowSessionStatus: i32 {
    Started = 0, Completed = 1, Aborted = 2, Closed = 3,
}}
DEFINE_IID!(IID_IPrintWorkflowSourceContent, 438879809, 52913, 17715, 187, 115, 251, 230, 62, 239, 219, 24);
RT_INTERFACE!{interface IPrintWorkflowSourceContent(IPrintWorkflowSourceContentVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSourceContent] {
    fn GetJobPrintTicketAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>) -> HRESULT,
    fn GetSourceSpoolDataAsStreamContent(&self, out: *mut *mut PrintWorkflowSpoolStreamContent) -> HRESULT,
    fn GetSourceSpoolDataAsXpsObjectModel(&self, out: *mut *mut PrintWorkflowObjectModelSourceFileContent) -> HRESULT
}}
impl IPrintWorkflowSourceContent {
    #[inline] pub fn get_job_print_ticket_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetJobPrintTicketAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_spool_data_as_stream_content(&self) -> Result<Option<ComPtr<PrintWorkflowSpoolStreamContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSourceSpoolDataAsStreamContent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_spool_data_as_xps_object_model(&self) -> Result<Option<ComPtr<PrintWorkflowObjectModelSourceFileContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetSourceSpoolDataAsXpsObjectModel)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSourceContent: IPrintWorkflowSourceContent}
DEFINE_IID!(IID_IPrintWorkflowSpoolStreamContent, 1927634638, 58374, 19316, 132, 225, 63, 243, 253, 205, 175, 112);
RT_INTERFACE!{interface IPrintWorkflowSpoolStreamContent(IPrintWorkflowSpoolStreamContentVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSpoolStreamContent] {
    #[cfg(feature="windows-storage")] fn GetInputStream(&self, out: *mut *mut crate::windows::storage::streams::IInputStream) -> HRESULT
}}
impl IPrintWorkflowSpoolStreamContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream(&self) -> Result<Option<ComPtr<crate::windows::storage::streams::IInputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetInputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSpoolStreamContent: IPrintWorkflowSpoolStreamContent}
DEFINE_IID!(IID_IPrintWorkflowStreamTarget, 2990258820, 34149, 18571, 152, 57, 28, 158, 124, 122, 169, 22);
RT_INTERFACE!{interface IPrintWorkflowStreamTarget(IPrintWorkflowStreamTargetVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowStreamTarget] {
    #[cfg(feature="windows-storage")] fn GetOutputStream(&self, out: *mut *mut crate::windows::storage::streams::IOutputStream) -> HRESULT
}}
impl IPrintWorkflowStreamTarget {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<ComPtr<crate::windows::storage::streams::IOutputStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetOutputStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowStreamTarget: IPrintWorkflowStreamTarget}
DEFINE_IID!(IID_IPrintWorkflowSubmittedEventArgs, 987564609, 14228, 21865, 92, 135, 64, 232, 255, 114, 15, 131);
RT_INTERFACE!{interface IPrintWorkflowSubmittedEventArgs(IPrintWorkflowSubmittedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSubmittedEventArgs] {
    fn get_Operation(&self, out: *mut *mut PrintWorkflowSubmittedOperation) -> HRESULT,
    fn GetTarget(&self, jobPrintTicket: *mut super::printticket::WorkflowPrintTicket, out: *mut *mut PrintWorkflowTarget) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IPrintWorkflowSubmittedEventArgs {
    #[inline] pub fn get_operation(&self) -> Result<Option<ComPtr<PrintWorkflowSubmittedOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Operation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_target(&self, jobPrintTicket: &super::printticket::WorkflowPrintTicket) -> Result<Option<ComPtr<PrintWorkflowTarget>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTarget)(self as *const _ as *mut _, jobPrintTicket as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSubmittedEventArgs: IPrintWorkflowSubmittedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowSubmittedOperation, 776888854, 15329, 24335, 92, 129, 165, 162, 189, 78, 171, 14);
RT_INTERFACE!{interface IPrintWorkflowSubmittedOperation(IPrintWorkflowSubmittedOperationVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSubmittedOperation] {
    fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> HRESULT,
    fn get_Configuration(&self, out: *mut *mut PrintWorkflowConfiguration) -> HRESULT,
    fn get_XpsContent(&self, out: *mut *mut PrintWorkflowSourceContent) -> HRESULT
}}
impl IPrintWorkflowSubmittedOperation {
    #[inline] pub fn complete(&self, status: PrintWorkflowSubmittedStatus) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Complete)(self as *const _ as *mut _, status);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<ComPtr<PrintWorkflowConfiguration>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Configuration)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xps_content(&self) -> Result<Option<ComPtr<PrintWorkflowSourceContent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_XpsContent)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSubmittedOperation: IPrintWorkflowSubmittedOperation}
RT_ENUM! { enum PrintWorkflowSubmittedStatus: i32 {
    Succeeded = 0, Canceled = 1, Failed = 2,
}}
DEFINE_IID!(IID_IPrintWorkflowTarget, 702162796, 2675, 23277, 79, 61, 151, 13, 50, 81, 240, 87);
RT_INTERFACE!{interface IPrintWorkflowTarget(IPrintWorkflowTargetVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowTarget] {
    fn get_TargetAsStream(&self, out: *mut *mut PrintWorkflowStreamTarget) -> HRESULT,
    fn get_TargetAsXpsObjectModelPackage(&self, out: *mut *mut PrintWorkflowObjectModelTargetPackage) -> HRESULT
}}
impl IPrintWorkflowTarget {
    #[inline] pub fn get_target_as_stream(&self) -> Result<Option<ComPtr<PrintWorkflowStreamTarget>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetAsStream)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_target_as_xps_object_model_package(&self) -> Result<Option<ComPtr<PrintWorkflowObjectModelTargetPackage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TargetAsXpsObjectModelPackage)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowTarget: IPrintWorkflowTarget}
DEFINE_IID!(IID_IPrintWorkflowTriggerDetails, 1463408744, 40326, 16466, 176, 203, 243, 16, 190, 205, 89, 187);
RT_INTERFACE!{interface IPrintWorkflowTriggerDetails(IPrintWorkflowTriggerDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowTriggerDetails] {
    fn get_PrintWorkflowSession(&self, out: *mut *mut PrintWorkflowBackgroundSession) -> HRESULT
}}
impl IPrintWorkflowTriggerDetails {
    #[inline] pub fn get_print_workflow_session(&self) -> Result<Option<ComPtr<PrintWorkflowBackgroundSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrintWorkflowSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowTriggerDetails: IPrintWorkflowTriggerDetails}
DEFINE_IID!(IID_IPrintWorkflowUIActivatedEventArgs, 3163194445, 2539, 22342, 114, 166, 141, 200, 181, 237, 190, 155);
RT_INTERFACE!{interface IPrintWorkflowUIActivatedEventArgs(IPrintWorkflowUIActivatedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowUIActivatedEventArgs] {
    fn get_PrintWorkflowSession(&self, out: *mut *mut PrintWorkflowForegroundSession) -> HRESULT
}}
impl IPrintWorkflowUIActivatedEventArgs {
    #[inline] pub fn get_print_workflow_session(&self) -> Result<Option<ComPtr<PrintWorkflowForegroundSession>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrintWorkflowSession)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowUIActivatedEventArgs: IPrintWorkflowUIActivatedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowXpsDataAvailableEventArgs, 1293009713, 21713, 17230, 190, 14, 130, 197, 250, 88, 229, 178);
RT_INTERFACE!{interface IPrintWorkflowXpsDataAvailableEventArgs(IPrintWorkflowXpsDataAvailableEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowXpsDataAvailableEventArgs] {
    fn get_Operation(&self, out: *mut *mut PrintWorkflowSubmittedOperation) -> HRESULT,
    fn GetDeferral(&self, out: *mut *mut foundation::Deferral) -> HRESULT
}}
impl IPrintWorkflowXpsDataAvailableEventArgs {
    #[inline] pub fn get_operation(&self) -> Result<Option<ComPtr<PrintWorkflowSubmittedOperation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Operation)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<ComPtr<foundation::Deferral>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetDeferral)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowXpsDataAvailableEventArgs: IPrintWorkflowXpsDataAvailableEventArgs}
} // Windows.Graphics.Printing.Workflow
} // Windows.Graphics.Printing
pub mod printing3d { // Windows.Graphics.Printing3D
use crate::prelude::*;
DEFINE_IID!(IID_IPrint3DManager, 1294977802, 29542, 18801, 139, 213, 23, 196, 227, 232, 198, 192);
RT_INTERFACE!{interface IPrint3DManager(IPrint3DManagerVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DManager] {
    fn add_TaskRequested(&self, eventHandler: *mut foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TaskRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrint3DManager {
    #[inline] pub fn add_task_requested(&self, eventHandler: &foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_TaskRequested)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_task_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_TaskRequested)(self as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DManager: IPrint3DManager}
impl RtActivatable<IPrint3DManagerStatics> for Print3DManager {}
impl Print3DManager {
    #[inline] pub fn get_for_current_view() -> Result<Option<ComPtr<Print3DManager>>> {
        <Self as RtActivatable<IPrint3DManagerStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn show_print_ui_async() -> Result<ComPtr<foundation::IAsyncOperation<bool>>> {
        <Self as RtActivatable<IPrint3DManagerStatics>>::get_activation_factory().show_print_ui_async()
    }
}
DEFINE_CLSID!(Print3DManager(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,51,68,77,97,110,97,103,101,114,0]) [CLSID_Print3DManager]);
DEFINE_IID!(IID_IPrint3DManagerStatics, 250727166, 43437, 19464, 169, 23, 29, 31, 134, 62, 171, 203);
RT_INTERFACE!{static interface IPrint3DManagerStatics(IPrint3DManagerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DManagerStatics] {
    fn GetForCurrentView(&self, out: *mut *mut Print3DManager) -> HRESULT,
    fn ShowPrintUIAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT
}}
impl IPrint3DManagerStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<ComPtr<Print3DManager>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetForCurrentView)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_print_ui_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).ShowPrintUIAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrint3DTask, 2363740288, 8472, 19496, 128, 222, 244, 38, 215, 1, 145, 174);
RT_INTERFACE!{interface IPrint3DTask(IPrint3DTaskVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTask] {
    fn get_Source(&self, out: *mut *mut Printing3D3MFPackage) -> HRESULT,
    fn add_Submitting(&self, eventHandler: *mut foundation::TypedEventHandler<Print3DTask, IInspectable>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitting(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Completed(&self, eventHandler: *mut foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Completed(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_SourceChanged(&self, eventHandler: *mut foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SourceChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrint3DTask {
    #[inline] pub fn get_source(&self) -> Result<Option<ComPtr<Printing3D3MFPackage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_submitting(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Submitting)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitting(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Submitting)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_completed(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_Completed)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_completed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_Completed)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_source_changed(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).add_SourceChanged)(self as *const _ as *mut _, eventHandler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_source_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).remove_SourceChanged)(self as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTask: IPrint3DTask}
DEFINE_IID!(IID_IPrint3DTaskCompletedEventArgs, 3424195759, 9748, 20253, 172, 204, 214, 252, 79, 218, 84, 85);
RT_INTERFACE!{interface IPrint3DTaskCompletedEventArgs(IPrint3DTaskCompletedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskCompletedEventArgs] {
    fn get_Completion(&self, out: *mut Print3DTaskCompletion) -> HRESULT,
    fn get_ExtendedStatus(&self, out: *mut Print3DTaskDetail) -> HRESULT
}}
impl IPrint3DTaskCompletedEventArgs {
    #[inline] pub fn get_completion(&self) -> Result<Print3DTaskCompletion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Completion)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_status(&self) -> Result<Print3DTaskDetail> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_ExtendedStatus)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskCompletedEventArgs: IPrint3DTaskCompletedEventArgs}
RT_ENUM! { enum Print3DTaskCompletion: i32 {
    Abandoned = 0, Canceled = 1, Failed = 2, Slicing = 3, Submitted = 4,
}}
RT_ENUM! { enum Print3DTaskDetail: i32 {
    Unknown = 0, ModelExceedsPrintBed = 1, UploadFailed = 2, InvalidMaterialSelection = 3, InvalidModel = 4, ModelNotManifold = 5, InvalidPrintTicket = 6,
}}
DEFINE_IID!(IID_IPrint3DTaskRequest, 630572143, 8773, 19546, 135, 49, 13, 96, 77, 198, 188, 60);
RT_INTERFACE!{interface IPrint3DTaskRequest(IPrint3DTaskRequestVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskRequest] {
    fn CreateTask(&self, title: HSTRING, printerId: HSTRING, handler: *mut Print3DTaskSourceRequestedHandler, out: *mut *mut Print3DTask) -> HRESULT
}}
impl IPrint3DTaskRequest {
    #[inline] pub fn create_task(&self, title: &HStringArg, printerId: &HStringArg, handler: &Print3DTaskSourceRequestedHandler) -> Result<Option<ComPtr<Print3DTask>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).CreateTask)(self as *const _ as *mut _, title.get(), printerId.get(), handler as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskRequest: IPrint3DTaskRequest}
DEFINE_IID!(IID_IPrint3DTaskRequestedEventArgs, 353154943, 6341, 16599, 159, 64, 250, 179, 9, 110, 5, 169);
RT_INTERFACE!{interface IPrint3DTaskRequestedEventArgs(IPrint3DTaskRequestedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskRequestedEventArgs] {
    fn get_Request(&self, out: *mut *mut Print3DTaskRequest) -> HRESULT
}}
impl IPrint3DTaskRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<ComPtr<Print3DTaskRequest>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Request)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskRequestedEventArgs: IPrint3DTaskRequestedEventArgs}
DEFINE_IID!(IID_IPrint3DTaskSourceChangedEventArgs, 1540175023, 9449, 19472, 141, 7, 20, 195, 70, 186, 63, 207);
RT_INTERFACE!{interface IPrint3DTaskSourceChangedEventArgs(IPrint3DTaskSourceChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskSourceChangedEventArgs] {
    fn get_Source(&self, out: *mut *mut Printing3D3MFPackage) -> HRESULT
}}
impl IPrint3DTaskSourceChangedEventArgs {
    #[inline] pub fn get_source(&self) -> Result<Option<ComPtr<Printing3D3MFPackage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Source)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskSourceChangedEventArgs: IPrint3DTaskSourceChangedEventArgs}
DEFINE_IID!(IID_IPrint3DTaskSourceRequestedArgs, 3346832058, 9391, 16973, 163, 191, 146, 37, 12, 53, 86, 2);
RT_INTERFACE!{interface IPrint3DTaskSourceRequestedArgs(IPrint3DTaskSourceRequestedArgsVtbl): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskSourceRequestedArgs] {
    fn SetSource(&self, source: *mut Printing3D3MFPackage) -> HRESULT
}}
impl IPrint3DTaskSourceRequestedArgs {
    #[inline] pub fn set_source(&self, source: &Printing3D3MFPackage) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).SetSource)(self as *const _ as *mut _, source as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskSourceRequestedArgs: IPrint3DTaskSourceRequestedArgs}
DEFINE_IID!(IID_Print3DTaskSourceRequestedHandler, 3910622832, 51479, 18142, 187, 81, 217, 169, 77, 179, 113, 31);
RT_DELEGATE!{delegate Print3DTaskSourceRequestedHandler(Print3DTaskSourceRequestedHandlerVtbl, Print3DTaskSourceRequestedHandlerImpl) [IID_Print3DTaskSourceRequestedHandler] {
    fn Invoke(&self, args: *mut Print3DTaskSourceRequestedArgs) -> HRESULT
}}
impl Print3DTaskSourceRequestedHandler {
    #[inline] pub fn invoke(&self, args: &Print3DTaskSourceRequestedArgs) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).Invoke)(self as *const _ as *mut _, args as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3D3MFPackage, 4132296136, 10935, 17833, 161, 183, 38, 126, 148, 141, 91, 24);
RT_INTERFACE!{interface IPrinting3D3MFPackage(IPrinting3D3MFPackageVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackage] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn SaveAsync(&self, out: *mut *mut foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_PrintTicket(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_PrintTicket(&self, value: *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ModelPart(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_ModelPart(&self, value: *mut super::super::storage::streams::IRandomAccessStream) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut *mut Printing3DTextureResource) -> HRESULT,
    fn put_Thumbnail(&self, value: *mut Printing3DTextureResource) -> HRESULT,
    fn get_Textures(&self, out: *mut *mut foundation::collections::IVector<Printing3DTextureResource>) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn LoadModelFromPackageAsync(&self, value: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<Printing3DModel>) -> HRESULT,
    fn SaveModelToPackageAsync(&self, value: *mut Printing3DModel, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl IPrinting3D3MFPackage {
    #[cfg(feature="windows-storage")] #[inline] pub fn save_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_print_ticket(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PrintTicket)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_print_ticket(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PrintTicket)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_model_part(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStream>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ModelPart)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_model_part(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_ModelPart)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail(&self) -> Result<Option<ComPtr<Printing3DTextureResource>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thumbnail)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbnail(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Thumbnail)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_textures(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DTextureResource>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Textures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_package_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<Printing3DModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadModelFromPackageAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_model_to_package_async(&self, value: &Printing3DModel) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).SaveModelToPackageAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3D3MFPackage: IPrinting3D3MFPackage}
impl RtActivatable<IPrinting3D3MFPackageStatics> for Printing3D3MFPackage {}
impl RtActivatable<IActivationFactory> for Printing3D3MFPackage {}
impl Printing3D3MFPackage {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_async(value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<Printing3D3MFPackage>>> {
        <Self as RtActivatable<IPrinting3D3MFPackageStatics>>::get_activation_factory().load_async(value)
    }
}
DEFINE_CLSID!(Printing3D3MFPackage(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,51,77,70,80,97,99,107,97,103,101,0]) [CLSID_Printing3D3MFPackage]);
DEFINE_IID!(IID_IPrinting3D3MFPackage2, 2522643140, 37835, 17456, 146, 184, 120, 156, 212, 84, 248, 131);
RT_INTERFACE!{interface IPrinting3D3MFPackage2(IPrinting3D3MFPackage2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackage2] {
    fn get_Compression(&self, out: *mut Printing3DPackageCompression) -> HRESULT,
    fn put_Compression(&self, value: Printing3DPackageCompression) -> HRESULT
}}
impl IPrinting3D3MFPackage2 {
    #[inline] pub fn get_compression(&self) -> Result<Printing3DPackageCompression> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Compression)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_compression(&self, value: Printing3DPackageCompression) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Compression)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3D3MFPackageStatics, 1884871087, 31386, 18311, 184, 23, 246, 244, 89, 33, 72, 35);
RT_INTERFACE!{static interface IPrinting3D3MFPackageStatics(IPrinting3D3MFPackageStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackageStatics] {
    #[cfg(feature="windows-storage")] fn LoadAsync(&self, value: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<Printing3D3MFPackage>) -> HRESULT
}}
impl IPrinting3D3MFPackageStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<ComPtr<foundation::IAsyncOperation<Printing3D3MFPackage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).LoadAsync)(self as *const _ as *mut _, value as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DBaseMaterial, 3505448771, 50444, 19403, 157, 4, 252, 22, 173, 206, 162, 201);
RT_INTERFACE!{interface IPrinting3DBaseMaterial(IPrinting3DBaseMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterial] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Color(&self, out: *mut *mut Printing3DColorMaterial) -> HRESULT,
    fn put_Color(&self, value: *mut Printing3DColorMaterial) -> HRESULT
}}
impl IPrinting3DBaseMaterial {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_color(&self) -> Result<Option<ComPtr<Printing3DColorMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Color)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_color(&self, value: &Printing3DColorMaterial) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Color)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DBaseMaterial: IPrinting3DBaseMaterial}
impl RtActivatable<IPrinting3DBaseMaterialStatics> for Printing3DBaseMaterial {}
impl RtActivatable<IActivationFactory> for Printing3DBaseMaterial {}
impl Printing3DBaseMaterial {
    #[inline] pub fn get_abs() -> Result<HString> {
        <Self as RtActivatable<IPrinting3DBaseMaterialStatics>>::get_activation_factory().get_abs()
    }
    #[inline] pub fn get_pla() -> Result<HString> {
        <Self as RtActivatable<IPrinting3DBaseMaterialStatics>>::get_activation_factory().get_pla()
    }
}
DEFINE_CLSID!(Printing3DBaseMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,66,97,115,101,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DBaseMaterial]);
DEFINE_IID!(IID_IPrinting3DBaseMaterialGroup, 2498785464, 9493, 19085, 161, 240, 208, 252, 19, 208, 96, 33);
RT_INTERFACE!{interface IPrinting3DBaseMaterialGroup(IPrinting3DBaseMaterialGroupVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialGroup] {
    fn get_Bases(&self, out: *mut *mut foundation::collections::IVector<Printing3DBaseMaterial>) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DBaseMaterialGroup {
    #[inline] pub fn get_bases(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DBaseMaterial>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Bases)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaterialGroupId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DBaseMaterialGroup: IPrinting3DBaseMaterialGroup}
impl RtActivatable<IPrinting3DBaseMaterialGroupFactory> for Printing3DBaseMaterialGroup {}
impl Printing3DBaseMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<ComPtr<Printing3DBaseMaterialGroup>> {
        <Self as RtActivatable<IPrinting3DBaseMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DBaseMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,66,97,115,101,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DBaseMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DBaseMaterialGroupFactory, 1544898268, 34455, 16787, 151, 107, 132, 187, 65, 22, 229, 191);
RT_INTERFACE!{static interface IPrinting3DBaseMaterialGroupFactory(IPrinting3DBaseMaterialGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut *mut Printing3DBaseMaterialGroup) -> HRESULT
}}
impl IPrinting3DBaseMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<ComPtr<Printing3DBaseMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DBaseMaterialStatics, 2170177468, 14154, 18285, 190, 146, 62, 207, 209, 203, 151, 118);
RT_INTERFACE!{static interface IPrinting3DBaseMaterialStatics(IPrinting3DBaseMaterialStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialStatics] {
    fn get_Abs(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pla(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrinting3DBaseMaterialStatics {
    #[inline] pub fn get_abs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Abs)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pla(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Pla)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct Printing3DBufferDescription {
    Format: Printing3DBufferFormat, Stride: u32,
}}
RT_ENUM! { enum Printing3DBufferFormat: i32 {
    Unknown = 0, R32G32B32A32Float = 2, R32G32B32A32UInt = 3, R32G32B32Float = 6, R32G32B32UInt = 7, Printing3DDouble = 500, Printing3DUInt = 501,
}}
DEFINE_IID!(IID_IPrinting3DColorMaterial, 3783891240, 31975, 17029, 163, 93, 241, 69, 201, 81, 12, 123);
RT_INTERFACE!{interface IPrinting3DColorMaterial(IPrinting3DColorMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterial] {
    fn get_Value(&self, out: *mut u32) -> HRESULT,
    fn put_Value(&self, value: u32) -> HRESULT
}}
impl IPrinting3DColorMaterial {
    #[inline] pub fn get_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Value)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Value)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DColorMaterial: IPrinting3DColorMaterial}
impl RtActivatable<IActivationFactory> for Printing3DColorMaterial {}
DEFINE_CLSID!(Printing3DColorMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,108,111,114,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DColorMaterial]);
DEFINE_IID!(IID_IPrinting3DColorMaterial2, 4205897810, 2799, 17641, 157, 221, 54, 238, 234, 90, 205, 68);
RT_INTERFACE!{interface IPrinting3DColorMaterial2(IPrinting3DColorMaterial2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterial2] {
    #[cfg(feature="windows-ui")] fn get_Color(&self, out: *mut super::super::ui::Color) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_Color(&self, value: super::super::ui::Color) -> HRESULT
}}
impl IPrinting3DColorMaterial2 {
    #[cfg(feature="windows-ui")] #[inline] pub fn get_color(&self) -> Result<super::super::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Color)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_color(&self, value: super::super::ui::Color) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Color)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DColorMaterialGroup, 1731536, 43743, 16934, 175, 233, 243, 105, 160, 180, 80, 4);
RT_INTERFACE!{interface IPrinting3DColorMaterialGroup(IPrinting3DColorMaterialGroupVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterialGroup] {
    fn get_Colors(&self, out: *mut *mut foundation::collections::IVector<Printing3DColorMaterial>) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DColorMaterialGroup {
    #[inline] pub fn get_colors(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DColorMaterial>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Colors)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaterialGroupId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DColorMaterialGroup: IPrinting3DColorMaterialGroup}
impl RtActivatable<IPrinting3DColorMaterialGroupFactory> for Printing3DColorMaterialGroup {}
impl Printing3DColorMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<ComPtr<Printing3DColorMaterialGroup>> {
        <Self as RtActivatable<IPrinting3DColorMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DColorMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,108,111,114,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DColorMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DColorMaterialGroupFactory, 1909689709, 45546, 19035, 188, 84, 25, 198, 95, 61, 240, 68);
RT_INTERFACE!{static interface IPrinting3DColorMaterialGroupFactory(IPrinting3DColorMaterialGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut *mut Printing3DColorMaterialGroup) -> HRESULT
}}
impl IPrinting3DColorMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<ComPtr<Printing3DColorMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DComponent, 2116581445, 49023, 19675, 162, 127, 48, 160, 20, 55, 254, 222);
RT_INTERFACE!{interface IPrinting3DComponent(IPrinting3DComponentVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DComponent] {
    fn get_Mesh(&self, out: *mut *mut Printing3DMesh) -> HRESULT,
    fn put_Mesh(&self, value: *mut Printing3DMesh) -> HRESULT,
    fn get_Components(&self, out: *mut *mut foundation::collections::IVector<Printing3DComponentWithMatrix>) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut *mut Printing3DTextureResource) -> HRESULT,
    fn put_Thumbnail(&self, value: *mut Printing3DTextureResource) -> HRESULT,
    fn get_Type(&self, out: *mut Printing3DObjectType) -> HRESULT,
    fn put_Type(&self, value: Printing3DObjectType) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_PartNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PartNumber(&self, value: HSTRING) -> HRESULT
}}
impl IPrinting3DComponent {
    #[inline] pub fn get_mesh(&self) -> Result<Option<ComPtr<Printing3DMesh>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Mesh)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_mesh(&self, value: &Printing3DMesh) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Mesh)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_components(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DComponentWithMatrix>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Components)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail(&self) -> Result<Option<ComPtr<Printing3DTextureResource>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Thumbnail)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbnail(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Thumbnail)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<Printing3DObjectType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Type)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_type(&self, value: Printing3DObjectType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Type)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Name)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Name)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_part_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_PartNumber)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_part_number(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_PartNumber)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DComponent: IPrinting3DComponent}
impl RtActivatable<IActivationFactory> for Printing3DComponent {}
DEFINE_CLSID!(Printing3DComponent(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,110,101,110,116,0]) [CLSID_Printing3DComponent]);
DEFINE_IID!(IID_IPrinting3DComponentWithMatrix, 846852917, 3824, 17771, 154, 33, 73, 190, 190, 139, 81, 194);
RT_INTERFACE!{interface IPrinting3DComponentWithMatrix(IPrinting3DComponentWithMatrixVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DComponentWithMatrix] {
    fn get_Component(&self, out: *mut *mut Printing3DComponent) -> HRESULT,
    fn put_Component(&self, value: *mut Printing3DComponent) -> HRESULT,
    fn get_Matrix(&self, out: *mut foundation::numerics::Matrix4x4) -> HRESULT,
    fn put_Matrix(&self, value: foundation::numerics::Matrix4x4) -> HRESULT
}}
impl IPrinting3DComponentWithMatrix {
    #[inline] pub fn get_component(&self) -> Result<Option<ComPtr<Printing3DComponent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Component)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_component(&self, value: &Printing3DComponent) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Component)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_matrix(&self) -> Result<foundation::numerics::Matrix4x4> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Matrix)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_matrix(&self, value: foundation::numerics::Matrix4x4) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Matrix)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DComponentWithMatrix: IPrinting3DComponentWithMatrix}
impl RtActivatable<IActivationFactory> for Printing3DComponentWithMatrix {}
DEFINE_CLSID!(Printing3DComponentWithMatrix(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,110,101,110,116,87,105,116,104,77,97,116,114,105,120,0]) [CLSID_Printing3DComponentWithMatrix]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterial, 1176647901, 22062, 20332, 136, 45, 244, 216, 65, 253, 99, 199);
RT_INTERFACE!{interface IPrinting3DCompositeMaterial(IPrinting3DCompositeMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterial] {
    fn get_Values(&self, out: *mut *mut foundation::collections::IVector<f64>) -> HRESULT
}}
impl IPrinting3DCompositeMaterial {
    #[inline] pub fn get_values(&self) -> Result<Option<ComPtr<foundation::collections::IVector<f64>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Values)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DCompositeMaterial: IPrinting3DCompositeMaterial}
impl RtActivatable<IActivationFactory> for Printing3DCompositeMaterial {}
DEFINE_CLSID!(Printing3DCompositeMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,115,105,116,101,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DCompositeMaterial]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroup, 2375314011, 16625, 18797, 165, 251, 52, 10, 90, 103, 142, 48);
RT_INTERFACE!{interface IPrinting3DCompositeMaterialGroup(IPrinting3DCompositeMaterialGroupVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroup] {
    fn get_Composites(&self, out: *mut *mut foundation::collections::IVector<Printing3DCompositeMaterial>) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT,
    fn get_MaterialIndices(&self, out: *mut *mut foundation::collections::IVector<u32>) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroup {
    #[inline] pub fn get_composites(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DCompositeMaterial>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Composites)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaterialGroupId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_material_indices(&self) -> Result<Option<ComPtr<foundation::collections::IVector<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaterialIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DCompositeMaterialGroup: IPrinting3DCompositeMaterialGroup}
impl RtActivatable<IPrinting3DCompositeMaterialGroupFactory> for Printing3DCompositeMaterialGroup {}
impl Printing3DCompositeMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<ComPtr<Printing3DCompositeMaterialGroup>> {
        <Self as RtActivatable<IPrinting3DCompositeMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DCompositeMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,115,105,116,101,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DCompositeMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroup2, 115895650, 32059, 16865, 148, 76, 186, 253, 228, 85, 84, 131);
RT_INTERFACE!{interface IPrinting3DCompositeMaterialGroup2(IPrinting3DCompositeMaterialGroup2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroup2] {
    fn get_BaseMaterialGroup(&self, out: *mut *mut Printing3DBaseMaterialGroup) -> HRESULT,
    fn put_BaseMaterialGroup(&self, value: *mut Printing3DBaseMaterialGroup) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroup2 {
    #[inline] pub fn get_base_material_group(&self) -> Result<Option<ComPtr<Printing3DBaseMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseMaterialGroup)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_base_material_group(&self, value: &Printing3DBaseMaterialGroup) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_BaseMaterialGroup)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroupFactory, 3499019539, 37631, 17322, 166, 39, 141, 67, 194, 44, 129, 126);
RT_INTERFACE!{static interface IPrinting3DCompositeMaterialGroupFactory(IPrinting3DCompositeMaterialGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut *mut Printing3DCompositeMaterialGroup) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<ComPtr<Printing3DCompositeMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DFaceReductionOptions, 3154039703, 11636, 18167, 190, 133, 153, 166, 123, 187, 102, 41);
RT_INTERFACE!{interface IPrinting3DFaceReductionOptions(IPrinting3DFaceReductionOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DFaceReductionOptions] {
    fn get_MaxReductionArea(&self, out: *mut f64) -> HRESULT,
    fn put_MaxReductionArea(&self, value: f64) -> HRESULT,
    fn get_TargetTriangleCount(&self, out: *mut u32) -> HRESULT,
    fn put_TargetTriangleCount(&self, value: u32) -> HRESULT,
    fn get_MaxEdgeLength(&self, out: *mut f64) -> HRESULT,
    fn put_MaxEdgeLength(&self, value: f64) -> HRESULT
}}
impl IPrinting3DFaceReductionOptions {
    #[inline] pub fn get_max_reduction_area(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxReductionArea)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_reduction_area(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxReductionArea)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_target_triangle_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TargetTriangleCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_target_triangle_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TargetTriangleCount)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_edge_length(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaxEdgeLength)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_edge_length(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_MaxEdgeLength)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DFaceReductionOptions: IPrinting3DFaceReductionOptions}
impl RtActivatable<IActivationFactory> for Printing3DFaceReductionOptions {}
DEFINE_CLSID!(Printing3DFaceReductionOptions(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,70,97,99,101,82,101,100,117,99,116,105,111,110,79,112,116,105,111,110,115,0]) [CLSID_Printing3DFaceReductionOptions]);
DEFINE_IID!(IID_IPrinting3DMaterial, 932033110, 60770, 18770, 184, 91, 3, 86, 125, 124, 70, 94);
RT_INTERFACE!{interface IPrinting3DMaterial(IPrinting3DMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMaterial] {
    fn get_BaseGroups(&self, out: *mut *mut foundation::collections::IVector<Printing3DBaseMaterialGroup>) -> HRESULT,
    fn get_ColorGroups(&self, out: *mut *mut foundation::collections::IVector<Printing3DColorMaterialGroup>) -> HRESULT,
    fn get_Texture2CoordGroups(&self, out: *mut *mut foundation::collections::IVector<Printing3DTexture2CoordMaterialGroup>) -> HRESULT,
    fn get_CompositeGroups(&self, out: *mut *mut foundation::collections::IVector<Printing3DCompositeMaterialGroup>) -> HRESULT,
    fn get_MultiplePropertyGroups(&self, out: *mut *mut foundation::collections::IVector<Printing3DMultiplePropertyMaterialGroup>) -> HRESULT
}}
impl IPrinting3DMaterial {
    #[inline] pub fn get_base_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DBaseMaterialGroup>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BaseGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_color_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DColorMaterialGroup>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ColorGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_texture2_coord_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DTexture2CoordMaterialGroup>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Texture2CoordGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_composite_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DCompositeMaterialGroup>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_CompositeGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_multiple_property_groups(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DMultiplePropertyMaterialGroup>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MultiplePropertyGroups)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMaterial: IPrinting3DMaterial}
impl RtActivatable<IActivationFactory> for Printing3DMaterial {}
DEFINE_CLSID!(Printing3DMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DMaterial]);
DEFINE_IID!(IID_IPrinting3DMesh, 422482140, 552, 11777, 188, 32, 197, 41, 12, 191, 50, 196);
RT_INTERFACE!{interface IPrinting3DMesh(IPrinting3DMeshVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMesh] {
    fn get_VertexCount(&self, out: *mut u32) -> HRESULT,
    fn put_VertexCount(&self, value: u32) -> HRESULT,
    fn get_IndexCount(&self, out: *mut u32) -> HRESULT,
    fn put_IndexCount(&self, value: u32) -> HRESULT,
    fn get_VertexPositionsDescription(&self, out: *mut Printing3DBufferDescription) -> HRESULT,
    fn put_VertexPositionsDescription(&self, value: Printing3DBufferDescription) -> HRESULT,
    fn get_VertexNormalsDescription(&self, out: *mut Printing3DBufferDescription) -> HRESULT,
    fn put_VertexNormalsDescription(&self, value: Printing3DBufferDescription) -> HRESULT,
    fn get_TriangleIndicesDescription(&self, out: *mut Printing3DBufferDescription) -> HRESULT,
    fn put_TriangleIndicesDescription(&self, value: Printing3DBufferDescription) -> HRESULT,
    fn get_TriangleMaterialIndicesDescription(&self, out: *mut Printing3DBufferDescription) -> HRESULT,
    fn put_TriangleMaterialIndicesDescription(&self, value: Printing3DBufferDescription) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetVertexPositions(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn CreateVertexPositions(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetVertexNormals(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn CreateVertexNormals(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy16(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetTriangleIndices(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn CreateTriangleIndices(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy18(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetTriangleMaterialIndices(&self, out: *mut *mut super::super::storage::streams::IBuffer) -> HRESULT,
    fn CreateTriangleMaterialIndices(&self, value: u32) -> HRESULT,
    fn get_BufferDescriptionSet(&self, out: *mut *mut foundation::collections::IPropertySet) -> HRESULT,
    fn get_BufferSet(&self, out: *mut *mut foundation::collections::IPropertySet) -> HRESULT,
    fn VerifyAsync(&self, value: Printing3DMeshVerificationMode, out: *mut *mut foundation::IAsyncOperation<Printing3DMeshVerificationResult>) -> HRESULT
}}
impl IPrinting3DMesh {
    #[inline] pub fn get_vertex_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_VertexCount)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_index_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IndexCount)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_index_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_IndexCount)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_positions_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexPositionsDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_positions_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_VertexPositionsDescription)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_normals_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_VertexNormalsDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_normals_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_VertexNormalsDescription)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_indices_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TriangleIndicesDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_triangle_indices_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TriangleIndicesDescription)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_material_indices_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TriangleMaterialIndicesDescription)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_triangle_material_indices_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TriangleMaterialIndicesDescription)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_vertex_positions(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVertexPositions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_vertex_positions(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CreateVertexPositions)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_vertex_normals(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetVertexNormals)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_vertex_normals(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CreateVertexNormals)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_triangle_indices(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTriangleIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_triangle_indices(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CreateTriangleIndices)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_triangle_material_indices(&self) -> Result<Option<ComPtr<super::super::storage::streams::IBuffer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).GetTriangleMaterialIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_triangle_material_indices(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).CreateTriangleMaterialIndices)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_buffer_description_set(&self) -> Result<Option<ComPtr<foundation::collections::IPropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BufferDescriptionSet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_buffer_set(&self) -> Result<Option<ComPtr<foundation::collections::IPropertySet>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_BufferSet)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn verify_async(&self, value: Printing3DMeshVerificationMode) -> Result<ComPtr<foundation::IAsyncOperation<Printing3DMeshVerificationResult>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).VerifyAsync)(self as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMesh: IPrinting3DMesh}
impl RtActivatable<IActivationFactory> for Printing3DMesh {}
DEFINE_CLSID!(Printing3DMesh(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,101,115,104,0]) [CLSID_Printing3DMesh]);
RT_ENUM! { enum Printing3DMeshVerificationMode: i32 {
    FindFirstError = 0, FindAllErrors = 1,
}}
DEFINE_IID!(IID_IPrinting3DMeshVerificationResult, 425095610, 59706, 20106, 164, 111, 222, 168, 232, 82, 25, 126);
RT_INTERFACE!{interface IPrinting3DMeshVerificationResult(IPrinting3DMeshVerificationResultVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMeshVerificationResult] {
    fn get_IsValid(&self, out: *mut bool) -> HRESULT,
    fn get_NonmanifoldTriangles(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT,
    fn get_ReversedNormalTriangles(&self, out: *mut *mut foundation::collections::IVectorView<u32>) -> HRESULT
}}
impl IPrinting3DMeshVerificationResult {
    #[inline] pub fn get_is_valid(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_IsValid)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_nonmanifold_triangles(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_NonmanifoldTriangles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reversed_normal_triangles(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_ReversedNormalTriangles)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMeshVerificationResult: IPrinting3DMeshVerificationResult}
DEFINE_IID!(IID_IPrinting3DModel, 755052272, 21243, 37274, 119, 176, 75, 26, 59, 128, 50, 79);
RT_INTERFACE!{interface IPrinting3DModel(IPrinting3DModelVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DModel] {
    fn get_Unit(&self, out: *mut Printing3DModelUnit) -> HRESULT,
    fn put_Unit(&self, value: Printing3DModelUnit) -> HRESULT,
    fn get_Textures(&self, out: *mut *mut foundation::collections::IVector<Printing3DModelTexture>) -> HRESULT,
    fn get_Meshes(&self, out: *mut *mut foundation::collections::IVector<Printing3DMesh>) -> HRESULT,
    fn get_Components(&self, out: *mut *mut foundation::collections::IVector<Printing3DComponent>) -> HRESULT,
    fn get_Material(&self, out: *mut *mut Printing3DMaterial) -> HRESULT,
    fn put_Material(&self, value: *mut Printing3DMaterial) -> HRESULT,
    fn get_Build(&self, out: *mut *mut Printing3DComponent) -> HRESULT,
    fn put_Build(&self, value: *mut Printing3DComponent) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT,
    fn get_RequiredExtensions(&self, out: *mut *mut foundation::collections::IVector<HString>) -> HRESULT,
    fn get_Metadata(&self, out: *mut *mut foundation::collections::IMap<HString, HString>) -> HRESULT,
    fn RepairAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn Clone(&self, out: *mut *mut Printing3DModel) -> HRESULT
}}
impl IPrinting3DModel {
    #[inline] pub fn get_unit(&self) -> Result<Printing3DModelUnit> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_Unit)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_unit(&self, value: Printing3DModelUnit) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Unit)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_textures(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DModelTexture>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Textures)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_meshes(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DMesh>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Meshes)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_components(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DComponent>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Components)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material(&self) -> Result<Option<ComPtr<Printing3DMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Material)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_material(&self, value: &Printing3DMaterial) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Material)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_build(&self) -> Result<Option<ComPtr<Printing3DComponent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Build)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_build(&self, value: &Printing3DComponent) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Build)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_version(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Version)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_version(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Version)(self as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_required_extensions(&self) -> Result<Option<ComPtr<foundation::collections::IVector<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_RequiredExtensions)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<ComPtr<foundation::collections::IMap<HString, HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Metadata)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn repair_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RepairAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn clone(&self) -> Result<Option<ComPtr<Printing3DModel>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Clone)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DModel: IPrinting3DModel}
impl RtActivatable<IActivationFactory> for Printing3DModel {}
DEFINE_CLSID!(Printing3DModel(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,111,100,101,108,0]) [CLSID_Printing3DModel]);
DEFINE_IID!(IID_IPrinting3DModel2, 3374344647, 51265, 18419, 168, 78, 161, 73, 253, 8, 182, 87);
RT_INTERFACE!{interface IPrinting3DModel2(IPrinting3DModel2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DModel2] {
    fn TryPartialRepairAsync(&self, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryPartialRepairWithTimeAsync(&self, maxWaitTime: foundation::TimeSpan, out: *mut *mut foundation::IAsyncOperation<bool>) -> HRESULT,
    fn TryReduceFacesAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<bool, f64>) -> HRESULT,
    fn TryReduceFacesWithOptionsAsync(&self, printing3DFaceReductionOptions: *mut Printing3DFaceReductionOptions, out: *mut *mut foundation::IAsyncOperationWithProgress<bool, f64>) -> HRESULT,
    fn TryReduceFacesWithOptionsAndTimeAsync(&self, printing3DFaceReductionOptions: *mut Printing3DFaceReductionOptions, maxWait: foundation::TimeSpan, out: *mut *mut foundation::IAsyncOperationWithProgress<bool, f64>) -> HRESULT,
    fn RepairWithProgressAsync(&self, out: *mut *mut foundation::IAsyncOperationWithProgress<bool, f64>) -> HRESULT
}}
impl IPrinting3DModel2 {
    #[inline] pub fn try_partial_repair_async(&self) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryPartialRepairAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_partial_repair_with_time_async(&self, maxWaitTime: foundation::TimeSpan) -> Result<ComPtr<foundation::IAsyncOperation<bool>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryPartialRepairWithTimeAsync)(self as *const _ as *mut _, maxWaitTime, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<bool, f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryReduceFacesAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_with_options_async(&self, printing3DFaceReductionOptions: &Printing3DFaceReductionOptions) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<bool, f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryReduceFacesWithOptionsAsync)(self as *const _ as *mut _, printing3DFaceReductionOptions as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_with_options_and_time_async(&self, printing3DFaceReductionOptions: &Printing3DFaceReductionOptions, maxWait: foundation::TimeSpan) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<bool, f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).TryReduceFacesWithOptionsAndTimeAsync)(self as *const _ as *mut _, printing3DFaceReductionOptions as *const _ as *mut _, maxWait, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn repair_with_progress_async(&self) -> Result<ComPtr<foundation::IAsyncOperationWithProgress<bool, f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).RepairWithProgressAsync)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DModelTexture, 1571802881, 46493, 18492, 151, 187, 164, 213, 70, 209, 199, 92);
RT_INTERFACE!{interface IPrinting3DModelTexture(IPrinting3DModelTextureVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DModelTexture] {
    fn get_TextureResource(&self, out: *mut *mut Printing3DTextureResource) -> HRESULT,
    fn put_TextureResource(&self, value: *mut Printing3DTextureResource) -> HRESULT,
    fn get_TileStyleU(&self, out: *mut Printing3DTextureEdgeBehavior) -> HRESULT,
    fn put_TileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> HRESULT,
    fn get_TileStyleV(&self, out: *mut Printing3DTextureEdgeBehavior) -> HRESULT,
    fn put_TileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> HRESULT
}}
impl IPrinting3DModelTexture {
    #[inline] pub fn get_texture_resource(&self) -> Result<Option<ComPtr<Printing3DTextureResource>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TextureResource)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture_resource(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TextureResource)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_tile_style_u(&self) -> Result<Printing3DTextureEdgeBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TileStyleU)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_tile_style_u(&self, value: Printing3DTextureEdgeBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TileStyleU)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_tile_style_v(&self) -> Result<Printing3DTextureEdgeBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_TileStyleV)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_tile_style_v(&self, value: Printing3DTextureEdgeBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TileStyleV)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DModelTexture: IPrinting3DModelTexture}
impl RtActivatable<IActivationFactory> for Printing3DModelTexture {}
DEFINE_CLSID!(Printing3DModelTexture(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,111,100,101,108,84,101,120,116,117,114,101,0]) [CLSID_Printing3DModelTexture]);
RT_ENUM! { enum Printing3DModelUnit: i32 {
    Meter = 0, Micron = 1, Millimeter = 2, Centimeter = 3, Inch = 4, Foot = 5,
}}
DEFINE_IID!(IID_IPrinting3DMultiplePropertyMaterial, 631645515, 50921, 18509, 162, 20, 162, 94, 87, 118, 186, 98);
RT_INTERFACE!{interface IPrinting3DMultiplePropertyMaterial(IPrinting3DMultiplePropertyMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterial] {
    fn get_MaterialIndices(&self, out: *mut *mut foundation::collections::IVector<u32>) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterial {
    #[inline] pub fn get_material_indices(&self) -> Result<Option<ComPtr<foundation::collections::IVector<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaterialIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMultiplePropertyMaterial: IPrinting3DMultiplePropertyMaterial}
impl RtActivatable<IActivationFactory> for Printing3DMultiplePropertyMaterial {}
DEFINE_CLSID!(Printing3DMultiplePropertyMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,117,108,116,105,112,108,101,80,114,111,112,101,114,116,121,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DMultiplePropertyMaterial]);
DEFINE_IID!(IID_IPrinting3DMultiplePropertyMaterialGroup, 4036298009, 44729, 17685, 163, 155, 160, 136, 251, 187, 39, 124);
RT_INTERFACE!{interface IPrinting3DMultiplePropertyMaterialGroup(IPrinting3DMultiplePropertyMaterialGroupVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterialGroup] {
    fn get_MultipleProperties(&self, out: *mut *mut foundation::collections::IVector<Printing3DMultiplePropertyMaterial>) -> HRESULT,
    fn get_MaterialGroupIndices(&self, out: *mut *mut foundation::collections::IVector<u32>) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterialGroup {
    #[inline] pub fn get_multiple_properties(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DMultiplePropertyMaterial>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MultipleProperties)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_indices(&self) -> Result<Option<ComPtr<foundation::collections::IVector<u32>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_MaterialGroupIndices)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaterialGroupId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMultiplePropertyMaterialGroup: IPrinting3DMultiplePropertyMaterialGroup}
impl RtActivatable<IPrinting3DMultiplePropertyMaterialGroupFactory> for Printing3DMultiplePropertyMaterialGroup {}
impl Printing3DMultiplePropertyMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<ComPtr<Printing3DMultiplePropertyMaterialGroup>> {
        <Self as RtActivatable<IPrinting3DMultiplePropertyMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DMultiplePropertyMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,117,108,116,105,112,108,101,80,114,111,112,101,114,116,121,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DMultiplePropertyMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DMultiplePropertyMaterialGroupFactory, 842930542, 54470, 17694, 168, 20, 77, 120, 162, 16, 254, 83);
RT_INTERFACE!{static interface IPrinting3DMultiplePropertyMaterialGroupFactory(IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut *mut Printing3DMultiplePropertyMaterialGroup) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<ComPtr<Printing3DMultiplePropertyMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum Printing3DObjectType: i32 {
    Model = 0, Support = 1, Others = 2,
}}
RT_ENUM! { enum Printing3DPackageCompression: i32 {
    Low = 0, Medium = 1, High = 2,
}}
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterial, 2374257659, 2025, 18822, 152, 51, 141, 211, 212, 140, 104, 89);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterial(IPrinting3DTexture2CoordMaterialVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterial] {
    fn get_Texture(&self, out: *mut *mut Printing3DModelTexture) -> HRESULT,
    fn put_Texture(&self, value: *mut Printing3DModelTexture) -> HRESULT,
    fn get_U(&self, out: *mut f64) -> HRESULT,
    fn put_U(&self, value: f64) -> HRESULT,
    fn get_V(&self, out: *mut f64) -> HRESULT,
    fn put_V(&self, value: f64) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterial {
    #[inline] pub fn get_texture(&self) -> Result<Option<ComPtr<Printing3DModelTexture>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Texture)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture(&self, value: &Printing3DModelTexture) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Texture)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_u(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_U)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_u(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_U)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_v(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_V)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_v(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_V)(self as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DTexture2CoordMaterial: IPrinting3DTexture2CoordMaterial}
impl RtActivatable<IActivationFactory> for Printing3DTexture2CoordMaterial {}
DEFINE_CLSID!(Printing3DTexture2CoordMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,50,67,111,111,114,100,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DTexture2CoordMaterial]);
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroup, 1652391079, 28048, 20409, 159, 196, 159, 239, 243, 223, 168, 146);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterialGroup(IPrinting3DTexture2CoordMaterialGroupVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroup] {
    fn get_Texture2Coords(&self, out: *mut *mut foundation::collections::IVector<Printing3DTexture2CoordMaterial>) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroup {
    #[inline] pub fn get_texture2_coords(&self) -> Result<Option<ComPtr<foundation::collections::IVector<Printing3DTexture2CoordMaterial>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Texture2Coords)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.lpVtbl).get_MaterialGroupId)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DTexture2CoordMaterialGroup: IPrinting3DTexture2CoordMaterialGroup}
impl RtActivatable<IPrinting3DTexture2CoordMaterialGroupFactory> for Printing3DTexture2CoordMaterialGroup {}
impl Printing3DTexture2CoordMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<ComPtr<Printing3DTexture2CoordMaterialGroup>> {
        <Self as RtActivatable<IPrinting3DTexture2CoordMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DTexture2CoordMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,50,67,111,111,114,100,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DTexture2CoordMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroup2, 1778113466, 45358, 17051, 131, 134, 223, 82, 132, 246, 232, 15);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterialGroup2(IPrinting3DTexture2CoordMaterialGroup2Vtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroup2] {
    fn get_Texture(&self, out: *mut *mut Printing3DModelTexture) -> HRESULT,
    fn put_Texture(&self, value: *mut Printing3DModelTexture) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroup2 {
    #[inline] pub fn get_texture(&self) -> Result<Option<ComPtr<Printing3DModelTexture>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_Texture)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture(&self, value: &Printing3DModelTexture) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_Texture)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroupFactory, 3417328048, 18058, 19567, 178, 162, 142, 184, 186, 141, 234, 72);
RT_INTERFACE!{static interface IPrinting3DTexture2CoordMaterialGroupFactory(IPrinting3DTexture2CoordMaterialGroupFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut *mut Printing3DTexture2CoordMaterialGroup) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<ComPtr<Printing3DTexture2CoordMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).Create)(self as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum Printing3DTextureEdgeBehavior: i32 {
    None = 0, Wrap = 1, Mirror = 2, Clamp = 3,
}}
DEFINE_IID!(IID_IPrinting3DTextureResource, 2802709293, 27313, 17582, 188, 69, 162, 115, 130, 192, 211, 140);
RT_INTERFACE!{interface IPrinting3DTextureResource(IPrinting3DTextureResourceVtbl): IInspectable(IInspectableVtbl) [IID_IPrinting3DTextureResource] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_TextureData(&self, out: *mut *mut super::super::storage::streams::IRandomAccessStreamWithContentType) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_TextureData(&self, value: *mut super::super::storage::streams::IRandomAccessStreamWithContentType) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT
}}
impl IPrinting3DTextureResource {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_texture_data(&self) -> Result<Option<ComPtr<super::super::storage::streams::IRandomAccessStreamWithContentType>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.lpVtbl).get_TextureData)(self as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_texture_data(&self, value: &super::super::storage::streams::IRandomAccessStreamWithContentType) -> Result<()> { unsafe { 
        let hr = ((*self.lpVtbl).put_TextureData)(self as *const _ as *mut _, value as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
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
RT_CLASS!{class Printing3DTextureResource: IPrinting3DTextureResource}
impl RtActivatable<IActivationFactory> for Printing3DTextureResource {}
DEFINE_CLSID!(Printing3DTextureResource(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,82,101,115,111,117,114,99,101,0]) [CLSID_Printing3DTextureResource]);
} // Windows.Graphics.Printing3D
