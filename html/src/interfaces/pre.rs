use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlPreElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum PreProp {
    width(i32),
}

impl ElementComponent for HtmlPreElement {
    type PropEnum = PreProp;
}
impl PropEnum<HtmlPreElement> for PreProp {
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

impl ElementProps<HtmlPreElement> {
    pub fn width(mut self, val: i32) -> Self {
        self.0.push_back(ElementProp::Own(PreProp::width(val)));
        self
    }
}
