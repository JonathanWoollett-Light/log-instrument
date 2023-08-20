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

    let enter = format!("{} enter", item_fn.sig.ident);
    let exit = format!("{} exit", item_fn.sig.ident);
    let old_block = *item_fn.block;
    let new_block_token_stream = quote! {
        {
            log::trace!(#enter);
            let out = || #old_block;
            let output = out();
            log::trace!(#exit);
            output
        }
    };
    let new_block = syn::parse::<syn::Block>(new_block_token_stream.into()).unwrap();
    item_fn.block = Box::new(new_block);

    let out = quote! { #item_fn };
    proc_macro::TokenStream::from(out)
}
