use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlOptGroupElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OptGroupProp {
    disabled(bool),
    label(String),
}

impl ElementComponent for HtmlOptGroupElement {
    type PropEnum = OptGroupProp;
}
impl PropEnum<HtmlOptGroupElement> for OptGroupProp {
    fn unset_on(&self, elem: &HtmlOptGroupElement) {
        match self {
            OptGroupProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            OptGroupProp::label(_) => elem.remove_attribute("label").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlOptGroupElement) {
        match self {
            OptGroupProp::disabled(v) => elem.set_disabled(*v),
            OptGroupProp::label(v) => elem.set_label(v),
        }
    }
}

impl ElementProps<HtmlOptGroupElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(OptGroupProp::disabled(val)));
        self
    }

    pub fn label(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(OptGroupProp::label(val)));
        self
    }
}
