use web_sys::{window, HtmlDivElement};

use crate::{
    component::ComponentBuilder,
    hooks::{use_ref, ReiaRef},
    ContainerReturn,
};
use wasm_bindgen::JsCast;

#[derive(PartialEq, Clone)]
pub struct DivProps {}

pub fn div(reia: ComponentBuilder, _props: DivProps) -> impl ContainerReturn {
    let reia = reia.init();
    let (reia, comp): (_, ReiaRef<Option<HtmlDivElement>>) = reia.hook(use_ref, ());
    let div = comp
        .visit_mut_with(|opt| {
            let div = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let div: HtmlDivElement =
                    document.create_element("div").unwrap().dyn_into().unwrap();
                reia.parent_node.append_child(&div).unwrap();
                div
            });
            div.clone()
        })
        .unwrap();
    reia.bare_container_node(div.into())
}
