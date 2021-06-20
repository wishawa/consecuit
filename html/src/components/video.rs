use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlVideoElement;
pub fn video(
    reia: ComponentBuilder,
    props: ElementProps<HtmlVideoElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlVideoElement>,
        UseElementArgs {
            tag_name: "video",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
