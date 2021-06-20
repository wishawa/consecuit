use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOutputElement;
pub fn output(
    reia: ComponentBuilder,
    props: ElementProps<HtmlOutputElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlOutputElement>,
        UseElementArgs {
            tag_name: "output",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
