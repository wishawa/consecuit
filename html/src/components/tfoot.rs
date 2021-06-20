use crate::elem::{use_element, HtmlProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlTableSectionElement;
pub fn tfoot(
    reia: ComponentBuilder,
    props: HtmlProps<HtmlTableSectionElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlTableSectionElement>,
        UseElementArgs {
            tag_name: "tfoot",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
