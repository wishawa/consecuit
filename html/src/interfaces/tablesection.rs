use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlTableSectionElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableSectionProp {
    align(String),
    ch(String),
    ch_off(String),
    v_align(String),
}

impl ElementComponent for HtmlTableSectionElement {
    type PropEnum = TableSectionProp;
}
impl PropEnum<HtmlTableSectionElement> for TableSectionProp {
    fn unset_on(&self, elem: &HtmlTableSectionElement) {
        match self {
            TableSectionProp::align(_) => elem.remove_attribute("align").unwrap(),
            TableSectionProp::ch(_) => elem.remove_attribute("ch").unwrap(),
            TableSectionProp::ch_off(_) => elem.remove_attribute("ch_off").unwrap(),
            TableSectionProp::v_align(_) => elem.remove_attribute("v_align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableSectionElement) {
        match self {
            TableSectionProp::align(v) => elem.set_align(v),
            TableSectionProp::ch(v) => elem.set_ch(v),
            TableSectionProp::ch_off(v) => elem.set_ch_off(v),
            TableSectionProp::v_align(v) => elem.set_v_align(v),
        }
    }
}

impl ElementProps<HtmlTableSectionElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableSectionProp::align(val)));
        self
    }

    pub fn ch(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableSectionProp::ch(val)));
        self
    }

    pub fn ch_off(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableSectionProp::ch_off(val)));
        self
    }

    pub fn v_align(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableSectionProp::v_align(val)));
        self
    }
}
