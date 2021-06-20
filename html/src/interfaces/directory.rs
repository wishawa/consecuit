use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlDirectoryElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DirectoryProp {
    compact(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlDirectoryElement {
    type PropEnum = DirectoryProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlDirectoryElement> for DirectoryProp {
    fn unset_on(&self, elem: &HtmlDirectoryElement) {
        match self {
            DirectoryProp::compact(_) => elem.remove_attribute("compact").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlDirectoryElement) {
        match self {
            DirectoryProp::compact(v) => elem.set_compact(*v),
        }
    }
}

impl HtmlProps<HtmlDirectoryElement> {
    pub fn compact(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(DirectoryProp::compact(val)));
        self
    }
}
