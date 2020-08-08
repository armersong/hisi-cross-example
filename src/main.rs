mod trait_inherit;

trait Dump {
    fn dump(&self) -> String;
}

struct FileDump;

impl Dump for FileDump  {
    fn dump(&self) -> String {
        "FileDump".to_string()
    }
}

struct MemDump;

impl Dump for MemDump {
    fn dump(&self) -> String {
        "MemDump".to_string()
    }
}

fn create_dump(t:u8) -> Box<dyn Dump> {
    if t == 0 {
        Box::new(FileDump{})
    } else {
        Box::new(MemDump{})
    }
}

use trait_inherit::IoHandler;
use std::io::Read;

fn main() {
    let dump = create_dump(0);
    let dump1 = create_dump(1);
    println!("Hello, world! {} {}", dump.dump(), dump1.dump());

    for i in 0..100 {
        let s = format!("#{}", i);
        println!("{}", s); 
    }

    let mut test = trait_inherit::IoTest{};
    let mut buf = [0;32];
    test.read(&mut buf).unwrap();
}
