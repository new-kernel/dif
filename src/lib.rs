#![no_std]

#[cfg(feature = "alloc")]
#[macro_use] extern crate alloc;

pub mod fields;
pub mod parse;

pub use fields::*;
//use fields::*;

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

    pub fn new() -> Self {
        let test = ((DifFieldNames::None, ""), (DifFieldNames::None, ""), (DifFieldNames::None, ""), (DifFieldNames::None, ""));

        return Dif {
            dif_name: None,

            #[cfg(feature = "alloc")]
            dif_lines: alloc::vec::Vec::with_capacity(Dif::MAX_SIZE),

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
