use crate::elem::{ElementComponent, ElementProps, PropEnum};
use web_sys::HtmlSpanElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SpanProp {}

impl ElementComponent for HtmlSpanElement {
    type PropEnum = SpanProp;
}
impl PropEnum<HtmlSpanElement> for SpanProp {
    fn unset_on(&self, _elem: &HtmlSpanElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlSpanElement) {
        match self {
            _ => {}
        }
    }
}

impl ElementProps<HtmlSpanElement> {}
