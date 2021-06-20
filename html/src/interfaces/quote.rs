use crate::elem::{ElementComponent, HtmlProp, HtmlProps, PropEnum};
use web_sys::HtmlQuoteElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum QuoteProp {
    cite(String),
}

impl ElementComponent for HtmlQuoteElement {
    type PropEnum = QuoteProp;
}
impl PropEnum<HtmlQuoteElement> for QuoteProp {
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
    pub fn cite(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(QuoteProp::cite(val)));
        self
    }
}
