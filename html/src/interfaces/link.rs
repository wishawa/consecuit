use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlLinkElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LinkProp {
    disabled(bool),
    href(Cow<'static, str>),
    cross_origin(Cow<'static, str>),
    rel(Cow<'static, str>),
    media(Cow<'static, str>),
    hreflang(Cow<'static, str>),
    r#type(Cow<'static, str>),
    referrer_policy(Cow<'static, str>),
    charset(Cow<'static, str>),
    rev(Cow<'static, str>),
    target(Cow<'static, str>),
    integrity(Cow<'static, str>),
    r#as(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlLinkElement {
    type PropEnum = LinkProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlLinkElement> for LinkProp {
    fn unset_on(&self, elem: &HtmlLinkElement) {
        match self {
            LinkProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            LinkProp::href(_) => elem.remove_attribute("href").unwrap(),
            LinkProp::cross_origin(_) => elem.set_cross_origin(None),
            LinkProp::rel(_) => elem.remove_attribute("rel").unwrap(),
            LinkProp::media(_) => elem.remove_attribute("media").unwrap(),
            LinkProp::hreflang(_) => elem.remove_attribute("hreflang").unwrap(),
            LinkProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            LinkProp::referrer_policy(_) => elem.remove_attribute("referrer_policy").unwrap(),
            LinkProp::charset(_) => elem.remove_attribute("charset").unwrap(),
            LinkProp::rev(_) => elem.remove_attribute("rev").unwrap(),
            LinkProp::target(_) => elem.remove_attribute("target").unwrap(),
            LinkProp::integrity(_) => elem.remove_attribute("integrity").unwrap(),
            LinkProp::r#as(_) => elem.remove_attribute("as").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlLinkElement) {
        match self {
            LinkProp::disabled(v) => elem.set_disabled(*v),
            LinkProp::href(v) => elem.set_href(v),
            LinkProp::cross_origin(v) => elem.set_cross_origin(Some(v)),
            LinkProp::rel(v) => elem.set_rel(v),
            LinkProp::media(v) => elem.set_media(v),
            LinkProp::hreflang(v) => elem.set_hreflang(v),
            LinkProp::r#type(v) => elem.set_type(v),
            LinkProp::referrer_policy(v) => elem.set_referrer_policy(v),
            LinkProp::charset(v) => elem.set_charset(v),
            LinkProp::rev(v) => elem.set_rev(v),
            LinkProp::target(v) => elem.set_target(v),
            LinkProp::integrity(v) => elem.set_integrity(v),
            LinkProp::r#as(v) => elem.set_as(v),
        }
    }
}

impl HtmlProps<HtmlLinkElement> {
    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(LinkProp::disabled(val)));
        self
    }

    pub fn href(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::href(val)));
        self
    }

    pub fn cross_origin(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::cross_origin(val)));
        self
    }

    pub fn rel(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::rel(val)));
        self
    }

    pub fn media(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::media(val)));
        self
    }

    pub fn hreflang(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::hreflang(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(LinkProp::r#type(val)));
        self
    }

    pub fn referrer_policy(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(LinkProp::referrer_policy(val)));
        self
    }

    pub fn charset(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::charset(val)));
        self
    }

    pub fn rev(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::rev(val)));
        self
    }

    pub fn target(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::target(val)));
        self
    }

    pub fn integrity(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LinkProp::integrity(val)));
        self
    }

    pub fn r#as(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(LinkProp::r#as(val)));
        self
    }
}
