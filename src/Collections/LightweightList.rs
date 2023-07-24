use std::any::Any;


pub trait ILwItem: Any {
    fn get_header(&mut self) -> &mut LwHeader;
}


pub struct LwHeader {
    next: Option<*mut dyn Any>,
    prev: Option<*mut dyn Any>,
}

impl LwHeader {
    pub fn new() -> Self {
        LwHeader {
            next: None,
            prev: None,
        }
    }
}

pub struct LwList<T>
where
    T: ILwItem,
{
    head: Option<*mut T>,
    tail: Option<*mut T>,
    count: usize,
}

impl<T> LwList<T>
where
    T: ILwItem,
{
    pub fn new() -> Self {
        LwList {
            head: None,
            tail: None,
            count: 0,
        }
    }

    pub fn push(&mut self, value: *mut T) {
        if self.head.is_none() {
            self.head = Some(value);
        } else if self.tail.is_none() {
            self.tail = Some(value);
            unsafe {
                (*self.head.unwrap()).get_header().next = Some(value);
                (*value).get_header().prev = Some(self.head.unwrap());
            }
        } else {
            unsafe {
                (*value).get_header().prev = Some(self.tail.unwrap());
                (*self.tail.unwrap()).get_header().next = Some(value);
                self.tail = Some(value);
            }
        }
        self.count += 1;
    }

    pub fn front(&self) -> Option<*const T> {
        if let Some(value) = self.head {
            return Some(value);
        } else {
            return None;
        }
    }

    pub fn pop(&mut self) {
        if self.count >= 3 {
            unsafe {
                let mut ptr = self.head.unwrap();
                self.head = Some((*ptr).get_header().next.unwrap().cast());
                (*ptr).get_header().next = None;
            }
        }
        
    }

    


}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    struct MockValue<T> {
        lwheader: LwHeader,
        value: T,
    }

    impl<T> ILwItem for MockValue<T>
    where
        T: 'static,
    {
        fn get_header(&mut self) -> &mut LwHeader {
            &mut self.lwheader
        }
    }

    impl<T> MockValue<T> {
        pub fn new(value: T) -> Self {
            MockValue {
                lwheader: LwHeader::new(),
                value: value,
            }
        }

        pub fn create_ptr(value: T) -> Rc<RefCell<MockValue<T>>>{
            Rc::new(RefCell::new(MockValue::new(value)))
        }
    }

    #[test]
    fn test_push() {
        let mut list: LwList<MockValue<i32>> = LwList::new();
        let mut value = Rc::new(RefCell::new(MockValue::<i32>::new(1)));
        list.push(value.as_ptr());
        let v = list.front();
        unsafe {
            assert_eq!((*v.unwrap()).value, (*value.as_ptr()).value);
        }
    }

    #[test]
    fn test_pop() {
        let mut list: LwList<MockValue<i32>> = LwList::new();
        let mut v1 = MockValue::<i32>::create_ptr(3);
        let mut v2 = MockValue::<i32>::create_ptr(2);
        let mut v3 = MockValue::<i32>::create_ptr(1);
        
        list.push(v1.as_ptr());
        list.push(v2.as_ptr());
        list.push(v3.as_ptr());

        unsafe{
        assert_eq!((*v1.as_ptr()).value, (*list.front().unwrap()).value);
        list.pop();
        assert_eq!((*v2.as_ptr()).value, (*list.front().unwrap()).value);
        list.pop();
        assert_eq!((*v3.as_ptr()).value, (*list.front().unwrap()).value);


        }
    } 
}
