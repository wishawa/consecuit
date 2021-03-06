use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlFormElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FormProp {
    accept_charset(Cow<'static, str>),
    action(Cow<'static, str>),
    autocomplete(Cow<'static, str>),
    enctype(Cow<'static, str>),
    encoding(Cow<'static, str>),
    method(Cow<'static, str>),
    name(Cow<'static, str>),
    no_validate(bool),
    target(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlFormElement {
    type PropEnum = FormProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlFormElement> for FormProp {
    fn unset_on(&self, elem: &HtmlFormElement) {
        match self {
            FormProp::accept_charset(_) => elem.remove_attribute("accept_charset").unwrap(),
            FormProp::action(_) => elem.remove_attribute("action").unwrap(),
            FormProp::autocomplete(_) => elem.remove_attribute("autocomplete").unwrap(),
            FormProp::enctype(_) => elem.remove_attribute("enctype").unwrap(),
            FormProp::encoding(_) => elem.remove_attribute("encoding").unwrap(),
            FormProp::method(_) => elem.remove_attribute("method").unwrap(),
            FormProp::name(_) => elem.remove_attribute("name").unwrap(),
            FormProp::no_validate(_) => elem.remove_attribute("no_validate").unwrap(),
            FormProp::target(_) => elem.remove_attribute("target").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlFormElement) {
        match self {
            FormProp::accept_charset(v) => elem.set_accept_charset(v),
            FormProp::action(v) => elem.set_action(v),
            FormProp::autocomplete(v) => elem.set_autocomplete(v),
            FormProp::enctype(v) => elem.set_enctype(v),
            FormProp::encoding(v) => elem.set_encoding(v),
            FormProp::method(v) => elem.set_method(v),
            FormProp::name(v) => elem.set_name(v),
            FormProp::no_validate(v) => elem.set_no_validate(*v),
            FormProp::target(v) => elem.set_target(v),
        }
    }
}

impl HtmlProps<HtmlFormElement> {
    pub fn accept_charset(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(FormProp::accept_charset(val)));
        self
    }

    pub fn action(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::action(val)));
        self
    }

    pub fn autocomplete(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::autocomplete(val)));
        self
    }

    pub fn enctype(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::enctype(val)));
        self
    }

    pub fn encoding(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::encoding(val)));
        self
    }

    pub fn method(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::method(val)));
        self
    }

    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::name(val)));
        self
    }

    pub fn no_validate(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(FormProp::no_validate(val)));
        self
    }

    pub fn target(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FormProp::target(val)));
        self
    }
}
