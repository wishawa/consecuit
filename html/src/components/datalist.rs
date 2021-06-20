use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlDataListElement;
pub fn datalist(
    reia: ComponentBuilder,
    props: ElementProps<HtmlDataListElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlDataListElement>,
        UseElementArgs {
            tag_name: "datalist",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
