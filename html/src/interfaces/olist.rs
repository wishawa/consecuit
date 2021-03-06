use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlOListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OListProp {
    reversed(bool),
    start(i32),
    r#type(Cow<'static, str>),
    compact(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlOListElement {
    type PropEnum = OListProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlOListElement> for OListProp {
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

impl HtmlProps<HtmlOListElement> {
    pub fn reversed(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(OListProp::reversed(val)));
        self
    }

    pub fn start(mut self, val: i32) -> Self {
        self.0.push_back(HtmlProp::Own(OListProp::start(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(OListProp::r#type(val)));
        self
    }

    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(OListProp::compact(val)));
        self
    }
}
