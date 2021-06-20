use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlProgressElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ProgressProp {
    value(f64),
    max(f64),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlProgressElement {
    type PropEnum = ProgressProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlProgressElement> for ProgressProp {
    fn unset_on(&self, elem: &HtmlProgressElement) {
        match self {
            ProgressProp::value(_) => elem.remove_attribute("value").unwrap(),
            ProgressProp::max(_) => elem.remove_attribute("max").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlProgressElement) {
        match self {
            ProgressProp::value(v) => elem.set_value(*v),
            ProgressProp::max(v) => elem.set_max(*v),
        }
    }
}

impl HtmlProps<HtmlProgressElement> {
    pub fn value(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(ProgressProp::value(val)));
        self
    }

    pub fn max(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(ProgressProp::max(val)));
        self
    }
}
