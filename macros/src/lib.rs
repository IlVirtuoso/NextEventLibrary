mod support;
use proc_macro::TokenStream;
use quote::{quote, ToTokens, format_ident};
use support::{get_struct_name, get_attrib_field_name};

#[proc_macro_derive(LwItem,attributes(Header))]
pub fn lwitem_macro(input : TokenStream) -> TokenStream{
      
      
      let mut iter = input.into_iter();
      let name = format_ident!("{}",get_struct_name(iter.clone()).unwrap().to_string());

      //let field = get_attrib_field_name(grpStream.into_iter(), "Header".to_string());
      let field = format_ident!("{}","_header".to_string());
      
     let token = quote!(
      
           impl ILwItem for #name{
                  fn get_header(&mut self) -> &mut LwHeader{
                        &mut self.#field
                  }
            }
      );

      return token.into();    
}

#[proc_macro_attribute]
pub fn lwitem_attr(input : TokenStream, field: TokenStream) -> TokenStream{
      todo!()
}

