use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlMenuElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MenuProp {
    r#type(Cow<'static, str>),
    label(Cow<'static, str>),
    compact(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlMenuElement {
    type PropEnum = MenuProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlMenuElement> for MenuProp {
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

impl HtmlProps<HtmlMenuElement> {
    pub fn r#type(mut self, val: Cow<'static, str>) -> Self {
        self.0.push_back(HtmlProp::Own(MenuProp::r#type(val)));
        self
    }

    pub fn label(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MenuProp::label(val)));
        self
    }

    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(MenuProp::compact(val)));
        self
    }
}
