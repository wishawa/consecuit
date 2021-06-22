use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlVideoElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum VideoProp {
    width(u32),
    height(u32),
    poster(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlVideoElement {
    type PropEnum = VideoProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlVideoElement> for VideoProp {
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

    pub fn poster(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(VideoProp::poster(val)));
        self
    }
}
