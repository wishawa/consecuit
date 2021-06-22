use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlDivElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DivProp {
    align(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDivElement {
    type PropEnum = DivProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDivElement> for DivProp {
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

impl HtmlProps<HtmlDivElement> {
    pub fn align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(DivProp::align(val)));
        self
    }
}
