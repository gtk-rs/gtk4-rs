// Take a look at the license at the top of the repository in the LICENSE file.

use crate::util::*;
use proc_macro2::TokenStream;
use proc_macro_error::{emit_call_site_error, emit_error};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{parse::Parse, Token};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on the `impl` block for a CompositeTemplate widget";

mod keywords {
    syn::custom_keyword!(functions);
    syn::custom_keyword!(function);
    syn::custom_keyword!(name);
}

pub struct Args {
    functions: bool,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self { functions: false };
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::functions) {
                input.parse::<keywords::functions>()?;
                args.functions = true;
            } else {
                return Err(lookahead.error());
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(args)
    }
}

#[derive(Default)]
pub struct CallbackArgs {
    name: Option<String>,
    function: Option<bool>,
}

impl CallbackArgs {
    fn is_function(&self, args: &Args) -> bool {
        self.function.unwrap_or(args.functions)
    }
    fn start(&self, args: &Args) -> usize {
        match self.is_function(args) {
            true => 1,
            false => 0,
        }
    }
}

impl Parse for CallbackArgs {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = Self {
            name: None,
            function: None,
        };
        if stream.is_empty() {
            return Ok(args);
        }
        let input;
        syn::parenthesized!(input in stream);
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(keywords::name) {
                let kw = input.parse::<keywords::name>()?;
                if args.name.is_some() {
                    return Err(syn::Error::new_spanned(kw, "Duplicate `name` attribute"));
                }
                input.parse::<Token![=]>()?;
                let name = input.parse::<syn::LitStr>()?;
                args.name.replace(name.value());
            } else if lookahead.peek(keywords::function) {
                let kw = input.parse::<keywords::function>()?;
                if args.function.is_some() {
                    return Err(syn::Error::new_spanned(
                        kw,
                        "Only one of `function` is allowed",
                    ));
                }
                let function = if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;
                    input.parse::<syn::LitBool>()?.value
                } else {
                    true
                };
                args.function.replace(function);
            } else {
                return Err(lookahead.error());
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(args)
    }
}

