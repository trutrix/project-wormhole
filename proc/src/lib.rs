
use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
mod record_definition;
mod record_consts;
mod versioned;
mod versioned_consts;


// #[proc_macro]
// pub fn define_record(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as esm::RecordDefinition);
//     let out = quote! { #input };
//     out.into()
// }


/// ```rust
/// use proc::define_record2;
/// 
/// define_record3! {
///     b"TEST", // FourCC identifier
///     TestRecord, // Record Struct name
///     [
///         EditorId; // Common field
///         b"CUST", CustomField, u32; // Custom field
///         b"YNAM", PickUpSound, [b"SOUN"]; // Record reference and restraint to SOUN type
///     ],
///     // Flags - Position / Name
///     [
///         0x00000001, IsImportant;
///     ]
/// }
/// 
/// 
/// ```
#[proc_macro]
#[deprecated(note = "Use define_record3 instead, which supports child groups.")]
pub fn define_record2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as record_definition::RecordDefinition2);
    let out = quote! { #input };
    out.into()
}


#[proc_macro]
pub fn define_record3(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as record_definition::RecordDefinition3);
    let out = quote! { #input };
    out.into()
}




#[proc_macro_derive(VersionedParse)]
pub fn derive_versioned_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let out = versioned::impl_versioned_parse(&input);
    out.into()
}