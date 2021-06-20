use crate::elem::HtmlProps;
use web_sys::HtmlTemplateElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TemplateProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTemplateElement {
    type PropEnum = TemplateProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTemplateElement> for TemplateProp {
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

impl HtmlProps<HtmlTemplateElement> {}
