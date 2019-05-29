pub mod html { // Windows.Data.Html
use crate::prelude::*;
DEFINE_IID!(IID_IHtmlUtilities, 4273998557, 9113, 20396, 181, 167, 5, 233, 172, 215, 24, 29);
RT_INTERFACE!{static interface IHtmlUtilities(IHtmlUtilitiesVtbl, IHtmlUtilities_Abi): IInspectable(IInspectableVtbl) [IID_IHtmlUtilities] {
    fn ConvertToText(&self, html: HSTRING, out: *mut HSTRING) -> HRESULT
}}
impl IHtmlUtilities {
    #[inline] pub fn convert_to_text(&self, html: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ConvertToText)(self.get_abi() as *const _ as *mut _, html.get(), &mut out);
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
RT_INTERFACE!{interface IJsonArray(IJsonArrayVtbl, IJsonArray_Abi): IInspectable(IInspectableVtbl) [IID_IJsonArray] {
    fn GetObjectAt(&self, index: u32, out: *mut <JsonObject as RtType>::Abi) -> HRESULT,
    fn GetArrayAt(&self, index: u32, out: *mut <JsonArray as RtType>::Abi) -> HRESULT,
    fn GetStringAt(&self, index: u32, out: *mut HSTRING) -> HRESULT,
    fn GetNumberAt(&self, index: u32, out: *mut f64) -> HRESULT,
    fn GetBooleanAt(&self, index: u32, out: *mut bool) -> HRESULT
}}
impl IJsonArray {
    #[inline] pub fn get_object_at(&self, index: u32) -> Result<Option<JsonObject>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetObjectAt)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(JsonObject::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_array_at(&self, index: u32) -> Result<Option<JsonArray>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetArrayAt)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(JsonArray::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string_at(&self, index: u32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetStringAt)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_number_at(&self, index: u32) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNumberAt)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean_at(&self, index: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetBooleanAt)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonArray: IJsonArray}
impl RtActivatable<IJsonArrayStatics> for JsonArray {}
impl RtActivatable<IActivationFactory> for JsonArray {}
impl JsonArray {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<JsonArray>> {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<JsonArray>, bool)> {
        <Self as RtActivatable<IJsonArrayStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(JsonArray(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,65,114,114,97,121,0]) [CLSID_JsonArray]);
DEFINE_IID!(IID_IJsonArrayStatics, 3675534505, 57700, 18847, 147, 226, 138, 143, 73, 187, 144, 186);
RT_INTERFACE!{static interface IJsonArrayStatics(IJsonArrayStaticsVtbl, IJsonArrayStatics_Abi): IInspectable(IInspectableVtbl) [IID_IJsonArrayStatics] {
    fn Parse(&self, input: HSTRING, out: *mut <JsonArray as RtType>::Abi) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut <JsonArray as RtType>::Abi, out: *mut bool) -> HRESULT
}}
impl IJsonArrayStatics {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<JsonArray>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Parse)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(JsonArray::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<JsonArray>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).TryParse)(self.get_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((JsonArray::wrap(result), out)) } else { err(hr) }
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
RT_INTERFACE!{static interface IJsonErrorStatics2(IJsonErrorStatics2Vtbl, IJsonErrorStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IJsonErrorStatics2] {
    fn GetJsonStatus(&self, hresult: i32, out: *mut JsonErrorStatus) -> HRESULT
}}
impl IJsonErrorStatics2 {
    #[inline] pub fn get_json_status(&self, hresult: i32) -> Result<JsonErrorStatus> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetJsonStatus)(self.get_abi() as *const _ as *mut _, hresult, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_ENUM! { enum JsonErrorStatus: i32 {
    Unknown = 0, InvalidJsonString = 1, InvalidJsonNumber = 2, JsonValueNotFound = 3, ImplementationLimit = 4,
}}
DEFINE_IID!(IID_IJsonObject, 105784541, 10690, 20355, 154, 193, 158, 225, 21, 120, 190, 179);
RT_INTERFACE!{interface IJsonObject(IJsonObjectVtbl, IJsonObject_Abi): IInspectable(IInspectableVtbl) [IID_IJsonObject] {
    fn GetNamedValue(&self, name: HSTRING, out: *mut <JsonValue as RtType>::Abi) -> HRESULT,
    fn SetNamedValue(&self, name: HSTRING, value: <IJsonValue as RtType>::Abi) -> HRESULT,
    fn GetNamedObject(&self, name: HSTRING, out: *mut <JsonObject as RtType>::Abi) -> HRESULT,
    fn GetNamedArray(&self, name: HSTRING, out: *mut <JsonArray as RtType>::Abi) -> HRESULT,
    fn GetNamedString(&self, name: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedNumber(&self, name: HSTRING, out: *mut f64) -> HRESULT,
    fn GetNamedBoolean(&self, name: HSTRING, out: *mut bool) -> HRESULT
}}
impl IJsonObject {
    #[inline] pub fn get_named_value(&self, name: &HStringArg) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedValue)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_value(&self, name: &HStringArg, value: &IJsonValue) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetNamedValue)(self.get_abi() as *const _ as *mut _, name.get(), value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_named_object(&self, name: &HStringArg) -> Result<Option<JsonObject>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedObject)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(JsonObject::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_array(&self, name: &HStringArg) -> Result<Option<JsonArray>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedArray)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(JsonArray::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_string(&self, name: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedString)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_number(&self, name: &HStringArg) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNamedNumber)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_named_boolean(&self, name: &HStringArg) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNamedBoolean)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonObject: IJsonObject}
impl RtActivatable<IJsonObjectStatics> for JsonObject {}
impl RtActivatable<IActivationFactory> for JsonObject {}
impl JsonObject {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<JsonObject>> {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<JsonObject>, bool)> {
        <Self as RtActivatable<IJsonObjectStatics>>::get_activation_factory().try_parse(input)
    }
}
DEFINE_CLSID!(JsonObject(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,79,98,106,101,99,116,0]) [CLSID_JsonObject]);
DEFINE_IID!(IID_IJsonObjectStatics, 579465561, 21726, 17880, 171, 204, 34, 96, 63, 160, 102, 160);
RT_INTERFACE!{static interface IJsonObjectStatics(IJsonObjectStaticsVtbl, IJsonObjectStatics_Abi): IInspectable(IInspectableVtbl) [IID_IJsonObjectStatics] {
    fn Parse(&self, input: HSTRING, out: *mut <JsonObject as RtType>::Abi) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut <JsonObject as RtType>::Abi, out: *mut bool) -> HRESULT
}}
impl IJsonObjectStatics {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<JsonObject>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Parse)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(JsonObject::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<JsonObject>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).TryParse)(self.get_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((JsonObject::wrap(result), out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonObjectWithDefaultValues, 3647001250, 47088, 20224, 142, 68, 216, 44, 244, 21, 234, 19);
RT_INTERFACE!{interface IJsonObjectWithDefaultValues(IJsonObjectWithDefaultValuesVtbl, IJsonObjectWithDefaultValues_Abi): IInspectable(IInspectableVtbl) [IID_IJsonObjectWithDefaultValues] {
    fn GetNamedValueOrDefault(&self, name: HSTRING, defaultValue: <JsonValue as RtType>::Abi, out: *mut <JsonValue as RtType>::Abi) -> HRESULT,
    fn GetNamedObjectOrDefault(&self, name: HSTRING, defaultValue: <JsonObject as RtType>::Abi, out: *mut <JsonObject as RtType>::Abi) -> HRESULT,
    fn GetNamedStringOrDefault(&self, name: HSTRING, defaultValue: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn GetNamedArrayOrDefault(&self, name: HSTRING, defaultValue: <JsonArray as RtType>::Abi, out: *mut <JsonArray as RtType>::Abi) -> HRESULT,
    fn GetNamedNumberOrDefault(&self, name: HSTRING, defaultValue: f64, out: *mut f64) -> HRESULT,
    fn GetNamedBooleanOrDefault(&self, name: HSTRING, defaultValue: bool, out: *mut bool) -> HRESULT
}}
impl IJsonObjectWithDefaultValues {
    #[inline] pub fn get_named_value_or_default(&self, name: &HStringArg, defaultValue: &JsonValue) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedValueOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_object_or_default(&self, name: &HStringArg, defaultValue: &JsonObject) -> Result<Option<JsonObject>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedObjectOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonObject::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_string_or_default(&self, name: &HStringArg, defaultValue: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedStringOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_array_or_default(&self, name: &HStringArg, defaultValue: &JsonArray) -> Result<Option<JsonArray>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedArrayOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonArray::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_number_or_default(&self, name: &HStringArg, defaultValue: f64) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNamedNumberOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_named_boolean_or_default(&self, name: &HStringArg, defaultValue: bool) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNamedBooleanOrDefault)(self.get_abi() as *const _ as *mut _, name.get(), defaultValue, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonValue, 2736889547, 61619, 19917, 190, 238, 25, 212, 140, 211, 237, 30);
RT_INTERFACE!{interface IJsonValue(IJsonValueVtbl, IJsonValue_Abi): IInspectable(IInspectableVtbl) [IID_IJsonValue] {
    fn get_ValueType(&self, out: *mut JsonValueType) -> HRESULT,
    fn Stringify(&self, out: *mut HSTRING) -> HRESULT,
    fn GetString(&self, out: *mut HSTRING) -> HRESULT,
    fn GetNumber(&self, out: *mut f64) -> HRESULT,
    fn GetBoolean(&self, out: *mut bool) -> HRESULT,
    fn GetArray(&self, out: *mut <JsonArray as RtType>::Abi) -> HRESULT,
    fn GetObject(&self, out: *mut <JsonObject as RtType>::Abi) -> HRESULT
}}
impl IJsonValue {
    #[inline] pub fn get_value_type(&self) -> Result<JsonValueType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ValueType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn stringify(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Stringify)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_string(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetString)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_number(&self) -> Result<f64> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNumber)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_boolean(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetBoolean)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_array(&self) -> Result<Option<JsonArray>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetArray)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonArray::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_object(&self) -> Result<Option<JsonObject>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetObject)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonObject::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class JsonValue: IJsonValue}
impl RtActivatable<IJsonValueStatics> for JsonValue {}
impl RtActivatable<IJsonValueStatics2> for JsonValue {}
impl JsonValue {
    #[inline] pub fn parse(input: &HStringArg) -> Result<Option<JsonValue>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().parse(input)
    }
    #[inline] pub fn try_parse(input: &HStringArg) -> Result<(Option<JsonValue>, bool)> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().try_parse(input)
    }
    #[inline] pub fn create_boolean_value(input: bool) -> Result<Option<JsonValue>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_boolean_value(input)
    }
    #[inline] pub fn create_number_value(input: f64) -> Result<Option<JsonValue>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_number_value(input)
    }
    #[inline] pub fn create_string_value(input: &HStringArg) -> Result<Option<JsonValue>> {
        <Self as RtActivatable<IJsonValueStatics>>::get_activation_factory().create_string_value(input)
    }
    #[inline] pub fn create_null_value() -> Result<Option<JsonValue>> {
        <Self as RtActivatable<IJsonValueStatics2>>::get_activation_factory().create_null_value()
    }
}
DEFINE_CLSID!(JsonValue(&[87,105,110,100,111,119,115,46,68,97,116,97,46,74,115,111,110,46,74,115,111,110,86,97,108,117,101,0]) [CLSID_JsonValue]);
DEFINE_IID!(IID_IJsonValueStatics, 1600869450, 12115, 18657, 145, 163, 247, 139, 80, 166, 52, 92);
RT_INTERFACE!{static interface IJsonValueStatics(IJsonValueStaticsVtbl, IJsonValueStatics_Abi): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics] {
    fn Parse(&self, input: HSTRING, out: *mut <JsonValue as RtType>::Abi) -> HRESULT,
    fn TryParse(&self, input: HSTRING, result: *mut <JsonValue as RtType>::Abi, out: *mut bool) -> HRESULT,
    fn CreateBooleanValue(&self, input: bool, out: *mut <JsonValue as RtType>::Abi) -> HRESULT,
    fn CreateNumberValue(&self, input: f64, out: *mut <JsonValue as RtType>::Abi) -> HRESULT,
    fn CreateStringValue(&self, input: HSTRING, out: *mut <JsonValue as RtType>::Abi) -> HRESULT
}}
impl IJsonValueStatics {
    #[inline] pub fn parse(&self, input: &HStringArg) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Parse)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn try_parse(&self, input: &HStringArg) -> Result<(Option<JsonValue>, bool)> { unsafe { 
        let mut result = null_mut(); let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).TryParse)(self.get_abi() as *const _ as *mut _, input.get(), &mut result, &mut out);
        if hr == S_OK { Ok((JsonValue::wrap(result), out)) } else { err(hr) }
    }}
    #[inline] pub fn create_boolean_value(&self, input: bool) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateBooleanValue)(self.get_abi() as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_number_value(&self, input: f64) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateNumberValue)(self.get_abi() as *const _ as *mut _, input, &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_string_value(&self, input: &HStringArg) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateStringValue)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IJsonValueStatics2, 496946148, 16360, 17205, 131, 146, 147, 216, 227, 104, 101, 240);
