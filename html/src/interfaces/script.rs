use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlScriptElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ScriptProp {
    src(String),
    r#type(String),
    no_module(bool),
    charset(String),
    r#async(bool),
    defer(bool),
    cross_origin(String),
    event(String),
    html_for(String),
    integrity(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlScriptElement {
    type PropEnum = ScriptProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlScriptElement> for ScriptProp {
    fn unset_on(&self, elem: &HtmlScriptElement) {
        match self {
            ScriptProp::src(_) => elem.remove_attribute("src").unwrap(),
            ScriptProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            ScriptProp::no_module(_) => elem.remove_attribute("no_module").unwrap(),
            ScriptProp::charset(_) => elem.remove_attribute("charset").unwrap(),
            ScriptProp::r#async(_) => elem.remove_attribute("async").unwrap(),
            ScriptProp::defer(_) => elem.remove_attribute("defer").unwrap(),
            ScriptProp::cross_origin(_) => elem.set_cross_origin(None),
            ScriptProp::event(_) => elem.remove_attribute("event").unwrap(),
            ScriptProp::html_for(_) => elem.remove_attribute("html_for").unwrap(),
            ScriptProp::integrity(_) => elem.remove_attribute("integrity").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlScriptElement) {
        match self {
            ScriptProp::src(v) => elem.set_src(v),
            ScriptProp::r#type(v) => elem.set_type(v),
            ScriptProp::no_module(v) => elem.set_no_module(*v),
            ScriptProp::charset(v) => elem.set_charset(v),
            ScriptProp::r#async(v) => elem.set_async(*v),
            ScriptProp::defer(v) => elem.set_defer(*v),
            ScriptProp::cross_origin(v) => elem.set_cross_origin(Some(v)),
            ScriptProp::event(v) => elem.set_event(v),
            ScriptProp::html_for(v) => elem.set_html_for(v),
            ScriptProp::integrity(v) => elem.set_integrity(v),
        }
    }
}

impl HtmlProps<HtmlScriptElement> {
    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::src(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::r#type(val)));
        self
    }

    pub fn no_module(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::no_module(val)));
        self
    }

    pub fn charset(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::charset(val)));
        self
    }

    pub fn r#async(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::r#async(val)));
        self
    }

    pub fn defer(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::defer(val)));
        self
    }

    pub fn cross_origin(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(ScriptProp::cross_origin(val)));
        self
    }

    pub fn event(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::event(val)));
        self
    }

    pub fn html_for(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::html_for(val)));
        self
    }

    pub fn integrity(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ScriptProp::integrity(val)));
        self
    }
}
