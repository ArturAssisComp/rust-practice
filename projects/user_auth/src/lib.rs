use log::{error, info};

pub struct User {
    name: String,
    password: String,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            name: name.to_string(),
            password: password.to_string(),
        }
    }
    pub fn sign_in(&self, password: &str) {
        if password == self.password {
            info!("Signing in user: {}", self.name);
        } else {
            error!("Login failed for user: {}", self.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME1: &str = "name1";
    const PASSWORD1: &str = "password1";

    // new constructor
    #[test]
    fn new_constructor() {
        let user = User::new(NAME1, PASSWORD1);
        assert_eq!(user.name, NAME1);
        assert_eq!(user.password, PASSWORD1);
    }
}
