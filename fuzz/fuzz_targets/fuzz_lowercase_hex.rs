#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    lowercase_hex::fuzzing::fuzz(data).unwrap();
});
