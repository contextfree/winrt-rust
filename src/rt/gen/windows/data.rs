pub mod html { // Windows.Data.Html
use crate::prelude::*;
DEFINE_IID!(IID_IHtmlUtilities, 4273998557, 9113, 20396, 181, 167, 5, 233, 172, 215, 24, 29);
RT_INTERFACE!{static interface IHtmlUtilities(IHtmlUtilitiesVtbl): IInspectable(IInspectableVtbl) [IID_IHtmlUtilities] {
    fn ConvertToText(&self, html: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IHtmlUtilities> {
    #[inline] pub fn convert_to_text(&self, html: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ConvertToText)(self.as_abi() as *const _ as *mut _, html.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class HtmlUtilities}
impl RtActivatable<IHtmlUtilities> for HtmlUtilities {}
impl HtmlUtilities {
    #[inline] pub fn convert_to_text(html: &HStringArg) -> Result<HString> {
        <Self as RtActivatable<IHtmlUtilities>>::get_activation_factory().convert_to_text(html)
    }
}
DEFINE_CLSID!(HtmlUtilities(&[87,105,110,100,111,119,115,46,68,97,116,97,46,72,116,109,108,46,72,116,109,108,85,116,105,108,105,116,105,101,115,0]) [CLSID_HtmlUtilities]);
} // Windows.Data.Html
pub mod json { // Windows.Data.Json
use crate::prelude::*;
DEFINE_IID!(IID_IJsonArray, 146922934, 3261, 19098, 181, 211, 47, 133, 45, 195, 126, 129);
RT_INTERFACE!{interface IJsonArray(IJsonArrayVtbl): IInspectable(IInspectableVtbl) [IID_IJsonArray] {
    fn GetObjectAt(&self, index: u32, out: *mut *mut JsonObject) -> HRESULT,
    fn GetArrayAt(&self, index: u32, out: *mut *mut JsonArray) -> HRESULT,
    fn GetStringAt(&self, index: u32, out: *mut HSTRING) -> HRESULT,
    fn GetNumberAt(&self, index: u32, out: *mut f64) -> HRESULT,
    fn GetBooleanAt(&self, index: u32, out: *mut bool) -> HRESULT
}}
impl ComPtr<IJsonArray> {
    #[inline] pub fn get_object_at(&self, index: u32) -> Result<Option<ComPtr<JsonObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetObjectAt)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_array_at(&self, index: u32) -> Result<Option<ComPtr<JsonArray>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetArrayAt)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string_at(&self, index: u32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetStringAt)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_number_at(&self, index: u32) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNumberAt)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean_at(&self, index: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetBooleanAt)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonArray: IJsonArray}
impl RtActivatable<IJsonArrayStatics> for JsonArray {}
impl RtActivatable<IActivationFactory> for JsonArray {}
impl JsonArray {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<JsonArray>>> {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<JsonArray>>, bool)> {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(JsonArray(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,65,114,114,97,121,0]) [CLSID_JsonArray]);
DEFINE_IID!(IID_IJsonArrayStatics, 3675534505, 57700, 18847, 147, 226, 138, 143, 73, 187, 144, 186);
RT_INTERFACE!{static interface IJsonArrayStatics(IJsonArrayStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonArrayStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonArray) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonArray, out: *mut bool) -> HRESULT
}}
impl ComPtr<IJsonArrayStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<JsonArray>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Parse)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<JsonArray>>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).TryParse)(self.as_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(result), out)) } else { err(hr) }
    }}
}
RT_CLASS!{static class JsonError}
impl RtActivatable<IJsonErrorStatics2> for JsonError {}
impl JsonError {
    #[inline] pub fn get_json_status(hresult: i32) -> Result<JsonErrorStatus> {
        <Self as RtActivatable<IJsonErrorStatics2>>::get_activation_factory().get_json_status(hresult)
    }
}
DEFINE_CLSID!(JsonError(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,69,114,114,111,114,0]) [CLSID_JsonError]);
DEFINE_IID!(IID_IJsonErrorStatics2, 1077948634, 34768, 17260, 131, 171, 252, 123, 18, 192, 204, 38);
RT_INTERFACE!{static interface IJsonErrorStatics2(IJsonErrorStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IJsonErrorStatics2] {
    fn GetJsonStatus(&self, hresult: i32, out: *mut JsonErrorStatus) -> HRESULT
}}
impl ComPtr<IJsonErrorStatics2> {
    #[inline] pub fn get_json_status(&self, hresult: i32) -> Result<JsonErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetJsonStatus)(self.as_abi() as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum JsonErrorStatus: i32 {
    Unknown = 0, InvalidJsonString = 1, InvalidJsonNumber = 2, JsonValueNotFound = 3, ImplementationLimit = 4,
}}
DEFINE_IID!(IID_IJsonObject, 105784541, 10690, 20355, 154, 193, 158, 225, 21, 120, 190, 179);
RT_INTERFACE!{interface IJsonObject(IJsonObjectVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObject] {
    fn GetNamedValue(&self, name: HSTRING, out: *mut *mut JsonValue) -> HRESULT,
    fn SetNamedValue(&self, name: HSTRING, value: *mut IJsonValue) -> HRESULT,
    fn GetNamedObject(&self, name: HSTRING, out: *mut *mut JsonObject) -> HRESULT,
    fn GetNamedArray(&self, name: HSTRING, out: *mut *mut JsonArray) -> HRESULT,
    fn GetNamedString(&self, name: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedNumber(&self, name: HSTRING, out: *mut f64) -> HRESULT,
    fn GetNamedBoolean(&self, name: HSTRING, out: *mut bool) -> HRESULT
}}
impl ComPtr<IJsonObject> {
    #[inline] pub fn get_named_value(&self, name: &HStringArg) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedValue)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_value(&self, name: &HStringArg, value: &ComPtr<IJsonValue>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetNamedValue)(self.as_abi() as *const _ as *mut _, name.get(), value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_named_object(&self, name: &HStringArg) -> Result<Option<ComPtr<JsonObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedObject)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_array(&self, name: &HStringArg) -> Result<Option<ComPtr<JsonArray>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedArray)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_string(&self, name: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedString)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_number(&self, name: &HStringArg) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNamedNumber)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_named_boolean(&self, name: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNamedBoolean)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonObject: IJsonObject}
impl RtActivatable<IJsonObjectStatics> for JsonObject {}
impl RtActivatable<IActivationFactory> for JsonObject {}
impl JsonObject {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<JsonObject>>> {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<JsonObject>>, bool)> {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(JsonObject(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,79,98,106,101,99,116,0]) [CLSID_JsonObject]);
DEFINE_IID!(IID_IJsonObjectStatics, 579465561, 21726, 17880, 171, 204, 34, 96, 63, 160, 102, 160);
RT_INTERFACE!{static interface IJsonObjectStatics(IJsonObjectStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObjectStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonObject) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonObject, out: *mut bool) -> HRESULT
}}
impl ComPtr<IJsonObjectStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<JsonObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Parse)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<JsonObject>>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).TryParse)(self.as_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(result), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonObjectWithDefaultValues, 3647001250, 47088, 20224, 142, 68, 216, 44, 244, 21, 234, 19);
RT_INTERFACE!{interface IJsonObjectWithDefaultValues(IJsonObjectWithDefaultValuesVtbl): IInspectable(IInspectableVtbl) [IID_IJsonObjectWithDefaultValues] {
    fn GetNamedValueOrDefault(&self, name: HSTRING, defaultValue: *mut JsonValue, out: *mut *mut JsonValue) -> HRESULT,
    fn GetNamedObjectOrDefault(&self, name: HSTRING, defaultValue: *mut JsonObject, out: *mut *mut JsonObject) -> HRESULT,
    fn GetNamedStringOrDefault(&self, name: HSTRING, defaultValue: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedArrayOrDefault(&self, name: HSTRING, defaultValue: *mut JsonArray, out: *mut *mut JsonArray) -> HRESULT,
    fn GetNamedNumberOrDefault(&self, name: HSTRING, defaultValue: f64, out: *mut f64) -> HRESULT,
    fn GetNamedBooleanOrDefault(&self, name: HSTRING, defaultValue: bool, out: *mut bool) -> HRESULT
}}
impl ComPtr<IJsonObjectWithDefaultValues> {
    #[inline] pub fn get_named_value_or_default(&self, name: &HStringArg, defaultValue: &ComPtr<JsonValue>) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedValueOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_object_or_default(&self, name: &HStringArg, defaultValue: &ComPtr<JsonObject>) -> Result<Option<ComPtr<JsonObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedObjectOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_string_or_default(&self, name: &HStringArg, defaultValue: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedStringOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_array_or_default(&self, name: &HStringArg, defaultValue: &ComPtr<JsonArray>) -> Result<Option<ComPtr<JsonArray>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedArrayOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_number_or_default(&self, name: &HStringArg, defaultValue: f64) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNamedNumberOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_named_boolean_or_default(&self, name: &HStringArg, defaultValue: bool) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNamedBooleanOrDefault)(self.as_abi() as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonValue, 2736889547, 61619, 19917, 190, 238, 25, 212, 140, 211, 237, 30);
RT_INTERFACE!{interface IJsonValue(IJsonValueVtbl): IInspectable(IInspectableVtbl) [IID_IJsonValue] {
    fn get_ValueType(&self, out: *mut JsonValueType) -> HRESULT,
    fn Stringify(&self, out: *mut HSTRING) -> HRESULT,
    fn GetString(&self, out: *mut HSTRING) -> HRESULT,
    fn GetNumber(&self, out: *mut f64) -> HRESULT,
    fn GetBoolean(&self, out: *mut bool) -> HRESULT,
    fn GetArray(&self, out: *mut *mut JsonArray) -> HRESULT,
    fn GetObject(&self, out: *mut *mut JsonObject) -> HRESULT
}}
impl ComPtr<IJsonValue> {
    #[inline] pub fn get_value_type(&self) -> Result<JsonValueType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ValueType)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn stringify(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Stringify)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetString)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_number(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNumber)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetBoolean)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_array(&self) -> Result<Option<ComPtr<JsonArray>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetArray)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_object(&self) -> Result<Option<ComPtr<JsonObject>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetObject)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonValue: IJsonValue}
impl RtActivatable<IJsonValueStatics> for JsonValue {}
impl RtActivatable<IJsonValueStatics2> for JsonValue {}
impl JsonValue {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<ComPtr<JsonValue>>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<ComPtr<JsonValue>>, bool)> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().try_parse(input)
    }
    #[inline] pub fn create_boolean_value(input: bool) -> Result<Option<ComPtr<JsonValue>>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_boolean_value(input)
    }
    #[inline] pub fn create_number_value(input: f64) -> Result<Option<ComPtr<JsonValue>>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_number_value(input)
    }
    #[inline] pub fn create_string_value(input: &HStringArg) -> Result<Option<ComPtr<JsonValue>>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_string_value(input)
    }
    #[inline] pub fn create_null_value() -> Result<Option<ComPtr<JsonValue>>> {
        <Self as RtActivatable<IJsonValueStatics2>>::get_activation_factory().create_null_value()
    }
}
DEFINE_CLSID!(JsonValue(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,86,97,108,117,101,0]) [CLSID_JsonValue]);
DEFINE_IID!(IID_IJsonValueStatics, 1600869450, 12115, 18657, 145, 163, 247, 139, 80, 166, 52, 92);
RT_INTERFACE!{static interface IJsonValueStatics(IJsonValueStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut *mut JsonValue) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut *mut JsonValue, out: *mut bool) -> HRESULT,
    fn CreateBooleanValue(&self, input: bool, out: *mut *mut JsonValue) -> HRESULT,
    fn CreateNumberValue(&self, input: f64, out: *mut *mut JsonValue) -> HRESULT,
    fn CreateStringValue(&self, input: HSTRING, out: *mut *mut JsonValue) -> HRESULT
}}
impl ComPtr<IJsonValueStatics> {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Parse)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<ComPtr<JsonValue>>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).TryParse)(self.as_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((ComPtr::wrap_optional(result), out)) } else { err(hr) }
    }}
    #[inline] pub fn create_boolean_value(&self, input: bool) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateBooleanValue)(self.as_abi() as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_number_value(&self, input: f64) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateNumberValue)(self.as_abi() as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_string_value(&self, input: &HStringArg) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateStringValue)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonValueStatics2, 496946148, 16360, 17205, 131, 146, 147, 216, 227, 104, 101, 240);
