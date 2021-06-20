use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlLegendElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LegendProp {
    align(String),
}

impl ElementComponent for HtmlLegendElement {
    type PropEnum = LegendProp;
}
impl PropEnum<HtmlLegendElement> for LegendProp {
    fn unset_on(&self, elem: &HtmlLegendElement) {
        match self {
            LegendProp::align(_) => elem.remove_attribute("align").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlLegendElement) {
        match self {
            LegendProp::align(v) => elem.set_align(v),
        }
    }
}

impl ElementProps<HtmlLegendElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(LegendProp::align(val)));
        self
    }
}
