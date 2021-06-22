use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlTextAreaElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TextAreaProp {
    autocomplete(Cow<'static, str>),
    autofocus(bool),
    cols(u32),
    disabled(bool),
    max_length(i32),
    min_length(i32),
    name(Cow<'static, str>),
    placeholder(Cow<'static, str>),
    read_only(bool),
    required(bool),
    rows(u32),
    wrap(Cow<'static, str>),
    value(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTextAreaElement {
    type PropEnum = TextAreaProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTextAreaElement> for TextAreaProp {
    fn unset_on(&self, elem: &HtmlTextAreaElement) {
        match self {
            TextAreaProp::autocomplete(_) => elem.remove_attribute("autocomplete").unwrap(),
            TextAreaProp::autofocus(_) => elem.remove_attribute("autofocus").unwrap(),
            TextAreaProp::cols(_) => elem.remove_attribute("cols").unwrap(),
            TextAreaProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            TextAreaProp::max_length(_) => elem.remove_attribute("max_length").unwrap(),
            TextAreaProp::min_length(_) => elem.remove_attribute("min_length").unwrap(),
            TextAreaProp::name(_) => elem.remove_attribute("name").unwrap(),
            TextAreaProp::placeholder(_) => elem.remove_attribute("placeholder").unwrap(),
            TextAreaProp::read_only(_) => elem.remove_attribute("read_only").unwrap(),
            TextAreaProp::required(_) => elem.remove_attribute("required").unwrap(),
            TextAreaProp::rows(_) => elem.remove_attribute("rows").unwrap(),
            TextAreaProp::wrap(_) => elem.remove_attribute("wrap").unwrap(),
            TextAreaProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTextAreaElement) {
        match self {
            TextAreaProp::autocomplete(v) => elem.set_autocomplete(v),
            TextAreaProp::autofocus(v) => elem.set_autofocus(*v),
            TextAreaProp::cols(v) => elem.set_cols(*v),
            TextAreaProp::disabled(v) => elem.set_disabled(*v),
            TextAreaProp::max_length(v) => elem.set_max_length(*v),
            TextAreaProp::min_length(v) => elem.set_min_length(*v),
            TextAreaProp::name(v) => elem.set_name(v),
            TextAreaProp::placeholder(v) => elem.set_placeholder(v),
            TextAreaProp::read_only(v) => elem.set_read_only(*v),
            TextAreaProp::required(v) => elem.set_required(*v),
            TextAreaProp::rows(v) => elem.set_rows(*v),
            TextAreaProp::wrap(v) => elem.set_wrap(v),
            TextAreaProp::value(v) => elem.set_value(v),
        }
    }
}

impl HtmlProps<HtmlTextAreaElement> {
    pub fn autocomplete(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::autocomplete(val)));
        self
    }

    pub fn autofocus(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::autofocus(val)));
        self
    }

    pub fn cols(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(TextAreaProp::cols(val)));
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(TextAreaProp::disabled(val)));
        self
    }

    pub fn max_length(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::max_length(val)));
        self
    }

    pub fn min_length(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::min_length(val)));
        self
    }

    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TextAreaProp::name(val)));
        self
    }

    pub fn placeholder(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::placeholder(val)));
        self
    }

    pub fn read_only(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(TextAreaProp::read_only(val)));
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(TextAreaProp::required(val)));
        self
    }

    pub fn rows(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(TextAreaProp::rows(val)));
        self
    }

    pub fn wrap(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TextAreaProp::wrap(val)));
        self
    }

    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TextAreaProp::value(val)));
        self
    }
}
