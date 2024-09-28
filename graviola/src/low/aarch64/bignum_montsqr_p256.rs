#![allow(non_upper_case_globals, unused_macros, unused_imports)]
use crate::low::macros::*;

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Montgomery square, z := (x^2 / 2^256) mod p_256
// Input x[4]; output z[4]
//
//    extern void bignum_montsqr_p256_neon
//     (uint64_t z[static 4], uint64_t x[static 4]);
//
// Does z := (x^2 / 2^256) mod p_256, assuming x^2 <= 2^256 * p_256, which is
// guaranteed in particular if x < p_256 initially (the "intended" case).
//
// Standard ARM ABI: X0 = z, X1 = x
// ----------------------------------------------------------------------------

// bignum_montsqr_p256_neon is functionally equivalent to bignum_montsqr_p256.
// It is written in a way that
// 1. A subset of scalar multiplications in bignum_montsqr_p256 are carefully
//    chosen and vectorized
// 2. The vectorized assembly is rescheduled using the SLOTHY superoptimizer.
//    https://github.com/slothy-optimizer/slothy
//
// The output program of step 1. is as follows:
//
//        ldp x7, x3, [x1]
//        ldr q6, [x1]
//        ldp x9, x8, [x1, #16]
//        ldr q18, [x1, #16]
//        ldr q27, [x1]
//        umull v16.2D, v27.2S, v27.2S
//        umull2 v17.2D, v27.4S, v27.4S
//        xtn v30.2S, v27.2D
//        uzp2 v27.4S, v27.4S, v27.4S
//        umull v27.2D, v27.2S, v30.2S
//        mov x6, v16.d[0]
//        mov x12, v16.d[1]
//        mov x13, v17.d[0]
//        mov x1, v17.d[1]
//        mov x15, v27.d[0]
//        mov x10, v27.d[1]
//        adds x4, x6, x15, lsl #33
//        lsr x6, x15, #31
//        adc x15, x12, x6
//        adds x13, x13, x10, lsl #33
//        lsr x6, x10, #31
//        adc x12, x1, x6
//        mul x6, x7, x3
//        umulh x1, x7, x3
//        adds x5, x15, x6, lsl #1
//        extr x6, x1, x6, #63
//        adcs x10, x13, x6
//        lsr x6, x1, #63
//        adc x15, x12, x6
//        lsl x6, x4, #32
//        subs x13, x4, x6
//        lsr x12, x4, #32
//        sbc x1, x4, x12
//        adds x6, x5, x6
//        adcs x5, x10, x12
//        adcs x10, x15, x13
//        adc x15, x1, xzr
//        lsl x13, x6, #32
//        subs x12, x6, x13
//        lsr x1, x6, #32
//        sbc x6, x6, x1
//        adds x16, x5, x13
//        adcs x11, x10, x1
//        adcs x2, x15, x12
//        adc x17, x6, xzr
//        uzp1 v30.4S, v18.4S, v6.4S
//        rev64 v27.4S, v18.4S
//        uzp1 v18.4S, v6.4S, v6.4S
//        mul v27.4S, v27.4S, v6.4S
//        uaddlp v5.2D, v27.4S
//        shl v6.2D, v5.2D, #32
//        umlal v6.2D, v18.2S, v30.2S
//        mov x4, v6.d[0]
//        mov x5, v6.d[1]
//        umulh x10, x7, x9
//        subs x6, x7, x3
//        cneg x13, x6, cc
//        csetm x12, cc
//        subs x6, x8, x9
//        cneg x6, x6, cc
//        mul x1, x13, x6
//        umulh x6, x13, x6
//        cinv x15, x12, cc
//        eor x12, x1, x15
//        eor x13, x6, x15
//        adds x1, x4, x10
//        adc x6, x10, xzr
//        umulh x3, x3, x8
//        adds x1, x1, x5
//        adcs x6, x6, x3
//        adc x3, x3, xzr
//        adds x6, x6, x5
//        adc x3, x3, xzr
//        cmn x15, #0x1
//        adcs x12, x1, x12
//        adcs x1, x6, x13
//        adc x3, x3, x15
//        adds x6, x4, x4
//        adcs x13, x12, x12
//        adcs x12, x1, x1
//        adcs x1, x3, x3
//        adc x3, xzr, xzr
//        adds x6, x6, x16
//        adcs x5, x13, x11
//        adcs x10, x12, x2
//        adcs x15, x1, x17
//        adc x13, x3, xzr
//        lsl x3, x6, #32
//        subs x12, x6, x3
//        lsr x1, x6, #32
//        sbc x6, x6, x1
//        adds x3, x5, x3
//        adcs x5, x10, x1
//        adcs x15, x15, x12
//        adcs x13, x13, x6
//        adc x10, xzr, xzr
//        lsl x6, x3, #32
//        subs x12, x3, x6
//        lsr x1, x3, #32
//        sbc x3, x3, x1
//        adds x6, x5, x6
//        adcs x15, x15, x1
//        adcs x13, x13, x12
//        adcs x12, x10, x3
//        adc x1, xzr, xzr
//        mul x3, x9, x9
//        adds x5, x6, x3
//        mul x6, x8, x8
//        umulh x3, x9, x9
//        adcs x15, x15, x3
//        adcs x13, x13, x6
//        umulh x3, x8, x8
//        adcs x12, x12, x3
//        adc x1, x1, xzr
//        mul x6, x9, x8
//        umulh x3, x9, x8
//        adds x8, x6, x6
//        adcs x9, x3, x3
//        adc x3, xzr, xzr
//        adds x10, x15, x8
//        adcs x15, x13, x9
//        adcs x13, x12, x3
//        adcs x12, x1, xzr
//        mov x3, #0xffffffff
//        adds x6, x5, #0x1
//        sbcs x8, x10, x3
//        mov x3, #0xffffffff00000001
//        sbcs x9, x15, xzr
//        sbcs x1, x13, x3
//        sbcs xzr, x12, xzr
//        csel x6, x6, x5, cs
//        csel x8, x8, x10, cs
//        csel x9, x9, x15, cs
//        csel x3, x1, x13, cs
//        stp x6, x8, [x0]                    // @slothy:writes=buffer0
//        stp x9, x3, [x0, #16]               // @slothy:writes=buffer16
//        ret
//
// The bash script used for step 2 is as follows:
//
//        # Store the assembly instructions except the last 'ret' as, say, 'input.S'
//        export OUTPUTS="[hint_buffer0,hint_buffer16]"
//        export RESERVED_REGS="[x18,x19,x20,x21,x22,x23,x24,x25,x26,x27,x28,x29,x30,sp,q8,q9,q10,q11,q12,q13,q14,q15,v8,v9,v10,v11,v12,v13,v14,v15]"
//        <s2n-bignum>/tools/external/slothy.sh input.S my_out_dir
//        # my_out_dir/3.opt.s is the optimized assembly. Its output may differ
//        # from this file since the sequence is non-deterministically chosen.
//        # Please add 'ret' at the end of the output assembly.

