use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            _ => Err(format!("Invalid antigen: {}", s)),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(format!("Invalid Rh factor: {}", s)),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            ord => ord,
        }
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(format!("Invalid blood type string: {}", s));
        }
        let (ant_str, rh_str) = s.split_at(s.len() - 1);
        let antigen = ant_str.parse::<Antigen>()?;
        let rh_factor = rh_str.parse::<RhFactor>()?;
        Ok(BloodType { antigen, rh_factor })
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ant = match &self.antigen {
            Antigen::A => "A",
            Antigen::AB => "AB",
            Antigen::B => "B",
            Antigen::O => "O",
        };
        let rh = match &self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", ant, rh)
    }
}

impl BloodType {
    /// Can self receive blood from other?
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // ABO compatibility
        let abo_ok = match &self.antigen {
            Antigen::O => other.antigen == Antigen::O,
            Antigen::A => other.antigen == Antigen::A || other.antigen == Antigen::O,
            Antigen::B => other.antigen == Antigen::B || other.antigen == Antigen::O,
            Antigen::AB => true,
        };
        // Rh compatibility
        let rh_ok = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            (RhFactor::Negative, RhFactor::Positive) => false,
        };
        abo_ok && rh_ok
    }

    /// All blood types that can donate to self
    pub fn donors(&self) -> Vec<Self> {
        let antigens = [Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let rhos = [RhFactor::Positive, RhFactor::Negative];
        let mut donors = Vec::new();
        for a in &antigens {
            for r in &rhos {
                let bt = BloodType {
                    antigen: a.clone(),
                    rh_factor: r.clone(),
                };
                if self.can_receive_from(&bt) {
                    donors.push(bt);
                }
            }
        }
        donors.sort();
        donors
    }

    /// All blood types that self can donate to
    pub fn recipients(&self) -> Vec<Self> {
        let antigens = [Antigen::A, Antigen::AB, Antigen::B, Antigen::O];
        let rhos = [RhFactor::Positive, RhFactor::Negative];
        let mut recips = Vec::new();
        for a in &antigens {
            for r in &rhos {
                let bt = BloodType {
                    antigen: a.clone(),
                    rh_factor: r.clone(),
                };
                if bt.can_receive_from(self) {
                    recips.push(bt);
                }
            }
        }
        recips.sort();
        recips
    }
}
