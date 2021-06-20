use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlOutputElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum OutputProp {
    name(String),
    default_value(String),
    value(String),
}

impl ElementComponent for HtmlOutputElement {
    type PropEnum = OutputProp;
}
impl PropEnum<HtmlOutputElement> for OutputProp {
    fn unset_on(&self, elem: &HtmlOutputElement) {
        match self {
            OutputProp::name(_) => elem.remove_attribute("name").unwrap(),
            OutputProp::default_value(_) => elem.remove_attribute("default_value").unwrap(),
            OutputProp::value(_) => elem.remove_attribute("value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlOutputElement) {
        match self {
            OutputProp::name(v) => elem.set_name(v),
            OutputProp::default_value(v) => elem.set_default_value(v),
            OutputProp::value(v) => elem.set_value(v),
        }
    }
}

impl ElementProps<HtmlOutputElement> {
    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(OutputProp::name(val)));
        self
    }

    pub fn default_value(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(OutputProp::default_value(val)));
        self
    }

    pub fn value(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(OutputProp::value(val)));
        self
    }
}
