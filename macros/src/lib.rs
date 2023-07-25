mod support;
use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens, format_ident};
use support::{get_struct_name, get_attrib_field_name, lwitem_impl};

  

#[proc_macro_derive(LwItem,attributes(Header))]
pub fn lwitem_macro(input : TokenStream) -> TokenStream{
      lwitem_impl(input)
}

#[proc_macro_attribute]
pub fn lwitem_attr(input : TokenStream, field: TokenStream) -> TokenStream{
      todo!()
}


