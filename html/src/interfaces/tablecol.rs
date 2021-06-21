use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlTableColElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableColProp {
    span(u32),
    align(String),
    ch(String),
    ch_off(String),
    v_align(String),
    width(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTableColElement {
    type PropEnum = TableColProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTableColElement> for TableColProp {
    fn unset_on(&self, elem: &HtmlTableColElement) {
        match self {
            TableColProp::span(_) => elem.remove_attribute("span").unwrap(),
            TableColProp::align(_) => elem.remove_attribute("align").unwrap(),
            TableColProp::ch(_) => elem.remove_attribute("ch").unwrap(),
            TableColProp::ch_off(_) => elem.remove_attribute("ch_off").unwrap(),
            TableColProp::v_align(_) => elem.remove_attribute("v_align").unwrap(),
            TableColProp::width(_) => elem.remove_attribute("width").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableColElement) {
        match self {
            TableColProp::span(v) => elem.set_span(*v),
            TableColProp::align(v) => elem.set_align(v),
            TableColProp::ch(v) => elem.set_ch(v),
            TableColProp::ch_off(v) => elem.set_ch_off(v),
            TableColProp::v_align(v) => elem.set_v_align(v),
            TableColProp::width(v) => elem.set_width(v),
        }
    }
}

impl HtmlProps<HtmlTableColElement> {
    pub fn span(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(TableColProp::span(val)));
        self
    }

    pub fn align(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableColProp::align(val)));
        self
    }

    pub fn ch(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableColProp::ch(val)));
        self
    }

    pub fn ch_off(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableColProp::ch_off(val)));
        self
    }

    pub fn v_align(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableColProp::v_align(val)));
        self
    }

    pub fn width(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableColProp::width(val)));
        self
    }
}
