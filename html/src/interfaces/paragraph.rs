use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlParagraphElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ParagraphProp {
    align(String),
}

impl ElementComponent for HtmlParagraphElement {
    type PropEnum = ParagraphProp;
}
impl PropEnum<HtmlParagraphElement> for ParagraphProp {
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

impl ElementProps<HtmlParagraphElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(ParagraphProp::align(val)));
        self
    }
}
