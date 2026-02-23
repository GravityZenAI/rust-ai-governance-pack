//! Kata 18: Builder pattern (owned data, optional fields).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub name: String,
    pub age: Option<u8>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserBuilder {
    name: String,
    age: Option<u8>,
    email: Option<String>,
}

impl UserBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        todo!("create builder")
    }

    pub fn age(mut self, age: u8) -> Self {
        todo!("set age")
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        todo!("set email")
    }

    pub fn build(self) -> User {
        todo!("build User")
    }
}
