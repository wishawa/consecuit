use web_sys::{window, Node};

use crate::{
    component::{ComponentBuilder, ComponentValue},
    hooks::{use_ref, ReiaRef},
};

pub fn div(reia: ComponentBuilder, _: ()) -> impl ComponentValue {
    let reia = reia.init();
    let (reia, comp): (_, ReiaRef<Option<Node>>) = reia.hook(use_ref, ());
    let node = comp
        .visit_mut_with(|opt| {
            let node = opt.get_or_insert_with(|| {
                let node: Node = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap()
                    .into();
                reia.parent_node.append_child(&node).unwrap();
                node
            });
            node.clone()
        })
        .unwrap();
    reia.bare_node(node)
}
