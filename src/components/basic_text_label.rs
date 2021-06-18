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
    reia.comp(span, SpanProps {})
        .child(|reia| reia.comp(text_node, props.text))
}
