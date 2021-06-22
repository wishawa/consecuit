use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlAnchorElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum AnchorProp {
    target(Cow<'static, str>),
    download(Cow<'static, str>),
    ping(Cow<'static, str>),
    rel(Cow<'static, str>),
    referrer_policy(Cow<'static, str>),
    hreflang(Cow<'static, str>),
    r#type(Cow<'static, str>),
    coords(Cow<'static, str>),
    charset(Cow<'static, str>),
    name(Cow<'static, str>),
    rev(Cow<'static, str>),
    shape(Cow<'static, str>),
    href(Cow<'static, str>),
    protocol(Cow<'static, str>),
    username(Cow<'static, str>),
    password(Cow<'static, str>),
    host(Cow<'static, str>),
    hostname(Cow<'static, str>),
    port(Cow<'static, str>),
    pathname(Cow<'static, str>),
    search(Cow<'static, str>),
    hash(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlAnchorElement {
    type PropEnum = AnchorProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlAnchorElement> for AnchorProp {
    fn unset_on(&self, elem: &HtmlAnchorElement) {
        match self {
            AnchorProp::target(_) => elem.remove_attribute("target").unwrap(),
            AnchorProp::download(_) => elem.remove_attribute("download").unwrap(),
            AnchorProp::ping(_) => elem.remove_attribute("ping").unwrap(),
            AnchorProp::rel(_) => elem.remove_attribute("rel").unwrap(),
            AnchorProp::referrer_policy(_) => elem.remove_attribute("referrer_policy").unwrap(),
            AnchorProp::hreflang(_) => elem.remove_attribute("hreflang").unwrap(),
            AnchorProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            AnchorProp::coords(_) => elem.remove_attribute("coords").unwrap(),
            AnchorProp::charset(_) => elem.remove_attribute("charset").unwrap(),
            AnchorProp::name(_) => elem.remove_attribute("name").unwrap(),
            AnchorProp::rev(_) => elem.remove_attribute("rev").unwrap(),
            AnchorProp::shape(_) => elem.remove_attribute("shape").unwrap(),
            AnchorProp::href(_) => elem.remove_attribute("href").unwrap(),
            AnchorProp::protocol(_) => elem.remove_attribute("protocol").unwrap(),
            AnchorProp::username(_) => elem.remove_attribute("username").unwrap(),
            AnchorProp::password(_) => elem.remove_attribute("password").unwrap(),
            AnchorProp::host(_) => elem.remove_attribute("host").unwrap(),
            AnchorProp::hostname(_) => elem.remove_attribute("hostname").unwrap(),
            AnchorProp::port(_) => elem.remove_attribute("port").unwrap(),
            AnchorProp::pathname(_) => elem.remove_attribute("pathname").unwrap(),
            AnchorProp::search(_) => elem.remove_attribute("search").unwrap(),
            AnchorProp::hash(_) => elem.remove_attribute("hash").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlAnchorElement) {
        match self {
            AnchorProp::target(v) => elem.set_target(v),
            AnchorProp::download(v) => elem.set_download(v),
            AnchorProp::ping(v) => elem.set_ping(v),
            AnchorProp::rel(v) => elem.set_rel(v),
            AnchorProp::referrer_policy(v) => elem.set_referrer_policy(v),
            AnchorProp::hreflang(v) => elem.set_hreflang(v),
            AnchorProp::r#type(v) => elem.set_type(v),
            AnchorProp::coords(v) => elem.set_coords(v),
            AnchorProp::charset(v) => elem.set_charset(v),
            AnchorProp::name(v) => elem.set_name(v),
            AnchorProp::rev(v) => elem.set_rev(v),
            AnchorProp::shape(v) => elem.set_shape(v),
            AnchorProp::href(v) => elem.set_href(v),
            AnchorProp::protocol(v) => elem.set_protocol(v),
            AnchorProp::username(v) => elem.set_username(v),
            AnchorProp::password(v) => elem.set_password(v),
            AnchorProp::host(v) => elem.set_host(v),
            AnchorProp::hostname(v) => elem.set_hostname(v),
            AnchorProp::port(v) => elem.set_port(v),
            AnchorProp::pathname(v) => elem.set_pathname(v),
            AnchorProp::search(v) => elem.set_search(v),
            AnchorProp::hash(v) => elem.set_hash(v),
        }
    }
}

impl HtmlProps<HtmlAnchorElement> {
    pub fn target(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::target(val)));
        self
    }

    pub fn download(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::download(val)));
        self
    }

    pub fn ping(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::ping(val)));
        self
    }

    pub fn rel(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::rel(val)));
        self
    }

    pub fn referrer_policy(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(AnchorProp::referrer_policy(val)));
        self
    }

    pub fn hreflang(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::hreflang(val)));
        self
    }

    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(AnchorProp::r#type(val)));
        self
    }

    pub fn coords(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::coords(val)));
        self
    }

    pub fn charset(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::charset(val)));
        self
    }

    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::name(val)));
        self
    }

    pub fn rev(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::rev(val)));
        self
    }

    pub fn shape(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::shape(val)));
        self
    }

    pub fn href(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::href(val)));
        self
    }

    pub fn protocol(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::protocol(val)));
        self
    }

    pub fn username(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::username(val)));
        self
    }

    pub fn password(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::password(val)));
        self
    }

    pub fn host(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::host(val)));
        self
    }

    pub fn hostname(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::hostname(val)));
        self
    }

    pub fn port(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::port(val)));
        self
    }

    pub fn pathname(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::pathname(val)));
        self
    }

    pub fn search(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::search(val)));
        self
    }

    pub fn hash(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(AnchorProp::hash(val)));
        self
    }
}
