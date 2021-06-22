use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlBrElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum BrProp {
    clear(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlBrElement {
    type PropEnum = BrProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlBrElement> for BrProp {
    fn unset_on(&self, elem: &HtmlBrElement) {
        match self {
            BrProp::clear(_) => elem.remove_attribute("clear").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlBrElement) {
        match self {
            BrProp::clear(v) => elem.set_clear(v),
        }
    }
}

impl HtmlProps<HtmlBrElement> {
    pub fn clear(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(BrProp::clear(val)));
        self
    }
}
