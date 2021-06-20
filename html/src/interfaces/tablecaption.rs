use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlTableCaptionElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableCaptionProp {
    align(String),
}

impl ElementComponent for HtmlTableCaptionElement {
    type PropEnum = TableCaptionProp;
}
impl PropEnum<HtmlTableCaptionElement> for TableCaptionProp {
    fn unset_on(&self, elem: &HtmlTableCaptionElement) {
        match self {
            TableCaptionProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableCaptionElement) {
        match self {
            TableCaptionProp::align(v) => elem.set_align(v),
        }
    }
}

impl ElementProps<HtmlTableCaptionElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableCaptionProp::align(val)));
        self
    }
}
