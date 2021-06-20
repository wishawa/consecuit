use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlDetailsElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DetailsProp {
    open(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDetailsElement {
    type PropEnum = DetailsProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDetailsElement> for DetailsProp {
    fn unset_on(&self, elem: &HtmlDetailsElement) {
        match self {
            DetailsProp::open(_) => elem.remove_attribute("open").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDetailsElement) {
        match self {
            DetailsProp::open(v) => elem.set_open(*v),
        }
    }
}

impl HtmlProps<HtmlDetailsElement> {
    pub fn open(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(DetailsProp::open(val)));
        self
    }
}
