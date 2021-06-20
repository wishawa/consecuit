use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlSourceElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SourceProp {
    src(String),
    r#type(String),
    srcset(String),
    sizes(String),
    media(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlSourceElement {
    type PropEnum = SourceProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlSourceElement> for SourceProp {
    fn unset_on(&self, elem: &HtmlSourceElement) {
        match self {
            SourceProp::src(_) => elem.remove_attribute("src").unwrap(),
            SourceProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            SourceProp::srcset(_) => elem.remove_attribute("srcset").unwrap(),
            SourceProp::sizes(_) => elem.remove_attribute("sizes").unwrap(),
            SourceProp::media(_) => elem.remove_attribute("media").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlSourceElement) {
        match self {
            SourceProp::src(v) => elem.set_src(v),
            SourceProp::r#type(v) => elem.set_type(v),
            SourceProp::srcset(v) => elem.set_srcset(v),
            SourceProp::sizes(v) => elem.set_sizes(v),
            SourceProp::media(v) => elem.set_media(v),
        }
    }
}

impl HtmlProps<HtmlSourceElement> {
    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(SourceProp::src(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(SourceProp::r#type(val)));
        self
    }

    pub fn srcset(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(SourceProp::srcset(val)));
        self
    }

    pub fn sizes(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(SourceProp::sizes(val)));
        self
    }

    pub fn media(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(SourceProp::media(val)));
        self
    }
}