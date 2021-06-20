use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlMenuItemElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MenuItemProp {
    r#type(String),
    label(String),
    icon(String),
    disabled(bool),
    checked(bool),
    radiogroup(String),
    default_checked(bool),
}

impl ElementComponent for HtmlMenuItemElement {
    type PropEnum = MenuItemProp;
}
impl PropEnum<HtmlMenuItemElement> for MenuItemProp {
    fn unset_on(&self, elem: &HtmlMenuItemElement) {
        match self {
            MenuItemProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            MenuItemProp::label(_) => elem.remove_attribute("label").unwrap(),
            MenuItemProp::icon(_) => elem.remove_attribute("icon").unwrap(),
            MenuItemProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            MenuItemProp::checked(_) => elem.remove_attribute("checked").unwrap(),
            MenuItemProp::radiogroup(_) => elem.remove_attribute("radiogroup").unwrap(),
            MenuItemProp::default_checked(_) => elem.remove_attribute("default_checked").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlMenuItemElement) {
        match self {
            MenuItemProp::r#type(v) => elem.set_type(v),
            MenuItemProp::label(v) => elem.set_label(v),
            MenuItemProp::icon(v) => elem.set_icon(v),
            MenuItemProp::disabled(v) => elem.set_disabled(*v),
            MenuItemProp::checked(v) => elem.set_checked(*v),
            MenuItemProp::radiogroup(v) => elem.set_radiogroup(v),
            MenuItemProp::default_checked(v) => elem.set_default_checked(*v),
        }
    }
}

impl ElementProps<HtmlMenuItemElement> {
    pub fn r#type(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(MenuItemProp::r#type(val)));
        self
    }

    pub fn label(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MenuItemProp::label(val)));
        self
    }

    pub fn icon(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MenuItemProp::icon(val)));
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(MenuItemProp::disabled(val)));
        self
    }

    pub fn checked(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(MenuItemProp::checked(val)));
        self
    }

    pub fn radiogroup(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(MenuItemProp::radiogroup(val)));
        self
    }

    pub fn default_checked(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(MenuItemProp::default_checked(val)));
        self
    }
}
