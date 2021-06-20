use crate::{ElementComponent, ElementProps, PropEnum};
use web_sys::HtmlTemplateElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TemplateProp {}

impl ElementComponent for HtmlTemplateElement {
    type PropEnum = TemplateProp;
}
impl PropEnum<HtmlTemplateElement> for TemplateProp {
    fn unset_on(&self, _elem: &HtmlTemplateElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlTemplateElement) {
        match self {
            _ => {}
        }
    }
}

impl ElementProps<HtmlTemplateElement> {}
