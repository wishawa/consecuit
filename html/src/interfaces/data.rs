use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlDataElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DataProp {
    value(String),
}

impl ElementComponent for HtmlDataElement {
    type PropEnum = DataProp;
}
impl PropEnum<HtmlDataElement> for DataProp {
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
    pub fn value(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(DataProp::value(val)));
        self
    }
}
