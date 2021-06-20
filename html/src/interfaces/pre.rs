use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlPreElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum PreProp {
    width(i32),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlPreElement {
    type PropEnum = PreProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlPreElement> for PreProp {
    fn unset_on(&self, elem: &HtmlPreElement) {
        match self {
            PreProp::width(_) => elem.remove_attribute("width").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlPreElement) {
        match self {
            PreProp::width(v) => elem.set_width(*v),
        }
    }
}

impl HtmlProps<HtmlPreElement> {
    pub fn width(mut self, val: i32) -> Self {
        self.0.push_back(HtmlProp::Own(PreProp::width(val)));
        self
    }
}
