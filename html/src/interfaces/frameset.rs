use crate::{
    callback::Callback,
    elem::{HtmlProp, HtmlProps},
};
use web_sys::HtmlFrameSetElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FrameSetProp {
    cols(String),
    rows(String),
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

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlFrameSetElement {
    type PropEnum = FrameSetProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlFrameSetElement> for FrameSetProp {
    fn unset_on(&self, elem: &HtmlFrameSetElement) {
        match self {
            FrameSetProp::cols(_) => elem.remove_attribute("cols").unwrap(),
            FrameSetProp::rows(_) => elem.remove_attribute("rows").unwrap(),
            FrameSetProp::onafterprint(_) => elem.set_onafterprint(None),
            FrameSetProp::onbeforeprint(_) => elem.set_onbeforeprint(None),
            FrameSetProp::onbeforeunload(_) => elem.set_onbeforeunload(None),
            FrameSetProp::onhashchange(_) => elem.set_onhashchange(None),
            FrameSetProp::onlanguagechange(_) => elem.set_onlanguagechange(None),
            FrameSetProp::onmessage(_) => elem.set_onmessage(None),
            FrameSetProp::onmessageerror(_) => elem.set_onmessageerror(None),
            FrameSetProp::onoffline(_) => elem.set_onoffline(None),
            FrameSetProp::ononline(_) => elem.set_ononline(None),
            FrameSetProp::onpagehide(_) => elem.set_onpagehide(None),
            FrameSetProp::onpageshow(_) => elem.set_onpageshow(None),
            FrameSetProp::onpopstate(_) => elem.set_onpopstate(None),
            FrameSetProp::onstorage(_) => elem.set_onstorage(None),
            FrameSetProp::onunload(_) => elem.set_onunload(None),
        }
    }

    fn set_on(&self, elem: &HtmlFrameSetElement) {
        match self {
            FrameSetProp::cols(v) => elem.set_cols(v),
            FrameSetProp::rows(v) => elem.set_rows(v),
            FrameSetProp::onafterprint(v) => elem.set_onafterprint(Some(v.as_websys_function())),
            FrameSetProp::onbeforeprint(v) => elem.set_onbeforeprint(Some(v.as_websys_function())),
            FrameSetProp::onbeforeunload(v) => {
                elem.set_onbeforeunload(Some(v.as_websys_function()))
            }
            FrameSetProp::onhashchange(v) => elem.set_onhashchange(Some(v.as_websys_function())),
            FrameSetProp::onlanguagechange(v) => {
                elem.set_onlanguagechange(Some(v.as_websys_function()))
            }
            FrameSetProp::onmessage(v) => elem.set_onmessage(Some(v.as_websys_function())),
            FrameSetProp::onmessageerror(v) => {
                elem.set_onmessageerror(Some(v.as_websys_function()))
            }
            FrameSetProp::onoffline(v) => elem.set_onoffline(Some(v.as_websys_function())),
            FrameSetProp::ononline(v) => elem.set_ononline(Some(v.as_websys_function())),
            FrameSetProp::onpagehide(v) => elem.set_onpagehide(Some(v.as_websys_function())),
            FrameSetProp::onpageshow(v) => elem.set_onpageshow(Some(v.as_websys_function())),
            FrameSetProp::onpopstate(v) => elem.set_onpopstate(Some(v.as_websys_function())),
            FrameSetProp::onstorage(v) => elem.set_onstorage(Some(v.as_websys_function())),
            FrameSetProp::onunload(v) => elem.set_onunload(Some(v.as_websys_function())),
        }
    }
}

impl HtmlProps<HtmlFrameSetElement> {
    pub fn cols(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameSetProp::cols(val)));
        self
    }

    pub fn rows(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(FrameSetProp::rows(val)));
        self
    }

    pub fn onafterprint(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onafterprint(val)));
        self
    }

    pub fn onbeforeprint(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onbeforeprint(val)));
        self
    }

    pub fn onbeforeunload(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onbeforeunload(val)));
        self
    }

    pub fn onhashchange(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onhashchange(val)));
        self
    }

    pub fn onlanguagechange(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onlanguagechange(val)));
        self
    }

    pub fn onmessage(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onmessage(val)));
        self
    }

    pub fn onmessageerror(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onmessageerror(val)));
        self
    }

    pub fn onoffline(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onoffline(val)));
        self
    }

    pub fn ononline(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Own(FrameSetProp::ononline(val)));
        self
    }

    pub fn onpagehide(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onpagehide(val)));
        self
    }

    pub fn onpageshow(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onpageshow(val)));
        self
    }

    pub fn onpopstate(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onpopstate(val)));
        self
    }

    pub fn onstorage(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Own(FrameSetProp::onstorage(val)));
        self
    }

    pub fn onunload(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Own(FrameSetProp::onunload(val)));
        self
    }
}
