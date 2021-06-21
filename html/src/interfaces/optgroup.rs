use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlOptGroupElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OptGroupProp {
    disabled(bool),
    label(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlOptGroupElement {
    type PropEnum = OptGroupProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlOptGroupElement> for OptGroupProp {
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

impl HtmlProps<HtmlOptGroupElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(OptGroupProp::disabled(val)));
        self
    }

    pub fn label(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(OptGroupProp::label(val)));
        self
    }
}
