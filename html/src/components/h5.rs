use crate::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadingElement;
pub fn h5(reia: ComponentBuilder, props: ElementProps<HtmlHeadingElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHeadingElement>,
        UseElementArgs {
            tag_name: "h5",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
