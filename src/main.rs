// COPROSIZE VERSION 1.0.0 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// MAIN FILE

use std::env;

fn main() {

   let clear = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let bright_yellow = "\x1b[93m";
   let blue_underlined = "\x1b[34;4m";
   let cyan = "\x1b[36m";

// ARGUMENTS ANYWHERE WITHIN THE STRING

   for argument in env::args() {

// ABOUT

      if argument == "-a" || argument == "--about" {

      println!("Program:  coprosize");
      println!("Version:  1.0.0");
      println!("Date:     July 2, 2022");
      println!("Author:   Piotr Bajdek");
      println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + clear);
      println!("ORCID:    {}", blue_underlined.to_owned() + "https://orcid.org/0000-0003-2678-3122" + clear);
      println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/coprosize" + clear);
      println!("License:  The MIT License (MIT) © 2022 Piotr Bajdek");
      return;
      }

// CHANGES

      if argument == "-c" || argument == "--changes" {

      println!("02.07.2022 v1.0.0 – Implemented {}", cyan.to_owned() + "--carnivorous --amniota" + clear + ", " + cyan + "--carnivorous --amphibia" + clear + ", " + cyan + "--carnivorous --archosauria" + clear + ", " + cyan + "--carnivorous --felidae" + clear + ", " + cyan + "--carnivorous --mammalia" + clear + ", " + cyan + "--carnivorous --squamata" + clear + ", " + cyan + "--carnivorous --tetrapoda" + clear + ", " + cyan + "--herbivorous --afrotheria" + clear + ", " + cyan + "--herbivorous --amniota" + clear + ", " + cyan + "--herbivorous --artiodactyla" + clear + ", " + cyan + "--herbivorous --aves" + clear + ", " + cyan + "--herbivorous --bovidae" + clear + ", " + cyan + "--herbivorous --cervidae" + clear + ", " + cyan + "--herbivorous --lagomorpha" + clear + ", " + cyan + "--herbivorous --mammalia" + clear  + ", " + cyan + "--herbivorous --marsupialia" + clear + ", " + cyan + "--herbivorous --placentalia" + clear + ", " + cyan + "--herbivorous --reptilia" + clear + ", " + cyan + "--herbivorous --rodentia" + clear + ", " + cyan + "--omnivorous --amniota" + clear + ", " + cyan + "--omnivorous --artiodactyla" + clear + ", " + cyan + "--omnivorous --aves" + clear + ", " + cyan + "--omnivorous --mammalia" + clear + ", " + cyan + "--omnivorous --rodentia" + clear + ", " + cyan + "--unspecified --canidae" + clear + ", " + cyan + "--unspecified --carnivora" + clear + ", " + cyan + "--unspecified --cricetidae" + clear + ", " + cyan + "--unspecified --herpestidae" + clear + ", " + cyan + "--unspecified --marsupialia" + clear + ", " + cyan + "--unspecified --mustelidae" + clear + ", " + cyan + "--unspecified --sciuridae" + clear + ", " + cyan + "--unspecified --testudines" + clear);
      return;
      }

// CITATION

      if argument == "-C" || argument == "--citation" {

      coprosize::citation();
      return;
      }

// HELP

      if argument == "-h" || argument == "--help" {

      println!("Usage:{}", bright_yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [COPROLITE DIAMETER IN MM]");
      println!("      {}", bright_yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [OPTIONS] [DIAMETER IN MM]");
      println!("{}", clear);
      println!("   Mamm.: {}", cyan.to_owned() + "--carnivorous --felidae       " + red + "[m]" + clear + " Model for carnivorous felids");
      println!("          {}", cyan.to_owned() + "--carnivorous --mammalia      " + red + "[l]" + clear + " Model for carnivorous mammalians");
      println!("          {}", cyan.to_owned() + "--herbivorous --afrotheria    " + red + "[l]" + clear + " Model for herbivor. afrotherians");
      println!("          {}", cyan.to_owned() + "--herbivorous --artiodactyla  " + red + "[m]" + clear + " Model for herbivor. artiodactyls");
      println!("          {}", cyan.to_owned() + "--herbivorous --bovidae       " + red + "[m]" + clear + " Model for herbivorous bovids");
      println!("          {}", cyan.to_owned() + "--herbivorous --cervidae      " + red + "[h]" + clear + " Model for herbivorous cervids");
      println!("          {}", cyan.to_owned() + "--herbivorous --lagomorpha    " + red + "[h]" + clear + " Model for herbivorous lagomorphs");
      println!("          {}", cyan.to_owned() + "--herbivorous --mammalia      " + red + "[l]" + clear + " Model for herbivorous mammalians");
      println!("          {}", cyan.to_owned() + "--herbivorous --marsupialia   " + red + "[m]" + clear + " Model for herbivorous marsupials");
      println!("          {}", cyan.to_owned() + "--herbivorous --placentalia   " + red + "[l]" + clear + " Model for herbivorous placentals");
      println!("          {}", cyan.to_owned() + "--herbivorous --rodentia      " + red + "[m]" + clear + " Model for herbivorous rodents");
      println!("          {}", cyan.to_owned() + "--omnivorous --artiodactyla   " + red + "[h]" + clear + " Model for omnivor. artiodactyls");
      println!("          {}", cyan.to_owned() + "--omnivorous --mammalia       " + red + "[m]" + clear + " Model for omnivorous mammalians");
      println!("          {}", cyan.to_owned() + "--omnivorous --rodentia       " + red + "[m]" + clear + " Model for omnivorous rodents");
      println!("          {}", cyan.to_owned() + "--unspecified --canidae       " + red + "[m]" + clear + " Model for canids");
      println!("          {}", cyan.to_owned() + "--unspecified --carnivora     " + red + "[l]" + clear + " Model for carnivorans");
      println!("          {}", cyan.to_owned() + "--unspecified --cricetidae    " + red + "[m]" + clear + " Model for cricetids");
      println!("          {}", cyan.to_owned() + "--unspecified --herpestidae   " + red + "[m]" + clear + " Model for herpestids");
      println!("          {}", cyan.to_owned() + "--unspecified --marsupialia   " + red + "[m]" + clear + " Model for marsupials");
      println!("          {}", cyan.to_owned() + "--unspecified --mustelidae    " + red + "[m]" + clear + " Model for mustelids");
      println!("          {}", cyan.to_owned() + "--unspecified --sciuridae     " + red + "[h]" + clear + " Model for sciurids");
      println!("");
      println!("   Rept.: {}", cyan.to_owned() + "--carnivorous --archosauria   " + red + "[m]" + clear + " Model for carnivorous archosaurs");
      println!("          {}", cyan.to_owned() + "--carnivorous --squamata      " + red + "[m]" + clear + " Model for carnivorous squamates");
      println!("          {}", cyan.to_owned() + "--herbivorous --aves          " + red + "[h]" + clear + " Model for herbivorous birds");
      println!("          {}", cyan.to_owned() + "--herbivorous --reptilia      " + red + "[h]" + clear + " Model for herbivorous reptiles");
      println!("          {}", cyan.to_owned() + "--omnivorous --aves           " + red + "[m]" + clear + " Model for omnivorous birds");
      println!("          {}", cyan.to_owned() + "--unspecified --testudines    " + red + "[l]" + clear + " Model for turtles");
      println!("");
      println!("   Other: {}", cyan.to_owned() + "--carnivorous --amniota       " + red + "[l]" + clear + " Model for carnivorous amniotes");
      println!("          {}", cyan.to_owned() + "--carnivorous --amphibia      " + red + "[h]" + clear + " Model for carnivorous amphibians");
      println!("          {}", cyan.to_owned() + "--carnivorous --tetrapoda     " + red + "[l]" + clear + " Model for carnivorous tetrapods");
      println!("          {}", cyan.to_owned() + "--herbivorous --amniota       " + red + "[l]" + clear + " Model for herbivorous amniotes");
      println!("          {}", cyan.to_owned() + "--omnivorous --amniota        " + red + "[l]" + clear + " Model for omnivorous amniotes");
      println!("");
      println!("          Model reliability: {}", red.to_owned() + "[l]" + clear + " - low, " + red + "[m]" + clear + " - moderate, " + red + "[h]" + clear + " - seemingly high");
      println!("");
      println!("Examples:{}", bright_yellow.to_owned() + " coprosize --carnivorous --archosauria 27.75" + clear + " [diameter given in mm]");
      println!("{}", bright_yellow.to_owned() + "          coprosize --archosauria --carnivorous 27.75" + clear + " [diameter given in mm]");
      println!("");
      println!("          {}", cyan.to_owned() + "-s" + clear + ", " + cyan + "--subgroups" + clear + " Show all available subgroups within a higher taxon");
      println!("");
      println!("Examples:{}", bright_yellow.to_owned() + " coprosize --herbivorous --reptilia --subgroups 115.25" + clear);
      println!("         {}", bright_yellow.to_owned() + " coprosize --unspecified --carnivora -s 14.5" + clear);
      println!("");
      println!("See also: {}", cyan.to_owned() + "-a" + clear + ", " + cyan + "--about" + clear + "     Show contact and program info");
      println!("          {}", cyan.to_owned() + "-c" + clear + ", " + cyan + "--changes" + clear + "   Show simplified change notes");
      println!("          {}", cyan.to_owned() + "-C" + clear + ", " + cyan + "--citation" + clear + "  Show how to cite this program");
      println!("          {}", cyan.to_owned() + "-h" + clear + ", " + cyan + "--help" + clear + "      Show this help");
      println!("          {}", cyan.to_owned() + "-l" + clear + ", " + cyan + "--license" + clear + "   Show licesing information");
      println!("          {}", cyan.to_owned() + "-v" + clear + ", " + cyan + "--version" + clear + "   Show the program version");
      return;
      }

// LICENSE

      if argument == "-l" || argument == "--license" {

      println!("{}", bright_yellow.to_owned() + "The MIT License (MIT)" + clear);
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
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
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
