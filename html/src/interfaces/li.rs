use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlLiElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LiProp {
    value(i32),
    r#type(String),
}

impl ElementComponent for HtmlLiElement {
    type PropEnum = LiProp;
}
impl PropEnum<HtmlLiElement> for LiProp {
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

impl ElementProps<HtmlLiElement> {
    pub fn value(mut self, val: i32) -> Self {
        self.0.push_back(ElementProp::Own(LiProp::value(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(LiProp::r#type(val)));
        self
    }
}
