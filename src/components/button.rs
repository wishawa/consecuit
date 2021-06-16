use web_sys::{window, HtmlButtonElement};

use crate::{
    component::{ComponentBuilder, ComponentReturn},
    hooks::{use_ref, ReiaFunction, ReiaRef},
};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: ReiaFunction,
}

pub fn button(reia: ComponentBuilder, props: ButtonProps) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, comp): (_, ReiaRef<Option<HtmlButtonElement>>) = reia.hook(use_ref, ());
    let button = comp
        .visit_mut_with(|opt| {
            let button = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let button: HtmlButtonElement = document
                    .create_element("button")
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                reia.parent_node.append_child(&button).unwrap();
                button
            });
            button.clone()
        })
        .unwrap();
    button.set_onclick(Some(props.onclick.as_websys_function()));
    reia.bare_container_node(button.clone().into())
}
