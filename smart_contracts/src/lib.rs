use ink_lang as ink;
use ink_storage::traits::{SpreadAllocate, SpreadLayout};

#[ink::contract]
mod smart_contracts {
    use super::*; // Importing the parent module

    #[ink(storage)]
    #[derive(Debug, SpreadLayout, SpreadAllocate)] // Updated from PackedLayout to SpreadAllocate
    pub struct UserIdentity {
        id: u64,
        name: String,
        email: String,
        timestamp: u64,
    }

    impl UserIdentity {
        #[ink(constructor)]
        pub fn new(id: u64, name: String, email: String, timestamp: u64) -> Self {
            Self { id, name, email, timestamp }
        }

        #[ink(message)]
        pub fn register_identity(&mut self, new_id: u64, new_name: String, new_email: String, new_timestamp: u64) -> Result<(), &'static str> {
            if self.id == new_id {
                return Err("User ID already exists");
            }
            self.id = new_id;
            self.name = new_name;
            self.email = new_email;
            self.timestamp = new_timestamp;
            Ok(())
        }

        #[ink(message)]
        pub fn get_id(&self) -> u64 {
            self.id
        }

        #[ink(message)]
        pub fn get_name(&self) -> &String {
            &self.name
        }

        #[ink(message)]
        pub fn get_email(&self) -> &String {
            &self.email
        }

        #[ink(message)]
        pub fn get_timestamp(&self) -> u64 {
            self.timestamp
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_user_identity() {
            let mut contract = UserIdentity::new(123, "Alice".to_string(), "alice@example.com".to_string(), 123456789);
            assert_eq!(contract.get_id(), 123);
            assert_eq!(contract.get_name(), &"Alice".to_string());
            assert_eq!(contract.get_email(), &"alice@example.com".to_string());

            let result = contract.register_identity(456, "Bob".to_string(), "bob@example.com".to_string(), 987654321);
            assert_eq!(result, Ok(()));
        }
    }
}