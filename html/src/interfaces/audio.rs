use crate::elem::HtmlProps;
use web_sys::HtmlAudioElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum AudioProp {}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlAudioElement {
    type PropEnum = AudioProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlAudioElement> for AudioProp {
    fn unset_on(&self, _elem: &HtmlAudioElement) {
        match self {
            _ => {}
        }
    }

    fn set_on(&self, _elem: &HtmlAudioElement) {
        match self {
            _ => {}
        }
    }
}

impl HtmlProps<HtmlAudioElement> {}
