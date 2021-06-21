use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlBaseElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum BaseProp {
    href(String),
    target(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlBaseElement {
    type PropEnum = BaseProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlBaseElement> for BaseProp {
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

impl HtmlProps<HtmlBaseElement> {
    pub fn href(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(BaseProp::href(val)));
        self
    }

    pub fn target(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(BaseProp::target(val)));
        self
    }
}
