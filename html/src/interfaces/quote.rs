use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlQuoteElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum QuoteProp {
    cite(String),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlQuoteElement {
    type PropEnum = QuoteProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlQuoteElement> for QuoteProp {
    fn unset_on(&self, elem: &HtmlQuoteElement) {
        match self {
            QuoteProp::cite(_) => elem.remove_attribute("cite").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlQuoteElement) {
        match self {
            QuoteProp::cite(v) => elem.set_cite(v),
        }
    }
}

impl HtmlProps<HtmlQuoteElement> {
    pub fn cite(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(QuoteProp::cite(val)));
        self
    }
}
