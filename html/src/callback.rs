use std::{ops::Deref, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast};

#[derive(Clone)]
pub struct Callback(Rc<Closure<dyn Fn()>>);

impl PartialEq for Callback {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Callback {
    pub fn as_websys_function(&self) -> &Function {
        self.0.deref().as_ref().unchecked_ref()
    }
}
