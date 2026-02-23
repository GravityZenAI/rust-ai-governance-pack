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
        Self {
            name: name.into(),
            age: None,
            email: None,
        }
    }

    pub fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    pub fn build(self) -> User {
        User {
            name: self.name,
            age: self.age,
            email: self.email,
        }
    }
}
