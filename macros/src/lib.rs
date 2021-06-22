use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ExprBlock, ExprPath,
};
use syn_rsx::Node;

#[proc_macro]
pub fn reia_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let nodes = parse_macro_input!(input as ReiaNodes);
    let tokens = nodes.build_top();
    tokens.into()
}

struct ReiaNodes(Vec<ReiaNode>);

struct ReiaNode {
    func: ExprPath,
    props: ExprBlock,
    children: ReiaNodes,
    hole: bool,
}

impl Parse for ReiaNodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parser_cfg = syn_rsx::ParserConfig::default();
        let parser = syn_rsx::Parser::new(parser_cfg);
        let nodes: Vec<Node> = parser.parse(input)?;
        let nodes: syn::Result<Vec<ReiaNode>> = nodes.into_iter().map(ReiaNode::try_from).collect();
        let nodes = nodes?;
        Ok(ReiaNodes(nodes))
    }
}

impl TryFrom<Node> for ReiaNode {
    type Error = syn::Error;
    fn try_from(node: Node) -> syn::Result<Self> {
        fn expr_into_text_node(expr: syn::Expr) -> ReiaNode {
            let func = parse_quote!(::reia_html::misc_components::text_node);
            let props = parse_quote!({#expr});
            ReiaNode {
                func,
                props,
                children: ReiaNodes(Vec::new()),
                hole: false,
            }
        }

        let Node {
            name,
            node_type,
            value,
            attributes,
            children,
        } = node;
        let res = match node_type {
            syn_rsx::NodeType::Element => {
                let name = name.unwrap();
                let func = if let syn_rsx::NodeName::Path(path) = &name {
                    Ok(path.clone())
                } else {
                    Err(syn::Error::new(name.span(), "Invalid component"))
                }?;

                let children: syn::Result<Vec<Self>> =
                    children.into_iter().map(ReiaNode::try_from).collect();
                let children = ReiaNodes(children?);

                let mut props = None;
                let mut hole = false;
                for attribute in attributes.iter() {
                    match attribute.node_type {
                        syn_rsx::NodeType::Block => {
                            if let syn::Expr::Block(block) = attribute.value.clone().unwrap() {
                                props = Some(block);
                            } else {
                                panic!()
                            }
                        }
                        syn_rsx::NodeType::Attribute => {
                            if attribute.name_as_string() == Some("HOLE".to_string()) {
                                hole = true;
                                if !children.0.is_empty() {
                                    return Err(syn::Error::new(
                                        attribute.name_span().unwrap(),
                                        "Only childless elements can be hole.",
                                    ));
                                }
                            }
                        }
                        _ => panic!(),
                    }
                }
                let props =
                    props.unwrap_or_else(|| parse_quote!({ ::std::default::Default::default() }));
                ReiaNode {
                    func,
                    props,
                    children,
                    hole,
                }
            }
            syn_rsx::NodeType::Attribute => panic!(),
            syn_rsx::NodeType::Text => expr_into_text_node(value.unwrap()),
            syn_rsx::NodeType::Comment => todo!(),
            syn_rsx::NodeType::Doctype => todo!(),
            syn_rsx::NodeType::Fragment => todo!(),
            syn_rsx::NodeType::Block => expr_into_text_node(value.unwrap()),
        };
        Ok(res)
    }
}

impl ReiaNodes {
    fn build_top(&self) -> TokenStream {
        let tokens = self.build();
        quote! {
            #[allow(unused_braces)]
            reia#tokens
        }
    }
    fn build(&self) -> TokenStream {
        let tokens = self.0.iter().map(|n| {
            let ReiaNode {
                func,
                props,
                children,
                hole,
            } = n;
            let child_part = if children.0.is_empty() {
                if *hole {
                    quote! {
                        .hole_here()
                    }
                } else {
                    quote! {}
                }
            } else {
                let inner = children.build();
                quote! {
                    .child(|reia| {
                        reia
                        #inner
                    })
                }
            };
            quote! {
                .comp(#func, #props)#child_part
            }
        });
        quote! {
            #(#tokens)*
        }
    }
}
