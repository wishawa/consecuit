use std::{ops::Deref, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::console;

use crate::{
    batched_updates,
    hook::{HookBuilder, HookReturn},
};

use super::{use_ref, ReiaRef};

type DynFnClosure = Closure<dyn Fn()>;

#[derive(Clone)]
pub struct CallbackFunction {
    closure: Rc<DynFnClosure>,
}

impl PartialEq for CallbackFunction {
    fn eq(&self, other: &CallbackFunction) -> bool {
        (&*self.closure as *const DynFnClosure) == (&*other.closure as *const DynFnClosure)
    }
}

impl CallbackFunction {
    pub fn as_websys_function(&self) -> &Function {
        self.closure.deref().as_ref().unchecked_ref()
    }
}

pub fn use_function<F: Fn() + 'static>(
    reia: HookBuilder,
    function: F,
) -> impl HookReturn<CallbackFunction> {
    let reia = reia.init();
    let lock = reia.lock.clone();
    let closure = Closure::wrap(Box::new(move || {
        if lock.is_mounted() {
            batched_updates(|| {
                function();
            });
        } else {
            console::warn_1(&"Trying to call a function whose component tree had been unmounted. This is a no-op.".into());
        }
    }) as Box<dyn Fn()>);
    let rf = CallbackFunction {
        closure: Rc::new(closure),
    };
    let (reia, func_store): (_, ReiaRef<Option<CallbackFunction>>) = reia.hook(use_ref, ());
    func_store
        .visit_mut_with(|opt| *opt = Some(rf.clone()))
        .unwrap();
    (reia, rf)
}
