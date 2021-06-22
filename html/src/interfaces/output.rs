use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlOutputElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OutputProp {
    name(Cow<'static, str>),
    default_value(Cow<'static, str>),
    value(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlOutputElement {
    type PropEnum = OutputProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlOutputElement> for OutputProp {
    fn unset_on(&self, elem: &HtmlOutputElement) {
        match self {
            OutputProp::name(_) => elem.remove_attribute("name").unwrap(),
            OutputProp::default_value(_) => elem.remove_attribute("default_value").unwrap(),
            OutputProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlOutputElement) {
        match self {
            OutputProp::name(v) => elem.set_name(v),
            OutputProp::default_value(v) => elem.set_default_value(v),
            OutputProp::value(v) => elem.set_value(v),
        }
    }
}

impl HtmlProps<HtmlOutputElement> {
    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OutputProp::name(val)));
        self
    }

    pub fn default_value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(OutputProp::default_value(val)));
        self
    }

    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OutputProp::value(val)));
        self
    }
}
