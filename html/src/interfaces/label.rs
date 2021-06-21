use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlLabelElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LabelProp {
    html_for(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlLabelElement {
    type PropEnum = LabelProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlLabelElement> for LabelProp {
    fn unset_on(&self, elem: &HtmlLabelElement) {
        match self {
            LabelProp::html_for(_) => elem.remove_attribute("html_for").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlLabelElement) {
        match self {
            LabelProp::html_for(v) => elem.set_html_for(v),
        }
    }
}

impl HtmlProps<HtmlLabelElement> {
    pub fn html_for(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(LabelProp::html_for(val)));
        self
    }
}
