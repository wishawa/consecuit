use crate::elem::{HtmlProp, HtmlProps};
use std::borrow::Cow;
use web_sys::HtmlTableElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableProp {
    caption(web_sys::HtmlTableCaptionElement),
    t_head(web_sys::HtmlTableSectionElement),
    t_foot(web_sys::HtmlTableSectionElement),
    align(Cow<'static, str>),
    border(Cow<'static, str>),
    frame(Cow<'static, str>),
    rules(Cow<'static, str>),
    summary(Cow<'static, str>),
    width(Cow<'static, str>),
    bg_color(Cow<'static, str>),
    cell_padding(Cow<'static, str>),
    cell_spacing(Cow<'static, str>),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlTableElement {
    type PropEnum = TableProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlTableElement> for TableProp {
    fn unset_on(&self, elem: &HtmlTableElement) {
        match self {
            TableProp::caption(_) => elem.set_caption(None),
            TableProp::t_head(_) => elem.set_t_head(None),
            TableProp::t_foot(_) => elem.set_t_foot(None),
            TableProp::align(_) => elem.remove_attribute("align").unwrap(),
            TableProp::border(_) => elem.remove_attribute("border").unwrap(),
            TableProp::frame(_) => elem.remove_attribute("frame").unwrap(),
            TableProp::rules(_) => elem.remove_attribute("rules").unwrap(),
            TableProp::summary(_) => elem.remove_attribute("summary").unwrap(),
            TableProp::width(_) => elem.remove_attribute("width").unwrap(),
            TableProp::bg_color(_) => elem.remove_attribute("bg_color").unwrap(),
            TableProp::cell_padding(_) => elem.remove_attribute("cell_padding").unwrap(),
            TableProp::cell_spacing(_) => elem.remove_attribute("cell_spacing").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlTableElement) {
        match self {
            TableProp::caption(v) => elem.set_caption(Some(v)),
            TableProp::t_head(v) => elem.set_t_head(Some(v)),
            TableProp::t_foot(v) => elem.set_t_foot(Some(v)),
            TableProp::align(v) => elem.set_align(v),
            TableProp::border(v) => elem.set_border(v),
            TableProp::frame(v) => elem.set_frame(v),
            TableProp::rules(v) => elem.set_rules(v),
            TableProp::summary(v) => elem.set_summary(v),
            TableProp::width(v) => elem.set_width(v),
            TableProp::bg_color(v) => elem.set_bg_color(v),
            TableProp::cell_padding(v) => elem.set_cell_padding(v),
            TableProp::cell_spacing(v) => elem.set_cell_spacing(v),
        }
    }
}

impl HtmlProps<HtmlTableElement> {
    pub fn caption(mut self, val: web_sys::HtmlTableCaptionElement) -> Self {
        self.0.push_back(HtmlProp::Own(TableProp::caption(val)));
        self
    }

    pub fn t_head(mut self, val: web_sys::HtmlTableSectionElement) -> Self {
        self.0.push_back(HtmlProp::Own(TableProp::t_head(val)));
        self
    }

    pub fn t_foot(mut self, val: web_sys::HtmlTableSectionElement) -> Self {
        self.0.push_back(HtmlProp::Own(TableProp::t_foot(val)));
        self
    }

    pub fn align(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::align(val)));
        self
    }

    pub fn border(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::border(val)));
        self
    }

    pub fn frame(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::frame(val)));
        self
    }

    pub fn rules(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::rules(val)));
        self
    }

    pub fn summary(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::summary(val)));
        self
    }

    pub fn width(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::width(val)));
        self
    }

    pub fn bg_color(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(TableProp::bg_color(val)));
        self
    }

    pub fn cell_padding(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(TableProp::cell_padding(val)));
        self
    }

    pub fn cell_spacing(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(TableProp::cell_spacing(val)));
        self
    }
}
