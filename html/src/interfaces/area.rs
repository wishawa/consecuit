use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlAreaElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum AreaProp {
    alt(String),
    coords(String),
    shape(String),
    target(String),
    download(String),
    ping(String),
    rel(String),
    referrer_policy(String),
    no_href(bool),
    href(String),
    protocol(String),
    username(String),
    password(String),
    host(String),
    hostname(String),
    port(String),
    pathname(String),
    search(String),
    hash(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlAreaElement {
    type PropEnum = AreaProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlAreaElement> for AreaProp {
    fn unset_on(&self, elem: &HtmlAreaElement) {
        match self {
            AreaProp::alt(_) => elem.remove_attribute("alt").unwrap(),
            AreaProp::coords(_) => elem.remove_attribute("coords").unwrap(),
            AreaProp::shape(_) => elem.remove_attribute("shape").unwrap(),
            AreaProp::target(_) => elem.remove_attribute("target").unwrap(),
            AreaProp::download(_) => elem.remove_attribute("download").unwrap(),
            AreaProp::ping(_) => elem.remove_attribute("ping").unwrap(),
            AreaProp::rel(_) => elem.remove_attribute("rel").unwrap(),
            AreaProp::referrer_policy(_) => elem.remove_attribute("referrer_policy").unwrap(),
            AreaProp::no_href(_) => elem.remove_attribute("no_href").unwrap(),
            AreaProp::href(_) => elem.remove_attribute("href").unwrap(),
            AreaProp::protocol(_) => elem.remove_attribute("protocol").unwrap(),
            AreaProp::username(_) => elem.remove_attribute("username").unwrap(),
            AreaProp::password(_) => elem.remove_attribute("password").unwrap(),
            AreaProp::host(_) => elem.remove_attribute("host").unwrap(),
            AreaProp::hostname(_) => elem.remove_attribute("hostname").unwrap(),
            AreaProp::port(_) => elem.remove_attribute("port").unwrap(),
            AreaProp::pathname(_) => elem.remove_attribute("pathname").unwrap(),
            AreaProp::search(_) => elem.remove_attribute("search").unwrap(),
            AreaProp::hash(_) => elem.remove_attribute("hash").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlAreaElement) {
        match self {
            AreaProp::alt(v) => elem.set_alt(v),
            AreaProp::coords(v) => elem.set_coords(v),
            AreaProp::shape(v) => elem.set_shape(v),
            AreaProp::target(v) => elem.set_target(v),
            AreaProp::download(v) => elem.set_download(v),
            AreaProp::ping(v) => elem.set_ping(v),
            AreaProp::rel(v) => elem.set_rel(v),
            AreaProp::referrer_policy(v) => elem.set_referrer_policy(v),
            AreaProp::no_href(v) => elem.set_no_href(*v),
            AreaProp::href(v) => elem.set_href(v),
            AreaProp::protocol(v) => elem.set_protocol(v),
            AreaProp::username(v) => elem.set_username(v),
            AreaProp::password(v) => elem.set_password(v),
            AreaProp::host(v) => elem.set_host(v),
            AreaProp::hostname(v) => elem.set_hostname(v),
            AreaProp::port(v) => elem.set_port(v),
            AreaProp::pathname(v) => elem.set_pathname(v),
            AreaProp::search(v) => elem.set_search(v),
            AreaProp::hash(v) => elem.set_hash(v),
        }
    }
}

impl HtmlProps<HtmlAreaElement> {
    pub fn alt(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::alt(val)));
        self
    }

    pub fn coords(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::coords(val)));
        self
    }

    pub fn shape(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::shape(val)));
        self
    }

    pub fn target(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::target(val)));
        self
    }

    pub fn download(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::download(val)));
        self
    }

    pub fn ping(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::ping(val)));
        self
    }

    pub fn rel(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::rel(val)));
        self
    }

    pub fn referrer_policy(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(AreaProp::referrer_policy(val)));
        self
    }

    pub fn no_href(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::no_href(val)));
        self
    }

    pub fn href(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::href(val)));
        self
    }

    pub fn protocol(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::protocol(val)));
        self
    }

    pub fn username(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::username(val)));
        self
    }

    pub fn password(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::password(val)));
        self
    }

    pub fn host(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::host(val)));
        self
    }

    pub fn hostname(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::hostname(val)));
        self
    }

    pub fn port(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::port(val)));
        self
    }

    pub fn pathname(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::pathname(val)));
        self
    }

    pub fn search(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::search(val)));
        self
    }

    pub fn hash(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(AreaProp::hash(val)));
        self
    }
}
