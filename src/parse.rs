use crate::{to_DifFieldNames, DifFieldNames, DifLine};
use crate::Dif;

impl Dif {
    fn get_dif_line(line: (&'static str, &'static str)) -> DifLine {
        let (field, value) = (to_DifFieldNames(line.0), line.1);
        return (field, value);
    }

    #[cfg(feature = "alloc")]
    fn alloc_parse(file: &[(&'static str, &'static str)]) -> Dif {
        let mut dif = Dif::new();

        for line_num in 0..file.len() {
            let line = Dif::get_dif_line(file[line_num]);

            if line.0 == DifFieldNames::DifName {
                dif.dif_name = Some(line.1)
            }

            dif.dif_lines.push(line);
        }

        return dif;
    }

    #[cfg(not(feature = "alloc"))]
    fn no_alloc_parse(file: &[(&'static str, &'static str)]) -> Dif {
        let mut dif = Dif::new();

        for line_num in 0..file.len() {
            match line_num {
                0 => { dif.dif_lines.0 = Dif::get_dif_line(file[line_num]) },
                1 => { dif.dif_lines.1 = Dif::get_dif_line(file[line_num]) },
                2 => { dif.dif_lines.2 = Dif::get_dif_line(file[line_num]) },
                3 => { dif.dif_lines.3 = Dif::get_dif_line(file[line_num]) },
                4 => { dif.dif_lines.4 = Dif::get_dif_line(file[line_num]) },
                5 => { dif.dif_lines.5 = Dif::get_dif_line(file[line_num]) },
                6 => { dif.dif_lines.6 = Dif::get_dif_line(file[line_num]) },
                7 => { dif.dif_lines.7 = Dif::get_dif_line(file[line_num]) },
                8 => { dif.dif_lines.8 = Dif::get_dif_line(file[line_num]) },
                9 => { dif.dif_lines.9 = Dif::get_dif_line(file[line_num]) },
                10 => { dif.dif_lines.10 = Dif::get_dif_line(file[line_num]) },
                11 => { dif.dif_lines.11 = Dif::get_dif_line(file[line_num]) },
                12 => { dif.dif_lines.12 = Dif::get_dif_line(file[line_num]) },
                13 => { dif.dif_lines.13 = Dif::get_dif_line(file[line_num]) },
                14 => { dif.dif_lines.14 = Dif::get_dif_line(file[line_num]) },
                15 => { dif.dif_lines.15 = Dif::get_dif_line(file[line_num]) },
                16 => { dif.dif_lines.16 = Dif::get_dif_line(file[line_num]) },
                17 => { dif.dif_lines.17 = Dif::get_dif_line(file[line_num]) },
                18 => { dif.dif_lines.18 = Dif::get_dif_line(file[line_num]) },
                _ => break,
            }
        }

        dif.dif_name = Some(dif.get(DifFieldNames::DifName));

        return dif;
    }

    /// Reads static generated from [``difi``](docs.rs/difi/latest/difi) and returns it as a
    /// ``Dif``.
    pub fn parse(&self, file: &[(&'static str, &'static str)]) -> Dif {
        #[cfg(feature = "alloc")]
        return Dif::alloc_parse(file);

        #[cfg(not(feature = "alloc"))]
        return Dif::no_alloc_parse(file);
    }
}
