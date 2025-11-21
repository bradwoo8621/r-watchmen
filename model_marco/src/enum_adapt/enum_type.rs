use proc_macro::TokenStream;
use quote::quote;

/// do nothing, just identify the enum type
pub fn impl_enum_type(_item: TokenStream) -> TokenStream {
    TokenStream::from(quote! {})
}
