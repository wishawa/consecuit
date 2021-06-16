use web_sys::{window, Node};

use crate::{
    component::{ComponentBuilder, ComponentValue},
    hooks::{use_ref, ReiaRef},
};

#[derive(Clone, PartialEq)]
pub struct SpanProps {
    pub text: String,
}

pub fn span(reia: ComponentBuilder, props: SpanProps) -> impl ComponentValue {
    let reia = reia.init();
    let (reia, store): (_, ReiaRef<Option<(Node, Node)>>) = reia.hook(use_ref, ());
    store
        .visit_mut_with(|opt| {
            opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let text: Node = document.create_text_node(&props.text).into();
                let span: Node = document.create_element("span").unwrap().into();
                span.append_child(&text).unwrap();
                reia.parent_node.append_child(&span).unwrap();
                (span, text)
            });
        })
        .unwrap();
    let span_node = store
        .visit_with(|opt| {
            let (span, text) = opt.as_ref().unwrap();
            text.set_node_value(Some(&props.text));
            span.clone()
        })
        .unwrap();
    reia.bare_node(span_node)
}
