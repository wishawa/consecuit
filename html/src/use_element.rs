use reia::{
    hooks::{use_ref, ReiaRef},
    HookBuilder, HookReturn,
};
use web_sys::{window, Element, HtmlElement};

use wasm_bindgen::JsCast;

#[derive(Clone)]
struct WrappedElement<T: Clone + AsRef<HtmlElement>>(T);

pub(crate) fn use_element<T: 'static + Clone + AsRef<HtmlElement> + JsCast>(
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
                let html_element: &HtmlElement = elem.as_ref();
                parent.append_child(html_element.as_ref()).unwrap();
                WrappedElement(elem)
            });
            elem.0.clone()
        })
        .unwrap();
    (reia, elem)
}
