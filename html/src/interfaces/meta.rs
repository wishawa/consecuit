use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlMetaElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MetaProp {
    name(String),
    http_equiv(String),
    content(String),
    scheme(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlMetaElement {
    type PropEnum = MetaProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlMetaElement> for MetaProp {
    fn unset_on(&self, elem: &HtmlMetaElement) {
        match self {
            MetaProp::name(_) => elem.remove_attribute("name").unwrap(),
            MetaProp::http_equiv(_) => elem.remove_attribute("http_equiv").unwrap(),
            MetaProp::content(_) => elem.remove_attribute("content").unwrap(),
            MetaProp::scheme(_) => elem.remove_attribute("scheme").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlMetaElement) {
        match self {
            MetaProp::name(v) => elem.set_name(v),
            MetaProp::http_equiv(v) => elem.set_http_equiv(v),
            MetaProp::content(v) => elem.set_content(v),
            MetaProp::scheme(v) => elem.set_scheme(v),
        }
    }
}

impl HtmlProps<HtmlMetaElement> {
    pub fn name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MetaProp::name(val)));
        self
    }

    pub fn http_equiv(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MetaProp::http_equiv(val)));
        self
    }

    pub fn content(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MetaProp::content(val)));
        self
    }

    pub fn scheme(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MetaProp::scheme(val)));
        self
    }
}
