mod utils;


//adding sum random shitttttt

use wasm_bindgen::prelude::*;
use std::fmt;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Pair {
    issuer: Issuer,
    recipient: Recipient,
}

#[wasm_bindgen]
pub struct Issuer {
    key: f64,
}

#[wasm_bindgen]
pub struct Recipient {
    key: f64,
}

#[wasm_bindgen]
impl Pair {
    pub fn new() -> Pair {
        let issuer = Issuer::new();
        let recipient = Recipient::new();
        Pair {
            issuer,
            recipient,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn issue(&mut self) {
        let new_key = self.issuer.key;
        self.recipient.update(new_key);
    }

    pub fn change(&mut self) {
        let new_key = js_sys::Math::random();
        self.issuer.update(new_key);
    }

    pub fn show(&self) -> bool {
        let i_key = self.issuer.key;
        let r_key = self.recipient.key;
        i_key == r_key
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Issuer Key: {} \n", self.issuer.key)?;
        write!(f, "Recipient Key: {} \n", self.recipient.key)?;

        Ok(())
    }
}

#[wasm_bindgen]
impl Issuer {
    pub fn new() -> Issuer {
        let key = js_sys::Math::random();
        Issuer {
            key,
        }
    }

    pub fn update(&mut self, key: f64) {
        self.key = key;
    }
}

#[wasm_bindgen]
impl Recipient {
    pub fn new() -> Recipient {
        let key = 0.0;
        Recipient {
            key,
        }
    }

    pub fn update(&mut self, key: f64) {
        self.key = key;
    }
}




#[wasm_bindgen]
pub fn greet() {
    alert(&js_sys::Math::random().to_string());
}
