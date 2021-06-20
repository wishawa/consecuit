use crate::{ElementComponent, ElementProps, PropEnum};
use web_sys::HtmlTitleElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TitleProp {}

impl ElementComponent for HtmlTitleElement {
    type PropEnum = TitleProp;
}
impl PropEnum<HtmlTitleElement> for TitleProp {
    fn unset_on(&self, _elem: &HtmlTitleElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlTitleElement) {
        match self {
            _ => {}
        }
    }
}

impl ElementProps<HtmlTitleElement> {}
