use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlMeterElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum MeterProp {
    value(f64),
    min(f64),
    max(f64),
    low(f64),
    high(f64),
    optimum(f64),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlMeterElement {
    type PropEnum = MeterProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlMeterElement> for MeterProp {
    fn unset_on(&self, elem: &HtmlMeterElement) {
        match self {
            MeterProp::value(_) => elem.remove_attribute("value").unwrap(),
            MeterProp::min(_) => elem.remove_attribute("min").unwrap(),
            MeterProp::max(_) => elem.remove_attribute("max").unwrap(),
            MeterProp::low(_) => elem.remove_attribute("low").unwrap(),
            MeterProp::high(_) => elem.remove_attribute("high").unwrap(),
            MeterProp::optimum(_) => elem.remove_attribute("optimum").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlMeterElement) {
        match self {
            MeterProp::value(v) => elem.set_value(*v),
            MeterProp::min(v) => elem.set_min(*v),
            MeterProp::max(v) => elem.set_max(*v),
            MeterProp::low(v) => elem.set_low(*v),
            MeterProp::high(v) => elem.set_high(*v),
            MeterProp::optimum(v) => elem.set_optimum(*v),
        }
    }
}

impl HtmlProps<HtmlMeterElement> {
    pub fn value(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::value(val)));
        self
    }

    pub fn min(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::min(val)));
        self
    }

    pub fn max(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::max(val)));
        self
    }

    pub fn low(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::low(val)));
        self
    }

    pub fn high(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::high(val)));
        self
    }

    pub fn optimum(mut self, val: f64) -> Self {
        self.0.push_back(HtmlProp::Own(MeterProp::optimum(val)));
        self
    }
}
