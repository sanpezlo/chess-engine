use crate::macros::{create_enum, enum_str};

create_enum! {
    /// A `CastleRightsType` in chess.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CastleRightsType {
        /// No castle rights.
        None,
        /// King side castle rights.
        KingSide,
        /// Queen side castle rights.
        QueenSide,
        /// Both king and queen side castle rights.
        Both
    }
}

enum_str! {
    CastleRightsType, CastleRightsTypeError {
        None = "",
        KingSide = "k",
        QueenSide = "q",
        Both = "kq"
    }
}

impl CastleRightsType {
    /// Performs a bitwise OR operation on two `CastleRightsType`.
    ///
    /// # Errors
    ///
    /// Returns a [`CastleRightsTypeError`] if the result is not a valid castle rights.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_core::*;
    /// let mut castle_rights_type = CastleRightsType::None;
    /// castle_rights_type.try_bitor_assign(CastleRightsType::KingSide).unwrap();
    /// assert_eq!(castle_rights_type, CastleRightsType::KingSide);
    /// ```
    pub fn try_bitor_assign(&mut self, rhs: Self) -> Result<(), CastleRightsTypeError> {
        let castle_rights_type = *self as usize | rhs as usize;
        if castle_rights_type < CastleRightsType::LEN {
            *self = CastleRightsType::new(castle_rights_type);
            Ok(())
        } else {
            Err(CastleRightsTypeError(castle_rights_type.to_string()))
        }
    }
}
