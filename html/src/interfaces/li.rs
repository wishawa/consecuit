use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlLiElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LiProp {
    value(i32),
    r#type(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlLiElement {
    type PropEnum = LiProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlLiElement> for LiProp {
    fn unset_on(&self, elem: &HtmlLiElement) {
        match self {
            LiProp::value(_) => elem.remove_attribute("value").unwrap(),
            LiProp::r#type(_) => elem.remove_attribute("type").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlLiElement) {
        match self {
            LiProp::value(v) => elem.set_value(*v),
            LiProp::r#type(v) => elem.set_type(v),
        }
    }
}

impl HtmlProps<HtmlLiElement> {
    pub fn value(mut self, val: i32) -> Self {
        self.0.push_back(HtmlProp::Own(LiProp::value(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(LiProp::r#type(val)));
        self
    }
}
