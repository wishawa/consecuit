use crate::{callback::Callback, ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlMediaElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MediaProp {
    src(String),
    src_object(web_sys::MediaStream),
    cross_origin(String),
    preload(String),
    current_time(f64),
    default_playback_rate(f64),
    playback_rate(f64),
    autoplay(bool),
    r#loop(bool),
    controls(bool),
    volume(f64),
    muted(bool),
    default_muted(bool),
    onencrypted(Callback),
    onwaitingforkey(Callback),
}

impl ElementComponent for HtmlMediaElement {
    type PropEnum = MediaProp;
}
impl PropEnum<HtmlMediaElement> for MediaProp {
    fn unset_on(&self, elem: &HtmlMediaElement) {
        match self {
            MediaProp::src(_) => elem.remove_attribute("src").unwrap(),
            MediaProp::src_object(_) => elem.set_src_object(None),
            MediaProp::cross_origin(_) => elem.set_cross_origin(None),
            MediaProp::preload(_) => elem.remove_attribute("preload").unwrap(),
            MediaProp::current_time(_) => elem.remove_attribute("current_time").unwrap(),
            MediaProp::default_playback_rate(_) => {
                elem.remove_attribute("default_playback_rate").unwrap()
            }
            MediaProp::playback_rate(_) => elem.remove_attribute("playback_rate").unwrap(),
            MediaProp::autoplay(_) => elem.remove_attribute("autoplay").unwrap(),
            MediaProp::r#loop(_) => elem.remove_attribute("loop").unwrap(),
            MediaProp::controls(_) => elem.remove_attribute("controls").unwrap(),
            MediaProp::volume(_) => elem.remove_attribute("volume").unwrap(),
            MediaProp::muted(_) => elem.remove_attribute("muted").unwrap(),
            MediaProp::default_muted(_) => elem.remove_attribute("default_muted").unwrap(),
            MediaProp::onencrypted(_) => elem.set_onencrypted(None),
            MediaProp::onwaitingforkey(_) => elem.set_onwaitingforkey(None),
        }
    }

    fn set_on(&self, elem: &HtmlMediaElement) {
        match self {
            MediaProp::src(v) => elem.set_src(v),
            MediaProp::src_object(v) => elem.set_src_object(Some(v)),
            MediaProp::cross_origin(v) => elem.set_cross_origin(Some(v)),
            MediaProp::preload(v) => elem.set_preload(v),
            MediaProp::current_time(v) => elem.set_current_time(*v),
            MediaProp::default_playback_rate(v) => elem.set_default_playback_rate(*v),
            MediaProp::playback_rate(v) => elem.set_playback_rate(*v),
            MediaProp::autoplay(v) => elem.set_autoplay(*v),
            MediaProp::r#loop(v) => elem.set_loop(*v),
            MediaProp::controls(v) => elem.set_controls(*v),
            MediaProp::volume(v) => elem.set_volume(*v),
            MediaProp::muted(v) => elem.set_muted(*v),
            MediaProp::default_muted(v) => elem.set_default_muted(*v),
            MediaProp::onencrypted(v) => elem.set_onencrypted(Some(v.as_websys_function())),
            MediaProp::onwaitingforkey(v) => elem.set_onwaitingforkey(Some(v.as_websys_function())),
        }
    }
}

impl ElementProps<HtmlMediaElement> {
    pub fn src(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::src(val)));
        self
    }

    pub fn src_object(mut self, val: web_sys::MediaStream) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::src_object(val)));
        self
    }

    pub fn cross_origin(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::cross_origin(val)));
        self
    }

    pub fn preload(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::preload(val)));
        self
    }

    pub fn current_time(mut self, val: f64) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::current_time(val)));
        self
    }

    pub fn default_playback_rate(mut self, val: f64) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::default_playback_rate(val)));
        self
    }

    pub fn playback_rate(mut self, val: f64) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::playback_rate(val)));
        self
    }

    pub fn autoplay(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::autoplay(val)));
        self
    }

    pub fn r#loop(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::r#loop(val)));
        self
    }

    pub fn controls(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::controls(val)));
        self
    }

    pub fn volume(mut self, val: f64) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::volume(val)));
        self
    }

    pub fn muted(mut self, val: bool) -> Self {
        self.0.push_back(ElementProp::Own(MediaProp::muted(val)));
        self
    }

    pub fn default_muted(mut self, val: bool) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::default_muted(val)));
        self
    }

    pub fn onencrypted(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::onencrypted(val)));
        self
    }

    pub fn onwaitingforkey(mut self, val: Callback) -> Self {
        self.0
            .push_back(ElementProp::Own(MediaProp::onwaitingforkey(val)));
        self
    }
}
