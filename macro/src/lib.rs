use proc_macro2::TokenStream;
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
        let (name, is_unsafe) = match &*name.to_string() {
            "Send" | "Sync" => (quote!(::core::marker::#name), true),
            "Unpin" => (quote!(::core::marker::#name), false),
            "UnwindSafe" => (quote!(::core::panic::#name), false),
            "RefUnwindSafe" => (quote!(::core::panic::#name), false),
            "DynSync" | "DynSend" => (quote!(::rustc_data_structures::marker::#name), true),
            _ => {
                return syn::Error::new_spanned(name, "not a known auto-trait")
                    .into_compile_error()
                    .into()
            }
        };
        s.add_bounds(AddBounds::Generics);
        let impl_source = if is_unsafe {
            quote!(gen unsafe impl #name for @Self {})
        } else {
            quote!(gen impl #name for @Self {})
        };
        let impl_ = s.gen_impl(impl_source);

        let mut generics = s.ast().generics.clone();
        for par in generics.type_params_mut() {
            par.bounds.push(parse_quote! { #name });
        }
        let where_ = generics.where_clause.take();

        let fields = s
            .variants()
            .iter()
            .flat_map(|v| v.bindings())
            .map(|b| &b.ast().ty);

        bodies.push(quote! {
            const _: () = {
                #impl_

                fn _check_bound<T: #name>() {}
                fn _validate_fields #generics () #where_ {
                    #(
                        _check_bound::<#fields>();
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
