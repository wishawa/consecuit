use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlFieldSetElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FieldSetProp {
    disabled(bool),
    name(String),
}

impl ElementComponent for HtmlFieldSetElement {
    type PropEnum = FieldSetProp;
}
impl PropEnum<HtmlFieldSetElement> for FieldSetProp {
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

impl ElementProps<HtmlFieldSetElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(FieldSetProp::disabled(val)));
        self
    }

    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(FieldSetProp::name(val)));
        self
    }
}
