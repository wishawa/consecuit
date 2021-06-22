use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlMapElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MapProp {
    name(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlMapElement {
    type PropEnum = MapProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlMapElement> for MapProp {
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

impl HtmlProps<HtmlMapElement> {
    pub fn name(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(MapProp::name(val)));
        self
    }
}
