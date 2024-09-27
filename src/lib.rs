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
