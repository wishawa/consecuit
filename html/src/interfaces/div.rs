use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlDivElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DivProp {
    align(String),
}

impl ElementComponent for HtmlDivElement {
    type PropEnum = DivProp;
}
impl PropEnum<HtmlDivElement> for DivProp {
    fn unset_on(&self, elem: &HtmlDivElement) {
        match self {
            DivProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDivElement) {
        match self {
            DivProp::align(v) => elem.set_align(v),
        }
    }
}

impl ElementProps<HtmlDivElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(DivProp::align(val)));
        self
    }
}
