use std::{array::IntoIter, fmt::Debug, ops::Add};

use proc_macro::{Ident, TokenStream, TokenTree};
use quote::{format_ident,quote};

macro_rules! to_invariant {
    ($a:ident, $b:path ) => {{
        if let $b(t) = $a {
            return Some(t);
        }
        None
    }};

    ($a:ident, $b:ident ) => {{
        if let $b(t) = $a {
            return Some(t);
        }
        None
    }};
}

macro_rules! is_invariant {
    ($a:ident, $b:path ) => {{
        if let $b(_) = $a {
            return true;
        }
        false
    }};

    ($a:ident, $b:ident ) => {{
        if let $b(_) = $a {
            return true;
        }
        false
    }};
}

pub fn get_struct_name(stream: impl Iterator<Item = TokenTree>) -> Option<proc_macro::Ident> {
    let mut iter = stream;
    iter.find(|t| {
        if let TokenTree::Ident(v) = t {
            if v.to_string() == "struct" {
                true
            } else {
                false
            }
        } else {
            false
        }
    });
    let item = iter.next().unwrap();
    to_invariant!(item, TokenTree::Ident)
}

pub fn get_attrib_field_name(
    stream: impl Iterator<Item = TokenTree>,
    attrib_name: String,
) -> Ident {
    let mut iter = stream;
    loop {
        iter
            .find(|p| {
                if let TokenTree::Punct(t) = p {
                    t.to_string() == "#"
                } else {
                    false
                }
            });

            let item = iter.find(|p|{
                if let TokenTree::Group(g) = p{
                    p.to_string() == format!("[{}]",attrib_name)
                }
                else {false}
            });
        if item.is_none() {
            panic!("Could find any attribute");
        }
        if let TokenTree::Ident(t) = iter.next().unwrap(){
            return t;
        }
        else{
            panic!("No field found");
        }
    }
}


pub fn lwitem_impl(input: TokenStream) -> TokenStream{
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