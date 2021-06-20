use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlHeadingElement;
pub fn h2(reia: ComponentBuilder, props: HtmlProps<HtmlHeadingElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlHeadingElement>,
        UseElementArgs {
            tag_name: "h2",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
