use proc_macro::TokenStream;


pub fn get_struct_name(token: TokenStream) -> String{
      let mut result: String = "".to_string();
      
      for value in token.into_iter(){
            match value {
                proc_macro::TokenTree::Group(_) => {},
                proc_macro::TokenTree::Ident(_) => todo!(),
                proc_macro::TokenTree::Punct(_) => todo!(),
                proc_macro::TokenTree::Literal(_) => todo!(),
            }
      }
      result
}