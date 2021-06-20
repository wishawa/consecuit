use crate::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlDirectoryElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum DirectoryProp {
    compact(bool),
}

impl ElementComponent for HtmlDirectoryElement {
    type PropEnum = DirectoryProp;
}
impl PropEnum<HtmlDirectoryElement> for DirectoryProp {
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

impl ElementProps<HtmlDirectoryElement> {
    pub fn compact(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(DirectoryProp::compact(val)));
        self
    }
}
