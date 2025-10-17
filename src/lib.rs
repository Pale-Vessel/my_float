#![allow(unused)]

#[derive(Copy, Clone)]
struct Float {
    data: u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Sign {
    Positive,
    Negative
}

impl Float {
    fn sign(&self) -> Sign {
        if self.data & 0x80000000 == 0 {
            Sign::Positive
        } else {
            Sign::Negative
        }
    }

    fn exponent(&self) -> u8 {
        ((self.data >> 23) & 0xff) as u8
    }
    
    fn mantissa(&self) -> u32 {
        self.data & 0x7fffff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let float = Float {data : 0xf0000000};
        assert_eq!(float.sign(), Sign::Negative)
    }

    #[test]
    fn test_exponent() {
        #[expect(clippy::unusual_byte_groupings, reason="Separating the exponent on purpose")]
        let float = Float {data: 0b1_01101101_00000000000000000000000};
        assert_eq!(float.exponent(), 0b01101101)
    }

    #[test]
    fn test_mantissa() {
        let float = Float {data: 0b100000000_10101111011110110110111};
        assert_eq!(float.mantissa(), 0b10101111011110110110111)
    }
}