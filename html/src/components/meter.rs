use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlMeterElement;
pub fn meter(
    reia: ComponentBuilder,
    props: ElementProps<HtmlMeterElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlMeterElement>,
        UseElementArgs {
            tag_name: "meter",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
