// COPROSIZE VERSION 1.0.0 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// MAIN FILE

use coprosize::colors::*;
use std::{env, num::ParseFloatError, str::FromStr};

fn main() {
    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("Program:  coprosize");
            println!("Version:  1.0.0");
            println!("Date:     July 2, 2022");
            println!("Author:   Piotr Bajdek");
            println!("Contact:  {}", colorize(BLUE_UNDERLINED, "piotr.bajdek@proton.me"));
            println!("ORCID:    {}", colorize(BLUE_UNDERLINED, "https://orcid.org/0000-0003-2678-3122"));
            println!("Source:   {}", colorize(BLUE_UNDERLINED, "https://github.com/piotrbajdek/coprosize"));
            println!("License:  The MIT License (MIT) © 2022 Piotr Bajdek");
            return;
        }

        // CHANGES

        if argument == "-c" || argument == "--changes" {
            let all_implemented = [
                "--carnivorous --amniota",
                "--carnivorous --amphibia",
                "--carnivorous --archosauria",
                "--carnivorous --felidae",
                "--carnivorous --mammalia",
                "--carnivorous --squamata",
                "--carnivorous --tetrapoda",
                "--herbivorous --afrotheria",
                "--herbivorous --amniota",
                "--herbivorous --artiodactyla",
                "--herbivorous --aves",
                "--herbivorous --bovidae",
                "--herbivorous --cervidae",
                "--herbivorous --lagomorpha",
                "--herbivorous --mammalia",
                "--herbivorous --marsupialia",
                "--herbivorous --placentalia",
                "--herbivorous --reptilia",
                "--herbivorous --rodentia",
                "--omnivorous --amniota",
                "--omnivorous --artiodactyla",
                "--omnivorous --aves",
                "--omnivorous --mammalia",
                "--omnivorous --rodentia",
                "--unspecified --canidae",
                "--unspecified --carnivora",
                "--unspecified --cricetidae",
                "--unspecified --herpestidae",
                "--unspecified --marsupialia",
                "--unspecified --mustelidae",
                "--unspecified --sciuridae",
                "--unspecified --testudines",
            ];
            print!("02.07.2022 v1.0.0 – Implemented");
            for (index, implemented) in all_implemented.into_iter().enumerate() {
                let between_delimiter = if index + 1 == all_implemented.len() { "" } else { ", " };
                print!("{}{}", colorize(CYAN, implemented), between_delimiter);
            }
            println!();

            return;
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            coprosize::citation();
            return;
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            let mammals = [
                ("--carnivorous --felidae", "[m]", "Model for carnivorous felids"),
                ("--carnivorous --mammalia", "[l]", "Model for carnivorous mammalians"),
                ("--herbivorous --afrotheria", "[l]", "Model for herbivor. afrotherians"),
                ("--herbivorous --artiodactyla", "[m]", "Model for herbivor. artiodactyls"),
                ("--herbivorous --bovidae", "[m]", "Model for herbivorous bovids"),
                ("--herbivorous --cervidae", "[h]", "Model for herbivorous cervids"),
                ("--herbivorous --lagomorpha", "[h]", "Model for herbivorous lagomorphs"),
                ("--herbivorous --mammalia", "[l]", "Model for herbivorous mammalians"),
                ("--herbivorous --marsupialia", "[m]", "Model for herbivorous marsupials"),
                ("--herbivorous --placentalia", "[l]", "Model for herbivorous placentals"),
                ("--herbivorous --rodentia", "[m]", "Model for herbivorous rodents"),
                ("--omnivorous --artiodactyla", "[h]", "Model for omnivor. artiodactyls"),
                ("--omnivorous --mammalia", "[m]", "Model for omnivorous mammalians"),
                ("--omnivorous --rodentia", "[m]", "Model for omnivorous rodents"),
                ("--unspecified --canidae", "[m]", "Model for canids"),
                ("--unspecified --carnivora", "[l]", "Model for carnivorans"),
                ("--unspecified --cricetidae", "[m]", "Model for cricetids"),
                ("--unspecified --herpestidae", "[m]", "Model for herpestids"),
                ("--unspecified --marsupialia", "[m]", "Model for marsupials"),
                ("--unspecified --mustelidae", "[m]", "Model for mustelids"),
                ("--unspecified --sciuridae", "[h]", "Model for sciurids"),
            ];

            let reptiles = [
                ("--carnivorous --archosauria", "[m]", "Model for carnivorous archosaurs"),
                ("--carnivorous --squamata", "[m]", "Model for carnivorous squamates"),
                ("--herbivorous --aves", "[h]", "Model for herbivorous birds"),
                ("--herbivorous --reptilia", "[h]", "Model for herbivorous reptiles"),
                ("--omnivorous --aves", "[m]", "Model for omnivorous birds"),
                ("--unspecified --testudines", "[l]", "Model for turtles"),
            ];

            let others = [
                ("--carnivorous --amniota", "[l]", "Model for carnivorous amniotes"),
                ("--carnivorous --amphibia", "[h]", "Model for carnivorous amphibians"),
                ("--carnivorous --tetrapoda", "[l]", "Model for carnivorous tetrapods"),
                ("--herbivorous --amniota", "[l]", "Model for herbivorous amniotes"),
                ("--omnivorous --amniota", "[l]", "Model for omnivorous amniotes"),
            ];

            println!("Usage:    {}", colorize(BRIGHT_YELLOW, "coprosize [DIET / TAXON] [TAXON / DIET] [COPROLITE DIAMETER IN MM]"));
            println!("          {}", colorize(BRIGHT_YELLOW, "coprosize [DIET / TAXON] [TAXON / DIET] [OPTIONS] [DIAMETER IN MM]"));
            println!("");

            fn print_model_help(heading_title: &str, entries: &[(&str, &str, &str)]) {
                let mut first_loop = true;
                for (options, reliability, description) in entries {
                    if first_loop {
                        println!("{:>8}: {:<29} {} {}", heading_title, colorize(CYAN, options), colorize(RED, reliability), description);
                        first_loop = false;
                    } else {
                        let indent = "          ";
                        println!("{indent}{:<29} {} {}", colorize(CYAN, options), colorize(RED, reliability), description);
                    }
                }
            }
            print_model_help("Mamm.", &mammals);
            println!();
            print_model_help("Rept.", &reptiles);
            println!();
            print_model_help("Other", &others);
            println!();

            let indent10 = "          ";
            println!(
                "{indent10}Model reliability: {} - low, {} - moderate, {} - seemingly high",
                colorize(RED, "[l]"),
                colorize(RED, "[m]"),
                colorize(RED, "[h]")
            );
            println!("");
            println!("Examples: {} [diameter given in mm]", colorize(BRIGHT_YELLOW, "coprosize --carnivorous --archosauria 27.75"));
            println!("{indent10}{} [diameter given in mm]", colorize(BRIGHT_YELLOW, "coprosize --archosauria --carnivorous 27.75"));
            println!("");
            println!(
                "{indent10}{}, {} Show all available subgroups within a higher taxon",
                colorize(CYAN, "-s"),
                colorize(CYAN, "--subgroups")
            );
            println!("");
            println!("Examples: {}", colorize(BRIGHT_YELLOW, "coprosize --herbivorous --reptilia --subgroups 115.25"));
            println!("{indent10}{}", colorize(BRIGHT_YELLOW, "coprosize --unspecified --carnivora -s 14.5"));
            println!("");
            println!("See also: {}, {:<11} Show contact and program info", colorize(CYAN, "-a"), colorize(CYAN, "--about"));
            println!("          {}, {:<11} Show simplified change notes", colorize(CYAN, "-c"), colorize(CYAN, "--changes"));
            println!("          {}, {:<11} Show how to cite this program", colorize(CYAN, "-C"), colorize(CYAN, "--citation"));
            println!("          {}, {:<11} Show this help", colorize(CYAN, "-h"), colorize(CYAN, "--help"));
            println!("          {}, {:<11} Show licesing information", colorize(CYAN, "-l"), colorize(CYAN, "--license"));
            println!("          {}, {:<11} Show the program version", colorize(CYAN, "-v"), colorize(CYAN, "--version"));
            return;
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", colorize(BRIGHT_YELLOW, "The MIT License (MIT)"));
            println!("");
            println!("Copyright © 2022 Piotr Bajdek");
            println!("");
            println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
            println!("");
            println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
            println!("");
            println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
            return;
        }

        // VERSION

        if argument == "-v" || argument == "--version" {
            println!("Version: 1.0.0");
            println!("July 2, 2022");
            return;
        }
    }

    // COLLECT ARGUMENTS FOR CALCULATIONS

    let args = Args::from_args().unwrap();

    use Diet::*;
    use Taxon::*;
    let diameter = args.diameter;
    match (args.diet, args.taxon) {
        // CARNIVOROUS AMNIOTA
        (Carnivorous, Amniota) => {
            coprosize::carnivorous_amniota(diameter);
            if args.subgroups {
                println!("");
                coprosize::carnivorous_archosauria(diameter);
                println!("");
                coprosize::carnivorous_felidae(diameter);
                println!("");
                coprosize::carnivorous_mammalia(diameter);
                println!("");
                coprosize::carnivorous_squamata(diameter);
                println!("");
                coprosize::carnivorous_tetrapoda(diameter);
            }
        }

        // CARNIVOROUS AMPHIBIA
        (Carnivorous, Amphibia) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::carnivorous_amphibia(diameter);
        }

        // CARNIVOROUS ARCHOSAURIA
        (Carnivorous, Archosauria) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::carnivorous_archosauria(diameter);
        }

        // CARNIVOROUS FELIDAE
        (Carnivorous, Felidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::carnivorous_felidae(diameter);
        }

        // CARNIVOROUS MAMMALIA
        (Carnivorous, Mammalia) => {
            coprosize::carnivorous_mammalia(diameter);
            if args.subgroups {
                println!("");
                coprosize::carnivorous_felidae(diameter);
            }
        }

        // CARNIVOROUS SQUAMATA
        (Carnivorous, Squamata) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::carnivorous_squamata(diameter);
        }

        // CARNIVOROUS TETRAPODA
        (Carnivorous, Tetrapoda) => {
            coprosize::carnivorous_tetrapoda(diameter);
            if args.subgroups {
                println!("");
                coprosize::carnivorous_amniota(diameter);
                println!("");
                coprosize::carnivorous_amphibia(diameter);
                println!("");
                coprosize::carnivorous_archosauria(diameter);
                println!("");
                coprosize::carnivorous_felidae(diameter);
                println!("");
                coprosize::carnivorous_mammalia(diameter);
                println!("");
                coprosize::carnivorous_squamata(diameter);
            }
        }

        // HERBIVOROUS AFROTHERIA
        (Herbivorous, Afrotheria) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_afrotheria(diameter);
        }

        // HERBIVOROUS ARTIODACTYLA
        (Herbivorous, Artiodactyla) => {
            coprosize::herbivorous_artiodactyla(diameter);
            if args.subgroups {
                println!("");
                coprosize::herbivorous_bovidae(diameter);
                println!("");
                coprosize::herbivorous_cervidae(diameter);
            }
        }

        // HERBIVOROUS AMNIOTA
        (Herbivorous, Amniota) => {
            coprosize::herbivorous_amniota(diameter);
            if args.subgroups {
                println!("");
                coprosize::herbivorous_afrotheria(diameter);
                println!("");
                coprosize::herbivorous_artiodactyla(diameter);
                println!("");
                coprosize::herbivorous_aves(diameter);
                println!("");
                coprosize::herbivorous_bovidae(diameter);
                println!("");
                coprosize::herbivorous_cervidae(diameter);
                println!("");
                coprosize::herbivorous_lagomorpha(diameter);
                println!("");
                coprosize::herbivorous_mammalia(diameter);
                println!("");
                coprosize::herbivorous_marsupialia(diameter);
                println!("");
                coprosize::herbivorous_placentalia(diameter);
                println!("");
                coprosize::herbivorous_reptilia(diameter);
                println!("");
                coprosize::herbivorous_rodentia(diameter);
            }
        }

        // HERBIVOROUS AVES
        (Herbivorous, Aves) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_aves(diameter);
        }

        // HERBIVOROUS BOVIDAE
        (Herbivorous, Bovidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_bovidae(diameter);
        }

        // HERBIVOROUS CERVIDAE
        (Herbivorous, Cervidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_cervidae(diameter);
        }

        // HERBIVOROUS LAGOMORPHA
        (Herbivorous, Lagomorpha) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_lagomorpha(diameter);
        }

        // HERBIVOROUS MAMMALIA
        (Herbivorous, Mammalia) => {
            coprosize::herbivorous_mammalia(diameter);
            if args.subgroups {
                println!("");
                coprosize::herbivorous_afrotheria(diameter);
                println!("");
                coprosize::herbivorous_artiodactyla(diameter);
                println!("");
                coprosize::herbivorous_bovidae(diameter);
                println!("");
                coprosize::herbivorous_cervidae(diameter);
                println!("");
                coprosize::herbivorous_lagomorpha(diameter);
                println!("");
                coprosize::herbivorous_marsupialia(diameter);
                println!("");
                coprosize::herbivorous_placentalia(diameter);
                println!("");
                coprosize::herbivorous_rodentia(diameter);
            }
        }

        // HERBIVOROUS MARSUPIALIA
        (Herbivorous, Marsupialia) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_marsupialia(diameter);
        }

        // HERBIVOROUS PLACENTALIA
        (Herbivorous, Placentalia) => {
            coprosize::herbivorous_placentalia(diameter);
            if args.subgroups {
                println!("");
                coprosize::herbivorous_afrotheria(diameter);
                println!("");
                coprosize::herbivorous_artiodactyla(diameter);
                println!("");
                coprosize::herbivorous_bovidae(diameter);
                println!("");
                coprosize::herbivorous_cervidae(diameter);
                println!("");
                coprosize::herbivorous_lagomorpha(diameter);
                println!("");
                coprosize::herbivorous_rodentia(diameter);
            }
        }

        // HERBIVOROUS REPTILIA
        (Herbivorous, Reptilia) => {
            coprosize::herbivorous_reptilia(diameter);
            if args.subgroups {
                println!();
                coprosize::herbivorous_aves(diameter);
            }
        }

        // HERBIVOROUS RODENTIA
        (Herbivorous, Rodentia) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::herbivorous_rodentia(diameter);
        }

        // OMNIVOROUS AMNIOTA
        (Omnivorous, Amniota) => {
            coprosize::omnivorous_amniota(diameter);
            if args.subgroups {
                println!("");
                coprosize::omnivorous_artiodactyla(diameter);
                println!("");
                coprosize::omnivorous_aves(diameter);
                println!("");
                coprosize::omnivorous_mammalia(diameter);
                println!("");
                coprosize::omnivorous_rodentia(diameter);
            }
        }

        // OMNIVOROUS ARTIODACTYLA
        (Omnivorous, Artiodactyla) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::omnivorous_artiodactyla(diameter);
        }

        // OMNIVOROUS AVES
        (Omnivorous, Aves) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::omnivorous_aves(diameter);
        }

        // OMNIVOROUS MAMMALIA
        (Omnivorous, Mammalia) => {
            coprosize::omnivorous_mammalia(diameter);
            if args.subgroups {
                println!("");
                coprosize::omnivorous_artiodactyla(diameter);
                println!("");
                coprosize::omnivorous_rodentia(diameter);
            }
        }

        // OMNIVOROUS RODENTIA
        (Omnivorous, Rodentia) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::omnivorous_rodentia(diameter);
        }

        // UNSPECIFIED CANIDAE
        (Unspecified, Canidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_canidae(diameter);
        }

        // UNSPECIFIED CARNIVORA
        (Unspecified, Carnivora) => {
            coprosize::unspecified_carnivora(diameter);
            if args.subgroups {
                println!("");
                coprosize::unspecified_canidae(diameter);
                println!("");
                coprosize::unspecified_herpestidae(diameter);
                println!("");
                coprosize::unspecified_mustelidae(diameter);
            }
        }

        // UNSPECIFIED CRICETIDAE
        (Unspecified, Cricetidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_cricetidae(diameter);
        }

        // UNSPECIFIED HERPESTIDAE
        (Unspecified, Herpestidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_herpestidae(diameter);
        }

        // UNSPECIFIED MARSUPIALIA
        (Unspecified, Marsupialia) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_marsupialia(diameter);
        }

        // UNSPECIFIED MUSTELIDAE
        (Unspecified, Mustelidae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_mustelidae(diameter);
        }

        // UNSPECIFIED SCIURIDAE
        (Unspecified, Sciuridae) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_sciuridae(diameter);
        }

        // UNSPECIFIED TESTUDINES
        (Unspecified, Testudines) => {
            if args.subgroups {
                println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            }
            coprosize::unspecified_testudines(diameter);
        }

        _ => panic!("{}", colorize(RED, "Invalid arguments provided! See: --help")),
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    NotEnoughArguments,
    UnknownDietOrTaxon,
    MissingDiameter,
    FailedParsingDiameter(ParseFloatError),
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::FailedParsingDiameter(e)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Diet {
    Carnivorous,
    Herbivorous,
    Omnivorous,
    Unspecified,
}