pub(crate) fn bignum_montsqr_p256(z: &mut [u64; 4], x: &[u64; 4]) {
    // SAFETY: inline assembly. see [crate::low::inline_assembly_safety] for safety info.
    unsafe {
        core::arch::asm!(


        Q!("    ldr             " "q19, [x1]"),
        Q!("    ldp             " "x9, x13, [x1]"),
        Q!("    ldr             " "q23, [x1, #16]"),
        Q!("    ldr             " "q0, [x1]"),
        Q!("    ldp             " "x1, x10, [x1, #16]"),
        Q!("    uzp2            " "v29.4S, v19.4S, v19.4S"),
        Q!("    xtn             " "v4.2S, v19.2D"),
        Q!("    umulh           " "x8, x9, x13"),
        Q!("    rev64           " "v20.4S, v23.4S"),
        Q!("    umull           " "v16.2D, v19.2S, v19.2S"),
        Q!("    umull           " "v1.2D, v29.2S, v4.2S"),
        Q!("    mul             " "v20.4S, v20.4S, v0.4S"),
        Q!("    subs            " "x14, x9, x13"),
        Q!("    umulh           " "x15, x9, x1"),
        Q!("    mov             " "x16, v16.d[1]"),
        Q!("    umull2          " "v4.2D, v19.4S, v19.4S"),
        Q!("    mov             " "x4, v16.d[0]"),
        Q!("    uzp1            " "v17.4S, v23.4S, v0.4S"),
        Q!("    uaddlp          " "v19.2D, v20.4S"),
        Q!("    lsr             " "x7, x8, #63"),
        Q!("    mul             " "x11, x9, x13"),
        Q!("    mov             " "x12, v1.d[0]"),
        Q!("    csetm           " "x5, cc"),
        Q!("    cneg            " "x6, x14, cc"),
        Q!("    mov             " "x3, v4.d[1]"),
        Q!("    mov             " "x14, v4.d[0]"),
        Q!("    subs            " "x2, x10, x1"),
        Q!("    mov             " "x9, v1.d[1]"),
        Q!("    cneg            " "x17, x2, cc"),
        Q!("    cinv            " "x2, x5, cc"),
        Q!("    adds            " "x5, x4, x12, lsl #33"),
        Q!("    extr            " "x4, x8, x11, #63"),
        Q!("    lsr             " "x8, x12, #31"),
        Q!("    uzp1            " "v20.4S, v0.4S, v0.4S"),
        Q!("    shl             " "v19.2D, v19.2D, #32"),
        Q!("    adc             " "x16, x16, x8"),
        Q!("    adds            " "x8, x14, x9, lsl #33"),
        Q!("    lsr             " "x14, x9, #31"),
        Q!("    lsl             " "x9, x5, #32"),
        Q!("    umlal           " "v19.2D, v20.2S, v17.2S"),
        Q!("    adc             " "x14, x3, x14"),
        Q!("    adds            " "x16, x16, x11, lsl #1"),
        Q!("    lsr             " "x3, x5, #32"),
        Q!("    umulh           " "x12, x6, x17"),
        Q!("    adcs            " "x4, x8, x4"),
        Q!("    adc             " "x11, x14, x7"),
        Q!("    subs            " "x8, x5, x9"),
        Q!("    sbc             " "x5, x5, x3"),
        Q!("    adds            " "x16, x16, x9"),
        Q!("    mov             " "x14, v19.d[0]"),
        Q!("    mul             " "x17, x6, x17"),
        Q!("    adcs            " "x3, x4, x3"),
        Q!("    lsl             " "x7, x16, #32"),
        Q!("    umulh           " "x13, x13, x10"),
        Q!("    adcs            " "x11, x11, x8"),
        Q!("    lsr             " "x8, x16, #32"),
        Q!("    adc             " "x5, x5, xzr"),
        Q!("    subs            " "x9, x16, x7"),
        Q!("    sbc             " "x16, x16, x8"),
        Q!("    adds            " "x7, x3, x7"),
        Q!("    mov             " "x3, v19.d[1]"),
        Q!("    adcs            " "x6, x11, x8"),
        Q!("    umulh           " "x11, x1, x10"),
        Q!("    adcs            " "x5, x5, x9"),
        Q!("    eor             " "x8, x12, x2"),
        Q!("    adc             " "x9, x16, xzr"),
        Q!("    adds            " "x16, x14, x15"),
        Q!("    adc             " "x15, x15, xzr"),
        Q!("    adds            " "x12, x16, x3"),
        Q!("    eor             " "x16, x17, x2"),
        Q!("    mul             " "x4, x1, x10"),
        Q!("    adcs            " "x15, x15, x13"),
        Q!("    adc             " "x17, x13, xzr"),
        Q!("    adds            " "x15, x15, x3"),
        Q!("    adc             " "x3, x17, xzr"),
        Q!("    cmn             " "x2, #0x1"),
        Q!("    mul             " "x17, x10, x10"),
        Q!("    adcs            " "x12, x12, x16"),
        Q!("    adcs            " "x16, x15, x8"),
        Q!("    umulh           " "x10, x10, x10"),
        Q!("    adc             " "x2, x3, x2"),
        Q!("    adds            " "x14, x14, x14"),
        Q!("    adcs            " "x12, x12, x12"),
        Q!("    adcs            " "x16, x16, x16"),
        Q!("    adcs            " "x2, x2, x2"),
        Q!("    adc             " "x15, xzr, xzr"),
        Q!("    adds            " "x14, x14, x7"),
        Q!("    mul             " "x3, x1, x1"),
        Q!("    adcs            " "x12, x12, x6"),
        Q!("    lsr             " "x7, x14, #32"),
        Q!("    adcs            " "x16, x16, x5"),
        Q!("    lsl             " "x5, x14, #32"),
        Q!("    umulh           " "x13, x1, x1"),
        Q!("    adcs            " "x2, x2, x9"),
        Q!("    mov             " "x6, #0xffffffff"),
        Q!("    adc             " "x15, x15, xzr"),
        Q!("    adds            " "x8, x4, x4"),
        Q!("    adcs            " "x1, x11, x11"),
        Q!("    mov             " "x11, #0xffffffff00000001"),
        Q!("    adc             " "x4, xzr, xzr"),
        Q!("    subs            " "x9, x14, x5"),
        Q!("    sbc             " "x14, x14, x7"),
        Q!("    adds            " "x12, x12, x5"),
        Q!("    adcs            " "x16, x16, x7"),
        Q!("    lsl             " "x5, x12, #32"),
        Q!("    lsr             " "x7, x12, #32"),
        Q!("    adcs            " "x2, x2, x9"),
        Q!("    adcs            " "x14, x15, x14"),
        Q!("    adc             " "x15, xzr, xzr"),
        Q!("    subs            " "x9, x12, x5"),
        Q!("    sbc             " "x12, x12, x7"),
        Q!("    adds            " "x16, x16, x5"),
        Q!("    adcs            " "x2, x2, x7"),
        Q!("    adcs            " "x14, x14, x9"),
        Q!("    adcs            " "x12, x15, x12"),
        Q!("    adc             " "x15, xzr, xzr"),
        Q!("    adds            " "x16, x16, x3"),
        Q!("    adcs            " "x2, x2, x13"),
        Q!("    adcs            " "x14, x14, x17"),
        Q!("    adcs            " "x12, x12, x10"),
        Q!("    adc             " "x15, x15, xzr"),
        Q!("    adds            " "x2, x2, x8"),
        Q!("    adcs            " "x14, x14, x1"),
        Q!("    adcs            " "x12, x12, x4"),
        Q!("    adcs            " "x15, x15, xzr"),
        Q!("    adds            " "x3, x16, #0x1"),
        Q!("    sbcs            " "x5, x2, x6"),
        Q!("    sbcs            " "x8, x14, xzr"),
        Q!("    sbcs            " "x11, x12, x11"),
        Q!("    sbcs            " "xzr, x15, xzr"),
        Q!("    csel            " "x16, x3, x16, cs"),
        Q!("    csel            " "x14, x8, x14, cs"),
        Q!("    csel            " "x12, x11, x12, cs"),
        Q!("    csel            " "x2, x5, x2, cs"),
        Q!("    stp             " "x14, x12, [x0, #16]"),
        Q!("    stp             " "x16, x2, [x0]"),
        inout("x0") z.as_mut_ptr() => _,
        inout("x1") x.as_ptr() => _,
        // clobbers
        out("v0") _,
        out("v1") _,
        out("v16") _,
        out("v17") _,
        out("v19") _,
        out("v20") _,
        out("v23") _,
        out("v29") _,
        out("v4") _,
        out("x10") _,
        out("x11") _,
        out("x12") _,
        out("x13") _,
        out("x14") _,
        out("x15") _,
        out("x16") _,
        out("x17") _,
        out("x2") _,
        out("x3") _,
        out("x4") _,
        out("x5") _,
        out("x6") _,
        out("x7") _,
        out("x8") _,
        out("x9") _,
            )
    };
}
