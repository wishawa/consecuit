use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlHeadingElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HeadingProp {
    align(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlHeadingElement {
    type PropEnum = HeadingProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlHeadingElement> for HeadingProp {
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

impl HtmlProps<HtmlHeadingElement> {
    pub fn align(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(HeadingProp::align(val)));
        self
    }
}
