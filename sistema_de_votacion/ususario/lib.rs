#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::usuario::UsuarioRef;
#[ink::contract]
mod ususario {
    #[ink(storage)]
    pub struct Ususario {
        nombre:String,
        apellido:String,
        dni:String,
    }

    impl Ususario {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
