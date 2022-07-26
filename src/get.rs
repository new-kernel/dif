use crate::{Dif, DifFieldNames};

impl Dif {
    #[cfg(feature = "alloc")]
    fn alloc_get(&self, field: DifFieldNames) -> &'static str {
        for i in 0..self.dif_lines.len() {
            if self.dif_lines[i].0  == field {
                return  self.dif_lines[i].1;
            }
        }

        return "";
    }

    #[cfg(not(feature = "alloc"))]
    fn no_alloc_get(&self, field: DifFieldNames) -> &'static str {
        for i in 0..Dif::MAX_SIZE {
            match line_num {
                0 => { return self.dif_lines.0.1 },
                1 => { return self.dif_lines.1.1  },
                2 => { return self.dif_lines.2.1 },
                3 => { return self.dif_lines.3.1 },
                4 => { return self.dif_lines.4.1 },
                5 => { return self.dif_lines.5.1 },
                6 => { return self.dif_lines.6.1 },
                7 => { return self.dif_lines.7.1 },
                8 => { return self.dif_lines.8.1 },
                9 => { return self.dif_lines.9.1 },
                10 => { return self.dif_lines.10.1 },
                11 => { return self.dif_lines.11.1 },
                12 => { return self.dif_lines.12.1 },
                13 => { return self.dif_lines.13.1 },
                14 => { return self.dif_lines.14.1 },
                15 => { return self.dif_lines.15.1 },
                16 => { return self.dif_lines.16.1 },
                17 => { return self.dif_lines.17.1 },
                18 => { return self.dif_lines.18.1 },
                _ => break,
            }
        }

        return "";
    }

    pub fn get(&self, field: DifFieldNames) -> &'static str {
        #[cfg(feature = "alloc")]
        return self.alloc_get(field);

        #[cfg(not(feature = "alloc"))]
        return self.no_alloc_get(field);
    }
}