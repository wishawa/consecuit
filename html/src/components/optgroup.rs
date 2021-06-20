use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlOptGroupElement;
pub fn optgroup(
    reia: ComponentBuilder,
    props: HtmlProps<HtmlOptGroupElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlOptGroupElement>,
        UseElementArgs {
            tag_name: "optgroup",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}