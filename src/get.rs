use crate::{Dif, DifFieldNames};

macro_rules! return_value {
    ($self:ident, $index:expr, $field:ident) => {
        if $self.dif_lines.$index.0 == $field {
            return $self.dif_lines.0.1;
        }
    };
}

impl Dif {
    #[cfg(feature = "alloc")]
    fn alloc_get(&self, field: DifFieldNames) -> &'static str {
        for i in 0..self.dif_lines.len() {
            if self.dif_lines[i].0 == field {
                return  self.dif_lines[i].1;
            }
        }

        return "";
    }

    // This function is very inefficient because it's slow, this needs to be fixed
    #[cfg(not(feature = "alloc"))]
    fn no_alloc_get(&self, field: DifFieldNames) -> &'static str {
        for i in 0..Dif::MAX_SIZE {
            match i {
                0 => {
                    if self.dif_lines.0.0 == field {
                        return self.dif_lines.0.1;
                    }
                    // return_value!(self.dif_lines, 0, field);
                    //return self.dif_lines.0.1
                },
                1 => {
                    if self.dif_lines.1.0 == field {
                        return self.dif_lines.1.1;
                    }
                },
                2 => {
                    if self.dif_lines.2.0 == field {
                        return self.dif_lines.2.1
                    }
                },
                3 => {
                    if self.dif_lines.3.0 == field {
                        return self.dif_lines.3.1
                    }
                },
                4 => {
                    if self.dif_lines.4.0 == field {
                        return self.dif_lines.4.1
                    }
                },
                5 => {
                    if self.dif_lines.5.0 == field {
                        return self.dif_lines.5.1
                    }
                },
                6 => {
                    if self.dif_lines.6.0 == field {
                        return self.dif_lines.6.1
                    }
                },
                7 => {
                    if self.dif_lines.7.0 == field {
                        return self.dif_lines.7.1
                    }
                },
                8 => {
                    if self.dif_lines.8.0 == field {
                        return self.dif_lines.8.1
                    }
                },
                9 => {
                    if self.dif_lines.9.0 == field {
                        return self.dif_lines.9.1
                    }
                },
                10 => {
                    if self.dif_lines.10.0 == field {
                        return self.dif_lines.10.1
                    }
                },
                11 => {
                    if self.dif_lines.11.0 == field {
                        return self.dif_lines.11.1
                    }
                },
                12 => {
                    if self.dif_lines.12.0 == field {
                        return self.dif_lines.12.1
                    }
                },
                13 => {
                    if self.dif_lines.13.0 == field {
                        return self.dif_lines.13.1
                    }
                },
                14 => {
                    if self.dif_lines.14.0 == field {
                        return self.dif_lines.14.1
                    }
                },
                15 => {
                    if self.dif_lines.15.0 == field {
                        return self.dif_lines.15.1
                    }
                },
                16 => {
                    if self.dif_lines.16.0 == field {
                        return self.dif_lines.16.1
                    }
                },
                17 => {
                    if self.dif_lines.17.0 == field {
                        return self.dif_lines.17.1
                    }
                },
                18 => {
                    if self.dif_lines.18.0 == field {
                        return self.dif_lines.18.1
                    }
                },
                19 => {
                    if self.dif_lines.19.0 == field {
                        return self.dif_lines.19.1;
                    }
                },
                20 => {
                    if self.dif_lines.20.0 == field {
                        return self.dif_lines.20.1;
                    }
                }
                _ => break,
            }
        }

        return "";
    }

    /// Gets the value of the ``field`` argument.
    pub fn get(&self, field: DifFieldNames) -> &'static str {
        #[cfg(feature = "alloc")]
        return self.alloc_get(field);

        #[cfg(not(feature = "alloc"))]
        return self.no_alloc_get(field);
    }
}