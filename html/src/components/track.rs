use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTrackElement;
pub fn track(
    reia: ComponentBuilder,
    props: ElementProps<HtmlTrackElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTrackElement>,
        UseElementArgs {
            tag_name: "track",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
