use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlBrElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum BrProp {
    clear(String),
}

impl ElementComponent for HtmlBrElement {
    type PropEnum = BrProp;
}
impl PropEnum<HtmlBrElement> for BrProp {
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
    pub fn clear(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(BrProp::clear(val)));
        self
    }
}
