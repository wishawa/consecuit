use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlFontElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum FontProp {
    color(String),
    face(String),
    size(String),
}

impl ElementComponent for HtmlFontElement {
    type PropEnum = FontProp;
}
impl PropEnum<HtmlFontElement> for FontProp {
    fn unset_on(&self, elem: &HtmlFontElement) {
        match self {
            FontProp::color(_) => elem.remove_attribute("color").unwrap(),
            FontProp::face(_) => elem.remove_attribute("face").unwrap(),
            FontProp::size(_) => elem.remove_attribute("size").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlFontElement) {
        match self {
            FontProp::color(v) => elem.set_color(v),
            FontProp::face(v) => elem.set_face(v),
            FontProp::size(v) => elem.set_size(v),
        }
    }
}

impl ElementProps<HtmlFontElement> {
    pub fn color(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(FontProp::color(val)));
        self
    }

    pub fn face(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(FontProp::face(val)));
        self
    }

    pub fn size(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(FontProp::size(val)));
        self
    }
}
