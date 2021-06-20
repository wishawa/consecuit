use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlLabelElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum LabelProp {
    html_for(String),
}

impl ElementComponent for HtmlLabelElement {
    type PropEnum = LabelProp;
}
impl PropEnum<HtmlLabelElement> for LabelProp {
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

impl ElementProps<HtmlLabelElement> {
    pub fn html_for(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(LabelProp::html_for(val)));
        self
    }
}
