use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::prelude::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlObjectElement;
pub fn object(reia: ComponentBuilder, props: HtmlProps<HtmlObjectElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlObjectElement>,
        UseElementArgs {
            tag_name: "object",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
