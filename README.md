<h1 align="center">Graviola</h1>
<img width="40%" align="right" src="https://raw.githubusercontent.com/ctz/graviola/main/admin/picture.png">

> **Graviola** is a compendium of **high quality**,
> **fast** and **easy to build** cryptography for Rust, aimed
> at use with [rustls](https://github.com/rustls/rustls).

*High quality*: Graviola incorporates assembler routines
from the [s2n-bignum] project.  These have been formally proven
to be memory safe, free of side channels (at the architectural level),
and to correctly implement the desired mathematical operation.  They
are also high performance: using (in part) [SLOTHY].

*Fast*: Graviola beats or is competitive with other cryptography
libraries for Rust.  See [performance](#performance).

*Easy and fast to build*: no C compiler, assembler or other tooling
needed: just the Rust compiler.  Compiles in less than one second.

## Status

Active development.  Do not use.  Currently external contributions
are not welcomed; please do not file issues or PRs.

## TODO list, and ideas

- [ ] interdict cpuid usage and test all combinations
- [x] aarch64 sha2 using intrinsics
- [ ] 4-wide ghash for aarch64
- [ ] wide gcm for aarch64
- [x] p384
- [ ] source-based automated interleaving for intrinsic code
- [ ] add CI for enforcing SPDX header

## Goals

- [x] Fast and simple compilation
    - [x] `cargo build` takes less than one second, and requires only rustc
- [x] Competitive performance (with *ring*, aws-lc-rs, and rustcrypto)
- [x] Uses formally-verified assembler from other projects (where available)
- [ ] Intended to provide algorithms in wide use on web
- [ ] Intended for use as a rustls `CryptoProvider`

## Limitations

- `target_arch = "aarch64"` and `target_arch = "x86_64"` only
    - aarch64 requires `aes` and `neon` CPU features.
    - x86_64 requires `aes`, `ssse3` and `pclmulqdq` CPU features.

## Acknowledgements and Thanks

Graviola incorporates significant code from other open source projects.
We are grateful to:

- [s2n-bignum]: formally verified assembler for
    - P256, P384, P521 field arithmetic and group operations
    - x25519
    - Big integer arithmetic
- [wycheproof]: collated test vectors for all algorithms.

[s2n-bignum]: https://github.com/awslabs/s2n-bignum
[wycheproof]: https://github.com/C2SP/wycheproof
[SLOTHY]: https://github.com/slothy-optimizer/slothy

## Algorithms

### Public key signatures

- [x] RSA-PSS signature verification
- [x] RSA-PKCS#1 signature verification
- [x] RSA-PSS signing
- [x] RSA-PKCS#1 signing
- [x] ECDSA on P256 w/ SHA2
- [x] ECDSA on P384 w/ SHA2

### Hashing

- [x] SHA256
- [x] SHA384 & SHA512
- [x] HMAC
- [x] HMAC-DRBG

### Key exchange

- [x] X25519
- [x] P256
- [x] P384

### AEADs

- [x] AES-GCM
- [x] chacha20-poly1305

## Performance

See: https://jbp.io/graviola/

## Architecture

We have broadly three module layers:

- `low`: low level primitives. private. platform-specific. unsafe allowed. `no_std`. no alloc.
- `mid`: constructions, protocols and encodings. private. platform agnostic. no unsafe. `no_std`. alloc.
- `high`: public interface, primarily a rustls `CryptoProvider`. platform agnostic. no unsafe.

`low` code should not refer to `mid`, nor `mid` to `high`.

`low` must present the same interface irrespective of platform.  To this end,
`low::generic` contains pure-rust polyfills for items we don't have assembler-
or intrinsic-based implementations for a certain platform.

## License

Graviola incorporates and redistributes code from:

- [s2n-bignum]: Apache-2.0 OR ISC or MIT-0

New code written for Graviola is licensed under
Apache-2.0 OR ISC OR MIT-0.

Because we have a mix of licenses, every file has a
`SPDX-License-Identifier` comment.
