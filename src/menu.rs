// COPROSIZE VERSION 1.0.4 / MIT LICENSE © 2022–2023 PIOTR BAJDEK

// MODULE MENU

// CLIPPY LINTS

#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::too_many_lines)]

// IMPORTS

use std::env;
use std::process::exit;

// DOCUMENTATION

pub fn documentation() {
    let reset = "\x1b[0m";
    let red = "\x1b[31m";
    let blue_underlined = "\x1b[34;4m";
    let grey = "\x1b[38;5;240m";
    let violet = "\x1b[38;5;133m";
    let yellow = "\x1b[38;5;220m";

    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("{}", grey.to_owned() + "Program" + reset + ":  " + yellow + "coprosize" + reset);
            println!("{}", grey.to_owned() + "Version" + reset + ":  1.0.4");
            println!("{}", grey.to_owned() + "Date" + reset + ":     January 31, 2023");
            println!("{}", grey.to_owned() + "Author" + reset + ":   Piotr Bajdek");
            println!("{}", grey.to_owned() + "Contact" + reset + ":  " + blue_underlined + "piotr.bajdek@proton.me" + reset);
            println!("{}", grey.to_owned() + "ORCID" + reset + ":    " + blue_underlined + "https://orcid.org/0000-0003-2678-3122" + reset);
            println!("{}", grey.to_owned() + "Source" + reset + ":   " + blue_underlined + "https://github.com/piotrbajdek/coprosize" + reset);
            println!("{}", grey.to_owned() + "License" + reset + ":  MIT License © 2022–2023 Piotr Bajdek");
            exit(0);
        }

        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("{}", yellow.to_owned() + "31.01.2023 v1.0.4 – Enhanced documentation and layout" + reset);
            println!("19.01.2023 {}", yellow.to_owned() + "v1.0.3" + reset + " – Enhanced source code and layout");
            println!("30.09.2022 {}", yellow.to_owned() + "v1.0.2" + reset + " – Enhanced source code");
            println!("16.08.2022 {}", yellow.to_owned() + "v1.0.1" + reset + " – Enhanced source code and layout");
            println!("02.07.2022 {}", yellow.to_owned() + "v1.0.0" + reset + " – Implemented " + violet + "--carnivorous --amniota" + reset + ", " + violet + "--carnivorous --amphibia" + reset + ", " + violet + "--carnivorous --archosauria" + reset + ", " + violet + "--carnivorous --felidae" + reset + ", " + violet + "--carnivorous --mammalia" + reset + ", " + violet + "--carnivorous --squamata" + reset + ", " + violet + "--carnivorous --tetrapoda" + reset + ", " + violet + "--herbivorous --afrotheria" + reset + ", " + violet + "--herbivorous --amniota" + reset + ", " + violet + "--herbivorous --artiodactyla" + reset + ", " + violet + "--herbivorous --aves" + reset + ", " + violet + "--herbivorous --bovidae" + reset + ", " + violet + "--herbivorous --cervidae" + reset + ", " + violet + "--herbivorous --lagomorpha" + reset + ", " + violet + "--herbivorous --mammalia" + reset + ", " + violet + "--herbivorous --marsupialia" + reset + ", " + violet + "--herbivorous --placentalia" + reset + ", " + violet + "--herbivorous --reptilia" + reset + ", " + violet + "--herbivorous --rodentia" + reset + ", " + violet + "--omnivorous --amniota" + reset + ", " + violet + "--omnivorous --artiodactyla" + reset + ", " + violet + "--omnivorous --aves" + reset + ", " + violet + "--omnivorous --mammalia" + reset + ", " + violet + "--omnivorous --rodentia" + reset + ", " + violet + "--unspecified --canidae" + reset + ", " + violet + "--unspecified --carnivora" + reset + ", " + violet + "--unspecified --cricetidae" + reset + ", " + violet + "--unspecified --herpestidae" + reset + ", " + violet + "--unspecified --marsupialia" + reset + ", " + violet + "--unspecified --mustelidae" + reset + ", " + violet + "--unspecified --sciuridae" + reset + ", " + violet + "--unspecified --testudines" + reset);
            exit(0);
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            coprosize::citation();
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("{}", red.to_owned() + "Usage" + reset + ":" + yellow + "    coprosize [diet / taxon] [taxon / diet] [coprolite diameter in mm]");
            println!("      {}", yellow.to_owned() + "    coprosize [diet / taxon] [taxon / diet] [options] [diameter in mm]");
            println!("{reset}");
            println!("   Mamm.: {}", violet.to_owned() + "--carnivorous --felidae       " + red + "[m]" + reset + " Model for carnivorous felids");
            println!("          {}", violet.to_owned() + "--carnivorous --mammalia      " + red + "[l]" + reset + " Model for carnivorous mammalians");
            println!("          {}", violet.to_owned() + "--herbivorous --afrotheria    " + red + "[l]" + reset + " Model for herbivor. afrotherians");
            println!("          {}", violet.to_owned() + "--herbivorous --artiodactyla  " + red + "[m]" + reset + " Model for herbivor. artiodactyls");
            println!("          {}", violet.to_owned() + "--herbivorous --bovidae       " + red + "[m]" + reset + " Model for herbivorous bovids");
            println!("          {}", violet.to_owned() + "--herbivorous --cervidae      " + red + "[h]" + reset + " Model for herbivorous cervids");
            println!("          {}", violet.to_owned() + "--herbivorous --lagomorpha    " + red + "[h]" + reset + " Model for herbivorous lagomorphs");
            println!("          {}", violet.to_owned() + "--herbivorous --mammalia      " + red + "[l]" + reset + " Model for herbivorous mammalians");
            println!("          {}", violet.to_owned() + "--herbivorous --marsupialia   " + red + "[m]" + reset + " Model for herbivorous marsupials");
            println!("          {}", violet.to_owned() + "--herbivorous --placentalia   " + red + "[l]" + reset + " Model for herbivorous placentals");
            println!("          {}", violet.to_owned() + "--herbivorous --rodentia      " + red + "[m]" + reset + " Model for herbivorous rodents");
            println!("          {}", violet.to_owned() + "--omnivorous --artiodactyla   " + red + "[h]" + reset + " Model for omnivor. artiodactyls");
            println!("          {}", violet.to_owned() + "--omnivorous --mammalia       " + red + "[m]" + reset + " Model for omnivorous mammalians");
            println!("          {}", violet.to_owned() + "--omnivorous --rodentia       " + red + "[m]" + reset + " Model for omnivorous rodents");
            println!("          {}", violet.to_owned() + "--unspecified --canidae       " + red + "[m]" + reset + " Model for canids");
            println!("          {}", violet.to_owned() + "--unspecified --carnivora     " + red + "[l]" + reset + " Model for carnivorans");
            println!("          {}", violet.to_owned() + "--unspecified --cricetidae    " + red + "[m]" + reset + " Model for cricetids");
            println!("          {}", violet.to_owned() + "--unspecified --herpestidae   " + red + "[m]" + reset + " Model for herpestids");
            println!("          {}", violet.to_owned() + "--unspecified --marsupialia   " + red + "[m]" + reset + " Model for marsupials");
            println!("          {}", violet.to_owned() + "--unspecified --mustelidae    " + red + "[m]" + reset + " Model for mustelids");
            println!("          {}", violet.to_owned() + "--unspecified --sciuridae     " + red + "[h]" + reset + " Model for sciurids");
            println!();
            println!("   Rept.: {}", violet.to_owned() + "--carnivorous --archosauria   " + red + "[m]" + reset + " Model for carnivorous archosaurs");
            println!("          {}", violet.to_owned() + "--carnivorous --squamata      " + red + "[m]" + reset + " Model for carnivorous squamates");
            println!("          {}", violet.to_owned() + "--herbivorous --aves          " + red + "[h]" + reset + " Model for herbivorous birds");
            println!("          {}", violet.to_owned() + "--herbivorous --reptilia      " + red + "[h]" + reset + " Model for herbivorous reptiles");
            println!("          {}", violet.to_owned() + "--omnivorous --aves           " + red + "[m]" + reset + " Model for omnivorous birds");
            println!("          {}", violet.to_owned() + "--unspecified --testudines    " + red + "[l]" + reset + " Model for turtles");
            println!();
            println!("   Other: {}", violet.to_owned() + "--carnivorous --amniota       " + red + "[l]" + reset + " Model for carnivorous amniotes");
            println!("          {}", violet.to_owned() + "--carnivorous --amphibia      " + red + "[h]" + reset + " Model for carnivorous amphibians");
            println!("          {}", violet.to_owned() + "--carnivorous --tetrapoda     " + red + "[l]" + reset + " Model for carnivorous tetrapods");
            println!("          {}", violet.to_owned() + "--herbivorous --amniota       " + red + "[l]" + reset + " Model for herbivorous amniotes");
            println!("          {}", violet.to_owned() + "--omnivorous --amniota        " + red + "[l]" + reset + " Model for omnivorous amniotes");
            println!();
            println!("          Model reliability: {}", red.to_owned() + "[l]" + reset + " - low, " + red + "[m]" + reset + " - moderate, " + red + "[h]" + reset + " - seemingly high");
            println!();
            println!("{}", red.to_owned() + "Examples" + reset + ":" + yellow + " coprosize --carnivorous --archosauria 27.75" + reset + " [diameter given in mm]");
            println!("{}", yellow.to_owned() + "          coprosize --archosauria --carnivorous 27.75" + reset + " [diameter given in mm]");
            println!();
            println!("          {}", violet.to_owned() + "-s" + reset + ", " + violet + "--subgroups" + reset + " Show all available subgroups within a higher taxon");
            println!();
            println!("{}", red.to_owned() + "Examples" + reset + ":" + yellow + " coprosize --herbivorous --reptilia --subgroups 115.25" + reset);
            println!("         {}", yellow.to_owned() + " coprosize --unspecified --carnivora -s 14.5" + reset);
            println!();
            println!("{}", red.to_owned() + "See also" + reset + ": " + violet + "-a" + reset + ", " + violet + "--about" + reset + "     Show contact and program info");
            println!("          {}", violet.to_owned() + "-c" + reset + ", " + violet + "--changes" + reset + "   Show summarised change notes");
            println!("          {}", violet.to_owned() + "-C" + reset + ", " + violet + "--citation" + reset + "  Show how to cite this program");
            println!("          {}", violet.to_owned() + "-h" + reset + ", " + violet + "--help" + reset + "      Show the help menu");
            println!("          {}", violet.to_owned() + "-l" + reset + ", " + violet + "--license" + reset + "   Show licensing information");
            println!("          {}", violet.to_owned() + "-v" + reset + ", " + violet + "--version" + reset + "   Show the program version");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022–2023 Piotr Bajdek");
            println!();
            println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
            println!();
            println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
            println!();
            println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
            exit(0);
        }

        // VERSION

        if argument == "-v" || argument == "--version" {
            println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "1.0.4" + reset);
            println!("January 31, 2023");
            exit(0);
        }
    }
}
