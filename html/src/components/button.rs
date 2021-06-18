use crate::use_element::use_element;
use reia::{hooks::CallbackFunction, ComponentBuilder, ContainerReturn};
use web_sys::HtmlButtonElement;

#[derive(Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: CallbackFunction,
}

pub fn button(reia: ComponentBuilder, props: ButtonProps) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, button): (_, HtmlButtonElement) = reia.hook(use_element, ("button", parent));
    button.set_onclick(Some(props.onclick.as_websys_function()));
    reia.bare_container_node(button.clone().into())
}
