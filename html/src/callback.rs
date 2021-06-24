use std::{ops::Deref, rc::Rc};

use consecuit::prelude::batch_updates;
use js_sys::Function;
use wasm_bindgen::{convert::FromWasmAbi, prelude::Closure, JsCast};

/** A callback for HTML attributes like `onclick`, `oninput`, etc.

Create one with `Callback::new`, and pass it to the prop builder. Like this:

```
let click_handler = Callback::new(move |ev: web_sys::MouseEvent| {
    web_sys::console::log_1(
        &"You clicked the button!".into()
    );
});

cc_tree!(
    <button {html_props().onclick(click_handler)}>"click me!"</button>
)

*/
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
