use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDataElement;
pub fn data(reia: ComponentBuilder, props: ElementProps<HtmlDataElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDataElement>,
        UseElementArgs {
            tag_name: "data",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
