use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{parse_quote, Ident, Token};
use synstructure::{decl_attribute, AddBounds, Structure};

struct AttrConfig {
    pub elems: Punctuated<Ident, Token![,]>,
}

impl Parse for AttrConfig {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let elems = input.parse_terminated(Ident::parse, Token![,])?;
        Ok(AttrConfig { elems })
    }
}

fn non_structural_derive_impl(attr: TokenStream, mut s: Structure) -> TokenStream {
    let cfg: AttrConfig = match syn::parse2(attr) {
        Ok(x) => x,
        Err(e) => return e.into_compile_error(),
    };

    let mut bodies = vec![];
    for name in cfg.elems {
        let check_fn = match &*name.to_string() {
            "Send" => Ident::new("check_send", Span::call_site()),
            "Sync" => Ident::new("check_sync", Span::call_site()),
            _ => {
                return syn::Error::new_spanned(name, "only `Send` and `Sync` are supported")
                    .into_compile_error()
                    .into()
            }
        };
        s.add_bounds(AddBounds::Generics);

        let types = s
            .variants()
            .iter()
            .flat_map(|v| v.bindings())
            .map(|b| &b.ast().ty);

        let impl_ = s.gen_impl(quote! {
            gen unsafe impl #name for @Self {}
        });

        let mut generics = s.ast().generics.clone();
        for par in generics.type_params_mut() {
            par.bounds.push(parse_quote! { #name });
        }
        let where_ = generics.where_clause.take();

        bodies.push(quote! {
            const _: () = {
                #impl_
                fn _validate #generics () #where_ {
                    #(
                        ::non_structural_derive::#check_fn::<#types>();
                    )*
                }
            };
        });
    }
    Some(s.ast().into_token_stream())
        .into_iter()
        .chain(bodies.into_iter())
        .collect()
}

decl_attribute!([non_structural_derive] => non_structural_derive_impl);
