use proc_macro::TokenStream;

#[proc_macro_derive(LwItem)]
pub fn lwitem_macro(input : TokenStream) -> TokenStream{
      TokenStream::new()
}