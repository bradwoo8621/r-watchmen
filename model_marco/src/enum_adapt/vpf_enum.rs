use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, Expr, Lit, Meta, MetaNameValue, Token, Variant};

struct VPFRestriction<'a> {
    name: &'a Ident,
    context: bool,
    none_context: Option<bool>,
    min_param_count: usize,
    max_param_count: Option<usize>,
}

fn read_context_value(ident: &Ident, value: &MetaNameValue) -> bool {
    if let Expr::Lit(expr_lit) = &value.value {
        if let Lit::Bool(lit) = &expr_lit.lit {
            return lit.value;
        }
    }
    panic!(
        "For #[restrict(context)] at [{}], expected bool literal.",
        ident
    )
}

fn read_none_context_value(ident: &Ident, value: &MetaNameValue) -> bool {
    if let Expr::Lit(expr_lit) = &value.value {
        if let Lit::Bool(lit) = &expr_lit.lit {
            return lit.value;
        }
    }
    panic!(
        "For #[restrict(none_context)] at [{}], expected bool literal.",
        ident
    )
}

fn read_min_param_count_value(ident: &Ident, value: &MetaNameValue) -> usize {
    if let Expr::Lit(expr_lit) = &value.value {
        if let Lit::Int(lit) = &expr_lit.lit {
            return lit.base10_parse::<usize>().expect(
                format!(
                    "For #[restrict(min_param_count)] at [{}], expected usize literal.",
                    ident
                )
                .as_str(),
            );
        }
    }
    panic!(
        "For #[restrict(min_param_count)] at [{}], expected usize literal.",
        ident
    )
}

fn read_max_param_count_value(ident: &Ident, value: &MetaNameValue) -> Option<usize> {
    if let Expr::Lit(expr_lit) = &value.value {
        if let Lit::Int(lit) = &expr_lit.lit {
            return Some(
                lit.base10_parse::<usize>().expect(
                    format!(
                        "For #[restrict(max_param_count)] at [{}], expected usize literal.",
                        ident
                    )
                    .as_str(),
                ),
            );
        }
    }
    panic!(
        "For #[restrict(max_param_count)] at [{}], expected usize literal.",
        ident
    )
}

fn get_restriction(variant: &'_ Variant) -> VPFRestriction<'_> {
    let mut context = true;
    let mut min_param_count: usize = 0;
    let mut max_param_count: Option<usize> = None;
    let mut none_context: Option<bool> = None;

    for attr in &variant.attrs {
        if let Meta::List(list) = &attr.meta {
            if list.path.is_ident("restrict") {
                // println!("MetaList path: {:?}", list.tokens.to_string());
                let args_parsed = Punctuated::<MetaNameValue, Token![,]>::parse_terminated
                    .parse(TokenStream::from(list.tokens.clone()))
                    .expect(format!("Unrecognized attribute #[restrict(...)] at [{}], \
                    valid format are \
                    [context = bool, none_context = bool, min_param_count = usize, max_param_count = usize].", &variant.ident).as_str());
                args_parsed.iter().for_each(|arg| {
                    let arg_path = &arg.path;
                    if arg_path.is_ident("context") {
                        context = read_context_value(&variant.ident, arg);
                    } else if arg_path.is_ident("min_param_count") {
                        min_param_count = read_min_param_count_value(&variant.ident, arg);
                    } else if arg_path.is_ident("max_param_count") {
                        max_param_count = read_max_param_count_value(&variant.ident, arg);
                    } else if arg_path.is_ident("none_context") {
                        none_context = Some(read_none_context_value(&variant.ident, arg));
                    } else {
                        panic!(
                            "Unrecognized attribute #[restrict({})] at [{}].",
                            arg_path.get_ident().unwrap(),
                            &variant.ident
                        )
                    }
                })
            }
        }
    }

    if !context && none_context.is_some() {
        panic!(
            "For #[restrict(context = false, none_context)] at [{}], none_context is not allowed when context is false.",
            &variant.ident
        )
    }

    if let Some(max) = max_param_count {
        if max < min_param_count {
            panic!(
                "For #[restrict(min_param_count, max_param_count)] at [{}], expected min_param_count <= max_param_count.",
                &variant.ident
            )
        }
    }

    VPFRestriction {
        name: &variant.ident,
        context,
        none_context,
        min_param_count,
        max_param_count,
    }
}

pub fn impl_vpf_enum(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    if name.to_string() != "VariablePredefineFunctions" {
        panic!("`vpf` can only be derived for VariablePredefineFunctions")
    }

    if let syn::Data::Enum(data_enum) = input.data {
        let variants: Vec<VPFRestriction> =
            data_enum.variants.iter().map(get_restriction).collect();
        let for_context = variants.iter().map(|var| {
            let variant_name = var.name;
            let context = var.context;
            quote! {
                #name::#variant_name => #context,
            }
        });
        let for_none_context = variants.iter().map(|var| {
            let variant_name = var.name;
            match var.none_context {
                Some(allow) => quote! {
                    #name::#variant_name => #allow,
                },
                None => quote! {
                    #name::#variant_name => false,
                },
            }
        });
        let for_min_param_count = variants.iter().map(|var| {
            let variant_name = var.name;
            let min_param_count = var.min_param_count;
            quote! {
                #name::#variant_name => #min_param_count,
            }
        });
        let for_max_param_count = variants.iter().map(|var| {
            let variant_name = var.name;
            match var.max_param_count {
                Some(count) => quote! {
                    #name::#variant_name => Some(#count),
                },
                None => quote! {
                    #name::#variant_name => None,
                },
            }
        });

        let expanded = quote! {
            impl VariablePredefineFunctions {
                /// whether the function require context.
                pub fn require_context(&self) -> bool {
                    match self {
                        #(#for_context)*
                    }
                }

                /// whether the function allow [None] as context.
                /// returns false if the function require_context is false (context is not allowed).
                pub fn allow_none_context(&self) -> bool {
                    match self {
                        #(#for_none_context)*
                    }
                }

                /// minimum parameter count of the function.
                pub fn min_param_count(&self) -> usize {
                    match self {
                        #(#for_min_param_count)*
                    }
                }

                /// maximum parameter count of the function.
                /// returns None if there is no limit.
                pub fn max_param_count(&self) -> Option<usize> {
                    match self {
                        #(#for_max_param_count)*
                    }
                }
            }
        };

        TokenStream::from(expanded)
    } else {
        panic!("`vpf` can only be derived for VariablePredefineFunctions");
    }
}
