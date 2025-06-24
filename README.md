# lowercase-hex

[![github](https://img.shields.io/badge/github-andunieee/lowercase--hex-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/andunieee/lowercase-hex)
[![crates.io](https://img.shields.io/crates/v/lowercase-hex.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/lowercase-hex)
[![docs.rs](https://img.shields.io/badge/docs.rs-lowercase--hex-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/lowercase-hex)
[![build status](https://img.shields.io/github/actions/workflow/status/andunieee/lowercase-hex/ci.yml?branch=master&style=for-the-badge)](https://github.com/andunieee/lowercase-hex/actions?query=branch%3Amaster)

This crate provides a fast conversion of byte arrays to **lowercase** hexadecimal strings,
both at compile time, and at run time.

This is a fork of https://github.com/danipopes/const-hex that enforces strict lowercase.

_Version requirement: rustc 1.64+_

[const-eval]: https://docs.rs/lowercase-hex/latest/lowercase_hex/fn.const_encode.html
[buffer]: https://docs.rs/lowercase-hex/latest/lowercase_hex/struct.Buffer.html
[`itoa`]: https://docs.rs/itoa/latest/itoa/struct.Buffer.html

## Performance

This crate offers performance comparable to [`faster-hex`] on `x86`/`x86-64`
architectures but outperforms it on other platforms, as [`faster-hex`] is
only optimized for `x86`/`x86-64`.

This crate is 10 to 50 times faster than [`hex`] in encoding and decoding, and
100+ times faster than `libstd` in formatting.

The following benchmarks were ran on an AMD Ryzen 9 7950X, compiled with
`1.83.0-nightly (9ff5fc4ff 2024-10-03)` on `x86_64-unknown-linux-gnu`.

You can run these benchmarks with `cargo bench --features std` on a nightly
compiler.

```log
test check::faster_hex::bench1_32b               ... bench:           6.86 ns/iter (+/- 0.11)
test check::faster_hex::bench2_256b              ... bench:          41.83 ns/iter (+/- 0.39)
test check::faster_hex::bench3_2k                ... bench:         276.98 ns/iter (+/- 4.30)
test check::faster_hex::bench4_16k               ... bench:       2,165.20 ns/iter (+/- 31.02)
test check::faster_hex::bench5_128k              ... bench:      17,309.93 ns/iter (+/- 379.44)
test check::faster_hex::bench6_1m                ... bench:     142,285.03 ns/iter (+/- 5,249.29)
test check::lowercase_hex::bench1_32b            ... bench:           5.82 ns/iter (+/- 0.18)
test check::lowercase_hex::bench2_256b           ... bench:          29.98 ns/iter (+/- 1.17)
test check::lowercase_hex::bench3_2k             ... bench:         232.49 ns/iter (+/- 3.76)
test check::lowercase_hex::bench4_16k            ... bench:       1,809.51 ns/iter (+/- 19.41)
test check::lowercase_hex::bench5_128k           ... bench:      14,478.82 ns/iter (+/- 186.05)
test check::lowercase_hex::bench6_1m             ... bench:     118,342.35 ns/iter (+/- 1,806.59)
test check::naive::bench1_32b                    ... bench:          45.06 ns/iter (+/- 3.02)
test check::naive::bench2_256b                   ... bench:         512.15 ns/iter (+/- 3.01)
test check::naive::bench3_2k                     ... bench:       5,073.88 ns/iter (+/- 37.76)
test check::naive::bench4_16k                    ... bench:      88,054.18 ns/iter (+/- 858.42)
test check::naive::bench5_128k                   ... bench:     832,259.43 ns/iter (+/- 3,408.40)
test check::naive::bench6_1m                     ... bench:   6,781,407.55 ns/iter (+/- 73,030.65)
test decode::faster_hex::bench1_32b              ... bench:          27.74 ns/iter (+/- 0.37)
test decode::faster_hex::bench2_256b             ... bench:         101.41 ns/iter (+/- 3.30)
test decode::faster_hex::bench3_2k               ... bench:         703.92 ns/iter (+/- 9.18)
test decode::faster_hex::bench4_16k              ... bench:       5,128.69 ns/iter (+/- 63.89)
test decode::faster_hex::bench5_128k             ... bench:      40,764.42 ns/iter (+/- 709.74)
test decode::faster_hex::bench6_1m               ... bench:     360,419.77 ns/iter (+/- 10,452.05)
test decode::hex::bench1_32b                     ... bench:         201.09 ns/iter (+/- 8.97)
test decode::hex::bench2_256b                    ... bench:       1,866.91 ns/iter (+/- 27.74)
test decode::hex::bench3_2k                      ... bench:      18,459.65 ns/iter (+/- 132.40)
test decode::hex::bench4_16k                     ... bench:     156,200.02 ns/iter (+/- 1,087.62)
test decode::hex::bench5_128k                    ... bench:   1,259,235.00 ns/iter (+/- 16,768.44)
test decode::hex::bench6_1m                      ... bench:  10,090,523.50 ns/iter (+/- 24,818.34)
test decode::lowercase_hex::bench1_32b           ... bench:          25.20 ns/iter (+/- 0.47)
test decode::lowercase_hex::bench2_256b          ... bench:          85.39 ns/iter (+/- 0.68)
test decode::lowercase_hex::bench3_2k            ... bench:         622.14 ns/iter (+/- 6.47)
test decode::lowercase_hex::bench4_16k           ... bench:       4,480.13 ns/iter (+/- 63.32)
test decode::lowercase_hex::bench5_128k          ... bench:      35,632.35 ns/iter (+/- 486.96)
test decode::lowercase_hex::bench6_1m            ... bench:     311,295.43 ns/iter (+/- 16,437.37)
test decode_to_slice::faster_hex::bench1_32b     ... bench:          16.51 ns/iter (+/- 0.06)
test decode_to_slice::faster_hex::bench2_256b    ... bench:          85.09 ns/iter (+/- 1.64)
test decode_to_slice::faster_hex::bench3_2k      ... bench:         614.88 ns/iter (+/- 6.42)
test decode_to_slice::faster_hex::bench4_16k     ... bench:       4,825.86 ns/iter (+/- 85.11)
test decode_to_slice::faster_hex::bench5_128k    ... bench:      38,357.67 ns/iter (+/- 972.51)
test decode_to_slice::faster_hex::bench6_1m      ... bench:     330,883.88 ns/iter (+/- 32,079.73)
test decode_to_slice::hex::bench1_32b            ... bench:          58.89 ns/iter (+/- 1.44)
test decode_to_slice::hex::bench2_256b           ... bench:         610.67 ns/iter (+/- 15.75)
test decode_to_slice::hex::bench3_2k             ... bench:       8,220.72 ns/iter (+/- 259.07)
test decode_to_slice::hex::bench4_16k            ... bench:     103,786.71 ns/iter (+/- 2,026.26)
test decode_to_slice::hex::bench5_128k           ... bench:     990,568.62 ns/iter (+/- 1,976.89)
test decode_to_slice::hex::bench6_1m             ... bench:   7,876,872.30 ns/iter (+/- 18,905.00)
test decode_to_slice::lowercase_hex::bench1_32b  ... bench:          13.88 ns/iter (+/- 0.05)
test decode_to_slice::lowercase_hex::bench2_256b ... bench:          71.47 ns/iter (+/- 0.83)
test decode_to_slice::lowercase_hex::bench3_2k   ... bench:         572.47 ns/iter (+/- 76.37)
test decode_to_slice::lowercase_hex::bench4_16k  ... bench:       4,507.82 ns/iter (+/- 90.16)
test decode_to_slice::lowercase_hex::bench5_128k ... bench:      35,963.28 ns/iter (+/- 373.01)
test decode_to_slice::lowercase_hex::bench6_1m   ... bench:     320,093.83 ns/iter (+/- 75,070.88)
test encode::faster_hex::bench1_32b              ... bench:          29.90 ns/iter (+/- 0.34)
test encode::faster_hex::bench2_256b             ... bench:          58.10 ns/iter (+/- 1.71)
test encode::faster_hex::bench3_2k               ... bench:         313.61 ns/iter (+/- 5.77)
test encode::faster_hex::bench4_16k              ... bench:       2,119.70 ns/iter (+/- 67.17)
test encode::faster_hex::bench5_128k             ... bench:      16,664.43 ns/iter (+/- 456.81)
test encode::faster_hex::bench6_1m               ... bench:     178,190.80 ns/iter (+/- 29,494.59)
test encode::hex::bench1_32b                     ... bench:         191.27 ns/iter (+/- 6.92)
test encode::hex::bench2_256b                    ... bench:       1,318.34 ns/iter (+/- 73.17)
test encode::hex::bench3_2k                      ... bench:      10,176.20 ns/iter (+/- 523.88)
test encode::hex::bench4_16k                     ... bench:      82,000.56 ns/iter (+/- 1,539.21)
test encode::hex::bench5_128k                    ... bench:     655,616.70 ns/iter (+/- 16,414.17)
test encode::hex::bench6_1m                      ... bench:   5,435,120.10 ns/iter (+/- 369,061.12)
test encode::lowercase_hex::bench1_32b           ... bench:          12.54 ns/iter (+/- 0.17)
test encode::lowercase_hex::bench2_256b          ... bench:          26.46 ns/iter (+/- 0.11)
test encode::lowercase_hex::bench3_2k            ... bench:         166.18 ns/iter (+/- 4.83)
test encode::lowercase_hex::bench4_16k           ... bench:       1,095.94 ns/iter (+/- 32.44)
test encode::lowercase_hex::bench5_128k          ... bench:       8,516.21 ns/iter (+/- 331.74)
test encode::lowercase_hex::bench6_1m            ... bench:      98,046.15 ns/iter (+/- 17,868.56)
test encode_to_slice::faster_hex::bench1_32b     ... bench:          10.94 ns/iter (+/- 0.05)
test encode_to_slice::faster_hex::bench2_256b    ... bench:          29.70 ns/iter (+/- 0.22)
test encode_to_slice::faster_hex::bench3_2k      ... bench:         197.90 ns/iter (+/- 7.33)
test encode_to_slice::faster_hex::bench4_16k     ... bench:       1,506.35 ns/iter (+/- 36.96)
test encode_to_slice::faster_hex::bench5_128k    ... bench:      12,016.58 ns/iter (+/- 317.24)
test encode_to_slice::faster_hex::bench6_1m      ... bench:     122,383.91 ns/iter (+/- 17,791.74)
test encode_to_slice::hex::bench1_32b            ... bench:          31.84 ns/iter (+/- 1.28)
test encode_to_slice::hex::bench2_256b           ... bench:         179.54 ns/iter (+/- 7.68)
test encode_to_slice::hex::bench3_2k             ... bench:       1,398.08 ns/iter (+/- 30.64)
test encode_to_slice::hex::bench4_16k            ... bench:      11,089.17 ns/iter (+/- 205.89)
test encode_to_slice::hex::bench5_128k           ... bench:      89,321.90 ns/iter (+/- 2,078.97)
test encode_to_slice::hex::bench6_1m             ... bench:     768,422.28 ns/iter (+/- 27,858.36)
test encode_to_slice::lowercase_hex::bench1_32b  ... bench:           8.80 ns/iter (+/- 0.04)
test encode_to_slice::lowercase_hex::bench2_256b ... bench:          20.00 ns/iter (+/- 0.56)
test encode_to_slice::lowercase_hex::bench3_2k   ... bench:         135.98 ns/iter (+/- 2.71)
test encode_to_slice::lowercase_hex::bench4_16k  ... bench:       1,069.21 ns/iter (+/- 34.54)
test encode_to_slice::lowercase_hex::bench5_128k ... bench:       8,525.84 ns/iter (+/- 302.04)
test encode_to_slice::lowercase_hex::bench6_1m   ... bench:     100,242.20 ns/iter (+/- 29,819.70)
test format::lowercase_hex::bench1_32b           ... bench:          18.90 ns/iter (+/- 0.44)
test format::lowercase_hex::bench2_256b          ... bench:          52.15 ns/iter (+/- 1.66)
test format::lowercase_hex::bench3_2k            ... bench:         295.99 ns/iter (+/- 11.75)
test format::lowercase_hex::bench4_16k           ... bench:       2,266.58 ns/iter (+/- 76.90)
test format::lowercase_hex::bench5_128k          ... bench:      19,151.19 ns/iter (+/- 442.19)
test format::lowercase_hex::bench6_1m            ... bench:     582,283.03 ns/iter (+/- 244,417.02)
test format::std::bench1_32b                     ... bench:         817.64 ns/iter (+/- 1,609.66)
test format::std::bench2_256b                    ... bench:       5,599.23 ns/iter (+/- 1,526.70)
test format::std::bench3_2k                      ... bench:      46,168.31 ns/iter (+/- 17,688.12)
test format::std::bench4_16k                     ... bench:     354,106.31 ns/iter (+/- 16,870.09)
test format::std::bench5_128k                    ... bench:   2,861,531.80 ns/iter (+/- 79,434.52)
test format::std::bench6_1m                      ... bench:  23,120,824.60 ns/iter (+/- 409,923.02)
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in these crates by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
