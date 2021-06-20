use crate::{
    callback::Callback,
    elem::{ElementComponent, ElementProp, ElementProps, PropEnum},
};
use web_sys::HtmlBodyElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum BodyProp {
    text(String),
    link(String),
    v_link(String),
    a_link(String),
    bg_color(String),
    background(String),
    onafterprint(Callback),
    onbeforeprint(Callback),
    onbeforeunload(Callback),
    onhashchange(Callback),
    onlanguagechange(Callback),
    onmessage(Callback),
    onmessageerror(Callback),
    onoffline(Callback),
    ononline(Callback),
    onpagehide(Callback),
    onpageshow(Callback),
    onpopstate(Callback),
    onstorage(Callback),
    onunload(Callback),
}

impl ElementComponent for HtmlBodyElement {
    type PropEnum = BodyProp;
}
impl PropEnum<HtmlBodyElement> for BodyProp {
    fn unset_on(&self, elem: &HtmlBodyElement) {
        match self {
            BodyProp::text(_) => elem.remove_attribute("text").unwrap(),
            BodyProp::link(_) => elem.remove_attribute("link").unwrap(),
            BodyProp::v_link(_) => elem.remove_attribute("v_link").unwrap(),
            BodyProp::a_link(_) => elem.remove_attribute("a_link").unwrap(),
            BodyProp::bg_color(_) => elem.remove_attribute("bg_color").unwrap(),
            BodyProp::background(_) => elem.remove_attribute("background").unwrap(),
            BodyProp::onafterprint(_) => elem.set_onafterprint(None),
            BodyProp::onbeforeprint(_) => elem.set_onbeforeprint(None),
            BodyProp::onbeforeunload(_) => elem.set_onbeforeunload(None),
            BodyProp::onhashchange(_) => elem.set_onhashchange(None),
            BodyProp::onlanguagechange(_) => elem.set_onlanguagechange(None),
            BodyProp::onmessage(_) => elem.set_onmessage(None),
            BodyProp::onmessageerror(_) => elem.set_onmessageerror(None),
            BodyProp::onoffline(_) => elem.set_onoffline(None),
            BodyProp::ononline(_) => elem.set_ononline(None),
            BodyProp::onpagehide(_) => elem.set_onpagehide(None),
            BodyProp::onpageshow(_) => elem.set_onpageshow(None),
            BodyProp::onpopstate(_) => elem.set_onpopstate(None),
            BodyProp::onstorage(_) => elem.set_onstorage(None),
            BodyProp::onunload(_) => elem.set_onunload(None),
        }
    }

    fn set_on(&self, elem: &HtmlBodyElement) {
        match self {
            BodyProp::text(v) => elem.set_text(v),
            BodyProp::link(v) => elem.set_link(v),
            BodyProp::v_link(v) => elem.set_v_link(v),
            BodyProp::a_link(v) => elem.set_a_link(v),
            BodyProp::bg_color(v) => elem.set_bg_color(v),
            BodyProp::background(v) => elem.set_background(v),
            BodyProp::onafterprint(v) => elem.set_onafterprint(Some(v.as_websys_function())),
            BodyProp::onbeforeprint(v) => elem.set_onbeforeprint(Some(v.as_websys_function())),
            BodyProp::onbeforeunload(v) => elem.set_onbeforeunload(Some(v.as_websys_function())),
            BodyProp::onhashchange(v) => elem.set_onhashchange(Some(v.as_websys_function())),
            BodyProp::onlanguagechange(v) => {
                elem.set_onlanguagechange(Some(v.as_websys_function()))
            }
            BodyProp::onmessage(v) => elem.set_onmessage(Some(v.as_websys_function())),
            BodyProp::onmessageerror(v) => elem.set_onmessageerror(Some(v.as_websys_function())),
            BodyProp::onoffline(v) => elem.set_onoffline(Some(v.as_websys_function())),
            BodyProp::ononline(v) => elem.set_ononline(Some(v.as_websys_function())),
            BodyProp::onpagehide(v) => elem.set_onpagehide(Some(v.as_websys_function())),
            BodyProp::onpageshow(v) => elem.set_onpageshow(Some(v.as_websys_function())),
            BodyProp::onpopstate(v) => elem.set_onpopstate(Some(v.as_websys_function())),
            BodyProp::onstorage(v) => elem.set_onstorage(Some(v.as_websys_function())),
            BodyProp::onunload(v) => elem.set_onunload(Some(v.as_websys_function())),
        }
    }
}

impl ElementProps<HtmlBodyElement> {
    pub fn text(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::text(val)));
        self
    }

    pub fn link(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::link(val)));
        self
    }

    pub fn v_link(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::v_link(val)));
        self
    }

    pub fn a_link(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::a_link(val)));
        self
    }

    pub fn bg_color(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::bg_color(val)));
        self
    }

    pub fn background(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::background(val)));
        self
    }

    pub fn onafterprint(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onafterprint(val)));
        self
    }

    pub fn onbeforeprint(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onbeforeprint(val)));
        self
    }

    pub fn onbeforeunload(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onbeforeunload(val)));
        self
    }

    pub fn onhashchange(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onhashchange(val)));
        self
    }

    pub fn onlanguagechange(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onlanguagechange(val)));
        self
    }

    pub fn onmessage(mut self, val: Callback) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::onmessage(val)));
        self
    }

    pub fn onmessageerror(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onmessageerror(val)));
        self
    }

    pub fn onoffline(mut self, val: Callback) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::onoffline(val)));
        self
    }

    pub fn ononline(mut self, val: Callback) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::ononline(val)));
        self
    }

    pub fn onpagehide(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onpagehide(val)));
        self
    }

    pub fn onpageshow(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onpageshow(val)));
        self
    }

    pub fn onpopstate(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(BodyProp::onpopstate(val)));
        self
    }

    pub fn onstorage(mut self, val: Callback) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::onstorage(val)));
        self
    }

    pub fn onunload(mut self, val: Callback) -> Self {
        self.0.push_back(ElementProp::Own(BodyProp::onunload(val)));
        self
    }
}
