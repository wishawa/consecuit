use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlProgressElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ProgressProp {
    value(f64),
    max(f64),
}

impl ElementComponent for HtmlProgressElement {
    type PropEnum = ProgressProp;
}
impl PropEnum<HtmlProgressElement> for ProgressProp {
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

impl ElementProps<HtmlProgressElement> {
    pub fn value(mut self, val: f64) -> Self {
        self.0.push_back(ElementProp::Own(ProgressProp::value(val)));
        self
    }

    pub fn max(mut self, val: f64) -> Self {
        self.0.push_back(ElementProp::Own(ProgressProp::max(val)));
        self
    }
}
