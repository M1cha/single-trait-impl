use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned as _;

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn single_trait_impl(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let root_impl = syn::parse_macro_input!(input as syn::ItemImpl);
    let root_span = root_impl.span();
    let trait_ = match &root_impl.trait_ {
        Some((_, path, _)) => path,
        None => abort!(root_span, "this only works on trait impls"),
    };

    let mut trait_functions = Vec::with_capacity(root_impl.items.len());
    for item in &root_impl.items {
        let mut method = match item {
            syn::ImplItem::Method(m) => m.clone(),
            _ => abort!(item, "unsupported item"),
        };

        method.block.stmts = vec![syn::Stmt::Item(syn::Item::Verbatim(quote! {;}))];

        trait_functions.push(method);
    }

    proc_macro::TokenStream::from(quote! {
        pub trait #trait_ {
            #(#trait_functions)*
        }

        #root_impl
    })
}
