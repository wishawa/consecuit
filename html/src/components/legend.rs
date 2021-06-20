use crate::elem::{use_element, ElementProps, UseElementArgs};
use reia::{ComponentBuilder, ContainerReturn};
use web_sys::HtmlLegendElement;
pub fn legend(
    reia: ComponentBuilder,
    props: ElementProps<HtmlLegendElement>,
) -> impl ContainerReturn {
    let reia = reia.init();
    let parent = reia.get_parent_node();
    let (reia, elem) = reia.hook(
        use_element::<HtmlLegendElement>,
        UseElementArgs {
            tag_name: "legend",
            props,
            parent,
        },
    );
    reia.bare_container_node(elem.into())
}
