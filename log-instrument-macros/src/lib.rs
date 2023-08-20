#![warn(clippy::pedantic)]

extern crate proc_macro;
use quote::quote;

/// Adds `log::trace!` events at the start and end of an attributed function.
///
/// # Panics
///
/// When applied to anything other than a function.
#[proc_macro_attribute]
pub fn instrument(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn::Item);

    let syn::Item::Fn(mut item_fn) = input else {
        panic!("Instrument macro can only be on functions.")
    };
    let item_fn_ident = item_fn.sig.ident.to_string();
    let new_stmt_tokens = quote! { let __ = log_instrument::__Instrument::new(#item_fn_ident); };
    let new_stmt = syn::parse::<syn::Stmt>(new_stmt_tokens.into()).unwrap();
    item_fn.block.stmts.insert(0, new_stmt);

    let out = quote! { #item_fn };
    proc_macro::TokenStream::from(out)
}
