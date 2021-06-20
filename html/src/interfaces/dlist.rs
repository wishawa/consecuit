use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlDListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DListProp {
    compact(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDListElement {
    type PropEnum = DListProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDListElement> for DListProp {
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

impl HtmlProps<HtmlDListElement> {
    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(DListProp::compact(val)));
        self
    }
}
