use web_sys::{window, Node};

use crate::component::{ComponentBuilder, ComponentReturn};

use super::{span, text_node, SpanProps};

#[derive(Clone, PartialEq)]
pub struct BasicTextLabelProps {
    pub text: String,
}

pub fn basic_text_label(
    reia: ComponentBuilder,
    props: BasicTextLabelProps,
) -> impl ComponentReturn {
    let reia = reia.init();
    reia.node(span, SpanProps {})
        .child(|reia| reia.node(text_node, props.text))
}
