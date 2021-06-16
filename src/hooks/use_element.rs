use std::ops::Deref;

use web_sys::{window, Element, HtmlElement};

use crate::{HookBuilder, HookReturn};
use wasm_bindgen::JsCast;

use super::{use_ref, ReiaRef};

#[derive(Clone)]
struct WrappedElement<T: Clone + Deref<Target = HtmlElement>>(T);

pub(crate) fn use_element<T: 'static + Clone + Deref<Target = HtmlElement> + JsCast>(
    reia: HookBuilder,
    (tag_name, parent): (&'static str, Element),
) -> impl HookReturn<T> {
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<WrappedElement<T>>>) = reia.hook(use_ref, ());
    let elem = store
        .visit_mut_with(|opt| {
            let elem = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let elem: T = document
                    .create_element(tag_name)
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                parent.append_child(&elem).unwrap();
                WrappedElement(elem)
            });
            elem.0.clone()
        })
        .unwrap();
    (reia, elem)
}
