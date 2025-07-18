#![allow(clippy::format_collect)]

use lowercase_hex::Buffer;

#[test]
fn buffer_fmt() {
    let mut buffer = Buffer::<256, true>::new();
    buffer.format(&ALL);
    let s = format!("{buffer:?}");
    let pre = "Buffer(\"0x";
    let post = "\")";
    assert_eq!(&s[..pre.len()], pre);
    assert_eq!(&s[s.len() - post.len()..], post);
    assert_lower(&s[pre.len()..s.len() - post.len()]);
}

#[test]
fn buffer_array_lower() {
    let mut buffer = Buffer::<256>::new();
    let s = buffer.format(&ALL);
    assert_lower(s);
}

#[test]
fn buffer_slice_lower() {
    let mut buffer = Buffer::<256>::new();
    let s = buffer.format_slice(ALL);
    assert_lower(s);
}

#[test]
fn buffer_const_lower() {
    const BUFFER: Buffer<256> = Buffer::new().const_format(&ALL);
    assert_lower(BUFFER.as_str());
}

#[test]
#[cfg(feature = "alloc")]
fn encode_lower() {
    let encoded = lowercase_hex::encode(ALL);
    assert_lower(&encoded);
}

#[test]
#[cfg(feature = "alloc")]
fn decode_lower() {
    let decoded = lowercase_hex::decode(ALL_LOWER).unwrap();
    assert_eq!(decoded, ALL);
    let decoded = lowercase_hex::decode_to_array(ALL_LOWER).unwrap();
    assert_eq!(decoded, ALL);
}

#[test]
#[cfg(feature = "alloc")]
fn decode_upper_rejected() {
    // Uppercase should be rejected
    assert!(lowercase_hex::decode(ALL_UPPER).is_err());
    assert!(lowercase_hex::decode_to_array::<_, 256>(ALL_UPPER).is_err());
}

#[test]
#[cfg(feature = "alloc")]
fn roundtrips() {
    test_roundtrip("1234");
    test_roundtrip("00000000000011");
    test_roundtrip("0000000000000022");
    test_roundtrip("000000000000000033");
    test_roundtrip("00000000000000003344");
    test_roundtrip("05161049138038061049183398181016");
}

#[test]
#[cfg(feature = "alloc")]
fn roundtrip_long() {
    test_roundtrip("608060405234801561001057600080fd5b50600436106100365760003560e01c8063a41368621461003b578063cfae3217146100e3575b600080fd5b6100e16004803603602081101561005157600080fd5b81019060208101813564010000000081111561006c57600080fd5b82018360208201111561007e57600080fd5b803590602001918460018302840111640100000000831117156100a057600080fd5b91908080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929550610160945050505050565b005b6100eb610177565b6040805160208082528351818301528351919283929083019185019080838360005b8381101561012557818101518382015260200161010d565b50505050905090810190601f1680156101525780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b805161017390600090602084019061020d565b5050565b60008054604080516020601f60026000196101006001881615020190951694909404938401819004810282018101909252828152606093909290918301828280156102035780601f106101d857610100808354040283529160200191610203565b820191906000526020600020905b8154815290600101906020018083116101e657829003601f168201915b5050505050905090565b828054600181600116156101000203166002900490600052602060002090601f0160209004810192826102435760008555610289565b82601f1061025c57805160ff1916838001178555610289565b82800160010185558215610289579182015b8281111561028957825182559160200191906001019061026e565b50610295929150610299565b5090565b5b80821115610295576000815560010161029a56fea26469706673582212208b9161dfd195d53618942a72a3b481d61a7b142de919925a0b34f9c986e5707e64736f6c63430007060033");
}

#[cfg(feature = "alloc")]
fn test_roundtrip(s: &str) {
    lowercase_hex::check(s).expect(s);
    let decoded = lowercase_hex::decode(s).expect(s);
    assert_eq!(decoded, hex::decode(s).expect(s));
    assert_eq!(lowercase_hex::encode(&decoded), s);
}

