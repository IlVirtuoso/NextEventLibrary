

pub fn forward<T>(value: &mut T)-> &mut T{
    unsafe {
        let mut refer = value as *mut T;
        &mut (*refer)
    }   
}