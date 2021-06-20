use crate::elem::HtmlProps;
use web_sys::HtmlSpanElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SpanProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlSpanElement {
    type PropEnum = SpanProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlSpanElement> for SpanProp {
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

impl HtmlProps<HtmlSpanElement> {}