impl FromStr for Diet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "carnivorous" => Self::Carnivorous,
            "herbivorous" => Self::Herbivorous,
            "omnivorous" => Self::Omnivorous,
            "unspecified" => Self::Unspecified,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Taxon {
    Afrotheria,
    Amniota,
    Amphibia,
    Archosauria,
    Artiodactyla,
    Aves,
    Bovidae,
    Canidae,
    Carnivora,
    Cervidae,
    Cricetidae,
    Felidae,
    Herpestidae,
    Lagomorpha,
    Mammalia,
    Marsupialia,
    Mustelidae,
    Placentalia,
    Reptilia,
    Rodentia,
    Sciuridae,
    Squamata,
    Testudines,
    Tetrapoda,
}

impl FromStr for Taxon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "afrotheria" => Self::Afrotheria,
            "amniota" => Self::Amniota,
            "amphibia" => Self::Amphibia,
            "archosauria" => Self::Archosauria,
            "artiodactyla" => Self::Artiodactyla,
            "aves" => Self::Aves,
            "bovidae" => Self::Bovidae,
            "canidae" => Self::Canidae,
            "carnivora" => Self::Carnivora,
            "cervidae" => Self::Cervidae,
            "cricetidae" => Self::Cricetidae,
            "felidae" => Self::Felidae,
            "herpestidae" => Self::Herpestidae,
            "lagomorpha" => Self::Lagomorpha,
            "mammalia" => Self::Mammalia,
            "marsupialia" => Self::Marsupialia,
            "mustelidae" => Self::Mustelidae,
            "placentalia" => Self::Placentalia,
            "reptilia" => Self::Reptilia,
            "rodentia" => Self::Rodentia,
            "sciuridae" => Self::Sciuridae,
            "squamata" => Self::Squamata,
            "testudines" => Self::Testudines,
            "tetrapoda" => Self::Tetrapoda,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    diet: Diet,
    taxon: Taxon,
    diameter: f32,
    subgroups: bool,
}

