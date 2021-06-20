use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlIFrameElement;
pub fn iframe(
    reia: ComponentBuilder,
    props: ElementProps<HtmlIFrameElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlIFrameElement>,
        UseElementArgs {
            tag_name: "iframe",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
