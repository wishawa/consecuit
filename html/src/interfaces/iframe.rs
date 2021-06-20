use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlIFrameElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum IFrameProp {
    src(String),
    srcdoc(String),
    name(String),
    allow_fullscreen(bool),
    allow_payment_request(bool),
    width(String),
    height(String),
    referrer_policy(String),
    align(String),
    scrolling(String),
    frame_border(String),
    long_desc(String),
    margin_height(String),
    margin_width(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlIFrameElement {
    type PropEnum = IFrameProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlIFrameElement> for IFrameProp {
    fn unset_on(&self, elem: &HtmlIFrameElement) {
        match self {
            IFrameProp::src(_) => elem.remove_attribute("src").unwrap(),
            IFrameProp::srcdoc(_) => elem.remove_attribute("srcdoc").unwrap(),
            IFrameProp::name(_) => elem.remove_attribute("name").unwrap(),
            IFrameProp::allow_fullscreen(_) => elem.remove_attribute("allow_fullscreen").unwrap(),
            IFrameProp::allow_payment_request(_) => {
                elem.remove_attribute("allow_payment_request").unwrap()
            }
            IFrameProp::width(_) => elem.remove_attribute("width").unwrap(),
            IFrameProp::height(_) => elem.remove_attribute("height").unwrap(),
            IFrameProp::referrer_policy(_) => elem.remove_attribute("referrer_policy").unwrap(),
            IFrameProp::align(_) => elem.remove_attribute("align").unwrap(),
            IFrameProp::scrolling(_) => elem.remove_attribute("scrolling").unwrap(),
            IFrameProp::frame_border(_) => elem.remove_attribute("frame_border").unwrap(),
            IFrameProp::long_desc(_) => elem.remove_attribute("long_desc").unwrap(),
            IFrameProp::margin_height(_) => elem.remove_attribute("margin_height").unwrap(),
            IFrameProp::margin_width(_) => elem.remove_attribute("margin_width").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlIFrameElement) {
        match self {
            IFrameProp::src(v) => elem.set_src(v),
            IFrameProp::srcdoc(v) => elem.set_srcdoc(v),
            IFrameProp::name(v) => elem.set_name(v),
            IFrameProp::allow_fullscreen(v) => elem.set_allow_fullscreen(*v),
            IFrameProp::allow_payment_request(v) => elem.set_allow_payment_request(*v),
            IFrameProp::width(v) => elem.set_width(v),
            IFrameProp::height(v) => elem.set_height(v),
            IFrameProp::referrer_policy(v) => elem.set_referrer_policy(v),
            IFrameProp::align(v) => elem.set_align(v),
            IFrameProp::scrolling(v) => elem.set_scrolling(v),
            IFrameProp::frame_border(v) => elem.set_frame_border(v),
            IFrameProp::long_desc(v) => elem.set_long_desc(v),
            IFrameProp::margin_height(v) => elem.set_margin_height(v),
            IFrameProp::margin_width(v) => elem.set_margin_width(v),
        }
    }
}

impl HtmlProps<HtmlIFrameElement> {
    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::src(val)));
        self
    }

    pub fn srcdoc(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::srcdoc(val)));
        self
    }

    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::name(val)));
        self
    }

    pub fn allow_fullscreen(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::allow_fullscreen(val)));
        self
    }

    pub fn allow_payment_request(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::allow_payment_request(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::width(val)));
        self
    }

    pub fn height(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::height(val)));
        self
    }

    pub fn referrer_policy(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::referrer_policy(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::align(val)));
        self
    }

    pub fn scrolling(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::scrolling(val)));
        self
    }

    pub fn frame_border(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::frame_border(val)));
        self
    }

    pub fn long_desc(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(IFrameProp::long_desc(val)));
        self
    }

    pub fn margin_height(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::margin_height(val)));
        self
    }

    pub fn margin_width(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(IFrameProp::margin_width(val)));
        self
    }
}
