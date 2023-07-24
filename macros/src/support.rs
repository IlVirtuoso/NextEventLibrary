use std::{array::IntoIter, fmt::Debug};

use proc_macro::{Ident, TokenStream, TokenTree};

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
    to_invariant!(item,TokenTree::Ident)
}

pub fn get_attrib_field_name(
    stream: impl Iterator<Item = TokenTree>,
    attrib_name: String,
) -> Option<Ident> {
    let mut iter = stream;
    iter.find(|p| {
        is_invariant!(p,TokenTree::Punct)
    });
    if let TokenTree::Ident(v) = iter.next().unwrap() {
        if v.to_string() == attrib_name {}
    }
    return None;
}
