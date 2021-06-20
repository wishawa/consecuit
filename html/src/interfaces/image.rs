use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlImageElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ImageProp {
    alt(String),
    src(String),
    srcset(String),
    cross_origin(String),
    use_map(String),
    referrer_policy(String),
    is_map(bool),
    width(u32),
    height(u32),
    decoding(String),
    name(String),
    align(String),
    hspace(u32),
    vspace(u32),
    long_desc(String),
    border(String),
    sizes(String),
}

impl ElementComponent for HtmlImageElement {
    type PropEnum = ImageProp;
}
impl PropEnum<HtmlImageElement> for ImageProp {
    fn unset_on(&self, elem: &HtmlImageElement) {
        match self {
            ImageProp::alt(_) => elem.remove_attribute("alt").unwrap(),
            ImageProp::src(_) => elem.remove_attribute("src").unwrap(),
            ImageProp::srcset(_) => elem.remove_attribute("srcset").unwrap(),
            ImageProp::cross_origin(_) => elem.set_cross_origin(None),
            ImageProp::use_map(_) => elem.remove_attribute("use_map").unwrap(),
            ImageProp::referrer_policy(_) => elem.remove_attribute("referrer_policy").unwrap(),
            ImageProp::is_map(_) => elem.remove_attribute("is_map").unwrap(),
            ImageProp::width(_) => elem.remove_attribute("width").unwrap(),
            ImageProp::height(_) => elem.remove_attribute("height").unwrap(),
            ImageProp::decoding(_) => elem.remove_attribute("decoding").unwrap(),
            ImageProp::name(_) => elem.remove_attribute("name").unwrap(),
            ImageProp::align(_) => elem.remove_attribute("align").unwrap(),
            ImageProp::hspace(_) => elem.remove_attribute("hspace").unwrap(),
            ImageProp::vspace(_) => elem.remove_attribute("vspace").unwrap(),
            ImageProp::long_desc(_) => elem.remove_attribute("long_desc").unwrap(),
            ImageProp::border(_) => elem.remove_attribute("border").unwrap(),
            ImageProp::sizes(_) => elem.remove_attribute("sizes").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlImageElement) {
        match self {
            ImageProp::alt(v) => elem.set_alt(v),
            ImageProp::src(v) => elem.set_src(v),
            ImageProp::srcset(v) => elem.set_srcset(v),
            ImageProp::cross_origin(v) => elem.set_cross_origin(Some(v)),
            ImageProp::use_map(v) => elem.set_use_map(v),
            ImageProp::referrer_policy(v) => elem.set_referrer_policy(v),
            ImageProp::is_map(v) => elem.set_is_map(*v),
            ImageProp::width(v) => elem.set_width(*v),
            ImageProp::height(v) => elem.set_height(*v),
            ImageProp::decoding(v) => elem.set_decoding(v),
            ImageProp::name(v) => elem.set_name(v),
            ImageProp::align(v) => elem.set_align(v),
            ImageProp::hspace(v) => elem.set_hspace(*v),
            ImageProp::vspace(v) => elem.set_vspace(*v),
            ImageProp::long_desc(v) => elem.set_long_desc(v),
            ImageProp::border(v) => elem.set_border(v),
            ImageProp::sizes(v) => elem.set_sizes(v),
        }
    }
}

impl ElementProps<HtmlImageElement> {
    pub fn alt(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::alt(val)));
        self
    }

    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::src(val)));
        self
    }

    pub fn srcset(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::srcset(val)));
        self
    }

    pub fn cross_origin(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(ImageProp::cross_origin(val)));
        self
    }

    pub fn use_map(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::use_map(val)));
        self
    }

    pub fn referrer_policy(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(ImageProp::referrer_policy(val)));
        self
    }

    pub fn is_map(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::is_map(val)));
        self
    }

    pub fn width(mut self, val: u32) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::width(val)));
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::height(val)));
        self
    }

    pub fn decoding(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::decoding(val)));
        self
    }

    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::name(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::align(val)));
        self
    }

    pub fn hspace(mut self, val: u32) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::hspace(val)));
        self
    }

    pub fn vspace(mut self, val: u32) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::vspace(val)));
        self
    }

    pub fn long_desc(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(ImageProp::long_desc(val)));
        self
    }

    pub fn border(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::border(val)));
        self
    }

    pub fn sizes(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(ImageProp::sizes(val)));
        self
    }
}
