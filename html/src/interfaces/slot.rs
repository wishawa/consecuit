use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlSlotElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SlotProp {
    name(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlSlotElement {
    type PropEnum = SlotProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlSlotElement> for SlotProp {
    fn unset_on(&self, elem: &HtmlSlotElement) {
        match self {
            SlotProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlSlotElement) {
        match self {
            SlotProp::name(v) => elem.set_name(v),
        }
    }
}

impl HtmlProps<HtmlSlotElement> {
    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(SlotProp::name(val)));
        self
    }
}
