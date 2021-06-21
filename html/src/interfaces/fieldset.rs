use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlFieldSetElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FieldSetProp {
    disabled(bool),
    name(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlFieldSetElement {
    type PropEnum = FieldSetProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlFieldSetElement> for FieldSetProp {
    fn unset_on(&self, elem: &HtmlFieldSetElement) {
        match self {
            FieldSetProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            FieldSetProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlFieldSetElement) {
        match self {
            FieldSetProp::disabled(v) => elem.set_disabled(*v),
            FieldSetProp::name(v) => elem.set_name(v),
        }
    }
}

impl HtmlProps<HtmlFieldSetElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(FieldSetProp::disabled(val)));
        self
    }

    pub fn name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FieldSetProp::name(val)));
        self
    }
}
