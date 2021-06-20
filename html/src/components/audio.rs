use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlAudioElement;
pub fn audio(reia: ComponentBuilder, props: HtmlProps<HtmlAudioElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlAudioElement>,
        UseElementArgs {
            tag_name: "audio",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
