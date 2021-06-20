use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlTimeElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TimeProp {
    date_time(String),
}

impl ElementComponent for HtmlTimeElement {
    type PropEnum = TimeProp;
}
impl PropEnum<HtmlTimeElement> for TimeProp {
    fn unset_on(&self, elem: &HtmlTimeElement) {
        match self {
            TimeProp::date_time(_) => elem.remove_attribute("date_time").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTimeElement) {
        match self {
            TimeProp::date_time(v) => elem.set_date_time(v),
        }
    }
}

impl HtmlProps<HtmlTimeElement> {
    pub fn date_time(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TimeProp::date_time(val)));
        self
    }
}
