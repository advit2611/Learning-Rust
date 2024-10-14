#![allow(dead_code, unused_variables)]

pub struct Credentials{
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub enum Status {
    Authenticated,
    Rejected,    
}

pub fn authenticate(cred: Credentials) -> Status{
    return Status::Authenticated;
}

// Writing tests
pub fn add_two(left: usize, right: usize) -> usize{
    left + right
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        let result = add_two(2, 2);
        assert_eq!(result, 4);
    }
}
