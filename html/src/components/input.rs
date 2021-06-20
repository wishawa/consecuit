use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlInputElement;
pub fn input(
    reia: ComponentBuilder,
    props: ElementProps<HtmlInputElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlInputElement>,
        UseElementArgs {
            tag_name: "input",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