#[test]
fn check() {
    assert_eq!(lowercase_hex::check(""), Ok(()));
    assert!(lowercase_hex::check_raw(""));

    assert_eq!(lowercase_hex::check(ALL_LOWER), Ok(()));
    assert!(lowercase_hex::check_raw(ALL_LOWER));

    // Uppercase should be rejected
    assert!(lowercase_hex::check(ALL_UPPER).is_err());
    assert!(!lowercase_hex::check_raw(ALL_UPPER));

    // 0x prefix should be rejected
    assert!(lowercase_hex::check("0x48656c6c6f20776f726c6421").is_err());

    let error_cases = [
        ("ag", 1, 'g'),
        ("0xbz", 1, 'x'),           // 'x' is invalid hex character
        ("0x12340000000n", 1, 'x'), // 'x' is invalid hex character
        ("AB", 0, 'A'),             // Uppercase should be rejected
        ("0x123F", 1, 'x'),         // 'x' is invalid hex character
    ];
    for (s, index, c) in error_cases {
        assert_eq!(s[index..].chars().next(), Some(c), "{s:?}");
        assert_eq!(
            lowercase_hex::check(s),
            Err(lowercase_hex::FromHexError::InvalidHexCharacter { c, index })
        );
    }
}

#[test]
#[cfg(all(feature = "serde", feature = "alloc", not(feature = "hex")))]
fn serde() {
    #[derive(serde::Serialize, serde::Deserialize)]
    struct All {
        #[serde(with = "lowercase_hex")]
        x: Vec<u8>,
    }

    let all = All { x: ALL.to_vec() };
    let encoded = serde_json::to_string(&all).unwrap();
    assert_eq!(encoded, format!(r#"{{"x":"0x{ALL_LOWER}"}}"#));
    let decoded: All = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.x, ALL);
}

const ALL: [u8; 256] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
    0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F,
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F,
    0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F,
    0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x7F,
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8C, 0x8D, 0x8E, 0x8F,
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F,
    0xA0, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xAF,
    0xB0, 0xB1, 0xB2, 0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD, 0xBE, 0xBF,
    0xC0, 0xC1, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCE, 0xCF,
    0xD0, 0xD1, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDC, 0xDD, 0xDE, 0xDF,
    0xE0, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xE9, 0xEA, 0xEB, 0xEC, 0xED, 0xEE, 0xEF,
    0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC, 0xFD, 0xFE, 0xFF,
];

const ALL_LOWER: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f606162636465666768696a6b6c6d6e6f707172737475767778797a7b7c7d7e7f808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9fa0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebfc0c1c2c3c4c5c6c7c8c9cacbcccdcecfd0d1d2d3d4d5d6d7d8d9dadbdcdddedfe0e1e2e3e4e5e6e7e8e9eaebecedeeeff0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";
const ALL_UPPER: &str = "000102030405060708090A0B0C0D0E0F101112131415161718191A1B1C1D1E1F202122232425262728292A2B2C2D2E2F303132333435363738393A3B3C3D3E3F404142434445464748494A4B4C4D4E4F505152535455565758595A5B5C5D5E5F606162636465666768696A6B6C6D6E6F707172737475767778797A7B7C7D7E7F808182838485868788898A8B8C8D8E8F909192939495969798999A9B9C9D9E9FA0A1A2A3A4A5A6A7A8A9AAABACADAEAFB0B1B2B3B4B5B6B7B8B9BABBBCBDBEBFC0C1C2C3C4C5C6C7C8C9CACBCCCDCECFD0D1D2D3D4D5D6D7D8D9DADBDCDDDEDFE0E1E2E3E4E5E6E7E8E9EAEBECEDEEEFF";

#[track_caller]
fn assert_lower(s: &str) {
    let expected = (0..=u8::MAX)
        .map(|i| format!("{i:02x}"))
        .collect::<String>();
    assert_eq!(ALL_LOWER, expected);
    assert_eq!(s, expected);
}
