use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlInputElement;
pub fn input(reia: ComponentBuilder, props: HtmlProps<HtmlInputElement>) -> impl ContainerReturn {
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
