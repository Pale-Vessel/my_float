#![allow(unused)]

#[derive(Copy, Clone)]
struct Float {
    data: u32,
}

#[derive(PartialEq, Eq, Debug)]
enum Sign {
    Positive,
    Negative,
}

impl Float {
    pub fn from_details(sign: Sign, exponent: u8, mantissa: [u8; 3]) -> Self {
        Self {
            data: (match sign {
                Sign::Positive => 0,
                Sign::Negative => 1,
            } << 31)
                + ((exponent as u32) << 23)
                + u32::from_ne_bytes([0, mantissa[0], mantissa[1], mantissa[2]]),
        }
    }

    pub fn sign(&self) -> Sign {
        if (self.data >> 31) & 1 == 1 {
            Sign::Negative
        } else {
            Sign::Positive
        }
    }

    pub fn exponent(&self) -> u8 {
        ((self.data >> 23) & (u8::MAX as u32)) as u8
    }

    pub fn mantissa(&self) -> [u8; 3] {
        let mantissa_bits = self.data & 0b0000000001111111111111111111111;
        let mantissa_bytes = mantissa_bits.to_ne_bytes();
        [mantissa_bytes[1], mantissa_bytes[2], mantissa_bytes[3]]
    }

    fn to_primitive(self) -> f32 {
        f32::from_bits(self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        for data in 0..u32::MAX {
            let input = Float {data};
            let actual = input.to_primitive();
            assert_eq!(input.sign(), if actual >= 0. {Sign::Positive} else {Sign::Negative})
        }        
    }

    fn test_exponent() {
        let input = Float {
            data: 0b11010110100000000000000000000000,
        };
        assert_eq!(input.exponent(), 0b10101101)
    }

    fn test_mantissa() {
        let input = Float {
            data: 0b0000000001111111111111111111110,
        };
        assert_eq!(input.mantissa(), [255, 255, 254])
    }

    fn test_construction() {
        let input = Float::from_details(Sign::Positive, 234, [255, 72, 159]);
        assert_eq!(input.sign(), Sign::Positive);
        assert_eq!(input.exponent(), 234);
        assert_eq!(input.mantissa(), [255, 72, 159])
    }
}
