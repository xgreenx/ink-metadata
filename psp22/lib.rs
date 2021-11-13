pub trait PSP22 {}

startup::on_startup! {
    ink::MUTEX_MAP.lock().unwrap().methods.insert(141414, "psp22_transfer");
    ink::MUTEX_MAP.lock().unwrap().methods.insert(131313, "psp22_balance_of");
    ink::MUTEX_MAP.lock().unwrap().events.push(111);
}