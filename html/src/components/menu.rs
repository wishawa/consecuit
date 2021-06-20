use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMenuElement;
pub fn menu(reia: ComponentBuilder, props: ElementProps<HtmlMenuElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlMenuElement>,
        UseElementArgs {
            tag_name: "menu",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
