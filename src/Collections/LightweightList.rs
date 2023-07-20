use std::any::Any;


pub trait ILwItem{
      fn get_header(&mut self)-> LwHeader; 
}



pub struct LwHeader{
      next: Option<&'static dyn Any>,
      prev: Option<&'static dyn Any>
}


pub struct LwList{
      head: Option<&'static dyn ILwItem>,
      tail: Option<&'static dyn ILwItem>
}


