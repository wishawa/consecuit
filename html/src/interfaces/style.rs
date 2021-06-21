use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlStyleElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum StyleProp {
    disabled(bool),
    media(String),
    r#type(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlStyleElement {
    type PropEnum = StyleProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlStyleElement> for StyleProp {
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

impl HtmlProps<HtmlStyleElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(StyleProp::disabled(val)));
        self
    }

    pub fn media(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(StyleProp::media(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(StyleProp::r#type(val)));
        self
    }
}
