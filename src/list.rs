// COPROSIZE VERSION 1.0.3 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE LIST

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cognitive_complexity, clippy::missing_panics_doc, clippy::too_many_lines)]

// IMPORTS

use std::env;

// LIST MODELS

pub fn models() {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";

    // COLLECT ARGUMENTS FOR CALCULATIONS

    let args: Vec<String> = env::args().collect();

    let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));
    let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + reset));

    // CARNIVOROUS AMNIOTA

    if input1 == "--carnivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_amniota(diameter);
            println!();
            coprosize::carnivorous_archosauria(diameter);
            println!();
            coprosize::carnivorous_felidae(diameter);
            println!();
            coprosize::carnivorous_mammalia(diameter);
            println!();
            coprosize::carnivorous_squamata(diameter);
            println!();
            coprosize::carnivorous_tetrapoda(diameter);
            return;
        }

        coprosize::carnivorous_amniota(diameter);
        return;
    }

    // CARNIVOROUS AMPHIBIA

    if input1 == "--carnivorous" && input2 == "--amphibia" || input1 == "--amphibia" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_amphibia(diameter);
            return;
        }

        coprosize::carnivorous_amphibia(diameter);
        return;
    }

    // CARNIVOROUS ARCHOSAURIA

    if input1 == "--carnivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_archosauria(diameter);
            return;
        }

        coprosize::carnivorous_archosauria(diameter);
        return;
    }

    // CARNIVOROUS FELIDAE

    if input1 == "--carnivorous" && input2 == "--felidae" || input1 == "--felidae" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_felidae(diameter);
            return;
        }

        coprosize::carnivorous_felidae(diameter);
        return;
    }

    // CARNIVOROUS MAMMALIA

    if input1 == "--carnivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_mammalia(diameter);
            println!();
            coprosize::carnivorous_felidae(diameter);
            return;
        }

        coprosize::carnivorous_mammalia(diameter);
        return;
    }

    // CARNIVOROUS SQUAMATA

    if input1 == "--carnivorous" && input2 == "--squamata" || input1 == "--squamata" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_squamata(diameter);
            return;
        }

        coprosize::carnivorous_squamata(diameter);
        return;
    }

    // CARNIVOROUS TETRAPODA

    if input1 == "--carnivorous" && input2 == "--tetrapoda" || input1 == "--tetrapoda" && input2 == "--carnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::carnivorous_tetrapoda(diameter);
            println!();
            coprosize::carnivorous_amniota(diameter);
            println!();
            coprosize::carnivorous_amphibia(diameter);
            println!();
            coprosize::carnivorous_archosauria(diameter);
            println!();
            coprosize::carnivorous_felidae(diameter);
            println!();
            coprosize::carnivorous_mammalia(diameter);
            println!();
            coprosize::carnivorous_squamata(diameter);
            return;
        }

        coprosize::carnivorous_tetrapoda(diameter);
        return;
    }

    // HERBIVOROUS AFROTHERIA

    if input1 == "--herbivorous" && input2 == "--afrotheria" || input1 == "--afrotheria" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_afrotheria(diameter);
            return;
        }

        coprosize::herbivorous_afrotheria(diameter);
        return;
    }

    // HERBIVOROUS ARTIODACTYLA

    if input1 == "--herbivorous" && input2 == "--artiodactyla" || input1 == "--artiodactyla" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_artiodactyla(diameter);
            println!();
            coprosize::herbivorous_bovidae(diameter);
            println!();
            coprosize::herbivorous_cervidae(diameter);
            return;
        }

        coprosize::herbivorous_artiodactyla(diameter);
        return;
    }

    // HERBIVOROUS AMNIOTA

    if input1 == "--herbivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_amniota(diameter);
            println!();
            coprosize::herbivorous_afrotheria(diameter);
            println!();
            coprosize::herbivorous_artiodactyla(diameter);
            println!();
            coprosize::herbivorous_aves(diameter);
            println!();
            coprosize::herbivorous_bovidae(diameter);
            println!();
            coprosize::herbivorous_cervidae(diameter);
            println!();
            coprosize::herbivorous_lagomorpha(diameter);
            println!();
            coprosize::herbivorous_mammalia(diameter);
            println!();
            coprosize::herbivorous_marsupialia(diameter);
            println!();
            coprosize::herbivorous_placentalia(diameter);
            println!();
            coprosize::herbivorous_reptilia(diameter);
            println!();
            coprosize::herbivorous_rodentia(diameter);
            return;
        }

        coprosize::herbivorous_amniota(diameter);
        return;
    }

    // HERBIVOROUS AVES

    if input1 == "--herbivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_aves(diameter);
            return;
        }

        coprosize::herbivorous_aves(diameter);
        return;
    }

    // HERBIVOROUS BOVIDAE

    if input1 == "--herbivorous" && input2 == "--bovidae" || input1 == "--bovidae" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_bovidae(diameter);
            return;
        }

        coprosize::herbivorous_bovidae(diameter);
        return;
    }

    // HERBIVOROUS CERVIDAE

    if input1 == "--herbivorous" && input2 == "--cervidae" || input1 == "--cervidae" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_cervidae(diameter);
            return;
        }

        coprosize::herbivorous_cervidae(diameter);
        return;
    }

    // HERBIVOROUS LAGOMORPHA

    if input1 == "--herbivorous" && input2 == "--lagomorpha" || input1 == "--lagomorpha" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_lagomorpha(diameter);
            return;
        }

        coprosize::herbivorous_lagomorpha(diameter);
        return;
    }

    // HERBIVOROUS MAMMALIA

    if input1 == "--herbivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_mammalia(diameter);
            println!();
            coprosize::herbivorous_afrotheria(diameter);
            println!();
            coprosize::herbivorous_artiodactyla(diameter);
            println!();
            coprosize::herbivorous_bovidae(diameter);
            println!();
            coprosize::herbivorous_cervidae(diameter);
            println!();
            coprosize::herbivorous_lagomorpha(diameter);
            println!();
            coprosize::herbivorous_marsupialia(diameter);
            println!();
            coprosize::herbivorous_placentalia(diameter);
            println!();
            coprosize::herbivorous_rodentia(diameter);
            return;
        }

        coprosize::herbivorous_mammalia(diameter);
        return;
    }

    // HERBIVOROUS MARSUPIALIA

    if input1 == "--herbivorous" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_marsupialia(diameter);
            return;
        }

        coprosize::herbivorous_marsupialia(diameter);
        return;
    }

    // HERBIVOROUS PLACENTALIA

    if input1 == "--herbivorous" && input2 == "--placentalia" || input1 == "--placentalia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_placentalia(diameter);
            println!();
            coprosize::herbivorous_afrotheria(diameter);
            println!();
            coprosize::herbivorous_artiodactyla(diameter);
            println!();
            coprosize::herbivorous_bovidae(diameter);
            println!();
            coprosize::herbivorous_cervidae(diameter);
            println!();
            coprosize::herbivorous_lagomorpha(diameter);
            println!();
            coprosize::herbivorous_rodentia(diameter);
            return;
        }

        coprosize::herbivorous_placentalia(diameter);
        return;
    }

    // HERBIVOROUS REPTILIA

    if input1 == "--herbivorous" && input2 == "--reptilia" || input1 == "--reptilia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_reptilia(diameter);
            println!();
            coprosize::herbivorous_aves(diameter);
            return;
        }

        coprosize::herbivorous_reptilia(diameter);
        return;
    }

    // HERBIVOROUS RODENTIA

    if input1 == "--herbivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--herbivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::herbivorous_rodentia(diameter);
            return;
        }

        coprosize::herbivorous_rodentia(diameter);
        return;
    }

    // OMNIVOROUS AMNIOTA

    if input1 == "--omnivorous" && input2 == "--amniota" || input1 == "--amniota" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::omnivorous_amniota(diameter);
            println!();
            coprosize::omnivorous_artiodactyla(diameter);
            println!();
            coprosize::omnivorous_aves(diameter);
            println!();
            coprosize::omnivorous_mammalia(diameter);
            println!();
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_amniota(diameter);
        return;
    }

    // OMNIVOROUS ARTIODACTYLA

    if input1 == "--omnivorous" && input2 == "--artiodactyla" || input1 == "--artiodactyla" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::omnivorous_artiodactyla(diameter);
            return;
        }

        coprosize::omnivorous_artiodactyla(diameter);
        return;
    }

    // OMNIVOROUS AVES

    if input1 == "--omnivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::omnivorous_aves(diameter);
            return;
        }

        coprosize::omnivorous_aves(diameter);
        return;
    }

    // OMNIVOROUS MAMMALIA

    if input1 == "--omnivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::omnivorous_mammalia(diameter);
            println!();
            coprosize::omnivorous_artiodactyla(diameter);
            println!();
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_mammalia(diameter);
        return;
    }

    // OMNIVOROUS RODENTIA

    if input1 == "--omnivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--omnivorous" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::omnivorous_rodentia(diameter);
            return;
        }

        coprosize::omnivorous_rodentia(diameter);
        return;
    }

    // UNSPECIFIED CANIDAE

    if input1 == "--unspecified" && input2 == "--canidae" || input1 == "--canidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_canidae(diameter);
            return;
        }

        coprosize::unspecified_canidae(diameter);
        return;
    }

    // UNSPECIFIED CARNIVORA

    if input1 == "--unspecified" && input2 == "--carnivora" || input1 == "--carnivora" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_carnivora(diameter);
            println!();
            coprosize::unspecified_canidae(diameter);
            println!();
            coprosize::unspecified_herpestidae(diameter);
            println!();
            coprosize::unspecified_mustelidae(diameter);
            return;
        }

        coprosize::unspecified_carnivora(diameter);
        return;
    }

    // UNSPECIFIED CRICETIDAE

    if input1 == "--unspecified" && input2 == "--cricetidae" || input1 == "--cricetidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_cricetidae(diameter);
            return;
        }

        coprosize::unspecified_cricetidae(diameter);
        return;
    }

    // UNSPECIFIED HERPESTIDAE

    if input1 == "--unspecified" && input2 == "--herpestidae" || input1 == "--herpestidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_herpestidae(diameter);
            return;
        }

        coprosize::unspecified_herpestidae(diameter);
        return;
    }

    // UNSPECIFIED MARSUPIALIA

    if input1 == "--unspecified" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_marsupialia(diameter);
            return;
        }

        coprosize::unspecified_marsupialia(diameter);
        return;
    }

    // UNSPECIFIED MUSTELIDAE

    if input1 == "--unspecified" && input2 == "--mustelidae" || input1 == "--mustelidae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_mustelidae(diameter);
            return;
        }

        coprosize::unspecified_mustelidae(diameter);
        return;
    }

    // UNSPECIFIED SCIURIDAE

    if input1 == "--unspecified" && input2 == "--sciuridae" || input1 == "--sciuridae" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_sciuridae(diameter);
            return;
        }

        coprosize::unspecified_sciuridae(diameter);
        return;
    }

    // UNSPECIFIED TESTUDINES

    if input1 == "--unspecified" && input2 == "--testudines" || input1 == "--testudines" && input2 == "--unspecified" {
        let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));

        if diameter == "-s" || diameter == "--subgroups" {
            println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + reset);
            let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + reset));
            coprosize::unspecified_testudines(diameter);
            return;
        }

        coprosize::unspecified_testudines(diameter);
        return;
    }

    // INVALID ARGUMENTS

    panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + reset);
}
