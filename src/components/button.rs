use web_sys::HtmlButtonElement;

use crate::{
    component::ComponentBuilder,
    hooks::{use_element, JsFunction},
    ContainerReturn,
};

#[derive(Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: JsFunction,
}

pub fn button(reia: ComponentBuilder, props: ButtonProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.parent_node.clone();
    let (reia, button): (_, HtmlButtonElement) = reia.hook(use_element, ("button", parent));
    button.set_onclick(Some(props.onclick.as_websys_function()));
    reia.bare_container_node(button.clone().into())
}
