#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
#[macro_use] extern crate alloc;

pub mod fields;
pub mod get;
pub mod parse;

pub use fields::*;

pub struct Dif {
    /// The name of the DIF that's being used.
    pub dif_name: Option<&'static str>,

    #[cfg(feature = "alloc")]
    /// The lines in the Dif as a ``DifLine``.
    pub dif_lines: alloc::vec::Vec<DifLine>,

    /// The lines in the Dif as a ``DifLine``.
    /// With the ``alloc`` feature enabled this field will be a ``Vec`` instead of a tuple.
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
    /// Since there are only 21 field options this is the limit to how many lines this library will
    /// read.
    pub const MAX_SIZE: usize = 21;

    /// Creates a new ``Dif`` value.
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
