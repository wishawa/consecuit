use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlSelectElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SelectProp {
    autofocus(bool),
    autocomplete(Cow<'static, str>),
    disabled(bool),
    multiple(bool),
    name(Cow<'static, str>),
    required(bool),
    size(u32),
    length(u32),
    selected_index(i32),
    value(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlSelectElement {
    type PropEnum = SelectProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlSelectElement> for SelectProp {
    fn unset_on(&self, elem: &HtmlSelectElement) {
        match self {
            SelectProp::autofocus(_) => elem.remove_attribute("autofocus").unwrap(),
            SelectProp::autocomplete(_) => elem.remove_attribute("autocomplete").unwrap(),
            SelectProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            SelectProp::multiple(_) => elem.remove_attribute("multiple").unwrap(),
            SelectProp::name(_) => elem.remove_attribute("name").unwrap(),
            SelectProp::required(_) => elem.remove_attribute("required").unwrap(),
            SelectProp::size(_) => elem.remove_attribute("size").unwrap(),
            SelectProp::length(_) => elem.remove_attribute("length").unwrap(),
            SelectProp::selected_index(_) => elem.remove_attribute("selected_index").unwrap(),
            SelectProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlSelectElement) {
        match self {
            SelectProp::autofocus(v) => elem.set_autofocus(*v),
            SelectProp::autocomplete(v) => elem.set_autocomplete(v),
            SelectProp::disabled(v) => elem.set_disabled(*v),
            SelectProp::multiple(v) => elem.set_multiple(*v),
            SelectProp::name(v) => elem.set_name(v),
            SelectProp::required(v) => elem.set_required(*v),
            SelectProp::size(v) => elem.set_size(*v),
            SelectProp::length(v) => elem.set_length(*v),
            SelectProp::selected_index(v) => elem.set_selected_index(*v),
            SelectProp::value(v) => elem.set_value(v),
        }
    }
}

impl HtmlProps<HtmlSelectElement> {
    pub fn autofocus(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::autofocus(val)));
        self
    }

    pub fn autocomplete(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(SelectProp::autocomplete(val)));
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::disabled(val)));
        self
    }

    pub fn multiple(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::multiple(val)));
        self
    }

    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(SelectProp::name(val)));
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::required(val)));
        self
    }

    pub fn size(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::size(val)));
        self
    }

    pub fn length(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(SelectProp::length(val)));
        self
    }

    pub fn selected_index(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Own(SelectProp::selected_index(val)));
        self
    }

    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(SelectProp::value(val)));
        self
    }
}
