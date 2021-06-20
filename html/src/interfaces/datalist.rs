use crate::{ElementComponent, ElementProps, PropEnum};
use web_sys::HtmlDataListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DataListProp {}

impl ElementComponent for HtmlDataListElement {
    type PropEnum = DataListProp;
}
impl PropEnum<HtmlDataListElement> for DataListProp {
    fn unset_on(&self, _elem: &HtmlDataListElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlDataListElement) {
        match self {
            _ => {}
        }
    }
}

impl ElementProps<HtmlDataListElement> {}
