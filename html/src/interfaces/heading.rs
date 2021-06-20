use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlHeadingElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HeadingProp {
    align(String),
}

impl ElementComponent for HtmlHeadingElement {
    type PropEnum = HeadingProp;
}
impl PropEnum<HtmlHeadingElement> for HeadingProp {
    fn unset_on(&self, elem: &HtmlHeadingElement) {
        match self {
            HeadingProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlHeadingElement) {
        match self {
            HeadingProp::align(v) => elem.set_align(v),
        }
    }
}

impl ElementProps<HtmlHeadingElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HeadingProp::align(val)));
        self
    }
}
