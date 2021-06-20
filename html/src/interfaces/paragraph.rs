use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlParagraphElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ParagraphProp {
    align(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlParagraphElement {
    type PropEnum = ParagraphProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlParagraphElement> for ParagraphProp {
    fn unset_on(&self, elem: &HtmlParagraphElement) {
        match self {
            ParagraphProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlParagraphElement) {
        match self {
            ParagraphProp::align(v) => elem.set_align(v),
        }
    }
}

impl HtmlProps<HtmlParagraphElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ParagraphProp::align(val)));
        self
    }
}
