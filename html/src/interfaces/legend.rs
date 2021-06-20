use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlLegendElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LegendProp {
    align(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlLegendElement {
    type PropEnum = LegendProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlLegendElement> for LegendProp {
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

impl HtmlProps<HtmlLegendElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(LegendProp::align(val)));
        self
    }
}
