use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("{} is not a single RomanDigit value", value),
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }
        let mut digits = Vec::new();
        let table = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for &(val, sym) in &table {
            while num >= val {
                match sym {
                    "M" => digits.push(M),
                    "D" => digits.push(D),
                    "C" => digits.push(C),
                    "L" => digits.push(L),
                    "X" => digits.push(X),
                    "V" => digits.push(V),
                    "I" => digits.push(I),
                    "CM" => {
                        digits.push(C);
                        digits.push(M);
                    }
                    "CD" => {
                        digits.push(C);
                        digits.push(D);
                    }
                    "XC" => {
                        digits.push(X);
                        digits.push(C);
                    }
                    "XL" => {
                        digits.push(X);
                        digits.push(L);
                    }
                    "IX" => {
                        digits.push(I);
                        digits.push(X);
                    }
                    "IV" => {
                        digits.push(I);
                        digits.push(V);
                    }
                    _ => unreachable!(),
                }
                num -= val;
            }
        }
        RomanNumber(digits)
    }
}