impl Args {
    pub fn from_args() -> Result<Self, Error> {
        let mut args = env::args().skip(1);
        let (diet, taxon) = {
            // not enough arguments
            let a = args.next().ok_or(Error::NotEnoughArguments)?;
            let b = args.next().ok_or(Error::NotEnoughArguments)?;
            let (a, b) = (a.strip_prefix("--").unwrap_or(&a), b.strip_prefix("--").unwrap_or(&b));
            if let (Ok(diet), Ok(taxon)) = (a.parse::<Diet>(), b.parse::<Taxon>()) {
                (diet, taxon)
            } else if let (Ok(taxon), Ok(diet)) = (a.parse::<Taxon>(), b.parse::<Diet>()) {
                (diet, taxon)
            } else {
                // unrecognized diet or taxon
                return Err(Error::UnknownDietOrTaxon);
            }
        };

        let (diameter, subgroups) = match (args.next(), args.next()) {
            (Some(maybe_extra_flag), Some(maybe_diameter)) => {
                if maybe_extra_flag == "-s" || maybe_extra_flag == "--subgroups" {
                    (maybe_diameter.parse::<f32>()?, true)
                } else {
                    // treat extra flag like the diameter
                    (maybe_extra_flag.parse::<f32>()?, false)
                }
            }
            (Some(diameter), None) => {
                // couldn't parse diameter
                let diameter = diameter.parse::<f32>()?;
                (diameter, false)
            }
            _ => {
                // not enough args, need diameter
                return Err(Error::MissingDiameter);
            }
        };

        Ok(Self { diet, taxon, diameter, subgroups })
    }
}
