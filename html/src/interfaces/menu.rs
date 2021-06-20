use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlMenuElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MenuProp {
    r#type(String),
    label(String),
    compact(bool),
}

impl ElementComponent for HtmlMenuElement {
    type PropEnum = MenuProp;
}
impl PropEnum<HtmlMenuElement> for MenuProp {
    fn unset_on(&self, elem: &HtmlMenuElement) {
        match self {
            MenuProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            MenuProp::label(_) => elem.remove_attribute("label").unwrap(),
            MenuProp::compact(_) => elem.remove_attribute("compact").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlMenuElement) {
        match self {
            MenuProp::r#type(v) => elem.set_type(v),
            MenuProp::label(v) => elem.set_label(v),
            MenuProp::compact(v) => elem.set_compact(*v),
        }
    }
}

impl ElementProps<HtmlMenuElement> {
    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MenuProp::r#type(val)));
        self
    }

    pub fn label(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MenuProp::label(val)));
        self
    }

    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(MenuProp::compact(val)));
        self
    }
}
