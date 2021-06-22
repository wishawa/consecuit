use reia::*;
use reia_html::prelude::*;

fn footer_span(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    let reia = reia.init();
    reia_tree! (
        <span {html_props().class_name("m-0.5 text-xs text-center text-gray-500")} HOLE />
    )
}

fn hoverable_link(
    reia: ComponentBuilder,
    (text, href): (&'static str, &'static str),
) -> impl ComponentReturn {
    let reia = reia.init();
    reia_tree!(
        <a {HtmlProps::<web_sys::HtmlAnchorElement>::new().class_name("hover:underline").href(href)}>
            {text}
        </a>
    )
}

pub fn app_footer(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    reia_tree!(
        <footer {html_props().class_name("mt-16 p-4 flex flex-col")}>
            <footer_span>
                "Written by "
                <hoverable_link {("Wisha Wa", "https://github.com/wishawa/")} />
            </footer_span>
            <footer_span>
                "with "
                <hoverable_link {("Reia", "https://github.com/wishawa/reia")} />
                " and "
                <hoverable_link {("Tailwind CSS", "https://tailwindcss.com/")} />
            </footer_span>
            <footer_span>
                "to the "
                <hoverable_link {("TodoMVC", "https://todomvc.com/")} />
                " "
                <hoverable_link {("specification", "https://github.com/tastejs/todomvc/blob/master/app-spec.md")} />
            </footer_span>
        </footer>
    )
}
