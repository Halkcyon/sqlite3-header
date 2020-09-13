use std::{
    convert::TryInto,
    array::TryFromSliceError,
};

const MAGIC_HEADER_STRING: [u8; 16] = [
    0x53, 0x51, 0x4c, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x33, 0x00
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read("data.sqlite")?;

    assert!(contents[0..16] == MAGIC_HEADER_STRING);
    println!("MAGIC HEADER STRING: {}", String::from_utf8_lossy(&contents[0..16]));

    let bytes: [u8; 2] = contents[16..18].try_into().unwrap();
    let page_size = u16::from_be_bytes(bytes);

    assert!(page_size.is_power_of_two());
    assert!(page_size > 511);
    assert!(page_size < 32769);

    println!("PAGE SIZE: {}", page_size);

    println!("FILE FORMAT WRITE VERSION: {}", match contents[18] {
        1 => "legacy",
        2 => "Write-Ahead Logging",
        _ => unimplemented!(),
    });
    println!("FILE FORMAT READ VERSION: {}", match contents[19] {
        1 => "legacy",
        2 => "Write-Ahead Logging",
        _ => unimplemented!(),
    });

    println!("RESERVED BYTES PER PAGE: {}", contents[20]);

    assert!(contents[21] == 64);
    println!("MAXIMUM EMBEDDED PAYLOAD FRACTION: {}", contents[21]);

    assert!(contents[22] == 32);
    println!("MINIMUM EMBEDDED PAYLOAD FRACTION: {}", contents[22]);

    assert!(contents[23] == 32);
    println!("LEAF PAYLOAD FRACTION: {}", contents[23]);

    let file_change_counter = slice_to_word(&contents[24..28])?;
    println!("FILE CHANGE COUNTER: {}", file_change_counter);

    let in_header_database_size = slice_to_word(&contents[28..32])?;
    println!("IN-HEADER DATABASE SIZE: {:?}", in_header_database_size);

    let freelist_index = slice_to_word(&contents[32..36])?;
    println!("FREELIST PAGE INDEX: {}", freelist_index);

    let freelist_count = slice_to_word(&contents[36..40])?;
    println!("FREELIST COUNT: {}", freelist_count);

    let schema_cookie = slice_to_word(&contents[40..44])?;
    println!("SCHEMA COOKIE: {}", schema_cookie);

    let schema_format = slice_to_word(&contents[44..48])?;
    assert!([1, 2, 3, 4].contains(&schema_format));
    println!("SCHEMA FORMAT: {}", schema_format);

    let default_page_cache_size = slice_to_word(&contents[48..52])?;
    println!("DEFAULT PAGE CACHE SIZE: {}", default_page_cache_size);

    let btree_largest_page_root = slice_to_word(&contents[52..56])?;
    println!("LARGEST ROOT B-TREE PAGE: {}", btree_largest_page_root);

    let db_encoding = slice_to_word(&contents[56..60])?;
    println!("DATABASE TEXT ENCODING: {}", match db_encoding {
        1 => "UTF-8",
        2 => "UTF-16le",
        3 => "UTF-16be",
        _ => unimplemented!(),
    });

    let user_version = slice_to_word(&contents[60..64])?;
    println!("USER VERSION: {}", user_version);

    let incremental_vacuum_mode = slice_to_word(&contents[64..68])? != 0;
    println!("INCREMENTAL-VACUUM MODE: {}", incremental_vacuum_mode);

    let application_id = slice_to_word(&contents[68..72])?;
    println!("APPLICATION ID: {}", application_id);

    let reserved = &contents[72..92];
    assert!(reserved.iter().all(|&v| v == 0));

    let version_valid_for_number = slice_to_word(&contents[92..96])?;
    println!("VERSION VALID FOR NUMBER: {}", version_valid_for_number);

    let sqlite_version_number = slice_to_word(&contents[96..100])?;
    println!("SQLITE VERSION NUMBER: {}", sqlite_version_number);

    Ok(())
}

fn slice_to_word(slice: &[u8]) -> Result<u32, TryFromSliceError> {
    let bytes: [u8; 4] = slice.try_into()?;
    Ok(u32::from_be_bytes(bytes))
}
