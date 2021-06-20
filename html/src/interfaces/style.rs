use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlStyleElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum StyleProp {
    disabled(bool),
    media(String),
    r#type(String),
}

impl ElementComponent for HtmlStyleElement {
    type PropEnum = StyleProp;
}
impl PropEnum<HtmlStyleElement> for StyleProp {
    fn unset_on(&self, elem: &HtmlStyleElement) {
        match self {
            StyleProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            StyleProp::media(_) => elem.remove_attribute("media").unwrap(),
            StyleProp::r#type(_) => elem.remove_attribute("type").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlStyleElement) {
        match self {
            StyleProp::disabled(v) => elem.set_disabled(*v),
            StyleProp::media(v) => elem.set_media(v),
            StyleProp::r#type(v) => elem.set_type(v),
        }
    }
}

impl ElementProps<HtmlStyleElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(StyleProp::disabled(val)));
        self
    }

    pub fn media(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(StyleProp::media(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(StyleProp::r#type(val)));
        self
    }
}
