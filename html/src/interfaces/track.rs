use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlTrackElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TrackProp {
    kind(String),
    src(String),
    srclang(String),
    label(String),
    default(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTrackElement {
    type PropEnum = TrackProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTrackElement> for TrackProp {
    fn unset_on(&self, elem: &HtmlTrackElement) {
        match self {
            TrackProp::kind(_) => elem.remove_attribute("kind").unwrap(),
            TrackProp::src(_) => elem.remove_attribute("src").unwrap(),
            TrackProp::srclang(_) => elem.remove_attribute("srclang").unwrap(),
            TrackProp::label(_) => elem.remove_attribute("label").unwrap(),
            TrackProp::default(_) => elem.remove_attribute("default").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTrackElement) {
        match self {
            TrackProp::kind(v) => elem.set_kind(v),
            TrackProp::src(v) => elem.set_src(v),
            TrackProp::srclang(v) => elem.set_srclang(v),
            TrackProp::label(v) => elem.set_label(v),
            TrackProp::default(v) => elem.set_default(*v),
        }
    }
}

impl HtmlProps<HtmlTrackElement> {
    pub fn kind(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TrackProp::kind(val)));
        self
    }

    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TrackProp::src(val)));
        self
    }

    pub fn srclang(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TrackProp::srclang(val)));
        self
    }

    pub fn label(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(TrackProp::label(val)));
        self
    }

    pub fn default(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(TrackProp::default(val)));
        self
    }
}