RT_INTERFACE!{static interface IJsonValueStatics2(IJsonValueStatics2Vtbl, IJsonValueStatics2_Abi): IInspectable(IInspectableVtbl) [IID_IJsonValueStatics2] {
    fn CreateNullValue(&self, out: *mut <JsonValue as RtType>::Abi) -> HRESULT
}}
impl IJsonValueStatics2 {
    #[inline] pub fn create_null_value(&self) -> Result<Option<JsonValue>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateNullValue)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(JsonValue::wrap(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum JsonValueType: i32 {
    Null = 0, Boolean = 1, Number = 2, String = 3, Array = 4, Object = 5,
}}
} // Windows.Data.Json
pub mod pdf { // Windows.Data.Pdf
use crate::prelude::*;
DEFINE_IID!(IID_IPdfDocument, 2893987549, 33018, 16521, 132, 110, 129, 183, 127, 245, 168, 108);
RT_INTERFACE!{interface IPdfDocument(IPdfDocumentVtbl, IPdfDocument_Abi): IInspectable(IInspectableVtbl) [IID_IPdfDocument] {
    fn GetPage(&self, pageIndex: u32, out: *mut <PdfPage as RtType>::Abi) -> HRESULT,
    fn get_PageCount(&self, out: *mut u32) -> HRESULT,
    fn get_IsPasswordProtected(&self, out: *mut bool) -> HRESULT
}}
impl IPdfDocument {
    #[inline] pub fn get_page(&self, pageIndex: u32) -> Result<Option<PdfPage>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPage)(self.get_abi() as *const _ as *mut _, pageIndex, &mut out);
        if hr == S_OK { Ok(PdfPage::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_page_count(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PageCount)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_is_password_protected(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsPasswordProtected)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfDocument: IPdfDocument}
impl RtActivatable<IPdfDocumentStatics> for PdfDocument {}
impl PdfDocument {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<PdfDocument>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_password_async(file: &super::super::storage::IStorageFile, password: &HStringArg) -> Result<foundation::IAsyncOperation<PdfDocument>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_file_with_password_async(file, password)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(inputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<PdfDocument>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_async(inputStream)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_password_async(inputStream: &super::super::storage::streams::IRandomAccessStream, password: &HStringArg) -> Result<foundation::IAsyncOperation<PdfDocument>> {
        <Self as RtActivatable<IPdfDocumentStatics>>::get_activation_factory().load_from_stream_with_password_async(inputStream, password)
    }
}
DEFINE_CLSID!(PdfDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,80,100,102,46,80,100,102,68,111,99,117,109,101,110,116,0]) [CLSID_PdfDocument]);
DEFINE_IID!(IID_IPdfDocumentStatics, 1127877471, 49159, 18312, 144, 242, 8, 20, 61, 146, 37, 153);
RT_INTERFACE!{static interface IPdfDocumentStatics(IPdfDocumentStaticsVtbl, IPdfDocumentStatics_Abi): IInspectable(IInspectableVtbl) [IID_IPdfDocumentStatics] {
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: <super::super::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<PdfDocument> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithPasswordAsync(&self, file: <super::super::storage::IStorageFile as RtType>::Abi, password: HSTRING, out: *mut <foundation::IAsyncOperation<PdfDocument> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamAsync(&self, inputStream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncOperation<PdfDocument> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromStreamWithPasswordAsync(&self, inputStream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, password: HSTRING, out: *mut <foundation::IAsyncOperation<PdfDocument> as RtType>::Abi) -> HRESULT
}}
impl IPdfDocumentStatics {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(&self, file: &super::super::storage::IStorageFile) -> Result<foundation::IAsyncOperation<PdfDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromFileAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_password_async(&self, file: &super::super::storage::IStorageFile, password: &HStringArg) -> Result<foundation::IAsyncOperation<PdfDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromFileWithPasswordAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_async(&self, inputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncOperation<PdfDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromStreamAsync)(self.get_abi() as *const _ as *mut _, inputStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_stream_with_password_async(&self, inputStream: &super::super::storage::streams::IRandomAccessStream, password: &HStringArg) -> Result<foundation::IAsyncOperation<PdfDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromStreamWithPasswordAsync)(self.get_abi() as *const _ as *mut _, inputStream.get_abi() as *const _ as *mut _, password.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IPdfPage, 2645864648, 21280, 19708, 173, 118, 73, 63, 218, 208, 229, 148);
RT_INTERFACE!{interface IPdfPage(IPdfPageVtbl, IPdfPage_Abi): IInspectable(IInspectableVtbl) [IID_IPdfPage] {
    #[cfg(not(feature="windows-storage"))] fn __Dummy0(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderToStreamAsync(&self, outputStream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    #[cfg(not(feature="windows-storage"))] fn __Dummy1(&self) -> (),
    #[cfg(feature="windows-storage")] fn RenderWithOptionsToStreamAsync(&self, outputStream: <super::super::storage::streams::IRandomAccessStream as RtType>::Abi, options: <PdfPageRenderOptions as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn PreparePageAsync(&self, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT,
    fn get_Index(&self, out: *mut u32) -> HRESULT,
    fn get_Size(&self, out: *mut foundation::Size) -> HRESULT,
    fn get_Dimensions(&self, out: *mut <PdfPageDimensions as RtType>::Abi) -> HRESULT,
    fn get_Rotation(&self, out: *mut PdfPageRotation) -> HRESULT,
    fn get_PreferredZoom(&self, out: *mut f32) -> HRESULT
}}
impl IPdfPage {
    #[cfg(feature="windows-storage")] #[inline] pub fn render_to_stream_async(&self, outputStream: &super::super::storage::streams::IRandomAccessStream) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RenderToStreamAsync)(self.get_abi() as *const _ as *mut _, outputStream.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn render_with_options_to_stream_async(&self, outputStream: &super::super::storage::streams::IRandomAccessStream, options: &PdfPageRenderOptions) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RenderWithOptionsToStreamAsync)(self.get_abi() as *const _ as *mut _, outputStream.get_abi() as *const _ as *mut _, options.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn prepare_page_async(&self) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).PreparePageAsync)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_index(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Index)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_size(&self) -> Result<foundation::Size> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Size)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_dimensions(&self) -> Result<Option<PdfPageDimensions>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Dimensions)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(PdfPageDimensions::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_rotation(&self) -> Result<PdfPageRotation> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Rotation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_preferred_zoom(&self) -> Result<f32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_PreferredZoom)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfPage: IPdfPage}
DEFINE_IID!(IID_IPdfPageDimensions, 571933809, 12606, 17640, 131, 93, 99, 163, 231, 98, 74, 16);
RT_INTERFACE!{interface IPdfPageDimensions(IPdfPageDimensionsVtbl, IPdfPageDimensions_Abi): IInspectable(IInspectableVtbl) [IID_IPdfPageDimensions] {
    fn get_MediaBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_CropBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_BleedBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_TrimBox(&self, out: *mut foundation::Rect) -> HRESULT,
    fn get_ArtBox(&self, out: *mut foundation::Rect) -> HRESULT
}}
impl IPdfPageDimensions {
    #[inline] pub fn get_media_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MediaBox)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_crop_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_CropBox)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_bleed_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BleedBox)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_trim_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_TrimBox)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_art_box(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ArtBox)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class PdfPageDimensions: IPdfPageDimensions}
DEFINE_IID!(IID_IPdfPageRenderOptions, 1016595823, 47055, 19497, 154, 4, 82, 217, 2, 103, 244, 37);
RT_INTERFACE!{interface IPdfPageRenderOptions(IPdfPageRenderOptionsVtbl, IPdfPageRenderOptions_Abi): IInspectable(IInspectableVtbl) [IID_IPdfPageRenderOptions] {
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
impl IPdfPageRenderOptions {
    #[inline] pub fn get_source_rect(&self) -> Result<foundation::Rect> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SourceRect)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_source_rect(&self, value: foundation::Rect) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_SourceRect)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_destination_width(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DestinationWidth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_destination_width(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DestinationWidth)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_destination_height(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_DestinationHeight)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_destination_height(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_DestinationHeight)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_background_color(&self) -> Result<super::super::ui::Color> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BackgroundColor)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_background_color(&self, value: super::super::ui::Color) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_BackgroundColor)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_is_ignoring_high_contrast(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_IsIgnoringHighContrast)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_is_ignoring_high_contrast(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_IsIgnoringHighContrast)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_bitmap_encoder_id(&self) -> Result<Guid> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_BitmapEncoderId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_bitmap_encoder_id(&self, value: Guid) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_BitmapEncoderId)(self.get_abi() as *const _ as *mut _, value);
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
RT_INTERFACE!{interface IAlternateWordForm(IAlternateWordFormVtbl, IAlternateWordForm_Abi): IInspectable(IInspectableVtbl) [IID_IAlternateWordForm] {
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_NormalizationFormat(&self, out: *mut AlternateNormalizationFormat) -> HRESULT
}}
impl IAlternateWordForm {
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SourceTextSegment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_alternate_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AlternateText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_normalization_format(&self) -> Result<AlternateNormalizationFormat> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NormalizationFormat)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class AlternateWordForm: IAlternateWordForm}
DEFINE_IID!(IID_ISelectableWordSegment, 2439662775, 35495, 19576, 179, 116, 93, 237, 183, 82, 230, 11);
RT_INTERFACE!{interface ISelectableWordSegment(ISelectableWordSegmentVtbl, ISelectableWordSegment_Abi): IInspectable(IInspectableVtbl) [IID_ISelectableWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT
}}
impl ISelectableWordSegment {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Text)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SourceTextSegment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class SelectableWordSegment: ISelectableWordSegment}
DEFINE_IID!(IID_SelectableWordSegmentsTokenizingHandler, 977140892, 44766, 19911, 158, 108, 65, 192, 68, 189, 53, 146);
RT_DELEGATE!{delegate SelectableWordSegmentsTokenizingHandler(SelectableWordSegmentsTokenizingHandlerVtbl, SelectableWordSegmentsTokenizingHandler_Abi, SelectableWordSegmentsTokenizingHandlerImpl) [IID_SelectableWordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: <foundation::collections::IIterable<SelectableWordSegment> as RtType>::Abi, words: <foundation::collections::IIterable<SelectableWordSegment> as RtType>::Abi) -> HRESULT
}}
impl SelectableWordSegmentsTokenizingHandler {
    #[inline] pub fn invoke(&self, precedingWords: &foundation::collections::IIterable<SelectableWordSegment>, words: &foundation::collections::IIterable<SelectableWordSegment>) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, precedingWords.get_abi() as *const _ as *mut _, words.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISelectableWordsSegmenter, 4141625831, 19219, 17861, 136, 151, 125, 113, 38, 158, 8, 93);
RT_INTERFACE!{interface ISelectableWordsSegmenter(ISelectableWordsSegmenterVtbl, ISelectableWordsSegmenter_Abi): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut <SelectableWordSegment as RtType>::Abi) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut <foundation::collections::IVectorView<SelectableWordSegment> as RtType>::Abi) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: <SelectableWordSegmentsTokenizingHandler as RtType>::Abi) -> HRESULT
}}
impl ISelectableWordsSegmenter {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<Option<SelectableWordSegment>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTokenAt)(self.get_abi() as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(SelectableWordSegment::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tokens(&self, text: &HStringArg) -> Result<Option<foundation::collections::IVectorView<SelectableWordSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTokens)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &SelectableWordSegmentsTokenizingHandler) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Tokenize)(self.get_abi() as *const _ as *mut _, text.get(), startIndex, handler.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class SelectableWordsSegmenter: ISelectableWordsSegmenter}
impl RtActivatable<ISelectableWordsSegmenterFactory> for SelectableWordsSegmenter {}
impl SelectableWordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<SelectableWordsSegmenter> {
        <Self as RtActivatable<ISelectableWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }
}
DEFINE_CLSID!(SelectableWordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,108,101,99,116,97,98,108,101,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_SelectableWordsSegmenter]);
DEFINE_IID!(IID_ISelectableWordsSegmenterFactory, 2356835912, 24663, 17209, 188, 112, 242, 16, 1, 10, 65, 80);
RT_INTERFACE!{static interface ISelectableWordsSegmenterFactory(ISelectableWordsSegmenterFactoryVtbl, ISelectableWordsSegmenterFactory_Abi): IInspectable(IInspectableVtbl) [IID_ISelectableWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut <SelectableWordsSegmenter as RtType>::Abi) -> HRESULT
}}
impl ISelectableWordsSegmenterFactory {
    #[inline] pub fn create_with_language(&self, language: &HStringArg) -> Result<SelectableWordsSegmenter> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithLanguage)(self.get_abi() as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(SelectableWordsSegmenter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ISemanticTextQuery, 1780263761, 8114, 18697, 128, 184, 53, 115, 26, 43, 62, 127);
RT_INTERFACE!{interface ISemanticTextQuery(ISemanticTextQueryVtbl, ISemanticTextQuery_Abi): IInspectable(IInspectableVtbl) [IID_ISemanticTextQuery] {
    fn Find(&self, content: HSTRING, out: *mut <foundation::collections::IVectorView<TextSegment> as RtType>::Abi) -> HRESULT,
    fn FindInProperty(&self, propertyContent: HSTRING, propertyName: HSTRING, out: *mut <foundation::collections::IVectorView<TextSegment> as RtType>::Abi) -> HRESULT
}}
impl ISemanticTextQuery {
    #[inline] pub fn find(&self, content: &HStringArg) -> Result<Option<foundation::collections::IVectorView<TextSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Find)(self.get_abi() as *const _ as *mut _, content.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn find_in_property(&self, propertyContent: &HStringArg, propertyName: &HStringArg) -> Result<Option<foundation::collections::IVectorView<TextSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).FindInProperty)(self.get_abi() as *const _ as *mut _, propertyContent.get(), propertyName.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class SemanticTextQuery: ISemanticTextQuery}
impl RtActivatable<ISemanticTextQueryFactory> for SemanticTextQuery {}
impl SemanticTextQuery {
    #[inline] pub fn create(aqsFilter: &HStringArg) -> Result<SemanticTextQuery> {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create(aqsFilter)
    }
    #[inline] pub fn create_with_language(aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<SemanticTextQuery> {
        <Self as RtActivatable<ISemanticTextQueryFactory>>::get_activation_factory().create_with_language(aqsFilter, filterLanguage)
    }
}
DEFINE_CLSID!(SemanticTextQuery(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,83,101,109,97,110,116,105,99,84,101,120,116,81,117,101,114,121,0]) [CLSID_SemanticTextQuery]);
DEFINE_IID!(IID_ISemanticTextQueryFactory, 596378883, 63893, 17799, 135, 119, 162, 183, 216, 10, 207, 239);
RT_INTERFACE!{static interface ISemanticTextQueryFactory(ISemanticTextQueryFactoryVtbl, ISemanticTextQueryFactory_Abi): IInspectable(IInspectableVtbl) [IID_ISemanticTextQueryFactory] {
    fn Create(&self, aqsFilter: HSTRING, out: *mut <SemanticTextQuery as RtType>::Abi) -> HRESULT,
    fn CreateWithLanguage(&self, aqsFilter: HSTRING, filterLanguage: HSTRING, out: *mut <SemanticTextQuery as RtType>::Abi) -> HRESULT
}}
impl ISemanticTextQueryFactory {
    #[inline] pub fn create(&self, aqsFilter: &HStringArg) -> Result<SemanticTextQuery> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, aqsFilter.get(), &mut out);
        if hr == S_OK { Ok(SemanticTextQuery::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_with_language(&self, aqsFilter: &HStringArg, filterLanguage: &HStringArg) -> Result<SemanticTextQuery> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithLanguage)(self.get_abi() as *const _ as *mut _, aqsFilter.get(), filterLanguage.get(), &mut out);
        if hr == S_OK { Ok(SemanticTextQuery::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextConversionGenerator, 56650334, 10921, 19126, 175, 139, 165, 98, 182, 58, 137, 146);
RT_INTERFACE!{interface ITextConversionGenerator(ITextConversionGeneratorVtbl, ITextConversionGenerator_Abi): IInspectable(IInspectableVtbl) [IID_ITextConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT
}}
impl ITextConversionGenerator {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_async(&self, input: &HStringArg) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCandidatesAsync)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCandidatesWithMaxCountAsync)(self.get_abi() as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextConversionGenerator: ITextConversionGenerator}
impl RtActivatable<ITextConversionGeneratorFactory> for TextConversionGenerator {}
impl TextConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<TextConversionGenerator> {
        <Self as RtActivatable<ITextConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextConversionGenerator]);
DEFINE_IID!(IID_ITextConversionGeneratorFactory, 4239013761, 12419, 18859, 190, 21, 86, 223, 187, 183, 77, 111);
RT_INTERFACE!{static interface ITextConversionGeneratorFactory(ITextConversionGeneratorFactoryVtbl, ITextConversionGeneratorFactory_Abi): IInspectable(IInspectableVtbl) [IID_ITextConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut <TextConversionGenerator as RtType>::Abi) -> HRESULT
}}
impl ITextConversionGeneratorFactory {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<TextConversionGenerator> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(TextConversionGenerator::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextPhoneme, 2472715274, 39802, 17769, 148, 207, 216, 79, 47, 56, 207, 155);
RT_INTERFACE!{interface ITextPhoneme(ITextPhonemeVtbl, ITextPhoneme_Abi): IInspectable(IInspectableVtbl) [IID_ITextPhoneme] {
    fn get_DisplayText(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ReadingText(&self, out: *mut HSTRING) -> HRESULT
}}
impl ITextPhoneme {
    #[inline] pub fn get_display_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DisplayText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_reading_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ReadingText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextPhoneme: ITextPhoneme}
DEFINE_IID!(IID_ITextPredictionGenerator, 1588374279, 44017, 19638, 157, 158, 50, 111, 43, 70, 135, 86);
RT_INTERFACE!{interface ITextPredictionGenerator(ITextPredictionGeneratorVtbl, ITextPredictionGenerator_Abi): IInspectable(IInspectableVtbl) [IID_ITextPredictionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn GetCandidatesAsync(&self, input: HSTRING, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT,
    fn GetCandidatesWithMaxCountAsync(&self, input: HSTRING, maxCandidates: u32, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT
}}
impl ITextPredictionGenerator {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_async(&self, input: &HStringArg) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCandidatesAsync)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_candidates_with_max_count_async(&self, input: &HStringArg, maxCandidates: u32) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCandidatesWithMaxCountAsync)(self.get_abi() as *const _ as *mut _, input.get(), maxCandidates, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextPredictionGenerator: ITextPredictionGenerator}
impl RtActivatable<ITextPredictionGeneratorFactory> for TextPredictionGenerator {}
impl TextPredictionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<TextPredictionGenerator> {
        <Self as RtActivatable<ITextPredictionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextPredictionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,80,114,101,100,105,99,116,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextPredictionGenerator]);
DEFINE_IID!(IID_ITextPredictionGenerator2, 3091669944, 11383, 18538, 144, 10, 163, 69, 62, 237, 193, 93);
RT_INTERFACE!{interface ITextPredictionGenerator2(ITextPredictionGenerator2Vtbl, ITextPredictionGenerator2_Abi): IInspectable(IInspectableVtbl) [IID_ITextPredictionGenerator2] {
    fn GetCandidatesWithParametersAsync(&self, input: HSTRING, maxCandidates: u32, predictionOptions: TextPredictionOptions, previousStrings: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT,
    fn GetNextWordCandidatesAsync(&self, maxCandidates: u32, previousStrings: <foundation::collections::IIterable<HString> as RtType>::Abi, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<HString>> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-ui")] fn get_InputScope(&self, out: *mut super::super::ui::text::core::CoreTextInputScope) -> HRESULT,
    #[cfg(feature="windows-ui")] fn put_InputScope(&self, value: super::super::ui::text::core::CoreTextInputScope) -> HRESULT
}}
impl ITextPredictionGenerator2 {
    #[inline] pub fn get_candidates_with_parameters_async(&self, input: &HStringArg, maxCandidates: u32, predictionOptions: TextPredictionOptions, previousStrings: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetCandidatesWithParametersAsync)(self.get_abi() as *const _ as *mut _, input.get(), maxCandidates, predictionOptions, previousStrings.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_word_candidates_async(&self, maxCandidates: u32, previousStrings: &foundation::collections::IIterable<HString>) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<HString>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNextWordCandidatesAsync)(self.get_abi() as *const _ as *mut _, maxCandidates, previousStrings.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn get_input_scope(&self) -> Result<super::super::ui::text::core::CoreTextInputScope> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_InputScope)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[cfg(feature="windows-ui")] #[inline] pub fn set_input_scope(&self, value: super::super::ui::text::core::CoreTextInputScope) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_InputScope)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextPredictionGeneratorFactory, 1918350358, 35746, 18257, 157, 48, 157, 133, 67, 86, 83, 162);
RT_INTERFACE!{static interface ITextPredictionGeneratorFactory(ITextPredictionGeneratorFactoryVtbl, ITextPredictionGeneratorFactory_Abi): IInspectable(IInspectableVtbl) [IID_ITextPredictionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut <TextPredictionGenerator as RtType>::Abi) -> HRESULT
}}
impl ITextPredictionGeneratorFactory {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<TextPredictionGenerator> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(TextPredictionGenerator::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_ENUM! { enum TextPredictionOptions: u32 {
    None = 0, Predictions = 1, Corrections = 2,
}}
DEFINE_IID!(IID_ITextReverseConversionGenerator, 1374156052, 40017, 19846, 174, 27, 180, 152, 251, 173, 131, 19);
RT_INTERFACE!{interface ITextReverseConversionGenerator(ITextReverseConversionGeneratorVtbl, ITextReverseConversionGenerator_Abi): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn get_LanguageAvailableButNotInstalled(&self, out: *mut bool) -> HRESULT,
    fn ConvertBackAsync(&self, input: HSTRING, out: *mut <foundation::IAsyncOperation<HString> as RtType>::Abi) -> HRESULT
}}
impl ITextReverseConversionGenerator {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_language_available_but_not_installed(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_LanguageAvailableButNotInstalled)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn convert_back_async(&self, input: &HStringArg) -> Result<foundation::IAsyncOperation<HString>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ConvertBackAsync)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class TextReverseConversionGenerator: ITextReverseConversionGenerator}
impl RtActivatable<ITextReverseConversionGeneratorFactory> for TextReverseConversionGenerator {}
impl TextReverseConversionGenerator {
    #[inline] pub fn create(languageTag: &HStringArg) -> Result<TextReverseConversionGenerator> {
        <Self as RtActivatable<ITextReverseConversionGeneratorFactory>>::get_activation_factory().create(languageTag)
    }
}
DEFINE_CLSID!(TextReverseConversionGenerator(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,84,101,120,116,82,101,118,101,114,115,101,67,111,110,118,101,114,115,105,111,110,71,101,110,101,114,97,116,111,114,0]) [CLSID_TextReverseConversionGenerator]);
DEFINE_IID!(IID_ITextReverseConversionGenerator2, 447730412, 34262, 18173, 130, 138, 58, 72, 48, 250, 110, 24);
RT_INTERFACE!{interface ITextReverseConversionGenerator2(ITextReverseConversionGenerator2Vtbl, ITextReverseConversionGenerator2_Abi): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGenerator2] {
    fn GetPhonemesAsync(&self, input: HSTRING, out: *mut <foundation::IAsyncOperation<foundation::collections::IVectorView<TextPhoneme>> as RtType>::Abi) -> HRESULT
}}
impl ITextReverseConversionGenerator2 {
    #[inline] pub fn get_phonemes_async(&self, input: &HStringArg) -> Result<foundation::IAsyncOperation<foundation::collections::IVectorView<TextPhoneme>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetPhonemesAsync)(self.get_abi() as *const _ as *mut _, input.get(), &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_ITextReverseConversionGeneratorFactory, 1673450278, 8154, 16886, 137, 213, 35, 221, 234, 60, 114, 154);
RT_INTERFACE!{static interface ITextReverseConversionGeneratorFactory(ITextReverseConversionGeneratorFactoryVtbl, ITextReverseConversionGeneratorFactory_Abi): IInspectable(IInspectableVtbl) [IID_ITextReverseConversionGeneratorFactory] {
    fn Create(&self, languageTag: HSTRING, out: *mut <TextReverseConversionGenerator as RtType>::Abi) -> HRESULT
}}
impl ITextReverseConversionGeneratorFactory {
    #[inline] pub fn create(&self, languageTag: &HStringArg) -> Result<TextReverseConversionGenerator> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Create)(self.get_abi() as *const _ as *mut _, languageTag.get(), &mut out);
        if hr == S_OK { Ok(TextReverseConversionGenerator::wrap_nonnull(out)) } else { err(hr) }
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
RT_INTERFACE!{static interface IUnicodeCharactersStatics(IUnicodeCharactersStaticsVtbl, IUnicodeCharactersStatics_Abi): IInspectable(IInspectableVtbl) [IID_IUnicodeCharactersStatics] {
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
impl IUnicodeCharactersStatics {
    #[inline] pub fn get_codepoint_from_surrogate_pair(&self, highSurrogate: u32, lowSurrogate: u32) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetCodepointFromSurrogatePair)(self.get_abi() as *const _ as *mut _, highSurrogate, lowSurrogate, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_surrogate_pair_from_codepoint(&self, codepoint: u32) -> Result<(Char, Char)> { unsafe { 
        let mut highSurrogate = zeroed(); let mut lowSurrogate = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetSurrogatePairFromCodepoint)(self.get_abi() as *const _ as *mut _, codepoint, &mut highSurrogate, &mut lowSurrogate);
        if hr == S_OK { Ok((highSurrogate, lowSurrogate)) } else { err(hr) }
    }}
    #[inline] pub fn is_high_surrogate(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsHighSurrogate)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_low_surrogate(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsLowSurrogate)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_supplementary(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsSupplementary)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_noncharacter(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsNoncharacter)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_whitespace(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsWhitespace)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_alphabetic(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsAlphabetic)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_cased(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsCased)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_uppercase(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsUppercase)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_lowercase(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsLowercase)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_id_start(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsIdStart)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_id_continue(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsIdContinue)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_grapheme_base(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsGraphemeBase)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn is_grapheme_extend(&self, codepoint: u32) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).IsGraphemeExtend)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_numeric_type(&self, codepoint: u32) -> Result<UnicodeNumericType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetNumericType)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_general_category(&self, codepoint: u32) -> Result<UnicodeGeneralCategory> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).GetGeneralCategory)(self.get_abi() as *const _ as *mut _, codepoint, &mut out);
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
RT_INTERFACE!{interface IWordSegment(IWordSegmentVtbl, IWordSegment_Abi): IInspectable(IInspectableVtbl) [IID_IWordSegment] {
    fn get_Text(&self, out: *mut HSTRING) -> HRESULT,
    fn get_SourceTextSegment(&self, out: *mut TextSegment) -> HRESULT,
    fn get_AlternateForms(&self, out: *mut <foundation::collections::IVectorView<AlternateWordForm> as RtType>::Abi) -> HRESULT
}}
impl IWordSegment {
    #[inline] pub fn get_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Text)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_source_text_segment(&self) -> Result<TextSegment> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_SourceTextSegment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_alternate_forms(&self) -> Result<Option<foundation::collections::IVectorView<AlternateWordForm>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_AlternateForms)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class WordSegment: IWordSegment}
DEFINE_IID!(IID_WordSegmentsTokenizingHandler, 2782749527, 48938, 19535, 163, 31, 41, 231, 28, 111, 139, 53);
RT_DELEGATE!{delegate WordSegmentsTokenizingHandler(WordSegmentsTokenizingHandlerVtbl, WordSegmentsTokenizingHandler_Abi, WordSegmentsTokenizingHandlerImpl) [IID_WordSegmentsTokenizingHandler] {
    fn Invoke(&self, precedingWords: <foundation::collections::IIterable<WordSegment> as RtType>::Abi, words: <foundation::collections::IIterable<WordSegment> as RtType>::Abi) -> HRESULT
}}
impl WordSegmentsTokenizingHandler {
    #[inline] pub fn invoke(&self, precedingWords: &foundation::collections::IIterable<WordSegment>, words: &foundation::collections::IIterable<WordSegment>) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Invoke)(self.get_abi() as *const _ as *mut _, precedingWords.get_abi() as *const _ as *mut _, words.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IWordsSegmenter, 2259997905, 45822, 20020, 168, 29, 102, 100, 3, 0, 69, 79);
RT_INTERFACE!{interface IWordsSegmenter(IWordsSegmenterVtbl, IWordsSegmenter_Abi): IInspectable(IInspectableVtbl) [IID_IWordsSegmenter] {
    fn get_ResolvedLanguage(&self, out: *mut HSTRING) -> HRESULT,
    fn GetTokenAt(&self, text: HSTRING, startIndex: u32, out: *mut <WordSegment as RtType>::Abi) -> HRESULT,
    fn GetTokens(&self, text: HSTRING, out: *mut <foundation::collections::IVectorView<WordSegment> as RtType>::Abi) -> HRESULT,
    fn Tokenize(&self, text: HSTRING, startIndex: u32, handler: <WordSegmentsTokenizingHandler as RtType>::Abi) -> HRESULT
}}
impl IWordsSegmenter {
    #[inline] pub fn get_resolved_language(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ResolvedLanguage)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_token_at(&self, text: &HStringArg, startIndex: u32) -> Result<Option<WordSegment>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTokenAt)(self.get_abi() as *const _ as *mut _, text.get(), startIndex, &mut out);
        if hr == S_OK { Ok(WordSegment::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_tokens(&self, text: &HStringArg) -> Result<Option<foundation::collections::IVectorView<WordSegment>>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetTokens)(self.get_abi() as *const _ as *mut _, text.get(), &mut out);
        if hr == S_OK { Ok(foundation::collections::IVectorView::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn tokenize(&self, text: &HStringArg, startIndex: u32, handler: &WordSegmentsTokenizingHandler) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Tokenize)(self.get_abi() as *const _ as *mut _, text.get(), startIndex, handler.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class WordsSegmenter: IWordsSegmenter}
impl RtActivatable<IWordsSegmenterFactory> for WordsSegmenter {}
impl WordsSegmenter {
    #[inline] pub fn create_with_language(language: &HStringArg) -> Result<WordsSegmenter> {
        <Self as RtActivatable<IWordsSegmenterFactory>>::get_activation_factory().create_with_language(language)
    }
}
DEFINE_CLSID!(WordsSegmenter(&[87,105,110,100,111,119,115,46,68,97,116,97,46,84,101,120,116,46,87,111,114,100,115,83,101,103,109,101,110,116,101,114,0]) [CLSID_WordsSegmenter]);
DEFINE_IID!(IID_IWordsSegmenterFactory, 3868684916, 64565, 17756, 139, 251, 109, 127, 70, 83, 202, 151);
RT_INTERFACE!{static interface IWordsSegmenterFactory(IWordsSegmenterFactoryVtbl, IWordsSegmenterFactory_Abi): IInspectable(IInspectableVtbl) [IID_IWordsSegmenterFactory] {
    fn CreateWithLanguage(&self, language: HSTRING, out: *mut <WordsSegmenter as RtType>::Abi) -> HRESULT
}}
impl IWordsSegmenterFactory {
    #[inline] pub fn create_with_language(&self, language: &HStringArg) -> Result<WordsSegmenter> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateWithLanguage)(self.get_abi() as *const _ as *mut _, language.get(), &mut out);
        if hr == S_OK { Ok(WordsSegmenter::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Data.Text
pub mod xml { // Windows.Data.Xml
pub mod dom { // Windows.Data.Xml.Dom
use crate::prelude::*;
DEFINE_IID!(IID_IDtdEntity, 1779130364, 25524, 18447, 158, 106, 138, 146, 129, 106, 173, 228);
RT_INTERFACE!{interface IDtdEntity(IDtdEntityVtbl, IDtdEntity_Abi): IInspectable(IInspectableVtbl) [IID_IDtdEntity] {
    fn get_PublicId(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_SystemId(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_NotationName(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IDtdEntity {
    #[inline] pub fn get_public_id(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_notation_name(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NotationName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DtdEntity: IDtdEntity}
DEFINE_IID!(IID_IDtdNotation, 2360664141, 27974, 20187, 171, 115, 223, 131, 197, 26, 211, 151);
RT_INTERFACE!{interface IDtdNotation(IDtdNotationVtbl, IDtdNotation_Abi): IInspectable(IInspectableVtbl) [IID_IDtdNotation] {
    fn get_PublicId(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_SystemId(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IDtdNotation {
    #[inline] pub fn get_public_id(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PublicId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_system_id(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_SystemId)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class DtdNotation: IDtdNotation}
RT_ENUM! { enum NodeType: i32 {
    Invalid = 0, ElementNode = 1, AttributeNode = 2, TextNode = 3, DataSectionNode = 4, EntityReferenceNode = 5, EntityNode = 6, ProcessingInstructionNode = 7, CommentNode = 8, DocumentNode = 9, DocumentTypeNode = 10, DocumentFragmentNode = 11, NotationNode = 12,
}}
DEFINE_IID!(IID_IXmlAttribute, 2887010980, 46321, 19894, 178, 6, 138, 34, 195, 8, 219, 10);
RT_INTERFACE!{interface IXmlAttribute(IXmlAttributeVtbl, IXmlAttribute_Abi): IInspectable(IInspectableVtbl) [IID_IXmlAttribute] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Specified(&self, out: *mut bool) -> HRESULT,
    fn get_Value(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Value(&self, value: HSTRING) -> HRESULT
}}
impl IXmlAttribute {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_specified(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Specified)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_value(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Value)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_value(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Value)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlAttribute: IXmlAttribute}
DEFINE_IID!(IID_IXmlCDataSection, 1292153967, 51389, 17844, 136, 153, 4, 0, 215, 194, 198, 15);
RT_INTERFACE!{interface IXmlCDataSection(IXmlCDataSectionVtbl, IXmlCDataSection_Abi): IInspectable(IInspectableVtbl) [IID_IXmlCDataSection] {
    
}}
RT_CLASS!{class XmlCDataSection: IXmlCDataSection}
DEFINE_IID!(IID_IXmlCharacterData, 321798827, 20022, 19958, 177, 200, 12, 230, 47, 216, 139, 38);
RT_INTERFACE!{interface IXmlCharacterData(IXmlCharacterDataVtbl, IXmlCharacterData_Abi): IInspectable(IInspectableVtbl) [IID_IXmlCharacterData] {
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT,
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn SubstringData(&self, offset: u32, count: u32, out: *mut HSTRING) -> HRESULT,
    fn AppendData(&self, data: HSTRING) -> HRESULT,
    fn InsertData(&self, offset: u32, data: HSTRING) -> HRESULT,
    fn DeleteData(&self, offset: u32, count: u32) -> HRESULT,
    fn ReplaceData(&self, offset: u32, count: u32, data: HSTRING) -> HRESULT
}}
impl IXmlCharacterData {
    #[inline] pub fn get_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Data)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Data)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Length)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn substring_data(&self, offset: u32, count: u32) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SubstringData)(self.get_abi() as *const _ as *mut _, offset, count, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_data(&self, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).AppendData)(self.get_abi() as *const _ as *mut _, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn insert_data(&self, offset: u32, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).InsertData)(self.get_abi() as *const _ as *mut _, offset, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn delete_data(&self, offset: u32, count: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).DeleteData)(self.get_abi() as *const _ as *mut _, offset, count);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn replace_data(&self, offset: u32, count: u32, data: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).ReplaceData)(self.get_abi() as *const _ as *mut _, offset, count, data.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlComment, 3164894421, 46623, 17937, 156, 172, 46, 146, 227, 71, 109, 71);
RT_INTERFACE!{interface IXmlComment(IXmlCommentVtbl, IXmlComment_Abi): IInspectable(IInspectableVtbl) [IID_IXmlComment] {
    
}}
RT_CLASS!{class XmlComment: IXmlComment}
DEFINE_IID!(IID_IXmlDocument, 4159939846, 7815, 17110, 188, 251, 184, 200, 9, 250, 84, 148);
RT_INTERFACE!{interface IXmlDocument(IXmlDocumentVtbl, IXmlDocument_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocument] {
    fn get_Doctype(&self, out: *mut <XmlDocumentType as RtType>::Abi) -> HRESULT,
    fn get_Implementation(&self, out: *mut <XmlDomImplementation as RtType>::Abi) -> HRESULT,
    fn get_DocumentElement(&self, out: *mut <XmlElement as RtType>::Abi) -> HRESULT,
    fn CreateElement(&self, tagName: HSTRING, out: *mut <XmlElement as RtType>::Abi) -> HRESULT,
    fn CreateDocumentFragment(&self, out: *mut <XmlDocumentFragment as RtType>::Abi) -> HRESULT,
    fn CreateTextNode(&self, data: HSTRING, out: *mut <XmlText as RtType>::Abi) -> HRESULT,
    fn CreateComment(&self, data: HSTRING, out: *mut <XmlComment as RtType>::Abi) -> HRESULT,
    fn CreateProcessingInstruction(&self, target: HSTRING, data: HSTRING, out: *mut <XmlProcessingInstruction as RtType>::Abi) -> HRESULT,
    fn CreateAttribute(&self, name: HSTRING, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn CreateEntityReference(&self, name: HSTRING, out: *mut <XmlEntityReference as RtType>::Abi) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut <XmlNodeList as RtType>::Abi) -> HRESULT,
    fn CreateCDataSection(&self, data: HSTRING, out: *mut <XmlCDataSection as RtType>::Abi) -> HRESULT,
    fn get_DocumentUri(&self, out: *mut HSTRING) -> HRESULT,
    fn CreateAttributeNS(&self, namespaceUri: <IInspectable as RtType>::Abi, qualifiedName: HSTRING, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn CreateElementNS(&self, namespaceUri: <IInspectable as RtType>::Abi, qualifiedName: HSTRING, out: *mut <XmlElement as RtType>::Abi) -> HRESULT,
    fn GetElementById(&self, elementId: HSTRING, out: *mut <XmlElement as RtType>::Abi) -> HRESULT,
    fn ImportNode(&self, node: <IXmlNode as RtType>::Abi, deep: bool, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT
}}
impl IXmlDocument {
    #[inline] pub fn get_doctype(&self) -> Result<Option<XmlDocumentType>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Doctype)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlDocumentType::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_implementation(&self) -> Result<Option<XmlDomImplementation>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Implementation)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlDomImplementation::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_element(&self) -> Result<Option<XmlElement>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentElement)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlElement::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_element(&self, tagName: &HStringArg) -> Result<Option<XmlElement>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateElement)(self.get_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(XmlElement::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_document_fragment(&self) -> Result<Option<XmlDocumentFragment>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateDocumentFragment)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlDocumentFragment::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_text_node(&self, data: &HStringArg) -> Result<Option<XmlText>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateTextNode)(self.get_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(XmlText::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_comment(&self, data: &HStringArg) -> Result<Option<XmlComment>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateComment)(self.get_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(XmlComment::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_processing_instruction(&self, target: &HStringArg, data: &HStringArg) -> Result<Option<XmlProcessingInstruction>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateProcessingInstruction)(self.get_abi() as *const _ as *mut _, target.get(), data.get(), &mut out);
        if hr == S_OK { Ok(XmlProcessingInstruction::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attribute(&self, name: &HStringArg) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateAttribute)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_entity_reference(&self, name: &HStringArg) -> Result<Option<XmlEntityReference>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateEntityReference)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(XmlEntityReference::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<Option<XmlNodeList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetElementsByTagName)(self.get_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(XmlNodeList::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_cdata_section(&self, data: &HStringArg) -> Result<Option<XmlCDataSection>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateCDataSection)(self.get_abi() as *const _ as *mut _, data.get(), &mut out);
        if hr == S_OK { Ok(XmlCDataSection::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_document_uri(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_DocumentUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_attribute_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateAttributeNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn create_element_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg) -> Result<Option<XmlElement>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateElementNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, qualifiedName.get(), &mut out);
        if hr == S_OK { Ok(XmlElement::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_element_by_id(&self, elementId: &HStringArg) -> Result<Option<XmlElement>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetElementById)(self.get_abi() as *const _ as *mut _, elementId.get(), &mut out);
        if hr == S_OK { Ok(XmlElement::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn import_node(&self, node: &IXmlNode, deep: bool) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ImportNode)(self.get_abi() as *const _ as *mut _, node.get_abi() as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDocument: IXmlDocument}
impl RtActivatable<IXmlDocumentStatics> for XmlDocument {}
impl RtActivatable<IActivationFactory> for XmlDocument {}
impl XmlDocument {
    #[inline] pub fn load_from_uri_async(uri: &foundation::Uri) -> Result<foundation::IAsyncOperation<XmlDocument>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_async(uri)
    }
    #[inline] pub fn load_from_uri_with_settings_async(uri: &foundation::Uri, loadSettings: &XmlLoadSettings) -> Result<foundation::IAsyncOperation<XmlDocument>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_uri_with_settings_async(uri, loadSettings)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(file: &crate::windows::storage::IStorageFile) -> Result<foundation::IAsyncOperation<XmlDocument>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_async(file)
    }
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_settings_async(file: &crate::windows::storage::IStorageFile, loadSettings: &XmlLoadSettings) -> Result<foundation::IAsyncOperation<XmlDocument>> {
        <Self as RtActivatable<IXmlDocumentStatics>>::get_activation_factory().load_from_file_with_settings_async(file, loadSettings)
    }
}
DEFINE_CLSID!(XmlDocument(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,68,111,99,117,109,101,110,116,0]) [CLSID_XmlDocument]);
DEFINE_IID!(IID_IXmlDocumentFragment, 3807013526, 3105, 17573, 139, 201, 158, 74, 38, 39, 8, 236);
RT_INTERFACE!{interface IXmlDocumentFragment(IXmlDocumentFragmentVtbl, IXmlDocumentFragment_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocumentFragment] {
    
}}
RT_CLASS!{class XmlDocumentFragment: IXmlDocumentFragment}
DEFINE_IID!(IID_IXmlDocumentIO, 1825630030, 61029, 17545, 158, 191, 202, 67, 232, 123, 166, 55);
RT_INTERFACE!{interface IXmlDocumentIO(IXmlDocumentIOVtbl, IXmlDocumentIO_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO] {
    fn LoadXml(&self, xml: HSTRING) -> HRESULT,
    fn LoadXmlWithSettings(&self, xml: HSTRING, loadSettings: <XmlLoadSettings as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn SaveToFileAsync(&self, file: <crate::windows::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncAction as RtType>::Abi) -> HRESULT
}}
impl IXmlDocumentIO {
    #[inline] pub fn load_xml(&self, xml: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).LoadXml)(self.get_abi() as *const _ as *mut _, xml.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn load_xml_with_settings(&self, xml: &HStringArg, loadSettings: &XmlLoadSettings) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).LoadXmlWithSettings)(self.get_abi() as *const _ as *mut _, xml.get(), loadSettings.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn save_to_file_async(&self, file: &crate::windows::storage::IStorageFile) -> Result<foundation::IAsyncAction> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SaveToFileAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncAction::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentIO2, 1560495713, 31704, 19157, 158, 191, 129, 230, 52, 114, 99, 177);
RT_INTERFACE!{interface IXmlDocumentIO2(IXmlDocumentIO2Vtbl, IXmlDocumentIO2_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocumentIO2] {
    #[cfg(feature="windows-storage")] fn LoadXmlFromBuffer(&self, buffer: <crate::windows::storage::streams::IBuffer as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadXmlFromBufferWithSettings(&self, buffer: <crate::windows::storage::streams::IBuffer as RtType>::Abi, loadSettings: <XmlLoadSettings as RtType>::Abi) -> HRESULT
}}
impl IXmlDocumentIO2 {
    #[cfg(feature="windows-storage")] #[inline] pub fn load_xml_from_buffer(&self, buffer: &crate::windows::storage::streams::IBuffer) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).LoadXmlFromBuffer)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_xml_from_buffer_with_settings(&self, buffer: &crate::windows::storage::streams::IBuffer, loadSettings: &XmlLoadSettings) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).LoadXmlFromBufferWithSettings)(self.get_abi() as *const _ as *mut _, buffer.get_abi() as *const _ as *mut _, loadSettings.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentStatics, 1430508116, 55127, 19321, 149, 57, 35, 43, 24, 245, 11, 241);
RT_INTERFACE!{static interface IXmlDocumentStatics(IXmlDocumentStaticsVtbl, IXmlDocumentStatics_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocumentStatics] {
    fn LoadFromUriAsync(&self, uri: <foundation::Uri as RtType>::Abi, out: *mut <foundation::IAsyncOperation<XmlDocument> as RtType>::Abi) -> HRESULT,
    fn LoadFromUriWithSettingsAsync(&self, uri: <foundation::Uri as RtType>::Abi, loadSettings: <XmlLoadSettings as RtType>::Abi, out: *mut <foundation::IAsyncOperation<XmlDocument> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileAsync(&self, file: <crate::windows::storage::IStorageFile as RtType>::Abi, out: *mut <foundation::IAsyncOperation<XmlDocument> as RtType>::Abi) -> HRESULT,
    #[cfg(feature="windows-storage")] fn LoadFromFileWithSettingsAsync(&self, file: <crate::windows::storage::IStorageFile as RtType>::Abi, loadSettings: <XmlLoadSettings as RtType>::Abi, out: *mut <foundation::IAsyncOperation<XmlDocument> as RtType>::Abi) -> HRESULT
}}
impl IXmlDocumentStatics {
    #[inline] pub fn load_from_uri_async(&self, uri: &foundation::Uri) -> Result<foundation::IAsyncOperation<XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromUriAsync)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[inline] pub fn load_from_uri_with_settings_async(&self, uri: &foundation::Uri, loadSettings: &XmlLoadSettings) -> Result<foundation::IAsyncOperation<XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromUriWithSettingsAsync)(self.get_abi() as *const _ as *mut _, uri.get_abi() as *const _ as *mut _, loadSettings.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_async(&self, file: &crate::windows::storage::IStorageFile) -> Result<foundation::IAsyncOperation<XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromFileAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
    #[cfg(feature="windows-storage")] #[inline] pub fn load_from_file_with_settings_async(&self, file: &crate::windows::storage::IStorageFile, loadSettings: &XmlLoadSettings) -> Result<foundation::IAsyncOperation<XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).LoadFromFileWithSettingsAsync)(self.get_abi() as *const _ as *mut _, file.get_abi() as *const _ as *mut _, loadSettings.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(foundation::IAsyncOperation::wrap_nonnull(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlDocumentType, 4147389477, 38785, 18788, 142, 148, 155, 28, 109, 252, 155, 199);
RT_INTERFACE!{interface IXmlDocumentType(IXmlDocumentTypeVtbl, IXmlDocumentType_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDocumentType] {
    fn get_Name(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Entities(&self, out: *mut <XmlNamedNodeMap as RtType>::Abi) -> HRESULT,
    fn get_Notations(&self, out: *mut <XmlNamedNodeMap as RtType>::Abi) -> HRESULT
}}
impl IXmlDocumentType {
    #[inline] pub fn get_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Name)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_entities(&self) -> Result<Option<XmlNamedNodeMap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Entities)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlNamedNodeMap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_notations(&self) -> Result<Option<XmlNamedNodeMap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Notations)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlNamedNodeMap::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDocumentType: IXmlDocumentType}
DEFINE_IID!(IID_IXmlDomImplementation, 1843757362, 61725, 20411, 140, 198, 88, 60, 186, 147, 17, 47);
RT_INTERFACE!{interface IXmlDomImplementation(IXmlDomImplementationVtbl, IXmlDomImplementation_Abi): IInspectable(IInspectableVtbl) [IID_IXmlDomImplementation] {
    fn HasFeature(&self, feature: HSTRING, version: <IInspectable as RtType>::Abi, out: *mut bool) -> HRESULT
}}
impl IXmlDomImplementation {
    #[inline] pub fn has_feature(&self, feature: &HStringArg, version: &IInspectable) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).HasFeature)(self.get_abi() as *const _ as *mut _, feature.get(), version.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlDomImplementation: IXmlDomImplementation}
DEFINE_IID!(IID_IXmlElement, 771459615, 27408, 20216, 159, 131, 239, 204, 232, 250, 236, 55);
RT_INTERFACE!{interface IXmlElement(IXmlElementVtbl, IXmlElement_Abi): IInspectable(IInspectableVtbl) [IID_IXmlElement] {
    fn get_TagName(&self, out: *mut HSTRING) -> HRESULT,
    fn GetAttribute(&self, attributeName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn SetAttribute(&self, attributeName: HSTRING, attributeValue: HSTRING) -> HRESULT,
    fn RemoveAttribute(&self, attributeName: HSTRING) -> HRESULT,
    fn GetAttributeNode(&self, attributeName: HSTRING, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn SetAttributeNode(&self, newAttribute: <XmlAttribute as RtType>::Abi, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn RemoveAttributeNode(&self, attributeNode: <XmlAttribute as RtType>::Abi, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn GetElementsByTagName(&self, tagName: HSTRING, out: *mut <XmlNodeList as RtType>::Abi) -> HRESULT,
    fn SetAttributeNS(&self, namespaceUri: <IInspectable as RtType>::Abi, qualifiedName: HSTRING, value: HSTRING) -> HRESULT,
    fn GetAttributeNS(&self, namespaceUri: <IInspectable as RtType>::Abi, localName: HSTRING, out: *mut HSTRING) -> HRESULT,
    fn RemoveAttributeNS(&self, namespaceUri: <IInspectable as RtType>::Abi, localName: HSTRING) -> HRESULT,
    fn SetAttributeNodeNS(&self, newAttribute: <XmlAttribute as RtType>::Abi, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT,
    fn GetAttributeNodeNS(&self, namespaceUri: <IInspectable as RtType>::Abi, localName: HSTRING, out: *mut <XmlAttribute as RtType>::Abi) -> HRESULT
}}
impl IXmlElement {
    #[inline] pub fn get_tag_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_TagName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute(&self, attributeName: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAttribute)(self.get_abi() as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute(&self, attributeName: &HStringArg, attributeValue: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetAttribute)(self.get_abi() as *const _ as *mut _, attributeName.get(), attributeValue.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute(&self, attributeName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).RemoveAttribute)(self.get_abi() as *const _ as *mut _, attributeName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_node(&self, attributeName: &HStringArg) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAttributeNode)(self.get_abi() as *const _ as *mut _, attributeName.get(), &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_node(&self, newAttribute: &XmlAttribute) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetAttributeNode)(self.get_abi() as *const _ as *mut _, newAttribute.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute_node(&self, attributeNode: &XmlAttribute) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RemoveAttributeNode)(self.get_abi() as *const _ as *mut _, attributeNode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_elements_by_tag_name(&self, tagName: &HStringArg) -> Result<Option<XmlNodeList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetElementsByTagName)(self.get_abi() as *const _ as *mut _, tagName.get(), &mut out);
        if hr == S_OK { Ok(XmlNodeList::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_ns(&self, namespaceUri: &IInspectable, qualifiedName: &HStringArg, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).SetAttributeNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, qualifiedName.get(), value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAttributeNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_attribute_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).RemoveAttributeNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, localName.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_attribute_node_ns(&self, newAttribute: &XmlAttribute) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetAttributeNodeNS)(self.get_abi() as *const _ as *mut _, newAttribute.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attribute_node_ns(&self, namespaceUri: &IInspectable, localName: &HStringArg) -> Result<Option<XmlAttribute>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetAttributeNodeNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, localName.get(), &mut out);
        if hr == S_OK { Ok(XmlAttribute::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlElement: IXmlElement}
DEFINE_IID!(IID_IXmlEntityReference, 774850492, 50128, 19663, 187, 134, 10, 184, 195, 106, 97, 207);
RT_INTERFACE!{interface IXmlEntityReference(IXmlEntityReferenceVtbl, IXmlEntityReference_Abi): IInspectable(IInspectableVtbl) [IID_IXmlEntityReference] {
    
}}
RT_CLASS!{class XmlEntityReference: IXmlEntityReference}
DEFINE_IID!(IID_IXmlLoadSettings, 1487538088, 65238, 18167, 180, 197, 251, 27, 167, 33, 8, 214);
RT_INTERFACE!{interface IXmlLoadSettings(IXmlLoadSettingsVtbl, IXmlLoadSettings_Abi): IInspectable(IInspectableVtbl) [IID_IXmlLoadSettings] {
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
impl IXmlLoadSettings {
    #[inline] pub fn get_max_element_depth(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_MaxElementDepth)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_max_element_depth(&self, value: u32) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_MaxElementDepth)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_prohibit_dtd(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ProhibitDtd)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_prohibit_dtd(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ProhibitDtd)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_resolve_externals(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ResolveExternals)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_resolve_externals(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ResolveExternals)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_validate_on_parse(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ValidateOnParse)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_validate_on_parse(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ValidateOnParse)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_element_content_white_space(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_ElementContentWhiteSpace)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn set_element_content_white_space(&self, value: bool) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_ElementContentWhiteSpace)(self.get_abi() as *const _ as *mut _, value);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlLoadSettings: IXmlLoadSettings}
impl RtActivatable<IActivationFactory> for XmlLoadSettings {}
DEFINE_CLSID!(XmlLoadSettings(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,68,111,109,46,88,109,108,76,111,97,100,83,101,116,116,105,110,103,115,0]) [CLSID_XmlLoadSettings]);
DEFINE_IID!(IID_IXmlNamedNodeMap, 3014041264, 43696, 19330, 166, 250, 177, 69, 63, 124, 2, 27);
RT_INTERFACE!{interface IXmlNamedNodeMap(IXmlNamedNodeMapVtbl, IXmlNamedNodeMap_Abi): IInspectable(IInspectableVtbl) [IID_IXmlNamedNodeMap] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn GetNamedItem(&self, name: HSTRING, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn SetNamedItem(&self, node: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn RemoveNamedItem(&self, name: HSTRING, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn GetNamedItemNS(&self, namespaceUri: <IInspectable as RtType>::Abi, name: HSTRING, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn RemoveNamedItemNS(&self, namespaceUri: <IInspectable as RtType>::Abi, name: HSTRING, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn SetNamedItemNS(&self, node: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT
}}
impl IXmlNamedNodeMap {
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Length)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn item(&self, index: u32) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Item)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_item(&self, name: &HStringArg) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedItem)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_item(&self, node: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetNamedItem)(self.get_abi() as *const _ as *mut _, node.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_named_item(&self, name: &HStringArg) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RemoveNamedItem)(self.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_named_item_ns(&self, namespaceUri: &IInspectable, name: &HStringArg) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetNamedItemNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_named_item_ns(&self, namespaceUri: &IInspectable, name: &HStringArg) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RemoveNamedItemNS)(self.get_abi() as *const _ as *mut _, namespaceUri.get_abi() as *const _ as *mut _, name.get(), &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_named_item_ns(&self, node: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SetNamedItemNS)(self.get_abi() as *const _ as *mut _, node.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlNamedNodeMap: IXmlNamedNodeMap}
DEFINE_IID!(IID_IXmlNode, 477371737, 8482, 18389, 168, 86, 131, 243, 212, 33, 72, 117);
RT_INTERFACE!{interface IXmlNode(IXmlNodeVtbl, IXmlNode_Abi): IInspectable(IInspectableVtbl) [IID_IXmlNode] {
    fn get_NodeValue(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn put_NodeValue(&self, value: <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_NodeType(&self, out: *mut NodeType) -> HRESULT,
    fn get_NodeName(&self, out: *mut HSTRING) -> HRESULT,
    fn get_ParentNode(&self, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_ChildNodes(&self, out: *mut <XmlNodeList as RtType>::Abi) -> HRESULT,
    fn get_FirstChild(&self, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_LastChild(&self, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_PreviousSibling(&self, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_NextSibling(&self, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_Attributes(&self, out: *mut <XmlNamedNodeMap as RtType>::Abi) -> HRESULT,
    fn HasChildNodes(&self, out: *mut bool) -> HRESULT,
    fn get_OwnerDocument(&self, out: *mut <XmlDocument as RtType>::Abi) -> HRESULT,
    fn InsertBefore(&self, newChild: <IXmlNode as RtType>::Abi, referenceChild: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn ReplaceChild(&self, newChild: <IXmlNode as RtType>::Abi, referenceChild: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn RemoveChild(&self, childNode: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn AppendChild(&self, newChild: <IXmlNode as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn CloneNode(&self, deep: bool, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn get_NamespaceUri(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_LocalName(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn get_Prefix(&self, out: *mut <IInspectable as RtType>::Abi) -> HRESULT,
    fn Normalize(&self) -> HRESULT,
    fn put_Prefix(&self, value: <IInspectable as RtType>::Abi) -> HRESULT
}}
impl IXmlNode {
    #[inline] pub fn get_node_value(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NodeValue)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_node_value(&self, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_NodeValue)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn get_node_type(&self) -> Result<NodeType> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_NodeType)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_node_name(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NodeName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_parent_node(&self) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ParentNode)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_child_nodes(&self) -> Result<Option<XmlNodeList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_ChildNodes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlNodeList::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_first_child(&self) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_FirstChild)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_last_child(&self) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LastChild)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_previous_sibling(&self) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_PreviousSibling)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_next_sibling(&self) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NextSibling)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_attributes(&self) -> Result<Option<XmlNamedNodeMap>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Attributes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlNamedNodeMap::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn has_child_nodes(&self) -> Result<bool> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).HasChildNodes)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn get_owner_document(&self) -> Result<Option<XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_OwnerDocument)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlDocument::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn insert_before(&self, newChild: &IXmlNode, referenceChild: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).InsertBefore)(self.get_abi() as *const _ as *mut _, newChild.get_abi() as *const _ as *mut _, referenceChild.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn replace_child(&self, newChild: &IXmlNode, referenceChild: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).ReplaceChild)(self.get_abi() as *const _ as *mut _, newChild.get_abi() as *const _ as *mut _, referenceChild.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn remove_child(&self, childNode: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).RemoveChild)(self.get_abi() as *const _ as *mut _, childNode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn append_child(&self, newChild: &IXmlNode) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).AppendChild)(self.get_abi() as *const _ as *mut _, newChild.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn clone_node(&self, deep: bool) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CloneNode)(self.get_abi() as *const _ as *mut _, deep, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_namespace_uri(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_NamespaceUri)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_local_name(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_LocalName)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_prefix(&self) -> Result<Option<IInspectable>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Prefix)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IInspectable::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn normalize(&self) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).Normalize)(self.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
    #[inline] pub fn set_prefix(&self, value: &IInspectable) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Prefix)(self.get_abi() as *const _ as *mut _, value.get_abi() as *const _ as *mut _);
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlNodeList, 2355146103, 33700, 20161, 156, 84, 123, 164, 41, 225, 61, 166);
RT_INTERFACE!{interface IXmlNodeList(IXmlNodeListVtbl, IXmlNodeList_Abi): IInspectable(IInspectableVtbl) [IID_IXmlNodeList] {
    fn get_Length(&self, out: *mut u32) -> HRESULT,
    fn Item(&self, index: u32, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT
}}
impl IXmlNodeList {
    #[inline] pub fn get_length(&self) -> Result<u32> { unsafe { 
        let mut out = zeroed();
        let hr = ((*self.get_abi().lpVtbl).get_Length)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(out) } else { err(hr) }
    }}
    #[inline] pub fn item(&self, index: u32) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).Item)(self.get_abi() as *const _ as *mut _, index, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlNodeList: IXmlNodeList}
DEFINE_IID!(IID_IXmlNodeSelector, 1675344523, 53467, 20449, 183, 69, 249, 67, 58, 253, 194, 91);
RT_INTERFACE!{interface IXmlNodeSelector(IXmlNodeSelectorVtbl, IXmlNodeSelector_Abi): IInspectable(IInspectableVtbl) [IID_IXmlNodeSelector] {
    fn SelectSingleNode(&self, xpath: HSTRING, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn SelectNodes(&self, xpath: HSTRING, out: *mut <XmlNodeList as RtType>::Abi) -> HRESULT,
    fn SelectSingleNodeNS(&self, xpath: HSTRING, namespaces: <IInspectable as RtType>::Abi, out: *mut <IXmlNode as RtType>::Abi) -> HRESULT,
    fn SelectNodesNS(&self, xpath: HSTRING, namespaces: <IInspectable as RtType>::Abi, out: *mut <XmlNodeList as RtType>::Abi) -> HRESULT
}}
impl IXmlNodeSelector {
    #[inline] pub fn select_single_node(&self, xpath: &HStringArg) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SelectSingleNode)(self.get_abi() as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_nodes(&self, xpath: &HStringArg) -> Result<Option<XmlNodeList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SelectNodes)(self.get_abi() as *const _ as *mut _, xpath.get(), &mut out);
        if hr == S_OK { Ok(XmlNodeList::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_single_node_ns(&self, xpath: &HStringArg, namespaces: &IInspectable) -> Result<Option<IXmlNode>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SelectSingleNodeNS)(self.get_abi() as *const _ as *mut _, xpath.get(), namespaces.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(IXmlNode::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn select_nodes_ns(&self, xpath: &HStringArg, namespaces: &IInspectable) -> Result<Option<XmlNodeList>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SelectNodesNS)(self.get_abi() as *const _ as *mut _, xpath.get(), namespaces.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XmlNodeList::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlNodeSerializer, 1556460418, 59101, 18833, 171, 239, 6, 216, 210, 231, 189, 12);
RT_INTERFACE!{interface IXmlNodeSerializer(IXmlNodeSerializerVtbl, IXmlNodeSerializer_Abi): IInspectable(IInspectableVtbl) [IID_IXmlNodeSerializer] {
    fn GetXml(&self, out: *mut HSTRING) -> HRESULT,
    fn get_InnerText(&self, out: *mut HSTRING) -> HRESULT,
    fn put_InnerText(&self, value: HSTRING) -> HRESULT
}}
impl IXmlNodeSerializer {
    #[inline] pub fn get_xml(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).GetXml)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_inner_text(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_InnerText)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_inner_text(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_InnerText)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXmlProcessingInstruction, 654834974, 7826, 20174, 182, 244, 38, 240, 105, 7, 141, 220);
RT_INTERFACE!{interface IXmlProcessingInstruction(IXmlProcessingInstructionVtbl, IXmlProcessingInstruction_Abi): IInspectable(IInspectableVtbl) [IID_IXmlProcessingInstruction] {
    fn get_Target(&self, out: *mut HSTRING) -> HRESULT,
    fn get_Data(&self, out: *mut HSTRING) -> HRESULT,
    fn put_Data(&self, value: HSTRING) -> HRESULT
}}
impl IXmlProcessingInstruction {
    #[inline] pub fn get_target(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Target)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn get_data(&self) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).get_Data)(self.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
    #[inline] pub fn set_data(&self, value: &HStringArg) -> Result<()> { unsafe { 
        let hr = ((*self.get_abi().lpVtbl).put_Data)(self.get_abi() as *const _ as *mut _, value.get());
        if hr == S_OK { Ok(()) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlProcessingInstruction: IXmlProcessingInstruction}
DEFINE_IID!(IID_IXmlText, 4180780235, 12429, 18272, 161, 213, 67, 182, 116, 80, 172, 126);
RT_INTERFACE!{interface IXmlText(IXmlTextVtbl, IXmlText_Abi): IInspectable(IInspectableVtbl) [IID_IXmlText] {
    fn SplitText(&self, offset: u32, out: *mut <IXmlText as RtType>::Abi) -> HRESULT
}}
impl IXmlText {
    #[inline] pub fn split_text(&self, offset: u32) -> Result<Option<IXmlText>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).SplitText)(self.get_abi() as *const _ as *mut _, offset, &mut out);
        if hr == S_OK { Ok(IXmlText::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XmlText: IXmlText}
} // Windows.Data.Xml.Dom
pub mod xsl { // Windows.Data.Xml.Xsl
use crate::prelude::*;
DEFINE_IID!(IID_IXsltProcessor, 2070179903, 21772, 18630, 169, 15, 147, 165, 185, 100, 81, 143);
RT_INTERFACE!{interface IXsltProcessor(IXsltProcessorVtbl, IXsltProcessor_Abi): IInspectable(IInspectableVtbl) [IID_IXsltProcessor] {
    fn TransformToString(&self, inputNode: <super::dom::IXmlNode as RtType>::Abi, out: *mut HSTRING) -> HRESULT
}}
impl IXsltProcessor {
    #[inline] pub fn transform_to_string(&self, inputNode: &super::dom::IXmlNode) -> Result<HString> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TransformToString)(self.get_abi() as *const _ as *mut _, inputNode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(HString::wrap(out)) } else { err(hr) }
    }}
}
RT_CLASS!{class XsltProcessor: IXsltProcessor}
impl RtActivatable<IXsltProcessorFactory> for XsltProcessor {}
impl XsltProcessor {
    #[inline] pub fn create_instance(document: &super::dom::XmlDocument) -> Result<XsltProcessor> {
        <Self as RtActivatable<IXsltProcessorFactory>>::get_activation_factory().create_instance(document)
    }
}
DEFINE_CLSID!(XsltProcessor(&[87,105,110,100,111,119,115,46,68,97,116,97,46,88,109,108,46,88,115,108,46,88,115,108,116,80,114,111,99,101,115,115,111,114,0]) [CLSID_XsltProcessor]);
DEFINE_IID!(IID_IXsltProcessor2, 2376358998, 38821, 17611, 168, 190, 39, 216, 98, 128, 199, 10);
RT_INTERFACE!{interface IXsltProcessor2(IXsltProcessor2Vtbl, IXsltProcessor2_Abi): IInspectable(IInspectableVtbl) [IID_IXsltProcessor2] {
    fn TransformToDocument(&self, inputNode: <super::dom::IXmlNode as RtType>::Abi, out: *mut <super::dom::XmlDocument as RtType>::Abi) -> HRESULT
}}
impl IXsltProcessor2 {
    #[inline] pub fn transform_to_document(&self, inputNode: &super::dom::IXmlNode) -> Result<Option<super::dom::XmlDocument>> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).TransformToDocument)(self.get_abi() as *const _ as *mut _, inputNode.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(super::dom::XmlDocument::wrap(out)) } else { err(hr) }
    }}
}
DEFINE_IID!(IID_IXsltProcessorFactory, 658589376, 39505, 18019, 191, 48, 14, 247, 66, 20, 111, 32);
RT_INTERFACE!{static interface IXsltProcessorFactory(IXsltProcessorFactoryVtbl, IXsltProcessorFactory_Abi): IInspectable(IInspectableVtbl) [IID_IXsltProcessorFactory] {
    fn CreateInstance(&self, document: <super::dom::XmlDocument as RtType>::Abi, out: *mut <XsltProcessor as RtType>::Abi) -> HRESULT
}}
impl IXsltProcessorFactory {
    #[inline] pub fn create_instance(&self, document: &super::dom::XmlDocument) -> Result<XsltProcessor> { unsafe { 
        let mut out = null_mut();
        let hr = ((*self.get_abi().lpVtbl).CreateInstance)(self.get_abi() as *const _ as *mut _, document.get_abi() as *const _ as *mut _, &mut out);
        if hr == S_OK { Ok(XsltProcessor::wrap_nonnull(out)) } else { err(hr) }
    }}
}
} // Windows.Data.Xml.Xsl
} // Windows.Data.Xml
