use std::{ops::Deref, rc::Rc};

use js_sys::Function;
use reia::prelude::batch_updates;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};

#[derive(Clone)]
pub struct Callback<E: FromWasmAbi + 'static>(Rc<Closure<dyn Fn(E)>>);

impl<E: FromWasmAbi + 'static> PartialEq for Callback<E> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<E: FromWasmAbi + 'static> Callback<E> {
    pub fn new<F: Fn(E) + 'static>(f: F) -> Self {
        Self(Rc::new(Closure::wrap(Box::new(move |e: E| {
            batch_updates(|| {
                f(e);
            })
        }))))
    }
    pub fn as_websys_function(&self) -> &Function {
        self.0.deref().as_ref().unchecked_ref()
    }
}
