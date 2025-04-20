use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
mod esm;

#[proc_macro]
pub fn define_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as esm::RecordDefinition);
    let out = quote! { #input };
    out.into()
}