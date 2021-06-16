use super::apple_sdk_base::{opts, Arch};
use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    let base = opts("watchos", Arch::Arm64_32);
    Target {
        llvm_target: "arm64_32-apple-watchos".to_string(),
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        options: TargetOptions {
            features: "+neon,+fp-armv8,+apple-a7".to_string(),
            max_atomic_width: Some(64),
            unsupported_abis: super::arm_base::unsupported_abis(),
            forces_embed_bitcode: true,
            // These arguments are not actually invoked - they just have
            // to look right to pass App Store validation.
            bitcode_llvm_cmdline: "-triple\0\
                arm64_32-apple-watchos5.0.0\0\
                -emit-obj\0\
                -disable-llvm-passes\0\
                -target-abi\0\
                darwinpcs\0\
                -Os\0"
                .to_string(),
            ..base
        },
    }
}
