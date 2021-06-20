use crate::elem::HtmlProps;
use web_sys::HtmlTitleElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TitleProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTitleElement {
    type PropEnum = TitleProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTitleElement> for TitleProp {
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

impl HtmlProps<HtmlTitleElement> {}
