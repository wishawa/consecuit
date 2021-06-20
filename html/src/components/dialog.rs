use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDialogElement;
pub fn dialog(
    reia: ComponentBuilder,
    props: ElementProps<HtmlDialogElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDialogElement>,
        UseElementArgs {
            tag_name: "dialog",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
