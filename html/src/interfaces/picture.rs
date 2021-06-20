use crate::elem::{ElementComponent, HtmlProps, PropEnum};
use web_sys::HtmlPictureElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum PictureProp {}

impl ElementComponent for HtmlPictureElement {
    type PropEnum = PictureProp;
}
impl PropEnum<HtmlPictureElement> for PictureProp {
    fn unset_on(&self, _elem: &HtmlPictureElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlPictureElement) {
        match self {
            _ => {}
        }
    }
}

impl HtmlProps<HtmlPictureElement> {}
