use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlMapElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MapProp {
    name(String),
}

impl ElementComponent for HtmlMapElement {
    type PropEnum = MapProp;
}
impl PropEnum<HtmlMapElement> for MapProp {
    fn unset_on(&self, elem: &HtmlMapElement) {
        match self {
            MapProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlMapElement) {
        match self {
            MapProp::name(v) => elem.set_name(v),
        }
    }
}

impl ElementProps<HtmlMapElement> {
    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MapProp::name(val)));
        self
    }
}
