use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlEmbedElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum EmbedProp {
    src(String),
    r#type(String),
    width(String),
    height(String),
    align(String),
    name(String),
}

impl ElementComponent for HtmlEmbedElement {
    type PropEnum = EmbedProp;
}
impl PropEnum<HtmlEmbedElement> for EmbedProp {
    fn unset_on(&self, elem: &HtmlEmbedElement) {
        match self {
            EmbedProp::src(_) => elem.remove_attribute("src").unwrap(),
            EmbedProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            EmbedProp::width(_) => elem.remove_attribute("width").unwrap(),
            EmbedProp::height(_) => elem.remove_attribute("height").unwrap(),
            EmbedProp::align(_) => elem.remove_attribute("align").unwrap(),
            EmbedProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlEmbedElement) {
        match self {
            EmbedProp::src(v) => elem.set_src(v),
            EmbedProp::r#type(v) => elem.set_type(v),
            EmbedProp::width(v) => elem.set_width(v),
            EmbedProp::height(v) => elem.set_height(v),
            EmbedProp::align(v) => elem.set_align(v),
            EmbedProp::name(v) => elem.set_name(v),
        }
    }
}

impl HtmlProps<HtmlEmbedElement> {
    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::src(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::r#type(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::width(val)));
        self
    }

    pub fn height(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::height(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::align(val)));
        self
    }

    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(EmbedProp::name(val)));
        self
    }
}
