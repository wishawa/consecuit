use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlBaseElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum BaseProp {
    href(String),
    target(String),
}

impl ElementComponent for HtmlBaseElement {
    type PropEnum = BaseProp;
}
impl PropEnum<HtmlBaseElement> for BaseProp {
    fn unset_on(&self, elem: &HtmlBaseElement) {
        match self {
            BaseProp::href(_) => elem.remove_attribute("href").unwrap(),
            BaseProp::target(_) => elem.remove_attribute("target").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlBaseElement) {
        match self {
            BaseProp::href(v) => elem.set_href(v),
            BaseProp::target(v) => elem.set_target(v),
        }
    }
}

impl ElementProps<HtmlBaseElement> {
    pub fn href(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BaseProp::href(val)));
        self
    }

    pub fn target(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BaseProp::target(val)));
        self
    }
}
