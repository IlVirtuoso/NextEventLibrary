use std::process::Output;


trait Generator{
    type Output;
    fn generate()->Output;
}


