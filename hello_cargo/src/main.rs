// Usage of something from `contracts` will import all `on_startup`(We only imported psp22 but it also contain stuff related to psp721).
// Commenting of this line will remove `on_startup` from `contracts`.
// But usage of any stuff from there will enable it again.
//
// But we can mark psp721 and psp22 as optional. It requires user to specify that he wants to include.
// In this case we can include `on_startup` only for psp22.
#[allow(unused_imports)]
use contracts::psp22 as _;

startup::on_startup! {
    ink::MUTEX_MAP.lock().unwrap().methods.insert(8, "flip");
    ink::MUTEX_MAP.lock().unwrap().methods.insert(9, "value");
}

fn main() {
    ink::MUTEX_MAP.lock().unwrap().events.push(123);
    ink::MUTEX_MAP.lock().unwrap().events.push(321);
    let metadata = ink::MUTEX_MAP.lock().unwrap();
    println!("Version: {}", metadata.version);
    metadata.methods.iter().for_each(|(key, value)| println!("selector: {}, name: {}", key, value));
    metadata.events.iter().for_each(|event| println!("event: {}", event));
}
