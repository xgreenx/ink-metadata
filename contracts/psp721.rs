pub trait PSP721 {}

startup::on_startup! {
    ink::MUTEX_MAP.lock().unwrap().methods.insert(151515, "psp721_transfer");
    ink::MUTEX_MAP.lock().unwrap().methods.insert(161616, "psp721_balance_of");
    ink::MUTEX_MAP.lock().unwrap().events.push(721);
}