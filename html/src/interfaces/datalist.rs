use crate::elem::HtmlProps;
use web_sys::HtmlDataListElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DataListProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDataListElement {
    type PropEnum = DataListProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDataListElement> for DataListProp {
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

impl HtmlProps<HtmlDataListElement> {}
