use web_sys::{window, Text};

use consecuit::prelude::*;

pub fn text_node(cc: ComponentBuilder, value: impl AsRef<str>) -> impl ComponentReturn {
    let cc = cc.init();
    let parent = cc.get_parent_node();
    let (cc, store): (_, Reference<Option<Text>>) = cc.hook(use_ref, ());
    let text = store
        .visit_mut_with(|opt| {
            let text = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let text = document.create_text_node(value.as_ref());
                parent.append_child(&text).unwrap();
                text
            });
            text.clone()
        })
        .unwrap();
    text.set_node_value(Some(value.as_ref()));
    cc.bare_leaf_node()
}