pub fn impl_template_callbacks(mut input: syn::ItemImpl, args: Args) -> TokenStream {
    let syn::ItemImpl {
        attrs,
        generics,
        trait_,
        self_ty,
        items,
        ..
    } = &mut input;
    if trait_.is_some() {
        emit_call_site_error!(WRONG_PLACE_MSG);
    }
    let crate_ident = crate_ident_new();

    let mut callbacks = vec![];
    for item in items.iter_mut() {
        if let syn::ImplItem::Method(method) = item {
            let mut i = 0;
            let mut attr = None;
            while i < method.attrs.len() {
                if method.attrs[i].path.is_ident("template_callback") {
                    let callback = method.attrs.remove(i);
                    if attr.is_some() {
                        emit_error!(callback, "Duplicate `template_callback` attribute");
                    } else {
                        attr.replace(callback);
                    }
                } else {
                    i += 1;
                }
            }

            let attr = match attr {
                Some(attr) => attr,
                None => continue,
            };

            let ident = &method.sig.ident;
            let callback_args = syn::parse2::<CallbackArgs>(attr.tokens).unwrap_or_else(|e| {
                emit_error!(e);
                Default::default()
            });
            let name = callback_args
                .name
                .as_ref()
                .cloned()
                .unwrap_or_else(|| ident.to_string());
            let start = callback_args.start(&args);

            let mut arg_names = vec![];
            let mut has_rest = false;
            let value_unpacks = method.sig.inputs.iter_mut().enumerate().map(|(index, arg)| {
                if has_rest {
                    emit_error!(arg, "Arguments past argument with `rest` attribute");
                }
                let index = index + start;
                let name = quote::format_ident!("value{}", index);
                arg_names.push(name.clone());
                let unwrap_value = |ty, err_msg| {
                    let index_err_msg = format!(
                        "Failed to get argument `{ident}` at index {index}: Closure invoked with only {{}} arguments",
                    );
                    quote! {
                        let #name = <[#crate_ident::glib::Value]>::get(&values, #index)
                            .unwrap_or_else(|| panic!(#index_err_msg, values.len()));
                        let #name = #crate_ident::glib::Value::get::<#ty>(#name)
                            .unwrap_or_else(|e| panic!(#err_msg, e));
                    }
                };
                match arg {
                    syn::FnArg::Receiver(receiver) => {
                        let err_msg = format!(
                            "Wrong type for `self` in template callback `{ident}`: {{:?}}",
                        );
                        if receiver.reference.is_none() {
                            Some(unwrap_value(quote! { #self_ty }, err_msg))
                        } else {
                            if receiver.mutability.is_some() {
                                emit_error!(receiver, "Receiver cannot be a mutable reference");
                                return None;
                            }
                            let self_value_ty = quote! {
                                &<#self_ty as #crate_ident::glib::subclass::types::FromObject>::FromObjectType
                            };
                            let mut unwrap = unwrap_value(self_value_ty, err_msg);
                            unwrap.append_all(quote! {
                                let #name = <#self_ty as #crate_ident::glib::subclass::types::FromObject>::from_object(#name);
                            });
                            Some(unwrap)
                        }
                    },
                    syn::FnArg::Typed(typed) => {
                        let mut i = 0;
                        let mut cur_is_rest = false;
                        while i < typed.attrs.len() {
                            if typed.attrs[i].path.is_ident("rest") {
                                let rest = typed.attrs.remove(i);
                                if cur_is_rest {
                                    emit_error!(rest, "Duplicate `rest` attribute");
                                } else if !rest.tokens.is_empty() {
                                    emit_error!(rest, "Tokens after `rest` attribute");
                                }
                                cur_is_rest = true;
                            } else {
                                i += 1;
                            }
                        }
                        if cur_is_rest {
                            has_rest = true;
                            let end = if callback_args.is_function(&args) {
                                quote! { (values.len() - #start) }
                            } else {
                                quote! { values.len() }
                            };
                            Some(quote! {
                                let #name = &values[#index..#end];
                            })
                        } else {
                            let ty = typed.ty.as_ref();
                            let err_msg = format!(
                                "Wrong type for argument {index} in template callback `{ident}`: {{:?}}",
                            );
                            Some(unwrap_value(ty.to_token_stream(), err_msg))
                        }
                    }
                }
            }).collect::<Option<Vec<_>>>();

            let body = value_unpacks
                .map(|value_unpacks| {
                    let call = quote! { #self_ty::#ident(#(#arg_names),*) };
                    match (&method.sig.asyncness, &method.sig.output) {
                        (None, syn::ReturnType::Default) => quote! {
                            #(#value_unpacks)*
                            #call;
                            ::std::option::Option::None
                        },
                        (None, syn::ReturnType::Type(_, _)) => quote! {
                            #(#value_unpacks)*
                            let ret = #call;
                            let ret: #crate_ident::glib::Value = ::std::convert::From::from(ret);
                            ::std::option::Option::Some(ret)
                        },
                        (Some(_), syn::ReturnType::Default) => quote! {
                            let values = values.to_vec();
                            #crate_ident::glib::MainContext::default().spawn_local(async move {
                                #(#value_unpacks)*
                                #call.await
                            });
                            ::std::option::Option::None
                        },
                        (Some(async_), syn::ReturnType::Type(_, _)) => {
                            emit_error!(
                                async_,
                                "`async` only allowed on template callbacks without a return value"
                            );
                            quote! { ::std::option::Option::None }
                        }
                    }
                })
                .unwrap_or_else(|| quote! { ::std::option::Option::None });

            callbacks.push(quote! {
                (#name, |values| {
                    #body
                })
            });
        }
    }

    quote! {
        #(#attrs)*
        impl #generics #self_ty {
            #(#items)*
        }

        impl #crate_ident::subclass::widget::CompositeTemplateCallbacks for #self_ty {
            const CALLBACKS: &'static [#crate_ident::subclass::widget::TemplateCallback] = &[
                #(#callbacks),*
            ];
        }
    }
}
