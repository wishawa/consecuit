use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlObjectElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ObjectProp {
    data(String),
    r#type(String),
    type_must_match(bool),
    name(String),
    use_map(String),
    width(String),
    height(String),
    align(String),
    archive(String),
    code(String),
    declare(bool),
    hspace(u32),
    standby(String),
    vspace(u32),
    code_base(String),
    code_type(String),
    border(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlObjectElement {
    type PropEnum = ObjectProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlObjectElement> for ObjectProp {
    fn unset_on(&self, elem: &HtmlObjectElement) {
        match self {
            ObjectProp::data(_) => elem.remove_attribute("data").unwrap(),
            ObjectProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            ObjectProp::type_must_match(_) => elem.remove_attribute("type_must_match").unwrap(),
            ObjectProp::name(_) => elem.remove_attribute("name").unwrap(),
            ObjectProp::use_map(_) => elem.remove_attribute("use_map").unwrap(),
            ObjectProp::width(_) => elem.remove_attribute("width").unwrap(),
            ObjectProp::height(_) => elem.remove_attribute("height").unwrap(),
            ObjectProp::align(_) => elem.remove_attribute("align").unwrap(),
            ObjectProp::archive(_) => elem.remove_attribute("archive").unwrap(),
            ObjectProp::code(_) => elem.remove_attribute("code").unwrap(),
            ObjectProp::declare(_) => elem.remove_attribute("declare").unwrap(),
            ObjectProp::hspace(_) => elem.remove_attribute("hspace").unwrap(),
            ObjectProp::standby(_) => elem.remove_attribute("standby").unwrap(),
            ObjectProp::vspace(_) => elem.remove_attribute("vspace").unwrap(),
            ObjectProp::code_base(_) => elem.remove_attribute("code_base").unwrap(),
            ObjectProp::code_type(_) => elem.remove_attribute("code_type").unwrap(),
            ObjectProp::border(_) => elem.remove_attribute("border").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlObjectElement) {
        match self {
            ObjectProp::data(v) => elem.set_data(v),
            ObjectProp::r#type(v) => elem.set_type(v),
            ObjectProp::type_must_match(v) => elem.set_type_must_match(*v),
            ObjectProp::name(v) => elem.set_name(v),
            ObjectProp::use_map(v) => elem.set_use_map(v),
            ObjectProp::width(v) => elem.set_width(v),
            ObjectProp::height(v) => elem.set_height(v),
            ObjectProp::align(v) => elem.set_align(v),
            ObjectProp::archive(v) => elem.set_archive(v),
            ObjectProp::code(v) => elem.set_code(v),
            ObjectProp::declare(v) => elem.set_declare(*v),
            ObjectProp::hspace(v) => elem.set_hspace(*v),
            ObjectProp::standby(v) => elem.set_standby(v),
            ObjectProp::vspace(v) => elem.set_vspace(*v),
            ObjectProp::code_base(v) => elem.set_code_base(v),
            ObjectProp::code_type(v) => elem.set_code_type(v),
            ObjectProp::border(v) => elem.set_border(v),
        }
    }
}

impl HtmlProps<HtmlObjectElement> {
    pub fn data(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::data(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(ObjectProp::r#type(val)));
        self
    }

    pub fn type_must_match(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(ObjectProp::type_must_match(val)));
        self
    }

    pub fn name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::name(val)));
        self
    }

    pub fn use_map(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::use_map(val)));
        self
    }

    pub fn width(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::width(val)));
        self
    }

    pub fn height(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::height(val)));
        self
    }

    pub fn align(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::align(val)));
        self
    }

    pub fn archive(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::archive(val)));
        self
    }

    pub fn code(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::code(val)));
        self
    }

    pub fn declare(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(ObjectProp::declare(val)));
        self
    }

    pub fn hspace(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(ObjectProp::hspace(val)));
        self
    }

    pub fn standby(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::standby(val)));
        self
    }

    pub fn vspace(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(ObjectProp::vspace(val)));
        self
    }

    pub fn code_base(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::code_base(val)));
        self
    }

    pub fn code_type(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::code_type(val)));
        self
    }

    pub fn border(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ObjectProp::border(val)));
        self
    }
}
