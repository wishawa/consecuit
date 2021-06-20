use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlSlotElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum SlotProp {
    name(String),
}

impl ElementComponent for HtmlSlotElement {
    type PropEnum = SlotProp;
}
impl PropEnum<HtmlSlotElement> for SlotProp {
    fn unset_on(&self, elem: &HtmlSlotElement) {
        match self {
            SlotProp::name(_) => elem.remove_attribute("name").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlSlotElement) {
        match self {
            SlotProp::name(v) => elem.set_name(v),
        }
    }
}

impl ElementProps<HtmlSlotElement> {
    pub fn name(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(SlotProp::name(val)));
        self
    }
}
