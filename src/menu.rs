// COPROSIZE VERSION 1.0.2 / MIT LICENSE © 2022 PIOTR BAJDEK

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
    let yellow = "\x1b[93m";
    let blue_underlined = "\x1b[34;4m";
    let cyan = "\x1b[36m";

    // ARGUMENTS ANYWHERE WITHIN THE STRING

    for argument in env::args() {
        // ABOUT

        if argument == "-a" || argument == "--about" {
            println!("Program:  {}", yellow.to_owned() + "coprosize" + reset);
            println!("Version:  1.0.2");
            println!("Date:     September 30, 2022");
            println!("Author:   Piotr Bajdek");
            println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + reset);
            println!("ORCID:    {}", blue_underlined.to_owned() + "https://orcid.org/0000-0003-2678-3122" + reset);
            println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/coprosize" + reset);
            println!("License:  MIT License © 2022 Piotr Bajdek");
            exit(0);
        }

        // CHANGES

        if argument == "-c" || argument == "--changes" {
            println!("30.09.2022 v1.0.2 – Enhanced source code");
            println!("16.08.2022 v1.0.1 – Enhanced source code and layout");
            println!("02.07.2022 v1.0.0 – Implemented {}", cyan.to_owned() + "--carnivorous --amniota" + reset + ", " + cyan + "--carnivorous --amphibia" + reset + ", " + cyan + "--carnivorous --archosauria" + reset + ", " + cyan + "--carnivorous --felidae" + reset + ", " + cyan + "--carnivorous --mammalia" + reset + ", " + cyan + "--carnivorous --squamata" + reset + ", " + cyan + "--carnivorous --tetrapoda" + reset + ", " + cyan + "--herbivorous --afrotheria" + reset + ", " + cyan + "--herbivorous --amniota" + reset + ", " + cyan + "--herbivorous --artiodactyla" + reset + ", " + cyan + "--herbivorous --aves" + reset + ", " + cyan + "--herbivorous --bovidae" + reset + ", " + cyan + "--herbivorous --cervidae" + reset + ", " + cyan + "--herbivorous --lagomorpha" + reset + ", " + cyan + "--herbivorous --mammalia" + reset + ", " + cyan + "--herbivorous --marsupialia" + reset + ", " + cyan + "--herbivorous --placentalia" + reset + ", " + cyan + "--herbivorous --reptilia" + reset + ", " + cyan + "--herbivorous --rodentia" + reset + ", " + cyan + "--omnivorous --amniota" + reset + ", " + cyan + "--omnivorous --artiodactyla" + reset + ", " + cyan + "--omnivorous --aves" + reset + ", " + cyan + "--omnivorous --mammalia" + reset + ", " + cyan + "--omnivorous --rodentia" + reset + ", " + cyan + "--unspecified --canidae" + reset + ", " + cyan + "--unspecified --carnivora" + reset + ", " + cyan + "--unspecified --cricetidae" + reset + ", " + cyan + "--unspecified --herpestidae" + reset + ", " + cyan + "--unspecified --marsupialia" + reset + ", " + cyan + "--unspecified --mustelidae" + reset + ", " + cyan + "--unspecified --sciuridae" + reset + ", " + cyan + "--unspecified --testudines" + reset);
            exit(0);
        }

        // CITATION

        if argument == "-C" || argument == "--citation" {
            coprosize::citation();
            exit(0);
        }

        // HELP

        if argument == "-h" || argument == "--help" {
            println!("Usage:{}", yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [COPROLITE DIAMETER IN MM]");
            println!("      {}", yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [OPTIONS] [DIAMETER IN MM]");
            println!("{}", reset);
            println!("   Mamm.: {}", cyan.to_owned() + "--carnivorous --felidae       " + red + "[m]" + reset + " Model for carnivorous felids");
            println!("          {}", cyan.to_owned() + "--carnivorous --mammalia      " + red + "[l]" + reset + " Model for carnivorous mammalians");
            println!("          {}", cyan.to_owned() + "--herbivorous --afrotheria    " + red + "[l]" + reset + " Model for herbivor. afrotherians");
            println!("          {}", cyan.to_owned() + "--herbivorous --artiodactyla  " + red + "[m]" + reset + " Model for herbivor. artiodactyls");
            println!("          {}", cyan.to_owned() + "--herbivorous --bovidae       " + red + "[m]" + reset + " Model for herbivorous bovids");
            println!("          {}", cyan.to_owned() + "--herbivorous --cervidae      " + red + "[h]" + reset + " Model for herbivorous cervids");
            println!("          {}", cyan.to_owned() + "--herbivorous --lagomorpha    " + red + "[h]" + reset + " Model for herbivorous lagomorphs");
            println!("          {}", cyan.to_owned() + "--herbivorous --mammalia      " + red + "[l]" + reset + " Model for herbivorous mammalians");
            println!("          {}", cyan.to_owned() + "--herbivorous --marsupialia   " + red + "[m]" + reset + " Model for herbivorous marsupials");
            println!("          {}", cyan.to_owned() + "--herbivorous --placentalia   " + red + "[l]" + reset + " Model for herbivorous placentals");
            println!("          {}", cyan.to_owned() + "--herbivorous --rodentia      " + red + "[m]" + reset + " Model for herbivorous rodents");
            println!("          {}", cyan.to_owned() + "--omnivorous --artiodactyla   " + red + "[h]" + reset + " Model for omnivor. artiodactyls");
            println!("          {}", cyan.to_owned() + "--omnivorous --mammalia       " + red + "[m]" + reset + " Model for omnivorous mammalians");
            println!("          {}", cyan.to_owned() + "--omnivorous --rodentia       " + red + "[m]" + reset + " Model for omnivorous rodents");
            println!("          {}", cyan.to_owned() + "--unspecified --canidae       " + red + "[m]" + reset + " Model for canids");
            println!("          {}", cyan.to_owned() + "--unspecified --carnivora     " + red + "[l]" + reset + " Model for carnivorans");
            println!("          {}", cyan.to_owned() + "--unspecified --cricetidae    " + red + "[m]" + reset + " Model for cricetids");
            println!("          {}", cyan.to_owned() + "--unspecified --herpestidae   " + red + "[m]" + reset + " Model for herpestids");
            println!("          {}", cyan.to_owned() + "--unspecified --marsupialia   " + red + "[m]" + reset + " Model for marsupials");
            println!("          {}", cyan.to_owned() + "--unspecified --mustelidae    " + red + "[m]" + reset + " Model for mustelids");
            println!("          {}", cyan.to_owned() + "--unspecified --sciuridae     " + red + "[h]" + reset + " Model for sciurids");
            println!();
            println!("   Rept.: {}", cyan.to_owned() + "--carnivorous --archosauria   " + red + "[m]" + reset + " Model for carnivorous archosaurs");
            println!("          {}", cyan.to_owned() + "--carnivorous --squamata      " + red + "[m]" + reset + " Model for carnivorous squamates");
            println!("          {}", cyan.to_owned() + "--herbivorous --aves          " + red + "[h]" + reset + " Model for herbivorous birds");
            println!("          {}", cyan.to_owned() + "--herbivorous --reptilia      " + red + "[h]" + reset + " Model for herbivorous reptiles");
            println!("          {}", cyan.to_owned() + "--omnivorous --aves           " + red + "[m]" + reset + " Model for omnivorous birds");
            println!("          {}", cyan.to_owned() + "--unspecified --testudines    " + red + "[l]" + reset + " Model for turtles");
            println!();
            println!("   Other: {}", cyan.to_owned() + "--carnivorous --amniota       " + red + "[l]" + reset + " Model for carnivorous amniotes");
            println!("          {}", cyan.to_owned() + "--carnivorous --amphibia      " + red + "[h]" + reset + " Model for carnivorous amphibians");
            println!("          {}", cyan.to_owned() + "--carnivorous --tetrapoda     " + red + "[l]" + reset + " Model for carnivorous tetrapods");
            println!("          {}", cyan.to_owned() + "--herbivorous --amniota       " + red + "[l]" + reset + " Model for herbivorous amniotes");
            println!("          {}", cyan.to_owned() + "--omnivorous --amniota        " + red + "[l]" + reset + " Model for omnivorous amniotes");
            println!();
            println!("          Model reliability: {}", red.to_owned() + "[l]" + reset + " - low, " + red + "[m]" + reset + " - moderate, " + red + "[h]" + reset + " - seemingly high");
            println!();
            println!("Examples:{}", yellow.to_owned() + " coprosize --carnivorous --archosauria 27.75" + reset + " [diameter given in mm]");
            println!("{}", yellow.to_owned() + "          coprosize --archosauria --carnivorous 27.75" + reset + " [diameter given in mm]");
            println!();
            println!("          {}", cyan.to_owned() + "-s" + reset + ", " + cyan + "--subgroups" + reset + " Show all available subgroups within a higher taxon");
            println!();
            println!("Examples:{}", yellow.to_owned() + " coprosize --herbivorous --reptilia --subgroups 115.25" + reset);
            println!("         {}", yellow.to_owned() + " coprosize --unspecified --carnivora -s 14.5" + reset);
            println!();
            println!("See also: {}", cyan.to_owned() + "-a" + reset + ", " + cyan + "--about" + reset + "     Show contact and program info");
            println!("          {}", cyan.to_owned() + "-c" + reset + ", " + cyan + "--changes" + reset + "   Show simplified change notes");
            println!("          {}", cyan.to_owned() + "-C" + reset + ", " + cyan + "--citation" + reset + "  Show how to cite this program");
            println!("          {}", cyan.to_owned() + "-h" + reset + ", " + cyan + "--help" + reset + "      Show this help");
            println!("          {}", cyan.to_owned() + "-l" + reset + ", " + cyan + "--license" + reset + "   Show licensing information");
            println!("          {}", cyan.to_owned() + "-v" + reset + ", " + cyan + "--version" + reset + "   Show the program version");
            exit(0);
        }

        // LICENSE

        if argument == "-l" || argument == "--license" {
            println!("{}", yellow.to_owned() + "MIT License" + reset);
            println!();
            println!("Copyright © 2022 Piotr Bajdek");
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
            println!("Version: {}", yellow.to_owned() + "1.0.2" + reset);
            println!("September 30, 2022");
            exit(0);
        }
    }
}
