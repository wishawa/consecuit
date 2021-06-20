use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlTableCellElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableCellProp {
    col_span(u32),
    row_span(u32),
    headers(String),
    align(String),
    axis(String),
    height(String),
    width(String),
    ch(String),
    ch_off(String),
    no_wrap(bool),
    v_align(String),
    bg_color(String),
}

impl ElementComponent for HtmlTableCellElement {
    type PropEnum = TableCellProp;
}
impl PropEnum<HtmlTableCellElement> for TableCellProp {
    fn unset_on(&self, elem: &HtmlTableCellElement) {
        match self {
            TableCellProp::col_span(_) => elem.remove_attribute("col_span").unwrap(),
            TableCellProp::row_span(_) => elem.remove_attribute("row_span").unwrap(),
            TableCellProp::headers(_) => elem.remove_attribute("headers").unwrap(),
            TableCellProp::align(_) => elem.remove_attribute("align").unwrap(),
            TableCellProp::axis(_) => elem.remove_attribute("axis").unwrap(),
            TableCellProp::height(_) => elem.remove_attribute("height").unwrap(),
            TableCellProp::width(_) => elem.remove_attribute("width").unwrap(),
            TableCellProp::ch(_) => elem.remove_attribute("ch").unwrap(),
            TableCellProp::ch_off(_) => elem.remove_attribute("ch_off").unwrap(),
            TableCellProp::no_wrap(_) => elem.remove_attribute("no_wrap").unwrap(),
            TableCellProp::v_align(_) => elem.remove_attribute("v_align").unwrap(),
            TableCellProp::bg_color(_) => elem.remove_attribute("bg_color").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableCellElement) {
        match self {
            TableCellProp::col_span(v) => elem.set_col_span(*v),
            TableCellProp::row_span(v) => elem.set_row_span(*v),
            TableCellProp::headers(v) => elem.set_headers(v),
            TableCellProp::align(v) => elem.set_align(v),
            TableCellProp::axis(v) => elem.set_axis(v),
            TableCellProp::height(v) => elem.set_height(v),
            TableCellProp::width(v) => elem.set_width(v),
            TableCellProp::ch(v) => elem.set_ch(v),
            TableCellProp::ch_off(v) => elem.set_ch_off(v),
            TableCellProp::no_wrap(v) => elem.set_no_wrap(*v),
            TableCellProp::v_align(v) => elem.set_v_align(v),
            TableCellProp::bg_color(v) => elem.set_bg_color(v),
        }
    }
}

impl HtmlProps<HtmlTableCellElement> {
    pub fn col_span(mut self, val: u32) -> Self {
        self.0
            .push_back(HtmlProp::Own(TableCellProp::col_span(val)));
        self
    }

    pub fn row_span(mut self, val: u32) -> Self {
        self.0
            .push_back(HtmlProp::Own(TableCellProp::row_span(val)));
        self
    }

    pub fn headers(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::headers(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::align(val)));
        self
    }

    pub fn axis(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::axis(val)));
        self
    }

    pub fn height(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::height(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::width(val)));
        self
    }

    pub fn ch(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::ch(val)));
        self
    }

    pub fn ch_off(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::ch_off(val)));
        self
    }

    pub fn no_wrap(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::no_wrap(val)));
        self
    }

    pub fn v_align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TableCellProp::v_align(val)));
        self
    }

    pub fn bg_color(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(TableCellProp::bg_color(val)));
        self
    }
}
