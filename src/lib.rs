#![no_std]

#[cfg(feature = "alloc")]
#[macro_use] extern crate alloc;

pub mod fields;
pub mod get;
pub mod parse;

pub use fields::*;

pub struct Dif {
    pub dif_name: Option<&'static str>,

    #[cfg(feature = "alloc")]
    pub dif_lines: alloc::vec::Vec<DifLine>,

    #[cfg(not(feature = "alloc"))]
    pub dif_lines: (
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine,
        DifLine
    ),
}

impl Dif {
    /// Since there are only 19 field options this is the limit to how many lines this library will
    /// read.
    pub const MAX_SIZE: usize = 19;

    pub const fn new() -> Self {
        return Dif {
            dif_name: None,

            #[cfg(feature = "alloc")]
            dif_lines: vec![],

            #[cfg(not(feature = "alloc"))]
            dif_lines: (
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
                (DifFieldNames::None, ""),
            ),
        };
    }
}
