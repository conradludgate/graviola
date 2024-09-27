from parse import parse_file
from driver import (
    Architecture_aarch64,
    Architecture_amd64,
    RustDriver,
)

if __name__ == "__main__":
    with open(
        "../../thirdparty/s2n-bignum/x86/p384/bignum_montsqr_p384.S"
    ) as input, open(
        "../../graviola/src/low/x86_64/bignum_montsqr_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_montsqr_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_montsqr_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/x86/p384/bignum_montmul_p384.S"
    ) as input, open(
        "../../graviola/src/low/x86_64/bignum_montmul_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_montmul_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
                ("inout", "rdx", "y.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_montmul_p384(z: &mut [u64; 6], x: &[u64; 6], y: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/bignum_add_p384.S") as input, open(
        "../../graviola/src/low/x86_64/bignum_add_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_add_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
                ("inout", "rdx", "y.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_add_p384(z: &mut [u64; 6], x: &[u64; 6], y: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/x86/p384/bignum_demont_p384.S"
    ) as input, open(
        "../../graviola/src/low/x86_64/bignum_demont_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_demont_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_demont_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/x86/p384/bignum_tomont_p384.S"
    ) as input, open(
        "../../graviola/src/low/x86_64/bignum_tomont_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_tomont_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_tomont_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/bignum_neg_p384.S") as input, open(
        "../../graviola/src/low/x86_64/bignum_neg_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_neg_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_neg_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/bignum_inv_p384.S") as input, open(
        "../../graviola/src/low/x86_64/bignum_inv_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_inv_p384",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_inv_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/x86/p384/bignum_mod_n384_6.S"
    ) as input, open("../../graviola/src/low/x86_64/bignum_mod_n384.rs", "w") as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "bignum_mod_n384_6",
            parameter_map=[
                ("inout", "rdi", "z.as_mut_ptr() => _"),
                ("inout", "rsi", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_mod_n384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/p384_montjadd.S") as input, open(
        "../../graviola/src/low/x86_64/p384_montjadd.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "p384_montjadd",
            parameter_map=[
                ("inout", "rdi", "p3.as_mut_ptr() => _"),
                ("inout", "rsi", "p1.as_ptr() => _"),
                ("inout", "rdx", "p2.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjadd(p3: &mut [u64; 18], p1: &[u64; 18], p2: &[u64; 18])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/p384_montjmixadd.S") as input, open(
        "../../graviola/src/low/x86_64/p384_montjmixadd.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "p384_montjmixadd",
            parameter_map=[
                ("inout", "rdi", "p3.as_mut_ptr() => _"),
                ("inout", "rsi", "p1.as_ptr() => _"),
                ("inout", "rdx", "p2.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjmixadd(p3: &mut [u64; 18], p1: &[u64; 18], p2: &[u64; 12])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/x86/p384/p384_montjdouble.S") as input, open(
        "../../graviola/src/low/x86_64/p384_montjdouble.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_amd64)
        d.emit_rust_function(
            "p384_montjdouble",
            parameter_map=[
                ("inout", "rdi", "p3.as_mut_ptr() => _"),
                ("inout", "rsi", "p1.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjdouble(p3: &mut [u64; 18], p1: &[u64; 18])",
        )
        parse_file(input, d)

    # aarch64
    with open(
        "../../thirdparty/s2n-bignum/arm/p384/bignum_montsqr_p384.S"
    ) as input, open(
        "../../graviola/src/low/aarch64/bignum_montsqr_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_montsqr_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_montsqr_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/arm/p384/bignum_montmul_p384.S"
    ) as input, open(
        "../../graviola/src/low/aarch64/bignum_montmul_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_montmul_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
                ("inout", "x2", "y.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_montmul_p384(z: &mut [u64; 6], x: &[u64; 6], y: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/bignum_add_p384.S") as input, open(
        "../../graviola/src/low/aarch64/bignum_add_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_add_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
                ("inout", "x2", "y.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_add_p384(z: &mut [u64; 6], x: &[u64; 6], y: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/arm/p384/bignum_demont_p384.S"
    ) as input, open(
        "../../graviola/src/low/aarch64/bignum_demont_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_demont_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_demont_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/arm/p384/bignum_tomont_p384.S"
    ) as input, open(
        "../../graviola/src/low/aarch64/bignum_tomont_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_tomont_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_tomont_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/bignum_neg_p384.S") as input, open(
        "../../graviola/src/low/aarch64/bignum_neg_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_neg_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_neg_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/bignum_inv_p384.S") as input, open(
        "../../graviola/src/low/aarch64/bignum_inv_p384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_inv_p384",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_inv_p384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open(
        "../../thirdparty/s2n-bignum/arm/p384/bignum_mod_n384_6.S"
    ) as input, open(
        "../../graviola/src/low/aarch64/bignum_mod_n384.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "bignum_mod_n384_6",
            parameter_map=[
                ("inout", "x0", "z.as_mut_ptr() => _"),
                ("inout", "x1", "x.as_ptr() => _"),
            ],
            rust_decl="pub fn bignum_mod_n384(z: &mut [u64; 6], x: &[u64; 6])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/p384_montjadd.S") as input, open(
        "../../graviola/src/low/aarch64/p384_montjadd.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "p384_montjadd",
            parameter_map=[
                ("inout", "x0", "p3.as_mut_ptr() => _"),
                ("inout", "x1", "p1.as_ptr() => _"),
                ("inout", "x2", "p2.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjadd(p3: &mut [u64; 18], p1: &[u64; 18], p2: &[u64; 18])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/p384_montjmixadd.S") as input, open(
        "../../graviola/src/low/aarch64/p384_montjmixadd.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "p384_montjmixadd",
            parameter_map=[
                ("inout", "x0", "p3.as_mut_ptr() => _"),
                ("inout", "x1", "p1.as_ptr() => _"),
                ("inout", "x2", "p2.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjmixadd(p3: &mut [u64; 18], p1: &[u64; 18], p2: &[u64; 12])",
        )
        parse_file(input, d)

    with open("../../thirdparty/s2n-bignum/arm/p384/p384_montjdouble.S") as input, open(
        "../../graviola/src/low/aarch64/p384_montjdouble.rs", "w"
    ) as output:
        d = RustDriver(output, Architecture_aarch64)
        d.emit_rust_function(
            "p384_montjdouble",
            parameter_map=[
                ("inout", "x0", "p3.as_mut_ptr() => _"),
                ("inout", "x1", "p1.as_ptr() => _"),
            ],
            rust_decl="pub fn p384_montjdouble(p3: &mut [u64; 18], p1: &[u64; 18])",
        )
        parse_file(input, d)