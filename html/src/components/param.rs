use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlParamElement;
pub fn param(
    reia: ComponentBuilder,
    props: ElementProps<HtmlParamElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlParamElement>,
        UseElementArgs {
            tag_name: "param",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
