pub mod code;

pub use self::code::Code;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Feature {
    Bases(i32, Vec<u8>),
    Scores(i32, Vec<u8>),
    ReadBase(i32, u8, u8),
    Substitution(i32, u8),
    Insertion(i32, Vec<u8>),
    Deletion(i32, i32),
    InsertBase(i32, u8),
    QualityScore(i32, u8),
    ReferenceSkip(i32, i32),
    SoftClip(i32, Vec<u8>),
    Padding(i32, i32),
    HardClip(i32, i32),
}

impl Feature {
    pub fn code(&self) -> Code {
        match self {
            Self::Bases(..) => Code::Bases,
            Self::Scores(..) => Code::Scores,
            Self::ReadBase(..) => Code::ReadBase,
            Self::Substitution(..) => Code::Substitution,
            Self::Insertion(..) => Code::Insertion,
            Self::Deletion(..) => Code::Deletion,
            Self::InsertBase(..) => Code::InsertBase,
            Self::QualityScore(..) => Code::QualityScore,
            Self::ReferenceSkip(..) => Code::ReferenceSkip,
            Self::SoftClip(..) => Code::SoftClip,
            Self::Padding(..) => Code::Padding,
            Self::HardClip(..) => Code::HardClip,
        }
    }

    pub fn position(&self) -> i32 {
        match self {
            Self::Bases(pos, _) => *pos,
            Self::Scores(pos, _) => *pos,
            Self::ReadBase(pos, _, _) => *pos,
            Self::Substitution(pos, _) => *pos,
            Self::Insertion(pos, _) => *pos,
            Self::Deletion(pos, _) => *pos,
            Self::InsertBase(pos, _) => *pos,
            Self::QualityScore(pos, _) => *pos,
            Self::ReferenceSkip(pos, _) => *pos,
            Self::SoftClip(pos, _) => *pos,
            Self::Padding(pos, _) => *pos,
            Self::HardClip(pos, _) => *pos,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        assert_eq!(Feature::Bases(1, Vec::new()).code(), Code::Bases);
        assert_eq!(Feature::Scores(1, Vec::new()).code(), Code::Scores);
        assert_eq!(Feature::ReadBase(1, 0, 0).code(), Code::ReadBase);
        assert_eq!(Feature::Substitution(1, 0).code(), Code::Substitution);
        assert_eq!(Feature::Insertion(1, Vec::new()).code(), Code::Insertion);
        assert_eq!(Feature::Deletion(1, 0).code(), Code::Deletion);
        assert_eq!(Feature::InsertBase(1, 0).code(), Code::InsertBase);
        assert_eq!(Feature::QualityScore(1, 0).code(), Code::QualityScore);
        assert_eq!(Feature::ReferenceSkip(1, 0).code(), Code::ReferenceSkip);
        assert_eq!(Feature::SoftClip(1, Vec::new()).code(), Code::SoftClip);
        assert_eq!(Feature::Padding(1, 0).code(), Code::Padding);
        assert_eq!(Feature::HardClip(1, 0).code(), Code::HardClip);
    }

    #[test]
    fn test_position() {
        assert_eq!(Feature::Bases(1, Vec::new()).position(), 1);
        assert_eq!(Feature::Scores(2, Vec::new()).position(), 2);
        assert_eq!(Feature::ReadBase(3, 0, 0).position(), 3);
        assert_eq!(Feature::Substitution(4, 0).position(), 4);
        assert_eq!(Feature::Insertion(5, Vec::new()).position(), 5);
        assert_eq!(Feature::Deletion(6, 0).position(), 6);
        assert_eq!(Feature::InsertBase(7, 0).position(), 7);
        assert_eq!(Feature::QualityScore(8, 0).position(), 8);
        assert_eq!(Feature::ReferenceSkip(9, 0).position(), 9);
        assert_eq!(Feature::SoftClip(10, Vec::new()).position(), 10);
        assert_eq!(Feature::Padding(11, 0).position(), 11);
        assert_eq!(Feature::HardClip(12, 0).position(), 12);
    }
}
