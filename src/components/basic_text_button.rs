use web_sys::{window, Node};

use crate::{
    component::{ComponentBuilder, ComponentReturn},
    hooks::ReiaFunction,
};

use super::{button, span, text_node, ButtonProps, SpanProps};

#[derive(Clone, PartialEq)]
pub struct BasicTextButtonProps {
    pub text: String,
    pub onclick: ReiaFunction,
}

pub fn basic_text_button(
    reia: ComponentBuilder,
    props: BasicTextButtonProps,
) -> impl ComponentReturn {
    let reia = reia.init();
    let BasicTextButtonProps { text, onclick } = props;
    reia.node(button, ButtonProps { onclick })
        .child(|reia| reia.node(text_node, text))
}
