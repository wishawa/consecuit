use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
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

impl ElementComponent for HtmlTableColElement {
    type PropEnum = TableColProp;
}
impl PropEnum<HtmlTableColElement> for TableColProp {
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

impl ElementProps<HtmlTableColElement> {
    pub fn span(mut self, val: u32) -> Self {
        self.0.push_back(ElementProp::Own(TableColProp::span(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableColProp::align(val)));
        self
    }

    pub fn ch(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableColProp::ch(val)));
        self
    }

    pub fn ch_off(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableColProp::ch_off(val)));
        self
    }

    pub fn v_align(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableColProp::v_align(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableColProp::width(val)));
        self
    }
}
