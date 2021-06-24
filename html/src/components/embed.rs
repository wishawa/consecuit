use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlEmbedElement;
pub fn embed(reia: ComponentBuilder, props: HtmlProps<HtmlEmbedElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlEmbedElement>,
        UseElementArgs {
            tag_name: "embed",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
