use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlTableRowElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableRowProp {
    align(Cow<'static, str>),
    ch(Cow<'static, str>),
    ch_off(Cow<'static, str>),
    v_align(Cow<'static, str>),
    bg_color(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTableRowElement {
    type PropEnum = TableRowProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTableRowElement> for TableRowProp {
    fn unset_on(&self, elem: &HtmlTableRowElement) {
        match self {
            TableRowProp::align(_) => elem.remove_attribute("align").unwrap(),
            TableRowProp::ch(_) => elem.remove_attribute("ch").unwrap(),
            TableRowProp::ch_off(_) => elem.remove_attribute("ch_off").unwrap(),
            TableRowProp::v_align(_) => elem.remove_attribute("v_align").unwrap(),
            TableRowProp::bg_color(_) => elem.remove_attribute("bg_color").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableRowElement) {
        match self {
            TableRowProp::align(v) => elem.set_align(v),
            TableRowProp::ch(v) => elem.set_ch(v),
            TableRowProp::ch_off(v) => elem.set_ch_off(v),
            TableRowProp::v_align(v) => elem.set_v_align(v),
            TableRowProp::bg_color(v) => elem.set_bg_color(v),
        }
    }
}

impl HtmlProps<HtmlTableRowElement> {
    pub fn align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableRowProp::align(val)));
        self
    }

    pub fn ch(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableRowProp::ch(val)));
        self
    }

    pub fn ch_off(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableRowProp::ch_off(val)));
        self
    }

    pub fn v_align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableRowProp::v_align(val)));
        self
    }

    pub fn bg_color(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableRowProp::bg_color(val)));
        self
    }
}
