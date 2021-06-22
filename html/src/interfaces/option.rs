use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlOptionElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OptionProp {
    disabled(bool),
    label(Cow<'static, str>),
    default_selected(bool),
    selected(bool),
    value(Cow<'static, str>),
    text(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlOptionElement {
    type PropEnum = OptionProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlOptionElement> for OptionProp {
    fn unset_on(&self, elem: &HtmlOptionElement) {
        match self {
            OptionProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            OptionProp::label(_) => elem.remove_attribute("label").unwrap(),
            OptionProp::default_selected(_) => elem.remove_attribute("default_selected").unwrap(),
            OptionProp::selected(_) => elem.remove_attribute("selected").unwrap(),
            OptionProp::value(_) => elem.remove_attribute("value").unwrap(),
            OptionProp::text(_) => elem.remove_attribute("text").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlOptionElement) {
        match self {
            OptionProp::disabled(v) => elem.set_disabled(*v),
            OptionProp::label(v) => elem.set_label(v),
            OptionProp::default_selected(v) => elem.set_default_selected(*v),
            OptionProp::selected(v) => elem.set_selected(*v),
            OptionProp::value(v) => elem.set_value(v),
            OptionProp::text(v) => elem.set_text(v),
        }
    }
}

impl HtmlProps<HtmlOptionElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(OptionProp::disabled(val)));
        self
    }

    pub fn label(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OptionProp::label(val)));
        self
    }

    pub fn default_selected(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(OptionProp::default_selected(val)));
        self
    }

    pub fn selected(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(OptionProp::selected(val)));
        self
    }

    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OptionProp::value(val)));
        self
    }

    pub fn text(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OptionProp::text(val)));
        self
    }
}
