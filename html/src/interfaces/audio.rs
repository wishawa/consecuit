use crate::elem::{ElementComponent, HtmlProps, PropEnum};
use web_sys::HtmlAudioElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum AudioProp {}

impl ElementComponent for HtmlAudioElement {
    type PropEnum = AudioProp;
}
impl PropEnum<HtmlAudioElement> for AudioProp {
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
