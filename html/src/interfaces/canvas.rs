use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlCanvasElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum CanvasProp {
    width(u32),
    height(u32),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlCanvasElement {
    type PropEnum = CanvasProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlCanvasElement> for CanvasProp {
    fn unset_on(&self, elem: &HtmlCanvasElement) {
        match self {
            CanvasProp::width(_) => elem.remove_attribute("width").unwrap(),
            CanvasProp::height(_) => elem.remove_attribute("height").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlCanvasElement) {
        match self {
            CanvasProp::width(v) => elem.set_width(*v),
            CanvasProp::height(v) => elem.set_height(*v),
        }
    }
}

impl HtmlProps<HtmlCanvasElement> {
    pub fn width(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(CanvasProp::width(val)));
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(CanvasProp::height(val)));
        self
    }
}
