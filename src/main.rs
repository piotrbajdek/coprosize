// COPROSIZE VERSION 1.0.0 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// MAIN FILE

use coprosize::colors::*;
use std::{env, num::ParseFloatError, str::FromStr};

fn main() {
    let clear = "\x1b[0m"; // reset the font color
    let red = "\x1b[31m";

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

    let args: Vec<String> = env::args().collect();

    let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + clear));
    let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + clear));

    // CARNIVOROUS AMNIOTA

    if input1 == "--carnivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_amniota(diameter);
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
            return;
        }

        coprosize::carnivorous_amniota(diameter);
        return;
    }

    // CARNIVOROUS AMPHIBIA

    if input1 == "--carnivorous" && input2 == "--amphibia" || input1 == "--amphibia" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_amphibia(diameter);
            return;
        }

        coprosize::carnivorous_amphibia(diameter);
        return;
    }

    // CARNIVOROUS ARCHOSAURIA

    if input1 == "--carnivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_archosauria(diameter);
            return;
        }

        coprosize::carnivorous_archosauria(diameter);
        return;
    }

    // CARNIVOROUS FELIDAE

    if input1 == "--carnivorous" && input2 == "--felidae" || input1 == "--felidae" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_felidae(diameter);
            return;
        }

        coprosize::carnivorous_felidae(diameter);
        return;
    }

    // CARNIVOROUS MAMMALIA

    if input1 == "--carnivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_mammalia(diameter);
            println!("");
            coprosize::carnivorous_felidae(diameter);
            return;
        }

        coprosize::carnivorous_mammalia(diameter);
        return;
    }

    // CARNIVOROUS SQUAMATA

    if input1 == "--carnivorous" && input2 == "--squamata" || input1 == "--squamata" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_squamata(diameter);
            return;
        }

        coprosize::carnivorous_squamata(diameter);
        return;
    }

    // CARNIVOROUS TETRAPODA

    if input1 == "--carnivorous" && input2 == "--tetrapoda" || input1 == "--tetrapoda" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::carnivorous_tetrapoda(diameter);
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
            return;
        }

        coprosize::carnivorous_tetrapoda(diameter);
        return;
    }

    // HERBIVOROUS AFROTHERIA

    if input1 == "--herbivorous" && input2 == "--afrotheria" || input1 == "--afrotheria" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_afrotheria(diameter);
            return;
        }

        coprosize::herbivorous_afrotheria(diameter);
        return;
    }

    // HERBIVOROUS ARTIODACTYLA

    if input1 == "--herbivorous" && input2 == "--artiodactyla" || input1 == "--artiodactyla" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_artiodactyla(diameter);
            println!("");
            coprosize::herbivorous_bovidae(diameter);
            println!("");
            coprosize::herbivorous_cervidae(diameter);
            return;
        }

        coprosize::herbivorous_artiodactyla(diameter);
        return;
    }

    // HERBIVOROUS AMNIOTA

    if input1 == "--herbivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_amniota(diameter);
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
            return;
        }

        coprosize::herbivorous_amniota(diameter);
        return;
    }

    // HERBIVOROUS AVES

    if input1 == "--herbivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", colorize(RED, "No subgroups available for this taxon and diet"));
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_aves(diameter);
            return;
        }

        coprosize::herbivorous_aves(diameter);
        return;
    }

    // HERBIVOROUS BOVIDAE

    if input1 == "--herbivorous" && input2 == "--bovidae" || input1 == "--bovidae" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_bovidae(diameter);
            return;
        }

        coprosize::herbivorous_bovidae(diameter);
        return;
    }

    // HERBIVOROUS CERVIDAE

    if input1 == "--herbivorous" && input2 == "--cervidae" || input1 == "--cervidae" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_cervidae(diameter);
            return;
        }

        coprosize::herbivorous_cervidae(diameter);
        return;
    }

    // HERBIVOROUS LAGOMORPHA

    if input1 == "--herbivorous" && input2 == "--lagomorpha" || input1 == "--lagomorpha" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_lagomorpha(diameter);
            return;
        }

        coprosize::herbivorous_lagomorpha(diameter);
        return;
    }

    // HERBIVOROUS MAMMALIA

    if input1 == "--herbivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_mammalia(diameter);
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
            return;
        }

        coprosize::herbivorous_mammalia(diameter);
        return;
    }

    // HERBIVOROUS MARSUPIALIA

    if input1 == "--herbivorous" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_marsupialia(diameter);
            return;
        }

        coprosize::herbivorous_marsupialia(diameter);
        return;
    }

    // HERBIVOROUS PLACENTALIA

    if input1 == "--herbivorous" && input2 == "--placentalia" || input1 == "--placentalia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_placentalia(diameter);
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
            return;
        }

        coprosize::herbivorous_placentalia(diameter);
        return;
    }

    // HERBIVOROUS REPTILIA

    if input1 == "--herbivorous" && input2 == "--reptilia" || input1 == "--reptilia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_reptilia(diameter);
            println!("");
            coprosize::herbivorous_aves(diameter);
            return;
        }

        coprosize::herbivorous_reptilia(diameter);
        return;
    }

    // HERBIVOROUS RODENTIA

    if input1 == "--herbivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::herbivorous_rodentia(diameter);
            return;
        }

        coprosize::herbivorous_rodentia(diameter);
        return;
    }

    // OMNIVOROUS AMNIOTA

    if input1 == "--omnivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::omnivorous_amniota(diameter);
            println!("");
            coprosize::omnivorous_artiodactyla(diameter);
            println!("");
            coprosize::omnivorous_aves(diameter);
            println!("");
            coprosize::omnivorous_mammalia(diameter);
            println!("");
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_amniota(diameter);
        return;
    }

    // OMNIVOROUS ARTIODACTYLA

    if input1 == "--omnivorous" && input2 == "--artiodactyla" || input1 == "--artiodactyla" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::omnivorous_artiodactyla(diameter);
            return;
        }

        coprosize::omnivorous_artiodactyla(diameter);
        return;
    }

    // OMNIVOROUS AVES

    if input1 == "--omnivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::omnivorous_aves(diameter);
            return;
        }

        coprosize::omnivorous_aves(diameter);
        return;
    }

    // OMNIVOROUS MAMMALIA

    if input1 == "--omnivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::omnivorous_mammalia(diameter);
            println!("");
            coprosize::omnivorous_artiodactyla(diameter);
            println!("");
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_mammalia(diameter);
        return;
    }

    // OMNIVOROUS RODENTIA

    if input1 == "--omnivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_rodentia(diameter);
        return;
    }

    // UNSPECIFIED CANIDAE

    if input1 == "--unspecified" && input2 == "--canidae" || input1 == "--canidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_canidae(diameter);
            return;
        }

        coprosize::unspecified_canidae(diameter);
        return;
    }

    // UNSPECIFIED CARNIVORA

    if input1 == "--unspecified" && input2 == "--carnivora" || input1 == "--carnivora" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_carnivora(diameter);
            println!("");
            coprosize::unspecified_canidae(diameter);
            println!("");
            coprosize::unspecified_herpestidae(diameter);
            println!("");
            coprosize::unspecified_mustelidae(diameter);
            return;
        }

        coprosize::unspecified_carnivora(diameter);
        return;
    }

    // UNSPECIFIED CRICETIDAE

    if input1 == "--unspecified" && input2 == "--cricetidae" || input1 == "--cricetidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_cricetidae(diameter);
            return;
        }

        coprosize::unspecified_cricetidae(diameter);
        return;
    }

    // UNSPECIFIED HERPESTIDAE

    if input1 == "--unspecified" && input2 == "--herpestidae" || input1 == "--herpestidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_herpestidae(diameter);
            return;
        }

        coprosize::unspecified_herpestidae(diameter);
        return;
    }

    // UNSPECIFIED MARSUPIALIA

    if input1 == "--unspecified" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_marsupialia(diameter);
            return;
        }

        coprosize::unspecified_marsupialia(diameter);
        return;
    }

    // UNSPECIFIED MUSTELIDAE

    if input1 == "--unspecified" && input2 == "--mustelidae" || input1 == "--mustelidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_mustelidae(diameter);
            return;
        }

        coprosize::unspecified_mustelidae(diameter);
        return;
    }

    // UNSPECIFIED SCIURIDAE

    if input1 == "--unspecified" && input2 == "--sciuridae" || input1 == "--sciuridae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_sciuridae(diameter);
            return;
        }

        coprosize::unspecified_sciuridae(diameter);
        return;
    }

    // UNSPECIFIED TESTUDINES

    if input1 == "--unspecified" && input2 == "--testudines" || input1 == "--testudines" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
            coprosize::unspecified_testudines(diameter);
            return;
        }

        coprosize::unspecified_testudines(diameter);
        return;
    }

    // INVALID ARGUMENT [1] AND/OR [2]

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + clear);
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
        let mut args = env::args();
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
