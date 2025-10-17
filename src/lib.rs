#![allow(unused)]

use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Copy, Clone)]
struct Float {
    data: u32,
}

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self {
            data: f32::to_bits(value),
        }
    }
}

impl From<Float> for f32 {
    fn from(value: Float) -> Self {
        f32::from_bits(value.data)
    }
}

impl Mul for Float {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let output_sign = if self.sign() == rhs.sign() {
            Sign::Positive
        } else {
            Sign::Negative
        };
        println!("{} {}", self.exponent(), rhs.exponent());
        let output_exponent = self.exponent() + rhs.exponent();
        let output_mantissa = self.mantissa() * rhs.mantissa();
        println!("hi");
        Self::from_raw_parts(output_sign, output_exponent, output_mantissa)
    }
}

impl Float {
    fn sign(&self) -> Sign {
        if self.data & 0x80000000 == 0 {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    fn exponent(&self) -> i8 {
        (((self.data >> 23) & 0xff) as u8).cast_signed()
    }

    fn mantissa(&self) -> u32 {
        self.data & 0x7fffff
    }

    fn from_raw_parts(sign: Sign, exponent: i8, mantissa: u32) -> Self {
        let exponent = unsafe { i8::cast_unsigned(exponent) } as u32;
        let data = match sign {
            Sign::Positive => 0,
            Sign::Negative => 0x80000000,
        } + (exponent << 23)
            + mantissa;
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let float = Float { data: 0xf0000000 };
        assert_eq!(float.sign(), Sign::Negative)
    }

    #[test]
    fn test_exponent() {
        #[expect(
            clippy::unusual_byte_groupings,
            reason = "Separating the exponent on purpose"
        )]
        let float = Float {
            data: 0b1_01101101_00000000000000000000000,
        };
        assert_eq!(float.exponent(), 0b01101101)
    }

    #[test]
    fn test_mantissa() {
        let float = Float {
            data: 0b100000000_10101111011110110110111,
        };
        assert_eq!(float.mantissa(), 0b10101111011110110110111)
    }

    #[test]
    fn multiply_test() {
        let primitive_one = 37.982;
        let primitive_two = 910.2224;
        let float_one: Float = primitive_one.into();
        let float_two: Float = primitive_two.into();
        assert_eq!(
            primitive_one * primitive_two,
            (float_one * float_two).into()
        )
    }
}
