use crate::prelude::*;
RT_STRUCT! { struct DisplayAdapterId {
    LowPart: u32, HighPart: i32,
}}
DEFINE_IID!(IID_IGeometrySource2D, 3405740290, 26380, 16769, 166, 36, 218, 151, 114, 3, 184, 69);
RT_INTERFACE!{interface IGeometrySource2D(IGeometrySource2DVtbl, IGeometrySource2D_Abi): IInspectable(IInspectableVtbl) [IID_IGeometrySource2D] {
    
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
RT_INTERFACE!{interface IDirect3D11CaptureFrame(IDirect3D11CaptureFrameVtbl, IDirect3D11CaptureFrame_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFrame] {
    fn get_Surface(&self, out: *mut <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi) -> HRESULT,
    fn get_SystemRelativeTime(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_ContentSize(&self, out: *mut super::SizeInt32) -> HRESULT
}}
impl IDirect3D11CaptureFrame {
    #[inline] pub fn get_surface(&self) -> Result<Option<super::directx::direct3d11::IDirect3DSurface>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Surface)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::directx::direct3d11::IDirect3DSurface::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_relative_time(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SystemRelativeTime)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_content_size(&self) -> Result<super::SizeInt32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ContentSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Direct3D11CaptureFrame: IDirect3D11CaptureFrame}
DEFINE_IID!(IID_IDirect3D11CaptureFramePool, 619408674, 6517, 16942, 130, 231, 120, 13, 189, 141, 223, 36);
RT_INTERFACE!{interface IDirect3D11CaptureFramePool(IDirect3D11CaptureFramePoolVtbl, IDirect3D11CaptureFramePool_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePool] {
    fn Recreate(&self, device: <super::directx::direct3d11::IDirect3DDevice as RtType>::Abi, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> HRESULT,
    fn TryGetNextFrame(&self, out: *mut <Direct3D11CaptureFrame as RtType>::Abi) -> HRESULT,
    fn add_FrameArrived(&self, handler: <foundation::TypedEventHandler<Direct3D11CaptureFramePool, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_FrameArrived(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateCaptureSession(&self, item: <GraphicsCaptureItem as RtType>::Abi, out: *mut <GraphicsCaptureSession as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-system")] fn get_DispatcherQueue(&self, out: *mut <super::super::system::DispatcherQueue as RtType>::Abi) -> HRESULT
}}
impl IDirect3D11CaptureFramePool {
    #[inline] pub fn recreate(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Recreate)(self.get_abi() as *const _ as *mut _, device.get_abi() as *const _ as *mut _, pixelFormat, numberOfBuffers, size);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn try_get_next_frame(&self) -> Result<Option<Direct3D11CaptureFrame>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetNextFrame)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Direct3D11CaptureFrame::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_frame_arrived(&self, handler: &foundation::TypedEventHandler<Direct3D11CaptureFramePool, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_FrameArrived)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_frame_arrived(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_FrameArrived)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_capture_session(&self, item: &GraphicsCaptureItem) -> Result<Option<GraphicsCaptureSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCaptureSession)(self.get_abi() as *const _ as *mut _, item.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(GraphicsCaptureSession::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-system")] #[inline] pub fn get_dispatcher_queue(&self) -> Result<Option<super::super::system::DispatcherQueue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DispatcherQueue)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::system::DispatcherQueue::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Direct3D11CaptureFramePool: IDirect3D11CaptureFramePool}
impl RtActivatable<IDirect3D11CaptureFramePoolStatics> for Direct3D11CaptureFramePool {}
impl RtActivatable<IDirect3D11CaptureFramePoolStatics2> for Direct3D11CaptureFramePool {}
impl Direct3D11CaptureFramePool {
    #[inline] pub fn create(device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<Direct3D11CaptureFramePool>> {
        <Self as RtActivatable<IDirect3D11CaptureFramePoolStatics>>::get_activation_factory().create(device, pixelFormat, numberOfBuffers, size)
    }
    #[inline] pub fn create_free_threaded(device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<Direct3D11CaptureFramePool>> {
        <Self as RtActivatable<IDirect3D11CaptureFramePoolStatics2>>::get_activation_factory().create_free_threaded(device, pixelFormat, numberOfBuffers, size)
    }
}
DEFINE_CLSID!(Direct3D11CaptureFramePool(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,68,105,114,101,99,116,51,68,49,49,67,97,112,116,117,114,101,70,114,97,109,101,80,111,111,108,0]) [CLSID_Direct3D11CaptureFramePool]);
DEFINE_IID!(IID_IDirect3D11CaptureFramePoolStatics, 2005140842, 26538, 19795, 174, 84, 16, 136, 213, 168, 202, 33);
RT_INTERFACE!{static interface IDirect3D11CaptureFramePoolStatics(IDirect3D11CaptureFramePoolStaticsVtbl, IDirect3D11CaptureFramePoolStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePoolStatics] {
    fn Create(&self, device: <super::directx::direct3d11::IDirect3DDevice as RtType>::Abi, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32, out: *mut <Direct3D11CaptureFramePool as RtType>::Abi) -> HRESULT
}}
impl IDirect3D11CaptureFramePoolStatics {
    #[inline] pub fn create(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<Direct3D11CaptureFramePool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, device.get_abi() as *const _ as *mut _, pixelFormat, numberOfBuffers, size, &mut out);
        if hr == S_OK { Ok(Direct3D11CaptureFramePool::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDirect3D11CaptureFramePoolStatics2, 1486557247, 27580, 24053, 169, 145, 2, 226, 139, 59, 102, 213);
RT_INTERFACE!{static interface IDirect3D11CaptureFramePoolStatics2(IDirect3D11CaptureFramePoolStatics2Vtbl, IDirect3D11CaptureFramePoolStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3D11CaptureFramePoolStatics2] {
    fn CreateFreeThreaded(&self, device: <super::directx::direct3d11::IDirect3DDevice as RtType>::Abi, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32, out: *mut <Direct3D11CaptureFramePool as RtType>::Abi) -> HRESULT
}}
impl IDirect3D11CaptureFramePoolStatics2 {
    #[inline] pub fn create_free_threaded(&self, device: &super::directx::direct3d11::IDirect3DDevice, pixelFormat: super::directx::DirectXPixelFormat, numberOfBuffers: i32, size: super::SizeInt32) -> Result<Option<Direct3D11CaptureFramePool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFreeThreaded)(self.get_abi() as *const _ as *mut _, device.get_abi() as *const _ as *mut _, pixelFormat, numberOfBuffers, size, &mut out);
        if hr == S_OK { Ok(Direct3D11CaptureFramePool::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsCaptureItem, 2042886491, 12791, 20162, 164, 100, 99, 46, 245, 211, 7, 96);
RT_INTERFACE!{interface IGraphicsCaptureItem(IGraphicsCaptureItemVtbl, IGraphicsCaptureItem_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureItem] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Size(&self, out: *mut super::SizeInt32) -> HRESULT,
    fn add_Closed(&self, handler: <foundation::TypedEventHandler<GraphicsCaptureItem, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Closed(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IGraphicsCaptureItem {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<super::SizeInt32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Size)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_closed(&self, handler: &foundation::TypedEventHandler<GraphicsCaptureItem, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Closed)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_closed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Closed)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class GraphicsCaptureItem: IGraphicsCaptureItem}
impl RtActivatable<IGraphicsCaptureItemStatics> for GraphicsCaptureItem {}
impl GraphicsCaptureItem {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_from_visual(visual: &super::super::ui::composition::Visual) -> Result<Option<GraphicsCaptureItem>> {
        <Self as RtActivatable<IGraphicsCaptureItemStatics>>::get_activation_factory().create_from_visual(visual)
    }
}
DEFINE_CLSID!(GraphicsCaptureItem(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,71,114,97,112,104,105,99,115,67,97,112,116,117,114,101,73,116,101,109,0]) [CLSID_GraphicsCaptureItem]);
DEFINE_IID!(IID_IGraphicsCaptureItemStatics, 2826878629, 17788, 22408, 171, 71, 12, 241, 211, 99, 126, 116);
RT_INTERFACE!{static interface IGraphicsCaptureItemStatics(IGraphicsCaptureItemStaticsVtbl, IGraphicsCaptureItemStatics_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureItemStatics] {
    #[cfg(feature="windows-ui")] fn CreateFromVisual(&self, visual: <super::super::ui::composition::Visual as RtType>::Abi, out: *mut <GraphicsCaptureItem as RtType>::Abi) -> HRESULT
}}
impl IGraphicsCaptureItemStatics {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_from_visual(&self, visual: &super::super::ui::composition::Visual) -> Result<Option<GraphicsCaptureItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromVisual)(self.get_abi() as *const _ as *mut _, visual.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(GraphicsCaptureItem::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsCapturePicker, 1511461299, 44409, 19274, 147, 54, 19, 24, 253, 222, 53, 57);
RT_INTERFACE!{interface IGraphicsCapturePicker(IGraphicsCapturePickerVtbl, IGraphicsCapturePicker_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsCapturePicker] {
    fn PickSingleItemAsync(&self, out: *mut <foundation::IAsyncOperation<GraphicsCaptureItem> as RtType>::Abi) -> HRESULT
}}
impl IGraphicsCapturePicker {
    #[inline] pub fn pick_single_item_async(&self) -> Result<foundation::IAsyncOperation<GraphicsCaptureItem>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PickSingleItemAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class GraphicsCapturePicker: IGraphicsCapturePicker}
impl RtActivatable<IActivationFactory> for GraphicsCapturePicker {}
DEFINE_CLSID!(GraphicsCapturePicker(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,67,97,112,116,117,114,101,46,71,114,97,112,104,105,99,115,67,97,112,116,117,114,101,80,105,99,107,101,114,0]) [CLSID_GraphicsCapturePicker]);
DEFINE_IID!(IID_IGraphicsCaptureSession, 2169389737, 63247, 19159, 147, 155, 253, 220, 198, 235, 136, 13);
RT_INTERFACE!{interface IGraphicsCaptureSession(IGraphicsCaptureSessionVtbl, IGraphicsCaptureSession_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureSession] {
    fn StartCapture(&self) -> HRESULT
}}
impl IGraphicsCaptureSession {
    #[inline] pub fn start_capture(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).StartCapture)(self.get_abi() as *const _ as *mut _);
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
RT_INTERFACE!{static interface IGraphicsCaptureSessionStatics(IGraphicsCaptureSessionStaticsVtbl, IGraphicsCaptureSessionStatics_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsCaptureSessionStatics] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IGraphicsCaptureSessionStatics {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IDirect3DDevice(IDirect3DDeviceVtbl, IDirect3DDevice_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3DDevice] {
    fn Trim(&self) -> HRESULT
}}
impl IDirect3DDevice {
    #[inline] pub fn trim(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Trim)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_STRUCT! { struct Direct3DMultisampleDescription {
    Count: i32, Quality: i32,
}}
DEFINE_IID!(IID_IDirect3DSurface, 200581446, 5057, 18068, 190, 227, 122, 191, 21, 234, 245, 134);
RT_INTERFACE!{interface IDirect3DSurface(IDirect3DSurfaceVtbl, IDirect3DSurface_Abi): IInspectable(IInspectableVtbl) [IID_IDirect3DSurface] {
    fn get_Description(&self, out: *mut Direct3DSurfaceDescription) -> HRESULT
}}
impl IDirect3DSurface {
    #[inline] pub fn get_description(&self) -> Result<Direct3DSurfaceDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IAdvancedColorInfo(IAdvancedColorInfoVtbl, IAdvancedColorInfo_Abi): IInspectable(IInspectableVtbl) [IID_IAdvancedColorInfo] {
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
        let hr = ((*self.get_abi().lpVtbl).get_CurrentAdvancedColorKind)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_red_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RedPrimary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_green_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_GreenPrimary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_blue_primary(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BluePrimary)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_white_point(&self) -> Result<foundation::Point> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_WhitePoint)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxLuminanceInNits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MinLuminanceInNits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_average_full_frame_luminance_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxAverageFullFrameLuminanceInNits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_sdr_white_level_in_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SdrWhiteLevelInNits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_hdr_metadata_format_currently_supported(&self, format: HdrMetadataFormat) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsHdrMetadataFormatCurrentlySupported)(self.get_abi() as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_advanced_color_kind_available(&self, kind: AdvancedColorKind) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsAdvancedColorKindAvailable)(self.get_abi() as *const _ as *mut _, kind, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AdvancedColorInfo: IAdvancedColorInfo}
RT_ENUM! { enum AdvancedColorKind: i32 {
    StandardDynamicRange = 0, WideColorGamut = 1, HighDynamicRange = 2,
}}
DEFINE_IID!(IID_IBrightnessOverride, 2529780250, 49475, 17298, 190, 221, 74, 126, 149, 116, 200, 253);
RT_INTERFACE!{interface IBrightnessOverride(IBrightnessOverrideVtbl, IBrightnessOverride_Abi): IInspectable(IInspectableVtbl) [IID_IBrightnessOverride] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsOverrideActive(&self, out: *mut bool) -> HRESULT,
    fn get_BrightnessLevel(&self, out: *mut f64) -> HRESULT,
    fn SetBrightnessLevel(&self, brightnessLevel: f64, options: DisplayBrightnessOverrideOptions) -> HRESULT,
    fn SetBrightnessScenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> HRESULT,
    fn GetLevelForScenario(&self, scenario: DisplayBrightnessScenario, out: *mut f64) -> HRESULT,
    fn StartOverride(&self) -> HRESULT,
    fn StopOverride(&self) -> HRESULT,
    fn add_IsSupportedChanged(&self, handler: <foundation::TypedEventHandler<BrightnessOverride, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsSupportedChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsOverrideActiveChanged(&self, handler: <foundation::TypedEventHandler<BrightnessOverride, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsOverrideActiveChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_BrightnessLevelChanged(&self, handler: <foundation::TypedEventHandler<BrightnessOverride, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BrightnessLevelChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IBrightnessOverride {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_override_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsOverrideActive)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_brightness_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BrightnessLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_level(&self, brightnessLevel: f64, options: DisplayBrightnessOverrideOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetBrightnessLevel)(self.get_abi() as *const _ as *mut _, brightnessLevel, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_scenario(&self, scenario: DisplayBrightnessScenario, options: DisplayBrightnessOverrideOptions) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetBrightnessScenario)(self.get_abi() as *const _ as *mut _, scenario, options);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_level_for_scenario(&self, scenario: DisplayBrightnessScenario) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetLevelForScenario)(self.get_abi() as *const _ as *mut _, scenario, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).StartOverride)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).StopOverride)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_supported_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_IsSupportedChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_supported_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_IsSupportedChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_override_active_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_IsOverrideActiveChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_override_active_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_IsOverrideActiveChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_brightness_level_changed(&self, handler: &foundation::TypedEventHandler<BrightnessOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_BrightnessLevelChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_brightness_level_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_BrightnessLevelChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BrightnessOverride: IBrightnessOverride}
impl RtActivatable<IBrightnessOverrideStatics> for BrightnessOverride {}
impl BrightnessOverride {
    #[inline] pub fn get_default_for_system() -> Result<Option<BrightnessOverride>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().get_default_for_system()
    }
    #[inline] pub fn get_for_current_view() -> Result<Option<BrightnessOverride>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn save_for_system_async(value: &BrightnessOverride) -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IBrightnessOverrideStatics>>::get_activation_factory().save_for_system_async(value)
    }
}
DEFINE_CLSID!(BrightnessOverride(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,66,114,105,103,104,116,110,101,115,115,79,118,101,114,114,105,100,101,0]) [CLSID_BrightnessOverride]);
DEFINE_IID!(IID_IBrightnessOverrideSettings, 3507661610, 30212, 19898, 188, 248, 75, 111, 73, 80, 44, 176);
RT_INTERFACE!{interface IBrightnessOverrideSettings(IBrightnessOverrideSettingsVtbl, IBrightnessOverrideSettings_Abi): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideSettings] {
    fn get_DesiredLevel(&self, out: *mut f64) -> HRESULT,
    fn get_DesiredNits(&self, out: *mut f32) -> HRESULT
}}
impl IBrightnessOverrideSettings {
    #[inline] pub fn get_desired_level(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DesiredLevel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_desired_nits(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DesiredNits)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BrightnessOverrideSettings: IBrightnessOverrideSettings}
impl RtActivatable<IBrightnessOverrideSettingsStatics> for BrightnessOverrideSettings {}
impl BrightnessOverrideSettings {
    #[inline] pub fn create_from_level(level: f64) -> Result<Option<BrightnessOverrideSettings>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_level(level)
    }
    #[inline] pub fn create_from_nits(nits: f32) -> Result<Option<BrightnessOverrideSettings>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_nits(nits)
    }
    #[inline] pub fn create_from_display_brightness_override_scenario(overrideScenario: DisplayBrightnessOverrideScenario) -> Result<Option<BrightnessOverrideSettings>> {
        <Self as RtActivatable<IBrightnessOverrideSettingsStatics>>::get_activation_factory().create_from_display_brightness_override_scenario(overrideScenario)
    }
}
DEFINE_CLSID!(BrightnessOverrideSettings(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,66,114,105,103,104,116,110,101,115,115,79,118,101,114,114,105,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_BrightnessOverrideSettings]);
DEFINE_IID!(IID_IBrightnessOverrideSettingsStatics, 3565673616, 28532, 17419, 179, 131, 95, 233, 108, 240, 11, 15);
RT_INTERFACE!{static interface IBrightnessOverrideSettingsStatics(IBrightnessOverrideSettingsStaticsVtbl, IBrightnessOverrideSettingsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideSettingsStatics] {
    fn CreateFromLevel(&self, level: f64, out: *mut <BrightnessOverrideSettings as RtType>::Abi) -> HRESULT,
    fn CreateFromNits(&self, nits: f32, out: *mut <BrightnessOverrideSettings as RtType>::Abi) -> HRESULT,
    fn CreateFromDisplayBrightnessOverrideScenario(&self, overrideScenario: DisplayBrightnessOverrideScenario, out: *mut <BrightnessOverrideSettings as RtType>::Abi) -> HRESULT
}}
impl IBrightnessOverrideSettingsStatics {
    #[inline] pub fn create_from_level(&self, level: f64) -> Result<Option<BrightnessOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromLevel)(self.get_abi() as *const _ as *mut _, level, &mut out);
        if hr == S_OK { Ok(BrightnessOverrideSettings::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_nits(&self, nits: f32) -> Result<Option<BrightnessOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromNits)(self.get_abi() as *const _ as *mut _, nits, &mut out);
        if hr == S_OK { Ok(BrightnessOverrideSettings::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_from_display_brightness_override_scenario(&self, overrideScenario: DisplayBrightnessOverrideScenario) -> Result<Option<BrightnessOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromDisplayBrightnessOverrideScenario)(self.get_abi() as *const _ as *mut _, overrideScenario, &mut out);
        if hr == S_OK { Ok(BrightnessOverrideSettings::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBrightnessOverrideStatics, 61323757, 57841, 19048, 161, 31, 148, 106, 216, 206, 83, 147);
RT_INTERFACE!{static interface IBrightnessOverrideStatics(IBrightnessOverrideStaticsVtbl, IBrightnessOverrideStatics_Abi): IInspectable(IInspectableVtbl) [IID_IBrightnessOverrideStatics] {
    fn GetDefaultForSystem(&self, out: *mut <BrightnessOverride as RtType>::Abi) -> HRESULT,
    fn GetForCurrentView(&self, out: *mut <BrightnessOverride as RtType>::Abi) -> HRESULT,
    fn SaveForSystemAsync(&self, value: <BrightnessOverride as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IBrightnessOverrideStatics {
    #[inline] pub fn get_default_for_system(&self) -> Result<Option<BrightnessOverride>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDefaultForSystem)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BrightnessOverride::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<BrightnessOverride>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BrightnessOverride::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_for_system_async(&self, value: &BrightnessOverride) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SaveForSystemAsync)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IColorOverrideSettings, 4226785588, 19073, 19533, 165, 182, 125, 27, 92, 75, 208, 11);
RT_INTERFACE!{interface IColorOverrideSettings(IColorOverrideSettingsVtbl, IColorOverrideSettings_Abi): IInspectable(IInspectableVtbl) [IID_IColorOverrideSettings] {
    fn get_DesiredDisplayColorOverrideScenario(&self, out: *mut DisplayColorOverrideScenario) -> HRESULT
}}
impl IColorOverrideSettings {
    #[inline] pub fn get_desired_display_color_override_scenario(&self) -> Result<DisplayColorOverrideScenario> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DesiredDisplayColorOverrideScenario)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class ColorOverrideSettings: IColorOverrideSettings}
impl RtActivatable<IColorOverrideSettingsStatics> for ColorOverrideSettings {}
impl ColorOverrideSettings {
    #[inline] pub fn create_from_display_color_override_scenario(overrideScenario: DisplayColorOverrideScenario) -> Result<Option<ColorOverrideSettings>> {
        <Self as RtActivatable<IColorOverrideSettingsStatics>>::get_activation_factory().create_from_display_color_override_scenario(overrideScenario)
    }
}
DEFINE_CLSID!(ColorOverrideSettings(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,67,111,108,111,114,79,118,101,114,114,105,100,101,83,101,116,116,105,110,103,115,0]) [CLSID_ColorOverrideSettings]);
DEFINE_IID!(IID_IColorOverrideSettingsStatics, 2959663199, 50207, 19145, 175, 171, 130, 122, 182, 36, 143, 154);
RT_INTERFACE!{static interface IColorOverrideSettingsStatics(IColorOverrideSettingsStaticsVtbl, IColorOverrideSettingsStatics_Abi): IInspectable(IInspectableVtbl) [IID_IColorOverrideSettingsStatics] {
    fn CreateFromDisplayColorOverrideScenario(&self, overrideScenario: DisplayColorOverrideScenario, out: *mut <ColorOverrideSettings as RtType>::Abi) -> HRESULT
}}
impl IColorOverrideSettingsStatics {
    #[inline] pub fn create_from_display_color_override_scenario(&self, overrideScenario: DisplayColorOverrideScenario) -> Result<Option<ColorOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFromDisplayColorOverrideScenario)(self.get_abi() as *const _ as *mut _, overrideScenario, &mut out);
        if hr == S_OK { Ok(ColorOverrideSettings::wrap(out)) } else { err(hr) }
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
RT_INTERFACE!{interface IDisplayEnhancementOverride(IDisplayEnhancementOverrideVtbl, IDisplayEnhancementOverride_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverride] {
    fn get_ColorOverrideSettings(&self, out: *mut <ColorOverrideSettings as RtType>::Abi) -> HRESULT,
    fn put_ColorOverrideSettings(&self, value: <ColorOverrideSettings as RtType>::Abi) -> HRESULT,
    fn get_BrightnessOverrideSettings(&self, out: *mut <BrightnessOverrideSettings as RtType>::Abi) -> HRESULT,
    fn put_BrightnessOverrideSettings(&self, value: <BrightnessOverrideSettings as RtType>::Abi) -> HRESULT,
    fn get_CanOverride(&self, out: *mut bool) -> HRESULT,
    fn get_IsOverrideActive(&self, out: *mut bool) -> HRESULT,
    fn GetCurrentDisplayEnhancementOverrideCapabilities(&self, out: *mut <DisplayEnhancementOverrideCapabilities as RtType>::Abi) -> HRESULT,
    fn RequestOverride(&self) -> HRESULT,
    fn StopOverride(&self) -> HRESULT,
    fn add_CanOverrideChanged(&self, handler: <foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CanOverrideChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_IsOverrideActiveChanged(&self, handler: <foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsOverrideActiveChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DisplayEnhancementOverrideCapabilitiesChanged(&self, handler: <foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayEnhancementOverrideCapabilitiesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayEnhancementOverride {
    #[inline] pub fn get_color_override_settings(&self) -> Result<Option<ColorOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ColorOverrideSettings)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ColorOverrideSettings::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_color_override_settings(&self, value: &ColorOverrideSettings) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ColorOverrideSettings)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_brightness_override_settings(&self) -> Result<Option<BrightnessOverrideSettings>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BrightnessOverrideSettings)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BrightnessOverrideSettings::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_brightness_override_settings(&self, value: &BrightnessOverrideSettings) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_BrightnessOverrideSettings)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_can_override(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CanOverride)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_override_active(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsOverrideActive)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_display_enhancement_override_capabilities(&self) -> Result<Option<DisplayEnhancementOverrideCapabilities>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCurrentDisplayEnhancementOverrideCapabilities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DisplayEnhancementOverrideCapabilities::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).RequestOverride)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn stop_override(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).StopOverride)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_can_override_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_CanOverrideChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_can_override_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_CanOverrideChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_is_override_active_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_IsOverrideActiveChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_override_active_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_IsOverrideActiveChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_enhancement_override_capabilities_changed(&self, handler: &foundation::TypedEventHandler<DisplayEnhancementOverride, DisplayEnhancementOverrideCapabilitiesChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DisplayEnhancementOverrideCapabilitiesChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_enhancement_override_capabilities_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DisplayEnhancementOverrideCapabilitiesChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverride: IDisplayEnhancementOverride}
impl RtActivatable<IDisplayEnhancementOverrideStatics> for DisplayEnhancementOverride {}
impl DisplayEnhancementOverride {
    #[inline] pub fn get_for_current_view() -> Result<Option<DisplayEnhancementOverride>> {
        <Self as RtActivatable<IDisplayEnhancementOverrideStatics>>::get_activation_factory().get_for_current_view()
    }
}
DEFINE_CLSID!(DisplayEnhancementOverride(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,68,105,115,112,108,97,121,69,110,104,97,110,99,101,109,101,110,116,79,118,101,114,114,105,100,101,0]) [CLSID_DisplayEnhancementOverride]);
DEFINE_IID!(IID_IDisplayEnhancementOverrideCapabilities, 1164992734, 61018, 18359, 153, 24, 30, 81, 232, 18, 204, 200);
RT_INTERFACE!{interface IDisplayEnhancementOverrideCapabilities(IDisplayEnhancementOverrideCapabilitiesVtbl, IDisplayEnhancementOverrideCapabilities_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideCapabilities] {
    fn get_IsBrightnessControlSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsBrightnessNitsControlSupported(&self, out: *mut bool) -> HRESULT,
    fn GetSupportedNitRanges(&self, out: *mut <foundation::collections::IVectorView<NitRange> as RtType>::Abi) -> HRESULT
}}
impl IDisplayEnhancementOverrideCapabilities {
    #[inline] pub fn get_is_brightness_control_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsBrightnessControlSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_brightness_nits_control_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsBrightnessNitsControlSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_supported_nit_ranges(&self) -> Result<Option<foundation::collections::IVectorView<NitRange>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSupportedNitRanges)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverrideCapabilities: IDisplayEnhancementOverrideCapabilities}
DEFINE_IID!(IID_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs, 3680626276, 5626, 18906, 139, 119, 7, 219, 210, 175, 88, 93);
RT_INTERFACE!{interface IDisplayEnhancementOverrideCapabilitiesChangedEventArgs(IDisplayEnhancementOverrideCapabilitiesChangedEventArgsVtbl, IDisplayEnhancementOverrideCapabilitiesChangedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideCapabilitiesChangedEventArgs] {
    fn get_Capabilities(&self, out: *mut <DisplayEnhancementOverrideCapabilities as RtType>::Abi) -> HRESULT
}}
impl IDisplayEnhancementOverrideCapabilitiesChangedEventArgs {
    #[inline] pub fn get_capabilities(&self) -> Result<Option<DisplayEnhancementOverrideCapabilities>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Capabilities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DisplayEnhancementOverrideCapabilities::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayEnhancementOverrideCapabilitiesChangedEventArgs: IDisplayEnhancementOverrideCapabilitiesChangedEventArgs}
DEFINE_IID!(IID_IDisplayEnhancementOverrideStatics, 3478879937, 38801, 17491, 176, 19, 41, 182, 247, 120, 229, 25);
RT_INTERFACE!{static interface IDisplayEnhancementOverrideStatics(IDisplayEnhancementOverrideStaticsVtbl, IDisplayEnhancementOverrideStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayEnhancementOverrideStatics] {
    fn GetForCurrentView(&self, out: *mut <DisplayEnhancementOverride as RtType>::Abi) -> HRESULT
}}
impl IDisplayEnhancementOverrideStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<DisplayEnhancementOverride>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DisplayEnhancementOverride::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation, 3201372846, 44483, 19913, 174, 101, 133, 31, 77, 125, 71, 153);
RT_INTERFACE!{interface IDisplayInformation(IDisplayInformationVtbl, IDisplayInformation_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformation] {
    fn get_CurrentOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_NativeOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn add_OrientationChanged(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OrientationChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_ResolutionScale(&self, out: *mut ResolutionScale) -> HRESULT,
    fn get_LogicalDpi(&self, out: *mut f32) -> HRESULT,
    fn get_RawDpiX(&self, out: *mut f32) -> HRESULT,
    fn get_RawDpiY(&self, out: *mut f32) -> HRESULT,
    fn add_DpiChanged(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DpiChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn add_StereoEnabledChanged(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StereoEnabledChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetColorProfileAsync(&self, out: *mut <foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn add_ColorProfileChanged(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ColorProfileChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformation {
    #[inline] pub fn get_current_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CurrentOrientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_native_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NativeOrientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_orientation_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_OrientationChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_orientation_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_OrientationChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_scale(&self) -> Result<ResolutionScale> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ResolutionScale)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_logical_dpi(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LogicalDpi)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_dpi_x(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RawDpiX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_raw_dpi_y(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RawDpiY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_dpi_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DpiChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_dpi_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DpiChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_StereoEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_stereo_enabled_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_StereoEnabledChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stereo_enabled_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_StereoEnabledChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async(&self) -> Result<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetColorProfileAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_color_profile_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_ColorProfileChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_color_profile_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_ColorProfileChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class DisplayInformation: IDisplayInformation}
impl RtActivatable<IDisplayInformationStatics> for DisplayInformation {}
impl DisplayInformation {
    #[inline] pub fn get_for_current_view() -> Result<Option<DisplayInformation>> {
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
RT_INTERFACE!{interface IDisplayInformation2(IDisplayInformation2Vtbl, IDisplayInformation2_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformation2] {
    fn get_RawPixelsPerViewPixel(&self, out: *mut f64) -> HRESULT
}}
impl IDisplayInformation2 {
    #[inline] pub fn get_raw_pixels_per_view_pixel(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RawPixelsPerViewPixel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation3, 3675586845, 3849, 17510, 143, 243, 17, 222, 154, 60, 146, 154);
RT_INTERFACE!{interface IDisplayInformation3(IDisplayInformation3Vtbl, IDisplayInformation3_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformation3] {
    fn get_DiagonalSizeInInches(&self, out: *mut <foundation::IReference<f64> as RtType>::Abi) -> HRESULT
}}
impl IDisplayInformation3 {
    #[inline] pub fn get_diagonal_size_in_inches(&self) -> Result<Option<foundation::IReference<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DiagonalSizeInInches)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation4, 3379744303, 4674, 18110, 181, 54, 225, 170, 254, 158, 122, 207);
RT_INTERFACE!{interface IDisplayInformation4(IDisplayInformation4Vtbl, IDisplayInformation4_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformation4] {
    fn get_ScreenWidthInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_ScreenHeightInRawPixels(&self, out: *mut u32) -> HRESULT
}}
impl IDisplayInformation4 {
    #[inline] pub fn get_screen_width_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ScreenWidthInRawPixels)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_screen_height_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ScreenHeightInRawPixels)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformation5, 978600668, 11486, 19085, 128, 209, 33, 220, 90, 220, 193, 170);
RT_INTERFACE!{interface IDisplayInformation5(IDisplayInformation5Vtbl, IDisplayInformation5_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformation5] {
    fn GetAdvancedColorInfo(&self, out: *mut <AdvancedColorInfo as RtType>::Abi) -> HRESULT,
    fn add_AdvancedColorInfoChanged(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_AdvancedColorInfoChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformation5 {
    #[inline] pub fn get_advanced_color_info(&self) -> Result<Option<AdvancedColorInfo>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAdvancedColorInfo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(AdvancedColorInfo::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_advanced_color_info_changed(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_AdvancedColorInfoChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_advanced_color_info_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_AdvancedColorInfoChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayInformationStatics, 3332385388, 54354, 17628, 186, 7, 150, 243, 198, 173, 249, 209);
RT_INTERFACE!{static interface IDisplayInformationStatics(IDisplayInformationStaticsVtbl, IDisplayInformationStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayInformationStatics] {
    fn GetForCurrentView(&self, out: *mut <DisplayInformation as RtType>::Abi) -> HRESULT,
    fn get_AutoRotationPreferences(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn put_AutoRotationPreferences(&self, value: DisplayOrientations) -> HRESULT,
    fn add_DisplayContentsInvalidated(&self, handler: <foundation::TypedEventHandler<DisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayContentsInvalidated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayInformationStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<DisplayInformation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(DisplayInformation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_auto_rotation_preferences(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AutoRotationPreferences)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_auto_rotation_preferences(&self, value: DisplayOrientations) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AutoRotationPreferences)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_contents_invalidated(&self, handler: &foundation::TypedEventHandler<DisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DisplayContentsInvalidated)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_contents_invalidated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DisplayContentsInvalidated)(self.get_abi() as *const _ as *mut _, token);
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
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async() -> Result<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>> {
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
RT_DELEGATE!{delegate DisplayPropertiesEventHandler(DisplayPropertiesEventHandlerVtbl, DisplayPropertiesEventHandler_Abi, DisplayPropertiesEventHandlerImpl) [IID_DisplayPropertiesEventHandler] {
    fn Invoke(&self, sender: <IInspectable as RtType>::Abi) -> HRESULT
}}
impl DisplayPropertiesEventHandler {
    #[inline] pub fn invoke(&self, sender: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, sender.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IDisplayPropertiesStatics, 1765272973, 12522, 19949, 130, 113, 69, 83, 255, 2, 246, 138);
RT_INTERFACE!{static interface IDisplayPropertiesStatics(IDisplayPropertiesStaticsVtbl, IDisplayPropertiesStatics_Abi): IInspectable(IInspectableVtbl) [IID_IDisplayPropertiesStatics] {
    fn get_CurrentOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_NativeOrientation(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn get_AutoRotationPreferences(&self, out: *mut DisplayOrientations) -> HRESULT,
    fn put_AutoRotationPreferences(&self, value: DisplayOrientations) -> HRESULT,
    fn add_OrientationChanged(&self, handler: <DisplayPropertiesEventHandler as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OrientationChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_ResolutionScale(&self, out: *mut ResolutionScale) -> HRESULT,
    fn get_LogicalDpi(&self, out: *mut f32) -> HRESULT,
    fn add_LogicalDpiChanged(&self, handler: <DisplayPropertiesEventHandler as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_LogicalDpiChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn add_StereoEnabledChanged(&self, handler: <DisplayPropertiesEventHandler as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_StereoEnabledChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy13(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetColorProfileAsync(&self, out: *mut <foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    fn add_ColorProfileChanged(&self, handler: <DisplayPropertiesEventHandler as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_ColorProfileChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_DisplayContentsInvalidated(&self, handler: <DisplayPropertiesEventHandler as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayContentsInvalidated(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IDisplayPropertiesStatics {
    #[inline] pub fn get_current_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CurrentOrientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_native_orientation(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NativeOrientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_auto_rotation_preferences(&self) -> Result<DisplayOrientations> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AutoRotationPreferences)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_auto_rotation_preferences(&self, value: DisplayOrientations) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AutoRotationPreferences)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_orientation_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_OrientationChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_orientation_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_OrientationChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_scale(&self) -> Result<ResolutionScale> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ResolutionScale)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_logical_dpi(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LogicalDpi)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_logical_dpi_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_LogicalDpiChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_logical_dpi_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_LogicalDpiChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_StereoEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_stereo_enabled_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_StereoEnabledChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_stereo_enabled_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_StereoEnabledChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_color_profile_async(&self) -> Result<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetColorProfileAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_color_profile_changed(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_ColorProfileChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_color_profile_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_ColorProfileChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_display_contents_invalidated(&self, handler: &DisplayPropertiesEventHandler) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DisplayContentsInvalidated)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_contents_invalidated(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DisplayContentsInvalidated)(self.get_abi() as *const _ as *mut _, token);
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
RT_INTERFACE!{interface IHdmiDisplayInformation(IHdmiDisplayInformationVtbl, IHdmiDisplayInformation_Abi): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayInformation] {
    fn GetSupportedDisplayModes(&self, out: *mut <foundation::collections::IVectorView<HdmiDisplayMode> as RtType>::Abi) -> HRESULT,
    fn GetCurrentDisplayMode(&self, out: *mut <HdmiDisplayMode as RtType>::Abi) -> HRESULT,
    fn SetDefaultDisplayModeAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn RequestSetCurrentDisplayModeAsync(&self, mode: <HdmiDisplayMode as RtType>::Abi, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn RequestSetCurrentDisplayModeWithHdrAsync(&self, mode: <HdmiDisplayMode as RtType>::Abi, hdrOption: HdmiDisplayHdrOption, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&self, mode: <HdmiDisplayMode as RtType>::Abi, hdrOption: HdmiDisplayHdrOption, hdrMetadata: HdmiDisplayHdr2086Metadata, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn add_DisplayModesChanged(&self, value: <foundation::TypedEventHandler<HdmiDisplayInformation, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_DisplayModesChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IHdmiDisplayInformation {
    #[inline] pub fn get_supported_display_modes(&self) -> Result<Option<foundation::collections::IVectorView<HdmiDisplayMode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSupportedDisplayModes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_current_display_mode(&self) -> Result<Option<HdmiDisplayMode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCurrentDisplayMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HdmiDisplayMode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_default_display_mode_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetDefaultDisplayModeAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_async(&self, mode: &HdmiDisplayMode) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RequestSetCurrentDisplayModeAsync)(self.get_abi() as *const _ as *mut _, mode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_with_hdr_async(&self, mode: &HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RequestSetCurrentDisplayModeWithHdrAsync)(self.get_abi() as *const _ as *mut _, mode.get_abi() as *const _ as *mut _, hdrOption, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn request_set_current_display_mode_with_hdr_and_metadata_async(&self, mode: &HdmiDisplayMode, hdrOption: HdmiDisplayHdrOption, hdrMetadata: HdmiDisplayHdr2086Metadata) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RequestSetCurrentDisplayModeWithHdrAndMetadataAsync)(self.get_abi() as *const _ as *mut _, mode.get_abi() as *const _ as *mut _, hdrOption, hdrMetadata, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_display_modes_changed(&self, value: &foundation::TypedEventHandler<HdmiDisplayInformation, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_DisplayModesChanged)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_display_modes_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_DisplayModesChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HdmiDisplayInformation: IHdmiDisplayInformation}
impl RtActivatable<IHdmiDisplayInformationStatics> for HdmiDisplayInformation {}
impl HdmiDisplayInformation {
    #[inline] pub fn get_for_current_view() -> Result<Option<HdmiDisplayInformation>> {
        <Self as RtActivatable<IHdmiDisplayInformationStatics>>::get_activation_factory().get_for_current_view()
    }
}
DEFINE_CLSID!(HdmiDisplayInformation(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,68,105,115,112,108,97,121,46,67,111,114,101,46,72,100,109,105,68,105,115,112,108,97,121,73,110,102,111,114,109,97,116,105,111,110,0]) [CLSID_HdmiDisplayInformation]);
DEFINE_IID!(IID_IHdmiDisplayInformationStatics, 1827058272, 62506, 18965, 145, 76, 123, 142, 42, 90, 101, 223);
RT_INTERFACE!{static interface IHdmiDisplayInformationStatics(IHdmiDisplayInformationStaticsVtbl, IHdmiDisplayInformationStatics_Abi): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayInformationStatics] {
    fn GetForCurrentView(&self, out: *mut <HdmiDisplayInformation as RtType>::Abi) -> HRESULT
}}
impl IHdmiDisplayInformationStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<HdmiDisplayInformation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HdmiDisplayInformation::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHdmiDisplayMode, 201774509, 7056, 20305, 153, 129, 239, 90, 28, 13, 223, 102);
RT_INTERFACE!{interface IHdmiDisplayMode(IHdmiDisplayModeVtbl, IHdmiDisplayMode_Abi): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayMode] {
    fn get_ResolutionWidthInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_ResolutionHeightInRawPixels(&self, out: *mut u32) -> HRESULT,
    fn get_RefreshRate(&self, out: *mut f64) -> HRESULT,
    fn get_StereoEnabled(&self, out: *mut bool) -> HRESULT,
    fn get_BitsPerPixel(&self, out: *mut u16) -> HRESULT,
    fn IsEqual(&self, mode: <HdmiDisplayMode as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn get_ColorSpace(&self, out: *mut HdmiDisplayColorSpace) -> HRESULT,
    fn get_PixelEncoding(&self, out: *mut HdmiDisplayPixelEncoding) -> HRESULT,
    fn get_IsSdrLuminanceSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsSmpte2084Supported(&self, out: *mut bool) -> HRESULT,
    fn get_Is2086MetadataSupported(&self, out: *mut bool) -> HRESULT
}}
impl IHdmiDisplayMode {
    #[inline] pub fn get_resolution_width_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ResolutionWidthInRawPixels)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_resolution_height_in_raw_pixels(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ResolutionHeightInRawPixels)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_refresh_rate(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RefreshRate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_stereo_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_StereoEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bits_per_pixel(&self) -> Result<u16> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitsPerPixel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_equal(&self, mode: &HdmiDisplayMode) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsEqual)(self.get_abi() as *const _ as *mut _, mode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_color_space(&self) -> Result<HdmiDisplayColorSpace> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ColorSpace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_encoding(&self) -> Result<HdmiDisplayPixelEncoding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelEncoding)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_sdr_luminance_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsSdrLuminanceSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_smpte_2084_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsSmpte2084Supported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_2086_metadata_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Is2086MetadataSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HdmiDisplayMode: IHdmiDisplayMode}
DEFINE_IID!(IID_IHdmiDisplayMode2, 130895519, 19260, 17080, 132, 231, 137, 83, 104, 113, 138, 242);
RT_INTERFACE!{interface IHdmiDisplayMode2(IHdmiDisplayMode2Vtbl, IHdmiDisplayMode2_Abi): IInspectable(IInspectableVtbl) [IID_IHdmiDisplayMode2] {
    fn get_IsDolbyVisionLowLatencySupported(&self, out: *mut bool) -> HRESULT
}}
impl IHdmiDisplayMode2 {
    #[inline] pub fn get_is_dolby_vision_low_latency_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsDolbyVisionLowLatencySupported)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IGraphicsEffect(IGraphicsEffectVtbl, IGraphicsEffect_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsEffect] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, name: HSTRING) -> HRESULT
}}
impl IGraphicsEffect {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, name: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Name)(self.get_abi() as *const _ as *mut _, name.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IGraphicsEffectSource, 764386780, 17209, 20153, 146, 22, 249, 222, 183, 86, 88, 162);
RT_INTERFACE!{interface IGraphicsEffectSource(IGraphicsEffectSourceVtbl, IGraphicsEffectSource_Abi): IInspectable(IInspectableVtbl) [IID_IGraphicsEffectSource] {
    
}}
} // Windows.Graphics.Effects
pub mod holographic { // Windows.Graphics.Holographic
use crate::prelude::*;
RT_STRUCT! { struct HolographicAdapterId {
    LowPart: u32, HighPart: i32,
}}
DEFINE_IID!(IID_IHolographicCamera, 3840508997, 39917, 18816, 155, 160, 232, 118, 128, 209, 203, 116);
RT_INTERFACE!{interface IHolographicCamera(IHolographicCameraVtbl, IHolographicCamera_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCamera] {
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
        let hr = ((*self.get_abi().lpVtbl).get_RenderTargetSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_viewport_scale_factor(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ViewportScaleFactor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_viewport_scale_factor(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ViewportScaleFactor)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_stereo(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsStereo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Id)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_near_plane_distance(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetNearPlaneDistance)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_far_plane_distance(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetFarPlaneDistance)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCamera: IHolographicCamera}
DEFINE_IID!(IID_IHolographicCamera2, 3042680602, 47756, 20356, 173, 121, 46, 126, 30, 36, 80, 243);
RT_INTERFACE!{interface IHolographicCamera2(IHolographicCamera2Vtbl, IHolographicCamera2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCamera2] {
    fn get_LeftViewportParameters(&self, out: *mut <HolographicCameraViewportParameters as RtType>::Abi) -> HRESULT,
    fn get_RightViewportParameters(&self, out: *mut <HolographicCameraViewportParameters as RtType>::Abi) -> HRESULT,
    fn get_Display(&self, out: *mut <HolographicDisplay as RtType>::Abi) -> HRESULT
}}
impl IHolographicCamera2 {
    #[inline] pub fn get_left_viewport_parameters(&self) -> Result<Option<HolographicCameraViewportParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LeftViewportParameters)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCameraViewportParameters::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_right_viewport_parameters(&self) -> Result<Option<HolographicCameraViewportParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RightViewportParameters)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCameraViewportParameters::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display(&self) -> Result<Option<HolographicDisplay>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Display)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicDisplay::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera3, 1168789427, 31577, 21070, 74, 63, 74, 106, 214, 101, 4, 119);
RT_INTERFACE!{interface IHolographicCamera3(IHolographicCamera3Vtbl, IHolographicCamera3_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCamera3] {
    fn get_IsPrimaryLayerEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsPrimaryLayerEnabled(&self, value: bool) -> HRESULT,
    fn get_MaxQuadLayerCount(&self, out: *mut u32) -> HRESULT,
    fn get_QuadLayers(&self, out: *mut <foundation::collections::IVector<HolographicQuadLayer> as RtType>::Abi) -> HRESULT
}}
impl IHolographicCamera3 {
    #[inline] pub fn get_is_primary_layer_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsPrimaryLayerEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_primary_layer_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsPrimaryLayerEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_quad_layer_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxQuadLayerCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_quad_layers(&self) -> Result<Option<foundation::collections::IVector<HolographicQuadLayer>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_QuadLayers)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera4, 2586128854, 18211, 20281, 169, 165, 157, 5, 24, 29, 155, 68);
RT_INTERFACE!{interface IHolographicCamera4(IHolographicCamera4Vtbl, IHolographicCamera4_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCamera4] {
    fn get_CanOverrideViewport(&self, out: *mut bool) -> HRESULT
}}
impl IHolographicCamera4 {
    #[inline] pub fn get_can_override_viewport(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CanOverrideViewport)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCamera5, 580323058, 25229, 20213, 156, 8, 166, 63, 221, 119, 135, 198);
RT_INTERFACE!{interface IHolographicCamera5(IHolographicCamera5Vtbl, IHolographicCamera5_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCamera5] {
    fn get_IsHardwareContentProtectionSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsHardwareContentProtectionEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsHardwareContentProtectionEnabled(&self, value: bool) -> HRESULT
}}
impl IHolographicCamera5 {
    #[inline] pub fn get_is_hardware_content_protection_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsHardwareContentProtectionSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_hardware_content_protection_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsHardwareContentProtectionEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_hardware_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsHardwareContentProtectionEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraPose, 226328112, 4830, 17853, 145, 43, 199, 246, 86, 21, 153, 209);
RT_INTERFACE!{interface IHolographicCameraPose(IHolographicCameraPoseVtbl, IHolographicCameraPose_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraPose] {
    fn get_HolographicCamera(&self, out: *mut <HolographicCamera as RtType>::Abi) -> HRESULT,
    fn get_Viewport(&self, out: *mut foundation::Rect) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetViewTransform(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, out: *mut <foundation::IReference<HolographicStereoTransform> as RtType>::Abi) -> HRESULT,
    fn get_ProjectionTransform(&self, out: *mut HolographicStereoTransform) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetCullingFrustum(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, out: *mut <foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-perception")] fn TryGetVisibleFrustum(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, out: *mut <foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum> as RtType>::Abi) -> HRESULT,
    fn get_NearPlaneDistance(&self, out: *mut f64) -> HRESULT,
    fn get_FarPlaneDistance(&self, out: *mut f64) -> HRESULT
}}
impl IHolographicCameraPose {
    #[inline] pub fn get_holographic_camera(&self) -> Result<Option<HolographicCamera>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_HolographicCamera)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCamera::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_viewport(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Viewport)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_view_transform(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<foundation::IReference<HolographicStereoTransform>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetViewTransform)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_projection_transform(&self) -> Result<HolographicStereoTransform> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ProjectionTransform)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_culling_frustum(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetCullingFrustum)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn try_get_visible_frustum(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem) -> Result<Option<foundation::IReference<super::super::perception::spatial::SpatialBoundingFrustum>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryGetVisibleFrustum)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_near_plane_distance(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NearPlaneDistance)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_far_plane_distance(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_FarPlaneDistance)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraPose: IHolographicCameraPose}
DEFINE_IID!(IID_IHolographicCameraPose2, 590078067, 23853, 17760, 129, 78, 38, 151, 196, 252, 225, 107);
RT_INTERFACE!{interface IHolographicCameraPose2(IHolographicCameraPose2Vtbl, IHolographicCameraPose2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraPose2] {
    #[cfg(not(feature="windows-perception"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-perception")] fn OverrideViewTransform(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, coordinateSystemToViewTransform: HolographicStereoTransform) -> HRESULT,
    fn OverrideProjectionTransform(&self, projectionTransform: HolographicStereoTransform) -> HRESULT,
    fn OverrideViewport(&self, leftViewport: foundation::Rect, rightViewport: foundation::Rect) -> HRESULT
}}
impl IHolographicCameraPose2 {
    #[cfg(feature="windows-perception")] #[inline] pub fn override_view_transform(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, coordinateSystemToViewTransform: HolographicStereoTransform) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).OverrideViewTransform)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, coordinateSystemToViewTransform);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn override_projection_transform(&self, projectionTransform: HolographicStereoTransform) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).OverrideProjectionTransform)(self.get_abi() as *const _ as *mut _, projectionTransform);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn override_viewport(&self, leftViewport: foundation::Rect, rightViewport: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).OverrideViewport)(self.get_abi() as *const _ as *mut _, leftViewport, rightViewport);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters, 2393648849, 23540, 19990, 130, 54, 174, 8, 0, 193, 29, 13);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters(IHolographicCameraRenderingParametersVtbl, IHolographicCameraRenderingParameters_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters] {
    #[cfg(not(feature="windows-perception"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPoint(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPointWithNormal(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-perception")] fn SetFocusPointWithNormalLinearVelocity(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3, linearVelocity: foundation::numerics::Vector3) -> HRESULT,
    fn get_Direct3D11Device(&self, out: *mut <super::directx::direct3d11::IDirect3DDevice as RtType>::Abi) -> HRESULT,
    fn get_Direct3D11BackBuffer(&self, out: *mut <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi) -> HRESULT
}}
impl IHolographicCameraRenderingParameters {
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetFocusPoint)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point_with_normal(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetFocusPointWithNormal)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position, normal);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn set_focus_point_with_normal_linear_velocity(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, normal: foundation::numerics::Vector3, linearVelocity: foundation::numerics::Vector3) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetFocusPointWithNormalLinearVelocity)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position, normal, linearVelocity);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_direct3d11_device(&self) -> Result<Option<super::directx::direct3d11::IDirect3DDevice>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Direct3D11Device)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::directx::direct3d11::IDirect3DDevice::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_direct3d11_back_buffer(&self) -> Result<Option<super::directx::direct3d11::IDirect3DSurface>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Direct3D11BackBuffer)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::directx::direct3d11::IDirect3DSurface::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraRenderingParameters: IHolographicCameraRenderingParameters}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters2, 638742755, 46742, 17972, 148, 214, 190, 6, 129, 100, 53, 153);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters2(IHolographicCameraRenderingParameters2Vtbl, IHolographicCameraRenderingParameters2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters2] {
    fn get_ReprojectionMode(&self, out: *mut HolographicReprojectionMode) -> HRESULT,
    fn put_ReprojectionMode(&self, value: HolographicReprojectionMode) -> HRESULT,
    fn CommitDirect3D11DepthBuffer(&self, value: <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi) -> HRESULT
}}
impl IHolographicCameraRenderingParameters2 {
    #[inline] pub fn get_reprojection_mode(&self) -> Result<HolographicReprojectionMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ReprojectionMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_reprojection_mode(&self, value: HolographicReprojectionMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ReprojectionMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn commit_direct3d11_depth_buffer(&self, value: &super::directx::direct3d11::IDirect3DSurface) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CommitDirect3D11DepthBuffer)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraRenderingParameters3, 2980729151, 4973, 19206, 185, 212, 228, 185, 20, 205, 6, 131);
RT_INTERFACE!{interface IHolographicCameraRenderingParameters3(IHolographicCameraRenderingParameters3Vtbl, IHolographicCameraRenderingParameters3_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraRenderingParameters3] {
    fn get_IsContentProtectionEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_IsContentProtectionEnabled(&self, value: bool) -> HRESULT
}}
impl IHolographicCameraRenderingParameters3 {
    #[inline] pub fn get_is_content_protection_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsContentProtectionEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsContentProtectionEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicCameraViewportParameters, 2160980983, 33834, 16865, 147, 237, 86, 146, 171, 31, 187, 16);
RT_INTERFACE!{interface IHolographicCameraViewportParameters(IHolographicCameraViewportParametersVtbl, IHolographicCameraViewportParameters_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicCameraViewportParameters] {
    fn get_HiddenAreaMesh(&self, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector2) -> HRESULT,
    fn get_VisibleAreaMesh(&self, outSize: *mut u32, out: *mut *mut foundation::numerics::Vector2) -> HRESULT
}}
impl IHolographicCameraViewportParameters {
    #[inline] pub fn get_hidden_area_mesh(&self) -> Result<ComArray<foundation::numerics::Vector2>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_HiddenAreaMesh)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
    #[inline] pub fn get_visible_area_mesh(&self) -> Result<ComArray<foundation::numerics::Vector2>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_VisibleAreaMesh)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicCameraViewportParameters: IHolographicCameraViewportParameters}
DEFINE_IID!(IID_IHolographicDisplay, 2597233684, 7583, 16528, 163, 136, 144, 192, 111, 110, 174, 156);
RT_INTERFACE!{interface IHolographicDisplay(IHolographicDisplayVtbl, IHolographicDisplay_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicDisplay] {
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MaxViewportSize(&self, out: *mut foundation::Size) -> HRESULT,
    fn get_IsStereo(&self, out: *mut bool) -> HRESULT,
    fn get_IsOpaque(&self, out: *mut bool) -> HRESULT,
    fn get_AdapterId(&self, out: *mut HolographicAdapterId) -> HRESULT,
    #[cfg(feature="windows-perception")] fn get_SpatialLocator(&self, out: *mut <super::super::perception::spatial::SpatialLocator as RtType>::Abi) -> HRESULT
}}
impl IHolographicDisplay {
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_max_viewport_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxViewportSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_stereo(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsStereo)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_opaque(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsOpaque)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_adapter_id(&self) -> Result<HolographicAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AdapterId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn get_spatial_locator(&self) -> Result<Option<super::super::perception::spatial::SpatialLocator>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SpatialLocator)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::perception::spatial::SpatialLocator::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicDisplay: IHolographicDisplay}
impl RtActivatable<IHolographicDisplayStatics> for HolographicDisplay {}
impl HolographicDisplay {
    #[inline] pub fn get_default() -> Result<Option<HolographicDisplay>> {
        <Self as RtActivatable<IHolographicDisplayStatics>>::get_activation_factory().get_default()
    }
}
DEFINE_CLSID!(HolographicDisplay(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,72,111,108,111,103,114,97,112,104,105,99,46,72,111,108,111,103,114,97,112,104,105,99,68,105,115,112,108,97,121,0]) [CLSID_HolographicDisplay]);
DEFINE_IID!(IID_IHolographicDisplay2, 1974222722, 59221, 17260, 141, 150, 77, 50, 209, 49, 71, 62);
RT_INTERFACE!{interface IHolographicDisplay2(IHolographicDisplay2Vtbl, IHolographicDisplay2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicDisplay2] {
    fn get_RefreshRate(&self, out: *mut f64) -> HRESULT
}}
impl IHolographicDisplay2 {
    #[inline] pub fn get_refresh_rate(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RefreshRate)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicDisplayStatics, 3409398147, 59312, 18497, 131, 85, 58, 229, 181, 54, 233, 164);
RT_INTERFACE!{static interface IHolographicDisplayStatics(IHolographicDisplayStaticsVtbl, IHolographicDisplayStatics_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicDisplayStatics] {
    fn GetDefault(&self, out: *mut <HolographicDisplay as RtType>::Abi) -> HRESULT
}}
impl IHolographicDisplayStatics {
    #[inline] pub fn get_default(&self) -> Result<Option<HolographicDisplay>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDefault)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicDisplay::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicFrame, 3331886774, 43193, 12372, 166, 235, 214, 36, 182, 83, 99, 117);
RT_INTERFACE!{interface IHolographicFrame(IHolographicFrameVtbl, IHolographicFrame_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicFrame] {
    fn get_AddedCameras(&self, out: *mut <foundation::collections::IVectorView<HolographicCamera> as RtType>::Abi) -> HRESULT,
    fn get_RemovedCameras(&self, out: *mut <foundation::collections::IVectorView<HolographicCamera> as RtType>::Abi) -> HRESULT,
    fn GetRenderingParameters(&self, cameraPose: <HolographicCameraPose as RtType>::Abi, out: *mut <HolographicCameraRenderingParameters as RtType>::Abi) -> HRESULT,
    fn get_Duration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_CurrentPrediction(&self, out: *mut <HolographicFramePrediction as RtType>::Abi) -> HRESULT,
    fn UpdateCurrentPrediction(&self) -> HRESULT,
    fn PresentUsingCurrentPrediction(&self, out: *mut HolographicFramePresentResult) -> HRESULT,
    fn PresentUsingCurrentPredictionWithBehavior(&self, waitBehavior: HolographicFramePresentWaitBehavior, out: *mut HolographicFramePresentResult) -> HRESULT,
    fn WaitForFrameToFinish(&self) -> HRESULT
}}
impl IHolographicFrame {
    #[inline] pub fn get_added_cameras(&self) -> Result<Option<foundation::collections::IVectorView<HolographicCamera>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AddedCameras)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_removed_cameras(&self) -> Result<Option<foundation::collections::IVectorView<HolographicCamera>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RemovedCameras)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rendering_parameters(&self, cameraPose: &HolographicCameraPose) -> Result<Option<HolographicCameraRenderingParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetRenderingParameters)(self.get_abi() as *const _ as *mut _, cameraPose.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCameraRenderingParameters::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Duration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_current_prediction(&self) -> Result<Option<HolographicFramePrediction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CurrentPrediction)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicFramePrediction::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_current_prediction(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateCurrentPrediction)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn present_using_current_prediction(&self) -> Result<HolographicFramePresentResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).PresentUsingCurrentPrediction)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn present_using_current_prediction_with_behavior(&self, waitBehavior: HolographicFramePresentWaitBehavior) -> Result<HolographicFramePresentResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).PresentUsingCurrentPredictionWithBehavior)(self.get_abi() as *const _ as *mut _, waitBehavior, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_frame_to_finish(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WaitForFrameToFinish)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFrame: IHolographicFrame}
DEFINE_IID!(IID_IHolographicFrame2, 675231679, 15346, 24209, 102, 51, 135, 5, 116, 230, 242, 23);
RT_INTERFACE!{interface IHolographicFrame2(IHolographicFrame2Vtbl, IHolographicFrame2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicFrame2] {
    fn GetQuadLayerUpdateParameters(&self, layer: <HolographicQuadLayer as RtType>::Abi, out: *mut <HolographicQuadLayerUpdateParameters as RtType>::Abi) -> HRESULT
}}
impl IHolographicFrame2 {
    #[inline] pub fn get_quad_layer_update_parameters(&self, layer: &HolographicQuadLayer) -> Result<Option<HolographicQuadLayerUpdateParameters>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetQuadLayerUpdateParameters)(self.get_abi() as *const _ as *mut _, layer.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicQuadLayerUpdateParameters::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicFramePrediction, 1376734689, 23562, 20089, 168, 30, 106, 190, 2, 187, 39, 57);
RT_INTERFACE!{interface IHolographicFramePrediction(IHolographicFramePredictionVtbl, IHolographicFramePrediction_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicFramePrediction] {
    fn get_CameraPoses(&self, out: *mut <foundation::collections::IVectorView<HolographicCameraPose> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-perception")] fn get_Timestamp(&self, out: *mut <super::super::perception::PerceptionTimestamp as RtType>::Abi) -> HRESULT
}}
impl IHolographicFramePrediction {
    #[inline] pub fn get_camera_poses(&self) -> Result<Option<foundation::collections::IVectorView<HolographicCameraPose>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CameraPoses)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn get_timestamp(&self) -> Result<Option<super::super::perception::PerceptionTimestamp>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Timestamp)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::perception::PerceptionTimestamp::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFramePrediction: IHolographicFramePrediction}
DEFINE_IID!(IID_IHolographicFramePresentationMonitor, 3397854572, 28590, 17038, 187, 131, 37, 223, 238, 81, 19, 107);
RT_INTERFACE!{interface IHolographicFramePresentationMonitor(IHolographicFramePresentationMonitorVtbl, IHolographicFramePresentationMonitor_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicFramePresentationMonitor] {
    fn ReadReports(&self, out: *mut <foundation::collections::IVectorView<HolographicFramePresentationReport> as RtType>::Abi) -> HRESULT
}}
impl IHolographicFramePresentationMonitor {
    #[inline] pub fn read_reports(&self) -> Result<Option<foundation::collections::IVectorView<HolographicFramePresentationReport>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReadReports)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicFramePresentationMonitor: IHolographicFramePresentationMonitor}
DEFINE_IID!(IID_IHolographicFramePresentationReport, 2159736340, 62196, 19594, 141, 227, 6, 92, 120, 246, 213, 222);
RT_INTERFACE!{interface IHolographicFramePresentationReport(IHolographicFramePresentationReportVtbl, IHolographicFramePresentationReport_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicFramePresentationReport] {
    fn get_CompositorGpuDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_AppGpuDuration(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_AppGpuOverrun(&self, out: *mut foundation::TimeSpan) -> HRESULT,
    fn get_MissedPresentationOpportunityCount(&self, out: *mut u32) -> HRESULT,
    fn get_PresentationCount(&self, out: *mut u32) -> HRESULT
}}
impl IHolographicFramePresentationReport {
    #[inline] pub fn get_compositor_gpu_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CompositorGpuDuration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_gpu_duration(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AppGpuDuration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_app_gpu_overrun(&self) -> Result<foundation::TimeSpan> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AppGpuOverrun)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_missed_presentation_opportunity_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MissedPresentationOpportunityCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_presentation_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PresentationCount)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IHolographicQuadLayer(IHolographicQuadLayerVtbl, IHolographicQuadLayer_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayer] {
    fn get_PixelFormat(&self, out: *mut super::directx::DirectXPixelFormat) -> HRESULT,
    fn get_Size(&self, out: *mut foundation::Size) -> HRESULT
}}
impl IHolographicQuadLayer {
    #[inline] pub fn get_pixel_format(&self) -> Result<super::directx::DirectXPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Size)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicQuadLayer: IHolographicQuadLayer}
impl RtActivatable<IHolographicQuadLayerFactory> for HolographicQuadLayer {}
impl HolographicQuadLayer {
    #[inline] pub fn create(size: foundation::Size) -> Result<HolographicQuadLayer> {
        <Self as RtActivatable<IHolographicQuadLayerFactory>>::get_activation_factory().create(size)
    }
    #[inline] pub fn create_with_pixel_format(size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat) -> Result<HolographicQuadLayer> {
        <Self as RtActivatable<IHolographicQuadLayerFactory>>::get_activation_factory().create_with_pixel_format(size, pixelFormat)
    }
}
DEFINE_CLSID!(HolographicQuadLayer(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,72,111,108,111,103,114,97,112,104,105,99,46,72,111,108,111,103,114,97,112,104,105,99,81,117,97,100,76,97,121,101,114,0]) [CLSID_HolographicQuadLayer]);
DEFINE_IID!(IID_IHolographicQuadLayerFactory, 2792700147, 23060, 23056, 72, 154, 69, 80, 101, 179, 123, 118);
RT_INTERFACE!{static interface IHolographicQuadLayerFactory(IHolographicQuadLayerFactoryVtbl, IHolographicQuadLayerFactory_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerFactory] {
    fn Create(&self, size: foundation::Size, out: *mut <HolographicQuadLayer as RtType>::Abi) -> HRESULT,
    fn CreateWithPixelFormat(&self, size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat, out: *mut <HolographicQuadLayer as RtType>::Abi) -> HRESULT
}}
impl IHolographicQuadLayerFactory {
    #[inline] pub fn create(&self, size: foundation::Size) -> Result<HolographicQuadLayer> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, size, &mut out);
        if hr == S_OK { Ok(HolographicQuadLayer::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_pixel_format(&self, size: foundation::Size, pixelFormat: super::directx::DirectXPixelFormat) -> Result<HolographicQuadLayer> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithPixelFormat)(self.get_abi() as *const _ as *mut _, size, pixelFormat, &mut out);
        if hr == S_OK { Ok(HolographicQuadLayer::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicQuadLayerUpdateParameters, 722379696, 31117, 23498, 85, 194, 44, 12, 118, 46, 187, 8);
RT_INTERFACE!{interface IHolographicQuadLayerUpdateParameters(IHolographicQuadLayerUpdateParametersVtbl, IHolographicQuadLayerUpdateParameters_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerUpdateParameters] {
    fn AcquireBufferToUpdateContent(&self, out: *mut <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi) -> HRESULT,
    fn UpdateViewport(&self, value: foundation::Rect) -> HRESULT,
    fn UpdateContentProtectionEnabled(&self, value: bool) -> HRESULT,
    fn UpdateExtents(&self, value: foundation::numerics::Vector2) -> HRESULT,
    #[cfg(not(feature="windows-perception"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-perception")] fn UpdateLocationWithStationaryMode(&self, coordinateSystem: <super::super::perception::spatial::SpatialCoordinateSystem as RtType>::Abi, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> HRESULT,
    fn UpdateLocationWithDisplayRelativeMode(&self, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> HRESULT
}}
impl IHolographicQuadLayerUpdateParameters {
    #[inline] pub fn acquire_buffer_to_update_content(&self) -> Result<Option<super::directx::direct3d11::IDirect3DSurface>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AcquireBufferToUpdateContent)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::directx::direct3d11::IDirect3DSurface::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn update_viewport(&self, value: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateViewport)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_content_protection_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateContentProtectionEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_extents(&self, value: foundation::numerics::Vector2) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateExtents)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-perception")] #[inline] pub fn update_location_with_stationary_mode(&self, coordinateSystem: &super::super::perception::spatial::SpatialCoordinateSystem, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateLocationWithStationaryMode)(self.get_abi() as *const _ as *mut _, coordinateSystem.get_abi() as *const _ as *mut _, position, orientation);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn update_location_with_display_relative_mode(&self, position: foundation::numerics::Vector3, orientation: foundation::numerics::Quaternion) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).UpdateLocationWithDisplayRelativeMode)(self.get_abi() as *const _ as *mut _, position, orientation);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicQuadLayerUpdateParameters: IHolographicQuadLayerUpdateParameters}
DEFINE_IID!(IID_IHolographicQuadLayerUpdateParameters2, 1328796461, 33473, 18113, 137, 128, 60, 183, 13, 152, 24, 43);
RT_INTERFACE!{interface IHolographicQuadLayerUpdateParameters2(IHolographicQuadLayerUpdateParameters2Vtbl, IHolographicQuadLayerUpdateParameters2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicQuadLayerUpdateParameters2] {
    fn get_CanAcquireWithHardwareProtection(&self, out: *mut bool) -> HRESULT,
    fn AcquireBufferToUpdateContentWithHardwareProtection(&self, out: *mut <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi) -> HRESULT
}}
impl IHolographicQuadLayerUpdateParameters2 {
    #[inline] pub fn get_can_acquire_with_hardware_protection(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CanAcquireWithHardwareProtection)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn acquire_buffer_to_update_content_with_hardware_protection(&self) -> Result<Option<super::directx::direct3d11::IDirect3DSurface>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AcquireBufferToUpdateContentWithHardwareProtection)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::directx::direct3d11::IDirect3DSurface::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum HolographicReprojectionMode: i32 {
    PositionAndOrientation = 0, OrientationOnly = 1, Disabled = 2,
}}
DEFINE_IID!(IID_IHolographicSpace, 1132518310, 24184, 17231, 128, 124, 52, 51, 209, 239, 232, 183);
RT_INTERFACE!{interface IHolographicSpace(IHolographicSpaceVtbl, IHolographicSpace_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpace] {
    fn get_PrimaryAdapterId(&self, out: *mut HolographicAdapterId) -> HRESULT,
    fn SetDirect3D11Device(&self, value: <super::directx::direct3d11::IDirect3DDevice as RtType>::Abi) -> HRESULT,
    fn add_CameraAdded(&self, handler: <foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CameraAdded(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_CameraRemoved(&self, handler: <foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_CameraRemoved(&self, cookie: foundation::EventRegistrationToken) -> HRESULT,
    fn CreateNextFrame(&self, out: *mut <HolographicFrame as RtType>::Abi) -> HRESULT
}}
impl IHolographicSpace {
    #[inline] pub fn get_primary_adapter_id(&self) -> Result<HolographicAdapterId> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PrimaryAdapterId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_direct3d11_device(&self, value: &super::directx::direct3d11::IDirect3DDevice) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetDirect3D11Device)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_camera_added(&self, handler: &foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_CameraAdded)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_camera_added(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_CameraAdded)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_camera_removed(&self, handler: &foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_CameraRemoved)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_camera_removed(&self, cookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_CameraRemoved)(self.get_abi() as *const _ as *mut _, cookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_next_frame(&self) -> Result<Option<HolographicFrame>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateNextFrame)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicFrame::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpace: IHolographicSpace}
impl RtActivatable<IHolographicSpaceStatics> for HolographicSpace {}
impl RtActivatable<IHolographicSpaceStatics2> for HolographicSpace {}
impl RtActivatable<IHolographicSpaceStatics3> for HolographicSpace {}
impl HolographicSpace {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_for_core_window(window: &super::super::ui::core::CoreWindow) -> Result<Option<HolographicSpace>> {
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
RT_INTERFACE!{interface IHolographicSpace2(IHolographicSpace2Vtbl, IHolographicSpace2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpace2] {
    fn get_UserPresence(&self, out: *mut HolographicSpaceUserPresence) -> HRESULT,
    fn add_UserPresenceChanged(&self, handler: <foundation::TypedEventHandler<HolographicSpace, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_UserPresenceChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn WaitForNextFrameReady(&self) -> HRESULT,
    fn WaitForNextFrameReadyWithHeadStart(&self, requestedHeadStartDuration: foundation::TimeSpan) -> HRESULT,
    fn CreateFramePresentationMonitor(&self, maxQueuedReports: u32, out: *mut <HolographicFramePresentationMonitor as RtType>::Abi) -> HRESULT
}}
impl IHolographicSpace2 {
    #[inline] pub fn get_user_presence(&self) -> Result<HolographicSpaceUserPresence> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_UserPresence)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_user_presence_changed(&self, handler: &foundation::TypedEventHandler<HolographicSpace, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_UserPresenceChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_user_presence_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_UserPresenceChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_next_frame_ready(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WaitForNextFrameReady)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn wait_for_next_frame_ready_with_head_start(&self, requestedHeadStartDuration: foundation::TimeSpan) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).WaitForNextFrameReadyWithHeadStart)(self.get_abi() as *const _ as *mut _, requestedHeadStartDuration);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn create_frame_presentation_monitor(&self, maxQueuedReports: u32) -> Result<Option<HolographicFramePresentationMonitor>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateFramePresentationMonitor)(self.get_abi() as *const _ as *mut _, maxQueuedReports, &mut out);
        if hr == S_OK { Ok(HolographicFramePresentationMonitor::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceCameraAddedEventArgs, 1492245045, 48051, 15503, 153, 61, 108, 128, 231, 254, 185, 159);
RT_INTERFACE!{interface IHolographicSpaceCameraAddedEventArgs(IHolographicSpaceCameraAddedEventArgsVtbl, IHolographicSpaceCameraAddedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceCameraAddedEventArgs] {
    fn get_Camera(&self, out: *mut <HolographicCamera as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IHolographicSpaceCameraAddedEventArgs {
    #[inline] pub fn get_camera(&self) -> Result<Option<HolographicCamera>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Camera)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCamera::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpaceCameraAddedEventArgs: IHolographicSpaceCameraAddedEventArgs}
DEFINE_IID!(IID_IHolographicSpaceCameraRemovedEventArgs, 2153006248, 62126, 12846, 141, 169, 131, 106, 10, 149, 164, 193);
RT_INTERFACE!{interface IHolographicSpaceCameraRemovedEventArgs(IHolographicSpaceCameraRemovedEventArgsVtbl, IHolographicSpaceCameraRemovedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceCameraRemovedEventArgs] {
    fn get_Camera(&self, out: *mut <HolographicCamera as RtType>::Abi) -> HRESULT
}}
impl IHolographicSpaceCameraRemovedEventArgs {
    #[inline] pub fn get_camera(&self) -> Result<Option<HolographicCamera>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Camera)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicCamera::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class HolographicSpaceCameraRemovedEventArgs: IHolographicSpaceCameraRemovedEventArgs}
DEFINE_IID!(IID_IHolographicSpaceStatics, 911106148, 51442, 15265, 131, 145, 102, 184, 72, 158, 103, 253);
RT_INTERFACE!{static interface IHolographicSpaceStatics(IHolographicSpaceStaticsVtbl, IHolographicSpaceStatics_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics] {
    #[cfg(feature="windows-ui")] fn CreateForCoreWindow(&self, window: <super::super::ui::core::CoreWindow as RtType>::Abi, out: *mut <HolographicSpace as RtType>::Abi) -> HRESULT
}}
impl IHolographicSpaceStatics {
    #[cfg(feature="windows-ui")] #[inline] pub fn create_for_core_window(&self, window: &super::super::ui::core::CoreWindow) -> Result<Option<HolographicSpace>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateForCoreWindow)(self.get_abi() as *const _ as *mut _, window.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HolographicSpace::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceStatics2, 242708616, 30204, 18607, 135, 88, 6, 82, 246, 240, 124, 89);
RT_INTERFACE!{static interface IHolographicSpaceStatics2(IHolographicSpaceStatics2Vtbl, IHolographicSpaceStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics2] {
    fn get_IsSupported(&self, out: *mut bool) -> HRESULT,
    fn get_IsAvailable(&self, out: *mut bool) -> HRESULT,
    fn add_IsAvailableChanged(&self, handler: <foundation::EventHandler<IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_IsAvailableChanged(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IHolographicSpaceStatics2 {
    #[inline] pub fn get_is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_available(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsAvailable)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn add_is_available_changed(&self, handler: &foundation::EventHandler<IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_IsAvailableChanged)(self.get_abi() as *const _ as *mut _, handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_is_available_changed(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_IsAvailableChanged)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IHolographicSpaceStatics3, 989912637, 45475, 19966, 142, 121, 254, 197, 144, 158, 109, 248);
RT_INTERFACE!{static interface IHolographicSpaceStatics3(IHolographicSpaceStatics3Vtbl, IHolographicSpaceStatics3_Abi): IInspectable(IInspectableVtbl) [IID_IHolographicSpaceStatics3] {
    fn get_IsConfigured(&self, out: *mut bool) -> HRESULT
}}
impl IHolographicSpaceStatics3 {
    #[inline] pub fn get_is_configured(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsConfigured)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IBitmapBuffer(IBitmapBufferVtbl, IBitmapBuffer_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapBuffer] {
    fn GetPlaneCount(&self, out: *mut i32) -> HRESULT,
    fn GetPlaneDescription(&self, index: i32, out: *mut BitmapPlaneDescription) -> HRESULT
}}
impl IBitmapBuffer {
    #[inline] pub fn get_plane_count(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetPlaneCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_plane_description(&self, index: i32) -> Result<BitmapPlaneDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetPlaneDescription)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapBuffer: IBitmapBuffer}
RT_ENUM! { enum BitmapBufferAccessMode: i32 {
    Read = 0, ReadWrite = 1, Write = 2,
}}
DEFINE_IID!(IID_IBitmapCodecInformation, 1074572018, 50352, 17298, 163, 176, 111, 111, 155, 169, 92, 180);
RT_INTERFACE!{interface IBitmapCodecInformation(IBitmapCodecInformationVtbl, IBitmapCodecInformation_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapCodecInformation] {
    fn get_CodecId(&self, out: *mut Guid) -> HRESULT,
    fn get_FileExtensions(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT,
    fn get_FriendlyName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_MimeTypes(&self, out: *mut <foundation::collections::IVectorView<HString> as RtType>::Abi) -> HRESULT
}}
impl IBitmapCodecInformation {
    #[inline] pub fn get_codec_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CodecId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_file_extensions(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FileExtensions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_friendly_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FriendlyName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_mime_types(&self) -> Result<Option<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MimeTypes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapCodecInformation: IBitmapCodecInformation}
DEFINE_IID!(IID_IBitmapDecoder, 2901353146, 7540, 19601, 157, 252, 150, 32, 116, 82, 51, 230);
RT_INTERFACE!{interface IBitmapDecoder(IBitmapDecoderVtbl, IBitmapDecoder_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapDecoder] {
    fn get_BitmapContainerProperties(&self, out: *mut <BitmapPropertiesView as RtType>::Abi) -> HRESULT,
    fn get_DecoderInformation(&self, out: *mut <BitmapCodecInformation as RtType>::Abi) -> HRESULT,
    fn get_FrameCount(&self, out: *mut u32) -> HRESULT,
    fn GetPreviewAsync(&self, out: *mut <foundation::IAsyncOperation<ImageStream> as RtType>::Abi) -> HRESULT,
    fn GetFrameAsync(&self, frameIndex: u32, out: *mut <foundation::IAsyncOperation<BitmapFrame> as RtType>::Abi) -> HRESULT
}}
impl IBitmapDecoder {
    #[inline] pub fn get_bitmap_container_properties(&self) -> Result<Option<BitmapPropertiesView>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapContainerProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapPropertiesView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_decoder_information(&self) -> Result<Option<BitmapCodecInformation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DecoderInformation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapCodecInformation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_FrameCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_preview_async(&self) -> Result<foundation::IAsyncOperation<ImageStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPreviewAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_frame_async(&self, frameIndex: u32) -> Result<foundation::IAsyncOperation<BitmapFrame>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFrameAsync)(self.get_abi() as *const _ as *mut _, frameIndex, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
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
    #[inline] pub fn get_decoder_information_enumerator() -> Result<Option<foundation::collections::IVectorView<BitmapCodecInformation>>> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().get_decoder_information_enumerator()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapDecoder>> {
        <Self as RtActivatable<IBitmapDecoderStatics>>::get_activation_factory().create_async(stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_id_async(decoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapDecoder>> {
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
RT_INTERFACE!{static interface IBitmapDecoderStatics(IBitmapDecoderStaticsVtbl, IBitmapDecoderStatics_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapDecoderStatics] {
    fn get_BmpDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_PngDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_TiffDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_GifDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegXRDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_IcoDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn GetDecoderInformationEnumerator(&self, out: *mut <foundation::collections::IVectorView<BitmapCodecInformation> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateAsync(&self, stream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapDecoder> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn CreateWithIdAsync(&self, decoderId: Guid, stream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapDecoder> as RtType>::Abi) -> HRESULT
}}
impl IBitmapDecoderStatics {
    #[inline] pub fn get_bmp_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BmpDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_JpegDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_png_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PngDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tiff_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TiffDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_gif_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_GifDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_xr_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_JpegXRDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_ico_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IcoDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_decoder_information_enumerator(&self) -> Result<Option<foundation::collections::IVectorView<BitmapCodecInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDecoderInformationEnumerator)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(&self, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapDecoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateAsync)(self.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_id_async(&self, decoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapDecoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithIdAsync)(self.get_abi() as *const _ as *mut _, decoderId, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapDecoderStatics2, 1354393834, 39329, 16580, 128, 217, 174, 240, 218, 250, 108, 63);
RT_INTERFACE!{static interface IBitmapDecoderStatics2(IBitmapDecoderStatics2Vtbl, IBitmapDecoderStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapDecoderStatics2] {
    fn get_HeifDecoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_WebpDecoderId(&self, out: *mut Guid) -> HRESULT
}}
impl IBitmapDecoderStatics2 {
    #[inline] pub fn get_heif_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HeifDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_webp_decoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_WebpDecoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoder, 734292195, 57848, 19284, 149, 232, 50, 145, 149, 81, 206, 98);
RT_INTERFACE!{interface IBitmapEncoder(IBitmapEncoderVtbl, IBitmapEncoder_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapEncoder] {
    fn get_EncoderInformation(&self, out: *mut <BitmapCodecInformation as RtType>::Abi) -> HRESULT,
    fn get_BitmapProperties(&self, out: *mut <BitmapProperties as RtType>::Abi) -> HRESULT,
    fn get_BitmapContainerProperties(&self, out: *mut <BitmapProperties as RtType>::Abi) -> HRESULT,
    fn get_IsThumbnailGenerated(&self, out: *mut bool) -> HRESULT,
    fn put_IsThumbnailGenerated(&self, value: bool) -> HRESULT,
    fn get_GeneratedThumbnailWidth(&self, out: *mut u32) -> HRESULT,
    fn put_GeneratedThumbnailWidth(&self, value: u32) -> HRESULT,
    fn get_GeneratedThumbnailHeight(&self, out: *mut u32) -> HRESULT,
    fn put_GeneratedThumbnailHeight(&self, value: u32) -> HRESULT,
    fn get_BitmapTransform(&self, out: *mut <BitmapTransform as RtType>::Abi) -> HRESULT,
    fn SetPixelData(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, width: u32, height: u32, dpiX: f64, dpiY: f64, pixelsSize: u32, pixels: *mut u8) -> HRESULT,
    fn GoToNextFrameAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn GoToNextFrameWithEncodingOptionsAsync(&self, encodingOptions: <foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn FlushAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IBitmapEncoder {
    #[inline] pub fn get_encoder_information(&self) -> Result<Option<BitmapCodecInformation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_EncoderInformation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapCodecInformation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_properties(&self) -> Result<Option<BitmapProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_container_properties(&self) -> Result<Option<BitmapProperties>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapContainerProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapProperties::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_is_thumbnail_generated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsThumbnailGenerated)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_thumbnail_generated(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsThumbnailGenerated)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_generated_thumbnail_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_GeneratedThumbnailWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_generated_thumbnail_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_GeneratedThumbnailWidth)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_generated_thumbnail_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_GeneratedThumbnailHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_generated_thumbnail_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_GeneratedThumbnailHeight)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_transform(&self) -> Result<Option<BitmapTransform>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapTransform)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapTransform::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_pixel_data(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, width: u32, height: u32, dpiX: f64, dpiY: f64, pixels: &[u8]) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetPixelData)(self.get_abi() as *const _ as *mut _, pixelFormat, alphaMode, width, height, dpiX, dpiY, pixels.len() as u32, pixels.as_ptr() as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn go_to_next_frame_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GoToNextFrameAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn go_to_next_frame_with_encoding_options_async(&self, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GoToNextFrameWithEncodingOptionsAsync)(self.get_abi() as *const _ as *mut _, encodingOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn flush_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FlushAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
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
    #[inline] pub fn get_encoder_information_enumerator() -> Result<Option<foundation::collections::IVectorView<BitmapCodecInformation>>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().get_encoder_information_enumerator()
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapEncoder>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_async(encoderId, stream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_encoding_options_async(encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<foundation::IAsyncOperation<BitmapEncoder>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_with_encoding_options_async(encoderId, stream, encodingOptions)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_transcoding_async(stream: &super::super::storage::streams::IRandomAccessStream, bitmapDecoder: &BitmapDecoder) -> Result<foundation::IAsyncOperation<BitmapEncoder>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_for_transcoding_async(stream, bitmapDecoder)
    }
    #[inline] pub fn create_for_in_place_property_encoding_async(bitmapDecoder: &BitmapDecoder) -> Result<foundation::IAsyncOperation<BitmapEncoder>> {
        <Self as RtActivatable<IBitmapEncoderStatics>>::get_activation_factory().create_for_in_place_property_encoding_async(bitmapDecoder)
    }
    #[inline] pub fn get_heif_encoder_id() -> Result<Guid> {
        <Self as RtActivatable<IBitmapEncoderStatics2>>::get_activation_factory().get_heif_encoder_id()
    }
}
DEFINE_CLSID!(BitmapEncoder(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,69,110,99,111,100,101,114,0]) [CLSID_BitmapEncoder]);
DEFINE_IID!(IID_IBitmapEncoderStatics, 2806208167, 42212, 20153, 142, 64, 86, 77, 231, 225, 204, 178);
RT_INTERFACE!{static interface IBitmapEncoderStatics(IBitmapEncoderStaticsVtbl, IBitmapEncoderStatics_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderStatics] {
    fn get_BmpEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_PngEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_TiffEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_GifEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn get_JpegXREncoderId(&self, out: *mut Guid) -> HRESULT,
    fn GetEncoderInformationEnumerator(&self, out: *mut <foundation::collections::IVectorView<BitmapCodecInformation> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateAsync(&self, encoderId: Guid, stream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapEncoder> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateWithEncodingOptionsAsync(&self, encoderId: Guid, stream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, encodingOptions: <foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapEncoder> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy9(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateForTranscodingAsync(&self, stream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, bitmapDecoder: <BitmapDecoder as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapEncoder> as RtType>::Abi) -> HRESULT,
    fn CreateForInPlacePropertyEncodingAsync(&self, bitmapDecoder: <BitmapDecoder as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapEncoder> as RtType>::Abi) -> HRESULT
}}
impl IBitmapEncoderStatics {
    #[inline] pub fn get_bmp_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BmpEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_JpegEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_png_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PngEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_tiff_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TiffEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_gif_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_GifEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_jpeg_xr_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_JpegXREncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_encoder_information_enumerator(&self) -> Result<Option<foundation::collections::IVectorView<BitmapCodecInformation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetEncoderInformationEnumerator)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_async(&self, encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<BitmapEncoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateAsync)(self.get_abi() as *const _ as *mut _, encoderId, stream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_with_encoding_options_async(&self, encoderId: Guid, stream: &super::super::storage::streams::IRandomAccessStream, encodingOptions: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<foundation::IAsyncOperation<BitmapEncoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithEncodingOptionsAsync)(self.get_abi() as *const _ as *mut _, encoderId, stream.get_abi() as *const _ as *mut _, encodingOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_for_transcoding_async(&self, stream: &super::super::storage::streams::IRandomAccessStream, bitmapDecoder: &BitmapDecoder) -> Result<foundation::IAsyncOperation<BitmapEncoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateForTranscodingAsync)(self.get_abi() as *const _ as *mut _, stream.get_abi() as *const _ as *mut _, bitmapDecoder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_for_in_place_property_encoding_async(&self, bitmapDecoder: &BitmapDecoder) -> Result<foundation::IAsyncOperation<BitmapEncoder>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateForInPlacePropertyEncodingAsync)(self.get_abi() as *const _ as *mut _, bitmapDecoder.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoderStatics2, 868991577, 65073, 16817, 184, 18, 8, 109, 33, 232, 126, 22);
RT_INTERFACE!{static interface IBitmapEncoderStatics2(IBitmapEncoderStatics2Vtbl, IBitmapEncoderStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderStatics2] {
    fn get_HeifEncoderId(&self, out: *mut Guid) -> HRESULT
}}
impl IBitmapEncoderStatics2 {
    #[inline] pub fn get_heif_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HeifEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IBitmapEncoderWithSoftwareBitmap, 1751962177, 17200, 19575, 172, 228, 3, 52, 150, 139, 23, 104);
RT_INTERFACE!{interface IBitmapEncoderWithSoftwareBitmap(IBitmapEncoderWithSoftwareBitmapVtbl, IBitmapEncoderWithSoftwareBitmap_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapEncoderWithSoftwareBitmap] {
    fn SetSoftwareBitmap(&self, bitmap: <SoftwareBitmap as RtType>::Abi) -> HRESULT
}}
impl IBitmapEncoderWithSoftwareBitmap {
    #[inline] pub fn set_software_bitmap(&self, bitmap: &SoftwareBitmap) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetSoftwareBitmap)(self.get_abi() as *const _ as *mut _, bitmap.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_ENUM! { enum BitmapFlip: i32 {
    None = 0, Horizontal = 1, Vertical = 2,
}}
DEFINE_IID!(IID_IBitmapFrame, 1923389980, 32897, 17293, 145, 188, 148, 236, 252, 129, 133, 198);
RT_INTERFACE!{interface IBitmapFrame(IBitmapFrameVtbl, IBitmapFrame_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapFrame] {
    fn GetThumbnailAsync(&self, out: *mut <foundation::IAsyncOperation<ImageStream> as RtType>::Abi) -> HRESULT,
    fn get_BitmapProperties(&self, out: *mut <BitmapPropertiesView as RtType>::Abi) -> HRESULT,
    fn get_BitmapPixelFormat(&self, out: *mut BitmapPixelFormat) -> HRESULT,
    fn get_BitmapAlphaMode(&self, out: *mut BitmapAlphaMode) -> HRESULT,
    fn get_DpiX(&self, out: *mut f64) -> HRESULT,
    fn get_DpiY(&self, out: *mut f64) -> HRESULT,
    fn get_PixelWidth(&self, out: *mut u32) -> HRESULT,
    fn get_PixelHeight(&self, out: *mut u32) -> HRESULT,
    fn get_OrientedPixelWidth(&self, out: *mut u32) -> HRESULT,
    fn get_OrientedPixelHeight(&self, out: *mut u32) -> HRESULT,
    fn GetPixelDataAsync(&self, out: *mut <foundation::IAsyncOperation<PixelDataProvider> as RtType>::Abi) -> HRESULT,
    fn GetPixelDataTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: <BitmapTransform as RtType>::Abi, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode, out: *mut <foundation::IAsyncOperation<PixelDataProvider> as RtType>::Abi) -> HRESULT
}}
impl IBitmapFrame {
    #[inline] pub fn get_thumbnail_async(&self) -> Result<foundation::IAsyncOperation<ImageStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetThumbnailAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_properties(&self) -> Result<Option<BitmapPropertiesView>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(BitmapPropertiesView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapPixelFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapAlphaMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_oriented_pixel_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_OrientedPixelWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_oriented_pixel_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_OrientedPixelHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_data_async(&self) -> Result<foundation::IAsyncOperation<PixelDataProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPixelDataAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_data_transformed_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: &BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> Result<foundation::IAsyncOperation<PixelDataProvider>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPixelDataTransformedAsync)(self.get_abi() as *const _ as *mut _, pixelFormat, alphaMode, transform.get_abi() as *const _ as *mut _, exifOrientationMode, colorManagementMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapFrame: IBitmapFrame}
DEFINE_IID!(IID_IBitmapFrameWithSoftwareBitmap, 4264066202, 16908, 18787, 135, 173, 105, 20, 54, 224, 131, 131);
RT_INTERFACE!{interface IBitmapFrameWithSoftwareBitmap(IBitmapFrameWithSoftwareBitmapVtbl, IBitmapFrameWithSoftwareBitmap_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapFrameWithSoftwareBitmap] {
    fn GetSoftwareBitmapAsync(&self, out: *mut <foundation::IAsyncOperation<SoftwareBitmap> as RtType>::Abi) -> HRESULT,
    fn GetSoftwareBitmapConvertedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, out: *mut <foundation::IAsyncOperation<SoftwareBitmap> as RtType>::Abi) -> HRESULT,
    fn GetSoftwareBitmapTransformedAsync(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: <BitmapTransform as RtType>::Abi, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode, out: *mut <foundation::IAsyncOperation<SoftwareBitmap> as RtType>::Abi) -> HRESULT
}}
impl IBitmapFrameWithSoftwareBitmap {
    #[inline] pub fn get_software_bitmap_async(&self) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSoftwareBitmapAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_software_bitmap_converted_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSoftwareBitmapConvertedAsync)(self.get_abi() as *const _ as *mut _, pixelFormat, alphaMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_software_bitmap_transformed_async(&self, pixelFormat: BitmapPixelFormat, alphaMode: BitmapAlphaMode, transform: &BitmapTransform, exifOrientationMode: ExifOrientationMode, colorManagementMode: ColorManagementMode) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSoftwareBitmapTransformedAsync)(self.get_abi() as *const _ as *mut _, pixelFormat, alphaMode, transform.get_abi() as *const _ as *mut _, exifOrientationMode, colorManagementMode, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
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
RT_INTERFACE!{interface IBitmapProperties(IBitmapPropertiesVtbl, IBitmapProperties_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapProperties] {
    fn SetPropertiesAsync(&self, propertiesToSet: <foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>> as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IBitmapProperties {
    #[inline] pub fn set_properties_async(&self, propertiesToSet: &foundation::collections::IIterable<foundation::collections::IKeyValuePair<HString, BitmapTypedValue>>) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetPropertiesAsync)(self.get_abi() as *const _ as *mut _, propertiesToSet.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapProperties: IBitmapProperties}
DEFINE_IID!(IID_IBitmapPropertiesView, 2114971770, 14960, 18680, 156, 85, 25, 108, 245, 165, 69, 245);
RT_INTERFACE!{interface IBitmapPropertiesView(IBitmapPropertiesViewVtbl, IBitmapPropertiesView_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapPropertiesView] {
    fn GetPropertiesAsync(&self, propertiesToRetrieve: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<BitmapPropertySet> as RtType>::Abi) -> HRESULT
}}
impl IBitmapPropertiesView {
    #[inline] pub fn get_properties_async(&self, propertiesToRetrieve: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<BitmapPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPropertiesAsync)(self.get_abi() as *const _ as *mut _, propertiesToRetrieve.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
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
RT_INTERFACE!{interface IBitmapTransform(IBitmapTransformVtbl, IBitmapTransform_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapTransform] {
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
        let hr = ((*self.get_abi().lpVtbl).get_ScaledWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_scaled_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ScaledWidth)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_scaled_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ScaledHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_scaled_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ScaledHeight)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_interpolation_mode(&self) -> Result<BitmapInterpolationMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_InterpolationMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_interpolation_mode(&self, value: BitmapInterpolationMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_InterpolationMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_flip(&self) -> Result<BitmapFlip> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Flip)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_flip(&self, value: BitmapFlip) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Flip)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_rotation(&self) -> Result<BitmapRotation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Rotation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_rotation(&self, value: BitmapRotation) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Rotation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bounds(&self) -> Result<BitmapBounds> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Bounds)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bounds(&self, value: BitmapBounds) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Bounds)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapTransform: IBitmapTransform}
impl RtActivatable<IActivationFactory> for BitmapTransform {}
DEFINE_CLSID!(BitmapTransform(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,84,114,97,110,115,102,111,114,109,0]) [CLSID_BitmapTransform]);
DEFINE_IID!(IID_IBitmapTypedValue, 3447735465, 9283, 16384, 176, 205, 121, 49, 108, 86, 245, 137);
RT_INTERFACE!{interface IBitmapTypedValue(IBitmapTypedValueVtbl, IBitmapTypedValue_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapTypedValue] {
    fn get_Value(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_Type(&self, out: *mut foundation::PropertyType) -> HRESULT
}}
impl IBitmapTypedValue {
    #[inline] pub fn get_value(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<foundation::PropertyType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Type)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class BitmapTypedValue: IBitmapTypedValue}
impl RtActivatable<IBitmapTypedValueFactory> for BitmapTypedValue {}
impl BitmapTypedValue {
    #[inline] pub fn create(value: &IInspectable, type_: foundation::PropertyType) -> Result<BitmapTypedValue> {
        <Self as RtActivatable<IBitmapTypedValueFactory>>::get_activation_factory().create(value, type_)
    }
}
DEFINE_CLSID!(BitmapTypedValue(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,66,105,116,109,97,112,84,121,112,101,100,86,97,108,117,101,0]) [CLSID_BitmapTypedValue]);
DEFINE_IID!(IID_IBitmapTypedValueFactory, 2463872409, 52755, 18107, 149, 69, 203, 58, 63, 99, 235, 139);
RT_INTERFACE!{static interface IBitmapTypedValueFactory(IBitmapTypedValueFactoryVtbl, IBitmapTypedValueFactory_Abi): IInspectable(IInspectableVtbl) [IID_IBitmapTypedValueFactory] {
    fn Create(&self, value: <IInspectable as RtType>::Abi, type_: foundation::PropertyType, out: *mut <BitmapTypedValue as RtType>::Abi) -> HRESULT
}}
impl IBitmapTypedValueFactory {
    #[inline] pub fn create(&self, value: &IInspectable, type_: foundation::PropertyType) -> Result<BitmapTypedValue> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, type_, &mut out);
        if hr == S_OK { Ok(BitmapTypedValue::wrap_nonnull(out)) } else { err(hr) }
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
RT_INTERFACE!{interface IPixelDataProvider(IPixelDataProviderVtbl, IPixelDataProvider_Abi): IInspectable(IInspectableVtbl) [IID_IPixelDataProvider] {
    fn DetachPixelData(&self, outSize: *mut u32, out: *mut *mut u8) -> HRESULT
}}
impl IPixelDataProvider {
    #[inline] pub fn detach_pixel_data(&self) -> Result<ComArray<u8>> { unsafe { 
        let mut outSize = 0; let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).DetachPixelData)(self.get_abi() as *const _ as *mut _, &mut outSize, &mut out);
        if hr == S_OK { Ok(ComArray::from_raw(outSize, out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PixelDataProvider: IPixelDataProvider}
RT_ENUM! { enum PngFilterMode: i32 {
    Automatic = 0, None = 1, Sub = 2, Up = 3, Average = 4, Paeth = 5, Adaptive = 6,
}}
DEFINE_IID!(IID_ISoftwareBitmap, 1755186952, 32495, 18495, 150, 63, 218, 147, 136, 24, 224, 115);
RT_INTERFACE!{interface ISoftwareBitmap(ISoftwareBitmapVtbl, ISoftwareBitmap_Abi): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmap] {
    fn get_BitmapPixelFormat(&self, out: *mut BitmapPixelFormat) -> HRESULT,
    fn get_BitmapAlphaMode(&self, out: *mut BitmapAlphaMode) -> HRESULT,
    fn get_PixelWidth(&self, out: *mut i32) -> HRESULT,
    fn get_PixelHeight(&self, out: *mut i32) -> HRESULT,
    fn get_IsReadOnly(&self, out: *mut bool) -> HRESULT,
    fn put_DpiX(&self, value: f64) -> HRESULT,
    fn get_DpiX(&self, out: *mut f64) -> HRESULT,
    fn put_DpiY(&self, value: f64) -> HRESULT,
    fn get_DpiY(&self, out: *mut f64) -> HRESULT,
    fn LockBuffer(&self, mode: BitmapBufferAccessMode, out: *mut <BitmapBuffer as RtType>::Abi) -> HRESULT,
    fn CopyTo(&self, bitmap: <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy11(&self) -> (),
    #[cfg(feature="windows-storage")] fn CopyFromBuffer(&self, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy12(&self) -> (),
    #[cfg(feature="windows-storage")] fn CopyToBuffer(&self, buffer: <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn GetReadOnlyView(&self, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT
}}
impl ISoftwareBitmap {
    #[inline] pub fn get_bitmap_pixel_format(&self) -> Result<BitmapPixelFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapPixelFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_alpha_mode(&self) -> Result<BitmapAlphaMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapAlphaMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_width(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_pixel_height(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PixelHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_read_only(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsReadOnly)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_x(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DpiX)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_y(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DpiY)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn lock_buffer(&self, mode: BitmapBufferAccessMode) -> Result<Option<BitmapBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LockBuffer)(self.get_abi() as *const _ as *mut _, mode, &mut out);
        if hr == S_OK { Ok(BitmapBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn copy_to(&self, bitmap: &SoftwareBitmap) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CopyTo)(self.get_abi() as *const _ as *mut _, bitmap.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_from_buffer(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CopyFromBuffer)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn copy_to_buffer(&self, buffer: &super::super::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CopyToBuffer)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_read_only_view(&self) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetReadOnlyView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SoftwareBitmap: ISoftwareBitmap}
impl RtActivatable<ISoftwareBitmapFactory> for SoftwareBitmap {}
impl RtActivatable<ISoftwareBitmapStatics> for SoftwareBitmap {}
impl SoftwareBitmap {
    #[inline] pub fn create(format: BitmapPixelFormat, width: i32, height: i32) -> Result<SoftwareBitmap> {
        <Self as RtActivatable<ISoftwareBitmapFactory>>::get_activation_factory().create(format, width, height)
    }
    #[inline] pub fn create_with_alpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<SoftwareBitmap> {
        <Self as RtActivatable<ISoftwareBitmapFactory>>::get_activation_factory().create_with_alpha(format, width, height, alpha)
    }
    #[inline] pub fn copy(source: &SoftwareBitmap) -> Result<Option<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().copy(source)
    }
    #[inline] pub fn convert(source: &SoftwareBitmap, format: BitmapPixelFormat) -> Result<Option<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().convert(source, format)
    }
    #[inline] pub fn convert_with_alpha(source: &SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> Result<Option<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().convert_with_alpha(source, format, alpha)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_from_buffer(source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32) -> Result<Option<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_from_buffer(source, format, width, height)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_with_alpha_from_buffer(source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<Option<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_with_alpha_from_buffer(source, format, width, height, alpha)
    }
    #[inline] pub fn create_copy_from_surface_async(surface: &super::directx::direct3d11::IDirect3DSurface) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_from_surface_async(surface)
    }
    #[inline] pub fn create_copy_with_alpha_from_surface_async(surface: &super::directx::direct3d11::IDirect3DSurface, alpha: BitmapAlphaMode) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> {
        <Self as RtActivatable<ISoftwareBitmapStatics>>::get_activation_factory().create_copy_with_alpha_from_surface_async(surface, alpha)
    }
}
DEFINE_CLSID!(SoftwareBitmap(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,73,109,97,103,105,110,103,46,83,111,102,116,119,97,114,101,66,105,116,109,97,112,0]) [CLSID_SoftwareBitmap]);
DEFINE_IID!(IID_ISoftwareBitmapFactory, 3382700905, 11618, 19783, 166, 179, 79, 219, 106, 7, 253, 248);
RT_INTERFACE!{static interface ISoftwareBitmapFactory(ISoftwareBitmapFactoryVtbl, ISoftwareBitmapFactory_Abi): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmapFactory] {
    fn Create(&self, format: BitmapPixelFormat, width: i32, height: i32, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    fn CreateWithAlpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT
}}
impl ISoftwareBitmapFactory {
    #[inline] pub fn create(&self, format: BitmapPixelFormat, width: i32, height: i32) -> Result<SoftwareBitmap> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, format, width, height, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_alpha(&self, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<SoftwareBitmap> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithAlpha)(self.get_abi() as *const _ as *mut _, format, width, height, alpha, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISoftwareBitmapStatics, 3741550043, 26415, 19101, 128, 110, 194, 68, 47, 52, 62, 134);
RT_INTERFACE!{static interface ISoftwareBitmapStatics(ISoftwareBitmapStaticsVtbl, ISoftwareBitmapStatics_Abi): IInspectable(IInspectableVtbl) [IID_ISoftwareBitmapStatics] {
    fn Copy(&self, source: <SoftwareBitmap as RtType>::Abi, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    fn Convert(&self, source: <SoftwareBitmap as RtType>::Abi, format: BitmapPixelFormat, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    fn ConvertWithAlpha(&self, source: <SoftwareBitmap as RtType>::Abi, format: BitmapPixelFormat, alpha: BitmapAlphaMode, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateCopyFromBuffer(&self, source: <super::super::storage::streams::IBuffer as RtType>::Abi, format: BitmapPixelFormat, width: i32, height: i32, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn CreateCopyWithAlphaFromBuffer(&self, source: <super::super::storage::streams::IBuffer as RtType>::Abi, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, out: *mut <SoftwareBitmap as RtType>::Abi) -> HRESULT,
    fn CreateCopyFromSurfaceAsync(&self, surface: <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi, out: *mut <foundation::IAsyncOperation<SoftwareBitmap> as RtType>::Abi) -> HRESULT,
    fn CreateCopyWithAlphaFromSurfaceAsync(&self, surface: <super::directx::direct3d11::IDirect3DSurface as RtType>::Abi, alpha: BitmapAlphaMode, out: *mut <foundation::IAsyncOperation<SoftwareBitmap> as RtType>::Abi) -> HRESULT
}}
impl ISoftwareBitmapStatics {
    #[inline] pub fn copy(&self, source: &SoftwareBitmap) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Copy)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn convert(&self, source: &SoftwareBitmap, format: BitmapPixelFormat) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Convert)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, format, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn convert_with_alpha(&self, source: &SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ConvertWithAlpha)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, format, alpha, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_from_buffer(&self, source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCopyFromBuffer)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, format, width, height, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn create_copy_with_alpha_from_buffer(&self, source: &super::super::storage::streams::IBuffer, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> Result<Option<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCopyWithAlphaFromBuffer)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _, format, width, height, alpha, &mut out);
        if hr == S_OK { Ok(SoftwareBitmap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_copy_from_surface_async(&self, surface: &super::directx::direct3d11::IDirect3DSurface) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCopyFromSurfaceAsync)(self.get_abi() as *const _ as *mut _, surface.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_copy_with_alpha_from_surface_async(&self, surface: &super::directx::direct3d11::IDirect3DSurface, alpha: BitmapAlphaMode) -> Result<foundation::IAsyncOperation<SoftwareBitmap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCopyWithAlphaFromSurfaceAsync)(self.get_abi() as *const _ as *mut _, surface.get_abi() as *const _ as *mut _, alpha, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
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
RT_INTERFACE!{interface IPrintDocumentSource(IPrintDocumentSourceVtbl, IPrintDocumentSource_Abi): IInspectable(IInspectableVtbl) [IID_IPrintDocumentSource] {
    
}}
RT_ENUM! { enum PrintDuplex: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, OneSided = 3, TwoSidedShortEdge = 4, TwoSidedLongEdge = 5,
}}
RT_ENUM! { enum PrintHolePunch: i32 {
    Default = 0, NotAvailable = 1, PrinterCustom = 2, None = 3, LeftEdge = 4, RightEdge = 5, TopEdge = 6, BottomEdge = 7,
}}
DEFINE_IID!(IID_IPrintManager, 4280981140, 35993, 17661, 174, 74, 25, 217, 170, 154, 15, 10);
RT_INTERFACE!{interface IPrintManager(IPrintManagerVtbl, IPrintManager_Abi): IInspectable(IInspectableVtbl) [IID_IPrintManager] {
    fn add_PrintTaskRequested(&self, eventHandler: <foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_PrintTaskRequested(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintManager {
    #[inline] pub fn add_print_task_requested(&self, eventHandler: &foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_PrintTaskRequested)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_print_task_requested(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_PrintTaskRequested)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintManager: IPrintManager}
impl RtActivatable<IPrintManagerStatic> for PrintManager {}
impl RtActivatable<IPrintManagerStatic2> for PrintManager {}
impl PrintManager {
    #[inline] pub fn get_for_current_view() -> Result<Option<PrintManager>> {
        <Self as RtActivatable<IPrintManagerStatic>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn show_print_ui_async() -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IPrintManagerStatic>>::get_activation_factory().show_print_ui_async()
    }
    #[inline] pub fn is_supported() -> Result<bool> {
        <Self as RtActivatable<IPrintManagerStatic2>>::get_activation_factory().is_supported()
    }
}
DEFINE_CLSID!(PrintManager(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,77,97,110,97,103,101,114,0]) [CLSID_PrintManager]);
DEFINE_IID!(IID_IPrintManagerStatic, 1477991885, 58932, 18004, 132, 240, 224, 21, 42, 130, 23, 172);
RT_INTERFACE!{static interface IPrintManagerStatic(IPrintManagerStaticVtbl, IPrintManagerStatic_Abi): IInspectable(IInspectableVtbl) [IID_IPrintManagerStatic] {
    fn GetForCurrentView(&self, out: *mut <PrintManager as RtType>::Abi) -> HRESULT,
    fn ShowPrintUIAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IPrintManagerStatic {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<PrintManager>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintManager::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_print_ui_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ShowPrintUIAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintManagerStatic2, 900307285, 59051, 16697, 154, 189, 184, 106, 114, 155, 53, 152);
RT_INTERFACE!{static interface IPrintManagerStatic2(IPrintManagerStatic2Vtbl, IPrintManagerStatic2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintManagerStatic2] {
    fn IsSupported(&self, out: *mut bool) -> HRESULT
}}
impl IPrintManagerStatic2 {
    #[inline] pub fn is_supported(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsSupported)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IPrintPageInfo(IPrintPageInfoVtbl, IPrintPageInfo_Abi): IInspectable(IInspectableVtbl) [IID_IPrintPageInfo] {
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
        let hr = ((*self.get_abi().lpVtbl).put_MediaSize)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_size(&self) -> Result<PrintMediaSize> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MediaSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_page_size(&self, value: foundation::Size) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_PageSize)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_page_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PageSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_x(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DpiX)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_x(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiX)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_dpi_y(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DpiY)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_dpi_y(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DpiY)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_orientation(&self, value: PrintOrientation) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Orientation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<PrintOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageInfo: IPrintPageInfo}
impl RtActivatable<IActivationFactory> for PrintPageInfo {}
DEFINE_CLSID!(PrintPageInfo(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,80,97,103,101,73,110,102,111,0]) [CLSID_PrintPageInfo]);
DEFINE_IID!(IID_IPrintPageRange, 4171263060, 28284, 20933, 87, 253, 6, 96, 194, 215, 21, 19);
RT_INTERFACE!{interface IPrintPageRange(IPrintPageRangeVtbl, IPrintPageRange_Abi): IInspectable(IInspectableVtbl) [IID_IPrintPageRange] {
    fn get_FirstPageNumber(&self, out: *mut i32) -> HRESULT,
    fn get_LastPageNumber(&self, out: *mut i32) -> HRESULT
}}
impl IPrintPageRange {
    #[inline] pub fn get_first_page_number(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_FirstPageNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_last_page_number(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LastPageNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageRange: IPrintPageRange}
impl RtActivatable<IPrintPageRangeFactory> for PrintPageRange {}
impl PrintPageRange {
    #[inline] pub fn create(firstPage: i32, lastPage: i32) -> Result<PrintPageRange> {
        <Self as RtActivatable<IPrintPageRangeFactory>>::get_activation_factory().create(firstPage, lastPage)
    }
    #[inline] pub fn create_with_single_page(page: i32) -> Result<PrintPageRange> {
        <Self as RtActivatable<IPrintPageRangeFactory>>::get_activation_factory().create_with_single_page(page)
    }
}
DEFINE_CLSID!(PrintPageRange(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,80,114,105,110,116,80,97,103,101,82,97,110,103,101,0]) [CLSID_PrintPageRange]);
DEFINE_IID!(IID_IPrintPageRangeFactory, 1083167839, 57415, 24453, 113, 41, 251, 8, 90, 79, 173, 20);
RT_INTERFACE!{static interface IPrintPageRangeFactory(IPrintPageRangeFactoryVtbl, IPrintPageRangeFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeFactory] {
    fn Create(&self, firstPage: i32, lastPage: i32, out: *mut <PrintPageRange as RtType>::Abi) -> HRESULT,
    fn CreateWithSinglePage(&self, page: i32, out: *mut <PrintPageRange as RtType>::Abi) -> HRESULT
}}
impl IPrintPageRangeFactory {
    #[inline] pub fn create(&self, firstPage: i32, lastPage: i32) -> Result<PrintPageRange> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, firstPage, lastPage, &mut out);
        if hr == S_OK { Ok(PrintPageRange::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_single_page(&self, page: i32) -> Result<PrintPageRange> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithSinglePage)(self.get_abi() as *const _ as *mut _, page, &mut out);
        if hr == S_OK { Ok(PrintPageRange::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintPageRangeOptions, 3463296808, 4951, 18098, 169, 35, 121, 249, 149, 244, 72, 252);
RT_INTERFACE!{interface IPrintPageRangeOptions(IPrintPageRangeOptionsVtbl, IPrintPageRangeOptions_Abi): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeOptions] {
    fn put_AllowAllPages(&self, value: bool) -> HRESULT,
    fn get_AllowAllPages(&self, out: *mut bool) -> HRESULT,
    fn put_AllowCurrentPage(&self, value: bool) -> HRESULT,
    fn get_AllowCurrentPage(&self, out: *mut bool) -> HRESULT,
    fn put_AllowCustomSetOfPages(&self, value: bool) -> HRESULT,
    fn get_AllowCustomSetOfPages(&self, out: *mut bool) -> HRESULT
}}
impl IPrintPageRangeOptions {
    #[inline] pub fn set_allow_all_pages(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AllowAllPages)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_all_pages(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AllowAllPages)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_current_page(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AllowCurrentPage)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_current_page(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AllowCurrentPage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_allow_custom_set_of_pages(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_AllowCustomSetOfPages)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_allow_custom_set_of_pages(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_AllowCustomSetOfPages)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IPrintTask(IPrintTaskVtbl, IPrintTask_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTask] {
    #[cfg(not(feature="windows-applicationmodel"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-applicationmodel")] fn get_Properties(&self, out: *mut <super::super::applicationmodel::datatransfer::DataPackagePropertySet as RtType>::Abi) -> HRESULT,
    fn get_Source(&self, out: *mut <IPrintDocumentSource as RtType>::Abi) -> HRESULT,
    fn get_Options(&self, out: *mut <PrintTaskOptions as RtType>::Abi) -> HRESULT,
    fn add_Previewing(&self, eventHandler: <foundation::TypedEventHandler<PrintTask, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Previewing(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Submitting(&self, eventHandler: <foundation::TypedEventHandler<PrintTask, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitting(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Progressing(&self, eventHandler: <foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Progressing(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Completed(&self, eventHandler: <foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Completed(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintTask {
    #[cfg(feature="windows-applicationmodel")] #[inline] pub fn get_properties(&self) -> Result<Option<super::super::applicationmodel::datatransfer::DataPackagePropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Properties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::applicationmodel::datatransfer::DataPackagePropertySet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source(&self) -> Result<Option<IPrintDocumentSource>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Source)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IPrintDocumentSource::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self) -> Result<Option<PrintTaskOptions>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Options)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTaskOptions::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_previewing(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Previewing)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_previewing(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Previewing)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_submitting(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Submitting)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitting(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Submitting)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_progressing(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Progressing)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_progressing(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Progressing)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_completed(&self, eventHandler: &foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Completed)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_completed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Completed)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTask: IPrintTask}
DEFINE_IID!(IID_IPrintTask2, 908281975, 15955, 19869, 143, 94, 49, 106, 200, 222, 218, 225);
RT_INTERFACE!{interface IPrintTask2(IPrintTask2Vtbl, IPrintTask2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTask2] {
    fn put_IsPreviewEnabled(&self, value: bool) -> HRESULT,
    fn get_IsPreviewEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IPrintTask2 {
    #[inline] pub fn set_is_preview_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsPreviewEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_preview_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsPreviewEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskCompletedEventArgs, 1540175023, 9449, 19472, 141, 7, 20, 195, 70, 186, 63, 206);
RT_INTERFACE!{interface IPrintTaskCompletedEventArgs(IPrintTaskCompletedEventArgsVtbl, IPrintTaskCompletedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskCompletedEventArgs] {
    fn get_Completion(&self, out: *mut PrintTaskCompletion) -> HRESULT
}}
impl IPrintTaskCompletedEventArgs {
    #[inline] pub fn get_completion(&self) -> Result<PrintTaskCompletion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Completion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskCompletedEventArgs: IPrintTaskCompletedEventArgs}
RT_ENUM! { enum PrintTaskCompletion: i32 {
    Abandoned = 0, Canceled = 1, Failed = 2, Submitted = 3,
}}
DEFINE_IID!(IID_IPrintTaskOptions, 1510631099, 53897, 16827, 150, 221, 87, 226, 131, 56, 174, 63);
RT_INTERFACE!{interface IPrintTaskOptions(IPrintTaskOptionsVtbl, IPrintTaskOptions_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptions] {
    fn put_Bordering(&self, value: PrintBordering) -> HRESULT,
    fn get_Bordering(&self, out: *mut PrintBordering) -> HRESULT,
    #[cfg(feature="windows-storage")] fn GetPagePrintTicket(&self, printPageInfo: <PrintPageInfo as RtType>::Abi, out: *mut <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptions {
    #[inline] pub fn set_bordering(&self, value: PrintBordering) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Bordering)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bordering(&self) -> Result<PrintBordering> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Bordering)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_page_print_ticket(&self, printPageInfo: &PrintPageInfo) -> Result<Option<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPagePrintTicket)(self.get_abi() as *const _ as *mut _, printPageInfo.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptions: IPrintTaskOptionsCore}
DEFINE_IID!(IID_IPrintTaskOptions2, 3952809478, 39478, 19289, 134, 23, 178, 23, 132, 146, 98, 225);
RT_INTERFACE!{interface IPrintTaskOptions2(IPrintTaskOptions2Vtbl, IPrintTaskOptions2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptions2] {
    fn get_PageRangeOptions(&self, out: *mut <PrintPageRangeOptions as RtType>::Abi) -> HRESULT,
    fn get_CustomPageRanges(&self, out: *mut <foundation::collections::IVector<PrintPageRange> as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptions2 {
    #[inline] pub fn get_page_range_options(&self) -> Result<Option<PrintPageRangeOptions>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageRangeOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintPageRangeOptions::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_custom_page_ranges(&self) -> Result<Option<foundation::collections::IVector<PrintPageRange>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CustomPageRanges)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCore, 467383412, 20177, 16875, 190, 60, 114, 209, 142, 214, 115, 55);
RT_INTERFACE!{interface IPrintTaskOptionsCore(IPrintTaskOptionsCoreVtbl, IPrintTaskOptionsCore_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCore] {
    fn GetPageDescription(&self, jobPageNumber: u32, out: *mut PrintPageDescription) -> HRESULT
}}
impl IPrintTaskOptionsCore {
    #[inline] pub fn get_page_description(&self, jobPageNumber: u32) -> Result<PrintPageDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetPageDescription)(self.get_abi() as *const _ as *mut _, jobPageNumber, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCoreProperties, 3250001970, 40595, 20053, 129, 75, 51, 38, 165, 158, 252, 225);
RT_INTERFACE!{interface IPrintTaskOptionsCoreProperties(IPrintTaskOptionsCorePropertiesVtbl, IPrintTaskOptionsCoreProperties_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCoreProperties] {
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
        let hr = ((*self.get_abi().lpVtbl).put_MediaSize)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_size(&self) -> Result<PrintMediaSize> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MediaSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_media_type(&self, value: PrintMediaType) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_MediaType)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<PrintMediaType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MediaType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_orientation(&self, value: PrintOrientation) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Orientation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<PrintOrientation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_print_quality(&self, value: PrintQuality) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_PrintQuality)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_print_quality(&self) -> Result<PrintQuality> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PrintQuality)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_color_mode(&self, value: PrintColorMode) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ColorMode)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_color_mode(&self) -> Result<PrintColorMode> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ColorMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_duplex(&self, value: PrintDuplex) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Duplex)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_duplex(&self) -> Result<PrintDuplex> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Duplex)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_collation(&self, value: PrintCollation) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Collation)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_collation(&self) -> Result<PrintCollation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Collation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_staple(&self, value: PrintStaple) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Staple)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_staple(&self) -> Result<PrintStaple> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Staple)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_hole_punch(&self, value: PrintHolePunch) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_HolePunch)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_hole_punch(&self) -> Result<PrintHolePunch> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_HolePunch)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_binding(&self, value: PrintBinding) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Binding)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_binding(&self) -> Result<PrintBinding> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Binding)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_min_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MinCopies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxCopies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_number_of_copies(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_NumberOfCopies)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_number_of_copies(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NumberOfCopies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionsCoreUIConfiguration, 1659280931, 39454, 17206, 183, 79, 60, 199, 244, 207, 247, 9);
RT_INTERFACE!{interface IPrintTaskOptionsCoreUIConfiguration(IPrintTaskOptionsCoreUIConfigurationVtbl, IPrintTaskOptionsCoreUIConfiguration_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionsCoreUIConfiguration] {
    fn get_DisplayedOptions(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptionsCoreUIConfiguration {
    #[inline] pub fn get_displayed_options(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayedOptions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskProgressingEventArgs, 2165101515, 46096, 17026, 160, 115, 90, 195, 120, 35, 65, 116);
RT_INTERFACE!{interface IPrintTaskProgressingEventArgs(IPrintTaskProgressingEventArgsVtbl, IPrintTaskProgressingEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskProgressingEventArgs] {
    fn get_DocumentPageCount(&self, out: *mut u32) -> HRESULT
}}
impl IPrintTaskProgressingEventArgs {
    #[inline] pub fn get_document_page_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentPageCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskProgressingEventArgs: IPrintTaskProgressingEventArgs}
DEFINE_IID!(IID_IPrintTaskRequest, 1878400558, 10018, 16960, 166, 124, 243, 100, 132, 154, 23, 243);
RT_INTERFACE!{interface IPrintTaskRequest(IPrintTaskRequestVtbl, IPrintTaskRequest_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequest] {
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn CreatePrintTask(&self, title: HSTRING, handler: <PrintTaskSourceRequestedHandler as RtType>::Abi, out: *mut <PrintTask as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <PrintTaskRequestedDeferral as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskRequest {
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Deadline)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn create_print_task(&self, title: &HStringArg, handler: &PrintTaskSourceRequestedHandler) -> Result<Option<PrintTask>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreatePrintTask)(self.get_abi() as *const _ as *mut _, title.get(), handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTask::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<PrintTaskRequestedDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTaskRequestedDeferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequest: IPrintTaskRequest}
DEFINE_IID!(IID_IPrintTaskRequestedDeferral, 3488592880, 52798, 17095, 148, 150, 100, 128, 12, 98, 44, 68);
RT_INTERFACE!{interface IPrintTaskRequestedDeferral(IPrintTaskRequestedDeferralVtbl, IPrintTaskRequestedDeferral_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequestedDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPrintTaskRequestedDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequestedDeferral: IPrintTaskRequestedDeferral}
DEFINE_IID!(IID_IPrintTaskRequestedEventArgs, 3501193508, 41755, 17740, 167, 182, 93, 12, 197, 34, 252, 22);
RT_INTERFACE!{interface IPrintTaskRequestedEventArgs(IPrintTaskRequestedEventArgsVtbl, IPrintTaskRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskRequestedEventArgs] {
    fn get_Request(&self, out: *mut <PrintTaskRequest as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<PrintTaskRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Request)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTaskRequest::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskRequestedEventArgs: IPrintTaskRequestedEventArgs}
DEFINE_IID!(IID_IPrintTaskSourceRequestedArgs, 4193281982, 62550, 16880, 156, 152, 92, 231, 62, 133, 20, 16);
RT_INTERFACE!{interface IPrintTaskSourceRequestedArgs(IPrintTaskSourceRequestedArgsVtbl, IPrintTaskSourceRequestedArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskSourceRequestedArgs] {
    fn get_Deadline(&self, out: *mut foundation::DateTime) -> HRESULT,
    fn SetSource(&self, source: <IPrintDocumentSource as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <PrintTaskSourceRequestedDeferral as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskSourceRequestedArgs {
    #[inline] pub fn get_deadline(&self) -> Result<foundation::DateTime> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Deadline)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_source(&self, source: &IPrintDocumentSource) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetSource)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<PrintTaskSourceRequestedDeferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTaskSourceRequestedDeferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskSourceRequestedArgs: IPrintTaskSourceRequestedArgs}
DEFINE_IID!(IID_IPrintTaskSourceRequestedDeferral, 1242915025, 27026, 19869, 133, 85, 76, 164, 86, 63, 177, 102);
RT_INTERFACE!{interface IPrintTaskSourceRequestedDeferral(IPrintTaskSourceRequestedDeferralVtbl, IPrintTaskSourceRequestedDeferral_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskSourceRequestedDeferral] {
    fn Complete(&self) -> HRESULT
}}
impl IPrintTaskSourceRequestedDeferral {
    #[inline] pub fn complete(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskSourceRequestedDeferral: IPrintTaskSourceRequestedDeferral}
DEFINE_IID!(IID_PrintTaskSourceRequestedHandler, 1813028776, 23734, 19258, 134, 99, 243, 156, 176, 45, 201, 180);
RT_DELEGATE!{delegate PrintTaskSourceRequestedHandler(PrintTaskSourceRequestedHandlerVtbl, PrintTaskSourceRequestedHandler_Abi, PrintTaskSourceRequestedHandlerImpl) [IID_PrintTaskSourceRequestedHandler] {
    fn Invoke(&self, args: <PrintTaskSourceRequestedArgs as RtType>::Abi) -> HRESULT
}}
impl PrintTaskSourceRequestedHandler {
    #[inline] pub fn invoke(&self, args: &PrintTaskSourceRequestedArgs) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, args.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskTargetDeviceSupport, 693989568, 49867, 19325, 176, 234, 147, 9, 80, 145, 162, 32);
RT_INTERFACE!{interface IPrintTaskTargetDeviceSupport(IPrintTaskTargetDeviceSupportVtbl, IPrintTaskTargetDeviceSupport_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskTargetDeviceSupport] {
    fn put_IsPrinterTargetEnabled(&self, value: bool) -> HRESULT,
    fn get_IsPrinterTargetEnabled(&self, out: *mut bool) -> HRESULT,
    fn put_Is3DManufacturingTargetEnabled(&self, value: bool) -> HRESULT,
    fn get_Is3DManufacturingTargetEnabled(&self, out: *mut bool) -> HRESULT
}}
impl IPrintTaskTargetDeviceSupport {
    #[inline] pub fn set_is_printer_target_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsPrinterTargetEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_printer_target_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsPrinterTargetEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_3d_manufacturing_target_enabled(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Is3DManufacturingTargetEnabled)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_3d_manufacturing_target_enabled(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Is3DManufacturingTargetEnabled)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic(IStandardPrintTaskOptionsStaticVtbl, IStandardPrintTaskOptionsStatic_Abi): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic] {
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
        let hr = ((*self.get_abi().lpVtbl).get_MediaSize)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_media_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MediaType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_orientation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Orientation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_print_quality(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PrintQuality)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_color_mode(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ColorMode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_duplex(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Duplex)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_collation(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Collation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_staple(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Staple)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_hole_punch(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_HolePunch)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_binding(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Binding)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_copies(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Copies)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_n_up(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NUp)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_input_bin(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_InputBin)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStandardPrintTaskOptionsStatic2, 1004768244, 31300, 17001, 154, 82, 129, 38, 30, 40, 158, 233);
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic2(IStandardPrintTaskOptionsStatic2Vtbl, IStandardPrintTaskOptionsStatic2_Abi): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic2] {
    fn get_Bordering(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardPrintTaskOptionsStatic2 {
    #[inline] pub fn get_bordering(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Bordering)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IStandardPrintTaskOptionsStatic3, 3153497734, 14424, 16819, 167, 153, 85, 221, 152, 136, 212, 117);
RT_INTERFACE!{static interface IStandardPrintTaskOptionsStatic3(IStandardPrintTaskOptionsStatic3Vtbl, IStandardPrintTaskOptionsStatic3_Abi): IInspectable(IInspectableVtbl) [IID_IStandardPrintTaskOptionsStatic3] {
    fn get_CustomPageRanges(&self, out: *mut HSTRING) -> HRESULT
}}
impl IStandardPrintTaskOptionsStatic3 {
    #[inline] pub fn get_custom_page_ranges(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CustomPageRanges)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
pub mod optiondetails { // Windows.Graphics.Printing.OptionDetails
use crate::prelude::*;
DEFINE_IID!(IID_IPrintBindingOptionDetails, 3287600280, 38244, 20246, 160, 85, 169, 139, 154, 73, 233, 211);
RT_INTERFACE!{interface IPrintBindingOptionDetails(IPrintBindingOptionDetailsVtbl, IPrintBindingOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintBindingOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintBindingOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintBindingOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintBorderingOptionDetails, 1299430543, 64339, 20146, 152, 95, 29, 145, 222, 11, 118, 57);
RT_INTERFACE!{interface IPrintBorderingOptionDetails(IPrintBorderingOptionDetailsVtbl, IPrintBorderingOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintBorderingOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintBorderingOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintBorderingOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCollationOptionDetails, 3601576294, 42406, 16604, 172, 195, 115, 159, 40, 241, 229, 211);
RT_INTERFACE!{interface IPrintCollationOptionDetails(IPrintCollationOptionDetailsVtbl, IPrintCollationOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCollationOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCollationOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCollationOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintColorModeOptionDetails, 3685316356, 61910, 18499, 164, 132, 155, 68, 124, 220, 243, 182);
RT_INTERFACE!{interface IPrintColorModeOptionDetails(IPrintColorModeOptionDetailsVtbl, IPrintColorModeOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintColorModeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintColorModeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintColorModeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCopiesOptionDetails, 1107636377, 17209, 17219, 137, 141, 44, 71, 181, 224, 195, 65);
RT_INTERFACE!{interface IPrintCopiesOptionDetails(IPrintCopiesOptionDetailsVtbl, IPrintCopiesOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCopiesOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCopiesOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCopiesOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomItemDetails, 1459926583, 23610, 17562, 170, 54, 179, 41, 27, 17, 146, 253);
RT_INTERFACE!{interface IPrintCustomItemDetails(IPrintCustomItemDetailsVtbl, IPrintCustomItemDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemDetails] {
    fn get_ItemId(&self, out: *mut HSTRING) -> HRESULT,
    fn put_ItemDisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_ItemDisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomItemDetails {
    #[inline] pub fn get_item_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ItemId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_item_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ItemDisplayName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_item_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ItemDisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomItemDetails: IPrintCustomItemDetails}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails, 2784689544, 22770, 20157, 185, 15, 81, 228, 242, 148, 76, 93);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails(IPrintCustomItemListOptionDetailsVtbl, IPrintCustomItemListOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails] {
    fn AddItem(&self, itemId: HSTRING, displayName: HSTRING) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails {
    #[inline] pub fn add_item(&self, itemId: &HStringArg, displayName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AddItem)(self.get_abi() as *const _ as *mut _, itemId.get(), displayName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomItemListOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails2, 3386258749, 25884, 19001, 144, 110, 16, 145, 161, 128, 27, 241);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails2(IPrintCustomItemListOptionDetails2Vtbl, IPrintCustomItemListOptionDetails2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails2] {
    #[cfg(feature="windows-storage")] fn AddItem(&self, itemId: HSTRING, displayName: HSTRING, description: HSTRING, icon: <crate::windows::storage::streams::IRandomAccessStreamWithContentType as RtType>::Abi) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn add_item(&self, itemId: &HStringArg, displayName: &HStringArg, description: &HStringArg, icon: &crate::windows::storage::streams::IRandomAccessStreamWithContentType) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AddItem)(self.get_abi() as *const _ as *mut _, itemId.get(), displayName.get(), description.get(), icon.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomItemListOptionDetails3, 1335997759, 15412, 18536, 164, 7, 252, 94, 171, 37, 155, 33);
RT_INTERFACE!{interface IPrintCustomItemListOptionDetails3(IPrintCustomItemListOptionDetails3Vtbl, IPrintCustomItemListOptionDetails3_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomItemListOptionDetails3] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomItemListOptionDetails3 {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomOptionDetails, 3811302940, 10415, 19344, 149, 218, 163, 172, 243, 32, 185, 41);
RT_INTERFACE!{interface IPrintCustomOptionDetails(IPrintCustomOptionDetailsVtbl, IPrintCustomOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomOptionDetails] {
    fn put_DisplayName(&self, value: HSTRING) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomOptionDetails {
    #[inline] pub fn set_display_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DisplayName)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomTextOptionDetails, 718369272, 51389, 18693, 145, 146, 13, 117, 19, 110, 139, 49);
RT_INTERFACE!{interface IPrintCustomTextOptionDetails(IPrintCustomTextOptionDetailsVtbl, IPrintCustomTextOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomTextOptionDetails] {
    fn put_MaxCharacters(&self, value: u32) -> HRESULT,
    fn get_MaxCharacters(&self, out: *mut u32) -> HRESULT
}}
impl IPrintCustomTextOptionDetails {
    #[inline] pub fn set_max_characters(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_MaxCharacters)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_characters(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxCharacters)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomTextOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintCustomTextOptionDetails2, 3467053908, 47479, 18200, 131, 56, 126, 210, 176, 216, 111, 227);
RT_INTERFACE!{interface IPrintCustomTextOptionDetails2(IPrintCustomTextOptionDetails2Vtbl, IPrintCustomTextOptionDetails2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomTextOptionDetails2] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomTextOptionDetails2 {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintCustomToggleOptionDetails, 2645873940, 58465, 17928, 142, 233, 219, 111, 94, 208, 115, 198);
RT_INTERFACE!{interface IPrintCustomToggleOptionDetails(IPrintCustomToggleOptionDetailsVtbl, IPrintCustomToggleOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintCustomToggleOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintCustomToggleOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintCustomToggleOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintDuplexOptionDetails, 4242097553, 54436, 17658, 179, 254, 66, 224, 186, 40, 213, 173);
RT_INTERFACE!{interface IPrintDuplexOptionDetails(IPrintDuplexOptionDetailsVtbl, IPrintDuplexOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintDuplexOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintDuplexOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintDuplexOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintHolePunchOptionDetails, 2799574808, 18476, 18007, 157, 113, 141, 221, 219, 234, 30, 30);
RT_INTERFACE!{interface IPrintHolePunchOptionDetails(IPrintHolePunchOptionDetailsVtbl, IPrintHolePunchOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintHolePunchOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintHolePunchOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintHolePunchOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintItemListOptionDetails, 2585941951, 65121, 17368, 162, 79, 163, 246, 171, 115, 32, 231);
RT_INTERFACE!{interface IPrintItemListOptionDetails(IPrintItemListOptionDetailsVtbl, IPrintItemListOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintItemListOptionDetails] {
    fn get_Items(&self, out: *mut <foundation::collections::IVectorView<IInspectable> as RtType>::Abi) -> HRESULT
}}
impl IPrintItemListOptionDetails {
    #[inline] pub fn get_items(&self) -> Result<Option<foundation::collections::IVectorView<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Items)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintMediaSizeOptionDetails, 1821203407, 49343, 18376, 184, 74, 98, 142, 125, 13, 26, 29);
RT_INTERFACE!{interface IPrintMediaSizeOptionDetails(IPrintMediaSizeOptionDetailsVtbl, IPrintMediaSizeOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintMediaSizeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintMediaSizeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintMediaSizeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintMediaTypeOptionDetails, 4173791243, 44019, 19132, 142, 134, 34, 171, 197, 116, 74, 67);
RT_INTERFACE!{interface IPrintMediaTypeOptionDetails(IPrintMediaTypeOptionDetailsVtbl, IPrintMediaTypeOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintMediaTypeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintMediaTypeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintMediaTypeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintNumberOptionDetails, 1291959215, 25692, 19945, 150, 95, 111, 198, 187, 196, 124, 171);
RT_INTERFACE!{interface IPrintNumberOptionDetails(IPrintNumberOptionDetailsVtbl, IPrintNumberOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintNumberOptionDetails] {
    fn get_MinValue(&self, out: *mut u32) -> HRESULT,
    fn get_MaxValue(&self, out: *mut u32) -> HRESULT
}}
impl IPrintNumberOptionDetails {
    #[inline] pub fn get_min_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MinValue)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_max_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxValue)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintOptionDetails, 956729039, 54914, 18783, 173, 254, 215, 51, 63, 92, 24, 8);
RT_INTERFACE!{interface IPrintOptionDetails(IPrintOptionDetailsVtbl, IPrintOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintOptionDetails] {
    fn get_OptionId(&self, out: *mut HSTRING) -> HRESULT,
    fn get_OptionType(&self, out: *mut PrintOptionType) -> HRESULT,
    fn put_ErrorText(&self, value: HSTRING) -> HRESULT,
    fn get_ErrorText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_State(&self, value: PrintOptionStates) -> HRESULT,
    fn get_State(&self, out: *mut PrintOptionStates) -> HRESULT,
    fn get_Value(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn TrySetValue(&self, value: <IInspectable as RtType>::Abi, out: *mut bool) -> HRESULT
}}
impl IPrintOptionDetails {
    #[inline] pub fn get_option_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_OptionId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_option_type(&self) -> Result<PrintOptionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_OptionType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_error_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ErrorText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_error_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ErrorText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_state(&self, value: PrintOptionStates) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_State)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_state(&self) -> Result<PrintOptionStates> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_State)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_set_value(&self, value: &IInspectable) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).TrySetValue)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IPrintOrientationOptionDetails(IPrintOrientationOptionDetailsVtbl, IPrintOrientationOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintOrientationOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintOrientationOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintOrientationOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintPageRangeOptionDetails, 1511646391, 11240, 19111, 158, 165, 222, 251, 232, 113, 59, 78);
RT_INTERFACE!{interface IPrintPageRangeOptionDetails(IPrintPageRangeOptionDetailsVtbl, IPrintPageRangeOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintPageRangeOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintPageRangeOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintPageRangeOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintQualityOptionDetails, 768633761, 52762, 17638, 132, 249, 58, 146, 234, 30, 48, 68);
RT_INTERFACE!{interface IPrintQualityOptionDetails(IPrintQualityOptionDetailsVtbl, IPrintQualityOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintQualityOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintQualityOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintQualityOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintStapleOptionDetails, 3560011197, 39947, 17632, 132, 246, 206, 235, 206, 101, 56, 0);
RT_INTERFACE!{interface IPrintStapleOptionDetails(IPrintStapleOptionDetailsVtbl, IPrintStapleOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintStapleOptionDetails] {
    fn put_WarningText(&self, value: HSTRING) -> HRESULT,
    fn get_WarningText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Description(&self, value: HSTRING) -> HRESULT,
    fn get_Description(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintStapleOptionDetails {
    #[inline] pub fn set_warning_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_WarningText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_warning_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_WarningText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_description(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Description)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_description(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Description)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintStapleOptionDetails: IPrintOptionDetails}
DEFINE_IID!(IID_IPrintTaskOptionChangedEventArgs, 1696169221, 42478, 17159, 148, 7, 154, 202, 209, 71, 103, 156);
RT_INTERFACE!{interface IPrintTaskOptionChangedEventArgs(IPrintTaskOptionChangedEventArgsVtbl, IPrintTaskOptionChangedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionChangedEventArgs] {
    fn get_OptionId(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptionChangedEventArgs {
    #[inline] pub fn get_option_id(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_OptionId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptionChangedEventArgs: IPrintTaskOptionChangedEventArgs}
DEFINE_IID!(IID_IPrintTaskOptionDetails, 4117891825, 43166, 17062, 129, 175, 248, 224, 16, 179, 138, 104);
RT_INTERFACE!{interface IPrintTaskOptionDetails(IPrintTaskOptionDetailsVtbl, IPrintTaskOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetails] {
    fn get_Options(&self, out: *mut <foundation::collections::IMapView<HString, IPrintOptionDetails> as RtType>::Abi) -> HRESULT,
    fn CreateItemListOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut <PrintCustomItemListOptionDetails as RtType>::Abi) -> HRESULT,
    fn CreateTextOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut <PrintCustomTextOptionDetails as RtType>::Abi) -> HRESULT,
    fn add_OptionChanged(&self, eventHandler: <foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_OptionChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_BeginValidation(&self, eventHandler: <foundation::TypedEventHandler<PrintTaskOptionDetails, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_BeginValidation(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrintTaskOptionDetails {
    #[inline] pub fn get_options(&self) -> Result<Option<foundation::collections::IMapView<HString, IPrintOptionDetails>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Options)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMapView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_item_list_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<PrintCustomItemListOptionDetails>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateItemListOption)(self.get_abi() as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(PrintCustomItemListOptionDetails::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_text_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<PrintCustomTextOptionDetails>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateTextOption)(self.get_abi() as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(PrintCustomTextOptionDetails::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_option_changed(&self, eventHandler: &foundation::TypedEventHandler<PrintTaskOptionDetails, PrintTaskOptionChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_OptionChanged)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_option_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_OptionChanged)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_begin_validation(&self, eventHandler: &foundation::TypedEventHandler<PrintTaskOptionDetails, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_BeginValidation)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_begin_validation(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_BeginValidation)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTaskOptionDetails: IPrintTaskOptionDetails}
impl RtActivatable<IPrintTaskOptionDetailsStatic> for PrintTaskOptionDetails {}
impl PrintTaskOptionDetails {
    #[inline] pub fn get_from_print_task_options(printTaskOptions: &super::PrintTaskOptions) -> Result<Option<PrintTaskOptionDetails>> {
        <Self as RtActivatable<IPrintTaskOptionDetailsStatic>>::get_activation_factory().get_from_print_task_options(printTaskOptions)
    }
}
DEFINE_CLSID!(PrintTaskOptionDetails(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,46,79,112,116,105,111,110,68,101,116,97,105,108,115,46,80,114,105,110,116,84,97,115,107,79,112,116,105,111,110,68,101,116,97,105,108,115,0]) [CLSID_PrintTaskOptionDetails]);
DEFINE_IID!(IID_IPrintTaskOptionDetails2, 1400048137, 63848, 18066, 161, 119, 192, 116, 89, 113, 134, 219);
RT_INTERFACE!{interface IPrintTaskOptionDetails2(IPrintTaskOptionDetails2Vtbl, IPrintTaskOptionDetails2_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetails2] {
    fn CreateToggleOption(&self, optionId: HSTRING, displayName: HSTRING, out: *mut <PrintCustomToggleOptionDetails as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptionDetails2 {
    #[inline] pub fn create_toggle_option(&self, optionId: &HStringArg, displayName: &HStringArg) -> Result<Option<PrintCustomToggleOptionDetails>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateToggleOption)(self.get_abi() as *const _ as *mut _, optionId.get(), displayName.get(), &mut out);
        if hr == S_OK { Ok(PrintCustomToggleOptionDetails::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTaskOptionDetailsStatic, 324903315, 2401, 19310, 135, 102, 241, 59, 127, 188, 205, 88);
RT_INTERFACE!{static interface IPrintTaskOptionDetailsStatic(IPrintTaskOptionDetailsStaticVtbl, IPrintTaskOptionDetailsStatic_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTaskOptionDetailsStatic] {
    fn GetFromPrintTaskOptions(&self, printTaskOptions: <super::PrintTaskOptions as RtType>::Abi, out: *mut <PrintTaskOptionDetails as RtType>::Abi) -> HRESULT
}}
impl IPrintTaskOptionDetailsStatic {
    #[inline] pub fn get_from_print_task_options(&self, printTaskOptions: &super::PrintTaskOptions) -> Result<Option<PrintTaskOptionDetails>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFromPrintTaskOptions)(self.get_abi() as *const _ as *mut _, printTaskOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTaskOptionDetails::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrintTextOptionDetails, 2910184803, 23780, 18108, 153, 24, 171, 159, 173, 20, 76, 91);
RT_INTERFACE!{interface IPrintTextOptionDetails(IPrintTextOptionDetailsVtbl, IPrintTextOptionDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTextOptionDetails] {
    fn get_MaxCharacters(&self, out: *mut u32) -> HRESULT
}}
impl IPrintTextOptionDetails {
    #[inline] pub fn get_max_characters(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxCharacters)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
} // Windows.Graphics.Printing.OptionDetails
pub mod printticket { // Windows.Graphics.Printing.PrintTicket
use crate::prelude::*;
DEFINE_IID!(IID_IPrintTicketCapabilities, 2353352843, 48092, 16982, 161, 66, 47, 214, 21, 236, 180, 22);
RT_INTERFACE!{interface IPrintTicketCapabilities(IPrintTicketCapabilitiesVtbl, IPrintTicketCapabilities_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketCapabilities] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_DocumentBindingFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentCollateFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentDuplexFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentHolePunchFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentInputBinFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentNUpFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentStapleFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_JobPasscodeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageBorderlessFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageMediaSizeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageMediaTypeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOrientationFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOutputColorFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOutputQualityFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageResolutionFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn GetFeature(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn GetParameterDefinition(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketParameterDefinition as RtType>::Abi) -> HRESULT
}}
impl IPrintTicketCapabilities {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_binding_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentBindingFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_collate_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentCollateFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_duplex_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentDuplexFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_hole_punch_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentHolePunchFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_input_bin_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentInputBinFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_n_up_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentNUpFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_staple_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentStapleFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_passcode_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_JobPasscodeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_borderless_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageBorderlessFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_size_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageMediaSizeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_type_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageMediaTypeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_orientation_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOrientationFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_color_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOutputColorFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_quality_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOutputQualityFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_resolution_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageResolutionFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_feature(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFeature)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parameter_definition(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketParameterDefinition>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetParameterDefinition)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketParameterDefinition::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketCapabilities: IPrintTicketCapabilities}
DEFINE_IID!(IID_IPrintTicketFeature, 3881860458, 23029, 16643, 136, 88, 185, 119, 16, 150, 61, 57);
RT_INTERFACE!{interface IPrintTicketFeature(IPrintTicketFeatureVtbl, IPrintTicketFeature_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketFeature] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetOption(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketOption as RtType>::Abi) -> HRESULT,
    fn get_Options(&self, out: *mut <foundation::collections::IVectorView<PrintTicketOption> as RtType>::Abi) -> HRESULT,
    fn GetSelectedOption(&self, out: *mut <PrintTicketOption as RtType>::Abi) -> HRESULT,
    fn SetSelectedOption(&self, value: <PrintTicketOption as RtType>::Abi) -> HRESULT,
    fn get_SelectionType(&self, out: *mut PrintTicketFeatureSelectionType) -> HRESULT
}}
impl IPrintTicketFeature {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_option(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketOption>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetOption)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketOption::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_options(&self) -> Result<Option<foundation::collections::IVectorView<PrintTicketOption>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Options)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_selected_option(&self) -> Result<Option<PrintTicketOption>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSelectedOption)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketOption::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_selected_option(&self, value: &PrintTicketOption) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetSelectedOption)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_selection_type(&self) -> Result<PrintTicketFeatureSelectionType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SelectionType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketFeature: IPrintTicketFeature}
RT_ENUM! { enum PrintTicketFeatureSelectionType: i32 {
    PickOne = 0, PickMany = 1,
}}
DEFINE_IID!(IID_IPrintTicketOption, 2961624976, 45927, 20043, 189, 72, 156, 120, 160, 187, 49, 206);
RT_INTERFACE!{interface IPrintTicketOption(IPrintTicketOptionVtbl, IPrintTicketOption_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketOption] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_DisplayName(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-data")] fn GetPropertyNode(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy5(&self) -> (),
    #[cfg(feature="windows-data")] fn GetScoredPropertyNode(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn GetPropertyValue(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketValue as RtType>::Abi) -> HRESULT,
    fn GetScoredPropertyValue(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketValue as RtType>::Abi) -> HRESULT
}}
impl IPrintTicketOption {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_property_node(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPropertyNode)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_scored_property_node(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetScoredPropertyNode)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_property_value(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPropertyValue)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_scored_property_value(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetScoredPropertyValue)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketValue::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketOption: IPrintTicketOption}
RT_ENUM! { enum PrintTicketParameterDataType: i32 {
    Integer = 0, NumericString = 1, String = 2,
}}
DEFINE_IID!(IID_IPrintTicketParameterDefinition, 3602560228, 10594, 19457, 183, 243, 154, 146, 148, 235, 131, 53);
RT_INTERFACE!{interface IPrintTicketParameterDefinition(IPrintTicketParameterDefinitionVtbl, IPrintTicketParameterDefinition_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketParameterDefinition] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_DataType(&self, out: *mut PrintTicketParameterDataType) -> HRESULT,
    fn get_UnitType(&self, out: *mut HSTRING) -> HRESULT,
    fn get_RangeMin(&self, out: *mut i32) -> HRESULT,
    fn get_RangeMax(&self, out: *mut i32) -> HRESULT
}}
impl IPrintTicketParameterDefinition {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data_type(&self) -> Result<PrintTicketParameterDataType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DataType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_unit_type(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_UnitType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_range_min(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RangeMin)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_range_max(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_RangeMax)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketParameterDefinition: IPrintTicketParameterDefinition}
DEFINE_IID!(IID_IPrintTicketParameterInitializer, 1580414395, 41125, 18609, 157, 92, 7, 17, 109, 220, 89, 122);
RT_INTERFACE!{interface IPrintTicketParameterInitializer(IPrintTicketParameterInitializerVtbl, IPrintTicketParameterInitializer_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketParameterInitializer] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn put_Value(&self, value: <PrintTicketValue as RtType>::Abi) -> HRESULT,
    fn get_Value(&self, out: *mut <PrintTicketValue as RtType>::Abi) -> HRESULT
}}
impl IPrintTicketParameterInitializer {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &PrintTicketValue) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Value)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<Option<PrintTicketValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketValue::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketParameterInitializer: IPrintTicketParameterInitializer}
DEFINE_IID!(IID_IPrintTicketValue, 1723009586, 9293, 20002, 169, 139, 187, 60, 241, 242, 221, 145);
RT_INTERFACE!{interface IPrintTicketValue(IPrintTicketValueVtbl, IPrintTicketValue_Abi): IInspectable(IInspectableVtbl) [IID_IPrintTicketValue] {
    fn get_Type(&self, out: *mut PrintTicketValueType) -> HRESULT,
    fn GetValueAsInteger(&self, out: *mut i32) -> HRESULT,
    fn GetValueAsString(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintTicketValue {
    #[inline] pub fn get_type(&self) -> Result<PrintTicketValueType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Type)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_as_integer(&self) -> Result<i32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetValueAsInteger)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value_as_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetValueAsString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintTicketValue: IPrintTicketValue}
RT_ENUM! { enum PrintTicketValueType: i32 {
    Integer = 0, String = 1, Unknown = 2,
}}
DEFINE_IID!(IID_IWorkflowPrintTicket, 1104487045, 13800, 17550, 168, 197, 228, 182, 162, 207, 130, 108);
RT_INTERFACE!{interface IWorkflowPrintTicket(IWorkflowPrintTicketVtbl, IWorkflowPrintTicket_Abi): IInspectable(IInspectableVtbl) [IID_IWorkflowPrintTicket] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_XmlNamespace(&self, out: *mut HSTRING) -> HRESULT,
    #[cfg(not(feature="windows-data"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-data")] fn get_XmlNode(&self, out: *mut <crate::windows::data::xml::dom::IXmlNode as RtType>::Abi) -> HRESULT,
    fn GetCapabilities(&self, out: *mut <PrintTicketCapabilities as RtType>::Abi) -> HRESULT,
    fn get_DocumentBindingFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentCollateFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentDuplexFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentHolePunchFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentInputBinFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentNUpFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_DocumentStapleFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_JobPasscodeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageBorderlessFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageMediaSizeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageMediaTypeFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOrientationFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOutputColorFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageOutputQualityFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn get_PageResolutionFeature(&self, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn GetFeature(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketFeature as RtType>::Abi) -> HRESULT,
    fn NotifyXmlChangedAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn ValidateAsync(&self, out: *mut <foundation::IAsyncOperation<WorkflowPrintTicketValidationResult> as RtType>::Abi) -> HRESULT,
    fn GetParameterInitializer(&self, name: HSTRING, xmlNamespace: HSTRING, out: *mut <PrintTicketParameterInitializer as RtType>::Abi) -> HRESULT,
    fn SetParameterInitializerAsInteger(&self, name: HSTRING, xmlNamespace: HSTRING, integerValue: i32, out: *mut <PrintTicketParameterInitializer as RtType>::Abi) -> HRESULT,
    fn SetParameterInitializerAsString(&self, name: HSTRING, xmlNamespace: HSTRING, stringValue: HSTRING, out: *mut <PrintTicketParameterInitializer as RtType>::Abi) -> HRESULT,
    fn MergeAndValidateTicket(&self, deltaShemaTicket: <WorkflowPrintTicket as RtType>::Abi, out: *mut <WorkflowPrintTicket as RtType>::Abi) -> HRESULT
}}
impl IWorkflowPrintTicket {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xml_namespace(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNamespace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-data")] #[inline] pub fn get_xml_node(&self) -> Result<Option<crate::windows::data::xml::dom::IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XmlNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::data::xml::dom::IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_capabilities(&self) -> Result<Option<PrintTicketCapabilities>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCapabilities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketCapabilities::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_binding_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentBindingFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_collate_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentCollateFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_duplex_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentDuplexFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_hole_punch_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentHolePunchFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_input_bin_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentInputBinFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_n_up_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentNUpFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_staple_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentStapleFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_passcode_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_JobPasscodeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_borderless_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageBorderlessFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_size_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageMediaSizeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_media_type_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageMediaTypeFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_orientation_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOrientationFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_color_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOutputColorFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_output_quality_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageOutputQualityFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_resolution_feature(&self) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PageResolutionFeature)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_feature(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketFeature>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetFeature)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketFeature::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn notify_xml_changed_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).NotifyXmlChangedAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn validate_async(&self) -> Result<foundation::IAsyncOperation<WorkflowPrintTicketValidationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ValidateAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parameter_initializer(&self, name: &HStringArg, xmlNamespace: &HStringArg) -> Result<Option<PrintTicketParameterInitializer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetParameterInitializer)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketParameterInitializer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_parameter_initializer_as_integer(&self, name: &HStringArg, xmlNamespace: &HStringArg, integerValue: i32) -> Result<Option<PrintTicketParameterInitializer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetParameterInitializerAsInteger)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), integerValue, &mut out);
        if hr == S_OK { Ok(PrintTicketParameterInitializer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_parameter_initializer_as_string(&self, name: &HStringArg, xmlNamespace: &HStringArg, stringValue: &HStringArg) -> Result<Option<PrintTicketParameterInitializer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetParameterInitializerAsString)(self.get_abi() as *const _ as *mut _, name.get(), xmlNamespace.get(), stringValue.get(), &mut out);
        if hr == S_OK { Ok(PrintTicketParameterInitializer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn merge_and_validate_ticket(&self, deltaShemaTicket: &WorkflowPrintTicket) -> Result<Option<WorkflowPrintTicket>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).MergeAndValidateTicket)(self.get_abi() as *const _ as *mut _, deltaShemaTicket.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(WorkflowPrintTicket::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WorkflowPrintTicket: IWorkflowPrintTicket}
DEFINE_IID!(IID_IWorkflowPrintTicketValidationResult, 181531538, 55931, 18998, 191, 54, 106, 153, 166, 46, 32, 89);
RT_INTERFACE!{interface IWorkflowPrintTicketValidationResult(IWorkflowPrintTicketValidationResultVtbl, IWorkflowPrintTicketValidationResult_Abi): IInspectable(IInspectableVtbl) [IID_IWorkflowPrintTicketValidationResult] {
    fn get_Validated(&self, out: *mut bool) -> HRESULT,
    fn get_ExtendedError(&self, out: *mut foundation::HResult) -> HRESULT
}}
impl IWorkflowPrintTicketValidationResult {
    #[inline] pub fn get_validated(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Validated)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_error(&self) -> Result<foundation::HResult> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ExtendedError)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class WorkflowPrintTicketValidationResult: IWorkflowPrintTicketValidationResult}
} // Windows.Graphics.Printing.PrintTicket
pub mod workflow { // Windows.Graphics.Printing.Workflow
use crate::prelude::*;
DEFINE_IID!(IID_IPrintWorkflowBackgroundSession, 1534661562, 3166, 21130, 116, 88, 134, 164, 108, 189, 220, 69);
RT_INTERFACE!{interface IPrintWorkflowBackgroundSession(IPrintWorkflowBackgroundSessionVtbl, IPrintWorkflowBackgroundSession_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowBackgroundSession] {
    fn add_SetupRequested(&self, setupEventHandler: <foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SetupRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Submitted(&self, submittedEventHandler: <foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitted(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut PrintWorkflowSessionStatus) -> HRESULT,
    fn Start(&self) -> HRESULT
}}
impl IPrintWorkflowBackgroundSession {
    #[inline] pub fn add_setup_requested(&self, setupEventHandler: &foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowBackgroundSetupRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_SetupRequested)(self.get_abi() as *const _ as *mut _, setupEventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_setup_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_SetupRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_submitted(&self, submittedEventHandler: &foundation::TypedEventHandler<PrintWorkflowBackgroundSession, PrintWorkflowSubmittedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Submitted)(self.get_abi() as *const _ as *mut _, submittedEventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitted(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Submitted)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<PrintWorkflowSessionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Start)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowBackgroundSession: IPrintWorkflowBackgroundSession}
DEFINE_IID!(IID_IPrintWorkflowBackgroundSetupRequestedEventArgs, 1139372866, 5968, 22985, 97, 251, 56, 55, 72, 162, 3, 98);
RT_INTERFACE!{interface IPrintWorkflowBackgroundSetupRequestedEventArgs(IPrintWorkflowBackgroundSetupRequestedEventArgsVtbl, IPrintWorkflowBackgroundSetupRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowBackgroundSetupRequestedEventArgs] {
    fn GetUserPrintTicketAsync(&self, out: *mut <foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket> as RtType>::Abi) -> HRESULT,
    fn get_Configuration(&self, out: *mut <PrintWorkflowConfiguration as RtType>::Abi) -> HRESULT,
    fn SetRequiresUI(&self) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowBackgroundSetupRequestedEventArgs {
    #[inline] pub fn get_user_print_ticket_async(&self) -> Result<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetUserPrintTicketAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<PrintWorkflowConfiguration>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Configuration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowConfiguration::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_requires_ui(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetRequiresUI)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowBackgroundSetupRequestedEventArgs: IPrintWorkflowBackgroundSetupRequestedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowConfiguration, 3500852461, 64843, 24053, 75, 182, 141, 13, 21, 158, 190, 63);
RT_INTERFACE!{interface IPrintWorkflowConfiguration(IPrintWorkflowConfigurationVtbl, IPrintWorkflowConfiguration_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowConfiguration] {
    fn get_SourceAppDisplayName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_JobTitle(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SessionId(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrintWorkflowConfiguration {
    #[inline] pub fn get_source_app_display_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SourceAppDisplayName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_job_title(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_JobTitle)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_session_id(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SessionId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowConfiguration: IPrintWorkflowConfiguration}
DEFINE_IID!(IID_IPrintWorkflowForegroundSession, 3348849616, 63724, 19691, 149, 58, 200, 135, 97, 87, 221, 51);
RT_INTERFACE!{interface IPrintWorkflowForegroundSession(IPrintWorkflowForegroundSessionVtbl, IPrintWorkflowForegroundSession_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowForegroundSession] {
    fn add_SetupRequested(&self, setupEventHandler: <foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SetupRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn add_XpsDataAvailable(&self, xpsDataAvailableEventHandler: <foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_XpsDataAvailable(&self, token: foundation::EventRegistrationToken) -> HRESULT,
    fn get_Status(&self, out: *mut PrintWorkflowSessionStatus) -> HRESULT,
    fn Start(&self) -> HRESULT
}}
impl IPrintWorkflowForegroundSession {
    #[inline] pub fn add_setup_requested(&self, setupEventHandler: &foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowForegroundSetupRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_SetupRequested)(self.get_abi() as *const _ as *mut _, setupEventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_setup_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_SetupRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_xps_data_available(&self, xpsDataAvailableEventHandler: &foundation::TypedEventHandler<PrintWorkflowForegroundSession, PrintWorkflowXpsDataAvailableEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_XpsDataAvailable)(self.get_abi() as *const _ as *mut _, xpsDataAvailableEventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_xps_data_available(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_XpsDataAvailable)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_status(&self) -> Result<PrintWorkflowSessionStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Status)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn start(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Start)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowForegroundSession: IPrintWorkflowForegroundSession}
DEFINE_IID!(IID_IPrintWorkflowForegroundSetupRequestedEventArgs, 3152249415, 39963, 19923, 155, 43, 200, 4, 104, 217, 65, 179);
RT_INTERFACE!{interface IPrintWorkflowForegroundSetupRequestedEventArgs(IPrintWorkflowForegroundSetupRequestedEventArgsVtbl, IPrintWorkflowForegroundSetupRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowForegroundSetupRequestedEventArgs] {
    fn GetUserPrintTicketAsync(&self, out: *mut <foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket> as RtType>::Abi) -> HRESULT,
    fn get_Configuration(&self, out: *mut <PrintWorkflowConfiguration as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowForegroundSetupRequestedEventArgs {
    #[inline] pub fn get_user_print_ticket_async(&self) -> Result<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetUserPrintTicketAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<PrintWorkflowConfiguration>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Configuration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowConfiguration::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowForegroundSetupRequestedEventArgs: IPrintWorkflowForegroundSetupRequestedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowObjectModelSourceFileContent, 3278670442, 35370, 16794, 179, 195, 32, 144, 230, 191, 171, 47);
RT_INTERFACE!{interface IPrintWorkflowObjectModelSourceFileContent(IPrintWorkflowObjectModelSourceFileContentVtbl, IPrintWorkflowObjectModelSourceFileContent_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowObjectModelSourceFileContent] {
    
}}
RT_CLASS!{class PrintWorkflowObjectModelSourceFileContent: IPrintWorkflowObjectModelSourceFileContent}
DEFINE_IID!(IID_IPrintWorkflowObjectModelTargetPackage, 2107030644, 39764, 19617, 173, 58, 151, 156, 61, 68, 221, 172);
RT_INTERFACE!{interface IPrintWorkflowObjectModelTargetPackage(IPrintWorkflowObjectModelTargetPackageVtbl, IPrintWorkflowObjectModelTargetPackage_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowObjectModelTargetPackage] {
    
}}
RT_CLASS!{class PrintWorkflowObjectModelTargetPackage: IPrintWorkflowObjectModelTargetPackage}
RT_ENUM! { enum PrintWorkflowSessionStatus: i32 {
    Started = 0, Completed = 1, Aborted = 2, Closed = 3,
}}
DEFINE_IID!(IID_IPrintWorkflowSourceContent, 438879809, 52913, 17715, 187, 115, 251, 230, 62, 239, 219, 24);
RT_INTERFACE!{interface IPrintWorkflowSourceContent(IPrintWorkflowSourceContentVtbl, IPrintWorkflowSourceContent_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSourceContent] {
    fn GetJobPrintTicketAsync(&self, out: *mut <foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket> as RtType>::Abi) -> HRESULT,
    fn GetSourceSpoolDataAsStreamContent(&self, out: *mut <PrintWorkflowSpoolStreamContent as RtType>::Abi) -> HRESULT,
    fn GetSourceSpoolDataAsXpsObjectModel(&self, out: *mut <PrintWorkflowObjectModelSourceFileContent as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowSourceContent {
    #[inline] pub fn get_job_print_ticket_async(&self) -> Result<foundation::IAsyncOperation<super::printticket::WorkflowPrintTicket>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetJobPrintTicketAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_spool_data_as_stream_content(&self) -> Result<Option<PrintWorkflowSpoolStreamContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSourceSpoolDataAsStreamContent)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowSpoolStreamContent::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_spool_data_as_xps_object_model(&self) -> Result<Option<PrintWorkflowObjectModelSourceFileContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetSourceSpoolDataAsXpsObjectModel)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowObjectModelSourceFileContent::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSourceContent: IPrintWorkflowSourceContent}
DEFINE_IID!(IID_IPrintWorkflowSpoolStreamContent, 1927634638, 58374, 19316, 132, 225, 63, 243, 253, 205, 175, 112);
RT_INTERFACE!{interface IPrintWorkflowSpoolStreamContent(IPrintWorkflowSpoolStreamContentVtbl, IPrintWorkflowSpoolStreamContent_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSpoolStreamContent] {
    #[cfg(feature="windows-storage")] fn GetInputStream(&self, out: *mut <crate::windows::storage::streams::IInputStream as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowSpoolStreamContent {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_input_stream(&self) -> Result<Option<crate::windows::storage::streams::IInputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetInputStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IInputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSpoolStreamContent: IPrintWorkflowSpoolStreamContent}
DEFINE_IID!(IID_IPrintWorkflowStreamTarget, 2990258820, 34149, 18571, 152, 57, 28, 158, 124, 122, 169, 22);
RT_INTERFACE!{interface IPrintWorkflowStreamTarget(IPrintWorkflowStreamTargetVtbl, IPrintWorkflowStreamTarget_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowStreamTarget] {
    #[cfg(feature="windows-storage")] fn GetOutputStream(&self, out: *mut <crate::windows::storage::streams::IOutputStream as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowStreamTarget {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_output_stream(&self) -> Result<Option<crate::windows::storage::streams::IOutputStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetOutputStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(crate::windows::storage::streams::IOutputStream::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowStreamTarget: IPrintWorkflowStreamTarget}
DEFINE_IID!(IID_IPrintWorkflowSubmittedEventArgs, 987564609, 14228, 21865, 92, 135, 64, 232, 255, 114, 15, 131);
RT_INTERFACE!{interface IPrintWorkflowSubmittedEventArgs(IPrintWorkflowSubmittedEventArgsVtbl, IPrintWorkflowSubmittedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSubmittedEventArgs] {
    fn get_Operation(&self, out: *mut <PrintWorkflowSubmittedOperation as RtType>::Abi) -> HRESULT,
    fn GetTarget(&self, jobPrintTicket: <super::printticket::WorkflowPrintTicket as RtType>::Abi, out: *mut <PrintWorkflowTarget as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowSubmittedEventArgs {
    #[inline] pub fn get_operation(&self) -> Result<Option<PrintWorkflowSubmittedOperation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Operation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowSubmittedOperation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_target(&self, jobPrintTicket: &super::printticket::WorkflowPrintTicket) -> Result<Option<PrintWorkflowTarget>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTarget)(self.get_abi() as *const _ as *mut _, jobPrintTicket.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowTarget::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSubmittedEventArgs: IPrintWorkflowSubmittedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowSubmittedOperation, 776888854, 15329, 24335, 92, 129, 165, 162, 189, 78, 171, 14);
RT_INTERFACE!{interface IPrintWorkflowSubmittedOperation(IPrintWorkflowSubmittedOperationVtbl, IPrintWorkflowSubmittedOperation_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowSubmittedOperation] {
    fn Complete(&self, status: PrintWorkflowSubmittedStatus) -> HRESULT,
    fn get_Configuration(&self, out: *mut <PrintWorkflowConfiguration as RtType>::Abi) -> HRESULT,
    fn get_XpsContent(&self, out: *mut <PrintWorkflowSourceContent as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowSubmittedOperation {
    #[inline] pub fn complete(&self, status: PrintWorkflowSubmittedStatus) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Complete)(self.get_abi() as *const _ as *mut _, status);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_configuration(&self) -> Result<Option<PrintWorkflowConfiguration>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Configuration)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowConfiguration::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_xps_content(&self) -> Result<Option<PrintWorkflowSourceContent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_XpsContent)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowSourceContent::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowSubmittedOperation: IPrintWorkflowSubmittedOperation}
RT_ENUM! { enum PrintWorkflowSubmittedStatus: i32 {
    Succeeded = 0, Canceled = 1, Failed = 2,
}}
DEFINE_IID!(IID_IPrintWorkflowTarget, 702162796, 2675, 23277, 79, 61, 151, 13, 50, 81, 240, 87);
RT_INTERFACE!{interface IPrintWorkflowTarget(IPrintWorkflowTargetVtbl, IPrintWorkflowTarget_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowTarget] {
    fn get_TargetAsStream(&self, out: *mut <PrintWorkflowStreamTarget as RtType>::Abi) -> HRESULT,
    fn get_TargetAsXpsObjectModelPackage(&self, out: *mut <PrintWorkflowObjectModelTargetPackage as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowTarget {
    #[inline] pub fn get_target_as_stream(&self) -> Result<Option<PrintWorkflowStreamTarget>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TargetAsStream)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowStreamTarget::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_target_as_xps_object_model_package(&self) -> Result<Option<PrintWorkflowObjectModelTargetPackage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TargetAsXpsObjectModelPackage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowObjectModelTargetPackage::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowTarget: IPrintWorkflowTarget}
DEFINE_IID!(IID_IPrintWorkflowTriggerDetails, 1463408744, 40326, 16466, 176, 203, 243, 16, 190, 205, 89, 187);
RT_INTERFACE!{interface IPrintWorkflowTriggerDetails(IPrintWorkflowTriggerDetailsVtbl, IPrintWorkflowTriggerDetails_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowTriggerDetails] {
    fn get_PrintWorkflowSession(&self, out: *mut <PrintWorkflowBackgroundSession as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowTriggerDetails {
    #[inline] pub fn get_print_workflow_session(&self) -> Result<Option<PrintWorkflowBackgroundSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PrintWorkflowSession)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowBackgroundSession::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowTriggerDetails: IPrintWorkflowTriggerDetails}
DEFINE_IID!(IID_IPrintWorkflowUIActivatedEventArgs, 3163194445, 2539, 22342, 114, 166, 141, 200, 181, 237, 190, 155);
RT_INTERFACE!{interface IPrintWorkflowUIActivatedEventArgs(IPrintWorkflowUIActivatedEventArgsVtbl, IPrintWorkflowUIActivatedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowUIActivatedEventArgs] {
    fn get_PrintWorkflowSession(&self, out: *mut <PrintWorkflowForegroundSession as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowUIActivatedEventArgs {
    #[inline] pub fn get_print_workflow_session(&self) -> Result<Option<PrintWorkflowForegroundSession>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PrintWorkflowSession)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowForegroundSession::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowUIActivatedEventArgs: IPrintWorkflowUIActivatedEventArgs}
DEFINE_IID!(IID_IPrintWorkflowXpsDataAvailableEventArgs, 1293009713, 21713, 17230, 190, 14, 130, 197, 250, 88, 229, 178);
RT_INTERFACE!{interface IPrintWorkflowXpsDataAvailableEventArgs(IPrintWorkflowXpsDataAvailableEventArgsVtbl, IPrintWorkflowXpsDataAvailableEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrintWorkflowXpsDataAvailableEventArgs] {
    fn get_Operation(&self, out: *mut <PrintWorkflowSubmittedOperation as RtType>::Abi) -> HRESULT,
    fn GetDeferral(&self, out: *mut <foundation::Deferral as RtType>::Abi) -> HRESULT
}}
impl IPrintWorkflowXpsDataAvailableEventArgs {
    #[inline] pub fn get_operation(&self) -> Result<Option<PrintWorkflowSubmittedOperation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Operation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PrintWorkflowSubmittedOperation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_deferral(&self) -> Result<Option<foundation::Deferral>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetDeferral)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::Deferral::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class PrintWorkflowXpsDataAvailableEventArgs: IPrintWorkflowXpsDataAvailableEventArgs}
} // Windows.Graphics.Printing.Workflow
} // Windows.Graphics.Printing
pub mod printing3d { // Windows.Graphics.Printing3D
use crate::prelude::*;
DEFINE_IID!(IID_IPrint3DManager, 1294977802, 29542, 18801, 139, 213, 23, 196, 227, 232, 198, 192);
RT_INTERFACE!{interface IPrint3DManager(IPrint3DManagerVtbl, IPrint3DManager_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DManager] {
    fn add_TaskRequested(&self, eventHandler: <foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_TaskRequested(&self, token: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrint3DManager {
    #[inline] pub fn add_task_requested(&self, eventHandler: &foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_TaskRequested)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_task_requested(&self, token: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_TaskRequested)(self.get_abi() as *const _ as *mut _, token);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DManager: IPrint3DManager}
impl RtActivatable<IPrint3DManagerStatics> for Print3DManager {}
impl Print3DManager {
    #[inline] pub fn get_for_current_view() -> Result<Option<Print3DManager>> {
        <Self as RtActivatable<IPrint3DManagerStatics>>::get_activation_factory().get_for_current_view()
    }
    #[inline] pub fn show_print_ui_async() -> Result<foundation::IAsyncOperation<bool>> {
        <Self as RtActivatable<IPrint3DManagerStatics>>::get_activation_factory().show_print_ui_async()
    }
}
DEFINE_CLSID!(Print3DManager(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,51,68,77,97,110,97,103,101,114,0]) [CLSID_Print3DManager]);
DEFINE_IID!(IID_IPrint3DManagerStatics, 250727166, 43437, 19464, 169, 23, 29, 31, 134, 62, 171, 203);
RT_INTERFACE!{static interface IPrint3DManagerStatics(IPrint3DManagerStaticsVtbl, IPrint3DManagerStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DManagerStatics] {
    fn GetForCurrentView(&self, out: *mut <Print3DManager as RtType>::Abi) -> HRESULT,
    fn ShowPrintUIAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT
}}
impl IPrint3DManagerStatics {
    #[inline] pub fn get_for_current_view(&self) -> Result<Option<Print3DManager>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetForCurrentView)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Print3DManager::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn show_print_ui_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ShowPrintUIAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrint3DTask, 2363740288, 8472, 19496, 128, 222, 244, 38, 215, 1, 145, 174);
RT_INTERFACE!{interface IPrint3DTask(IPrint3DTaskVtbl, IPrint3DTask_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTask] {
    fn get_Source(&self, out: *mut <Printing3D3MFPackage as RtType>::Abi) -> HRESULT,
    fn add_Submitting(&self, eventHandler: <foundation::TypedEventHandler<Print3DTask, IInspectable> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Submitting(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_Completed(&self, eventHandler: <foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_Completed(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT,
    fn add_SourceChanged(&self, eventHandler: <foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs> as RtType>::Abi, out: *mut foundation::EventRegistrationToken) -> HRESULT,
    fn remove_SourceChanged(&self, eventCookie: foundation::EventRegistrationToken) -> HRESULT
}}
impl IPrint3DTask {
    #[inline] pub fn get_source(&self) -> Result<Option<Printing3D3MFPackage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Source)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3D3MFPackage::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn add_submitting(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, IInspectable>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Submitting)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_submitting(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Submitting)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_completed(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_Completed)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_completed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_Completed)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn add_source_changed(&self, eventHandler: &foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>) -> Result<foundation::EventRegistrationToken> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).add_SourceChanged)(self.get_abi() as *const _ as *mut _, eventHandler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn remove_source_changed(&self, eventCookie: foundation::EventRegistrationToken) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).remove_SourceChanged)(self.get_abi() as *const _ as *mut _, eventCookie);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTask: IPrint3DTask}
DEFINE_IID!(IID_IPrint3DTaskCompletedEventArgs, 3424195759, 9748, 20253, 172, 204, 214, 252, 79, 218, 84, 85);
RT_INTERFACE!{interface IPrint3DTaskCompletedEventArgs(IPrint3DTaskCompletedEventArgsVtbl, IPrint3DTaskCompletedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskCompletedEventArgs] {
    fn get_Completion(&self, out: *mut Print3DTaskCompletion) -> HRESULT,
    fn get_ExtendedStatus(&self, out: *mut Print3DTaskDetail) -> HRESULT
}}
impl IPrint3DTaskCompletedEventArgs {
    #[inline] pub fn get_completion(&self) -> Result<Print3DTaskCompletion> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Completion)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_extended_status(&self) -> Result<Print3DTaskDetail> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ExtendedStatus)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IPrint3DTaskRequest(IPrint3DTaskRequestVtbl, IPrint3DTaskRequest_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskRequest] {
    fn CreateTask(&self, title: HSTRING, printerId: HSTRING, handler: <Print3DTaskSourceRequestedHandler as RtType>::Abi, out: *mut <Print3DTask as RtType>::Abi) -> HRESULT
}}
impl IPrint3DTaskRequest {
    #[inline] pub fn create_task(&self, title: &HStringArg, printerId: &HStringArg, handler: &Print3DTaskSourceRequestedHandler) -> Result<Option<Print3DTask>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateTask)(self.get_abi() as *const _ as *mut _, title.get(), printerId.get(), handler.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Print3DTask::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskRequest: IPrint3DTaskRequest}
DEFINE_IID!(IID_IPrint3DTaskRequestedEventArgs, 353154943, 6341, 16599, 159, 64, 250, 179, 9, 110, 5, 169);
RT_INTERFACE!{interface IPrint3DTaskRequestedEventArgs(IPrint3DTaskRequestedEventArgsVtbl, IPrint3DTaskRequestedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskRequestedEventArgs] {
    fn get_Request(&self, out: *mut <Print3DTaskRequest as RtType>::Abi) -> HRESULT
}}
impl IPrint3DTaskRequestedEventArgs {
    #[inline] pub fn get_request(&self) -> Result<Option<Print3DTaskRequest>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Request)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Print3DTaskRequest::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskRequestedEventArgs: IPrint3DTaskRequestedEventArgs}
DEFINE_IID!(IID_IPrint3DTaskSourceChangedEventArgs, 1540175023, 9449, 19472, 141, 7, 20, 195, 70, 186, 63, 207);
RT_INTERFACE!{interface IPrint3DTaskSourceChangedEventArgs(IPrint3DTaskSourceChangedEventArgsVtbl, IPrint3DTaskSourceChangedEventArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskSourceChangedEventArgs] {
    fn get_Source(&self, out: *mut <Printing3D3MFPackage as RtType>::Abi) -> HRESULT
}}
impl IPrint3DTaskSourceChangedEventArgs {
    #[inline] pub fn get_source(&self) -> Result<Option<Printing3D3MFPackage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Source)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3D3MFPackage::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskSourceChangedEventArgs: IPrint3DTaskSourceChangedEventArgs}
DEFINE_IID!(IID_IPrint3DTaskSourceRequestedArgs, 3346832058, 9391, 16973, 163, 191, 146, 37, 12, 53, 86, 2);
RT_INTERFACE!{interface IPrint3DTaskSourceRequestedArgs(IPrint3DTaskSourceRequestedArgsVtbl, IPrint3DTaskSourceRequestedArgs_Abi): IInspectable(IInspectableVtbl) [IID_IPrint3DTaskSourceRequestedArgs] {
    fn SetSource(&self, source: <Printing3D3MFPackage as RtType>::Abi) -> HRESULT
}}
impl IPrint3DTaskSourceRequestedArgs {
    #[inline] pub fn set_source(&self, source: &Printing3D3MFPackage) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetSource)(self.get_abi() as *const _ as *mut _, source.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Print3DTaskSourceRequestedArgs: IPrint3DTaskSourceRequestedArgs}
DEFINE_IID!(IID_Print3DTaskSourceRequestedHandler, 3910622832, 51479, 18142, 187, 81, 217, 169, 77, 179, 113, 31);
RT_DELEGATE!{delegate Print3DTaskSourceRequestedHandler(Print3DTaskSourceRequestedHandlerVtbl, Print3DTaskSourceRequestedHandler_Abi, Print3DTaskSourceRequestedHandlerImpl) [IID_Print3DTaskSourceRequestedHandler] {
    fn Invoke(&self, args: <Print3DTaskSourceRequestedArgs as RtType>::Abi) -> HRESULT
}}
impl Print3DTaskSourceRequestedHandler {
    #[inline] pub fn invoke(&self, args: &Print3DTaskSourceRequestedArgs) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, args.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3D3MFPackage, 4132296136, 10935, 17833, 161, 183, 38, 126, 148, 141, 91, 24);
RT_INTERFACE!{interface IPrinting3D3MFPackage(IPrinting3D3MFPackageVtbl, IPrinting3D3MFPackage_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackage] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn SaveAsync(&self, out: *mut <foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_PrintTicket(&self, out: *mut <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy2(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_PrintTicket(&self, value: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy3(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_ModelPart(&self, out: *mut <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy4(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_ModelPart(&self, value: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn put_Thumbnail(&self, value: <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn get_Textures(&self, out: *mut <foundation::collections::IVector<Printing3DTextureResource> as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy8(&self) -> (),
    #[cfg(feature="windows-storage")] fn LoadModelFromPackageAsync(&self, value: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<Printing3DModel> as RtType>::Abi) -> HRESULT,
    fn SaveModelToPackageAsync(&self, value: <Printing3DModel as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IPrinting3D3MFPackage {
    #[cfg(feature="windows-storage")] #[inline] pub fn save_async(&self) -> Result<foundation::IAsyncOperation<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SaveAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_print_ticket(&self) -> Result<Option<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PrintTicket)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_print_ticket(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_PrintTicket)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_model_part(&self) -> Result<Option<super::super::storage::streams::IRandomAccessStream>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ModelPart)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IRandomAccessStream::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_model_part(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ModelPart)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail(&self) -> Result<Option<Printing3DTextureResource>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Thumbnail)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DTextureResource::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbnail(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Thumbnail)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_textures(&self) -> Result<Option<foundation::collections::IVector<Printing3DTextureResource>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Textures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_model_from_package_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<Printing3DModel>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadModelFromPackageAsync)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn save_model_to_package_async(&self, value: &Printing3DModel) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SaveModelToPackageAsync)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3D3MFPackage: IPrinting3D3MFPackage}
impl RtActivatable<IPrinting3D3MFPackageStatics> for Printing3D3MFPackage {}
impl RtActivatable<IActivationFactory> for Printing3D3MFPackage {}
impl Printing3D3MFPackage {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_async(value: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<Printing3D3MFPackage>> {
        <Self as RtActivatable<IPrinting3D3MFPackageStatics>>::get_activation_factory().load_async(value)
    }
}
DEFINE_CLSID!(Printing3D3MFPackage(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,51,77,70,80,97,99,107,97,103,101,0]) [CLSID_Printing3D3MFPackage]);
DEFINE_IID!(IID_IPrinting3D3MFPackage2, 2522643140, 37835, 17456, 146, 184, 120, 156, 212, 84, 248, 131);
RT_INTERFACE!{interface IPrinting3D3MFPackage2(IPrinting3D3MFPackage2Vtbl, IPrinting3D3MFPackage2_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackage2] {
    fn get_Compression(&self, out: *mut Printing3DPackageCompression) -> HRESULT,
    fn put_Compression(&self, value: Printing3DPackageCompression) -> HRESULT
}}
impl IPrinting3D3MFPackage2 {
    #[inline] pub fn get_compression(&self) -> Result<Printing3DPackageCompression> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Compression)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_compression(&self, value: Printing3DPackageCompression) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Compression)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3D3MFPackageStatics, 1884871087, 31386, 18311, 184, 23, 246, 244, 89, 33, 72, 35);
RT_INTERFACE!{static interface IPrinting3D3MFPackageStatics(IPrinting3D3MFPackageStaticsVtbl, IPrinting3D3MFPackageStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3D3MFPackageStatics] {
    #[cfg(feature="windows-storage")] fn LoadAsync(&self, value: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<Printing3D3MFPackage> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3D3MFPackageStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_async(&self, value: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<Printing3D3MFPackage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadAsync)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DBaseMaterial, 3505448771, 50444, 19403, 157, 4, 252, 22, 173, 206, 162, 201);
RT_INTERFACE!{interface IPrinting3DBaseMaterial(IPrinting3DBaseMaterialVtbl, IPrinting3DBaseMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterial] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_Color(&self, out: *mut <Printing3DColorMaterial as RtType>::Abi) -> HRESULT,
    fn put_Color(&self, value: <Printing3DColorMaterial as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DBaseMaterial {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Name)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_color(&self) -> Result<Option<Printing3DColorMaterial>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Color)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DColorMaterial::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_color(&self, value: &Printing3DColorMaterial) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Color)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
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
RT_INTERFACE!{interface IPrinting3DBaseMaterialGroup(IPrinting3DBaseMaterialGroupVtbl, IPrinting3DBaseMaterialGroup_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialGroup] {
    fn get_Bases(&self, out: *mut <foundation::collections::IVector<Printing3DBaseMaterial> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DBaseMaterialGroup {
    #[inline] pub fn get_bases(&self) -> Result<Option<foundation::collections::IVector<Printing3DBaseMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Bases)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DBaseMaterialGroup: IPrinting3DBaseMaterialGroup}
impl RtActivatable<IPrinting3DBaseMaterialGroupFactory> for Printing3DBaseMaterialGroup {}
impl Printing3DBaseMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<Printing3DBaseMaterialGroup> {
        <Self as RtActivatable<IPrinting3DBaseMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DBaseMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,66,97,115,101,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DBaseMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DBaseMaterialGroupFactory, 1544898268, 34455, 16787, 151, 107, 132, 187, 65, 22, 229, 191);
RT_INTERFACE!{static interface IPrinting3DBaseMaterialGroupFactory(IPrinting3DBaseMaterialGroupFactoryVtbl, IPrinting3DBaseMaterialGroupFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut <Printing3DBaseMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DBaseMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<Printing3DBaseMaterialGroup> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(Printing3DBaseMaterialGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DBaseMaterialStatics, 2170177468, 14154, 18285, 190, 146, 62, 207, 209, 203, 151, 118);
RT_INTERFACE!{static interface IPrinting3DBaseMaterialStatics(IPrinting3DBaseMaterialStaticsVtbl, IPrinting3DBaseMaterialStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DBaseMaterialStatics] {
    fn get_Abs(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Pla(&self, out: *mut HSTRING) -> HRESULT
}}
impl IPrinting3DBaseMaterialStatics {
    #[inline] pub fn get_abs(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Abs)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_pla(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Pla)(self.get_abi() as *const _ as *mut _, &mut out);
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
RT_INTERFACE!{interface IPrinting3DColorMaterial(IPrinting3DColorMaterialVtbl, IPrinting3DColorMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterial] {
    fn get_Value(&self, out: *mut u32) -> HRESULT,
    fn put_Value(&self, value: u32) -> HRESULT
}}
impl IPrinting3DColorMaterial {
    #[inline] pub fn get_value(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Value)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DColorMaterial: IPrinting3DColorMaterial}
impl RtActivatable<IActivationFactory> for Printing3DColorMaterial {}
DEFINE_CLSID!(Printing3DColorMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,108,111,114,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DColorMaterial]);
DEFINE_IID!(IID_IPrinting3DColorMaterial2, 4205897810, 2799, 17641, 157, 221, 54, 238, 234, 90, 205, 68);
RT_INTERFACE!{interface IPrinting3DColorMaterial2(IPrinting3DColorMaterial2Vtbl, IPrinting3DColorMaterial2_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterial2] {
    #[cfg(feature="windows-ui")] fn get_Color(&self, out: *mut super::super::ui::Color) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_Color(&self, value: super::super::ui::Color) -> HRESULT
}}
impl IPrinting3DColorMaterial2 {
    #[cfg(feature="windows-ui")] #[inline] pub fn get_color(&self) -> Result<super::super::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Color)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_color(&self, value: super::super::ui::Color) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Color)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DColorMaterialGroup, 1731536, 43743, 16934, 175, 233, 243, 105, 160, 180, 80, 4);
RT_INTERFACE!{interface IPrinting3DColorMaterialGroup(IPrinting3DColorMaterialGroupVtbl, IPrinting3DColorMaterialGroup_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterialGroup] {
    fn get_Colors(&self, out: *mut <foundation::collections::IVector<Printing3DColorMaterial> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DColorMaterialGroup {
    #[inline] pub fn get_colors(&self) -> Result<Option<foundation::collections::IVector<Printing3DColorMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Colors)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DColorMaterialGroup: IPrinting3DColorMaterialGroup}
impl RtActivatable<IPrinting3DColorMaterialGroupFactory> for Printing3DColorMaterialGroup {}
impl Printing3DColorMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<Printing3DColorMaterialGroup> {
        <Self as RtActivatable<IPrinting3DColorMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DColorMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,108,111,114,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DColorMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DColorMaterialGroupFactory, 1909689709, 45546, 19035, 188, 84, 25, 198, 95, 61, 240, 68);
RT_INTERFACE!{static interface IPrinting3DColorMaterialGroupFactory(IPrinting3DColorMaterialGroupFactoryVtbl, IPrinting3DColorMaterialGroupFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DColorMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut <Printing3DColorMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DColorMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<Printing3DColorMaterialGroup> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(Printing3DColorMaterialGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DComponent, 2116581445, 49023, 19675, 162, 127, 48, 160, 20, 55, 254, 222);
RT_INTERFACE!{interface IPrinting3DComponent(IPrinting3DComponentVtbl, IPrinting3DComponent_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DComponent] {
    fn get_Mesh(&self, out: *mut <Printing3DMesh as RtType>::Abi) -> HRESULT,
    fn put_Mesh(&self, value: <Printing3DMesh as RtType>::Abi) -> HRESULT,
    fn get_Components(&self, out: *mut <foundation::collections::IVector<Printing3DComponentWithMatrix> as RtType>::Abi) -> HRESULT,
    fn get_Thumbnail(&self, out: *mut <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn put_Thumbnail(&self, value: <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn get_Type(&self, out: *mut Printing3DObjectType) -> HRESULT,
    fn put_Type(&self, value: Printing3DObjectType) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT,
    fn get_PartNumber(&self, out: *mut HSTRING) -> HRESULT,
    fn put_PartNumber(&self, value: HSTRING) -> HRESULT
}}
impl IPrinting3DComponent {
    #[inline] pub fn get_mesh(&self) -> Result<Option<Printing3DMesh>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Mesh)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DMesh::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_mesh(&self, value: &Printing3DMesh) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Mesh)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_components(&self) -> Result<Option<foundation::collections::IVector<Printing3DComponentWithMatrix>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Components)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_thumbnail(&self) -> Result<Option<Printing3DTextureResource>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Thumbnail)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DTextureResource::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_thumbnail(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Thumbnail)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_type(&self) -> Result<Printing3DObjectType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Type)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_type(&self, value: Printing3DObjectType) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Type)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Name)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_part_number(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PartNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_part_number(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_PartNumber)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DComponent: IPrinting3DComponent}
impl RtActivatable<IActivationFactory> for Printing3DComponent {}
DEFINE_CLSID!(Printing3DComponent(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,110,101,110,116,0]) [CLSID_Printing3DComponent]);
DEFINE_IID!(IID_IPrinting3DComponentWithMatrix, 846852917, 3824, 17771, 154, 33, 73, 190, 190, 139, 81, 194);
RT_INTERFACE!{interface IPrinting3DComponentWithMatrix(IPrinting3DComponentWithMatrixVtbl, IPrinting3DComponentWithMatrix_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DComponentWithMatrix] {
    fn get_Component(&self, out: *mut <Printing3DComponent as RtType>::Abi) -> HRESULT,
    fn put_Component(&self, value: <Printing3DComponent as RtType>::Abi) -> HRESULT,
    fn get_Matrix(&self, out: *mut foundation::numerics::Matrix4x4) -> HRESULT,
    fn put_Matrix(&self, value: foundation::numerics::Matrix4x4) -> HRESULT
}}
impl IPrinting3DComponentWithMatrix {
    #[inline] pub fn get_component(&self) -> Result<Option<Printing3DComponent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Component)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DComponent::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_component(&self, value: &Printing3DComponent) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Component)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_matrix(&self) -> Result<foundation::numerics::Matrix4x4> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Matrix)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_matrix(&self, value: foundation::numerics::Matrix4x4) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Matrix)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DComponentWithMatrix: IPrinting3DComponentWithMatrix}
impl RtActivatable<IActivationFactory> for Printing3DComponentWithMatrix {}
DEFINE_CLSID!(Printing3DComponentWithMatrix(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,110,101,110,116,87,105,116,104,77,97,116,114,105,120,0]) [CLSID_Printing3DComponentWithMatrix]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterial, 1176647901, 22062, 20332, 136, 45, 244, 216, 65, 253, 99, 199);
RT_INTERFACE!{interface IPrinting3DCompositeMaterial(IPrinting3DCompositeMaterialVtbl, IPrinting3DCompositeMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterial] {
    fn get_Values(&self, out: *mut <foundation::collections::IVector<f64> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DCompositeMaterial {
    #[inline] pub fn get_values(&self) -> Result<Option<foundation::collections::IVector<f64>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Values)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DCompositeMaterial: IPrinting3DCompositeMaterial}
impl RtActivatable<IActivationFactory> for Printing3DCompositeMaterial {}
DEFINE_CLSID!(Printing3DCompositeMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,115,105,116,101,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DCompositeMaterial]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroup, 2375314011, 16625, 18797, 165, 251, 52, 10, 90, 103, 142, 48);
RT_INTERFACE!{interface IPrinting3DCompositeMaterialGroup(IPrinting3DCompositeMaterialGroupVtbl, IPrinting3DCompositeMaterialGroup_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroup] {
    fn get_Composites(&self, out: *mut <foundation::collections::IVector<Printing3DCompositeMaterial> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT,
    fn get_MaterialIndices(&self, out: *mut <foundation::collections::IVector<u32> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroup {
    #[inline] pub fn get_composites(&self) -> Result<Option<foundation::collections::IVector<Printing3DCompositeMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Composites)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_material_indices(&self) -> Result<Option<foundation::collections::IVector<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DCompositeMaterialGroup: IPrinting3DCompositeMaterialGroup}
impl RtActivatable<IPrinting3DCompositeMaterialGroupFactory> for Printing3DCompositeMaterialGroup {}
impl Printing3DCompositeMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<Printing3DCompositeMaterialGroup> {
        <Self as RtActivatable<IPrinting3DCompositeMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DCompositeMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,67,111,109,112,111,115,105,116,101,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DCompositeMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroup2, 115895650, 32059, 16865, 148, 76, 186, 253, 228, 85, 84, 131);
RT_INTERFACE!{interface IPrinting3DCompositeMaterialGroup2(IPrinting3DCompositeMaterialGroup2Vtbl, IPrinting3DCompositeMaterialGroup2_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroup2] {
    fn get_BaseMaterialGroup(&self, out: *mut <Printing3DBaseMaterialGroup as RtType>::Abi) -> HRESULT,
    fn put_BaseMaterialGroup(&self, value: <Printing3DBaseMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroup2 {
    #[inline] pub fn get_base_material_group(&self) -> Result<Option<Printing3DBaseMaterialGroup>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BaseMaterialGroup)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DBaseMaterialGroup::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_base_material_group(&self, value: &Printing3DBaseMaterialGroup) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_BaseMaterialGroup)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DCompositeMaterialGroupFactory, 3499019539, 37631, 17322, 166, 39, 141, 67, 194, 44, 129, 126);
RT_INTERFACE!{static interface IPrinting3DCompositeMaterialGroupFactory(IPrinting3DCompositeMaterialGroupFactoryVtbl, IPrinting3DCompositeMaterialGroupFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DCompositeMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut <Printing3DCompositeMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DCompositeMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<Printing3DCompositeMaterialGroup> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(Printing3DCompositeMaterialGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DFaceReductionOptions, 3154039703, 11636, 18167, 190, 133, 153, 166, 123, 187, 102, 41);
RT_INTERFACE!{interface IPrinting3DFaceReductionOptions(IPrinting3DFaceReductionOptionsVtbl, IPrinting3DFaceReductionOptions_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DFaceReductionOptions] {
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
        let hr = ((*self.get_abi().lpVtbl).get_MaxReductionArea)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_reduction_area(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_MaxReductionArea)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_target_triangle_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TargetTriangleCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_target_triangle_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TargetTriangleCount)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_max_edge_length(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxEdgeLength)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_edge_length(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_MaxEdgeLength)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DFaceReductionOptions: IPrinting3DFaceReductionOptions}
impl RtActivatable<IActivationFactory> for Printing3DFaceReductionOptions {}
DEFINE_CLSID!(Printing3DFaceReductionOptions(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,70,97,99,101,82,101,100,117,99,116,105,111,110,79,112,116,105,111,110,115,0]) [CLSID_Printing3DFaceReductionOptions]);
DEFINE_IID!(IID_IPrinting3DMaterial, 932033110, 60770, 18770, 184, 91, 3, 86, 125, 124, 70, 94);
RT_INTERFACE!{interface IPrinting3DMaterial(IPrinting3DMaterialVtbl, IPrinting3DMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMaterial] {
    fn get_BaseGroups(&self, out: *mut <foundation::collections::IVector<Printing3DBaseMaterialGroup> as RtType>::Abi) -> HRESULT,
    fn get_ColorGroups(&self, out: *mut <foundation::collections::IVector<Printing3DColorMaterialGroup> as RtType>::Abi) -> HRESULT,
    fn get_Texture2CoordGroups(&self, out: *mut <foundation::collections::IVector<Printing3DTexture2CoordMaterialGroup> as RtType>::Abi) -> HRESULT,
    fn get_CompositeGroups(&self, out: *mut <foundation::collections::IVector<Printing3DCompositeMaterialGroup> as RtType>::Abi) -> HRESULT,
    fn get_MultiplePropertyGroups(&self, out: *mut <foundation::collections::IVector<Printing3DMultiplePropertyMaterialGroup> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DMaterial {
    #[inline] pub fn get_base_groups(&self) -> Result<Option<foundation::collections::IVector<Printing3DBaseMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BaseGroups)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_color_groups(&self) -> Result<Option<foundation::collections::IVector<Printing3DColorMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ColorGroups)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_texture2_coord_groups(&self) -> Result<Option<foundation::collections::IVector<Printing3DTexture2CoordMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Texture2CoordGroups)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_composite_groups(&self) -> Result<Option<foundation::collections::IVector<Printing3DCompositeMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_CompositeGroups)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_multiple_property_groups(&self) -> Result<Option<foundation::collections::IVector<Printing3DMultiplePropertyMaterialGroup>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MultiplePropertyGroups)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMaterial: IPrinting3DMaterial}
impl RtActivatable<IActivationFactory> for Printing3DMaterial {}
DEFINE_CLSID!(Printing3DMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DMaterial]);
DEFINE_IID!(IID_IPrinting3DMesh, 422482140, 552, 11777, 188, 32, 197, 41, 12, 191, 50, 196);
RT_INTERFACE!{interface IPrinting3DMesh(IPrinting3DMeshVtbl, IPrinting3DMesh_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMesh] {
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
    #[cfg(feature="windows-storage")] fn GetVertexPositions(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn CreateVertexPositions(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy14(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetVertexNormals(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn CreateVertexNormals(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy16(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetTriangleIndices(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn CreateTriangleIndices(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy18(&self) -> (),
    #[cfg(feature="windows-storage")] fn GetTriangleMaterialIndices(&self, out: *mut <super::super::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    fn CreateTriangleMaterialIndices(&self, value: u32) -> HRESULT,
    fn get_BufferDescriptionSet(&self, out: *mut <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn get_BufferSet(&self, out: *mut <foundation::collections::IPropertySet as RtType>::Abi) -> HRESULT,
    fn VerifyAsync(&self, value: Printing3DMeshVerificationMode, out: *mut <foundation::IAsyncOperation<Printing3DMeshVerificationResult> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DMesh {
    #[inline] pub fn get_vertex_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_VertexCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_VertexCount)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_index_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IndexCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_index_count(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IndexCount)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_positions_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_VertexPositionsDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_positions_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_VertexPositionsDescription)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_vertex_normals_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_VertexNormalsDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_vertex_normals_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_VertexNormalsDescription)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_indices_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TriangleIndicesDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_triangle_indices_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TriangleIndicesDescription)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_triangle_material_indices_description(&self) -> Result<Printing3DBufferDescription> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TriangleMaterialIndicesDescription)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_triangle_material_indices_description(&self, value: Printing3DBufferDescription) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TriangleMaterialIndicesDescription)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_vertex_positions(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVertexPositions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_vertex_positions(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CreateVertexPositions)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_vertex_normals(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetVertexNormals)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_vertex_normals(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CreateVertexNormals)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_triangle_indices(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTriangleIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_triangle_indices(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CreateTriangleIndices)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn get_triangle_material_indices(&self) -> Result<Option<super::super::storage::streams::IBuffer>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTriangleMaterialIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IBuffer::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_triangle_material_indices(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).CreateTriangleMaterialIndices)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_buffer_description_set(&self) -> Result<Option<foundation::collections::IPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BufferDescriptionSet)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IPropertySet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_buffer_set(&self) -> Result<Option<foundation::collections::IPropertySet>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_BufferSet)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IPropertySet::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn verify_async(&self, value: Printing3DMeshVerificationMode) -> Result<foundation::IAsyncOperation<Printing3DMeshVerificationResult>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).VerifyAsync)(self.get_abi() as *const _ as *mut _, value, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMesh: IPrinting3DMesh}
impl RtActivatable<IActivationFactory> for Printing3DMesh {}
DEFINE_CLSID!(Printing3DMesh(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,101,115,104,0]) [CLSID_Printing3DMesh]);
RT_ENUM! { enum Printing3DMeshVerificationMode: i32 {
    FindFirstError = 0, FindAllErrors = 1,
}}
DEFINE_IID!(IID_IPrinting3DMeshVerificationResult, 425095610, 59706, 20106, 164, 111, 222, 168, 232, 82, 25, 126);
RT_INTERFACE!{interface IPrinting3DMeshVerificationResult(IPrinting3DMeshVerificationResultVtbl, IPrinting3DMeshVerificationResult_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMeshVerificationResult] {
    fn get_IsValid(&self, out: *mut bool) -> HRESULT,
    fn get_NonmanifoldTriangles(&self, out: *mut <foundation::collections::IVectorView<u32> as RtType>::Abi) -> HRESULT,
    fn get_ReversedNormalTriangles(&self, out: *mut <foundation::collections::IVectorView<u32> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DMeshVerificationResult {
    #[inline] pub fn get_is_valid(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsValid)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_nonmanifold_triangles(&self) -> Result<Option<foundation::collections::IVectorView<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NonmanifoldTriangles)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reversed_normal_triangles(&self) -> Result<Option<foundation::collections::IVectorView<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ReversedNormalTriangles)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMeshVerificationResult: IPrinting3DMeshVerificationResult}
DEFINE_IID!(IID_IPrinting3DModel, 755052272, 21243, 37274, 119, 176, 75, 26, 59, 128, 50, 79);
RT_INTERFACE!{interface IPrinting3DModel(IPrinting3DModelVtbl, IPrinting3DModel_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DModel] {
    fn get_Unit(&self, out: *mut Printing3DModelUnit) -> HRESULT,
    fn put_Unit(&self, value: Printing3DModelUnit) -> HRESULT,
    fn get_Textures(&self, out: *mut <foundation::collections::IVector<Printing3DModelTexture> as RtType>::Abi) -> HRESULT,
    fn get_Meshes(&self, out: *mut <foundation::collections::IVector<Printing3DMesh> as RtType>::Abi) -> HRESULT,
    fn get_Components(&self, out: *mut <foundation::collections::IVector<Printing3DComponent> as RtType>::Abi) -> HRESULT,
    fn get_Material(&self, out: *mut <Printing3DMaterial as RtType>::Abi) -> HRESULT,
    fn put_Material(&self, value: <Printing3DMaterial as RtType>::Abi) -> HRESULT,
    fn get_Build(&self, out: *mut <Printing3DComponent as RtType>::Abi) -> HRESULT,
    fn put_Build(&self, value: <Printing3DComponent as RtType>::Abi) -> HRESULT,
    fn get_Version(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Version(&self, value: HSTRING) -> HRESULT,
    fn get_RequiredExtensions(&self, out: *mut <foundation::collections::IVector<HString> as RtType>::Abi) -> HRESULT,
    fn get_Metadata(&self, out: *mut <foundation::collections::IMap<HString, HString> as RtType>::Abi) -> HRESULT,
    fn RepairAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn Clone(&self, out: *mut <Printing3DModel as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DModel {
    #[inline] pub fn get_unit(&self) -> Result<Printing3DModelUnit> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Unit)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_unit(&self, value: Printing3DModelUnit) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Unit)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_textures(&self) -> Result<Option<foundation::collections::IVector<Printing3DModelTexture>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Textures)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_meshes(&self) -> Result<Option<foundation::collections::IVector<Printing3DMesh>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Meshes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_components(&self) -> Result<Option<foundation::collections::IVector<Printing3DComponent>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Components)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material(&self) -> Result<Option<Printing3DMaterial>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Material)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DMaterial::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_material(&self, value: &Printing3DMaterial) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Material)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_build(&self) -> Result<Option<Printing3DComponent>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Build)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DComponent::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_build(&self, value: &Printing3DComponent) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Build)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
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
    #[inline] pub fn get_required_extensions(&self) -> Result<Option<foundation::collections::IVector<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_RequiredExtensions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_metadata(&self) -> Result<Option<foundation::collections::IMap<HString, HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Metadata)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IMap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn repair_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RepairAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn clone(&self) -> Result<Option<Printing3DModel>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Clone)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DModel::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DModel: IPrinting3DModel}
impl RtActivatable<IActivationFactory> for Printing3DModel {}
DEFINE_CLSID!(Printing3DModel(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,111,100,101,108,0]) [CLSID_Printing3DModel]);
DEFINE_IID!(IID_IPrinting3DModel2, 3374344647, 51265, 18419, 168, 78, 161, 73, 253, 8, 182, 87);
RT_INTERFACE!{interface IPrinting3DModel2(IPrinting3DModel2Vtbl, IPrinting3DModel2_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DModel2] {
    fn TryPartialRepairAsync(&self, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn TryPartialRepairWithTimeAsync(&self, maxWaitTime: foundation::TimeSpan, out: *mut <foundation::IAsyncOperation<bool> as RtType>::Abi) -> HRESULT,
    fn TryReduceFacesAsync(&self, out: *mut <foundation::IAsyncOperationWithProgress<bool, f64> as RtType>::Abi) -> HRESULT,
    fn TryReduceFacesWithOptionsAsync(&self, printing3DFaceReductionOptions: <Printing3DFaceReductionOptions as RtType>::Abi, out: *mut <foundation::IAsyncOperationWithProgress<bool, f64> as RtType>::Abi) -> HRESULT,
    fn TryReduceFacesWithOptionsAndTimeAsync(&self, printing3DFaceReductionOptions: <Printing3DFaceReductionOptions as RtType>::Abi, maxWait: foundation::TimeSpan, out: *mut <foundation::IAsyncOperationWithProgress<bool, f64> as RtType>::Abi) -> HRESULT,
    fn RepairWithProgressAsync(&self, out: *mut <foundation::IAsyncOperationWithProgress<bool, f64> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DModel2 {
    #[inline] pub fn try_partial_repair_async(&self) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryPartialRepairAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_partial_repair_with_time_async(&self, maxWaitTime: foundation::TimeSpan) -> Result<foundation::IAsyncOperation<bool>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryPartialRepairWithTimeAsync)(self.get_abi() as *const _ as *mut _, maxWaitTime, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_async(&self) -> Result<foundation::IAsyncOperationWithProgress<bool, f64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryReduceFacesAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_with_options_async(&self, printing3DFaceReductionOptions: &Printing3DFaceReductionOptions) -> Result<foundation::IAsyncOperationWithProgress<bool, f64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryReduceFacesWithOptionsAsync)(self.get_abi() as *const _ as *mut _, printing3DFaceReductionOptions.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_reduce_faces_with_options_and_time_async(&self, printing3DFaceReductionOptions: &Printing3DFaceReductionOptions, maxWait: foundation::TimeSpan) -> Result<foundation::IAsyncOperationWithProgress<bool, f64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TryReduceFacesWithOptionsAndTimeAsync)(self.get_abi() as *const _ as *mut _, printing3DFaceReductionOptions.get_abi() as *const _ as *mut _, maxWait, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn repair_with_progress_async(&self) -> Result<foundation::IAsyncOperationWithProgress<bool, f64>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RepairWithProgressAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperationWithProgress::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DModelTexture, 1571802881, 46493, 18492, 151, 187, 164, 213, 70, 209, 199, 92);
RT_INTERFACE!{interface IPrinting3DModelTexture(IPrinting3DModelTextureVtbl, IPrinting3DModelTexture_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DModelTexture] {
    fn get_TextureResource(&self, out: *mut <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn put_TextureResource(&self, value: <Printing3DTextureResource as RtType>::Abi) -> HRESULT,
    fn get_TileStyleU(&self, out: *mut Printing3DTextureEdgeBehavior) -> HRESULT,
    fn put_TileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> HRESULT,
    fn get_TileStyleV(&self, out: *mut Printing3DTextureEdgeBehavior) -> HRESULT,
    fn put_TileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> HRESULT
}}
impl IPrinting3DModelTexture {
    #[inline] pub fn get_texture_resource(&self) -> Result<Option<Printing3DTextureResource>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TextureResource)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DTextureResource::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture_resource(&self, value: &Printing3DTextureResource) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TextureResource)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_tile_style_u(&self) -> Result<Printing3DTextureEdgeBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TileStyleU)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_tile_style_u(&self, value: Printing3DTextureEdgeBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TileStyleU)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_tile_style_v(&self) -> Result<Printing3DTextureEdgeBehavior> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TileStyleV)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_tile_style_v(&self, value: Printing3DTextureEdgeBehavior) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TileStyleV)(self.get_abi() as *const _ as *mut _, value);
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
RT_INTERFACE!{interface IPrinting3DMultiplePropertyMaterial(IPrinting3DMultiplePropertyMaterialVtbl, IPrinting3DMultiplePropertyMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterial] {
    fn get_MaterialIndices(&self, out: *mut <foundation::collections::IVector<u32> as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterial {
    #[inline] pub fn get_material_indices(&self) -> Result<Option<foundation::collections::IVector<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMultiplePropertyMaterial: IPrinting3DMultiplePropertyMaterial}
impl RtActivatable<IActivationFactory> for Printing3DMultiplePropertyMaterial {}
DEFINE_CLSID!(Printing3DMultiplePropertyMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,117,108,116,105,112,108,101,80,114,111,112,101,114,116,121,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DMultiplePropertyMaterial]);
DEFINE_IID!(IID_IPrinting3DMultiplePropertyMaterialGroup, 4036298009, 44729, 17685, 163, 155, 160, 136, 251, 187, 39, 124);
RT_INTERFACE!{interface IPrinting3DMultiplePropertyMaterialGroup(IPrinting3DMultiplePropertyMaterialGroupVtbl, IPrinting3DMultiplePropertyMaterialGroup_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterialGroup] {
    fn get_MultipleProperties(&self, out: *mut <foundation::collections::IVector<Printing3DMultiplePropertyMaterial> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupIndices(&self, out: *mut <foundation::collections::IVector<u32> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterialGroup {
    #[inline] pub fn get_multiple_properties(&self) -> Result<Option<foundation::collections::IVector<Printing3DMultiplePropertyMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MultipleProperties)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_indices(&self) -> Result<Option<foundation::collections::IVector<u32>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupIndices)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DMultiplePropertyMaterialGroup: IPrinting3DMultiplePropertyMaterialGroup}
impl RtActivatable<IPrinting3DMultiplePropertyMaterialGroupFactory> for Printing3DMultiplePropertyMaterialGroup {}
impl Printing3DMultiplePropertyMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<Printing3DMultiplePropertyMaterialGroup> {
        <Self as RtActivatable<IPrinting3DMultiplePropertyMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DMultiplePropertyMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,77,117,108,116,105,112,108,101,80,114,111,112,101,114,116,121,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DMultiplePropertyMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DMultiplePropertyMaterialGroupFactory, 842930542, 54470, 17694, 168, 20, 77, 120, 162, 16, 254, 83);
RT_INTERFACE!{static interface IPrinting3DMultiplePropertyMaterialGroupFactory(IPrinting3DMultiplePropertyMaterialGroupFactoryVtbl, IPrinting3DMultiplePropertyMaterialGroupFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DMultiplePropertyMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut <Printing3DMultiplePropertyMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DMultiplePropertyMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<Printing3DMultiplePropertyMaterialGroup> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(Printing3DMultiplePropertyMaterialGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum Printing3DObjectType: i32 {
    Model = 0, Support = 1, Others = 2,
}}
RT_ENUM! { enum Printing3DPackageCompression: i32 {
    Low = 0, Medium = 1, High = 2,
}}
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterial, 2374257659, 2025, 18822, 152, 51, 141, 211, 212, 140, 104, 89);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterial(IPrinting3DTexture2CoordMaterialVtbl, IPrinting3DTexture2CoordMaterial_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterial] {
    fn get_Texture(&self, out: *mut <Printing3DModelTexture as RtType>::Abi) -> HRESULT,
    fn put_Texture(&self, value: <Printing3DModelTexture as RtType>::Abi) -> HRESULT,
    fn get_U(&self, out: *mut f64) -> HRESULT,
    fn put_U(&self, value: f64) -> HRESULT,
    fn get_V(&self, out: *mut f64) -> HRESULT,
    fn put_V(&self, value: f64) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterial {
    #[inline] pub fn get_texture(&self) -> Result<Option<Printing3DModelTexture>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Texture)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DModelTexture::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture(&self, value: &Printing3DModelTexture) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Texture)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_u(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_U)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_u(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_U)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_v(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_V)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_v(&self, value: f64) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_V)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DTexture2CoordMaterial: IPrinting3DTexture2CoordMaterial}
impl RtActivatable<IActivationFactory> for Printing3DTexture2CoordMaterial {}
DEFINE_CLSID!(Printing3DTexture2CoordMaterial(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,50,67,111,111,114,100,77,97,116,101,114,105,97,108,0]) [CLSID_Printing3DTexture2CoordMaterial]);
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroup, 1652391079, 28048, 20409, 159, 196, 159, 239, 243, 223, 168, 146);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterialGroup(IPrinting3DTexture2CoordMaterialGroupVtbl, IPrinting3DTexture2CoordMaterialGroup_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroup] {
    fn get_Texture2Coords(&self, out: *mut <foundation::collections::IVector<Printing3DTexture2CoordMaterial> as RtType>::Abi) -> HRESULT,
    fn get_MaterialGroupId(&self, out: *mut u32) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroup {
    #[inline] pub fn get_texture2_coords(&self) -> Result<Option<foundation::collections::IVector<Printing3DTexture2CoordMaterial>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Texture2Coords)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVector::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_material_group_id(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaterialGroupId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DTexture2CoordMaterialGroup: IPrinting3DTexture2CoordMaterialGroup}
impl RtActivatable<IPrinting3DTexture2CoordMaterialGroupFactory> for Printing3DTexture2CoordMaterialGroup {}
impl Printing3DTexture2CoordMaterialGroup {
    #[inline] pub fn create(materialGroupId: u32) -> Result<Printing3DTexture2CoordMaterialGroup> {
        <Self as RtActivatable<IPrinting3DTexture2CoordMaterialGroupFactory>>::get_activation_factory().create(materialGroupId)
    }
}
DEFINE_CLSID!(Printing3DTexture2CoordMaterialGroup(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,50,67,111,111,114,100,77,97,116,101,114,105,97,108,71,114,111,117,112,0]) [CLSID_Printing3DTexture2CoordMaterialGroup]);
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroup2, 1778113466, 45358, 17051, 131, 134, 223, 82, 132, 246, 232, 15);
RT_INTERFACE!{interface IPrinting3DTexture2CoordMaterialGroup2(IPrinting3DTexture2CoordMaterialGroup2Vtbl, IPrinting3DTexture2CoordMaterialGroup2_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroup2] {
    fn get_Texture(&self, out: *mut <Printing3DModelTexture as RtType>::Abi) -> HRESULT,
    fn put_Texture(&self, value: <Printing3DModelTexture as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroup2 {
    #[inline] pub fn get_texture(&self) -> Result<Option<Printing3DModelTexture>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Texture)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(Printing3DModelTexture::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_texture(&self, value: &Printing3DModelTexture) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Texture)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPrinting3DTexture2CoordMaterialGroupFactory, 3417328048, 18058, 19567, 178, 162, 142, 184, 186, 141, 234, 72);
RT_INTERFACE!{static interface IPrinting3DTexture2CoordMaterialGroupFactory(IPrinting3DTexture2CoordMaterialGroupFactoryVtbl, IPrinting3DTexture2CoordMaterialGroupFactory_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DTexture2CoordMaterialGroupFactory] {
    fn Create(&self, materialGroupId: u32, out: *mut <Printing3DTexture2CoordMaterialGroup as RtType>::Abi) -> HRESULT
}}
impl IPrinting3DTexture2CoordMaterialGroupFactory {
    #[inline] pub fn create(&self, materialGroupId: u32) -> Result<Printing3DTexture2CoordMaterialGroup> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, materialGroupId, &mut out);
        if hr == S_OK { Ok(Printing3DTexture2CoordMaterialGroup::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum Printing3DTextureEdgeBehavior: i32 {
    None = 0, Wrap = 1, Mirror = 2, Clamp = 3,
}}
DEFINE_IID!(IID_IPrinting3DTextureResource, 2802709293, 27313, 17582, 188, 69, 162, 115, 130, 192, 211, 140);
RT_INTERFACE!{interface IPrinting3DTextureResource(IPrinting3DTextureResourceVtbl, IPrinting3DTextureResource_Abi): IInspectable(IInspectableVtbl) [IID_IPrinting3DTextureResource] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn get_TextureData(&self, out: *mut <super::super::storage::streams::IRandomAccessStreamWithContentType as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn put_TextureData(&self, value: <super::super::storage::streams::IRandomAccessStreamWithContentType as RtType>::Abi) -> HRESULT,
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Name(&self, value: HSTRING) -> HRESULT
}}
impl IPrinting3DTextureResource {
    #[cfg(feature="windows-storage")] #[inline] pub fn get_texture_data(&self) -> Result<Option<super::super::storage::streams::IRandomAccessStreamWithContentType>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TextureData)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::super::storage::streams::IRandomAccessStreamWithContentType::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn set_texture_data(&self, value: &super::super::storage::streams::IRandomAccessStreamWithContentType) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_TextureData)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_name(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Name)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class Printing3DTextureResource: IPrinting3DTextureResource}
impl RtActivatable<IActivationFactory> for Printing3DTextureResource {}
DEFINE_CLSID!(Printing3DTextureResource(&[87,105,110,100,111,119,115,46,71,114,97,112,104,105,99,115,46,80,114,105,110,116,105,110,103,51,68,46,80,114,105,110,116,105,110,103,51,68,84,101,120,116,117,114,101,82,101,115,111,117,114,99,101,0]) [CLSID_Printing3DTextureResource]);
} // Windows.Graphics.Printing3D
