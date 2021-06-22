use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlFrameElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FrameProp {
    name(Cow<'static, str>),
    scrolling(Cow<'static, str>),
    src(Cow<'static, str>),
    frame_border(Cow<'static, str>),
    long_desc(Cow<'static, str>),
    no_resize(bool),
    margin_height(Cow<'static, str>),
    margin_width(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlFrameElement {
    type PropEnum = FrameProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlFrameElement> for FrameProp {
    fn unset_on(&self, elem: &HtmlFrameElement) {
        match self {
            FrameProp::name(_) => elem.remove_attribute("name").unwrap(),
            FrameProp::scrolling(_) => elem.remove_attribute("scrolling").unwrap(),
            FrameProp::src(_) => elem.remove_attribute("src").unwrap(),
            FrameProp::frame_border(_) => elem.remove_attribute("frame_border").unwrap(),
            FrameProp::long_desc(_) => elem.remove_attribute("long_desc").unwrap(),
            FrameProp::no_resize(_) => elem.remove_attribute("no_resize").unwrap(),
            FrameProp::margin_height(_) => elem.remove_attribute("margin_height").unwrap(),
            FrameProp::margin_width(_) => elem.remove_attribute("margin_width").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlFrameElement) {
        match self {
            FrameProp::name(v) => elem.set_name(v),
            FrameProp::scrolling(v) => elem.set_scrolling(v),
            FrameProp::src(v) => elem.set_src(v),
            FrameProp::frame_border(v) => elem.set_frame_border(v),
            FrameProp::long_desc(v) => elem.set_long_desc(v),
            FrameProp::no_resize(v) => elem.set_no_resize(*v),
            FrameProp::margin_height(v) => elem.set_margin_height(v),
            FrameProp::margin_width(v) => elem.set_margin_width(v),
        }
    }
}

impl HtmlProps<HtmlFrameElement> {
    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameProp::name(val)));
        self
    }

    pub fn scrolling(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameProp::scrolling(val)));
        self
    }

    pub fn src(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameProp::src(val)));
        self
    }

    pub fn frame_border(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(FrameProp::frame_border(val)));
        self
    }

    pub fn long_desc(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameProp::long_desc(val)));
        self
    }

    pub fn no_resize(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(FrameProp::no_resize(val)));
        self
    }

    pub fn margin_height(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(FrameProp::margin_height(val)));
        self
    }

    pub fn margin_width(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(FrameProp::margin_width(val)));
        self
    }
}
