use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlModElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum ModProp {
    cite(Cow<'static, str>),
    date_time(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlModElement {
    type PropEnum = ModProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlModElement> for ModProp {
    fn unset_on(&self, elem: &HtmlModElement) {
        match self {
            ModProp::cite(_) => elem.remove_attribute("cite").unwrap(),
            ModProp::date_time(_) => elem.remove_attribute("date_time").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlModElement) {
        match self {
            ModProp::cite(v) => elem.set_cite(v),
            ModProp::date_time(v) => elem.set_date_time(v),
        }
    }
}

impl HtmlProps<HtmlModElement> {
    pub fn cite(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ModProp::cite(val)));
        self
    }

    pub fn date_time(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(ModProp::date_time(val)));
        self
    }
}
