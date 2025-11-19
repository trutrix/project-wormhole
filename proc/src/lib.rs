use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;
mod esm;
mod consts;

#[proc_macro]
pub fn define_record(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as esm::RecordDefinition);
    let out = quote! { #input };
    out.into()
}


/// ```rust
/// use proc::define_record2;
/// 
/// define_record2! {
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
pub fn define_record2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as esm::RecordDefinition2);
    let out = quote! { #input };
    out.into()
}