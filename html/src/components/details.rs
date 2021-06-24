use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDetailsElement;
pub fn details(
    reia: ComponentBuilder,
    props: HtmlProps<HtmlDetailsElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDetailsElement>,
        UseElementArgs {
            tag_name: "details",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
