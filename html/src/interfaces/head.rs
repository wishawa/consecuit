use crate::elem::HtmlProps;
use web_sys::HtmlHeadElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HeadProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlHeadElement {
    type PropEnum = HeadProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlHeadElement> for HeadProp {
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

impl HtmlProps<HtmlHeadElement> {}
