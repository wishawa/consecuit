use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlUListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum UListProp {
    compact(bool),
    r#type(String),
}

impl ElementComponent for HtmlUListElement {
    type PropEnum = UListProp;
}
impl PropEnum<HtmlUListElement> for UListProp {
    fn unset_on(&self, elem: &HtmlUListElement) {
        match self {
            UListProp::compact(_) => elem.remove_attribute("compact").unwrap(),
            UListProp::r#type(_) => elem.remove_attribute("type").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlUListElement) {
        match self {
            UListProp::compact(v) => elem.set_compact(*v),
            UListProp::r#type(v) => elem.set_type(v),
        }
    }
}

impl ElementProps<HtmlUListElement> {
    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(UListProp::compact(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(UListProp::r#type(val)));
        self
    }
}
