use super::macros::{create_enum, enum_str};

create_enum! {
    /// A `File` on a chessboard.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum File {
        /// The File A.
        A,
        /// The File B.
        B,
        /// The File C.
        C,
        /// The File D.
        D,
        /// The File E.
        E,
        /// The File F.
        F,
        /// The File G.
        G,
        /// The File H.
        H,
    }
}

enum_str! {
    File, FileError {
        A = "a",
        B = "b",
        C = "c",
        D = "d",
        E = "e",
        F = "f",
        G = "g",
        H = "h"
    }
}
