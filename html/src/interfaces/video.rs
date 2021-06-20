use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlVideoElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum VideoProp {
    width(u32),
    height(u32),
    poster(String),
}

impl ElementComponent for HtmlVideoElement {
    type PropEnum = VideoProp;
}
impl PropEnum<HtmlVideoElement> for VideoProp {
    fn unset_on(&self, elem: &HtmlVideoElement) {
        match self {
            VideoProp::width(_) => elem.remove_attribute("width").unwrap(),
            VideoProp::height(_) => elem.remove_attribute("height").unwrap(),
            VideoProp::poster(_) => elem.remove_attribute("poster").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlVideoElement) {
        match self {
            VideoProp::width(v) => elem.set_width(*v),
            VideoProp::height(v) => elem.set_height(*v),
            VideoProp::poster(v) => elem.set_poster(v),
        }
    }
}

impl HtmlProps<HtmlVideoElement> {
    pub fn width(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(VideoProp::width(val)));
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(VideoProp::height(val)));
        self
    }

    pub fn poster(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(VideoProp::poster(val)));
        self
    }
}
