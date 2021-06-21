use crate::elem::{HtmlProp as CrateHtmlProp, HtmlProps};
use web_sys::HtmlHtmlElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HtmlProp {
    version(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlHtmlElement {
    type PropEnum = HtmlProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlHtmlElement> for HtmlProp {
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

impl HtmlProps<HtmlHtmlElement> {
    pub fn version(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(CrateHtmlProp::Own(HtmlProp::version(val)));
        self
    }
}
