use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlButtonElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ButtonProp {
    autofocus(bool),
    disabled(bool),
    form_action(String),
    form_enctype(String),
    form_method(String),
    form_no_validate(bool),
    form_target(String),
    name(String),
    r#type(String),
    value(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlButtonElement {
    type PropEnum = ButtonProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlButtonElement> for ButtonProp {
    fn unset_on(&self, elem: &HtmlButtonElement) {
        match self {
            ButtonProp::autofocus(_) => elem.remove_attribute("autofocus").unwrap(),
            ButtonProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            ButtonProp::form_action(_) => elem.remove_attribute("form_action").unwrap(),
            ButtonProp::form_enctype(_) => elem.remove_attribute("form_enctype").unwrap(),
            ButtonProp::form_method(_) => elem.remove_attribute("form_method").unwrap(),
            ButtonProp::form_no_validate(_) => elem.remove_attribute("form_no_validate").unwrap(),
            ButtonProp::form_target(_) => elem.remove_attribute("form_target").unwrap(),
            ButtonProp::name(_) => elem.remove_attribute("name").unwrap(),
            ButtonProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            ButtonProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlButtonElement) {
        match self {
            ButtonProp::autofocus(v) => elem.set_autofocus(*v),
            ButtonProp::disabled(v) => elem.set_disabled(*v),
            ButtonProp::form_action(v) => elem.set_form_action(v),
            ButtonProp::form_enctype(v) => elem.set_form_enctype(v),
            ButtonProp::form_method(v) => elem.set_form_method(v),
            ButtonProp::form_no_validate(v) => elem.set_form_no_validate(*v),
            ButtonProp::form_target(v) => elem.set_form_target(v),
            ButtonProp::name(v) => elem.set_name(v),
            ButtonProp::r#type(v) => elem.set_type(v),
            ButtonProp::value(v) => elem.set_value(v),
        }
    }
}

impl HtmlProps<HtmlButtonElement> {
    pub fn autofocus(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ButtonProp::autofocus(val)));
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ButtonProp::disabled(val)));
        self
    }

    pub fn form_action(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(ButtonProp::form_action(val)));
        self
    }

    pub fn form_enctype(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(ButtonProp::form_enctype(val)));
        self
    }

    pub fn form_method(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(ButtonProp::form_method(val)));
        self
    }

    pub fn form_no_validate(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(ButtonProp::form_no_validate(val)));
        self
    }

    pub fn form_target(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(ButtonProp::form_target(val)));
        self
    }

    pub fn name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ButtonProp::name(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ButtonProp::r#type(val)));
        self
    }

    pub fn value(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ButtonProp::value(val)));
        self
    }
}
