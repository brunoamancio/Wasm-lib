// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

pub use bytes::*;
pub use context::*;
pub use corecontracts::*;
pub use exports::ScExports;
pub use hashtypes::*;
pub use immutable::*;
pub use keys::*;
pub use mutable::*;

mod bytes;
mod context;
mod corecontracts;
mod exports;
mod hashtypes;
pub mod host;
mod immutable;
pub mod keys;
mod mutable;

mod utils;

// When the `wee_alloc` feature is enabled,
// use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl std::ops::BitXor for ScHname {
    type Output = Self;

    fn bitxor(self, rhs_schname: Self) -> Self::Output {
        let lhs: Vec<u8> = self.to_bytes();
        let rhs: Vec<u8> = rhs_schname.to_bytes();
        let result : Vec<u8> = lhs.iter()
                        .zip(rhs.iter())
                        .map(|(&x1, &x2)| x1 ^ x2)
                        .collect();
        
        Self::from_bytes(&result)
    }
}

#[cfg(test)]
mod tests {
    use super::ScHname;

    macro_rules! test_xor {
        ($test_name:ident, $lhs : expr, $rhs : expr, $expected :expr) => {
            #[test]
            fn $test_name(){
                let lhs : ScHname = ScHname::from_bytes($lhs);
                let rhs : ScHname = ScHname::from_bytes($rhs);

                let actual_schname : ScHname = lhs ^ rhs;
                let actual_bytes : Vec<u8> = actual_schname.to_bytes();
                
                let expected_bytes : Vec<u8> = $expected;
                // println!("expected: {:?}", expected_bytes);
                // println!("actual: {:?}", actual_bytes);

                assert_eq!(expected_bytes, actual_bytes);
            }
        };
    }

    // First byte
    test_xor!(xor_0001_to_0000, &*vec![0, 0, 0, 1], &*vec![0, 0, 0, 0], vec![0, 0, 0, 1]);
    test_xor!(xor_0001_to_0001, &*vec![0, 0, 0, 1], &*vec![0, 0, 0, 1], vec![0, 0, 0, 0]);
    test_xor!(xor_0000_to_0001, &*vec![0, 0, 0, 0], &*vec![0, 0, 0, 1], vec![0, 0, 0, 1]);

    // Second byte
    test_xor!(xor_0010_to_0000, &*vec![0, 0, 1, 0], &*vec![0, 0, 0, 0], vec![0, 0, 1, 0]);
    test_xor!(xor_0010_to_0010, &*vec![0, 0, 1, 0], &*vec![0, 0, 1, 0], vec![0, 0, 0, 0]);
    test_xor!(xor_0000_to_0010, &*vec![0, 0, 0, 0], &*vec![0, 0, 1, 0], vec![0, 0, 1, 0]);

    // Third byte
    test_xor!(xor_0100_to_0000, &*vec![0, 1, 0, 0], &*vec![0, 0, 0, 0], vec![0, 1, 0, 0]);
    test_xor!(xor_0100_to_0100, &*vec![0, 1, 0, 0], &*vec![0, 1, 0, 0], vec![0, 0, 0, 0]);
    test_xor!(xor_0000_to_0100, &*vec![0, 0, 0, 0], &*vec![0, 1, 0, 0], vec![0, 1, 0, 0]);

    // Fourth byte
    test_xor!(xor_1000_to_0000, &*vec![1, 0, 0, 0], &*vec![0, 0, 0, 0], vec![1, 0, 0, 0]);
    test_xor!(xor_1000_to_1000, &*vec![1, 0, 0, 0], &*vec![1, 0, 0, 0], vec![0, 0, 0, 0]);
    test_xor!(xor_0000_to_1000, &*vec![0, 0, 0, 0], &*vec![1, 0, 0, 0], vec![1, 0, 0, 0]);

    // All equal
    test_xor!(xor_0000_to_0000, &*vec![0, 0, 0, 0], &*vec![0, 0, 0, 0], vec![0, 0, 0, 0]);
    test_xor!(xor_1111_to_1111, &*vec![1, 1, 1, 1], &*vec![1, 1, 1, 1], vec![0, 0, 0, 0]);
}