#![allow(non_upper_case_globals, unused_macros, unused_imports)]
use crate::low::macros::*;

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT-0

// ----------------------------------------------------------------------------
// Square, z := x^2
// Input x[16]; output z[32]; temporary buffer t[>=24]
//
//    extern void bignum_ksqr_16_32
//     (uint64_t z[static 32], uint64_t x[static 16], uint64_t t[static 24]);
//
// This is a Karatsuba-style function squaring half-sized results
// and using temporary buffer t for intermediate results.
//
// Standard ARM ABI: X0 = z, X1 = x, X2 = t
// ----------------------------------------------------------------------------

pub(crate) fn bignum_ksqr_16_32(z: &mut [u64], x: &[u64], t: &mut [u64; 24]) {
    debug_assert!(z.len() == 32);
    debug_assert!(x.len() == 16);
    // SAFETY: inline assembly. see [crate::low::inline_assembly_safety] for safety info.
    unsafe {
        core::arch::asm!(

        Q!("    stp             " "x19, x20, [sp, #-16] !"),
        Q!("    stp             " "x21, x22, [sp, #-16] !"),
        Q!("    stp             " "x23, x24, [sp, #-16] !"),
        Q!("    stp             " "x25, x30, [sp, #-16] !"),
        Q!("    mov             " "x23, x0"),
        Q!("    mov             " "x24, x1"),
        Q!("    mov             " "x25, x2"),
        Q!("    bl              " Label!("bignum_ksqr_16_32_local_sqr_8_16", 2, After)),
        Q!("    ldp             " "x10, x11, [x24]"),
        Q!("    ldp             " "x8, x9, [x24, #64]"),
        Q!("    subs            " "x10, x10, x8"),
        Q!("    sbcs            " "x11, x11, x9"),
        Q!("    ldp             " "x12, x13, [x24, #16]"),
        Q!("    ldp             " "x8, x9, [x24, #80]"),
        Q!("    sbcs            " "x12, x12, x8"),
        Q!("    sbcs            " "x13, x13, x9"),
        Q!("    ldp             " "x14, x15, [x24, #32]"),
        Q!("    ldp             " "x8, x9, [x24, #96]"),
        Q!("    sbcs            " "x14, x14, x8"),
        Q!("    sbcs            " "x15, x15, x9"),
        Q!("    ldp             " "x16, x17, [x24, #48]"),
        Q!("    ldp             " "x8, x9, [x24, #112]"),
        Q!("    sbcs            " "x16, x16, x8"),
        Q!("    sbcs            " "x17, x17, x9"),
        Q!("    csetm           " "x19, cc"),
        Q!("    cmn             " "x19, x19"),
        Q!("    eor             " "x10, x10, x19"),
        Q!("    adcs            " "x10, x10, xzr"),
        Q!("    eor             " "x11, x11, x19"),
        Q!("    adcs            " "x11, x11, xzr"),
        Q!("    stp             " "x10, x11, [x25]"),
        Q!("    eor             " "x12, x12, x19"),
        Q!("    adcs            " "x12, x12, xzr"),
        Q!("    eor             " "x13, x13, x19"),
        Q!("    adcs            " "x13, x13, xzr"),
        Q!("    stp             " "x12, x13, [x25, #16]"),
        Q!("    eor             " "x14, x14, x19"),
        Q!("    adcs            " "x14, x14, xzr"),
        Q!("    eor             " "x15, x15, x19"),
        Q!("    adcs            " "x15, x15, xzr"),
        Q!("    stp             " "x14, x15, [x25, #32]"),
        Q!("    eor             " "x16, x16, x19"),
        Q!("    adcs            " "x16, x16, xzr"),
        Q!("    eor             " "x17, x17, x19"),
        Q!("    adcs            " "x17, x17, xzr"),
        Q!("    stp             " "x16, x17, [x25, #48]"),
        Q!("    add             " "x0, x23, #0x80"),
        Q!("    add             " "x1, x24, #0x40"),
        Q!("    bl              " Label!("bignum_ksqr_16_32_local_sqr_8_16", 2, After)),
        Q!("    ldp             " "x10, x11, [x23, #128]"),
        Q!("    ldp             " "x12, x13, [x23, #64]"),
        Q!("    adds            " "x10, x10, x12"),
        Q!("    adcs            " "x11, x11, x13"),
        Q!("    stp             " "x10, x11, [x23, #128]"),
        Q!("    ldp             " "x10, x11, [x23, #144]"),
        Q!("    ldp             " "x12, x13, [x23, #80]"),
        Q!("    adcs            " "x10, x10, x12"),
        Q!("    adcs            " "x11, x11, x13"),
        Q!("    stp             " "x10, x11, [x23, #144]"),
        Q!("    ldp             " "x10, x11, [x23, #160]"),
        Q!("    ldp             " "x12, x13, [x23, #96]"),
        Q!("    adcs            " "x10, x10, x12"),
        Q!("    adcs            " "x11, x11, x13"),
        Q!("    stp             " "x10, x11, [x23, #160]"),
        Q!("    ldp             " "x10, x11, [x23, #176]"),
        Q!("    ldp             " "x12, x13, [x23, #112]"),
        Q!("    adcs            " "x10, x10, x12"),
        Q!("    adcs            " "x11, x11, x13"),
        Q!("    stp             " "x10, x11, [x23, #176]"),
        Q!("    ldp             " "x10, x11, [x23, #192]"),
        Q!("    adcs            " "x10, x10, xzr"),
        Q!("    adcs            " "x11, x11, xzr"),
        Q!("    stp             " "x10, x11, [x23, #192]"),
        Q!("    ldp             " "x10, x11, [x23, #208]"),
        Q!("    adcs            " "x10, x10, xzr"),
        Q!("    adcs            " "x11, x11, xzr"),
        Q!("    stp             " "x10, x11, [x23, #208]"),
        Q!("    ldp             " "x10, x11, [x23, #224]"),
        Q!("    adcs            " "x10, x10, xzr"),
        Q!("    adcs            " "x11, x11, xzr"),
        Q!("    stp             " "x10, x11, [x23, #224]"),
        Q!("    ldp             " "x10, x11, [x23, #240]"),
        Q!("    adcs            " "x10, x10, xzr"),
        Q!("    adcs            " "x11, x11, xzr"),
        Q!("    stp             " "x10, x11, [x23, #240]"),
        Q!("    add             " "x0, x25, #0x40"),
        Q!("    mov             " "x1, x25"),
        Q!("    bl              " Label!("bignum_ksqr_16_32_local_sqr_8_16", 2, After)),
        Q!("    ldp             " "x0, x1, [x23]"),
        Q!("    ldp             " "x16, x17, [x23, #128]"),
        Q!("    adds            " "x0, x0, x16"),
        Q!("    adcs            " "x1, x1, x17"),
        Q!("    ldp             " "x2, x3, [x23, #16]"),
        Q!("    ldp             " "x16, x17, [x23, #144]"),
        Q!("    adcs            " "x2, x2, x16"),
        Q!("    adcs            " "x3, x3, x17"),
        Q!("    ldp             " "x4, x5, [x23, #32]"),
        Q!("    ldp             " "x16, x17, [x23, #160]"),
        Q!("    adcs            " "x4, x4, x16"),
        Q!("    adcs            " "x5, x5, x17"),
        Q!("    ldp             " "x6, x7, [x23, #48]"),
        Q!("    ldp             " "x16, x17, [x23, #176]"),
        Q!("    adcs            " "x6, x6, x16"),
        Q!("    adcs            " "x7, x7, x17"),
        Q!("    ldp             " "x8, x9, [x23, #128]"),
        Q!("    ldp             " "x16, x17, [x23, #192]"),
        Q!("    adcs            " "x8, x8, x16"),
        Q!("    adcs            " "x9, x9, x17"),
        Q!("    ldp             " "x10, x11, [x23, #144]"),
        Q!("    ldp             " "x16, x17, [x23, #208]"),
        Q!("    adcs            " "x10, x10, x16"),
        Q!("    adcs            " "x11, x11, x17"),
        Q!("    ldp             " "x12, x13, [x23, #160]"),
        Q!("    ldp             " "x16, x17, [x23, #224]"),
        Q!("    adcs            " "x12, x12, x16"),
        Q!("    adcs            " "x13, x13, x17"),
        Q!("    ldp             " "x14, x15, [x23, #176]"),
        Q!("    ldp             " "x16, x17, [x23, #240]"),
        Q!("    adcs            " "x14, x14, x16"),
        Q!("    adcs            " "x15, x15, x17"),
        Q!("    cset            " "x24, cs"),
        Q!("    ldp             " "x16, x17, [x25, #64]"),
        Q!("    subs            " "x0, x0, x16"),
        Q!("    sbcs            " "x1, x1, x17"),
        Q!("    stp             " "x0, x1, [x23, #64]"),
        Q!("    ldp             " "x16, x17, [x25, #80]"),
        Q!("    sbcs            " "x2, x2, x16"),
        Q!("    sbcs            " "x3, x3, x17"),
        Q!("    stp             " "x2, x3, [x23, #80]"),
        Q!("    ldp             " "x16, x17, [x25, #96]"),
        Q!("    sbcs            " "x4, x4, x16"),
        Q!("    sbcs            " "x5, x5, x17"),
        Q!("    stp             " "x4, x5, [x23, #96]"),
        Q!("    ldp             " "x16, x17, [x25, #112]"),
        Q!("    sbcs            " "x6, x6, x16"),
        Q!("    sbcs            " "x7, x7, x17"),
        Q!("    stp             " "x6, x7, [x23, #112]"),
        Q!("    ldp             " "x16, x17, [x25, #128]"),
        Q!("    sbcs            " "x8, x8, x16"),
        Q!("    sbcs            " "x9, x9, x17"),
        Q!("    stp             " "x8, x9, [x23, #128]"),
        Q!("    ldp             " "x16, x17, [x25, #144]"),
        Q!("    sbcs            " "x10, x10, x16"),
        Q!("    sbcs            " "x11, x11, x17"),
        Q!("    stp             " "x10, x11, [x23, #144]"),
        Q!("    ldp             " "x16, x17, [x25, #160]"),
        Q!("    sbcs            " "x12, x12, x16"),
        Q!("    sbcs            " "x13, x13, x17"),
        Q!("    stp             " "x12, x13, [x23, #160]"),
        Q!("    ldp             " "x16, x17, [x25, #176]"),
        Q!("    sbcs            " "x14, x14, x16"),
        Q!("    sbcs            " "x15, x15, x17"),
        Q!("    stp             " "x14, x15, [x23, #176]"),
        Q!("    sbcs            " "x24, x24, xzr"),
        Q!("    csetm           " "x25, cc"),
        Q!("    ldp             " "x10, x11, [x23, #192]"),
        Q!("    adds            " "x10, x10, x24"),
        Q!("    adcs            " "x11, x11, x25"),
        Q!("    stp             " "x10, x11, [x23, #192]"),
        Q!("    ldp             " "x10, x11, [x23, #208]"),
        Q!("    adcs            " "x10, x10, x25"),
        Q!("    adcs            " "x11, x11, x25"),
        Q!("    stp             " "x10, x11, [x23, #208]"),
        Q!("    ldp             " "x10, x11, [x23, #224]"),
        Q!("    adcs            " "x10, x10, x25"),
        Q!("    adcs            " "x11, x11, x25"),
        Q!("    stp             " "x10, x11, [x23, #224]"),
        Q!("    ldp             " "x10, x11, [x23, #240]"),
        Q!("    adcs            " "x10, x10, x25"),
        Q!("    adcs            " "x11, x11, x25"),
        Q!("    stp             " "x10, x11, [x23, #240]"),
        Q!("    ldp             " "x25, x30, [sp], #16"),
        Q!("    ldp             " "x23, x24, [sp], #16"),
        Q!("    ldp             " "x21, x22, [sp], #16"),
        Q!("    ldp             " "x19, x20, [sp], #16"),
        // proc hoisting in -> ret after bignum_ksqr_16_32_local_sqr_8_16
        Q!("    b               " Label!("hoist_finish", 3, After)),

        Q!(Label!("bignum_ksqr_16_32_local_sqr_8_16", 2) ":"),
        Q!("    ldp             " "x2, x3, [x1]"),
        Q!("    ldp             " "x4, x5, [x1, #16]"),
        Q!("    ldp             " "x6, x7, [x1, #32]"),
        Q!("    ldp             " "x8, x9, [x1, #48]"),
        Q!("    mul             " "x17, x2, x4"),
        Q!("    mul             " "x14, x3, x5"),
        Q!("    umulh           " "x20, x2, x4"),
        Q!("    subs            " "x21, x2, x3"),
        Q!("    cneg            " "x21, x21, cc"),
        Q!("    csetm           " "x11, cc"),
        Q!("    subs            " "x12, x5, x4"),
        Q!("    cneg            " "x12, x12, cc"),
        Q!("    mul             " "x13, x21, x12"),
        Q!("    umulh           " "x12, x21, x12"),
        Q!("    cinv            " "x11, x11, cc"),
        Q!("    eor             " "x13, x13, x11"),
        Q!("    eor             " "x12, x12, x11"),
        Q!("    adds            " "x19, x17, x20"),
        Q!("    adc             " "x20, x20, xzr"),
        Q!("    umulh           " "x21, x3, x5"),
        Q!("    adds            " "x19, x19, x14"),
        Q!("    adcs            " "x20, x20, x21"),
        Q!("    adc             " "x21, x21, xzr"),
        Q!("    adds            " "x20, x20, x14"),
        Q!("    adc             " "x21, x21, xzr"),
        Q!("    cmn             " "x11, #0x1"),
        Q!("    adcs            " "x19, x19, x13"),
        Q!("    adcs            " "x20, x20, x12"),
        Q!("    adc             " "x21, x21, x11"),
        Q!("    adds            " "x17, x17, x17"),
        Q!("    adcs            " "x19, x19, x19"),
        Q!("    adcs            " "x20, x20, x20"),
        Q!("    adcs            " "x21, x21, x21"),
        Q!("    adc             " "x10, xzr, xzr"),
        Q!("    mul             " "x12, x2, x2"),
        Q!("    mul             " "x13, x3, x3"),
        Q!("    mul             " "x15, x2, x3"),
        Q!("    umulh           " "x11, x2, x2"),
        Q!("    umulh           " "x14, x3, x3"),
        Q!("    umulh           " "x16, x2, x3"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    stp             " "x12, x11, [x0]"),
        Q!("    adds            " "x17, x17, x13"),
        Q!("    adcs            " "x19, x19, x14"),
        Q!("    adcs            " "x20, x20, xzr"),
        Q!("    adcs            " "x21, x21, xzr"),
        Q!("    adc             " "x10, x10, xzr"),
        Q!("    stp             " "x17, x19, [x0, #16]"),
        Q!("    mul             " "x12, x4, x4"),
        Q!("    mul             " "x13, x5, x5"),
        Q!("    mul             " "x15, x4, x5"),
        Q!("    umulh           " "x11, x4, x4"),
        Q!("    umulh           " "x14, x5, x5"),
        Q!("    umulh           " "x16, x4, x5"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x12, x12, x20"),
        Q!("    adcs            " "x11, x11, x21"),
        Q!("    stp             " "x12, x11, [x0, #32]"),
        Q!("    adcs            " "x13, x13, x10"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    stp             " "x13, x14, [x0, #48]"),
        Q!("    mul             " "x17, x6, x8"),
        Q!("    mul             " "x14, x7, x9"),
        Q!("    umulh           " "x20, x6, x8"),
        Q!("    subs            " "x21, x6, x7"),
        Q!("    cneg            " "x21, x21, cc"),
        Q!("    csetm           " "x11, cc"),
        Q!("    subs            " "x12, x9, x8"),
        Q!("    cneg            " "x12, x12, cc"),
        Q!("    mul             " "x13, x21, x12"),
        Q!("    umulh           " "x12, x21, x12"),
        Q!("    cinv            " "x11, x11, cc"),
        Q!("    eor             " "x13, x13, x11"),
        Q!("    eor             " "x12, x12, x11"),
        Q!("    adds            " "x19, x17, x20"),
        Q!("    adc             " "x20, x20, xzr"),
        Q!("    umulh           " "x21, x7, x9"),
        Q!("    adds            " "x19, x19, x14"),
        Q!("    adcs            " "x20, x20, x21"),
        Q!("    adc             " "x21, x21, xzr"),
        Q!("    adds            " "x20, x20, x14"),
        Q!("    adc             " "x21, x21, xzr"),
        Q!("    cmn             " "x11, #0x1"),
        Q!("    adcs            " "x19, x19, x13"),
        Q!("    adcs            " "x20, x20, x12"),
        Q!("    adc             " "x21, x21, x11"),
        Q!("    adds            " "x17, x17, x17"),
        Q!("    adcs            " "x19, x19, x19"),
        Q!("    adcs            " "x20, x20, x20"),
        Q!("    adcs            " "x21, x21, x21"),
        Q!("    adc             " "x10, xzr, xzr"),
        Q!("    mul             " "x12, x6, x6"),
        Q!("    mul             " "x13, x7, x7"),
        Q!("    mul             " "x15, x6, x7"),
        Q!("    umulh           " "x11, x6, x6"),
        Q!("    umulh           " "x14, x7, x7"),
        Q!("    umulh           " "x16, x6, x7"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    stp             " "x12, x11, [x0, #64]"),
        Q!("    adds            " "x17, x17, x13"),
        Q!("    adcs            " "x19, x19, x14"),
        Q!("    adcs            " "x20, x20, xzr"),
        Q!("    adcs            " "x21, x21, xzr"),
        Q!("    adc             " "x10, x10, xzr"),
        Q!("    stp             " "x17, x19, [x0, #80]"),
        Q!("    mul             " "x12, x8, x8"),
        Q!("    mul             " "x13, x9, x9"),
        Q!("    mul             " "x15, x8, x9"),
        Q!("    umulh           " "x11, x8, x8"),
        Q!("    umulh           " "x14, x9, x9"),
        Q!("    umulh           " "x16, x8, x9"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x11, x11, x15"),
        Q!("    adcs            " "x13, x13, x16"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    adds            " "x12, x12, x20"),
        Q!("    adcs            " "x11, x11, x21"),
        Q!("    stp             " "x12, x11, [x0, #96]"),
        Q!("    adcs            " "x13, x13, x10"),
        Q!("    adc             " "x14, x14, xzr"),
        Q!("    stp             " "x13, x14, [x0, #112]"),
        Q!("    mul             " "x10, x2, x6"),
        Q!("    mul             " "x14, x3, x7"),
        Q!("    mul             " "x15, x4, x8"),
        Q!("    mul             " "x16, x5, x9"),
        Q!("    umulh           " "x17, x2, x6"),
        Q!("    adds            " "x14, x14, x17"),
        Q!("    umulh           " "x17, x3, x7"),
        Q!("    adcs            " "x15, x15, x17"),
        Q!("    umulh           " "x17, x4, x8"),
        Q!("    adcs            " "x16, x16, x17"),
        Q!("    umulh           " "x17, x5, x9"),
        Q!("    adc             " "x17, x17, xzr"),
        Q!("    adds            " "x11, x14, x10"),
        Q!("    adcs            " "x14, x15, x14"),
        Q!("    adcs            " "x15, x16, x15"),
        Q!("    adcs            " "x16, x17, x16"),
        Q!("    adc             " "x17, xzr, x17"),
        Q!("    adds            " "x12, x14, x10"),
        Q!("    adcs            " "x13, x15, x11"),
        Q!("    adcs            " "x14, x16, x14"),
        Q!("    adcs            " "x15, x17, x15"),
        Q!("    adcs            " "x16, xzr, x16"),
        Q!("    adc             " "x17, xzr, x17"),
        Q!("    subs            " "x22, x4, x5"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x9, x8"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x15, x15, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x16, x16, x20"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    subs            " "x22, x2, x3"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x7, x6"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x11, x11, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x12, x12, x20"),
        Q!("    adcs            " "x13, x13, x19"),
        Q!("    adcs            " "x14, x14, x19"),
        Q!("    adcs            " "x15, x15, x19"),
        Q!("    adcs            " "x16, x16, x19"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    subs            " "x22, x3, x5"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x9, x7"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x14, x14, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x15, x15, x20"),
        Q!("    adcs            " "x16, x16, x19"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    subs            " "x22, x2, x4"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x8, x6"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x12, x12, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x13, x13, x20"),
        Q!("    adcs            " "x14, x14, x19"),
        Q!("    adcs            " "x15, x15, x19"),
        Q!("    adcs            " "x16, x16, x19"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    subs            " "x22, x2, x5"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x9, x6"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x13, x13, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x14, x14, x20"),
        Q!("    adcs            " "x15, x15, x19"),
        Q!("    adcs            " "x16, x16, x19"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    subs            " "x22, x3, x4"),
        Q!("    cneg            " "x22, x22, cc"),
        Q!("    csetm           " "x19, cc"),
        Q!("    subs            " "x20, x8, x7"),
        Q!("    cneg            " "x20, x20, cc"),
        Q!("    mul             " "x21, x22, x20"),
        Q!("    umulh           " "x20, x22, x20"),
        Q!("    cinv            " "x19, x19, cc"),
        Q!("    cmn             " "x19, #0x1"),
        Q!("    eor             " "x21, x21, x19"),
        Q!("    adcs            " "x13, x13, x21"),
        Q!("    eor             " "x20, x20, x19"),
        Q!("    adcs            " "x14, x14, x20"),
        Q!("    adcs            " "x15, x15, x19"),
        Q!("    adcs            " "x16, x16, x19"),
        Q!("    adc             " "x17, x17, x19"),
        Q!("    adds            " "x10, x10, x10"),
        Q!("    adcs            " "x11, x11, x11"),
        Q!("    adcs            " "x12, x12, x12"),
        Q!("    adcs            " "x13, x13, x13"),
        Q!("    adcs            " "x14, x14, x14"),
        Q!("    adcs            " "x15, x15, x15"),
        Q!("    adcs            " "x16, x16, x16"),
        Q!("    adcs            " "x17, x17, x17"),
        Q!("    adc             " "x19, xzr, xzr"),
        Q!("    ldp             " "x2, x3, [x0, #32]"),
        Q!("    adds            " "x10, x10, x2"),
        Q!("    adcs            " "x11, x11, x3"),
        Q!("    stp             " "x10, x11, [x0, #32]"),
        Q!("    ldp             " "x2, x3, [x0, #48]"),
        Q!("    adcs            " "x12, x12, x2"),
        Q!("    adcs            " "x13, x13, x3"),
        Q!("    stp             " "x12, x13, [x0, #48]"),
        Q!("    ldp             " "x2, x3, [x0, #64]"),
        Q!("    adcs            " "x14, x14, x2"),
        Q!("    adcs            " "x15, x15, x3"),
        Q!("    stp             " "x14, x15, [x0, #64]"),
        Q!("    ldp             " "x2, x3, [x0, #80]"),
        Q!("    adcs            " "x16, x16, x2"),
        Q!("    adcs            " "x17, x17, x3"),
        Q!("    stp             " "x16, x17, [x0, #80]"),
        Q!("    ldp             " "x2, x3, [x0, #96]"),
        Q!("    adcs            " "x2, x2, x19"),
        Q!("    adcs            " "x3, x3, xzr"),
        Q!("    stp             " "x2, x3, [x0, #96]"),
        Q!("    ldp             " "x2, x3, [x0, #112]"),
        Q!("    adcs            " "x2, x2, xzr"),
        Q!("    adc             " "x3, x3, xzr"),
        Q!("    stp             " "x2, x3, [x0, #112]"),
        Q!("    ret             " ),
        Q!(Label!("hoist_finish", 3) ":"),
        inout("x0") z.as_mut_ptr() => _,
        inout("x1") x.as_ptr() => _,
        inout("x2") t.as_mut_ptr() => _,
        // clobbers
        out("x10") _,
        out("x11") _,
        out("x12") _,
        out("x13") _,
        out("x14") _,
        out("x15") _,
        out("x16") _,
        out("x17") _,
        out("x20") _,
        out("x21") _,
        out("x22") _,
        out("x23") _,
        out("x24") _,
        out("x25") _,
        out("x3") _,
        out("x30") _,
        out("x4") _,
        out("x5") _,
        out("x6") _,
        out("x7") _,
        out("x8") _,
        out("x9") _,
            )
    };
}
