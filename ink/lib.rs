use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct Metadata {
    pub version: u32,
    pub methods: HashMap<u32, &'static str>,
    pub events: Vec<u32>,
}

lazy_static! {
    pub static ref MUTEX_MAP: Mutex<Metadata> = {
        let m = Metadata {
            version: 3,
            methods: HashMap::new(),
            events: Vec::new(),
        };
        Mutex::new(m)
    };
}

startup::on_startup! {
    MUTEX_MAP.lock().unwrap().methods.insert(1111111, "some_ink_method");
    MUTEX_MAP.lock().unwrap().methods.insert(0000000, "fallback_method");
    MUTEX_MAP.lock().unwrap().events.push(42);
}