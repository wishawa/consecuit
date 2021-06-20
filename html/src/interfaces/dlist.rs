use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlDListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DListProp {
    compact(bool),
}

impl ElementComponent for HtmlDListElement {
    type PropEnum = DListProp;
}
impl PropEnum<HtmlDListElement> for DListProp {
    fn unset_on(&self, elem: &HtmlDListElement) {
        match self {
            DListProp::compact(_) => elem.remove_attribute("compact").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDListElement) {
        match self {
            DListProp::compact(v) => elem.set_compact(*v),
        }
    }
}

impl ElementProps<HtmlDListElement> {
    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(DListProp::compact(val)));
        self
    }
}
