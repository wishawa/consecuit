use web_sys::{window, Text};

use crate::{
    hooks::{use_ref, ReiaRef},
    ComponentBuilder, ComponentReturn,
};

pub fn text_node(reia: ComponentBuilder, value: String) -> impl ComponentReturn {
    let reia = reia.init();
    let parent = reia.parent_node.clone();
    let (reia, store): (_, ReiaRef<Option<Text>>) = reia.hook(use_ref, ());
    let text = store
        .visit_mut_with(|opt| {
            let text = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let text = document.create_text_node(&value);
                parent.append_child(&text).unwrap();
                text
            });
            text.clone()
        })
        .unwrap();
    text.set_node_value(Some(&value));
    reia.bare_leaf_node()
}
