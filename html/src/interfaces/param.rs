use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlParamElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ParamProp {
    name(Cow<'static, str>),
    value(Cow<'static, str>),
    r#type(Cow<'static, str>),
    value_type(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlParamElement {
    type PropEnum = ParamProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlParamElement> for ParamProp {
    fn unset_on(&self, elem: &HtmlParamElement) {
        match self {
            ParamProp::name(_) => elem.remove_attribute("name").unwrap(),
            ParamProp::value(_) => elem.remove_attribute("value").unwrap(),
            ParamProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            ParamProp::value_type(_) => elem.remove_attribute("value_type").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlParamElement) {
        match self {
            ParamProp::name(v) => elem.set_name(v),
            ParamProp::value(v) => elem.set_value(v),
            ParamProp::r#type(v) => elem.set_type(v),
            ParamProp::value_type(v) => elem.set_value_type(v),
        }
    }
}

impl HtmlProps<HtmlParamElement> {
    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ParamProp::name(val)));
        self
    }

    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ParamProp::value(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(ParamProp::r#type(val)));
        self
    }

    pub fn value_type(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ParamProp::value_type(val)));
        self
    }
}
