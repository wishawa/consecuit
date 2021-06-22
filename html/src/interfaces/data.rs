use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlDataElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DataProp {
    value(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDataElement {
    type PropEnum = DataProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDataElement> for DataProp {
    fn unset_on(&self, elem: &HtmlDataElement) {
        match self {
            DataProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDataElement) {
        match self {
            DataProp::value(v) => elem.set_value(v),
        }
    }
}

impl HtmlProps<HtmlDataElement> {
    pub fn value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(DataProp::value(val)));
        self
    }
}
