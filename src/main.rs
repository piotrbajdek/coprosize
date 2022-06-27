// COPROSIZE VERSION 1.0.0-ALPHA.9 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// MAIN FILE

use std::env;

fn main() {

   let clear = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";

// ARGUMENTS ANYWHERE WITHIN THE STRING

   for argument in env::args() {

      if argument == "-a" || argument == "--about" {
      coprosize::about();
      return;
      }

      if argument == "-c" || argument == "--changes" {
      coprosize::changes();
      return;
      }

      if argument == "-C" || argument == "--citation" {
      coprosize::citation();
      return;
      }

      if argument == "-h" || argument == "--help" {
      coprosize::help();
      return;
      }

      if argument == "-l" || argument == "--license" {
      coprosize::license();
      return;
      }

      if argument == "-v" || argument == "--version" {
      coprosize::version();
      return;
      }
   }

// COLLECT ARGUMENTS FOR CALCULATIONS

   let args: Vec<String> = env::args().collect();

   let input1 = args.get(1).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + clear));
   let input2 = args.get(2).expect(&(red.to_owned() + "Invalid or missing arguments! See: --help" + clear));

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
