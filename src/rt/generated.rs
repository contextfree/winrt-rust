#![allow(non_camel_case_types)]
use std::ptr;
use super::super::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use super::{RtInterface, RtType, IInspectable, Windows_Storage_StorageFile, Windows_Storage_IStorageFolder};

		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Collections_CollectionChange
		{
			Reset = 0, ItemInserted = 1, ItemRemoved = 2, ItemChanged = 3
		}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IVectorChangedEventArgs, 1465463775, 13566, 17536, 175, 21, 7, 105, 31, 61, 93, 155);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IVectorChangedEventArgs(Windows_Foundation_Collections_IVectorChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IVectorChangedEventArgs] {
			fn get_CollectionChange(&mut self, out: *mut Windows_Foundation_Collections_CollectionChange) -> ::w::HRESULT,
			fn get_Index(&mut self, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IClosable, 819308585, 32676, 16422, 131, 187, 215, 91, 174, 78, 169, 158);
		RT_INTERFACE!{interface Windows_Foundation_IClosable(Windows_Foundation_IClosableVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IClosable] {
			fn Close(&mut self) -> ::w::HRESULT
		}}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_GCPressureAmount
		{
			Low = 0, Medium = 1, High = 2
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_PropertyType
		{
			Empty = 0, UInt8 = 1, Int16 = 2, UInt16 = 3, Int32 = 4, UInt32 = 5, Int64 = 6, UInt64 = 7, Single = 8, Double = 9, Char16 = 10, Boolean = 11, String = 12, Inspectable = 13, DateTime = 14, TimeSpan = 15, Guid = 16, Point = 17, Size = 18, Rect = 19, OtherType = 20, UInt8Array = 1025, Int16Array = 1026, UInt16Array = 1027, Int32Array = 1028, UInt32Array = 1029, Int64Array = 1030, UInt64Array = 1031, SingleArray = 1032, DoubleArray = 1033, Char16Array = 1034, BooleanArray = 1035, StringArray = 1036, InspectableArray = 1037, DateTimeArray = 1038, TimeSpanArray = 1039, GuidArray = 1040, PointArray = 1041, SizeArray = 1042, RectArray = 1043, OtherTypeArray = 1044
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Point
		{
			X: f32, Y: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Size
		{
			Width: f32, Height: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Rect
		{
			X: f32, Y: f32, Width: f32, Height: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_DateTime
		{
			UniversalTime: i64
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_TimeSpan
		{
			Duration: i64
		}
		DEFINE_GUID!(IID_Windows_Foundation_IPropertyValue, 1272349405, 30036, 16617, 154, 155, 130, 101, 78, 222, 126, 98);
		RT_INTERFACE!{interface Windows_Foundation_IPropertyValue(Windows_Foundation_IPropertyValueVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IPropertyValue] {
			fn get_Type(&mut self, out: *mut Windows_Foundation_PropertyType) -> ::w::HRESULT,
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
			fn GetDateTime(&mut self, out: *mut Windows_Foundation_DateTime) -> ::w::HRESULT,
			fn GetTimeSpan(&mut self, out: *mut Windows_Foundation_TimeSpan) -> ::w::HRESULT,
			fn GetPoint(&mut self, out: *mut Windows_Foundation_Point) -> ::w::HRESULT,
			fn GetSize(&mut self, out: *mut Windows_Foundation_Size) -> ::w::HRESULT,
			fn GetRect(&mut self, out: *mut Windows_Foundation_Rect) -> ::w::HRESULT,
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
			fn GetDateTimeArray(&mut self, value: *mut *mut Windows_Foundation_DateTime) -> ::w::HRESULT,
			fn GetTimeSpanArray(&mut self, value: *mut *mut Windows_Foundation_TimeSpan) -> ::w::HRESULT,
			fn GetPointArray(&mut self, value: *mut *mut Windows_Foundation_Point) -> ::w::HRESULT,
			fn GetSizeArray(&mut self, value: *mut *mut Windows_Foundation_Size) -> ::w::HRESULT,
			fn GetRectArray(&mut self, value: *mut *mut Windows_Foundation_Rect) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IPropertyValueStatics, 1654381512, 55602, 20468, 150, 185, 141, 150, 197, 193, 232, 88);
		RT_INTERFACE!{interface Windows_Foundation_IPropertyValueStatics(Windows_Foundation_IPropertyValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IPropertyValueStatics] {
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
			fn CreateDateTime(&mut self, value: Windows_Foundation_DateTime, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateTimeSpan(&mut self, value: Windows_Foundation_TimeSpan, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreatePoint(&mut self, value: Windows_Foundation_Point, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSize(&mut self, value: Windows_Foundation_Size, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateRect(&mut self, value: Windows_Foundation_Rect, out: *mut *mut IInspectable) -> ::w::HRESULT,
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
			fn CreateDateTimeArray(&mut self, value: *mut Windows_Foundation_DateTime, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateTimeSpanArray(&mut self, value: *mut Windows_Foundation_TimeSpan, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreatePointArray(&mut self, value: *mut Windows_Foundation_Point, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateSizeArray(&mut self, value: *mut Windows_Foundation_Size, out: *mut *mut IInspectable) -> ::w::HRESULT,
			fn CreateRectArray(&mut self, value: *mut Windows_Foundation_Rect, out: *mut *mut IInspectable) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IStringable, 2520162132, 36534, 18672, 171, 206, 193, 178, 17, 230, 39, 195);
		RT_INTERFACE!{interface Windows_Foundation_IStringable(Windows_Foundation_IStringableVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IStringable] {
			fn ToString(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IPropertySet, 2319707551, 62694, 17441, 172, 249, 29, 171, 41, 134, 130, 12);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IPropertySet(Windows_Foundation_Collections_IPropertySetVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IPropertySet] {
			
		}}
		pub type Windows_Foundation_Collections_PropertySet = Windows_Foundation_Collections_IPropertySet;
		pub type Windows_Foundation_Collections_ValueSet = Windows_Foundation_Collections_IPropertySet;
		pub type Windows_Foundation_Collections_StringMap<'a> = Windows_Foundation_Collections_IMap<&'a str, &'a str>;
		DEFINE_GUID!(IID_Windows_Foundation_AsyncActionCompletedHandler, 2767019137, 30409, 16573, 139, 230, 177, 217, 15, 178, 10, 231);
		RT_INTERFACE!{interface Windows_Foundation_AsyncActionCompletedHandler(Windows_Foundation_AsyncActionCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncActionCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncAction, asyncStatus: Windows_Foundation_AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IDeferral, 3592853298, 15231, 18087, 180, 11, 79, 220, 162, 162, 198, 147);
		RT_INTERFACE!{interface Windows_Foundation_IDeferral(Windows_Foundation_IDeferralVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IDeferral] {
			fn Complete(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_DeferralCompletedHandler, 3979518834, 62408, 20394, 156, 251, 71, 1, 72, 218, 56, 136);
		RT_INTERFACE!{interface Windows_Foundation_DeferralCompletedHandler(Windows_Foundation_DeferralCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_DeferralCompletedHandler] {
			fn Invoke(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IDeferralFactory, 1705110725, 16309, 18482, 140, 169, 240, 97, 178, 129, 209, 58);
		RT_INTERFACE!{interface Windows_Foundation_IDeferralFactory(Windows_Foundation_IDeferralFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IDeferralFactory] {
			fn Create(&mut self, handler: *mut Windows_Foundation_DeferralCompletedHandler, out: *mut *mut Windows_Foundation_Deferral) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Deferral = Windows_Foundation_IDeferral;
		DEFINE_GUID!(IID_Windows_Foundation_Metadata_IApiInformationStatics, 2574531070, 63105, 18961, 180, 22, 193, 58, 71, 232, 186, 54);
		RT_INTERFACE!{interface Windows_Foundation_Metadata_IApiInformationStatics(Windows_Foundation_Metadata_IApiInformationStaticsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Metadata_IApiInformationStatics] {
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
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_Platform
		{
			Windows = 0, WindowsPhone = 1
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_AttributeTargets
		{
			All = 4294967295, Delegate = 1, Enum = 2, Event = 4, Field = 8, Interface = 16, Method = 64, Parameter = 128, Property = 256, RuntimeClass = 512, Struct = 1024, InterfaceImpl = 2048, ApiContract = 8192
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_CompositionType
		{
			Protected = 1, Public = 2
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_ThreadingModel
		{
			STA = 1, MTA = 2, Both = 3, InvalidThreading = 0
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_MarshalingType
		{
			None = 1, Agile = 2, Standard = 3, InvalidMarshaling = 0
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Metadata_DeprecationType
		{
			Deprecate = 0, Remove = 1
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_AsyncStatus
		{
			Canceled = 2, Completed = 1, Error = 3, Started = 0
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_EventRegistrationToken
		{
			Value: i64
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_HResult
		{
			Value: i32
		}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncInfo, 54, 0, 0, 192, 0, 0, 0, 0, 0, 0, 70);
		RT_INTERFACE!{interface Windows_Foundation_IAsyncInfo(Windows_Foundation_IAsyncInfoVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IAsyncInfo] {
			fn get_Id(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn get_Status(&mut self, out: *mut Windows_Foundation_AsyncStatus) -> ::w::HRESULT,
			fn get_ErrorCode(&mut self, out: *mut Windows_Foundation_HResult) -> ::w::HRESULT,
			fn Cancel(&mut self) -> ::w::HRESULT,
			fn Close(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncAction, 1516535814, 33850, 19881, 134, 91, 157, 38, 229, 223, 173, 123);
		RT_INTERFACE!{interface Windows_Foundation_IAsyncAction(Windows_Foundation_IAsyncActionVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IAsyncAction] {
			fn put_Completed(&mut self, handler: *mut Windows_Foundation_AsyncActionCompletedHandler) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut Windows_Foundation_AsyncActionCompletedHandler) -> ::w::HRESULT,
			fn GetResults(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IIterable, 4205151722, 25108, 16919, 175, 218, 127, 70, 222, 88, 105, 179);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IIterable<T>(Windows_Foundation_Collections_IIterableVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IIterable] {
			fn First(&mut self, out: *mut *mut Windows_Foundation_Collections_IIterator<T>) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IIterator, 1786374243, 17152, 17818, 153, 102, 203, 182, 96, 150, 62, 225);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IIterator<T>(Windows_Foundation_Collections_IIteratorVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IIterator] {
			fn get_Current(&mut self, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_HasCurrent(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn MoveNext(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetMany(&mut self, items: *mut T::Abi, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IVectorView, 3152149068, 45283, 17795, 186, 239, 31, 27, 46, 72, 62, 86);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IVectorView<T>(Windows_Foundation_Collections_IVectorViewVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IVectorView] {
			fn GetAt(&mut self, index: u32, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn IndexOf(&mut self, value: T::Abi, index: *mut u32, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetMany(&mut self, startIndex: u32, items: *mut T::Abi, out: *mut u32) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IVector, 2436052969, 4513, 17221, 163, 162, 78, 127, 149, 110, 34, 45);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IVector<T>(Windows_Foundation_Collections_IVectorVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IVector] {
			fn GetAt(&mut self, index: u32, out: *mut T::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn GetView(&mut self, out: *mut *mut Windows_Foundation_Collections_IVectorView<T>) -> ::w::HRESULT,
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
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IKeyValuePair, 45422889, 49604, 19070, 137, 64, 3, 18, 181, 193, 133, 0);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IKeyValuePair<K, V>(Windows_Foundation_Collections_IKeyValuePairVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IKeyValuePair] {
			fn get_Key(&mut self, out: *mut K::Abi) -> ::w::HRESULT,
			fn get_Value(&mut self, out: *mut V::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IMap, 1009329662, 34073, 17857, 170, 121, 25, 123, 103, 24, 193, 193);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IMap<K, V>(Windows_Foundation_Collections_IMapVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IMap] {
			fn Lookup(&mut self, key: K::Abi, out: *mut V::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn HasKey(&mut self, key: K::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn GetView(&mut self, out: *mut *mut Windows_Foundation_Collections_IMapView<K, V>) -> ::w::HRESULT,
			fn Insert(&mut self, key: K::Abi, value: V::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Remove(&mut self, key: K::Abi) -> ::w::HRESULT,
			fn Clear(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IMapView, 3833646656, 41784, 19162, 173, 207, 39, 34, 114, 228, 140, 185);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IMapView<K, V>(Windows_Foundation_Collections_IMapViewVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IMapView] {
			fn Lookup(&mut self, key: K::Abi, out: *mut V::Abi) -> ::w::HRESULT,
			fn get_Size(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn HasKey(&mut self, key: K::Abi, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Split(&mut self, first: *mut *mut Windows_Foundation_Collections_IMapView<K, V>, second: *mut *mut Windows_Foundation_Collections_IMapView<K, V>) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_VectorChangedEventHandler, 201660242, 40895, 19568, 170, 12, 14, 76, 130, 217, 167, 97);
		RT_INTERFACE!{interface Windows_Foundation_Collections_VectorChangedEventHandler<T>(Windows_Foundation_Collections_VectorChangedEventHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_Collections_VectorChangedEventHandler] {
			fn Invoke(&mut self, sender: *mut Windows_Foundation_Collections_IObservableVector<T>, event: *mut Windows_Foundation_Collections_IVectorChangedEventArgs) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IObservableVector, 1494739795, 20660, 18957, 179, 9, 101, 134, 43, 63, 29, 188);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IObservableVector<T>(Windows_Foundation_Collections_IObservableVectorVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IObservableVector] {
			fn add_VectorChanged(&mut self, vhnd: *mut Windows_Foundation_Collections_VectorChangedEventHandler<T>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_VectorChanged(&mut self, token: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IMapChangedEventArgs, 2570712287, 1290, 19471, 170, 96, 119, 7, 95, 156, 71, 119);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IMapChangedEventArgs<K>(Windows_Foundation_Collections_IMapChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IMapChangedEventArgs] {
			fn get_CollectionChange(&mut self, out: *mut Windows_Foundation_Collections_CollectionChange) -> ::w::HRESULT,
			fn get_Key(&mut self, out: *mut K::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_MapChangedEventHandler, 395646963, 38126, 16888, 189, 220, 118, 138, 137, 85, 68, 243);
		RT_INTERFACE!{interface Windows_Foundation_Collections_MapChangedEventHandler<K, V>(Windows_Foundation_Collections_MapChangedEventHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_Collections_MapChangedEventHandler] {
			fn Invoke(&mut self, sender: *mut Windows_Foundation_Collections_IObservableMap<K, V>, event: *mut Windows_Foundation_Collections_IMapChangedEventArgs<K>) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IObservableMap, 1709124597, 48953, 16821, 174, 188, 90, 157, 134, 94, 71, 43);
		RT_INTERFACE!{interface Windows_Foundation_Collections_IObservableMap<K, V>(Windows_Foundation_Collections_IObservableMapVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Collections_IObservableMap] {
			fn add_MapChanged(&mut self, vhnd: *mut Windows_Foundation_Collections_MapChangedEventHandler<K, V>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_MapChanged(&mut self, token: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_AsyncOperationWithProgressCompletedHandler, 3898471453, 27303, 18147, 168, 226, 240, 9, 216, 64, 198, 39);
		RT_INTERFACE!{interface Windows_Foundation_AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(Windows_Foundation_AsyncOperationWithProgressCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncOperationWithProgressCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncOperationWithProgress<TResult, TProgress>, asyncStatus: Windows_Foundation_AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncOperationWithProgress, 3050321623, 58007, 18831, 186, 96, 2, 137, 231, 110, 35, 221);
		RT_INTERFACE!{interface Windows_Foundation_IAsyncOperationWithProgress<TResult, TProgress>(Windows_Foundation_IAsyncOperationWithProgressVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IAsyncOperationWithProgress] {
			fn put_Progress(&mut self, handler: *mut Windows_Foundation_AsyncOperationProgressHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn get_Progress(&mut self, out: *mut *mut Windows_Foundation_AsyncOperationProgressHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn put_Completed(&mut self, handler: *mut Windows_Foundation_AsyncOperationWithProgressCompletedHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut Windows_Foundation_AsyncOperationWithProgressCompletedHandler<TResult, TProgress>) -> ::w::HRESULT,
			fn GetResults(&mut self, out: *mut TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_AsyncOperationCompletedHandler, 4242337836, 58840, 17528, 145, 90, 77, 144, 183, 75, 131, 165);
		RT_INTERFACE!{interface Windows_Foundation_AsyncOperationCompletedHandler<TResult>(Windows_Foundation_AsyncOperationCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncOperationCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncOperation<TResult>, asyncStatus: Windows_Foundation_AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncOperation, 2680336571, 58438, 17634, 170, 97, 156, 171, 143, 99, 106, 242);
		RT_INTERFACE!{interface Windows_Foundation_IAsyncOperation<TResult>(Windows_Foundation_IAsyncOperationVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IAsyncOperation] {
			fn put_Completed(&mut self, handler: *mut Windows_Foundation_AsyncOperationCompletedHandler<TResult>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut Windows_Foundation_AsyncOperationCompletedHandler<TResult>) -> ::w::HRESULT,
			fn GetResults(&mut self, out: *mut TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_AsyncActionWithProgressCompletedHandler, 2617417617, 52356, 17661, 172, 38, 10, 108, 78, 85, 82, 129);
		RT_INTERFACE!{interface Windows_Foundation_AsyncActionWithProgressCompletedHandler<TProgress>(Windows_Foundation_AsyncActionWithProgressCompletedHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncActionWithProgressCompletedHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncActionWithProgress<TProgress>, asyncStatus: Windows_Foundation_AsyncStatus) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncActionWithProgress, 527282776, 59395, 18593, 149, 70, 235, 115, 83, 57, 136, 132);
		RT_INTERFACE!{interface Windows_Foundation_IAsyncActionWithProgress<TProgress>(Windows_Foundation_IAsyncActionWithProgressVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IAsyncActionWithProgress] {
			fn put_Progress(&mut self, handler: *mut Windows_Foundation_AsyncActionProgressHandler<TProgress>) -> ::w::HRESULT,
			fn get_Progress(&mut self, out: *mut *mut Windows_Foundation_AsyncActionProgressHandler<TProgress>) -> ::w::HRESULT,
			fn put_Completed(&mut self, handler: *mut Windows_Foundation_AsyncActionWithProgressCompletedHandler<TProgress>) -> ::w::HRESULT,
			fn get_Completed(&mut self, out: *mut *mut Windows_Foundation_AsyncActionWithProgressCompletedHandler<TProgress>) -> ::w::HRESULT,
			fn GetResults(&mut self) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_AsyncOperationProgressHandler, 1432946946, 2731, 16922, 135, 120, 248, 206, 80, 38, 215, 88);
		RT_INTERFACE!{interface Windows_Foundation_AsyncOperationProgressHandler<TResult, TProgress>(Windows_Foundation_AsyncOperationProgressHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncOperationProgressHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncOperationWithProgress<TResult, TProgress>, progressInfo: TProgress::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_AsyncActionProgressHandler, 1837385816, 3327, 17808, 174, 137, 149, 165, 165, 200, 180, 184);
		RT_INTERFACE!{interface Windows_Foundation_AsyncActionProgressHandler<TProgress>(Windows_Foundation_AsyncActionProgressHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_AsyncActionProgressHandler] {
			fn Invoke(&mut self, asyncInfo: *mut Windows_Foundation_IAsyncActionWithProgress<TProgress>, progressInfo: TProgress::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IReference, 1640068870, 11621, 4576, 154, 232, 212, 133, 100, 1, 84, 114);
		RT_INTERFACE!{interface Windows_Foundation_IReference<T>(Windows_Foundation_IReferenceVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IReference] {
			fn get_Value(&mut self, out: *mut T::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IReferenceArray, 1640068871, 11621, 4576, 154, 232, 212, 133, 100, 1, 84, 114);
		RT_INTERFACE!{interface Windows_Foundation_IReferenceArray<T>(Windows_Foundation_IReferenceArrayVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IReferenceArray] {
			fn get_Value(&mut self, out: *mut *mut T::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_TypedEventHandler, 2648818996, 27361, 4576, 132, 225, 24, 169, 5, 188, 197, 63);
		RT_INTERFACE!{interface Windows_Foundation_TypedEventHandler<TSender, TResult>(Windows_Foundation_TypedEventHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_TypedEventHandler] {
			fn Invoke(&mut self, sender: TSender::Abi, args: TResult::Abi) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_EventHandler, 2648818997, 27361, 4576, 132, 225, 24, 169, 5, 188, 197, 63);
		RT_INTERFACE!{interface Windows_Foundation_EventHandler<T>(Windows_Foundation_EventHandlerVtbl): IUnknown(IUnknownVtbl) [IID_Windows_Foundation_EventHandler] {
			fn Invoke(&mut self, sender: *mut IInspectable, args: T::Abi) -> ::w::HRESULT
		}}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_FoundationContract
		{
			
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_CausalityTraceLevel
		{
			Required = 0, Important = 1, Verbose = 2
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_CausalitySource
		{
			Application = 0, Library = 1, System = 2
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_CausalityRelation
		{
			AssignDelegate = 0, Join = 1, Choice = 2, Cancel = 3, Error = 4
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_CausalitySynchronousWork
		{
			CompletionNotification = 0, ProgressNotification = 1, Execution = 2
		}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs, 1091270417, 65339, 18303, 156, 154, 210, 239, 218, 48, 45, 195);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs(Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs] {
			fn get_Enabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn get_TraceLevel(&mut self, out: *mut Windows_Foundation_Diagnostics_CausalityTraceLevel) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IAsyncCausalityTracerStatics, 1350896422, 9854, 17691, 168, 144, 171, 106, 55, 2, 69, 238);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IAsyncCausalityTracerStatics(Windows_Foundation_Diagnostics_IAsyncCausalityTracerStaticsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IAsyncCausalityTracerStatics] {
			fn TraceOperationCreation(&mut self, traceLevel: Windows_Foundation_Diagnostics_CausalityTraceLevel, source: Windows_Foundation_Diagnostics_CausalitySource, platformId: ::w::GUID, operationId: u64, operationName: ::w::HSTRING, relatedContext: u64) -> ::w::HRESULT,
			fn TraceOperationCompletion(&mut self, traceLevel: Windows_Foundation_Diagnostics_CausalityTraceLevel, source: Windows_Foundation_Diagnostics_CausalitySource, platformId: ::w::GUID, operationId: u64, status: Windows_Foundation_AsyncStatus) -> ::w::HRESULT,
			fn TraceOperationRelation(&mut self, traceLevel: Windows_Foundation_Diagnostics_CausalityTraceLevel, source: Windows_Foundation_Diagnostics_CausalitySource, platformId: ::w::GUID, operationId: u64, relation: Windows_Foundation_Diagnostics_CausalityRelation) -> ::w::HRESULT,
			fn TraceSynchronousWorkStart(&mut self, traceLevel: Windows_Foundation_Diagnostics_CausalityTraceLevel, source: Windows_Foundation_Diagnostics_CausalitySource, platformId: ::w::GUID, operationId: u64, work: Windows_Foundation_Diagnostics_CausalitySynchronousWork) -> ::w::HRESULT,
			fn TraceSynchronousWorkCompletion(&mut self, traceLevel: Windows_Foundation_Diagnostics_CausalityTraceLevel, source: Windows_Foundation_Diagnostics_CausalitySource, work: Windows_Foundation_Diagnostics_CausalitySynchronousWork) -> ::w::HRESULT,
			fn add_TracingStatusChanged(&mut self, handler: *mut Windows_Foundation_EventHandler<&Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_TracingStatusChanged(&mut self, cookie: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs = Windows_Foundation_Diagnostics_ITracingStatusChangedEventArgs;
		DEFINE_GUID!(IID_Windows_Foundation_IUriRuntimeClass, 2654363223, 18610, 16736, 149, 111, 199, 56, 81, 32, 187, 252);
		RT_INTERFACE!{interface Windows_Foundation_IUriRuntimeClass(Windows_Foundation_IUriRuntimeClassVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IUriRuntimeClass] {
			fn get_AbsoluteUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_DisplayUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Domain(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Extension(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Fragment(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Host(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Password(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Path(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Query(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_QueryParsed(&mut self, out: *mut *mut Windows_Foundation_WwwFormUrlDecoder) -> ::w::HRESULT,
			fn get_RawUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_SchemeName(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_UserName(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Port(&mut self, out: *mut i32) -> ::w::HRESULT,
			fn get_Suspicious(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn Equals(&mut self, pUri: *mut Windows_Foundation_Uri, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn CombineUri(&mut self, relativeUri: ::w::HSTRING, out: *mut *mut Windows_Foundation_Uri) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_WwwFormUrlDecoder = Windows_Foundation_IWwwFormUrlDecoderRuntimeClass;
		pub type Windows_Foundation_Uri = Windows_Foundation_IUriRuntimeClass;
		DEFINE_GUID!(IID_Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUri, 1972213345, 8732, 18447, 163, 57, 80, 101, 102, 115, 244, 111);
		RT_INTERFACE!{interface Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUri(Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUriVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IUriRuntimeClassWithAbsoluteCanonicalUri] {
			fn get_AbsoluteCanonicalUri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_DisplayIri(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IUriEscapeStatics, 3251909306, 51236, 17490, 167, 253, 81, 43, 195, 187, 233, 161);
		RT_INTERFACE!{interface Windows_Foundation_IUriEscapeStatics(Windows_Foundation_IUriEscapeStaticsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IUriEscapeStatics] {
			fn UnescapeComponent(&mut self, toUnescape: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn EscapeComponent(&mut self, toEscape: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IUriRuntimeClassFactory, 1151957359, 29246, 20447, 162, 24, 3, 62, 117, 176, 192, 132);
		RT_INTERFACE!{interface Windows_Foundation_IUriRuntimeClassFactory(Windows_Foundation_IUriRuntimeClassFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IUriRuntimeClassFactory] {
			fn CreateUri(&mut self, uri: ::w::HSTRING, out: *mut *mut Windows_Foundation_Uri) -> ::w::HRESULT,
			fn CreateWithRelativeUri(&mut self, baseUri: ::w::HSTRING, relativeUri: ::w::HSTRING, out: *mut *mut Windows_Foundation_Uri) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IWwwFormUrlDecoderEntry, 308180017, 63096, 20110, 182, 112, 32, 169, 176, 108, 81, 45);
		RT_INTERFACE!{interface Windows_Foundation_IWwwFormUrlDecoderEntry(Windows_Foundation_IWwwFormUrlDecoderEntryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IWwwFormUrlDecoderEntry] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Value(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IWwwFormUrlDecoderRuntimeClass, 3562669137, 61989, 17730, 146, 150, 14, 29, 245, 210, 84, 223);
		RT_INTERFACE!{interface Windows_Foundation_IWwwFormUrlDecoderRuntimeClass(Windows_Foundation_IWwwFormUrlDecoderRuntimeClassVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IWwwFormUrlDecoderRuntimeClass] {
			fn GetFirstValueByName(&mut self, name: ::w::HSTRING, out: *mut ::w::HSTRING) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactory, 1535929149, 9390, 16821, 161, 191, 240, 195, 213, 68, 132, 91);
		RT_INTERFACE!{interface Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactory(Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IWwwFormUrlDecoderRuntimeClassFactory] {
			fn CreateWwwFormUrlDecoder(&mut self, query: ::w::HSTRING, out: *mut *mut Windows_Foundation_WwwFormUrlDecoder) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_WwwFormUrlDecoderEntry = Windows_Foundation_IWwwFormUrlDecoderEntry;
		DEFINE_GUID!(IID_Windows_Foundation_IGetActivationFactory, 1323011810, 38621, 18855, 148, 247, 70, 7, 221, 171, 142, 60);
		RT_INTERFACE!{interface Windows_Foundation_IGetActivationFactory(Windows_Foundation_IGetActivationFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IGetActivationFactory] {
			fn GetActivationFactory(&mut self, activatableClassId: ::w::HSTRING, out: *mut *mut IInspectable) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IMemoryBufferReference, 4223982889, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface Windows_Foundation_IMemoryBufferReference(Windows_Foundation_IMemoryBufferReferenceVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IMemoryBufferReference] {
			fn get_Capacity(&mut self, out: *mut u32) -> ::w::HRESULT,
			fn add_Closed(&mut self, handler: *mut Windows_Foundation_TypedEventHandler<&Windows_Foundation_IMemoryBufferReference, &IInspectable>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_Closed(&mut self, cookie: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IMemoryBuffer, 4223982890, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface Windows_Foundation_IMemoryBuffer(Windows_Foundation_IMemoryBufferVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IMemoryBuffer] {
			fn CreateReference(&mut self, out: *mut *mut Windows_Foundation_IMemoryBufferReference) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_IMemoryBufferFactory, 4223982891, 9307, 4580, 175, 152, 104, 148, 35, 38, 12, 248);
		RT_INTERFACE!{interface Windows_Foundation_IMemoryBufferFactory(Windows_Foundation_IMemoryBufferFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_IMemoryBufferFactory] {
			fn Create(&mut self, capacity: u32, out: *mut *mut Windows_Foundation_MemoryBuffer) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_MemoryBuffer = Windows_Foundation_IMemoryBuffer;
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_ErrorOptions
		{
			None = 0, SuppressExceptions = 1, ForceExceptions = 2, UseSetErrorInfo = 4, SuppressSetErrorInfo = 8
		}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IErrorReportingSettings, 372676498, 45118, 19361, 139, 184, 210, 143, 74, 180, 210, 192);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IErrorReportingSettings(Windows_Foundation_Diagnostics_IErrorReportingSettingsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IErrorReportingSettings] {
			fn SetErrorOptions(&mut self, value: Windows_Foundation_Diagnostics_ErrorOptions) -> ::w::HRESULT,
			fn GetErrorOptions(&mut self, out: *mut Windows_Foundation_Diagnostics_ErrorOptions) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_RuntimeBrokerErrorSettings = Windows_Foundation_Diagnostics_IErrorReportingSettings;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IErrorDetailsStatics, 3077584720, 2845, 18120, 170, 14, 75, 129, 120, 228, 252, 233);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IErrorDetailsStatics(Windows_Foundation_Diagnostics_IErrorDetailsStaticsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IErrorDetailsStatics] {
			fn CreateFromHResultAsync(&mut self, errorCode: i32, out: *mut *mut Windows_Foundation_IAsyncOperation<&Windows_Foundation_Diagnostics_ErrorDetails>) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_ErrorDetails = Windows_Foundation_Diagnostics_IErrorDetails;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IErrorDetails, 931969793, 11465, 17039, 140, 85, 44, 153, 13, 70, 62, 143);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IErrorDetails(Windows_Foundation_Diagnostics_IErrorDetailsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IErrorDetails] {
			fn get_Description(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_LongDescription(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_HelpUri(&mut self, out: *mut *mut Windows_Foundation_Uri) -> ::w::HRESULT
		}}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Vector2
		{
			X: f32, Y: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Vector3
		{
			X: f32, Y: f32, Z: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Vector4
		{
			X: f32, Y: f32, Z: f32, W: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Matrix3x2
		{
			M11: f32, M12: f32, M21: f32, M22: f32, M31: f32, M32: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Matrix4x4
		{
			M11: f32, M12: f32, M13: f32, M14: f32, M21: f32, M22: f32, M23: f32, M24: f32, M31: f32, M32: f32, M33: f32, M34: f32, M41: f32, M42: f32, M43: f32, M44: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Plane
		{
			Normal: Windows_Foundation_Numerics_Vector3, D: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_Numerics_Quaternion
		{
			X: f32, Y: f32, Z: f32, W: f32
		}
		#[repr(C)]
		#[derive(Debug,PartialEq)]
		pub struct Windows_Foundation_UniversalApiContract
		{
			
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_LoggingLevel
		{
			Verbose = 0, Information = 1, Warning = 2, Error = 3, Critical = 4
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_LoggingOpcode
		{
			Info = 0, Start = 1, Stop = 2, Reply = 6, Resume = 7, Suspend = 8, Send = 9
		}
		#[repr(C)]
		#[derive(Debug,PartialEq,Eq)]
		pub enum Windows_Foundation_Diagnostics_LoggingFieldFormat
		{
			Default = 0, Hidden = 1, String = 2, Boolean = 3, Hexadecimal = 4, ProcessId = 5, ThreadId = 6, Port = 7, Ipv4Address = 8, Ipv6Address = 9, SocketAddress = 10, Xml = 11, Json = 12, Win32Error = 13, NTStatus = 14, HResult = 15, FileTime = 16, Signed = 17, Unsigned = 18
		}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingOptions, 2428270672, 402, 20317, 172, 38, 0, 106, 218, 202, 18, 216);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingOptions(Windows_Foundation_Diagnostics_ILoggingOptionsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingOptions] {
			fn get_Keywords(&mut self, out: *mut i64) -> ::w::HRESULT,
			fn put_Keywords(&mut self, value: i64) -> ::w::HRESULT,
			fn get_Tags(&mut self, out: *mut i32) -> ::w::HRESULT,
			fn put_Tags(&mut self, value: i32) -> ::w::HRESULT,
			fn get_Task(&mut self, out: *mut i16) -> ::w::HRESULT,
			fn put_Task(&mut self, value: i16) -> ::w::HRESULT,
			fn get_Opcode(&mut self, out: *mut Windows_Foundation_Diagnostics_LoggingOpcode) -> ::w::HRESULT,
			fn put_Opcode(&mut self, value: Windows_Foundation_Diagnostics_LoggingOpcode) -> ::w::HRESULT,
			fn get_ActivityId(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_ActivityId(&mut self, value: ::w::GUID) -> ::w::HRESULT,
			fn get_RelatedActivityId(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_RelatedActivityId(&mut self, value: ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingOptionsFactory, 3608397515, 39083, 17995, 159, 34, 163, 38, 132, 120, 54, 138);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingOptionsFactory(Windows_Foundation_Diagnostics_ILoggingOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingOptionsFactory] {
			fn CreateWithKeywords(&mut self, keywords: i64, out: *mut *mut Windows_Foundation_Diagnostics_LoggingOptions) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingOptions = Windows_Foundation_Diagnostics_ILoggingOptions;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannelOptions, 3286779903, 3771, 19027, 140, 84, 222, 194, 73, 38, 203, 44);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannelOptions(Windows_Foundation_Diagnostics_ILoggingChannelOptionsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannelOptions] {
			fn get_Group(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT,
			fn put_Group(&mut self, value: ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactory, 2838581722, 32687, 16785, 135, 85, 94, 134, 220, 101, 216, 150);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactory(Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannelOptionsFactory] {
			fn Create(&mut self, group: ::w::GUID, out: *mut *mut Windows_Foundation_Diagnostics_LoggingChannelOptions) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingChannelOptions = Windows_Foundation_Diagnostics_ILoggingChannelOptions;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingFields, 3623270319, 30253, 17785, 131, 189, 82, 194, 59, 195, 51, 188);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingFields(Windows_Foundation_Diagnostics_ILoggingFieldsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingFields] {
			fn Clear(&mut self) -> ::w::HRESULT,
			fn BeginStruct(&mut self, name: ::w::HSTRING) -> ::w::HRESULT,
			fn BeginStructWithTags(&mut self, name: ::w::HSTRING, tags: i32) -> ::w::HRESULT,
			fn EndStruct(&mut self) -> ::w::HRESULT,
			fn AddEmpty(&mut self, name: ::w::HSTRING) -> ::w::HRESULT,
			fn AddEmptyWithFormat(&mut self, name: ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddEmptyWithFormatAndTags(&mut self, name: ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt8(&mut self, name: ::w::HSTRING, value: u8) -> ::w::HRESULT,
			fn AddUInt8WithFormat(&mut self, name: ::w::HSTRING, value: u8, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt8WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u8, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt8Array(&mut self, name: ::w::HSTRING, value: *mut u8) -> ::w::HRESULT,
			fn AddUInt8ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u8, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt8ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u8, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt16(&mut self, name: ::w::HSTRING, value: i16) -> ::w::HRESULT,
			fn AddInt16WithFormat(&mut self, name: ::w::HSTRING, value: i16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt16Array(&mut self, name: ::w::HSTRING, value: *mut i16) -> ::w::HRESULT,
			fn AddInt16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt16(&mut self, name: ::w::HSTRING, value: u16) -> ::w::HRESULT,
			fn AddUInt16WithFormat(&mut self, name: ::w::HSTRING, value: u16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt16Array(&mut self, name: ::w::HSTRING, value: *mut u16) -> ::w::HRESULT,
			fn AddUInt16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u16, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt32(&mut self, name: ::w::HSTRING, value: i32) -> ::w::HRESULT,
			fn AddInt32WithFormat(&mut self, name: ::w::HSTRING, value: i32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt32WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt32Array(&mut self, name: ::w::HSTRING, value: *mut i32) -> ::w::HRESULT,
			fn AddInt32ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt32ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt32(&mut self, name: ::w::HSTRING, value: u32) -> ::w::HRESULT,
			fn AddUInt32WithFormat(&mut self, name: ::w::HSTRING, value: u32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt32WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt32Array(&mut self, name: ::w::HSTRING, value: *mut u32) -> ::w::HRESULT,
			fn AddUInt32ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt32ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt64(&mut self, name: ::w::HSTRING, value: i64) -> ::w::HRESULT,
			fn AddInt64WithFormat(&mut self, name: ::w::HSTRING, value: i64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt64WithFormatAndTags(&mut self, name: ::w::HSTRING, value: i64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddInt64Array(&mut self, name: ::w::HSTRING, value: *mut i64) -> ::w::HRESULT,
			fn AddInt64ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut i64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddInt64ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut i64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt64(&mut self, name: ::w::HSTRING, value: u64) -> ::w::HRESULT,
			fn AddUInt64WithFormat(&mut self, name: ::w::HSTRING, value: u64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt64WithFormatAndTags(&mut self, name: ::w::HSTRING, value: u64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddUInt64Array(&mut self, name: ::w::HSTRING, value: *mut u64) -> ::w::HRESULT,
			fn AddUInt64ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut u64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddUInt64ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut u64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSingle(&mut self, name: ::w::HSTRING, value: f32) -> ::w::HRESULT,
			fn AddSingleWithFormat(&mut self, name: ::w::HSTRING, value: f32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSingleWithFormatAndTags(&mut self, name: ::w::HSTRING, value: f32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSingleArray(&mut self, name: ::w::HSTRING, value: *mut f32) -> ::w::HRESULT,
			fn AddSingleArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut f32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSingleArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut f32, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDouble(&mut self, name: ::w::HSTRING, value: f64) -> ::w::HRESULT,
			fn AddDoubleWithFormat(&mut self, name: ::w::HSTRING, value: f64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDoubleWithFormatAndTags(&mut self, name: ::w::HSTRING, value: f64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDoubleArray(&mut self, name: ::w::HSTRING, value: *mut f64) -> ::w::HRESULT,
			fn AddDoubleArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut f64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDoubleArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut f64, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddChar16(&mut self, name: ::w::HSTRING, value: ::w::wchar_t) -> ::w::HRESULT,
			fn AddChar16WithFormat(&mut self, name: ::w::HSTRING, value: ::w::wchar_t, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddChar16WithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::wchar_t, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddChar16Array(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t) -> ::w::HRESULT,
			fn AddChar16ArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddChar16ArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::wchar_t, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddBoolean(&mut self, name: ::w::HSTRING, value: ::w::BOOL) -> ::w::HRESULT,
			fn AddBooleanWithFormat(&mut self, name: ::w::HSTRING, value: ::w::BOOL, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddBooleanWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::BOOL, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddBooleanArray(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL) -> ::w::HRESULT,
			fn AddBooleanArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddBooleanArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::BOOL, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddString(&mut self, name: ::w::HSTRING, value: ::w::HSTRING) -> ::w::HRESULT,
			fn AddStringWithFormat(&mut self, name: ::w::HSTRING, value: ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddStringWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddStringArray(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn AddStringArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddStringArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::HSTRING, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddGuid(&mut self, name: ::w::HSTRING, value: ::w::GUID) -> ::w::HRESULT,
			fn AddGuidWithFormat(&mut self, name: ::w::HSTRING, value: ::w::GUID, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddGuidWithFormatAndTags(&mut self, name: ::w::HSTRING, value: ::w::GUID, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddGuidArray(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID) -> ::w::HRESULT,
			fn AddGuidArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddGuidArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut ::w::GUID, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDateTime(&mut self, name: ::w::HSTRING, value: Windows_Foundation_DateTime) -> ::w::HRESULT,
			fn AddDateTimeWithFormat(&mut self, name: ::w::HSTRING, value: Windows_Foundation_DateTime, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDateTimeWithFormatAndTags(&mut self, name: ::w::HSTRING, value: Windows_Foundation_DateTime, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddDateTimeArray(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_DateTime) -> ::w::HRESULT,
			fn AddDateTimeArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_DateTime, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddDateTimeArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_DateTime, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddTimeSpan(&mut self, name: ::w::HSTRING, value: Windows_Foundation_TimeSpan) -> ::w::HRESULT,
			fn AddTimeSpanWithFormat(&mut self, name: ::w::HSTRING, value: Windows_Foundation_TimeSpan, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddTimeSpanWithFormatAndTags(&mut self, name: ::w::HSTRING, value: Windows_Foundation_TimeSpan, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddTimeSpanArray(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_TimeSpan) -> ::w::HRESULT,
			fn AddTimeSpanArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_TimeSpan, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddTimeSpanArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_TimeSpan, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddPoint(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Point) -> ::w::HRESULT,
			fn AddPointWithFormat(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Point, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddPointWithFormatAndTags(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Point, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddPointArray(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Point) -> ::w::HRESULT,
			fn AddPointArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Point, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddPointArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Point, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSize(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Size) -> ::w::HRESULT,
			fn AddSizeWithFormat(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Size, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSizeWithFormatAndTags(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Size, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddSizeArray(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Size) -> ::w::HRESULT,
			fn AddSizeArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Size, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddSizeArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Size, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddRect(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Rect) -> ::w::HRESULT,
			fn AddRectWithFormat(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Rect, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddRectWithFormatAndTags(&mut self, name: ::w::HSTRING, value: Windows_Foundation_Rect, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT,
			fn AddRectArray(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Rect) -> ::w::HRESULT,
			fn AddRectArrayWithFormat(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Rect, format: Windows_Foundation_Diagnostics_LoggingFieldFormat) -> ::w::HRESULT,
			fn AddRectArrayWithFormatAndTags(&mut self, name: ::w::HSTRING, value: *mut Windows_Foundation_Rect, format: Windows_Foundation_Diagnostics_LoggingFieldFormat, tags: i32) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingFields = Windows_Foundation_Diagnostics_ILoggingFields;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingTarget, 1710320693, 58248, 20006, 177, 122, 245, 28, 211, 168, 57, 22);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingTarget(Windows_Foundation_Diagnostics_ILoggingTargetVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingTarget] {
			fn IsEnabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEnabledWithLevel(&mut self, level: Windows_Foundation_Diagnostics_LoggingLevel, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn IsEnabledWithLevelAndKeywords(&mut self, level: Windows_Foundation_Diagnostics_LoggingLevel, keywords: i64, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn LogEvent(&mut self, eventName: ::w::HSTRING) -> ::w::HRESULT,
			fn LogEventWithFields(&mut self, eventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields) -> ::w::HRESULT,
			fn LogEventWithFieldsAndLevel(&mut self, eventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, level: Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn LogEventWithFieldsAndOptions(&mut self, eventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, level: Windows_Foundation_Diagnostics_LoggingLevel, options: *mut Windows_Foundation_Diagnostics_LoggingOptions) -> ::w::HRESULT,
			fn StartActivity(&mut self, startEventName: ::w::HSTRING, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFields(&mut self, startEventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFieldsAndLevel(&mut self, startEventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, level: Windows_Foundation_Diagnostics_LoggingLevel, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT,
			fn StartActivityWithFieldsAndOptions(&mut self, startEventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, level: Windows_Foundation_Diagnostics_LoggingLevel, options: *mut Windows_Foundation_Diagnostics_LoggingOptions, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingActivity = Windows_Foundation_Diagnostics_ILoggingActivity;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannel, 3919905603, 4567, 20225, 181, 202, 207, 73, 82, 120, 192, 168);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannel(Windows_Foundation_Diagnostics_ILoggingChannelVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannel] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Enabled(&mut self, out: *mut ::w::BOOL) -> ::w::HRESULT,
			fn get_Level(&mut self, out: *mut Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn LogMessage(&mut self, eventString: ::w::HSTRING) -> ::w::HRESULT,
			fn LogMessageWithLevel(&mut self, eventString: ::w::HSTRING, level: Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn LogValuePair(&mut self, value1: ::w::HSTRING, value2: i32) -> ::w::HRESULT,
			fn LogValuePairWithLevel(&mut self, value1: ::w::HSTRING, value2: i32, level: Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn add_LoggingEnabled(&mut self, handler: *mut Windows_Foundation_TypedEventHandler<&Windows_Foundation_Diagnostics_ILoggingChannel, &IInspectable>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_LoggingEnabled(&mut self, token: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannel2, 2672573683, 2988, 17829, 158, 51, 186, 243, 243, 162, 70, 165);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannel2(Windows_Foundation_Diagnostics_ILoggingChannel2Vtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannel2] {
			fn get_Id(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannelFactory, 1323064220, 44928, 19099, 176, 220, 57, 143, 154, 229, 32, 123);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannelFactory(Windows_Foundation_Diagnostics_ILoggingChannelFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannelFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut Windows_Foundation_Diagnostics_LoggingChannel) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingChannel = Windows_Foundation_Diagnostics_ILoggingChannel;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingChannelFactory2, 1282340317, 15143, 19913, 153, 240, 41, 156, 110, 70, 3, 161);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingChannelFactory2(Windows_Foundation_Diagnostics_ILoggingChannelFactory2Vtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingChannelFactory2] {
			fn CreateWithOptions(&mut self, name: ::w::HSTRING, options: *mut Windows_Foundation_Diagnostics_LoggingChannelOptions, out: *mut *mut Windows_Foundation_Diagnostics_LoggingChannel) -> ::w::HRESULT,
			fn CreateWithOptionsAndId(&mut self, name: ::w::HSTRING, options: *mut Windows_Foundation_Diagnostics_LoggingChannelOptions, id: ::w::GUID, out: *mut *mut Windows_Foundation_Diagnostics_LoggingChannel) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingActivity, 3154323777, 46950, 19637, 152, 72, 151, 172, 107, 166, 214, 12);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingActivity(Windows_Foundation_Diagnostics_ILoggingActivityVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingActivity] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn get_Id(&mut self, out: *mut ::w::GUID) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingActivity2, 650287112, 25378, 17770, 175, 130, 128, 200, 100, 47, 23, 139);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingActivity2(Windows_Foundation_Diagnostics_ILoggingActivity2Vtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingActivity2] {
			fn get_Channel(&mut self, out: *mut *mut Windows_Foundation_Diagnostics_LoggingChannel) -> ::w::HRESULT,
			fn StopActivity(&mut self, stopEventName: ::w::HSTRING) -> ::w::HRESULT,
			fn StopActivityWithFields(&mut self, stopEventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields) -> ::w::HRESULT,
			fn StopActivityWithFieldsAndOptions(&mut self, stopEventName: ::w::HSTRING, fields: *mut Windows_Foundation_Diagnostics_LoggingFields, options: *mut Windows_Foundation_Diagnostics_LoggingOptions) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingActivityFactory, 1798550659, 57610, 19544, 151, 213, 16, 251, 69, 16, 116, 251);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingActivityFactory(Windows_Foundation_Diagnostics_ILoggingActivityFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingActivityFactory] {
			fn CreateLoggingActivity(&mut self, activityName: ::w::HSTRING, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT,
			fn CreateLoggingActivityWithLevel(&mut self, activityName: ::w::HSTRING, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel, level: Windows_Foundation_Diagnostics_LoggingLevel, out: *mut *mut Windows_Foundation_Diagnostics_LoggingActivity) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingSession, 1646392070, 37760, 19159, 186, 245, 65, 234, 147, 16, 215, 104);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingSession(Windows_Foundation_Diagnostics_ILoggingSessionVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingSession] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn SaveToFileAsync(&mut self, folder: *mut Windows_Storage_IStorageFolder, fileName: ::w::HSTRING, out: *mut *mut Windows_Foundation_IAsyncOperation<&Windows_Storage_StorageFile>) -> ::w::HRESULT,
			fn AddLoggingChannel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel) -> ::w::HRESULT,
			fn AddLoggingChannelWithLevel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel, maxLevel: Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn RemoveLoggingChannel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILoggingSessionFactory, 1318289125, 22781, 17888, 140, 47, 161, 50, 239, 249, 92, 30);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILoggingSessionFactory(Windows_Foundation_Diagnostics_ILoggingSessionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILoggingSessionFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut Windows_Foundation_Diagnostics_LoggingSession) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LoggingSession = Windows_Foundation_Diagnostics_ILoggingSession;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs, 647927663, 3384, 19482, 181, 63, 179, 149, 216, 129, 223, 132);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs(Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgsVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs] {
			fn get_File(&mut self, out: *mut *mut Windows_Storage_StorageFile) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs = Windows_Foundation_Diagnostics_ILogFileGeneratedEventArgs;
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IFileLoggingSession, 617038358, 65234, 16460, 137, 95, 31, 150, 153, 203, 2, 247);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IFileLoggingSession(Windows_Foundation_Diagnostics_IFileLoggingSessionVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IFileLoggingSession] {
			fn get_Name(&mut self, out: *mut ::w::HSTRING) -> ::w::HRESULT,
			fn AddLoggingChannel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel) -> ::w::HRESULT,
			fn AddLoggingChannelWithLevel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel, maxLevel: Windows_Foundation_Diagnostics_LoggingLevel) -> ::w::HRESULT,
			fn RemoveLoggingChannel(&mut self, loggingChannel: *mut Windows_Foundation_Diagnostics_ILoggingChannel) -> ::w::HRESULT,
			fn CloseAndSaveToFileAsync(&mut self, out: *mut *mut Windows_Foundation_IAsyncOperation<&Windows_Storage_StorageFile>) -> ::w::HRESULT,
			fn add_LogFileGenerated(&mut self, handler: *mut Windows_Foundation_TypedEventHandler<&Windows_Foundation_Diagnostics_IFileLoggingSession, &Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs>, out: *mut Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT,
			fn remove_LogFileGenerated(&mut self, token: Windows_Foundation_EventRegistrationToken) -> ::w::HRESULT
		}}
		DEFINE_GUID!(IID_Windows_Foundation_Diagnostics_IFileLoggingSessionFactory, 4003499470, 33863, 19882, 145, 51, 18, 235, 70, 246, 151, 212);
		RT_INTERFACE!{interface Windows_Foundation_Diagnostics_IFileLoggingSessionFactory(Windows_Foundation_Diagnostics_IFileLoggingSessionFactoryVtbl): IInspectable(IInspectableVtbl) [IID_Windows_Foundation_Diagnostics_IFileLoggingSessionFactory] {
			fn Create(&mut self, name: ::w::HSTRING, out: *mut *mut Windows_Foundation_Diagnostics_FileLoggingSession) -> ::w::HRESULT
		}}
		pub type Windows_Foundation_Diagnostics_FileLoggingSession = Windows_Foundation_Diagnostics_IFileLoggingSession;
		DEFINE_GUID!(IID_Windows_Foundation_Collections_IMap_2_System_String_System_String, 0xf6d1f700,0x49c2,0x52ae,0x81,0x54,0x82,0x6f,0x99,0x08,0x77,0x3c);
		impl<'a> ComIid for Windows_Foundation_Collections_IMap<&'a str, &'a str> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_Collections_IMap_2_System_String_System_String }
		}
		DEFINE_GUID!(IID_Windows_Foundation_EventHandler_1_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs, 0x2bf27008,0x2eb4,0x5675,0xb1,0xcd,0xe9,0x90,0x6c,0xc5,0xce,0x64);
		impl<'a> ComIid for Windows_Foundation_EventHandler<&'a Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_EventHandler_1_Windows_Foundation_Diagnostics_TracingStatusChangedEventArgs }
		}
		DEFINE_GUID!(IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_IMemoryBufferReference_System_Object, 0xf4637d4a,0x0760,0x5431,0xbf,0xc0,0x24,0xeb,0x1d,0x4f,0x6c,0x4f);
		impl<'a> ComIid for Windows_Foundation_TypedEventHandler<&'a Windows_Foundation_IMemoryBufferReference, &'a IInspectable> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_IMemoryBufferReference_System_Object }
		}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncOperation_1_Windows_Foundation_Diagnostics_ErrorDetails, 0x9b05106d,0x77e0,0x5c24,0x82,0xb0,0x9b,0x2d,0xc8,0xf7,0x96,0x71);
		impl<'a> ComIid for Windows_Foundation_IAsyncOperation<&'a Windows_Foundation_Diagnostics_ErrorDetails> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_IAsyncOperation_1_Windows_Foundation_Diagnostics_ErrorDetails }
		}
		DEFINE_GUID!(IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_Diagnostics_ILoggingChannel_System_Object, 0x52c9c2a1,0x54a3,0x5ef9,0x9a,0xff,0x01,0x4e,0x7c,0x45,0x46,0x55);
		impl<'a> ComIid for Windows_Foundation_TypedEventHandler<&'a Windows_Foundation_Diagnostics_ILoggingChannel, &'a IInspectable> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_Diagnostics_ILoggingChannel_System_Object }
		}
		DEFINE_GUID!(IID_Windows_Foundation_IAsyncOperation_1_Windows_Storage_StorageFile, 0x5e52f8ce,0xaced,0x5a42,0x95,0xb4,0xf6,0x74,0xdd,0x84,0x88,0x5e);
		impl<'a> ComIid for Windows_Foundation_IAsyncOperation<&'a Windows_Storage_StorageFile> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_IAsyncOperation_1_Windows_Storage_StorageFile }
		}
		DEFINE_GUID!(IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_Diagnostics_IFileLoggingSession_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs, 0x0c6563b0,0x9d8b,0x5b60,0x99,0x4b,0xde,0xe1,0x17,0x4d,0x1e,0xfb);
		impl<'a> ComIid for Windows_Foundation_TypedEventHandler<&'a Windows_Foundation_Diagnostics_IFileLoggingSession, &'a Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs> {
			fn iid()-> ::w::REFIID { &IID_Windows_Foundation_TypedEventHandler_2_Windows_Foundation_Diagnostics_IFileLoggingSession_Windows_Foundation_Diagnostics_LogFileGeneratedEventArgs }
		}