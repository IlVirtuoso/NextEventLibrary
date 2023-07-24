mod support;
use proc_macro::{TokenStream, Group};
use quote::quote;

#[proc_macro_derive(LwItem,attributes(Header))]
pub fn lwitem_macro(input : TokenStream) -> TokenStream{
      let name = "";
      let field = "";
      for v in input.into_iter(){
            match v {
                proc_macro::TokenTree::Group(grp) => {lwitem_macro(grp.stream());},
                proc_macro::TokenTree::Ident(id) => println!("id: {}",id),
                proc_macro::TokenTree::Punct(punc) => println!("punc: {}",punc),
                proc_macro::TokenTree::Literal(lit) => println!("lit: {}",lit),
            }
      }
      /* 
     let token = quote!(
            impl LwItem for #name{
                  pub fn get_header(&self) -> LwHeader{
                        self.#field
                  }
            }
      );

      return token.into();
      */
      TokenStream::new()     
}

#[proc_macro_attribute]
pub fn lwitem_attr(input : TokenStream, field: TokenStream) -> TokenStream{
      todo!()
}

