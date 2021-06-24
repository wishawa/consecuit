use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlSourceElement;
pub fn source(reia: ComponentBuilder, props: HtmlProps<HtmlSourceElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlSourceElement>,
        UseElementArgs {
            tag_name: "source",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
