use std::{ops::Deref, rc::Rc};

use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::console;

use crate::hook::{HookBuilder, HookValue};

use super::{use_ref, ReiaRef};

type DynFnClosure = Closure<dyn Fn()>;

#[derive(Clone)]
pub struct ReiaFunction {
    closure: Rc<DynFnClosure>,
}

impl PartialEq for ReiaFunction {
    fn eq(&self, other: &ReiaFunction) -> bool {
        (&*self.closure as *const DynFnClosure) == (&*other.closure as *const DynFnClosure)
    }
}

impl ReiaFunction {
    pub(crate) fn as_websys_function(&self) -> &Function {
        self.closure.deref().as_ref().unchecked_ref()
    }
}

pub fn use_function<F: Fn() + 'static>(
    reia: HookBuilder,
    function: F,
) -> impl HookValue<ReiaFunction> {
    let reia = reia.init();
    let lock = reia.lock.clone();
    let closure = Closure::wrap(Box::new(move || {
        if lock.is_mounted() {
            function();
        }
        else {
            unsafe { console::warn_1(&"Trying to call a function whose component tree had been unmounted. This is a no-op.".into()); }
        }
    }) as Box<dyn Fn()>);
    let rf = ReiaFunction {
        closure: Rc::new(closure),
    };
    let (reia, func_store): (_, ReiaRef<Option<ReiaFunction>>) = reia.hook(use_ref, ());
    func_store
        .visit_mut_with(|opt| *opt = Some(rf.clone()))
        .unwrap();
    (reia, rf)
}
