use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlOListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OListProp {
    reversed(bool),
    start(i32),
    r#type(String),
    compact(bool),
}

impl ElementComponent for HtmlOListElement {
    type PropEnum = OListProp;
}
impl PropEnum<HtmlOListElement> for OListProp {
    fn unset_on(&self, elem: &HtmlOListElement) {
        match self {
            OListProp::reversed(_) => elem.remove_attribute("reversed").unwrap(),
            OListProp::start(_) => elem.remove_attribute("start").unwrap(),
            OListProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            OListProp::compact(_) => elem.remove_attribute("compact").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlOListElement) {
        match self {
            OListProp::reversed(v) => elem.set_reversed(*v),
            OListProp::start(v) => elem.set_start(*v),
            OListProp::r#type(v) => elem.set_type(v),
            OListProp::compact(v) => elem.set_compact(*v),
        }
    }
}

impl ElementProps<HtmlOListElement> {
    pub fn reversed(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(OListProp::reversed(val)));
        self
    }

    pub fn start(mut self, val: i32) -> Self {
        self.0.push_back(ElementProp::Own(OListProp::start(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(OListProp::r#type(val)));
        self
    }

    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(OListProp::compact(val)));
        self
    }
}
