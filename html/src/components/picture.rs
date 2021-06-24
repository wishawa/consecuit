use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlPictureElement;
pub fn picture(
    reia: ComponentBuilder,
    props: HtmlProps<HtmlPictureElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlPictureElement>,
        UseElementArgs {
            tag_name: "picture",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
