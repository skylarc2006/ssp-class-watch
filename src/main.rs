use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn generate_watch(base_address: u32, offsets: Option<&[u32]>, current_variable: u32) -> String {
    const WORD_TYPE: u32 = 2; // 4 byte word type index
    let current_offset = (current_variable - 1) * 4;

    let mut watch = format!(
        "{{
    \"address\": \"{base_address:08X}\",
    \"baseIndex\": 0,
    \"label\": \"unknown{current_variable}\",");

    if let Some(offsets) = offsets {
        watch += format!("
    \"pointerOffsets\": [").as_str();

        for offset in offsets.iter() {
            watch += format!("
        \"{offset:X}\",").as_str();
        }

        watch += format!("
        \"{current_offset:X}\"
    ],").as_str();
    }

    watch += format!("
    \"typeIndex\": {WORD_TYPE},
    \"unsigned\": false
}}").as_str();

    watch
}

/*
{
    "address": "802CA970",
    "baseIndex": 0,
    "label": "Current Realm",
    "pointerOffsets": [
        "160",
        "19C"
    ],
    "typeIndex": 2,
    "unsigned": false
}
*/



fn main() {
    let base_address = 0x802ca6e0u32;
    let offsets = Some(Vec::from([0x8u32, 0x54u32]));
    let size = 0x2a0u32;

    let path = Path::new("output.txt");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for i in 1..=size / 4 {
        let mut watch = generate_watch(base_address, offsets.as_deref(), i);
        watch += ",\n";
        file.write_all(watch.as_bytes()).unwrap();
    }
}
