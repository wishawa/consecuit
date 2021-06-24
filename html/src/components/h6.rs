use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadingElement;
pub fn h6(reia: ComponentBuilder, props: HtmlProps<HtmlHeadingElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHeadingElement>,
        UseElementArgs {
            tag_name: "h6",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
