use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlTableCaptionElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableCaptionProp {
    align(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTableCaptionElement {
    type PropEnum = TableCaptionProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTableCaptionElement> for TableCaptionProp {
    fn unset_on(&self, elem: &HtmlTableCaptionElement) {
        match self {
            TableCaptionProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableCaptionElement) {
        match self {
            TableCaptionProp::align(v) => elem.set_align(v),
        }
    }
}

impl HtmlProps<HtmlTableCaptionElement> {
    pub fn align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(TableCaptionProp::align(val)));
        self
    }
}
