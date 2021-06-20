use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDListElement;
pub fn dl(reia: ComponentBuilder, props: ElementProps<HtmlDListElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDListElement>,
        UseElementArgs {
            tag_name: "dl",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
