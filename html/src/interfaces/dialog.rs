use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlDialogElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DialogProp {
    open(bool),
    return_value(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDialogElement {
    type PropEnum = DialogProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDialogElement> for DialogProp {
    fn unset_on(&self, elem: &HtmlDialogElement) {
        match self {
            DialogProp::open(_) => elem.remove_attribute("open").unwrap(),
            DialogProp::return_value(_) => elem.remove_attribute("return_value").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDialogElement) {
        match self {
            DialogProp::open(v) => elem.set_open(*v),
            DialogProp::return_value(v) => elem.set_return_value(v),
        }
    }
}

impl HtmlProps<HtmlDialogElement> {
    pub fn open(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(DialogProp::open(val)));
        self
    }

    pub fn return_value(mut self, val: String) -> Self {
        self.0
            .push_back(HtmlProp::Own(DialogProp::return_value(val)));
        self
    }
}
