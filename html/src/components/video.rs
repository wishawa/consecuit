use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlVideoElement;
pub fn video(reia: ComponentBuilder, props: HtmlProps<HtmlVideoElement>) -> impl ContainerReturn {
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
