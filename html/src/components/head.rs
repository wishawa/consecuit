use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadElement;
pub fn head(reia: ComponentBuilder, props: ElementProps<HtmlHeadElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHeadElement>,
        UseElementArgs {
            tag_name: "head",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
