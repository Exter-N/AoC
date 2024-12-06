use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

macro_rules! define_bit_matrix {
    ($name:ident, $row_type:ty) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[repr(transparent)]
        pub struct $name(pub [$row_type; <$row_type>::BITS as usize]);

        impl $name {
            pub const fn zero() -> Self {
                Self([0; <$row_type>::BITS as usize])
            }
            pub fn identity() -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    result[row] = 1 << row;
                }
                Self(result)
            }
            pub fn set(&mut self, row: usize, col: usize, value: bool) {
                if row >= <$row_type>::BITS as usize || col >= <$row_type>::BITS as usize {
                    panic!("indices in bit matrix must be in 0..{}", <$row_type>::BITS);
                }
                if value {
                    self.0[row] |= 1 << col;
                } else {
                    self.0[row] &= !(1 << col);
                }
            }
            pub fn get(&self, row: usize, col: usize) -> bool {
                if row >= <$row_type>::BITS as usize || col >= <$row_type>::BITS as usize {
                    panic!("indices in bit matrix must be in 0..{}", <$row_type>::BITS);
                }
                0 != self.0[row] & (1 << col)
            }
            pub fn or(&self, other: &'_ Self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    result[row] = self.0[row] | other.0[row];
                }
                $name(result)
            }
            pub fn inplace_or(&mut self, other: &'_ Self) {
                for row in 0..(<$row_type>::BITS as usize) {
                    self.0[row] |= other.0[row];
                }
            }
            pub fn and(&self, other: &'_ Self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    result[row] = self.0[row] & other.0[row];
                }
                $name(result)
            }
            pub fn inplace_and(&mut self, other: &'_ Self) {
                for row in 0..(<$row_type>::BITS as usize) {
                    self.0[row] &= other.0[row];
                }
            }
            pub fn xor(&self, other: &'_ Self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    result[row] = self.0[row] ^ other.0[row];
                }
                $name(result)
            }
            pub fn inplace_xor(&mut self, other: &'_ Self) {
                for row in 0..(<$row_type>::BITS as usize) {
                    self.0[row] ^= other.0[row];
                }
            }
            pub fn not(&self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    result[row] = !self.0[row];
                }
                $name(result)
            }
            pub fn inplace_not(&mut self) {
                for row in 0..(<$row_type>::BITS as usize) {
                    self.0[row] = !self.0[row];
                }
            }
            pub fn or_mul(&self, other: &'_ Self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    let self_row = self.0[row];
                    for col in 0..(<$row_type>::BITS as usize) {
                        if 0 != self_row & (1 << col) {
                            result[row] |= other.0[col];
                        }
                    }
                }
                Self(result)
            }
            pub fn inplace_or_mul(&mut self, other: &'_ Self) {
                *self = self.or_mul(other)
            }
            pub fn or_pow_infinity(&self) -> Self {
                let mut result = self.or_mul(self);
                if &result == self {
                    return result;
                }
                loop {
                    let result2 = result.or_mul(&result);
                    if &result2 == &result {
                        return result2;
                    }
                    result = result2.or_mul(&result2);
                    if &result == &result2 {
                        return result;
                    }
                }
            }
            pub fn inplace_or_pow_infinity(&mut self) {
                loop {
                    let result = self.or_mul(self);
                    if &result == self {
                        return;
                    }
                    *self = result.or_mul(&result);
                    if self == &result {
                        return;
                    }
                }
            }
            pub fn xor_mul(&self, other: &'_ Self) -> Self {
                let mut result = [0; <$row_type>::BITS as usize];
                for row in 0..(<$row_type>::BITS as usize) {
                    let self_row = self.0[row];
                    for col in 0..(<$row_type>::BITS as usize) {
                        if 0 != self_row & (1 << col) {
                            result[row] ^= other.0[col];
                        }
                    }
                }
                Self(result)
            }
            pub fn inplace_xor_mul(&mut self, other: &'_ Self) {
                *self = self.xor_mul(other)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self([Default::default(); <$row_type>::BITS as usize])
            }
        }

        impl BitOr<$name> for $name {
            type Output = $name;

            fn bitor(self, other: Self) -> Self {
                self.or(&other)
            }
        }

        impl BitOrAssign<Self> for $name {
            fn bitor_assign(&mut self, other: Self) {
                self.inplace_or(&other)
            }
        }

        impl BitAnd<$name> for $name {
            type Output = $name;

            fn bitand(self, other: Self) -> Self {
                self.and(&other)
            }
        }

        impl BitAndAssign<Self> for $name {
            fn bitand_assign(&mut self, other: Self) {
                self.inplace_and(&other)
            }
        }

        impl BitXor<$name> for $name {
            type Output = $name;

            fn bitxor(self, other: Self) -> Self {
                self.xor(&other)
            }
        }

        impl BitXorAssign<Self> for $name {
            fn bitxor_assign(&mut self, other: Self) {
                self.inplace_xor(&other)
            }
        }

        impl Not for $name {
            type Output = $name;

            fn not(self) -> $name {
                (&self).not()
            }
        }
    };
}

define_bit_matrix!(BitMatrix8, u8);
define_bit_matrix!(BitMatrix16, u16);
define_bit_matrix!(BitMatrix32, u32);
define_bit_matrix!(BitMatrix64, u64);
define_bit_matrix!(BitMatrix128, u128);
