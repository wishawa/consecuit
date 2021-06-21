use crate::elem::{HtmlProp, HtmlProps};
use web_sys::HtmlInputElement;

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub enum InputProp {
    accept(String),
    alt(String),
    autocomplete(String),
    autofocus(bool),
    default_checked(bool),
    checked(bool),
    disabled(bool),
    files(web_sys::FileList),
    form_action(String),
    form_enctype(String),
    form_method(String),
    form_no_validate(bool),
    form_target(String),
    height(u32),
    indeterminate(bool),
    input_mode(String),
    max(String),
    max_length(i32),
    min(String),
    min_length(i32),
    multiple(bool),
    name(String),
    pattern(String),
    placeholder(String),
    read_only(bool),
    required(bool),
    size(u32),
    src(String),
    step(String),
    r#type(String),
    default_value(String),
    value(String),
    value_as_number(f64),
    width(u32),
    align(String),
    use_map(String),
    webkitdirectory(bool),
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlInputElement {
    type PropEnum = InputProp;
}
#[sealed::sealed]
impl crate::elem::PropEnum<HtmlInputElement> for InputProp {
    fn unset_on(&self, elem: &HtmlInputElement) {
        match self {
            InputProp::accept(_) => elem.remove_attribute("accept").unwrap(),
            InputProp::alt(_) => elem.remove_attribute("alt").unwrap(),
            InputProp::autocomplete(_) => elem.remove_attribute("autocomplete").unwrap(),
            InputProp::autofocus(_) => elem.remove_attribute("autofocus").unwrap(),
            InputProp::default_checked(_) => elem.remove_attribute("default_checked").unwrap(),
            InputProp::checked(_) => elem.remove_attribute("checked").unwrap(),
            InputProp::disabled(_) => elem.remove_attribute("disabled").unwrap(),
            InputProp::files(_) => elem.set_files(None),
            InputProp::form_action(_) => elem.remove_attribute("form_action").unwrap(),
            InputProp::form_enctype(_) => elem.remove_attribute("form_enctype").unwrap(),
            InputProp::form_method(_) => elem.remove_attribute("form_method").unwrap(),
            InputProp::form_no_validate(_) => elem.remove_attribute("form_no_validate").unwrap(),
            InputProp::form_target(_) => elem.remove_attribute("form_target").unwrap(),
            InputProp::height(_) => elem.remove_attribute("height").unwrap(),
            InputProp::indeterminate(_) => elem.remove_attribute("indeterminate").unwrap(),
            InputProp::input_mode(_) => elem.remove_attribute("input_mode").unwrap(),
            InputProp::max(_) => elem.remove_attribute("max").unwrap(),
            InputProp::max_length(_) => elem.remove_attribute("max_length").unwrap(),
            InputProp::min(_) => elem.remove_attribute("min").unwrap(),
            InputProp::min_length(_) => elem.remove_attribute("min_length").unwrap(),
            InputProp::multiple(_) => elem.remove_attribute("multiple").unwrap(),
            InputProp::name(_) => elem.remove_attribute("name").unwrap(),
            InputProp::pattern(_) => elem.remove_attribute("pattern").unwrap(),
            InputProp::placeholder(_) => elem.remove_attribute("placeholder").unwrap(),
            InputProp::read_only(_) => elem.remove_attribute("read_only").unwrap(),
            InputProp::required(_) => elem.remove_attribute("required").unwrap(),
            InputProp::size(_) => elem.remove_attribute("size").unwrap(),
            InputProp::src(_) => elem.remove_attribute("src").unwrap(),
            InputProp::step(_) => elem.remove_attribute("step").unwrap(),
            InputProp::r#type(_) => elem.remove_attribute("type").unwrap(),
            InputProp::default_value(_) => elem.remove_attribute("default_value").unwrap(),
            InputProp::value(_) => elem.remove_attribute("value").unwrap(),
            InputProp::value_as_number(_) => elem.remove_attribute("value_as_number").unwrap(),
            InputProp::width(_) => elem.remove_attribute("width").unwrap(),
            InputProp::align(_) => elem.remove_attribute("align").unwrap(),
            InputProp::use_map(_) => elem.remove_attribute("use_map").unwrap(),
            InputProp::webkitdirectory(_) => elem.remove_attribute("webkitdirectory").unwrap(),
        }
    }

    fn set_on(&self, elem: &HtmlInputElement) {
        match self {
            InputProp::accept(v) => elem.set_accept(v),
            InputProp::alt(v) => elem.set_alt(v),
            InputProp::autocomplete(v) => elem.set_autocomplete(v),
            InputProp::autofocus(v) => elem.set_autofocus(*v),
            InputProp::default_checked(v) => elem.set_default_checked(*v),
            InputProp::checked(v) => elem.set_checked(*v),
            InputProp::disabled(v) => elem.set_disabled(*v),
            InputProp::files(v) => elem.set_files(Some(v)),
            InputProp::form_action(v) => elem.set_form_action(v),
            InputProp::form_enctype(v) => elem.set_form_enctype(v),
            InputProp::form_method(v) => elem.set_form_method(v),
            InputProp::form_no_validate(v) => elem.set_form_no_validate(*v),
            InputProp::form_target(v) => elem.set_form_target(v),
            InputProp::height(v) => elem.set_height(*v),
            InputProp::indeterminate(v) => elem.set_indeterminate(*v),
            InputProp::input_mode(v) => elem.set_input_mode(v),
            InputProp::max(v) => elem.set_max(v),
            InputProp::max_length(v) => elem.set_max_length(*v),
            InputProp::min(v) => elem.set_min(v),
            InputProp::min_length(v) => elem.set_min_length(*v),
            InputProp::multiple(v) => elem.set_multiple(*v),
            InputProp::name(v) => elem.set_name(v),
            InputProp::pattern(v) => elem.set_pattern(v),
            InputProp::placeholder(v) => elem.set_placeholder(v),
            InputProp::read_only(v) => elem.set_read_only(*v),
            InputProp::required(v) => elem.set_required(*v),
            InputProp::size(v) => elem.set_size(*v),
            InputProp::src(v) => elem.set_src(v),
            InputProp::step(v) => elem.set_step(v),
            InputProp::r#type(v) => elem.set_type(v),
            InputProp::default_value(v) => elem.set_default_value(v),
            InputProp::value(v) => elem.set_value(v),
            InputProp::value_as_number(v) => elem.set_value_as_number(*v),
            InputProp::width(v) => elem.set_width(*v),
            InputProp::align(v) => elem.set_align(v),
            InputProp::use_map(v) => elem.set_use_map(v),
            InputProp::webkitdirectory(v) => elem.set_webkitdirectory(*v),
        }
    }
}

impl HtmlProps<HtmlInputElement> {
    pub fn accept(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::accept(val)));
        self
    }

    pub fn alt(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::alt(val)));
        self
    }

    pub fn autocomplete(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(InputProp::autocomplete(val)));
        self
    }

    pub fn autofocus(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::autofocus(val)));
        self
    }

    pub fn default_checked(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(InputProp::default_checked(val)));
        self
    }

    pub fn checked(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::checked(val)));
        self
    }

    pub fn disabled(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::disabled(val)));
        self
    }

    pub fn files(mut self, val: web_sys::FileList) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::files(val)));
        self
    }

    pub fn form_action(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::form_action(val)));
        self
    }

    pub fn form_enctype(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(InputProp::form_enctype(val)));
        self
    }

    pub fn form_method(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::form_method(val)));
        self
    }

    pub fn form_no_validate(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(InputProp::form_no_validate(val)));
        self
    }

    pub fn form_target(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::form_target(val)));
        self
    }

    pub fn height(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::height(val)));
        self
    }

    pub fn indeterminate(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(InputProp::indeterminate(val)));
        self
    }

    pub fn input_mode(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::input_mode(val)));
        self
    }

    pub fn max(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::max(val)));
        self
    }

    pub fn max_length(mut self, val: i32) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::max_length(val)));
        self
    }

    pub fn min(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::min(val)));
        self
    }

    pub fn min_length(mut self, val: i32) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::min_length(val)));
        self
    }

    pub fn multiple(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::multiple(val)));
        self
    }

    pub fn name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::name(val)));
        self
    }

    pub fn pattern(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::pattern(val)));
        self
    }

    pub fn placeholder(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::placeholder(val)));
        self
    }

    pub fn read_only(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::read_only(val)));
        self
    }

    pub fn required(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::required(val)));
        self
    }

    pub fn size(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::size(val)));
        self
    }

    pub fn src(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::src(val)));
        self
    }

    pub fn step(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::step(val)));
        self
    }

    pub fn r#type(mut self, val: String) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::r#type(val)));
        self
    }

    pub fn default_value(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Own(InputProp::default_value(val)));
        self
    }

    pub fn value(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::value(val)));
        self
    }

    pub fn value_as_number(mut self, val: f64) -> Self {
        self.0
            .push_back(HtmlProp::Own(InputProp::value_as_number(val)));
        self
    }

    pub fn width(mut self, val: u32) -> Self {
        self.0.push_back(HtmlProp::Own(InputProp::width(val)));
        self
    }

    pub fn align(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::align(val)));
        self
    }

    pub fn use_map(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Own(InputProp::use_map(val)));
        self
    }

    pub fn webkitdirectory(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Own(InputProp::webkitdirectory(val)));
        self
    }
}