RT_INTERFACE!{static interface IJsonValueStatics2(IJsonValueStatics2Vtbl): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics2] {
    fn CreateNullValue(&self, out: *mut *mut JsonValue) -> HRESULT
}}
impl ComPtr<IJsonValueStatics2> {
    #[inline] pub fn create_null_value(&self) -> Result<Option<ComPtr<JsonValue>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateNullValue)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum JsonValueType: i32 {
    Null = 0, Boolean = 1, Number = 2, String = 3, Array = 4, Object = 5,
}}
} // Windows.Data.Json
pub mod pdf { // Windows.Data.Pdf
use crate::prelude::*;
DEFINE_IID!(IID_IPdfDocument, 2893987549, 33018, 16521, 132, 110, 129, 183, 127, 245, 168, 108);
RT_INTERFACE!{interface IPdfDocument(IPdfDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IPdfDocument] {
    fn GetPage(&self, pageIndex: u32, out: *mut *mut PdfPage) -> HRESULT,
    fn get_PageCount(&self, out: *mut u32) -> HRESULT,
    fn get_IsPasswordProtected(&self, out: *mut bool) -> HRESULT
}}
impl ComPtr<IPdfDocument> {
    #[inline] pub fn get_page(&self, pageIndex: u32) -> Result<Option<ComPtr<PdfPage>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetPage)(self.as_abi() as *const _ as *mut _, pageIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PageCount)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_password_protected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsPasswordProtected)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfDocument: IPdfDocument}
impl RtActivatable<IPdfDocumentStatics> for PdfDocument {}
impl PdfDocument {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &ComPtr<super::super::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_password_async(file: &ComPtr<super::super::storage::IStorageFile>, password: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_with_password_async(file, password)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(inputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_async(inputStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_password_async(inputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>, password: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_with_password_async(inputStream, password)
    }
}
DEFINE_CLSID!(PdfDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,80,100,102,46,80,100,102,68,111,99,117,109,101,110,116,0]) [CLSID_PdfDocument]);
DEFINE_IID!(IID_IPdfDocumentStatics, 1127877471, 49159, 18312, 144, 242, 8, 20, 61, 146, 37, 153);
RT_INTERFACE!{static interface IPdfDocumentStatics(IPdfDocumentStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfDocumentStatics] {
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: *mut super::super::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithPasswordAsync(&self, file: *mut super::super::storage::IStorageFile, password: HSTRING, out: *mut *mut foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamAsync(&self, inputStream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncOperation<PdfDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithPasswordAsync(&self, inputStream: *mut super::super::storage::streams::IRandomAccessStream, password: HSTRING, out: *mut *mut foundation::IAsyncOperation<PdfDocument>) -> HRESULT
}}
impl ComPtr<IPdfDocumentStatics> {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(&self, file: &ComPtr<super::super::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFileAsync)(self.as_abi() as *const _ as *mut _, file.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_password_async(&self, file: &ComPtr<super::super::storage::IStorageFile>, password: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFileWithPasswordAsync)(self.as_abi() as *const _ as *mut _, file.as_abi() as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(&self, inputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStreamAsync)(self.as_abi() as *const _ as *mut _, inputStream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_password_async(&self, inputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>, password: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<PdfDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromStreamWithPasswordAsync)(self.as_abi() as *const _ as *mut _, inputStream.as_abi() as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPdfPage, 2645864648, 21280, 19708, 173, 118, 73, 63, 218, 208, 229, 148);
RT_INTERFACE!{interface IPdfPage(IPdfPageVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPage] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IRandomAccessStream, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderWithOptionsToStreamAsync(&self, outputStream: *mut super::super::storage::streams::IRandomAccessStream, options: *mut PdfPageRenderOptions, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn PreparePageAsync(&self, out: *mut *mut foundation::IAsyncAction) -> HRESULT,
    fn get_Index(&self, out: *mut u32) -> HRESULT,
    fn get_Size(&self, out: *mut foundation::Size) -> HRESULT,
    fn get_Dimensions(&self, out: *mut *mut PdfPageDimensions) -> HRESULT,
    fn get_Rotation(&self, out: *mut PdfPageRotation) -> HRESULT,
    fn get_PreferredZoom(&self, out: *mut f32) -> HRESULT
}}
impl ComPtr<IPdfPage> {
    #[cfg(feature="windows-storage")] #[inline] pub fn render_to_stream_async(&self, outputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RenderToStreamAsync)(self.as_abi() as *const _ as *mut _, outputStream.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn render_with_options_to_stream_async(&self, outputStream: &ComPtr<super::super::storage::streams::IRandomAccessStream>, options: &ComPtr<PdfPageRenderOptions>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RenderWithOptionsToStreamAsync)(self.as_abi() as *const _ as *mut _, outputStream.as_abi() as *const _ as *mut _, options.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn prepare_page_async(&self) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).PreparePageAsync)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_index(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Index)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Size)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dimensions(&self) -> Result<Option<ComPtr<PdfPageDimensions>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Dimensions)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rotation(&self) -> Result<PdfPageRotation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Rotation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_preferred_zoom(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_PreferredZoom)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfPage: IPdfPage}
DEFINE_IID!(IID_IPdfPageDimensions, 571933809, 12606, 17640, 131, 93, 99, 163, 231, 98, 74, 16);
RT_INTERFACE!{interface IPdfPageDimensions(IPdfPageDimensionsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPageDimensions] {
    fn get_MediaBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_CropBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_BleedBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_TrimBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_ArtBox(&self, out: *mut foundation::Rect) -> HRESULT
}}
impl ComPtr<IPdfPageDimensions> {
    #[inline] pub fn get_media_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MediaBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_crop_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_CropBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bleed_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BleedBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trim_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_TrimBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_art_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ArtBox)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfPageDimensions: IPdfPageDimensions}
DEFINE_IID!(IID_IPdfPageRenderOptions, 1016595823, 47055, 19497, 154, 4, 82, 217, 2, 103, 244, 37);
RT_INTERFACE!{interface IPdfPageRenderOptions(IPdfPageRenderOptionsVtbl): IInspectable(IInspectableVtbl) [IID_IPdfPageRenderOptions] {
    fn get_SourceRect(&self, out: *mut foundation::Rect) -> HRESULT,
    fn put_SourceRect(&self, value: foundation::Rect) -> HRESULT,
    fn get_DestinationWidth(&self, out: *mut u32) -> HRESULT,
    fn put_DestinationWidth(&self, value: u32) -> HRESULT,
    fn get_DestinationHeight(&self, out: *mut u32) -> HRESULT,
    fn put_DestinationHeight(&self, value: u32) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy6(&self) -> (),
    #[cfg(feature="windows-ui")] fn get_BackgroundColor(&self, out: *mut super::super::ui::Color) -> HRESULT,
    #[cfg(not(feature="windows-ui"))] fn __Dummy7(&self) -> (),
    #[cfg(feature="windows-ui")] fn put_BackgroundColor(&self, value: super::super::ui::Color) -> HRESULT,
    fn get_IsIgnoringHighContrast(&self, out: *mut bool) -> HRESULT,
    fn put_IsIgnoringHighContrast(&self, value: bool) -> HRESULT,
    fn get_BitmapEncoderId(&self, out: *mut Guid) -> HRESULT,
    fn put_BitmapEncoderId(&self, value: Guid) -> HRESULT
}}
impl ComPtr<IPdfPageRenderOptions> {
    #[inline] pub fn get_source_rect(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SourceRect)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_source_rect(&self, value: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_SourceRect)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_destination_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DestinationWidth)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_destination_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DestinationWidth)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_destination_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_DestinationHeight)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_destination_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_DestinationHeight)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_background_color(&self) -> Result<super::super::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BackgroundColor)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_background_color(&self, value: super::super::ui::Color) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_BackgroundColor)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_ignoring_high_contrast(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_IsIgnoringHighContrast)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_ignoring_high_contrast(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_IsIgnoringHighContrast)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_BitmapEncoderId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bitmap_encoder_id(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_BitmapEncoderId)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfPageRenderOptions: IPdfPageRenderOptions}
impl RtActivatable<IActivationFactory> for PdfPageRenderOptions {}
DEFINE_CLSID!(PdfPageRenderOptions(&[87,105,110,100,111,119,115,46,68,97,116,97,46,80,100,102,46,80,100,102,80,97,103,101,82,101,110,100,101,114,79,112,116,105,111,110,115,0]) [CLSID_PdfPageRenderOptions]);
RT_ENUM! { enum PdfPageRotation: i32 {
    Normal = 0, Rotate90 = 1, Rotate180 = 2, Rotate270 = 3,
}}
} // Windows.Data.Pdf
pub mod text { // Windows.Data.Text
use crate::prelude::*;
RT_ENUM! { enum AlternateNormalizationFormat: i32 {
    NotNormalized = 0, Number = 1, Currency = 3, Date = 4, Time = 5,
}}
DEFINE_IID!(IID_IAlternateWordForm, 1194945566, 20921, 16903, 145, 70, 36, 142, 99, 106, 29, 29);
RT_INTERFACE!{interface IAlternateWordForm(IAlternateWordFormVtbl): IInspectable(IInspectableVtbl) [IID_IAlternateWordForm] {
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NormalizationFormat(&self, out: *mut AlternateNormalizationFormat) -> HRESULT
}}
impl ComPtr<IAlternateWordForm> {
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SourceTextSegment)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_alternate_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AlternateText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_normalization_format(&self) -> Result<AlternateNormalizationFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_NormalizationFormat)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AlternateWordForm: IAlternateWordForm}
DEFINE_IID!(IID_ISelectableWordSegment, 2439662775, 35495, 19576, 179, 116, 93, 237, 183, 82, 230, 11);
RT_INTERFACE!{interface ISelectableWordSegment(ISelectableWordSegmentVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT
}}
impl ComPtr<ISelectableWordSegment> {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Text)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SourceTextSegment)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SelectableWordSegment: ISelectableWordSegment}
DEFINE_IID!(IID_SelectableWordSegmentsTokenizingHandler, 977140892, 44766, 19911, 158, 108, 65, 192, 68, 189, 53, 146);
RT_DELEGATE!{delegate SelectableWordSegmentsTokenizingHandler(SelectableWordSegmentsTokenizingHandlerVtbl, SelectableWordSegmentsTokenizingHandlerImpl) [IID_SelectableWordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: *mut foundation::collections::IIterable<SelectableWordSegment>, words: *mut foundation::collections::IIterable<SelectableWordSegment>) -> HRESULT
}}
impl ComPtr<SelectableWordSegmentsTokenizingHandler> {
    #[inline] pub fn invoke(&self, precedingWords: &ComPtr<foundation::collections::IIterable<SelectableWordSegment>>, words: &ComPtr<foundation::collections::IIterable<SelectableWordSegment>>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Invoke)(self.as_abi() as *const _ as *mut _, precedingWords.as_abi() as *const _ as *mut _, words.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISelectableWordsSegmenter, 4141625831, 19219, 17861, 136, 151, 125, 113, 38, 158, 8, 93);
RT_INTERFACE!{interface ISelectableWordsSegmenter(ISelectableWordsSegmenterVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut *mut SelectableWordSegment) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut *mut foundation::collections::IVectorView<SelectableWordSegment>) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: *mut SelectableWordSegmentsTokenizingHandler) -> HRESULT
}}
impl ComPtr<ISelectableWordsSegmenter> {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ResolvedLanguage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<Option<ComPtr<SelectableWordSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetTokenAt)(self.as_abi() as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tokens(&self, text: &HStringArg) -> Result<Option<ComPtr<foundation::collections::IVectorView<SelectableWordSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetTokens)(self.as_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &ComPtr<SelectableWordSegmentsTokenizingHandler>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Tokenize)(self.as_abi() as *const _ as *mut _, text.get(), startIndex, handler.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SelectableWordsSegmenter: ISelectableWordsSegmenter}
impl RtActivatable<ISelectableWordsSegmenterFactory> for SelectableWordsSegmenter {}
impl SelectableWordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<ComPtr<SelectableWordsSegmenter>> {
        <Self as RtActivatable<ISelectableWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }
}
DEFINE_CLSID!(SelectableWordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,108,101,99,116,97,98,108,101,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_SelectableWordsSegmenter]);
DEFINE_IID!(IID_ISelectableWordsSegmenterFactory, 2356835912, 24663, 17209, 188, 112, 242, 16, 1, 10, 65, 80);
RT_INTERFACE!{static interface ISelectableWordsSegmenterFactory(ISelectableWordsSegmenterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut *mut SelectableWordsSegmenter) -> HRESULT
}}
impl ComPtr<ISelectableWordsSegmenterFactory> {
    #[inline] pub fn create_with_language(&self, language: &HStringArg) -> Result<ComPtr<SelectableWordsSegmenter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithLanguage)(self.as_abi() as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISemanticTextQuery, 1780263761, 8114, 18697, 128, 184, 53, 115, 26, 43, 62, 127);
RT_INTERFACE!{interface ISemanticTextQuery(ISemanticTextQueryVtbl): IInspectable(IInspectableVtbl) [IID_ISemanticTextQuery] {
    fn Find(&self, content: HSTRING, out: *mut *mut foundation::collections::IVectorView<TextSegment>) -> HRESULT,
    fn FindInProperty(&self, propertyContent: HSTRING, propertyName: HSTRING, out: *mut *mut foundation::collections::IVectorView<TextSegment>) -> HRESULT
}}
impl ComPtr<ISemanticTextQuery> {
    #[inline] pub fn find(&self, content: &HStringArg) -> Result<Option<ComPtr<foundation::collections::IVectorView<TextSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Find)(self.as_abi() as *const _ as *mut _, content.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_in_property(&self, propertyContent: &HStringArg, propertyName: &HStringArg) -> Result<Option<ComPtr<foundation::collections::IVectorView<TextSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).FindInProperty)(self.as_abi() as *const _ as *mut _, propertyContent.get(), propertyName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SemanticTextQuery: ISemanticTextQuery}
impl RtActivatable<ISemanticTextQueryFactory> for SemanticTextQuery {}
impl SemanticTextQuery {
    #[inline] pub fn create(aqsFilter: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create(aqsFilter)
    }
    #[inline] pub fn create_with_language(aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create_with_language(aqsFilter, filterLanguage)
    }
}
DEFINE_CLSID!(SemanticTextQuery(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,109,97,110,116,105,99,84,101,120,116,81,117,101,114,121,0]) [CLSID_SemanticTextQuery]);
DEFINE_IID!(IID_ISemanticTextQueryFactory, 596378883, 63893, 17799, 135, 119, 162, 183, 216, 10, 207, 239);
RT_INTERFACE!{static interface ISemanticTextQueryFactory(ISemanticTextQueryFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ISemanticTextQueryFactory] {
    fn Create(&self, aqsFilter: HSTRING, out: *mut *mut SemanticTextQuery) -> HRESULT,
    fn CreateWithLanguage(&self, aqsFilter: HSTRING, filterLanguage: HSTRING, out: *mut *mut SemanticTextQuery) -> HRESULT
}}
impl ComPtr<ISemanticTextQueryFactory> {
    #[inline] pub fn create(&self, aqsFilter: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, aqsFilter.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_language(&self, aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<ComPtr<SemanticTextQuery>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithLanguage)(self.as_abi() as *const _ as *mut _, aqsFilter.get(), filterLanguage.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextConversionGenerator, 56650334, 10921, 19126, 175, 139, 165, 98, 182, 58, 137, 146);
RT_INTERFACE!{interface ITextConversionGenerator(ITextConversionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT
}}
impl ComPtr<ITextConversionGenerator> {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ResolvedLanguage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_async(&self, input: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCandidatesAsync)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCandidatesWithMaxCountAsync)(self.as_abi() as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextConversionGenerator: ITextConversionGenerator}
impl RtActivatable<ITextConversionGeneratorFactory> for TextConversionGenerator {}
impl TextConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextConversionGenerator>> {
        <Self as RtActivatable<ITextConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextConversionGenerator]);
DEFINE_IID!(IID_ITextConversionGeneratorFactory, 4239013761, 12419, 18859, 190, 21, 86, 223, 187, 183, 77, 111);
RT_INTERFACE!{static interface ITextConversionGeneratorFactory(ITextConversionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextConversionGenerator) -> HRESULT
}}
impl ComPtr<ITextConversionGeneratorFactory> {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextConversionGenerator>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextPhoneme, 2472715274, 39802, 17769, 148, 207, 216, 79, 47, 56, 207, 155);
RT_INTERFACE!{interface ITextPhoneme(ITextPhonemeVtbl): IInspectable(IInspectableVtbl) [IID_ITextPhoneme] {
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReadingText(&self, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<ITextPhoneme> {
    #[inline] pub fn get_display_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DisplayText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reading_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ReadingText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextPhoneme: ITextPhoneme}
DEFINE_IID!(IID_ITextPredictionGenerator, 1588374279, 44017, 19638, 157, 158, 50, 111, 43, 70, 135, 86);
RT_INTERFACE!{interface ITextPredictionGenerator(ITextPredictionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextPredictionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT
}}
impl ComPtr<ITextPredictionGenerator> {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ResolvedLanguage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_async(&self, input: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCandidatesAsync)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCandidatesWithMaxCountAsync)(self.as_abi() as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextPredictionGenerator: ITextPredictionGenerator}
impl RtActivatable<ITextPredictionGeneratorFactory> for TextPredictionGenerator {}
impl TextPredictionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextPredictionGenerator>> {
        <Self as RtActivatable<ITextPredictionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextPredictionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,80,114,101,100,105,99,116,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextPredictionGenerator]);
DEFINE_IID!(IID_ITextPredictionGenerator2, 3091669944, 11383, 18538, 144, 10, 163, 69, 62, 237, 193, 93);
RT_INTERFACE!{interface ITextPredictionGenerator2(ITextPredictionGenerator2Vtbl): IInspectable(IInspectableVtbl) [IID_ITextPredictionGenerator2] {
    fn GetCandidatesWithParametersAsync(&self, input: HSTRING, maxCandidates: u32, predictionOptions: TextPredictionOptions, previousStrings: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT,
    fn GetNextWordCandidatesAsync(&self, maxCandidates: u32, previousStrings: *mut foundation::collections::IIterable<HString>, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_InputScope(&self, out: *mut super::super::ui::text::core::CoreTextInputScope) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_InputScope(&self, value: super::super::ui::text::core::CoreTextInputScope) -> HRESULT
}}
impl ComPtr<ITextPredictionGenerator2> {
    #[inline] pub fn get_candidates_with_parameters_async(&self, input: &HStringArg, maxCandidates: u32, predictionOptions: TextPredictionOptions, previousStrings: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetCandidatesWithParametersAsync)(self.as_abi() as *const _ as *mut _, input.get(), maxCandidates, predictionOptions, previousStrings.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_word_candidates_async(&self, maxCandidates: u32, previousStrings: &ComPtr<foundation::collections::IIterable<HString>>) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNextWordCandidatesAsync)(self.as_abi() as *const _ as *mut _, maxCandidates, previousStrings.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_input_scope(&self) -> Result<super::super::ui::text::core::CoreTextInputScope> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_InputScope)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_input_scope(&self, value: super::super::ui::text::core::CoreTextInputScope) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_InputScope)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextPredictionGeneratorFactory, 1918350358, 35746, 18257, 157, 48, 157, 133, 67, 86, 83, 162);
RT_INTERFACE!{static interface ITextPredictionGeneratorFactory(ITextPredictionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextPredictionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextPredictionGenerator) -> HRESULT
}}
impl ComPtr<ITextPredictionGeneratorFactory> {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextPredictionGenerator>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TextPredictionOptions: u32 {
    None = 0, Predictions = 1, Corrections = 2,
}}
DEFINE_IID!(IID_ITextReverseConversionGenerator, 1374156052, 40017, 19846, 174, 27, 180, 152, 251, 173, 131, 19);
RT_INTERFACE!{interface ITextReverseConversionGenerator(ITextReverseConversionGeneratorVtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn ConvertBackAsync(&self, input: HSTRING, out: *mut *mut foundation::IAsyncOperation<HString>) -> HRESULT
}}
impl ComPtr<ITextReverseConversionGenerator> {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ResolvedLanguage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn convert_back_async(&self, input: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ConvertBackAsync)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextReverseConversionGenerator: ITextReverseConversionGenerator}
impl RtActivatable<ITextReverseConversionGeneratorFactory> for TextReverseConversionGenerator {}
impl TextReverseConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<ComPtr<TextReverseConversionGenerator>> {
        <Self as RtActivatable<ITextReverseConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextReverseConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,82,101,118,101,114,115,101,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextReverseConversionGenerator]);
DEFINE_IID!(IID_ITextReverseConversionGenerator2, 447730412, 34262, 18173, 130, 138, 58, 72, 48, 250, 110, 24);
RT_INTERFACE!{interface ITextReverseConversionGenerator2(ITextReverseConversionGenerator2Vtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator2] {
    fn GetPhonemesAsync(&self, input: HSTRING, out: *mut *mut foundation::IAsyncOperation<foundation::collections::IVectorView<TextPhoneme>>) -> HRESULT
}}
impl ComPtr<ITextReverseConversionGenerator2> {
    #[inline] pub fn get_phonemes_async(&self, input: &HStringArg) -> Result<ComPtr<foundation::IAsyncOperation<foundation::collections::IVectorView<TextPhoneme>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetPhonemesAsync)(self.as_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextReverseConversionGeneratorFactory, 1673450278, 8154, 16886, 137, 213, 35, 221, 234, 60, 114, 154);
RT_INTERFACE!{static interface ITextReverseConversionGeneratorFactory(ITextReverseConversionGeneratorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut *mut TextReverseConversionGenerator) -> HRESULT
}}
impl ComPtr<ITextReverseConversionGeneratorFactory> {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<ComPtr<TextReverseConversionGenerator>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Create)(self.as_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
RT_STRUCT! { struct TextSegment {
    StartPosition: u32, Length: u32,
}}
RT_CLASS!{static class UnicodeCharacters}
impl RtActivatable<IUnicodeCharactersStatics> for UnicodeCharacters {}
impl UnicodeCharacters {
    #[inline] pub fn get_codepoint_from_surrogate_pair(highSurrogate: u32, lowSurrogate: u32) -> Result<u32> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_codepoint_from_surrogate_pair(highSurrogate, lowSurrogate)
    }
    #[inline] pub fn get_surrogate_pair_from_codepoint(codepoint: u32) -> Result<(Char, Char)> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_surrogate_pair_from_codepoint(codepoint)
    }
    #[inline] pub fn is_high_surrogate(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_high_surrogate(codepoint)
    }
    #[inline] pub fn is_low_surrogate(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_low_surrogate(codepoint)
    }
    #[inline] pub fn is_supplementary(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_supplementary(codepoint)
    }
    #[inline] pub fn is_noncharacter(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_noncharacter(codepoint)
    }
    #[inline] pub fn is_whitespace(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_whitespace(codepoint)
    }
    #[inline] pub fn is_alphabetic(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_alphabetic(codepoint)
    }
    #[inline] pub fn is_cased(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_cased(codepoint)
    }
    #[inline] pub fn is_uppercase(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_uppercase(codepoint)
    }
    #[inline] pub fn is_lowercase(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_lowercase(codepoint)
    }
    #[inline] pub fn is_id_start(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_id_start(codepoint)
    }
    #[inline] pub fn is_id_continue(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_id_continue(codepoint)
    }
    #[inline] pub fn is_grapheme_base(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_grapheme_base(codepoint)
    }
    #[inline] pub fn is_grapheme_extend(codepoint: u32) -> Result<bool> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().is_grapheme_extend(codepoint)
    }
    #[inline] pub fn get_numeric_type(codepoint: u32) -> Result<UnicodeNumericType> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_numeric_type(codepoint)
    }
    #[inline] pub fn get_general_category(codepoint: u32) -> Result<UnicodeGeneralCategory> {
        <Self as RtActivatable<IUnicodeCharactersStatics>>::get_activation_factory().get_general_category(codepoint)
    }
}
DEFINE_CLSID!(UnicodeCharacters(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,85,110,105,99,111,100,101,67,104,97,114,97,99,116,101,114,115,0]) [CLSID_UnicodeCharacters]);
DEFINE_IID!(IID_IUnicodeCharactersStatics, 2542837383, 37521, 20369, 182, 200, 182, 227, 89, 215, 167, 251);
RT_INTERFACE!{static interface IUnicodeCharactersStatics(IUnicodeCharactersStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IUnicodeCharactersStatics] {
    fn GetCodepointFromSurrogatePair(&self, highSurrogate: u32, lowSurrogate: u32, out: *mut u32) -> HRESULT,
    fn GetSurrogatePairFromCodepoint(&self, codepoint: u32, highSurrogate: *mut Char, lowSurrogate: *mut Char) -> HRESULT,
    fn IsHighSurrogate(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsLowSurrogate(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsSupplementary(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsNoncharacter(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsWhitespace(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsAlphabetic(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsCased(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsUppercase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsLowercase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsIdStart(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsIdContinue(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsGraphemeBase(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn IsGraphemeExtend(&self, codepoint: u32, out: *mut bool) -> HRESULT,
    fn GetNumericType(&self, codepoint: u32, out: *mut UnicodeNumericType) -> HRESULT,
    fn GetGeneralCategory(&self, codepoint: u32, out: *mut UnicodeGeneralCategory) -> HRESULT
}}
impl ComPtr<IUnicodeCharactersStatics> {
    #[inline] pub fn get_codepoint_from_surrogate_pair(&self, highSurrogate: u32, lowSurrogate: u32) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetCodepointFromSurrogatePair)(self.as_abi() as *const _ as *mut _, highSurrogate, lowSurrogate, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_surrogate_pair_from_codepoint(&self, codepoint: u32) -> Result<(Char, Char)> { unsafe { 
        let mut highSurrogate = zeroed(); let mut lowSurrogate = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetSurrogatePairFromCodepoint)(self.as_abi() as *const _ as *mut _, codepoint, &mut highSurrogate, &mut lowSurrogate);
        if hr == S_OK { Ok((highSurrogate, lowSurrogate)) } else { err(hr) }
    }}
    #[inline] pub fn is_high_surrogate(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsHighSurrogate)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_low_surrogate(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsLowSurrogate)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_supplementary(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsSupplementary)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_noncharacter(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsNoncharacter)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_whitespace(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsWhitespace)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_alphabetic(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsAlphabetic)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_cased(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsCased)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_uppercase(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsUppercase)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_lowercase(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsLowercase)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_id_start(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsIdStart)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_id_continue(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsIdContinue)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_grapheme_base(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsGraphemeBase)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_grapheme_extend(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).IsGraphemeExtend)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_numeric_type(&self, codepoint: u32) -> Result<UnicodeNumericType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetNumericType)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_general_category(&self, codepoint: u32) -> Result<UnicodeGeneralCategory> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).GetGeneralCategory)(self.as_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum UnicodeGeneralCategory: i32 {
    UppercaseLetter = 0, LowercaseLetter = 1, TitlecaseLetter = 2, ModifierLetter = 3, OtherLetter = 4, NonspacingMark = 5, SpacingCombiningMark = 6, EnclosingMark = 7, DecimalDigitNumber = 8, LetterNumber = 9, OtherNumber = 10, SpaceSeparator = 11, LineSeparator = 12, ParagraphSeparator = 13, Control = 14, Format = 15, Surrogate = 16, PrivateUse = 17, ConnectorPunctuation = 18, DashPunctuation = 19, OpenPunctuation = 20, ClosePunctuation = 21, InitialQuotePunctuation = 22, FinalQuotePunctuation = 23, OtherPunctuation = 24, MathSymbol = 25, CurrencySymbol = 26, ModifierSymbol = 27, OtherSymbol = 28, NotAssigned = 29,
}}
RT_ENUM! { enum UnicodeNumericType: i32 {
    None = 0, Decimal = 1, Digit = 2, Numeric = 3,
}}
DEFINE_IID!(IID_IWordSegment, 3537156717, 39036, 19648, 182, 189, 212, 154, 17, 179, 143, 154);
RT_INTERFACE!{interface IWordSegment(IWordSegmentVtbl): IInspectable(IInspectableVtbl) [IID_IWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateForms(&self, out: *mut *mut foundation::collections::IVectorView<AlternateWordForm>) -> HRESULT
}}
impl ComPtr<IWordSegment> {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Text)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_SourceTextSegment)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_alternate_forms(&self) -> Result<Option<ComPtr<foundation::collections::IVectorView<AlternateWordForm>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_AlternateForms)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WordSegment: IWordSegment}
DEFINE_IID!(IID_WordSegmentsTokenizingHandler, 2782749527, 48938, 19535, 163, 31, 41, 231, 28, 111, 139, 53);
RT_DELEGATE!{delegate WordSegmentsTokenizingHandler(WordSegmentsTokenizingHandlerVtbl, WordSegmentsTokenizingHandlerImpl) [IID_WordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: *mut foundation::collections::IIterable<WordSegment>, words: *mut foundation::collections::IIterable<WordSegment>) -> HRESULT
}}
impl ComPtr<WordSegmentsTokenizingHandler> {
    #[inline] pub fn invoke(&self, precedingWords: &ComPtr<foundation::collections::IIterable<WordSegment>>, words: &ComPtr<foundation::collections::IIterable<WordSegment>>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Invoke)(self.as_abi() as *const _ as *mut _, precedingWords.as_abi() as *const _ as *mut _, words.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWordsSegmenter, 2259997905, 45822, 20020, 168, 29, 102, 100, 3, 0, 69, 79);
RT_INTERFACE!{interface IWordsSegmenter(IWordsSegmenterVtbl): IInspectable(IInspectableVtbl) [IID_IWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut *mut WordSegment) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut *mut foundation::collections::IVectorView<WordSegment>) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: *mut WordSegmentsTokenizingHandler) -> HRESULT
}}
impl ComPtr<IWordsSegmenter> {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ResolvedLanguage)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<Option<ComPtr<WordSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetTokenAt)(self.as_abi() as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tokens(&self, text: &HStringArg) -> Result<Option<ComPtr<foundation::collections::IVectorView<WordSegment>>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetTokens)(self.as_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &ComPtr<WordSegmentsTokenizingHandler>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Tokenize)(self.as_abi() as *const _ as *mut _, text.get(), startIndex, handler.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WordsSegmenter: IWordsSegmenter}
impl RtActivatable<IWordsSegmenterFactory> for WordsSegmenter {}
impl WordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<ComPtr<WordsSegmenter>> {
        <Self as RtActivatable<IWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }
}
DEFINE_CLSID!(WordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_WordsSegmenter]);
DEFINE_IID!(IID_IWordsSegmenterFactory, 3868684916, 64565, 17756, 139, 251, 109, 127, 70, 83, 202, 151);
RT_INTERFACE!{static interface IWordsSegmenterFactory(IWordsSegmenterFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut *mut WordsSegmenter) -> HRESULT
}}
impl ComPtr<IWordsSegmenterFactory> {
    #[inline] pub fn create_with_language(&self, language: &HStringArg) -> Result<ComPtr<WordsSegmenter>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateWithLanguage)(self.as_abi() as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Data.Text
pub mod xml { // Windows.Data.Xml
pub mod dom { // Windows.Data.Xml.Dom
use crate::prelude::*;
DEFINE_IID!(IID_IDtdEntity, 1779130364, 25524, 18447, 158, 106, 138, 146, 129, 106, 173, 228);
RT_INTERFACE!{interface IDtdEntity(IDtdEntityVtbl): IInspectable(IInspectableVtbl) [IID_IDtdEntity] {
    fn get_PublicId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_SystemId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_NotationName(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl ComPtr<IDtdEntity> {
    #[inline] pub fn get_public_id(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PublicId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SystemId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_notation_name(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NotationName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DtdEntity: IDtdEntity}
DEFINE_IID!(IID_IDtdNotation, 2360664141, 27974, 20187, 171, 115, 223, 131, 197, 26, 211, 151);
RT_INTERFACE!{interface IDtdNotation(IDtdNotationVtbl): IInspectable(IInspectableVtbl) [IID_IDtdNotation] {
    fn get_PublicId(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_SystemId(&self, out: *mut *mut IInspectable) -> HRESULT
}}
impl ComPtr<IDtdNotation> {
    #[inline] pub fn get_public_id(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PublicId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_SystemId)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DtdNotation: IDtdNotation}
RT_ENUM! { enum NodeType: i32 {
    Invalid = 0, ElementNode = 1, AttributeNode = 2, TextNode = 3, DataSectionNode = 4, EntityReferenceNode = 5, EntityNode = 6, ProcessingInstructionNode = 7, CommentNode = 8, DocumentNode = 9, DocumentTypeNode = 10, DocumentFragmentNode = 11, NotationNode = 12,
}}
DEFINE_IID!(IID_IXmlAttribute, 2887010980, 46321, 19894, 178, 6, 138, 34, 195, 8, 219, 10);
RT_INTERFACE!{interface IXmlAttribute(IXmlAttributeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlAttribute] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Specified(&self, out: *mut bool) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IXmlAttribute> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_specified(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Specified)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Value)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Value)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlAttribute: IXmlAttribute}
DEFINE_IID!(IID_IXmlCDataSection, 1292153967, 51389, 17844, 136, 153, 4, 0, 215, 194, 198, 15);
RT_INTERFACE!{interface IXmlCDataSection(IXmlCDataSectionVtbl): IInspectable(IInspectableVtbl) [IID_IXmlCDataSection] {
    
}}
RT_CLASS!{class XmlCDataSection: IXmlCDataSection}
DEFINE_IID!(IID_IXmlCharacterData, 321798827, 20022, 19958, 177, 200, 12, 230, 47, 216, 139, 38);
RT_INTERFACE!{interface IXmlCharacterData(IXmlCharacterDataVtbl): IInspectable(IInspectableVtbl) [IID_IXmlCharacterData] {
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT,
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn SubstringData(&self, offset: u32, count: u32, out: *mut HSTRING) -> HRESULT,
    fn AppendData(&self, data: HSTRING) -> HRESULT,
    fn InsertData(&self, offset: u32, data: HSTRING) -> HRESULT,
    fn DeleteData(&self, offset: u32, count: u32) -> HRESULT,
    fn ReplaceData(&self, offset: u32, count: u32, data: HSTRING) -> HRESULT
}}
impl ComPtr<IXmlCharacterData> {
    #[inline] pub fn get_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Data)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Data)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Length)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn substring_data(&self, offset: u32, count: u32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SubstringData)(self.as_abi() as *const _ as *mut _, offset, count, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_data(&self, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).AppendData)(self.as_abi() as *const _ as *mut _, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn insert_data(&self, offset: u32, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).InsertData)(self.as_abi() as *const _ as *mut _, offset, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn delete_data(&self, offset: u32, count: u32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).DeleteData)(self.as_abi() as *const _ as *mut _, offset, count);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn replace_data(&self, offset: u32, count: u32, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).ReplaceData)(self.as_abi() as *const _ as *mut _, offset, count, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlComment, 3164894421, 46623, 17937, 156, 172, 46, 146, 227, 71, 109, 71);
RT_INTERFACE!{interface IXmlComment(IXmlCommentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlComment] {
    
}}
RT_CLASS!{class XmlComment: IXmlComment}
DEFINE_IID!(IID_IXmlDocument, 4159939846, 7815, 17110, 188, 251, 184, 200, 9, 250, 84, 148);
RT_INTERFACE!{interface IXmlDocument(IXmlDocumentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocument] {
    fn get_Doctype(&self, out: *mut *mut XmlDocumentType) -> HRESULT,
    fn get_Implementation(&self, out: *mut *mut XmlDomImplementation) -> HRESULT,
    fn get_DocumentElement(&self, out: *mut *mut XmlElement) -> HRESULT,
    fn CreateElement(&self, tagName: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn CreateDocumentFragment(&self, out: *mut *mut XmlDocumentFragment) -> HRESULT,
    fn CreateTextNode(&self, data: HSTRING, out: *mut *mut XmlText) -> HRESULT,
    fn CreateComment(&self, data: HSTRING, out: *mut *mut XmlComment) -> HRESULT,
    fn CreateProcessingInstruction(&self, target: HSTRING, data: HSTRING, out: *mut *mut XmlProcessingInstruction) -> HRESULT,
    fn CreateAttribute(&self, name: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn CreateEntityReference(&self, name: HSTRING, out: *mut *mut XmlEntityReference) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn CreateCDataSection(&self, data: HSTRING, out: *mut *mut XmlCDataSection) -> HRESULT,
    fn get_DocumentUri(&self, out: *mut HSTRING) -> HRESULT,
    fn CreateAttributeNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn CreateElementNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn GetElementById(&self, elementId: HSTRING, out: *mut *mut XmlElement) -> HRESULT,
    fn ImportNode(&self, node: *mut IXmlNode, deep: bool, out: *mut *mut IXmlNode) -> HRESULT
}}
impl ComPtr<IXmlDocument> {
    #[inline] pub fn get_doctype(&self) -> Result<Option<ComPtr<XmlDocumentType>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Doctype)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_implementation(&self) -> Result<Option<ComPtr<XmlDomImplementation>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Implementation)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_element(&self) -> Result<Option<ComPtr<XmlElement>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DocumentElement)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_element(&self, tagName: &HStringArg) -> Result<Option<ComPtr<XmlElement>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateElement)(self.as_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_document_fragment(&self) -> Result<Option<ComPtr<XmlDocumentFragment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateDocumentFragment)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_text_node(&self, data: &HStringArg) -> Result<Option<ComPtr<XmlText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateTextNode)(self.as_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_comment(&self, data: &HStringArg) -> Result<Option<ComPtr<XmlComment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateComment)(self.as_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_processing_instruction(&self, target: &HStringArg, data: &HStringArg) -> Result<Option<ComPtr<XmlProcessingInstruction>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateProcessingInstruction)(self.as_abi() as *const _ as *mut _, target.get(), data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attribute(&self, name: &HStringArg) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttribute)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_entity_reference(&self, name: &HStringArg) -> Result<Option<ComPtr<XmlEntityReference>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateEntityReference)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<Option<ComPtr<XmlNodeList>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetElementsByTagName)(self.as_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_cdata_section(&self, data: &HStringArg) -> Result<Option<ComPtr<XmlCDataSection>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateCDataSection)(self.as_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_DocumentUri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attribute_ns(&self, namespaceUri: &ComPtr<IInspectable>, qualifiedName: &HStringArg) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateAttributeNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_element_ns(&self, namespaceUri: &ComPtr<IInspectable>, qualifiedName: &HStringArg) -> Result<Option<ComPtr<XmlElement>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateElementNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_element_by_id(&self, elementId: &HStringArg) -> Result<Option<ComPtr<XmlElement>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetElementById)(self.as_abi() as *const _ as *mut _, elementId.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_node(&self, node: &ComPtr<IXmlNode>, deep: bool) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ImportNode)(self.as_abi() as *const _ as *mut _, node.as_abi() as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDocument: IXmlDocument}
impl RtActivatable<IXmlDocumentStatics> for XmlDocument {}
impl RtActivatable<IActivationFactory> for XmlDocument {}
impl XmlDocument {
    #[inline] pub fn load_from_uri_async(uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_async(uri)
    }
    #[inline] pub fn load_from_uri_with_settings_async(uri: &ComPtr<foundation::Uri>, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_with_settings_async(uri, loadSettings)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_settings_async(file: &ComPtr<crate::windows::storage::IStorageFile>, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_with_settings_async(file, loadSettings)
    }
}
DEFINE_CLSID!(XmlDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,68,111,99,117,109,101,110,116,0]) [CLSID_XmlDocument]);
DEFINE_IID!(IID_IXmlDocumentFragment, 3807013526, 3105, 17573, 139, 201, 158, 74, 38, 39, 8, 236);
RT_INTERFACE!{interface IXmlDocumentFragment(IXmlDocumentFragmentVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentFragment] {
    
}}
RT_CLASS!{class XmlDocumentFragment: IXmlDocumentFragment}
DEFINE_IID!(IID_IXmlDocumentIO, 1825630030, 61029, 17545, 158, 191, 202, 67, 232, 123, 166, 55);
RT_INTERFACE!{interface IXmlDocumentIO(IXmlDocumentIOVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO] {
    fn LoadXml(&self, xml: HSTRING) -> HRESULT,
    fn LoadXmlWithSettings(&self, xml: HSTRING, loadSettings: *mut XmlLoadSettings) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveToFileAsync(&self, file: *mut crate::windows::storage::IStorageFile, out: *mut *mut foundation::IAsyncAction) -> HRESULT
}}
impl ComPtr<IXmlDocumentIO> {
    #[inline] pub fn load_xml(&self, xml: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).LoadXml)(self.as_abi() as *const _ as *mut _, xml.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn load_xml_with_settings(&self, xml: &HStringArg, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).LoadXmlWithSettings)(self.as_abi() as *const _ as *mut _, xml.get(), loadSettings.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_to_file_async(&self, file: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncAction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SaveToFileAsync)(self.as_abi() as *const _ as *mut _, file.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentIO2, 1560495713, 31704, 19157, 158, 191, 129, 230, 52, 114, 99, 177);
RT_INTERFACE!{interface IXmlDocumentIO2(IXmlDocumentIO2Vtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO2] {
    #[cfg(feature="windows-storage")] fn LoadXmlFromBuffer(&self, buffer: *mut crate::windows::storage::streams::IBuffer) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadXmlFromBufferWithSettings(&self, buffer: *mut crate::windows::storage::streams::IBuffer, loadSettings: *mut XmlLoadSettings) -> HRESULT
}}
impl ComPtr<IXmlDocumentIO2> {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_xml_from_buffer(&self, buffer: &ComPtr<crate::windows::storage::streams::IBuffer>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).LoadXmlFromBuffer)(self.as_abi() as *const _ as *mut _, buffer.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_xml_from_buffer_with_settings(&self, buffer: &ComPtr<crate::windows::storage::streams::IBuffer>, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).LoadXmlFromBufferWithSettings)(self.as_abi() as *const _ as *mut _, buffer.as_abi() as *const _ as *mut _, loadSettings.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentStatics, 1430508116, 55127, 19321, 149, 57, 35, 43, 24, 245, 11, 241);
RT_INTERFACE!{static interface IXmlDocumentStatics(IXmlDocumentStaticsVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentStatics] {
    fn LoadFromUriAsync(&self, uri: *mut foundation::Uri, out: *mut *mut foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    fn LoadFromUriWithSettingsAsync(&self, uri: *mut foundation::Uri, loadSettings: *mut XmlLoadSettings, out: *mut *mut foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: *mut crate::windows::storage::IStorageFile, out: *mut *mut foundation::IAsyncOperation<XmlDocument>) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithSettingsAsync(&self, file: *mut crate::windows::storage::IStorageFile, loadSettings: *mut XmlLoadSettings, out: *mut *mut foundation::IAsyncOperation<XmlDocument>) -> HRESULT
}}
impl ComPtr<IXmlDocumentStatics> {
    #[inline] pub fn load_from_uri_async(&self, uri: &ComPtr<foundation::Uri>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromUriAsync)(self.as_abi() as *const _ as *mut _, uri.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_uri_with_settings_async(&self, uri: &ComPtr<foundation::Uri>, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromUriWithSettingsAsync)(self.as_abi() as *const _ as *mut _, uri.as_abi() as *const _ as *mut _, loadSettings.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(&self, file: &ComPtr<crate::windows::storage::IStorageFile>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFileAsync)(self.as_abi() as *const _ as *mut _, file.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_settings_async(&self, file: &ComPtr<crate::windows::storage::IStorageFile>, loadSettings: &ComPtr<XmlLoadSettings>) -> Result<ComPtr<foundation::IAsyncOperation<XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).LoadFromFileWithSettingsAsync)(self.as_abi() as *const _ as *mut _, file.as_abi() as *const _ as *mut _, loadSettings.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentType, 4147389477, 38785, 18788, 142, 148, 155, 28, 109, 252, 155, 199);
RT_INTERFACE!{interface IXmlDocumentType(IXmlDocumentTypeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDocumentType] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Entities(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT,
    fn get_Notations(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT
}}
impl ComPtr<IXmlDocumentType> {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Name)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_entities(&self) -> Result<Option<ComPtr<XmlNamedNodeMap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Entities)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_notations(&self) -> Result<Option<ComPtr<XmlNamedNodeMap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Notations)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDocumentType: IXmlDocumentType}
DEFINE_IID!(IID_IXmlDomImplementation, 1843757362, 61725, 20411, 140, 198, 88, 60, 186, 147, 17, 47);
RT_INTERFACE!{interface IXmlDomImplementation(IXmlDomImplementationVtbl): IInspectable(IInspectableVtbl) [IID_IXmlDomImplementation] {
    fn HasFeature(&self, feature: HSTRING, version: *mut IInspectable, out: *mut bool) -> HRESULT
}}
impl ComPtr<IXmlDomImplementation> {
    #[inline] pub fn has_feature(&self, feature: &HStringArg, version: &ComPtr<IInspectable>) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).HasFeature)(self.as_abi() as *const _ as *mut _, feature.get(), version.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDomImplementation: IXmlDomImplementation}
DEFINE_IID!(IID_IXmlElement, 771459615, 27408, 20216, 159, 131, 239, 204, 232, 250, 236, 55);
RT_INTERFACE!{interface IXmlElement(IXmlElementVtbl): IInspectable(IInspectableVtbl) [IID_IXmlElement] {
    fn get_TagName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetAttribute(&self, attributeName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn SetAttribute(&self, attributeName: HSTRING, attributeValue: HSTRING) -> HRESULT,
    fn RemoveAttribute(&self, attributeName: HSTRING) -> HRESULT,
    fn GetAttributeNode(&self, attributeName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT,
    fn SetAttributeNode(&self, newAttribute: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn RemoveAttributeNode(&self, attributeNode: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn SetAttributeNS(&self, namespaceUri: *mut IInspectable, qualifiedName: HSTRING, value: HSTRING) -> HRESULT,
    fn GetAttributeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn RemoveAttributeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING) -> HRESULT,
    fn SetAttributeNodeNS(&self, newAttribute: *mut XmlAttribute, out: *mut *mut XmlAttribute) -> HRESULT,
    fn GetAttributeNodeNS(&self, namespaceUri: *mut IInspectable, localName: HSTRING, out: *mut *mut XmlAttribute) -> HRESULT
}}
impl ComPtr<IXmlElement> {
    #[inline] pub fn get_tag_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_TagName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute(&self, attributeName: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAttribute)(self.as_abi() as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute(&self, attributeName: &HStringArg, attributeValue: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetAttribute)(self.as_abi() as *const _ as *mut _, attributeName.get(), attributeValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute(&self, attributeName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).RemoveAttribute)(self.as_abi() as *const _ as *mut _, attributeName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_node(&self, attributeName: &HStringArg) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAttributeNode)(self.as_abi() as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_node(&self, newAttribute: &ComPtr<XmlAttribute>) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SetAttributeNode)(self.as_abi() as *const _ as *mut _, newAttribute.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute_node(&self, attributeNode: &ComPtr<XmlAttribute>) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RemoveAttributeNode)(self.as_abi() as *const _ as *mut _, attributeNode.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<Option<ComPtr<XmlNodeList>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetElementsByTagName)(self.as_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_ns(&self, namespaceUri: &ComPtr<IInspectable>, qualifiedName: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).SetAttributeNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, qualifiedName.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_ns(&self, namespaceUri: &ComPtr<IInspectable>, localName: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAttributeNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute_ns(&self, namespaceUri: &ComPtr<IInspectable>, localName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).RemoveAttributeNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, localName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_node_ns(&self, newAttribute: &ComPtr<XmlAttribute>) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SetAttributeNodeNS)(self.as_abi() as *const _ as *mut _, newAttribute.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_node_ns(&self, namespaceUri: &ComPtr<IInspectable>, localName: &HStringArg) -> Result<Option<ComPtr<XmlAttribute>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetAttributeNodeNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlElement: IXmlElement}
DEFINE_IID!(IID_IXmlEntityReference, 774850492, 50128, 19663, 187, 134, 10, 184, 195, 106, 97, 207);
RT_INTERFACE!{interface IXmlEntityReference(IXmlEntityReferenceVtbl): IInspectable(IInspectableVtbl) [IID_IXmlEntityReference] {
    
}}
RT_CLASS!{class XmlEntityReference: IXmlEntityReference}
DEFINE_IID!(IID_IXmlLoadSettings, 1487538088, 65238, 18167, 180, 197, 251, 27, 167, 33, 8, 214);
RT_INTERFACE!{interface IXmlLoadSettings(IXmlLoadSettingsVtbl): IInspectable(IInspectableVtbl) [IID_IXmlLoadSettings] {
    fn get_MaxElementDepth(&self, out: *mut u32) -> HRESULT,
    fn put_MaxElementDepth(&self, value: u32) -> HRESULT,
    fn get_ProhibitDtd(&self, out: *mut bool) -> HRESULT,
    fn put_ProhibitDtd(&self, value: bool) -> HRESULT,
    fn get_ResolveExternals(&self, out: *mut bool) -> HRESULT,
    fn put_ResolveExternals(&self, value: bool) -> HRESULT,
    fn get_ValidateOnParse(&self, out: *mut bool) -> HRESULT,
    fn put_ValidateOnParse(&self, value: bool) -> HRESULT,
    fn get_ElementContentWhiteSpace(&self, out: *mut bool) -> HRESULT,
    fn put_ElementContentWhiteSpace(&self, value: bool) -> HRESULT
}}
impl ComPtr<IXmlLoadSettings> {
    #[inline] pub fn get_max_element_depth(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_MaxElementDepth)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_element_depth(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_MaxElementDepth)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_prohibit_dtd(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ProhibitDtd)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_prohibit_dtd(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ProhibitDtd)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolve_externals(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ResolveExternals)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_resolve_externals(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ResolveExternals)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_validate_on_parse(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ValidateOnParse)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_validate_on_parse(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ValidateOnParse)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_element_content_white_space(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_ElementContentWhiteSpace)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_element_content_white_space(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_ElementContentWhiteSpace)(self.as_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlLoadSettings: IXmlLoadSettings}
impl RtActivatable<IActivationFactory> for XmlLoadSettings {}
DEFINE_CLSID!(XmlLoadSettings(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,76,111,97,100,83,101,116,116,105,110,103,115,0]) [CLSID_XmlLoadSettings]);
DEFINE_IID!(IID_IXmlNamedNodeMap, 3014041264, 43696, 19330, 166, 250, 177, 69, 63, 124, 2, 27);
RT_INTERFACE!{interface IXmlNamedNodeMap(IXmlNamedNodeMapVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNamedNodeMap] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut *mut IXmlNode) -> HRESULT,
    fn GetNamedItem(&self, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SetNamedItem(&self, node: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveNamedItem(&self, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn GetNamedItemNS(&self, namespaceUri: *mut IInspectable, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveNamedItemNS(&self, namespaceUri: *mut IInspectable, name: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SetNamedItemNS(&self, node: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT
}}
impl ComPtr<IXmlNamedNodeMap> {
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Length)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn item(&self, index: u32) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Item)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_item(&self, name: &HStringArg) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedItem)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_item(&self, node: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SetNamedItem)(self.as_abi() as *const _ as *mut _, node.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_named_item(&self, name: &HStringArg) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RemoveNamedItem)(self.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_item_ns(&self, namespaceUri: &ComPtr<IInspectable>, name: &HStringArg) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetNamedItemNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_named_item_ns(&self, namespaceUri: &ComPtr<IInspectable>, name: &HStringArg) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RemoveNamedItemNS)(self.as_abi() as *const _ as *mut _, namespaceUri.as_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_item_ns(&self, node: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SetNamedItemNS)(self.as_abi() as *const _ as *mut _, node.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlNamedNodeMap: IXmlNamedNodeMap}
DEFINE_IID!(IID_IXmlNode, 477371737, 8482, 18389, 168, 86, 131, 243, 212, 33, 72, 117);
RT_INTERFACE!{interface IXmlNode(IXmlNodeVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNode] {
    fn get_NodeValue(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn put_NodeValue(&self, value: *mut IInspectable) -> HRESULT,
    fn get_NodeType(&self, out: *mut NodeType) -> HRESULT,
    fn get_NodeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ParentNode(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_ChildNodes(&self, out: *mut *mut XmlNodeList) -> HRESULT,
    fn get_FirstChild(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_LastChild(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_PreviousSibling(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_NextSibling(&self, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_Attributes(&self, out: *mut *mut XmlNamedNodeMap) -> HRESULT,
    fn HasChildNodes(&self, out: *mut bool) -> HRESULT,
    fn get_OwnerDocument(&self, out: *mut *mut XmlDocument) -> HRESULT,
    fn InsertBefore(&self, newChild: *mut IXmlNode, referenceChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn ReplaceChild(&self, newChild: *mut IXmlNode, referenceChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn RemoveChild(&self, childNode: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn AppendChild(&self, newChild: *mut IXmlNode, out: *mut *mut IXmlNode) -> HRESULT,
    fn CloneNode(&self, deep: bool, out: *mut *mut IXmlNode) -> HRESULT,
    fn get_NamespaceUri(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_LocalName(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn get_Prefix(&self, out: *mut *mut IInspectable) -> HRESULT,
    fn Normalize(&self) -> HRESULT,
    fn put_Prefix(&self, value: *mut IInspectable) -> HRESULT
}}
impl ComPtr<IXmlNode> {
    #[inline] pub fn get_node_value(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NodeValue)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_node_value(&self, value: &ComPtr<IInspectable>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_NodeValue)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_node_type(&self) -> Result<NodeType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_NodeType)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_node_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NodeName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parent_node(&self) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ParentNode)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_child_nodes(&self) -> Result<Option<ComPtr<XmlNodeList>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_ChildNodes)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_child(&self) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_FirstChild)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_child(&self) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LastChild)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_previous_sibling(&self) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_PreviousSibling)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_sibling(&self) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NextSibling)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attributes(&self) -> Result<Option<ComPtr<XmlNamedNodeMap>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Attributes)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn has_child_nodes(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).HasChildNodes)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_owner_document(&self) -> Result<Option<ComPtr<XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_OwnerDocument)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn insert_before(&self, newChild: &ComPtr<IXmlNode>, referenceChild: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).InsertBefore)(self.as_abi() as *const _ as *mut _, newChild.as_abi() as *const _ as *mut _, referenceChild.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn replace_child(&self, newChild: &ComPtr<IXmlNode>, referenceChild: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).ReplaceChild)(self.as_abi() as *const _ as *mut _, newChild.as_abi() as *const _ as *mut _, referenceChild.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_child(&self, childNode: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).RemoveChild)(self.as_abi() as *const _ as *mut _, childNode.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_child(&self, newChild: &ComPtr<IXmlNode>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).AppendChild)(self.as_abi() as *const _ as *mut _, newChild.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn clone_node(&self, deep: bool) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CloneNode)(self.as_abi() as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_namespace_uri(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_NamespaceUri)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_name(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_LocalName)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_prefix(&self) -> Result<Option<ComPtr<IInspectable>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Prefix)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn normalize(&self) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).Normalize)(self.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_prefix(&self, value: &ComPtr<IInspectable>) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Prefix)(self.as_abi() as *const _ as *mut _, value.as_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlNodeList, 2355146103, 33700, 20161, 156, 84, 123, 164, 41, 225, 61, 166);
RT_INTERFACE!{interface IXmlNodeList(IXmlNodeListVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeList] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut *mut IXmlNode) -> HRESULT
}}
impl ComPtr<IXmlNodeList> {
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.as_abi().lpVtbl).get_Length)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn item(&self, index: u32) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).Item)(self.as_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlNodeList: IXmlNodeList}
DEFINE_IID!(IID_IXmlNodeSelector, 1675344523, 53467, 20449, 183, 69, 249, 67, 58, 253, 194, 91);
RT_INTERFACE!{interface IXmlNodeSelector(IXmlNodeSelectorVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeSelector] {
    fn SelectSingleNode(&self, xpath: HSTRING, out: *mut *mut IXmlNode) -> HRESULT,
    fn SelectNodes(&self, xpath: HSTRING, out: *mut *mut XmlNodeList) -> HRESULT,
    fn SelectSingleNodeNS(&self, xpath: HSTRING, namespaces: *mut IInspectable, out: *mut *mut IXmlNode) -> HRESULT,
    fn SelectNodesNS(&self, xpath: HSTRING, namespaces: *mut IInspectable, out: *mut *mut XmlNodeList) -> HRESULT
}}
impl ComPtr<IXmlNodeSelector> {
    #[inline] pub fn select_single_node(&self, xpath: &HStringArg) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SelectSingleNode)(self.as_abi() as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_nodes(&self, xpath: &HStringArg) -> Result<Option<ComPtr<XmlNodeList>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SelectNodes)(self.as_abi() as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_single_node_ns(&self, xpath: &HStringArg, namespaces: &ComPtr<IInspectable>) -> Result<Option<ComPtr<IXmlNode>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SelectSingleNodeNS)(self.as_abi() as *const _ as *mut _, xpath.get(), namespaces.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_nodes_ns(&self, xpath: &HStringArg, namespaces: &ComPtr<IInspectable>) -> Result<Option<ComPtr<XmlNodeList>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SelectNodesNS)(self.as_abi() as *const _ as *mut _, xpath.get(), namespaces.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlNodeSerializer, 1556460418, 59101, 18833, 171, 239, 6, 216, 210, 231, 189, 12);
RT_INTERFACE!{interface IXmlNodeSerializer(IXmlNodeSerializerVtbl): IInspectable(IInspectableVtbl) [IID_IXmlNodeSerializer] {
    fn GetXml(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InnerText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_InnerText(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IXmlNodeSerializer> {
    #[inline] pub fn get_xml(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).GetXml)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inner_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_InnerText)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_inner_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_InnerText)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlProcessingInstruction, 654834974, 7826, 20174, 182, 244, 38, 240, 105, 7, 141, 220);
RT_INTERFACE!{interface IXmlProcessingInstruction(IXmlProcessingInstructionVtbl): IInspectable(IInspectableVtbl) [IID_IXmlProcessingInstruction] {
    fn get_Target(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT
}}
impl ComPtr<IXmlProcessingInstruction> {
    #[inline] pub fn get_target(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Target)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).get_Data)(self.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.as_abi().lpVtbl).put_Data)(self.as_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlProcessingInstruction: IXmlProcessingInstruction}
DEFINE_IID!(IID_IXmlText, 4180780235, 12429, 18272, 161, 213, 67, 182, 116, 80, 172, 126);
RT_INTERFACE!{interface IXmlText(IXmlTextVtbl): IInspectable(IInspectableVtbl) [IID_IXmlText] {
    fn SplitText(&self, offset: u32, out: *mut *mut IXmlText) -> HRESULT
}}
impl ComPtr<IXmlText> {
    #[inline] pub fn split_text(&self, offset: u32) -> Result<Option<ComPtr<IXmlText>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).SplitText)(self.as_abi() as *const _ as *mut _, offset, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlText: IXmlText}
} // Windows.Data.Xml.Dom
pub mod xsl { // Windows.Data.Xml.Xsl
use crate::prelude::*;
DEFINE_IID!(IID_IXsltProcessor, 2070179903, 21772, 18630, 169, 15, 147, 165, 185, 100, 81, 143);
RT_INTERFACE!{interface IXsltProcessor(IXsltProcessorVtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessor] {
    fn TransformToString(&self, inputNode: *mut super::dom::IXmlNode, out: *mut HSTRING) -> HRESULT
}}
impl ComPtr<IXsltProcessor> {
    #[inline] pub fn transform_to_string(&self, inputNode: &ComPtr<super::dom::IXmlNode>) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TransformToString)(self.as_abi() as *const _ as *mut _, inputNode.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XsltProcessor: IXsltProcessor}
impl RtActivatable<IXsltProcessorFactory> for XsltProcessor {}
impl XsltProcessor {
    #[inline] pub fn create_instance(document: &ComPtr<super::dom::XmlDocument>) -> Result<ComPtr<XsltProcessor>> {
        <Self as RtActivatable<IXsltProcessorFactory>>::get_activation_factory().create_instance(document)
    }
}
DEFINE_CLSID!(XsltProcessor(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,88,115,108,46,88,115,108,116,80,114,111,99,101,115,115,111,114,0]) [CLSID_XsltProcessor]);
DEFINE_IID!(IID_IXsltProcessor2, 2376358998, 38821, 17611, 168, 190, 39, 216, 98, 128, 199, 10);
RT_INTERFACE!{interface IXsltProcessor2(IXsltProcessor2Vtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessor2] {
    fn TransformToDocument(&self, inputNode: *mut super::dom::IXmlNode, out: *mut *mut super::dom::XmlDocument) -> HRESULT
}}
impl ComPtr<IXsltProcessor2> {
    #[inline] pub fn transform_to_document(&self, inputNode: &ComPtr<super::dom::IXmlNode>) -> Result<Option<ComPtr<super::dom::XmlDocument>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).TransformToDocument)(self.as_abi() as *const _ as *mut _, inputNode.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap_optional(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXsltProcessorFactory, 658589376, 39505, 18019, 191, 48, 14, 247, 66, 20, 111, 32);
RT_INTERFACE!{static interface IXsltProcessorFactory(IXsltProcessorFactoryVtbl): IInspectable(IInspectableVtbl) [IID_IXsltProcessorFactory] {
    fn CreateInstance(&self, document: *mut super::dom::XmlDocument, out: *mut *mut XsltProcessor) -> HRESULT
}}
impl ComPtr<IXsltProcessorFactory> {
    #[inline] pub fn create_instance(&self, document: &ComPtr<super::dom::XmlDocument>) -> Result<ComPtr<XsltProcessor>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.as_abi().lpVtbl).CreateInstance)(self.as_abi() as *const _ as *mut _, document.as_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(ComPtr::wrap(out)) } else { err(hr) }
    }}
}
} // Windows.Data.Xml.Xsl
} // Windows.Data.Xml
