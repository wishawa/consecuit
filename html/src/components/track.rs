use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTrackElement;
pub fn track(reia: ComponentBuilder, props: HtmlProps<HtmlTrackElement>) -> impl ContainerReturn {
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
