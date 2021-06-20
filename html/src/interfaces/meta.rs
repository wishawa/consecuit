use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlMetaElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MetaProp {
    name(String),
    http_equiv(String),
    content(String),
    scheme(String),
}

impl ElementComponent for HtmlMetaElement {
    type PropEnum = MetaProp;
}
impl PropEnum<HtmlMetaElement> for MetaProp {
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

impl ElementProps<HtmlMetaElement> {
    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MetaProp::name(val)));
        self
    }

    pub fn http_equiv(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(MetaProp::http_equiv(val)));
        self
    }

    pub fn content(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MetaProp::content(val)));
        self
    }

    pub fn scheme(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MetaProp::scheme(val)));
        self
    }
}
