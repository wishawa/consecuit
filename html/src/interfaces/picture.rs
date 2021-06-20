use crate::elem::HtmlProps;
use web_sys::HtmlPictureElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum PictureProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlPictureElement {
    type PropEnum = PictureProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlPictureElement> for PictureProp {
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
