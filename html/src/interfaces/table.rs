use crate::elem::{ElementComponent, ElementProp, ElementProps, PropEnum};
use web_sys::HtmlTableElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum TableProp {
    caption(web_sys::HtmlTableCaptionElement),
    t_head(web_sys::HtmlTableSectionElement),
    t_foot(web_sys::HtmlTableSectionElement),
    align(String),
    border(String),
    frame(String),
    rules(String),
    summary(String),
    width(String),
    bg_color(String),
    cell_padding(String),
    cell_spacing(String),
}

impl ElementComponent for HtmlTableElement {
    type PropEnum = TableProp;
}
impl PropEnum<HtmlTableElement> for TableProp {
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

impl ElementProps<HtmlTableElement> {
    pub fn caption(mut self, val: web_sys::HtmlTableCaptionElement) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::caption(val)));
        self
    }

    pub fn t_head(mut self, val: web_sys::HtmlTableSectionElement) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::t_head(val)));
        self
    }

    pub fn t_foot(mut self, val: web_sys::HtmlTableSectionElement) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::t_foot(val)));
        self
    }

    pub fn align(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::align(val)));
        self
    }

    pub fn border(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::border(val)));
        self
    }

    pub fn frame(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::frame(val)));
        self
    }

    pub fn rules(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::rules(val)));
        self
    }

    pub fn summary(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::summary(val)));
        self
    }

    pub fn width(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::width(val)));
        self
    }

    pub fn bg_color(mut self, val: String) -> Self {
        self.0.push_back(ElementProp::Own(TableProp::bg_color(val)));
        self
    }

    pub fn cell_padding(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableProp::cell_padding(val)));
        self
    }

    pub fn cell_spacing(mut self, val: String) -> Self {
        self.0
            .push_back(ElementProp::Own(TableProp::cell_spacing(val)));
        self
    }
}
