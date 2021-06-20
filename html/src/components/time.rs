use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTimeElement;
pub fn time(reia: ComponentBuilder, props: HtmlProps<HtmlTimeElement>) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTimeElement>,
        UseElementArgs {
            tag_name: "time",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
