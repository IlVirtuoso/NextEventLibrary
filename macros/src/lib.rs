mod support;
use proc_macro::{TokenStream, TokenTree};
use quote::{quote, ToTokens, format_ident};
use support::{get_struct_name, get_attrib_field_name};

  

#[proc_macro_derive(LwItem,attributes(Header))]
pub fn lwitem_macro(input : TokenStream) -> TokenStream{
      printstream(input.clone());
      let mut iter = input.into_iter();
      let name = format_ident!("{}",get_struct_name(iter.clone()).unwrap().to_string());
      let mut grpIter = iter.clone();

      let item = grpIter.find(|p| 
            if let TokenTree::Group(t) =p{true} else {false}     
      ).and_then(|p| if let TokenTree::Group(t) = p {Some(t)} else {None});
      
      if let None = item{
            panic!("Group token not found");
      }
      
      let field = format_ident!("{}",get_attrib_field_name(item.unwrap().stream().into_iter(), "Header".to_string()).to_string());
      
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


fn printstream(input : TokenStream){
      let mut iter = input.into_iter();
      loop {
          if let Some(value) = iter.next(){
            match value {
                TokenTree::Group(grp) => {
                  println!("Group: {}",grp.to_string());
                  printstream(grp.stream());
                },
                TokenTree::Ident(id) => println!("id {}",id.to_string()),
                TokenTree::Punct(p) => println!("punct {}",p.to_string()) ,
                TokenTree::Literal(lit) => println!("lit {}", lit.to_string()),
            }
          }
          else{
            break;
          }
      }
}