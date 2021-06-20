use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlHtmlElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HtmlProp {
    version(String),
}

impl ElementComponent for HtmlHtmlElement {
    type PropEnum = HtmlProp;
}
impl PropEnum<HtmlHtmlElement> for HtmlProp {
    fn unset_on(&self, elem: &HtmlHtmlElement) {
        match self {
            HtmlProp::version(_) => elem.remove_attribute("version").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlHtmlElement) {
        match self {
            HtmlProp::version(v) => elem.set_version(v),
        }
    }
}

impl ElementProps<HtmlHtmlElement> {
    pub fn version(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HtmlProp::version(val)));
        self
    }
}
