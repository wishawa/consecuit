use crate::{ElementComponent, ElementProps, PropEnum};
use web_sys::HtmlHeadElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HeadProp {}

impl ElementComponent for HtmlHeadElement {
    type PropEnum = HeadProp;
}
impl PropEnum<HtmlHeadElement> for HeadProp {
    fn unset_on(&self, _elem: &HtmlHeadElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlHeadElement) {
        match self {
            _ => {}
        }
    }
}

impl ElementProps<HtmlHeadElement> {}
