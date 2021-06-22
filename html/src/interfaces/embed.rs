use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlEmbedElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum EmbedProp {
    src(Cow<'static, str>),
    r#type(Cow<'static, str>),
    width(Cow<'static, str>),
    height(Cow<'static, str>),
    align(Cow<'static, str>),
    name(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlEmbedElement {
    type PropEnum = EmbedProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlEmbedElement> for EmbedProp {
    fn unset_on(&self, elem: &HtmlEmbedElement) {
        match self {
            EmbedProp::src(_) => elem.remove_attribute("src").unwrap(),
            EmbedProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            EmbedProp::width(_) => elem.remove_attribute("width").unwrap(),
            EmbedProp::height(_) => elem.remove_attribute("height").unwrap(),
            EmbedProp::align(_) => elem.remove_attribute("align").unwrap(),
            EmbedProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlEmbedElement) {
        match self {
            EmbedProp::src(v) => elem.set_src(v),
            EmbedProp::r#type(v) => elem.set_type(v),
            EmbedProp::width(v) => elem.set_width(v),
            EmbedProp::height(v) => elem.set_height(v),
            EmbedProp::align(v) => elem.set_align(v),
            EmbedProp::name(v) => elem.set_name(v),
        }
    }
}

impl HtmlProps<HtmlEmbedElement> {
    pub fn src(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(EmbedProp::src(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::r#type(val)));
        self
    }

    pub fn width(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(EmbedProp::width(val)));
        self
    }

    pub fn height(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(EmbedProp::height(val)));
        self
    }

    pub fn align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(EmbedProp::align(val)));
        self
    }

    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(EmbedProp::name(val)));
        self
    }
}
