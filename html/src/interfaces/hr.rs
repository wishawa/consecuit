use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlHrElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum HrProp {
    align(String),
    color(String),
    no_shade(bool),
    size(String),
    width(String),
}

impl ElementComponent for HtmlHrElement {
    type PropEnum = HrProp;
}
impl PropEnum<HtmlHrElement> for HrProp {
    fn unset_on(&self, elem: &HtmlHrElement) {
        match self {
            HrProp::align(_) => elem.remove_attribute("align").unwrap(),
            HrProp::color(_) => elem.remove_attribute("color").unwrap(),
            HrProp::no_shade(_) => elem.remove_attribute("no_shade").unwrap(),
            HrProp::size(_) => elem.remove_attribute("size").unwrap(),
            HrProp::width(_) => elem.remove_attribute("width").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlHrElement) {
        match self {
            HrProp::align(v) => elem.set_align(v),
            HrProp::color(v) => elem.set_color(v),
            HrProp::no_shade(v) => elem.set_no_shade(*v),
            HrProp::size(v) => elem.set_size(v),
            HrProp::width(v) => elem.set_width(v),
        }
    }
}

impl ElementProps<HtmlHrElement> {
    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HrProp::align(val)));
        self
    }

    pub fn color(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HrProp::color(val)));
        self
    }

    pub fn no_shade(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(HrProp::no_shade(val)));
        self
    }

    pub fn size(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HrProp::size(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(HrProp::width(val)));
        self
    }
}
