// DO NOT MODIFY THIS FILE - IT IS AUTOMATICALLY GENERATED!
#![allow(non_camel_case_types, unused_imports)]
pub mod windows { // Windows
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
pub mod foundation { // Windows.Foundation
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		DEFINE_IID!(IID_IClosable, 819308585, 32676, 16422, 131, 187, 215, 91, 174, 78, 169, 158);
		RT_INTERFACE!{interface IClosable(IClosableVtbl): IInspectable(IInspectableVtbl) [IID_IClosable] {
			fn Close(&mut self) -> ::w::HRESULT
		}}
		RT_ENUM! { enum PropertyType: i32 {
			Empty (PropertyType_Empty) = 0, UInt8 (PropertyType_UInt8) = 1, Int16 (PropertyType_Int16) = 2, UInt16 (PropertyType_UInt16) = 3, Int32 (PropertyType_Int32) = 4, UInt32 (PropertyType_UInt32) = 5, Int64 (PropertyType_Int64) = 6, UInt64 (PropertyType_UInt64) = 7, Single (PropertyType_Single) = 8, Double (PropertyType_Double) = 9, Char16 (PropertyType_Char16) = 10, Boolean (PropertyType_Boolean) = 11, String (PropertyType_String) = 12, Inspectable (PropertyType_Inspectable) = 13, DateTime (PropertyType_DateTime) = 14, TimeSpan (PropertyType_TimeSpan) = 15, Guid (PropertyType_Guid) = 16, Point (PropertyType_Point) = 17, Size (PropertyType_Size) = 18, Rect (PropertyType_Rect) = 19, OtherType (PropertyType_OtherType) = 20, UInt8Array (PropertyType_UInt8Array) = 1025, Int16Array (PropertyType_Int16Array) = 1026, UInt16Array (PropertyType_UInt16Array) = 1027, Int32Array (PropertyType_Int32Array) = 1028, UInt32Array (PropertyType_UInt32Array) = 1029, Int64Array (PropertyType_Int64Array) = 1030, UInt64Array (PropertyType_UInt64Array) = 1031, SingleArray (PropertyType_SingleArray) = 1032, DoubleArray (PropertyType_DoubleArray) = 1033, Char16Array (PropertyType_Char16Array) = 1034, BooleanArray (PropertyType_BooleanArray) = 1035, StringArray (PropertyType_StringArray) = 1036, InspectableArray (PropertyType_InspectableArray) = 1037, DateTimeArray (PropertyType_DateTimeArray) = 1038, TimeSpanArray (PropertyType_TimeSpanArray) = 1039, GuidArray (PropertyType_GuidArray) = 1040, PointArray (PropertyType_PointArray) = 1041, SizeArray (PropertyType_SizeArray) = 1042, RectArray (PropertyType_RectArray) = 1043, OtherTypeArray (PropertyType_OtherTypeArray) = 1044,
		}}
		RT_STRUCT! { struct Point {
			X: f32, Y: f32,
		}}
		RT_STRUCT! { struct Size {
			Width: f32, Height: f32,
		}}
		RT_STRUCT! { struct Rect {
			X: f32, Y: f32, Width: f32, Height: f32,
		}}
		RT_STRUCT! { struct DateTime {
			UniversalTime: i64,
		}}
		RT_STRUCT! { struct TimeSpan {
			Duration: i64,
		}}
		DEFINE_IID!(IID_IPropertyValue, 1272349405, 30036, 16617, 154, 155, 130, 101, 78, 222, 126, 98);
		RT_INTERFACE!{interface IPropertyValue(IPropertyValueVtbl): IInspectable(IInspectableVtbl) [IID_IPropertyValue] {
			fn get_Type(&mut self, out: *mut ::rt::gen::windows::foundation::PropertyType) -> ::w::HRESULT,
			fn get_IsNumericScalar(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetUInt8(&mut self, out: *mut u8) -> ::w::HRESULT,
			fn GetInt16(&mut self, out: *mut i16) -> ::w::HRESULT,
			fn GetUInt16(&mut self, out: *mut u16) -> ::w::HRESULT,
			fn GetInt32(&mut self, out: *mut i32) -> ::w::HRESULT,
			fn GetUInt32(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn GetInt64(&mut self, out: *mut i64) -> ::w::HRESULT,
			fn GetUInt64(&mut self, out: *mut u64) -> ::w::HRESULT,
			fn GetSingle(&mut self, out: *mut f32) -> ::w::HRESULT,
			fn GetDouble(&mut self, out: *mut f64) -> ::w::HRESULT,
			fn GetChar16(&mut self, out: *mut ::w::wchar_t) -> ::w::HRESULT,
			fn GetBoolean(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetString(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn GetGuid(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn GetDateTime(&mut self, out: *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn GetTimeSpan(&mut self, out: *mut ::rt::gen::windows::foundation::TimeSpan) -> ::w::HRESULT,
			fn GetPoint(&mut self, out: *mut ::rt::gen::windows::foundation::Point) -> ::w::HRESULT,
			fn GetSize(&mut self, out: *mut ::rt::gen::windows::foundation::Size) -> ::w::HRESULT,
			fn GetRect(&mut self, out: *mut ::rt::gen::windows::foundation::Rect) -> ::w::HRESULT,
			fn GetUInt8Array(&mut self, value: *mut *mut u8) -> ::w::HRESULT,
			fn GetInt16Array(&mut self, value: *mut *mut i16) -> ::w::HRESULT,
			fn GetUInt16Array(&mut self, value: *mut *mut u16) -> ::w::HRESULT,
			fn GetInt32Array(&mut self, value: *mut *mut i32) -> ::w::HRESULT,
			fn GetUInt32Array(&mut self, value: *mut *mut u32) -> ::w::HRESULT,
			fn GetInt64Array(&mut self, value: *mut *mut i64) -> ::w::HRESULT,
			fn GetUInt64Array(&mut self, value: *mut *mut u64) -> ::w::HRESULT,
			fn GetSingleArray(&mut self, value: *mut *mut f32) -> ::w::HRESULT,
			fn GetDoubleArray(&mut self, value: *mut *mut f64) -> ::w::HRESULT,
			fn GetChar16Array(&mut self, value: *mut *mut ::w::wchar_t) -> ::w::HRESULT,
			fn GetBooleanArray(&mut self, value: *mut *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetStringArray(&mut self, value: *mut *mut ::w::HSTRING) -> ::w::HRESULT,
			fn GetInspectableArray(&mut self, value: *mut *mut *mut IInspectable) -> ::w::HRESULT,
			fn GetGuidArray(&mut self, value: *mut *mut ::w::GUID) -> ::w::HRESULT,
			fn GetDateTimeArray(&mut self, value: *mut *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn GetTimeSpanArray(&mut self, value: *mut *mut ::rt::gen::windows::foundation::TimeSpan) -> ::w::HRESULT,
			fn GetPointArray(&mut self, value: *mut *mut ::rt::gen::windows::foundation::Point) -> ::w::HRESULT,
			fn GetSizeArray(&mut self, value: *mut *mut ::rt::gen::windows::foundation::Size) -> ::w::HRESULT,
			fn GetRectArray(&mut self, value: *mut *mut ::rt::gen::windows::foundation::Rect) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IPropertyValueStatics, 1654381512, 55602, 20468, 150, 185, 141, 150, 197, 193, 232, 88);
		RT_INTERFACE!{interface IPropertyValueStatics(IPropertyValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPropertyValueStatics] {
			fn CreateEmpty(&mut self, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt8(&mut self, value: u8, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt16(&mut self, value: i16, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt16(&mut self, value: u16, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt32(&mut self, value: i32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt32(&mut self, value: u32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt64(&mut self, value: i64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt64(&mut self, value: u64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSingle(&mut self, value: f32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateDouble(&mut self, value: f64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateChar16(&mut self, value: ::w::wchar_t, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateBoolean(&mut self, value: ::w::BOOL, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateString(&mut self, value: ::w::HSTRING, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInspectable(&mut self, value: *mut IInspectable, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateGuid(&mut self, value: ::w::GUID, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateDateTime(&mut self, value: ::rt::gen::windows::foundation::DateTime, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateTimeSpan(&mut self, value: ::rt::gen::windows::foundation::TimeSpan, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreatePoint(&mut self, value: ::rt::gen::windows::foundation::Point, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSize(&mut self, value: ::rt::gen::windows::foundation::Size, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateRect(&mut self, value: ::rt::gen::windows::foundation::Rect, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt8Array(&mut self, value: *mut u8, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt16Array(&mut self, value: *mut i16, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt16Array(&mut self, value: *mut u16, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt32Array(&mut self, value: *mut i32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt32Array(&mut self, value: *mut u32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInt64Array(&mut self, value: *mut i64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateUInt64Array(&mut self, value: *mut u64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSingleArray(&mut self, value: *mut f32, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateDoubleArray(&mut self, value: *mut f64, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateChar16Array(&mut self, value: *mut ::w::wchar_t, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateBooleanArray(&mut self, value: *mut ::w::BOOL, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateStringArray(&mut self, value: *mut ::w::HSTRING, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateInspectableArray(&mut self, value: *mut *mut IInspectable, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateGuidArray(&mut self, value: *mut ::w::GUID, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateDateTimeArray(&mut self, value: *mut ::rt::gen::windows::foundation::DateTime, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateTimeSpanArray(&mut self, value: *mut ::rt::gen::windows::foundation::TimeSpan, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreatePointArray(&mut self, value: *mut ::rt::gen::windows::foundation::Point, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSizeArray(&mut self, value: *mut ::rt::gen::windows::foundation::Size, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateRectArray(&mut self, value: *mut ::rt::gen::windows::foundation::Rect, out: *mut *mut IInspectable) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IStringable, 2520162132, 36534, 18672, 171, 206, 193, 178, 17, 230, 39, 195);
		RT_INTERFACE!{interface IStringable(IStringableVtbl): IInspectable(IInspectableVtbl) [IID_IStringable] {
			fn ToString(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncActionCompletedHandler, 2767019137, 30409, 16573, 139, 230, 177, 217, 15, 178, 10, 231);
		RT_DELEGATE!{delegate AsyncActionCompletedHandler(AsyncActionCompletedHandlerVtbl, AsyncActionCompletedHandlerImpl) [IID_AsyncActionCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncAction, asyncStatus: ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IDeferral, 3592853298, 15231, 18087, 180, 11, 79, 220, 162, 162, 198, 147);
		RT_INTERFACE!{interface IDeferral(IDeferralVtbl): IInspectable(IInspectableVtbl) [IID_IDeferral] {
			fn Complete(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_DeferralCompletedHandler, 3979518834, 62408, 20394, 156, 251, 71, 1, 72, 218, 56, 136);
		RT_DELEGATE!{delegate DeferralCompletedHandler(DeferralCompletedHandlerVtbl, DeferralCompletedHandlerImpl) [IID_DeferralCompletedHandler] {
			fn Invoke(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IDeferralFactory, 1705110725, 16309, 18482, 140, 169, 240, 97, 178, 129, 209, 58);
		RT_INTERFACE!{interface IDeferralFactory(IDeferralFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IDeferralFactory] {
			fn Create(&mut self, handler: *mut ::rt::gen::windows::foundation::DeferralCompletedHandler, out: *mut *mut ::rt::gen::windows::foundation::Deferral) -> ::w::HRESULT
		}}
		RT_CLASS!(Deferral: ::rt::gen::windows::foundation::IDeferral);
		RT_ENUM! { enum AsyncStatus: i32 {
			Canceled (AsyncStatus_Canceled) = 2, Completed (AsyncStatus_Completed) = 1, Error (AsyncStatus_Error) = 3, Started (AsyncStatus_Started) = 0,
		}}
		RT_STRUCT! { struct EventRegistrationToken {
			Value: i64,
		}}
		RT_STRUCT! { struct HResult {
			Value: i32,
		}}
		DEFINE_IID!(IID_IAsyncInfo, 54, 0, 0, 192, 0, 0, 0, 0, 0, 0, 70);
		RT_INTERFACE!{interface IAsyncInfo(IAsyncInfoVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncInfo] {
			fn get_Id(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn get_Status(&mut self, out: *mut ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT,
			fn get_ErrorCode(&mut self, out: *mut ::rt::gen::windows::foundation::HResult) -> ::w::HRESULT,
			fn Cancel(&mut self) -> ::w::HRESULT,
			fn Close(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IAsyncAction, 1516535814, 33850, 19881, 134, 91, 157, 38, 229, 223, 173, 123);
		RT_INTERFACE!{interface IAsyncAction(IAsyncActionVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncAction] {
			fn put_Completed(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncActionCompletedHandler) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncActionCompletedHandler) -> ::w::HRESULT,
			fn GetResults(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncOperationWithProgressCompletedHandler, 3898471453, 27303, 18147, 168, 226, 240, 9, 216, 64, 198, 39);
		RT_DELEGATE!{delegate AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(AsyncOperationWithProgressCompletedHandlerVtbl, AsyncOperationWithProgressCompletedHandlerImpl) [IID_AsyncOperationWithProgressCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<TResult, TProgress>, asyncStatus: ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IAsyncOperationWithProgress, 3050321623, 58007, 18831, 186, 96, 2, 137, 231, 110, 35, 221);
		RT_INTERFACE!{interface IAsyncOperationWithProgress<TResult, TProgress>(IAsyncOperationWithProgressVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncOperationWithProgress] {
			fn put_Progress(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncOperationProgressHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn get_Progress(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncOperationProgressHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn put_Completed(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncOperationWithProgressCompletedHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncOperationWithProgressCompletedHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn GetResults(&mut self, out: *mut TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncOperationCompletedHandler, 4242337836, 58840, 17528, 145, 90, 77, 144, 183, 75, 131, 165);
		RT_DELEGATE!{delegate AsyncOperationCompletedHandler<TResult>(AsyncOperationCompletedHandlerVtbl, AsyncOperationCompletedHandlerImpl) [IID_AsyncOperationCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncOperation<TResult>, asyncStatus: ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IAsyncOperation, 2680336571, 58438, 17634, 170, 97, 156, 171, 143, 99, 106, 242);
		RT_INTERFACE!{interface IAsyncOperation<TResult>(IAsyncOperationVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncOperation] {
			fn put_Completed(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncOperationCompletedHandler<TResult>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncOperationCompletedHandler<TResult>) -> ::w::HRESULT,
			fn GetResults(&mut self, out: *mut TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncActionWithProgressCompletedHandler, 2617417617, 52356, 17661, 172, 38, 10, 108, 78, 85, 82, 129);
		RT_DELEGATE!{delegate AsyncActionWithProgressCompletedHandler<TProgress>(AsyncActionWithProgressCompletedHandlerVtbl, AsyncActionWithProgressCompletedHandlerImpl) [IID_AsyncActionWithProgressCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress>, asyncStatus: ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IAsyncActionWithProgress, 527282776, 59395, 18593, 149, 70, 235, 115, 83, 57, 136, 132);
		RT_INTERFACE!{interface IAsyncActionWithProgress<TProgress>(IAsyncActionWithProgressVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncActionWithProgress] {
			fn put_Progress(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>) -> ::w::HRESULT,
			fn get_Progress(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncActionProgressHandler<TProgress>) -> ::w::HRESULT,
			fn put_Completed(&mut self, handler: *mut ::rt::gen::windows::foundation::AsyncActionWithProgressCompletedHandler<TProgress>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut ::rt::gen::windows::foundation::AsyncActionWithProgressCompletedHandler<TProgress>) -> ::w::HRESULT,
			fn GetResults(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncOperationProgressHandler, 1432946946, 2731, 16922, 135, 120, 248, 206, 80, 38, 215, 88);
		RT_DELEGATE!{delegate AsyncOperationProgressHandler<TResult, TProgress>(AsyncOperationProgressHandlerVtbl, AsyncOperationProgressHandlerImpl) [IID_AsyncOperationProgressHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<TResult, TProgress>, progressInfo: TProgress::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_AsyncActionProgressHandler, 1837385816, 3327, 17808, 174, 137, 149, 165, 165, 200, 180, 184);
		RT_DELEGATE!{delegate AsyncActionProgressHandler<TProgress>(AsyncActionProgressHandlerVtbl, AsyncActionProgressHandlerImpl) [IID_AsyncActionProgressHandler] {
			fn Invoke(&mut self, asyncInfo: *mut ::rt::gen::windows::foundation::IAsyncActionWithProgress<TProgress>, progressInfo: TProgress::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IReference, 1640068870, 11621, 4576, 154, 232, 212, 133, 100, 1, 84, 114);
		RT_INTERFACE!{interface IReference<T>(IReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IReference] {
			fn get_Value(&mut self, out: *mut T::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IReferenceArray, 1640068871, 11621, 4576, 154, 232, 212, 133, 100, 1, 84, 114);
		RT_INTERFACE!{interface IReferenceArray<T>(IReferenceArrayVtbl): IInspectable(IInspectableVtbl) [IID_IReferenceArray] {
			fn get_Value(&mut self, out: *mut *mut T::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_TypedEventHandler, 2648818996, 27361, 4576, 132, 225, 24, 169, 5, 188, 197, 63);
		RT_DELEGATE!{delegate TypedEventHandler<TSender, TResult>(TypedEventHandlerVtbl, TypedEventHandlerImpl) [IID_TypedEventHandler] {
			fn Invoke(&mut self, sender: TSender::Abi, args: TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_EventHandler, 2648818997, 27361, 4576, 132, 225, 24, 169, 5, 188, 197, 63);
		RT_DELEGATE!{delegate EventHandler<T>(EventHandlerVtbl, EventHandlerImpl) [IID_EventHandler] {
			fn Invoke(&mut self, sender: *mut IInspectable, args: T::Abi) -> ::w::HRESULT
		}}
		RT_STRUCT! { struct FoundationContract {
			
		}}
		DEFINE_IID!(IID_IUriRuntimeClass, 2654363223, 18610, 16736, 149, 111, 199, 56, 81, 32, 187, 252);
		RT_INTERFACE!{interface IUriRuntimeClass(IUriRuntimeClassVtbl): IInspectable(IInspectableVtbl) [IID_IUriRuntimeClass] {
			fn get_AbsoluteUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_DisplayUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Domain(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Extension(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Fragment(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Host(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Password(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Path(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Query(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_QueryParsed(&mut self, out: *mut *mut ::rt::gen::windows::foundation::WwwFormUrlDecoder) -> ::w::HRESULT,
			fn get_RawUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_SchemeName(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_UserName(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Port(&mut self, out: *mut i32) -> ::w::HRESULT,
			fn get_Suspicious(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Equals(&mut self, pUri: *mut ::rt::gen::windows::foundation::Uri, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn CombineUri(&mut self, relativeUri: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> ::w::HRESULT
		}}
		RT_CLASS!(WwwFormUrlDecoder: ::rt::gen::windows::foundation::IWwwFormUrlDecoderRuntimeClass);
		RT_CLASS!(Uri: ::rt::gen::windows::foundation::IUriRuntimeClass);
		DEFINE_IID!(IID_IUriRuntimeClassWithAbsoluteCanonicalUri, 1972213345, 8732, 18447, 163, 57, 80, 101, 102, 115, 244, 111);
		RT_INTERFACE!{interface IUriRuntimeClassWithAbsoluteCanonicalUri(IUriRuntimeClassWithAbsoluteCanonicalUriVtbl): IInspectable(IInspectableVtbl) [IID_IUriRuntimeClassWithAbsoluteCanonicalUri] {
			fn get_AbsoluteCanonicalUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_DisplayIri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IUriEscapeStatics, 3251909306, 51236, 17490, 167, 253, 81, 43, 195, 187, 233, 161);
		RT_INTERFACE!{interface IUriEscapeStatics(IUriEscapeStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUriEscapeStatics] {
			fn UnescapeComponent(&mut self, toUnescape: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn EscapeComponent(&mut self, toEscape: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IUriRuntimeClassFactory, 1151957359, 29246, 20447, 162, 24, 3, 62, 117, 176, 192, 132);
		RT_INTERFACE!{interface IUriRuntimeClassFactory(IUriRuntimeClassFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IUriRuntimeClassFactory] {
			fn CreateUri(&mut self, uri: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> ::w::HRESULT,
			fn CreateWithRelativeUri(&mut self, baseUri: ::w::HSTRING, relativeUri: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IWwwFormUrlDecoderEntry, 308180017, 63096, 20110, 182, 112, 32, 169, 176, 108, 81, 45);
		RT_INTERFACE!{interface IWwwFormUrlDecoderEntry(IWwwFormUrlDecoderEntryVtbl): IInspectable(IInspectableVtbl) [IID_IWwwFormUrlDecoderEntry] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Value(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IWwwFormUrlDecoderRuntimeClass, 3562669137, 61989, 17730, 146, 150, 14, 29, 245, 210, 84, 223);
		RT_INTERFACE!{interface IWwwFormUrlDecoderRuntimeClass(IWwwFormUrlDecoderRuntimeClassVtbl): IInspectable(IInspectableVtbl) [IID_IWwwFormUrlDecoderRuntimeClass] {
			fn GetFirstValueByName(&mut self, name: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IWwwFormUrlDecoderRuntimeClassFactory, 1535929149, 9390, 16821, 161, 191, 240, 195, 213, 68, 132, 91);
		RT_INTERFACE!{interface IWwwFormUrlDecoderRuntimeClassFactory(IWwwFormUrlDecoderRuntimeClassFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWwwFormUrlDecoderRuntimeClassFactory] {
			fn CreateWwwFormUrlDecoder(&mut self, query: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::WwwFormUrlDecoder) -> ::w::HRESULT
		}}
		RT_CLASS!(WwwFormUrlDecoderEntry: ::rt::gen::windows::foundation::IWwwFormUrlDecoderEntry);
		DEFINE_IID!(IID_IGetActivationFactory, 1323011810, 38621, 18855, 148, 247, 70, 7, 221, 171, 142, 60);
		RT_INTERFACE!{interface IGetActivationFactory(IGetActivationFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IGetActivationFactory] {
			fn GetActivationFactory(&mut self, activatableClassId: ::w::HSTRING, out: *mut *mut IInspectable) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMemoryBufferReference, 4223982889, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface IMemoryBufferReference(IMemoryBufferReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IMemoryBufferReference] {
			fn get_Capacity(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn add_Closed(&mut self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<&::rt::gen::windows::foundation::IMemoryBufferReference, &IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_Closed(&mut self, cookie: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMemoryBuffer, 4223982890, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface IMemoryBuffer(IMemoryBufferVtbl): IInspectable(IInspectableVtbl) [IID_IMemoryBuffer] {
			fn CreateReference(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IMemoryBufferReference) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMemoryBufferFactory, 4223982891, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface IMemoryBufferFactory(IMemoryBufferFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IMemoryBufferFactory] {
			fn Create(&mut self, capacity: u32, out: *mut *mut ::rt::gen::windows::foundation::MemoryBuffer) -> ::w::HRESULT
		}}
		RT_CLASS!(MemoryBuffer: ::rt::gen::windows::foundation::IMemoryBuffer);
		RT_STRUCT! { struct UniversalApiContract {
			
		}}
		RT_PINTERFACE!{ for<'a> EventHandler<&'a ::rt::gen::windows::foundation::diagnostics::TracingStatusChangedEventArgs> => [0x2bf27008,0x2eb4,0x5675,0xb1,0xcd,0xe9,0x90,0x6c,0xc5,0xce,0x64] as IID_EventHandler_1_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs }
		RT_PINTERFACE!{ for<'a> TypedEventHandler<&'a ::rt::gen::windows::foundation::IMemoryBufferReference, &'a IInspectable> => [0xf4637d4a,0x0760,0x5431,0xbf,0xc0,0x24,0xeb,0x1d,0x4f,0x6c,0x4f] as IID_TypedEventHandler_2_Windows_Foundation_IMemoryBufferReference_System_Object }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::foundation::diagnostics::ErrorDetails> => [0x9b05106d,0x77e0,0x5c24,0x82,0xb0,0x9b,0x2d,0xc8,0xf7,0x96,0x71] as IID_IAsyncOperation_1_Windows_Foundation_Diagnostics_ErrorDetails }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::foundation::diagnostics::ErrorDetails> => [0xa6997f9d,0x7195,0x5972,0x8e,0xcd,0x1c,0x73,0xaa,0x5c,0xb3,0x12] as IID_AsyncOperationCompletedHandler_1_Windows_Foundation_Diagnostics_ErrorDetails }
		RT_PINTERFACE!{ for<'a> TypedEventHandler<&'a ::rt::gen::windows::foundation::diagnostics::ILoggingChannel, &'a IInspectable> => [0x52c9c2a1,0x54a3,0x5ef9,0x9a,0xff,0x01,0x4e,0x7c,0x45,0x46,0x55] as IID_TypedEventHandler_2_Windows_Foundation_Diagnostics_ILoggingChannel_System_Object }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::StorageFile> => [0x5e52f8ce,0xaced,0x5a42,0x95,0xb4,0xf6,0x74,0xdd,0x84,0x88,0x5e] as IID_IAsyncOperation_1_Windows_Storage_StorageFile }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::StorageFile> => [0xe521c894,0x2c26,0x5946,0x9e,0x61,0x2b,0x5e,0x18,0x8d,0x01,0xed] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_StorageFile }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::streams::IRandomAccessStream> => [0x430ecece,0x1418,0x5d19,0x81,0xb2,0x5d,0xdb,0x38,0x16,0x03,0xcc] as IID_IAsyncOperation_1_Windows_Storage_Streams_IRandomAccessStream }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::streams::IRandomAccessStream> => [0x398c4183,0x793d,0x5b00,0x81,0x9b,0x4a,0xef,0x92,0x48,0x5e,0x94] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_Streams_IRandomAccessStream }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::StorageStreamTransaction> => [0x0d81405a,0x9bd3,0x5e87,0x82,0xf4,0x9b,0x41,0x28,0xa8,0x87,0xeb] as IID_IAsyncOperation_1_Windows_Storage_StorageStreamTransaction }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::StorageStreamTransaction> => [0xd11739e6,0x2995,0x5d33,0xbf,0xff,0x51,0xb6,0x04,0x1f,0x68,0xc1] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_StorageStreamTransaction }
		RT_PINTERFACE!{ for IAsyncOperationWithProgress<u32, u32> => [0xeccb574a,0xc684,0x5572,0xa6,0x79,0x6b,0x08,0x42,0xcf,0xb5,0x7f] as IID_IAsyncOperationWithProgress_2_System_UInt32_System_UInt32 }
		RT_PINTERFACE!{ for AsyncOperationProgressHandler<u32, u32> => [0xea0fe405,0xd432,0x5ac7,0x9e,0xf8,0x5a,0x65,0xe1,0xf9,0x7d,0x7e] as IID_AsyncOperationProgressHandler_2_System_UInt32_System_UInt32 }
		RT_PINTERFACE!{ for AsyncOperationWithProgressCompletedHandler<u32, u32> => [0x1e466dc5,0x840f,0x54f9,0xb8,0x77,0x5e,0x3a,0x9f,0x4b,0x6c,0x74] as IID_AsyncOperationWithProgressCompletedHandler_2_System_UInt32_System_UInt32 }
		RT_PINTERFACE!{ for IAsyncOperation<bool> => [0xcdb5efb3,0x5788,0x509d,0x9b,0xe1,0x71,0xcc,0xb8,0xa3,0x36,0x2a] as IID_IAsyncOperation_1_System_Boolean }
		RT_PINTERFACE!{ for AsyncOperationCompletedHandler<bool> => [0xc1d3d1a2,0xae17,0x5a5f,0xb5,0xa2,0xbd,0xcc,0x88,0x44,0x88,0x9a] as IID_AsyncOperationCompletedHandler_1_System_Boolean }
		RT_PINTERFACE!{ for<'a> IAsyncOperationWithProgress<&'a ::rt::gen::windows::storage::streams::IBuffer, u32> => [0xd26b2819,0x897f,0x5c7d,0x84,0xd6,0x56,0xd7,0x96,0x56,0x14,0x31] as IID_IAsyncOperationWithProgress_2_Windows_Storage_Streams_IBuffer_System_UInt32 }
		RT_PINTERFACE!{ for<'a> AsyncOperationProgressHandler<&'a ::rt::gen::windows::storage::streams::IBuffer, u32> => [0xbf666554,0x7605,0x5d9a,0xb1,0x4e,0x18,0xd8,0xc8,0x47,0x2a,0xfe] as IID_AsyncOperationProgressHandler_2_Windows_Storage_Streams_IBuffer_System_UInt32 }
		RT_PINTERFACE!{ for<'a> AsyncOperationWithProgressCompletedHandler<&'a ::rt::gen::windows::storage::streams::IBuffer, u32> => [0x06386a7a,0xe009,0x5b0b,0xab,0x68,0xa8,0xe4,0x8b,0x51,0x66,0x47] as IID_AsyncOperationWithProgressCompletedHandler_2_Windows_Storage_Streams_IBuffer_System_UInt32 }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::StorageFolder> => [0x6be9e7d7,0xe83a,0x5cbc,0x80,0x2c,0x17,0x68,0x96,0x0b,0x52,0xc3] as IID_IAsyncOperation_1_Windows_Storage_StorageFolder }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::StorageFolder> => [0xc211026e,0x9e63,0x5452,0xba,0x54,0x3a,0x07,0xd6,0xa9,0x68,0x74] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_StorageFolder }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::IStorageItem> => [0x5fc9c137,0xebb7,0x5e6c,0x9c,0xba,0x68,0x6f,0x2e,0xc2,0xb0,0xbb] as IID_IAsyncOperation_1_Windows_Storage_IStorageItem }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::IStorageItem> => [0x92c3102f,0xa327,0x5318,0xa6,0xc1,0x76,0xf6,0xb2,0xa0,0xab,0xfb] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_IStorageItem }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::StorageFile>> => [0x03362e33,0xe413,0x5f29,0x97,0xd0,0x48,0xa4,0x78,0x09,0x35,0xf9] as IID_IAsyncOperation_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_StorageFile }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::StorageFile>> => [0xcb4206c5,0x0988,0x5104,0xaf,0xa9,0x25,0x3c,0x29,0x8f,0x86,0xfd] as IID_AsyncOperationCompletedHandler_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_StorageFile }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::StorageFolder>> => [0xca40b21b,0xaeb1,0x5a61,0x9e,0x08,0x3b,0xd5,0xd9,0x59,0x40,0x23] as IID_IAsyncOperation_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_StorageFolder }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::StorageFolder>> => [0xed2d1d9b,0x26ec,0x5be7,0xa8,0xa3,0x56,0x45,0x89,0x33,0xd2,0x5f] as IID_AsyncOperationCompletedHandler_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_StorageFolder }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::IStorageItem>> => [0x4b1c0fd7,0x7a01,0x5e7a,0xa6,0xfe,0xbe,0x45,0x00,0x28,0x3f,0x23] as IID_IAsyncOperation_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_IStorageItem }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::foundation::collections::IVectorView<&'a ::rt::gen::windows::storage::IStorageItem>> => [0x51436e75,0xace1,0x5a68,0xb2,0x60,0xf8,0x43,0xb8,0x46,0xf0,0xdb] as IID_AsyncOperationCompletedHandler_1_Windows_Foundation_Collections_IVectorView_1_Windows_Storage_IStorageItem }
		RT_PINTERFACE!{ for<'a> IAsyncOperation<&'a ::rt::gen::windows::storage::fileproperties::BasicProperties> => [0x5186131a,0x4467,0x504b,0x97,0x7a,0x07,0x85,0xa8,0x23,0x04,0x85] as IID_IAsyncOperation_1_Windows_Storage_FileProperties_BasicProperties }
		RT_PINTERFACE!{ for<'a> AsyncOperationCompletedHandler<&'a ::rt::gen::windows::storage::fileproperties::BasicProperties> => [0xc8659aae,0x4926,0x52ad,0x8f,0x60,0xd8,0x9f,0xe5,0xa8,0xdf,0x5f] as IID_AsyncOperationCompletedHandler_1_Windows_Storage_FileProperties_BasicProperties }
		RT_PINTERFACE!{ for<'a> TypedEventHandler<&'a ::rt::gen::windows::foundation::diagnostics::IFileLoggingSession, &'a ::rt::gen::windows::foundation::diagnostics::LogFileGeneratedEventArgs> => [0x0c6563b0,0x9d8b,0x5b60,0x99,0x4b,0xde,0xe1,0x17,0x4d,0x1e,0xfb] as IID_TypedEventHandler_2_Windows_Foundation_Diagnostics_IFileLoggingSession_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs }
pub mod collections { // Windows.Foundation.Collections
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_ENUM! { enum CollectionChange: i32 {
			Reset (CollectionChange_Reset) = 0, ItemInserted (CollectionChange_ItemInserted) = 1, ItemRemoved (CollectionChange_ItemRemoved) = 2, ItemChanged (CollectionChange_ItemChanged) = 3,
		}}
		DEFINE_IID!(IID_IVectorChangedEventArgs, 1465463775, 13566, 17536, 175, 21, 7, 105, 31, 61, 93, 155);
		RT_INTERFACE!{interface IVectorChangedEventArgs(IVectorChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IVectorChangedEventArgs] {
			fn get_CollectionChange(&mut self, out: *mut ::rt::gen::windows::foundation::collections::CollectionChange) -> ::w::HRESULT,
			fn get_Index(&mut self, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IPropertySet, 2319707551, 62694, 17441, 172, 249, 29, 171, 41, 134, 130, 12);
		RT_INTERFACE!{interface IPropertySet(IPropertySetVtbl): IInspectable(IInspectableVtbl) [IID_IPropertySet] {
			
		}}
		RT_CLASS!(PropertySet: ::rt::gen::windows::foundation::collections::IPropertySet);
		RT_CLASS!(ValueSet: ::rt::gen::windows::foundation::collections::IPropertySet);
		RT_CLASS!(StringMap: ::rt::gen::windows::foundation::collections::IMap<&'static str, &'static str>);
		DEFINE_IID!(IID_IIterable, 4205151722, 25108, 16919, 175, 218, 127, 70, 222, 88, 105, 179);
		RT_INTERFACE!{interface IIterable<T>(IIterableVtbl): IInspectable(IInspectableVtbl) [IID_IIterable] {
			fn First(&mut self, out: *mut *mut ::rt::gen::windows::foundation::collections::IIterator<T>) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IIterator, 1786374243, 17152, 17818, 153, 102, 203, 182, 96, 150, 62, 225);
		RT_INTERFACE!{interface IIterator<T>(IIteratorVtbl): IInspectable(IInspectableVtbl) [IID_IIterator] {
			fn get_Current(&mut self, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_HasCurrent(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn MoveNext(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetMany(&mut self, items: *mut T::Abi, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IVectorView, 3152149068, 45283, 17795, 186, 239, 31, 27, 46, 72, 62, 86);
		RT_INTERFACE!{interface IVectorView<T>(IVectorViewVtbl): IInspectable(IInspectableVtbl) [IID_IVectorView] {
			fn GetAt(&mut self, index: u32, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn IndexOf(&mut self, value: T::Abi, index: *mut u32, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetMany(&mut self, startIndex: u32, items: *mut T::Abi, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IVector, 2436052969, 4513, 17221, 163, 162, 78, 127, 149, 110, 34, 45);
		RT_INTERFACE!{interface IVector<T>(IVectorVtbl): IInspectable(IInspectableVtbl) [IID_IVector] {
			fn GetAt(&mut self, index: u32, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn GetView(&mut self, out: *mut *mut ::rt::gen::windows::foundation::collections::IVectorView<T>) -> ::w::HRESULT,
			fn IndexOf(&mut self, value: T::Abi, index: *mut u32, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn SetAt(&mut self, index: u32, value: T::Abi) -> ::w::HRESULT,
			fn InsertAt(&mut self, index: u32, value: T::Abi) -> ::w::HRESULT,
			fn RemoveAt(&mut self, index: u32) -> ::w::HRESULT,
			fn Append(&mut self, value: T::Abi) -> ::w::HRESULT,
			fn RemoveAtEnd(&mut self) -> ::w::HRESULT,
			fn Clear(&mut self) -> ::w::HRESULT,
			fn GetMany(&mut self, startIndex: u32, items: *mut T::Abi, out: *mut u32) -> ::w::HRESULT,
			fn ReplaceAll(&mut self, items: *mut T::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IKeyValuePair, 45422889, 49604, 19070, 137, 64, 3, 18, 181, 193, 133, 0);
		RT_INTERFACE!{interface IKeyValuePair<K, V>(IKeyValuePairVtbl): IInspectable(IInspectableVtbl) [IID_IKeyValuePair] {
			fn get_Key(&mut self, out: *mut K::Abi) -> ::w::HRESULT,
			fn get_Value(&mut self, out: *mut V::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMap, 1009329662, 34073, 17857, 170, 121, 25, 123, 103, 24, 193, 193);
		RT_INTERFACE!{interface IMap<K, V>(IMapVtbl): IInspectable(IInspectableVtbl) [IID_IMap] {
			fn Lookup(&mut self, key: K::Abi, out: *mut V::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn HasKey(&mut self, key: K::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetView(&mut self, out: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<K, V>) -> ::w::HRESULT,
			fn Insert(&mut self, key: K::Abi, value: V::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Remove(&mut self, key: K::Abi) -> ::w::HRESULT,
			fn Clear(&mut self) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMapView, 3833646656, 41784, 19162, 173, 207, 39, 34, 114, 228, 140, 185);
		RT_INTERFACE!{interface IMapView<K, V>(IMapViewVtbl): IInspectable(IInspectableVtbl) [IID_IMapView] {
			fn Lookup(&mut self, key: K::Abi, out: *mut V::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn HasKey(&mut self, key: K::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Split(&mut self, first: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<K, V>, second: *mut *mut ::rt::gen::windows::foundation::collections::IMapView<K, V>) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_VectorChangedEventHandler, 201660242, 40895, 19568, 170, 12, 14, 76, 130, 217, 167, 97);
		RT_DELEGATE!{delegate VectorChangedEventHandler<T>(VectorChangedEventHandlerVtbl, VectorChangedEventHandlerImpl) [IID_VectorChangedEventHandler] {
			fn Invoke(&mut self, sender: *mut ::rt::gen::windows::foundation::collections::IObservableVector<T>, event: *mut ::rt::gen::windows::foundation::collections::IVectorChangedEventArgs) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IObservableVector, 1494739795, 20660, 18957, 179, 9, 101, 134, 43, 63, 29, 188);
		RT_INTERFACE!{interface IObservableVector<T>(IObservableVectorVtbl): IInspectable(IInspectableVtbl) [IID_IObservableVector] {
			fn add_VectorChanged(&mut self, vhnd: *mut ::rt::gen::windows::foundation::collections::VectorChangedEventHandler<T>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_VectorChanged(&mut self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IMapChangedEventArgs, 2570712287, 1290, 19471, 170, 96, 119, 7, 95, 156, 71, 119);
		RT_INTERFACE!{interface IMapChangedEventArgs<K>(IMapChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_IMapChangedEventArgs] {
			fn get_CollectionChange(&mut self, out: *mut ::rt::gen::windows::foundation::collections::CollectionChange) -> ::w::HRESULT,
			fn get_Key(&mut self, out: *mut K::Abi) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_MapChangedEventHandler, 395646963, 38126, 16888, 189, 220, 118, 138, 137, 85, 68, 243);
		RT_DELEGATE!{delegate MapChangedEventHandler<K, V>(MapChangedEventHandlerVtbl, MapChangedEventHandlerImpl) [IID_MapChangedEventHandler] {
			fn Invoke(&mut self, sender: *mut ::rt::gen::windows::foundation::collections::IObservableMap<K, V>, event: *mut ::rt::gen::windows::foundation::collections::IMapChangedEventArgs<K>) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IObservableMap, 1709124597, 48953, 16821, 174, 188, 90, 157, 134, 94, 71, 43);
		RT_INTERFACE!{interface IObservableMap<K, V>(IObservableMapVtbl): IInspectable(IInspectableVtbl) [IID_IObservableMap] {
			fn add_MapChanged(&mut self, vhnd: *mut ::rt::gen::windows::foundation::collections::MapChangedEventHandler<K, V>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_MapChanged(&mut self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		RT_PINTERFACE!{ for<'a> IMap<&'a str, &'a str> => [0xf6d1f700,0x49c2,0x52ae,0x81,0x54,0x82,0x6f,0x99,0x08,0x77,0x3c] as IID_IMap_2_System_String_System_String }
		RT_PINTERFACE!{ for<'a> IVectorView<&'a ::rt::gen::windows::storage::StorageFile> => [0x80646519,0x5e2a,0x595d,0xa8,0xcd,0x2a,0x24,0xb4,0x06,0x7f,0x1b] as IID_IVectorView_1_Windows_Storage_StorageFile }
		RT_PINTERFACE!{ for<'a> IVectorView<&'a ::rt::gen::windows::storage::StorageFolder> => [0xe20debc6,0xdc4e,0x542e,0xa2,0xe7,0xa2,0x4d,0x19,0xc8,0xdd,0x62] as IID_IVectorView_1_Windows_Storage_StorageFolder }
		RT_PINTERFACE!{ for<'a> IVectorView<&'a ::rt::gen::windows::storage::IStorageItem> => [0x85575a41,0x06cb,0x58d0,0xb9,0x8a,0x7c,0x8f,0x06,0xe6,0xe9,0xd7] as IID_IVectorView_1_Windows_Storage_IStorageItem }
} // Windows.Foundation.Collections
pub mod metadata { // Windows.Foundation.Metadata
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_ENUM! { enum GCPressureAmount: i32 {
			Low (GCPressureAmount_Low) = 0, Medium (GCPressureAmount_Medium) = 1, High (GCPressureAmount_High) = 2,
		}}
		DEFINE_IID!(IID_IApiInformationStatics, 2574531070, 63105, 18961, 180, 22, 193, 58, 71, 232, 186, 54);
		RT_INTERFACE!{interface IApiInformationStatics(IApiInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IApiInformationStatics] {
			fn IsTypePresent(&mut self, typeName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsMethodPresent(&mut self, typeName: ::w::HSTRING, methodName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsMethodPresentWithArity(&mut self, typeName: ::w::HSTRING, methodName: ::w::HSTRING, inputParameterCount: u32, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEventPresent(&mut self, typeName: ::w::HSTRING, eventName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsPropertyPresent(&mut self, typeName: ::w::HSTRING, propertyName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsReadOnlyPropertyPresent(&mut self, typeName: ::w::HSTRING, propertyName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsWriteablePropertyPresent(&mut self, typeName: ::w::HSTRING, propertyName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEnumNamedValuePresent(&mut self, enumTypeName: ::w::HSTRING, valueName: ::w::HSTRING, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsApiContractPresentByMajor(&mut self, contractName: ::w::HSTRING, majorVersion: u16, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsApiContractPresentByMajorAndMinor(&mut self, contractName: ::w::HSTRING, majorVersion: u16, minorVersion: u16, out: *mut ::w::BOOL) -> ::w::HRESULT
		}}
		RT_ENUM! { enum Platform: i32 {
			Windows (Platform_Windows) = 0, WindowsPhone (Platform_WindowsPhone) = 1,
		}}
		RT_ENUM! { enum AttributeTargets: u32 {
			All (AttributeTargets_All) = 4294967295, Delegate (AttributeTargets_Delegate) = 1, Enum (AttributeTargets_Enum) = 2, Event (AttributeTargets_Event) = 4, Field (AttributeTargets_Field) = 8, Interface (AttributeTargets_Interface) = 16, Method (AttributeTargets_Method) = 64, Parameter (AttributeTargets_Parameter) = 128, Property (AttributeTargets_Property) = 256, RuntimeClass (AttributeTargets_RuntimeClass) = 512, Struct (AttributeTargets_Struct) = 1024, InterfaceImpl (AttributeTargets_InterfaceImpl) = 2048, ApiContract (AttributeTargets_ApiContract) = 8192,
		}}
		RT_ENUM! { enum CompositionType: i32 {
			Protected (CompositionType_Protected) = 1, Public (CompositionType_Public) = 2,
		}}
		RT_ENUM! { enum ThreadingModel: i32 {
			STA (ThreadingModel_STA) = 1, MTA (ThreadingModel_MTA) = 2, Both (ThreadingModel_Both) = 3, InvalidThreading (ThreadingModel_InvalidThreading) = 0,
		}}
		RT_ENUM! { enum MarshalingType: i32 {
			None (MarshalingType_None) = 1, Agile (MarshalingType_Agile) = 2, Standard (MarshalingType_Standard) = 3, InvalidMarshaling (MarshalingType_InvalidMarshaling) = 0,
		}}
		RT_ENUM! { enum DeprecationType: i32 {
			Deprecate (DeprecationType_Deprecate) = 0, Remove (DeprecationType_Remove) = 1,
		}}
} // Windows.Foundation.Metadata
pub mod diagnostics { // Windows.Foundation.Diagnostics
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_ENUM! { enum CausalityTraceLevel: i32 {
			Required (CausalityTraceLevel_Required) = 0, Important (CausalityTraceLevel_Important) = 1, Verbose (CausalityTraceLevel_Verbose) = 2,
		}}
		RT_ENUM! { enum CausalitySource: i32 {
			Application (CausalitySource_Application) = 0, Library (CausalitySource_Library) = 1, System (CausalitySource_System) = 2,
		}}
		RT_ENUM! { enum CausalityRelation: i32 {
			AssignDelegate (CausalityRelation_AssignDelegate) = 0, Join (CausalityRelation_Join) = 1, Choice (CausalityRelation_Choice) = 2, Cancel (CausalityRelation_Cancel) = 3, Error (CausalityRelation_Error) = 4,
		}}
		RT_ENUM! { enum CausalitySynchronousWork: i32 {
			CompletionNotification (CausalitySynchronousWork_CompletionNotification) = 0, ProgressNotification (CausalitySynchronousWork_ProgressNotification) = 1, Execution (CausalitySynchronousWork_Execution) = 2,
		}}
		DEFINE_IID!(IID_ITracingStatusChangedEventArgs, 1091270417, 65339, 18303, 156, 154, 210, 239, 218, 48, 45, 195);
		RT_INTERFACE!{interface ITracingStatusChangedEventArgs(ITracingStatusChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ITracingStatusChangedEventArgs] {
			fn get_Enabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn get_TraceLevel(&mut self, out: *mut ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IAsyncCausalityTracerStatics, 1350896422, 9854, 17691, 168, 144, 171, 106, 55, 2, 69, 238);
		RT_INTERFACE!{interface IAsyncCausalityTracerStatics(IAsyncCausalityTracerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IAsyncCausalityTracerStatics] {
			fn TraceOperationCreation(&mut self, traceLevel: ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel, source: ::rt::gen::windows::foundation::diagnostics::CausalitySource, platformId: ::w::GUID, operationId: u64, operationName: ::w::HSTRING, relatedContext: u64) -> ::w::HRESULT,
			fn TraceOperationCompletion(&mut self, traceLevel: ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel, source: ::rt::gen::windows::foundation::diagnostics::CausalitySource, platformId: ::w::GUID, operationId: u64, status: ::rt::gen::windows::foundation::AsyncStatus) -> ::w::HRESULT,
			fn TraceOperationRelation(&mut self, traceLevel: ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel, source: ::rt::gen::windows::foundation::diagnostics::CausalitySource, platformId: ::w::GUID, operationId: u64, relation: ::rt::gen::windows::foundation::diagnostics::CausalityRelation) -> ::w::HRESULT,
			fn TraceSynchronousWorkStart(&mut self, traceLevel: ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel, source: ::rt::gen::windows::foundation::diagnostics::CausalitySource, platformId: ::w::GUID, operationId: u64, work: ::rt::gen::windows::foundation::diagnostics::CausalitySynchronousWork) -> ::w::HRESULT,
			fn TraceSynchronousWorkCompletion(&mut self, traceLevel: ::rt::gen::windows::foundation::diagnostics::CausalityTraceLevel, source: ::rt::gen::windows::foundation::diagnostics::CausalitySource, work: ::rt::gen::windows::foundation::diagnostics::CausalitySynchronousWork) -> ::w::HRESULT,
			fn add_TracingStatusChanged(&mut self, handler: *mut ::rt::gen::windows::foundation::EventHandler<&::rt::gen::windows::foundation::diagnostics::TracingStatusChangedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_TracingStatusChanged(&mut self, cookie: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		RT_CLASS!(TracingStatusChangedEventArgs: ::rt::gen::windows::foundation::diagnostics::ITracingStatusChangedEventArgs);
		RT_ENUM! { enum ErrorOptions: u32 {
			None (ErrorOptions_None) = 0, SuppressExceptions (ErrorOptions_SuppressExceptions) = 1, ForceExceptions (ErrorOptions_ForceExceptions) = 2, UseSetErrorInfo (ErrorOptions_UseSetErrorInfo) = 4, SuppressSetErrorInfo (ErrorOptions_SuppressSetErrorInfo) = 8,
		}}
		DEFINE_IID!(IID_IErrorReportingSettings, 372676498, 45118, 19361, 139, 184, 210, 143, 74, 180, 210, 192);
		RT_INTERFACE!{interface IErrorReportingSettings(IErrorReportingSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IErrorReportingSettings] {
			fn SetErrorOptions(&mut self, value: ::rt::gen::windows::foundation::diagnostics::ErrorOptions) -> ::w::HRESULT,
			fn GetErrorOptions(&mut self, out: *mut ::rt::gen::windows::foundation::diagnostics::ErrorOptions) -> ::w::HRESULT
		}}
		RT_CLASS!(RuntimeBrokerErrorSettings: ::rt::gen::windows::foundation::diagnostics::IErrorReportingSettings);
		DEFINE_IID!(IID_IErrorDetailsStatics, 3077584720, 2845, 18120, 170, 14, 75, 129, 120, 228, 252, 233);
		RT_INTERFACE!{interface IErrorDetailsStatics(IErrorDetailsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IErrorDetailsStatics] {
			fn CreateFromHResultAsync(&mut self, errorCode: i32, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::foundation::diagnostics::ErrorDetails>) -> ::w::HRESULT
		}}
		RT_CLASS!(ErrorDetails: ::rt::gen::windows::foundation::diagnostics::IErrorDetails);
		DEFINE_IID!(IID_IErrorDetails, 931969793, 11465, 17039, 140, 85, 44, 153, 13, 70, 62, 143);
		RT_INTERFACE!{interface IErrorDetails(IErrorDetailsVtbl): IInspectable(IInspectableVtbl) [IID_IErrorDetails] {
			fn get_Description(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_LongDescription(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_HelpUri(&mut self, out: *mut *mut ::rt::gen::windows::foundation::Uri) -> ::w::HRESULT
		}}
		RT_ENUM! { enum LoggingLevel: i32 {
			Verbose (LoggingLevel_Verbose) = 0, Information (LoggingLevel_Information) = 1, Warning (LoggingLevel_Warning) = 2, Error (LoggingLevel_Error) = 3, Critical (LoggingLevel_Critical) = 4,
		}}
		RT_ENUM! { enum LoggingOpcode: i32 {
			Info (LoggingOpcode_Info) = 0, Start (LoggingOpcode_Start) = 1, Stop (LoggingOpcode_Stop) = 2, Reply (LoggingOpcode_Reply) = 6, Resume (LoggingOpcode_Resume) = 7, Suspend (LoggingOpcode_Suspend) = 8, Send (LoggingOpcode_Send) = 9,
		}}
		RT_ENUM! { enum LoggingFieldFormat: i32 {
			Default (LoggingFieldFormat_Default) = 0, Hidden (LoggingFieldFormat_Hidden) = 1, String (LoggingFieldFormat_String) = 2, Boolean (LoggingFieldFormat_Boolean) = 3, Hexadecimal (LoggingFieldFormat_Hexadecimal) = 4, ProcessId (LoggingFieldFormat_ProcessId) = 5, ThreadId (LoggingFieldFormat_ThreadId) = 6, Port (LoggingFieldFormat_Port) = 7, Ipv4Address (LoggingFieldFormat_Ipv4Address) = 8, Ipv6Address (LoggingFieldFormat_Ipv6Address) = 9, SocketAddress (LoggingFieldFormat_SocketAddress) = 10, Xml (LoggingFieldFormat_Xml) = 11, Json (LoggingFieldFormat_Json) = 12, Win32Error (LoggingFieldFormat_Win32Error) = 13, NTStatus (LoggingFieldFormat_NTStatus) = 14, HResult (LoggingFieldFormat_HResult) = 15, FileTime (LoggingFieldFormat_FileTime) = 16, Signed (LoggingFieldFormat_Signed) = 17, Unsigned (LoggingFieldFormat_Unsigned) = 18,
		}}
		DEFINE_IID!(IID_ILoggingOptions, 2428270672, 402, 20317, 172, 38, 0, 106, 218, 202, 18, 216);
		RT_INTERFACE!{interface ILoggingOptions(ILoggingOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingOptions] {
			fn get_Keywords(&mut self, out: *mut i64) -> ::w::HRESULT,
			fn put_Keywords(&mut self, value: i64) -> ::w::HRESULT,
			fn get_Tags(&mut self, out: *mut i32) -> ::w::HRESULT,
			fn put_Tags(&mut self, value: i32) -> ::w::HRESULT,
			fn get_Task(&mut self, out: *mut i16) -> ::w::HRESULT,
			fn put_Task(&mut self, value: i16) -> ::w::HRESULT,
			fn get_Opcode(&mut self, out: *mut ::rt::gen::windows::foundation::diagnostics::LoggingOpcode) -> ::w::HRESULT,
			fn put_Opcode(&mut self, value: ::rt::gen::windows::foundation::diagnostics::LoggingOpcode) -> ::w::HRESULT,
			fn get_ActivityId(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_ActivityId(&mut self, value: ::w::GUID) -> ::w::HRESULT,
			fn get_RelatedActivityId(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_RelatedActivityId(&mut self, value: ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingOptionsFactory, 3608397515, 39083, 17995, 159, 34, 163, 38, 132, 120, 54, 138);
		RT_INTERFACE!{interface ILoggingOptionsFactory(ILoggingOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingOptionsFactory] {
			fn CreateWithKeywords(&mut self, keywords: i64, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingOptions) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingOptions: ::rt::gen::windows::foundation::diagnostics::ILoggingOptions);
		DEFINE_IID!(IID_ILoggingChannelOptions, 3286779903, 3771, 19027, 140, 84, 222, 194, 73, 38, 203, 44);
		RT_INTERFACE!{interface ILoggingChannelOptions(ILoggingChannelOptionsVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannelOptions] {
			fn get_Group(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_Group(&mut self, value: ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingChannelOptionsFactory, 2838581722, 32687, 16785, 135, 85, 94, 134, 220, 101, 216, 150);
		RT_INTERFACE!{interface ILoggingChannelOptionsFactory(ILoggingChannelOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannelOptionsFactory] {
			fn Create(&mut self, group: ::w::GUID, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannelOptions) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingChannelOptions: ::rt::gen::windows::foundation::diagnostics::ILoggingChannelOptions);
		DEFINE_IID!(IID_ILoggingFields, 3623270319, 30253, 17785, 131, 189, 82, 194, 59, 195, 51, 188);
		RT_INTERFACE!{interface ILoggingFields(ILoggingFieldsVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingFields] {
			fn Clear(&mut self) -> ::w::HRESULT,
			fn BeginStruct(&mut self, name: ::w::HSTRING) -> ::w::HRESULT,
			fn BeginStructWithTags(&mut self, name: ::w::HSTRING, tags: i32) -> ::w::HRESULT,
			fn EndStruct(&mut self) -> ::w::HRESULT,
			fn AddEmpty(&mut self, name: ::w::HSTRING) -> ::w::HRESULT,
			fn AddEmptyWithFormat(&mut self, name: ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddEmptyWithFormatAndTags(&mut self, name: ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt8(&mut self, name: ::w::HSTRING, value: u8) -> ::w::HRESULT,
			fn AddUInt8WithFormat(&mut self, name: ::w::HSTRING, value: u8, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt8WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u8, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt8Array(&mut self, name: ::w::HSTRING, value: *mut u8) -> ::w::HRESULT,
			fn AddUInt8ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u8, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt8ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u8, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt16(&mut self, name: ::w::HSTRING, value: i16) -> ::w::HRESULT,
			fn AddInt16WithFormat(&mut self, name: ::w::HSTRING, value: i16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt16Array(&mut self, name: ::w::HSTRING, value: *mut i16) -> ::w::HRESULT,
			fn AddInt16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt16(&mut self, name: ::w::HSTRING, value: u16) -> ::w::HRESULT,
			fn AddUInt16WithFormat(&mut self, name: ::w::HSTRING, value: u16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt16Array(&mut self, name: ::w::HSTRING, value: *mut u16) -> ::w::HRESULT,
			fn AddUInt16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u16, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt32(&mut self, name: ::w::HSTRING, value: i32) -> ::w::HRESULT,
			fn AddInt32WithFormat(&mut self, name: ::w::HSTRING, value: i32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt32WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt32Array(&mut self, name: ::w::HSTRING, value: *mut i32) -> ::w::HRESULT,
			fn AddInt32ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt32ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt32(&mut self, name: ::w::HSTRING, value: u32) -> ::w::HRESULT,
			fn AddUInt32WithFormat(&mut self, name: ::w::HSTRING, value: u32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt32WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt32Array(&mut self, name: ::w::HSTRING, value: *mut u32) -> ::w::HRESULT,
			fn AddUInt32ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt32ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt64(&mut self, name: ::w::HSTRING, value: i64) -> ::w::HRESULT,
			fn AddInt64WithFormat(&mut self, name: ::w::HSTRING, value: i64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt64WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt64Array(&mut self, name: ::w::HSTRING, value: *mut i64) -> ::w::HRESULT,
			fn AddInt64ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt64ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt64(&mut self, name: ::w::HSTRING, value: u64) -> ::w::HRESULT,
			fn AddUInt64WithFormat(&mut self, name: ::w::HSTRING, value: u64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt64WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt64Array(&mut self, name: ::w::HSTRING, value: *mut u64) -> ::w::HRESULT,
			fn AddUInt64ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt64ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSingle(&mut self, name: ::w::HSTRING, value: f32) -> ::w::HRESULT,
			fn AddSingleWithFormat(&mut self, name: ::w::HSTRING, value: f32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSingleWithFormatAndTags(&mut self, name: ::w::HSTRING, value: f32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSingleArray(&mut self, name: ::w::HSTRING, value: *mut f32) -> ::w::HRESULT,
			fn AddSingleArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut f32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSingleArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut f32, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDouble(&mut self, name: ::w::HSTRING, value: f64) -> ::w::HRESULT,
			fn AddDoubleWithFormat(&mut self, name: ::w::HSTRING, value: f64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDoubleWithFormatAndTags(&mut self, name: ::w::HSTRING, value: f64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDoubleArray(&mut self, name: ::w::HSTRING, value: *mut f64) -> ::w::HRESULT,
			fn AddDoubleArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut f64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDoubleArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut f64, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddChar16(&mut self, name: ::w::HSTRING, value: ::w::wchar_t) -> ::w::HRESULT,
			fn AddChar16WithFormat(&mut self, name: ::w::HSTRING, value: ::w::wchar_t, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddChar16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::wchar_t, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddChar16Array(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t) -> ::w::HRESULT,
			fn AddChar16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddChar16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddBoolean(&mut self, name: ::w::HSTRING, value: ::w::BOOL) -> ::w::HRESULT,
			fn AddBooleanWithFormat(&mut self, name: ::w::HSTRING, value: ::w::BOOL, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddBooleanWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::BOOL, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddBooleanArray(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL) -> ::w::HRESULT,
			fn AddBooleanArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddBooleanArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddString(&mut self, name: ::w::HSTRING, value: ::w::HSTRING) -> ::w::HRESULT,
			fn AddStringWithFormat(&mut self, name: ::w::HSTRING, value: ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddStringWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddStringArray(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn AddStringArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddStringArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddGuid(&mut self, name: ::w::HSTRING, value: ::w::GUID) -> ::w::HRESULT,
			fn AddGuidWithFormat(&mut self, name: ::w::HSTRING, value: ::w::GUID, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddGuidWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::GUID, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddGuidArray(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID) -> ::w::HRESULT,
			fn AddGuidArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddGuidArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDateTime(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn AddDateTimeWithFormat(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::DateTime, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDateTimeWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::DateTime, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDateTimeArray(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn AddDateTimeArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::DateTime, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDateTimeArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::DateTime, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddTimeSpan(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::TimeSpan) -> ::w::HRESULT,
			fn AddTimeSpanWithFormat(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::TimeSpan, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddTimeSpanWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::TimeSpan, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddTimeSpanArray(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::TimeSpan) -> ::w::HRESULT,
			fn AddTimeSpanArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::TimeSpan, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddTimeSpanArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::TimeSpan, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddPoint(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Point) -> ::w::HRESULT,
			fn AddPointWithFormat(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Point, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddPointWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Point, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddPointArray(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Point) -> ::w::HRESULT,
			fn AddPointArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Point, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddPointArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Point, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSize(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Size) -> ::w::HRESULT,
			fn AddSizeWithFormat(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Size, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSizeWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Size, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSizeArray(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Size) -> ::w::HRESULT,
			fn AddSizeArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Size, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSizeArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Size, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddRect(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Rect) -> ::w::HRESULT,
			fn AddRectWithFormat(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Rect, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddRectWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::rt::gen::windows::foundation::Rect, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddRectArray(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Rect) -> ::w::HRESULT,
			fn AddRectArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Rect, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat) -> ::w::HRESULT,
			fn AddRectArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::rt::gen::windows::foundation::Rect, format: ::rt::gen::windows::foundation::diagnostics::LoggingFieldFormat, tags: i32) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingFields: ::rt::gen::windows::foundation::diagnostics::ILoggingFields);
		DEFINE_IID!(IID_ILoggingTarget, 1710320693, 58248, 20006, 177, 122, 245, 28, 211, 168, 57, 22);
		RT_INTERFACE!{interface ILoggingTarget(ILoggingTargetVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingTarget] {
			fn IsEnabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEnabledWithLevel(&mut self, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEnabledWithLevelAndKeywords(&mut self, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, keywords: i64, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn LogEvent(&mut self, eventName: ::w::HSTRING) -> ::w::HRESULT,
			fn LogEventWithFields(&mut self, eventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields) -> ::w::HRESULT,
			fn LogEventWithFieldsAndLevel(&mut self, eventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn LogEventWithFieldsAndOptions(&mut self, eventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, options: *mut ::rt::gen::windows::foundation::diagnostics::LoggingOptions) -> ::w::HRESULT,
			fn StartActivity(&mut self, startEventName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFields(&mut self, startEventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFieldsAndLevel(&mut self, startEventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFieldsAndOptions(&mut self, startEventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, options: *mut ::rt::gen::windows::foundation::diagnostics::LoggingOptions, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingActivity: ::rt::gen::windows::foundation::diagnostics::ILoggingActivity);
		DEFINE_IID!(IID_ILoggingChannel, 3919905603, 4567, 20225, 181, 202, 207, 73, 82, 120, 192, 168);
		RT_INTERFACE!{interface ILoggingChannel(ILoggingChannelVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannel] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Enabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn get_Level(&mut self, out: *mut ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn LogMessage(&mut self, eventString: ::w::HSTRING) -> ::w::HRESULT,
			fn LogMessageWithLevel(&mut self, eventString: ::w::HSTRING, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn LogValuePair(&mut self, value1: ::w::HSTRING, value2: i32) -> ::w::HRESULT,
			fn LogValuePairWithLevel(&mut self, value1: ::w::HSTRING, value2: i32, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn add_LoggingEnabled(&mut self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<&::rt::gen::windows::foundation::diagnostics::ILoggingChannel, &IInspectable>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_LoggingEnabled(&mut self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingChannel2, 2672573683, 2988, 17829, 158, 51, 186, 243, 243, 162, 70, 165);
		RT_INTERFACE!{interface ILoggingChannel2(ILoggingChannel2Vtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannel2] {
			fn get_Id(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingChannelFactory, 1323064220, 44928, 19099, 176, 220, 57, 143, 154, 229, 32, 123);
		RT_INTERFACE!{interface ILoggingChannelFactory(ILoggingChannelFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannelFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannel) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingChannel: ::rt::gen::windows::foundation::diagnostics::ILoggingChannel);
		DEFINE_IID!(IID_ILoggingChannelFactory2, 1282340317, 15143, 19913, 153, 240, 41, 156, 110, 70, 3, 161);
		RT_INTERFACE!{interface ILoggingChannelFactory2(ILoggingChannelFactory2Vtbl): IInspectable(IInspectableVtbl) [IID_ILoggingChannelFactory2] {
			fn CreateWithOptions(&mut self, name: ::w::HSTRING, options: *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannelOptions, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannel) -> ::w::HRESULT,
			fn CreateWithOptionsAndId(&mut self, name: ::w::HSTRING, options: *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannelOptions, id: ::w::GUID, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannel) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingActivity, 3154323777, 46950, 19637, 152, 72, 151, 172, 107, 166, 214, 12);
		RT_INTERFACE!{interface ILoggingActivity(ILoggingActivityVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingActivity] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Id(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingActivity2, 650287112, 25378, 17770, 175, 130, 128, 200, 100, 47, 23, 139);
		RT_INTERFACE!{interface ILoggingActivity2(ILoggingActivity2Vtbl): IInspectable(IInspectableVtbl) [IID_ILoggingActivity2] {
			fn get_Channel(&mut self, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingChannel) -> ::w::HRESULT,
			fn StopActivity(&mut self, stopEventName: ::w::HSTRING) -> ::w::HRESULT,
			fn StopActivityWithFields(&mut self, stopEventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields) -> ::w::HRESULT,
			fn StopActivityWithFieldsAndOptions(&mut self, stopEventName: ::w::HSTRING, fields: *mut ::rt::gen::windows::foundation::diagnostics::LoggingFields, options: *mut ::rt::gen::windows::foundation::diagnostics::LoggingOptions) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingActivityFactory, 1798550659, 57610, 19544, 151, 213, 16, 251, 69, 16, 116, 251);
		RT_INTERFACE!{interface ILoggingActivityFactory(ILoggingActivityFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingActivityFactory] {
			fn CreateLoggingActivity(&mut self, activityName: ::w::HSTRING, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT,
			fn CreateLoggingActivityWithLevel(&mut self, activityName: ::w::HSTRING, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel, level: ::rt::gen::windows::foundation::diagnostics::LoggingLevel, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingActivity) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingSession, 1646392070, 37760, 19159, 186, 245, 65, 234, 147, 16, 215, 104);
		RT_INTERFACE!{interface ILoggingSession(ILoggingSessionVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingSession] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn SaveToFileAsync(&mut self, folder: *mut ::rt::gen::windows::storage::IStorageFolder, fileName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn AddLoggingChannel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel) -> ::w::HRESULT,
			fn AddLoggingChannelWithLevel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel, maxLevel: ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn RemoveLoggingChannel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_ILoggingSessionFactory, 1318289125, 22781, 17888, 140, 47, 161, 50, 239, 249, 92, 30);
		RT_INTERFACE!{interface ILoggingSessionFactory(ILoggingSessionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ILoggingSessionFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::LoggingSession) -> ::w::HRESULT
		}}
		RT_CLASS!(LoggingSession: ::rt::gen::windows::foundation::diagnostics::ILoggingSession);
		DEFINE_IID!(IID_ILogFileGeneratedEventArgs, 647927663, 3384, 19482, 181, 63, 179, 149, 216, 129, 223, 132);
		RT_INTERFACE!{interface ILogFileGeneratedEventArgs(ILogFileGeneratedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_ILogFileGeneratedEventArgs] {
			fn get_File(&mut self, out: *mut *mut ::rt::gen::windows::storage::StorageFile) -> ::w::HRESULT
		}}
		RT_CLASS!(LogFileGeneratedEventArgs: ::rt::gen::windows::foundation::diagnostics::ILogFileGeneratedEventArgs);
		DEFINE_IID!(IID_IFileLoggingSession, 617038358, 65234, 16460, 137, 95, 31, 150, 153, 203, 2, 247);
		RT_INTERFACE!{interface IFileLoggingSession(IFileLoggingSessionVtbl): IInspectable(IInspectableVtbl) [IID_IFileLoggingSession] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn AddLoggingChannel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel) -> ::w::HRESULT,
			fn AddLoggingChannelWithLevel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel, maxLevel: ::rt::gen::windows::foundation::diagnostics::LoggingLevel) -> ::w::HRESULT,
			fn RemoveLoggingChannel(&mut self, loggingChannel: *mut ::rt::gen::windows::foundation::diagnostics::ILoggingChannel) -> ::w::HRESULT,
			fn CloseAndSaveToFileAsync(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn add_LogFileGenerated(&mut self, handler: *mut ::rt::gen::windows::foundation::TypedEventHandler<&::rt::gen::windows::foundation::diagnostics::IFileLoggingSession, &::rt::gen::windows::foundation::diagnostics::LogFileGeneratedEventArgs>, out: *mut ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT,
			fn remove_LogFileGenerated(&mut self, token: ::rt::gen::windows::foundation::EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IFileLoggingSessionFactory, 4003499470, 33863, 19882, 145, 51, 18, 235, 70, 246, 151, 212);
		RT_INTERFACE!{interface IFileLoggingSessionFactory(IFileLoggingSessionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IFileLoggingSessionFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::diagnostics::FileLoggingSession) -> ::w::HRESULT
		}}
		RT_CLASS!(FileLoggingSession: ::rt::gen::windows::foundation::diagnostics::IFileLoggingSession);
} // Windows.Foundation.Diagnostics
pub mod numerics { // Windows.Foundation.Numerics
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_STRUCT! { struct Vector2 {
			X: f32, Y: f32,
		}}
		RT_STRUCT! { struct Vector3 {
			X: f32, Y: f32, Z: f32,
		}}
		RT_STRUCT! { struct Vector4 {
			X: f32, Y: f32, Z: f32, W: f32,
		}}
		RT_STRUCT! { struct Matrix3x2 {
			M11: f32, M12: f32, M21: f32, M22: f32, M31: f32, M32: f32,
		}}
		RT_STRUCT! { struct Matrix4x4 {
			M11: f32, M12: f32, M13: f32, M14: f32, M21: f32, M22: f32, M23: f32, M24: f32, M31: f32, M32: f32, M33: f32, M34: f32, M41: f32, M42: f32, M43: f32, M44: f32,
		}}
		RT_STRUCT! { struct Plane {
			Normal: ::rt::gen::windows::foundation::numerics::Vector3, D: f32,
		}}
		RT_STRUCT! { struct Quaternion {
			X: f32, Y: f32, Z: f32, W: f32,
		}}
} // Windows.Foundation.Numerics
} // Windows.Foundation
pub mod storage { // Windows.Storage
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_CLASS!(StorageFile: ::rt::gen::windows::storage::IStorageFile);
		DEFINE_IID!(IID_IStorageFile, 4198457734, 16916, 17036, 166, 76, 20, 201, 172, 115, 21, 234);
		RT_INTERFACE!{interface IStorageFile(IStorageFileVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFile] {
			fn get_FileType(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_ContentType(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn OpenAsync(&mut self, accessMode: ::rt::gen::windows::storage::FileAccessMode, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::streams::IRandomAccessStream>) -> ::w::HRESULT,
			fn OpenTransactedWriteAsync(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageStreamTransaction>) -> ::w::HRESULT,
			fn CopyOverloadDefaultNameAndOptions(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn CopyOverloadDefaultOptions(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, desiredNewName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn CopyOverload(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, desiredNewName: ::w::HSTRING, option: ::rt::gen::windows::storage::NameCollisionOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn CopyAndReplaceAsync(&mut self, fileToReplace: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn MoveOverloadDefaultNameAndOptions(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn MoveOverloadDefaultOptions(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, desiredNewName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn MoveOverload(&mut self, destinationFolder: *mut ::rt::gen::windows::storage::IStorageFolder, desiredNewName: ::w::HSTRING, option: ::rt::gen::windows::storage::NameCollisionOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn MoveAndReplaceAsync(&mut self, fileToReplace: *mut ::rt::gen::windows::storage::IStorageFile, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT
		}}
		RT_ENUM! { enum NameCollisionOption: i32 {
			GenerateUniqueName (NameCollisionOption_GenerateUniqueName) = 0, ReplaceExisting (NameCollisionOption_ReplaceExisting) = 1, FailIfExists (NameCollisionOption_FailIfExists) = 2,
		}}
		RT_CLASS!(StorageStreamTransaction: ::rt::gen::windows::storage::IStorageStreamTransaction);
		DEFINE_IID!(IID_IStorageStreamTransaction, 4135383907, 42301, 19860, 174, 44, 103, 35, 45, 147, 172, 221);
		RT_INTERFACE!{interface IStorageStreamTransaction(IStorageStreamTransactionVtbl): IInspectable(IInspectableVtbl) [IID_IStorageStreamTransaction] {
			fn get_Stream(&mut self, out: *mut *mut ::rt::gen::windows::storage::streams::IRandomAccessStream) -> ::w::HRESULT,
			fn CommitAsync(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT
		}}
		RT_ENUM! { enum FileAccessMode: i32 {
			Read (FileAccessMode_Read) = 0, ReadWrite (FileAccessMode_ReadWrite) = 1,
		}}
		DEFINE_IID!(IID_IStorageFolder, 1926351736, 46063, 20341, 168, 11, 111, 217, 218, 226, 148, 75);
		RT_INTERFACE!{interface IStorageFolder(IStorageFolderVtbl): IInspectable(IInspectableVtbl) [IID_IStorageFolder] {
			fn CreateFileAsyncOverloadDefaultOptions(&mut self, desiredName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn CreateFileAsync(&mut self, desiredName: ::w::HSTRING, options: ::rt::gen::windows::storage::CreationCollisionOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn CreateFolderAsyncOverloadDefaultOptions(&mut self, desiredName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFolder>) -> ::w::HRESULT,
			fn CreateFolderAsync(&mut self, desiredName: ::w::HSTRING, options: ::rt::gen::windows::storage::CreationCollisionOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFolder>) -> ::w::HRESULT,
			fn GetFileAsync(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFile>) -> ::w::HRESULT,
			fn GetFolderAsync(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::StorageFolder>) -> ::w::HRESULT,
			fn GetItemAsync(&mut self, name: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::IStorageItem>) -> ::w::HRESULT,
			fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::foundation::collections::IVectorView<&::rt::gen::windows::storage::StorageFile>>) -> ::w::HRESULT,
			fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::foundation::collections::IVectorView<&::rt::gen::windows::storage::StorageFolder>>) -> ::w::HRESULT,
			fn GetItemsAsyncOverloadDefaultStartAndCount(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::foundation::collections::IVectorView<&::rt::gen::windows::storage::IStorageItem>>) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IStorageItem, 1107798422, 51759, 17143, 189, 232, 139, 16, 69, 122, 127, 48);
		RT_INTERFACE!{interface IStorageItem(IStorageItemVtbl): IInspectable(IInspectableVtbl) [IID_IStorageItem] {
			fn RenameAsyncOverloadDefaultOptions(&mut self, desiredName: ::w::HSTRING, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn RenameAsync(&mut self, desiredName: ::w::HSTRING, option: ::rt::gen::windows::storage::NameCollisionOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn DeleteAsyncOverloadDefaultOptions(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn DeleteAsync(&mut self, option: ::rt::gen::windows::storage::StorageDeleteOption, out: *mut *mut ::rt::gen::windows::foundation::IAsyncAction) -> ::w::HRESULT,
			fn GetBasicPropertiesAsync(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<&::rt::gen::windows::storage::fileproperties::BasicProperties>) -> ::w::HRESULT,
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Path(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Attributes(&mut self, out: *mut ::rt::gen::windows::storage::FileAttributes) -> ::w::HRESULT,
			fn get_DateCreated(&mut self, out: *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn IsOfType(&mut self, type_: ::rt::gen::windows::storage::StorageItemTypes, out: *mut ::w::BOOL) -> ::w::HRESULT
		}}
		RT_ENUM! { enum StorageItemTypes: u32 {
			None (StorageItemTypes_None) = 0, File (StorageItemTypes_File) = 1, Folder (StorageItemTypes_Folder) = 2,
		}}
		RT_ENUM! { enum FileAttributes: u32 {
			Normal (FileAttributes_Normal) = 0, ReadOnly (FileAttributes_ReadOnly) = 1, Directory (FileAttributes_Directory) = 16, Archive (FileAttributes_Archive) = 32, Temporary (FileAttributes_Temporary) = 256, LocallyIncomplete (FileAttributes_LocallyIncomplete) = 512,
		}}
		RT_ENUM! { enum StorageDeleteOption: i32 {
			Default (StorageDeleteOption_Default) = 0, PermanentDelete (StorageDeleteOption_PermanentDelete) = 1,
		}}
		RT_CLASS!(StorageFolder: ::rt::gen::windows::storage::IStorageFolder);
		RT_ENUM! { enum CreationCollisionOption: i32 {
			GenerateUniqueName (CreationCollisionOption_GenerateUniqueName) = 0, ReplaceExisting (CreationCollisionOption_ReplaceExisting) = 1, FailIfExists (CreationCollisionOption_FailIfExists) = 2, OpenIfExists (CreationCollisionOption_OpenIfExists) = 3,
		}}
pub mod streams { // Windows.Storage.Streams
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		DEFINE_IID!(IID_IRandomAccessStream, 2421821409, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
		RT_INTERFACE!{interface IRandomAccessStream(IRandomAccessStreamVtbl): IInspectable(IInspectableVtbl) [IID_IRandomAccessStream] {
			fn get_Size(&mut self, out: *mut u64) -> ::w::HRESULT,
			fn put_Size(&mut self, value: u64) -> ::w::HRESULT,
			fn GetInputStreamAt(&mut self, position: u64, out: *mut *mut ::rt::gen::windows::storage::streams::IInputStream) -> ::w::HRESULT,
			fn GetOutputStreamAt(&mut self, position: u64, out: *mut *mut ::rt::gen::windows::storage::streams::IOutputStream) -> ::w::HRESULT,
			fn get_Position(&mut self, out: *mut u64) -> ::w::HRESULT,
			fn Seek(&mut self, position: u64) -> ::w::HRESULT,
			fn CloneStream(&mut self, out: *mut *mut ::rt::gen::windows::storage::streams::IRandomAccessStream) -> ::w::HRESULT,
			fn get_CanRead(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn get_CanWrite(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IOutputStream, 2421821414, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
		RT_INTERFACE!{interface IOutputStream(IOutputStreamVtbl): IInspectable(IInspectableVtbl) [IID_IOutputStream] {
			fn WriteAsync(&mut self, buffer: *mut ::rt::gen::windows::storage::streams::IBuffer, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<u32, u32>) -> ::w::HRESULT,
			fn FlushAsync(&mut self, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperation<bool>) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IBuffer, 2421821408, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
		RT_INTERFACE!{interface IBuffer(IBufferVtbl): IInspectable(IInspectableVtbl) [IID_IBuffer] {
			fn get_Capacity(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn get_Length(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn put_Length(&mut self, value: u32) -> ::w::HRESULT
		}}
		DEFINE_IID!(IID_IInputStream, 2421821410, 48211, 4575, 140, 73, 0, 30, 79, 198, 134, 218);
		RT_INTERFACE!{interface IInputStream(IInputStreamVtbl): IInspectable(IInspectableVtbl) [IID_IInputStream] {
			fn ReadAsync(&mut self, buffer: *mut ::rt::gen::windows::storage::streams::IBuffer, count: u32, options: ::rt::gen::windows::storage::streams::InputStreamOptions, out: *mut *mut ::rt::gen::windows::foundation::IAsyncOperationWithProgress<&::rt::gen::windows::storage::streams::IBuffer, u32>) -> ::w::HRESULT
		}}
		RT_ENUM! { enum InputStreamOptions: u32 {
			None (InputStreamOptions_None) = 0, Partial (InputStreamOptions_Partial) = 1, ReadAhead (InputStreamOptions_ReadAhead) = 2,
		}}
} // Windows.Storage.Streams
pub mod fileproperties { // Windows.Storage.FileProperties
use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable}; use ::rt::handler::IntoInterface;
		RT_CLASS!(BasicProperties: ::rt::gen::windows::storage::fileproperties::IBasicProperties);
		DEFINE_IID!(IID_IBasicProperties, 3495777755, 30814, 19046, 190, 2, 155, 238, 197, 138, 234, 129);
		RT_INTERFACE!{interface IBasicProperties(IBasicPropertiesVtbl): IInspectable(IInspectableVtbl) [IID_IBasicProperties] {
			fn get_Size(&mut self, out: *mut u64) -> ::w::HRESULT,
			fn get_DateModified(&mut self, out: *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT,
			fn get_ItemDate(&mut self, out: *mut ::rt::gen::windows::foundation::DateTime) -> ::w::HRESULT
		}}
} // Windows.Storage.FileProperties
} // Windows.Storage
} // Windows
