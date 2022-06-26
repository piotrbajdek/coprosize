// COPROSIZE VERSION 1.0.0-ALPHA.8 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

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

      if argument == "-a" || argument == "--about" { coprosize::about(clear, blue_underlined) }
      if argument == "-c" || argument == "--changes" { coprosize::changes(clear, cyan) }
      if argument == "-C" || argument == "--citation" { coprosize::citation() }
      if argument == "-h" || argument == "--help" { coprosize::help(clear, red, bright_yellow, cyan) }
      if argument == "-l" || argument == "--license" { coprosize::license(clear, bright_yellow) }
      if argument == "-v" || argument == "--version" { coprosize::version() }
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
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::carnivorous_amphibia(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }
   
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::carnivorous_amphibia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// CARNIVOROUS ARCHOSAURIA

   if input1 == "--carnivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--carnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::carnivorous_archosauria(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::carnivorous_archosauria(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// CARNIVOROUS FELIDAE

   if input1 == "--carnivorous" && input2 == "--felidae" || input1 == "--felidae" && input2 == "--carnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::carnivorous_felidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::carnivorous_felidae(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// CARNIVOROUS MAMMALIA

   if input1 == "--carnivorous" && input2 == "--mammalia" || input1 == "--mammalia" && input2 == "--carnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::carnivorous_mammalia(clear, bright_yellow, cyan, diameter, dcal);
         println!("");
         coprosize::carnivorous_felidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::carnivorous_mammalia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// CARNIVOROUS SQUAMATA

   if input1 == "--carnivorous" && input2 == "--squamata" || input1 == "--squamata" && input2 == "--carnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::carnivorous_squamata(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::carnivorous_squamata(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS AFROTHERIA

   if input1 == "--herbivorous" && input2 == "--afrotheria" || input1 == "--afrotheria" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_afrotheria(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_afrotheria(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS AVES

   if input1 == "--herbivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_aves(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_aves(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS LAGOMORPHA

   if input1 == "--herbivorous" && input2 == "--lagomorpha" || input1 == "--lagomorpha" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_lagomorpha(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_lagomorpha(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS MARSUPIALIA

   if input1 == "--herbivorous" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_marsupialia(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_marsupialia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS REPTILIA

   if input1 == "--herbivorous" && input2 == "--reptilia" || input1 == "--reptilia" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_reptilia(clear, bright_yellow, cyan, diameter, dcal);
         println!("");
         coprosize::herbivorous_aves(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_reptilia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// HERBIVOROUS RODENTIA

   if input1 == "--herbivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--herbivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::herbivorous_rodentia(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::herbivorous_rodentia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// OMNIVOROUS AVES

   if input1 == "--omnivorous" && input2 == "--aves" || input1 == "--aves" && input2 == "--omnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::omnivorous_aves(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::omnivorous_aves(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// OMNIVOROUS RODENTIA

   if input1 == "--omnivorous" && input2 == "--rodentia" || input1 == "--rodentia" && input2 == "--omnivorous" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::omnivorous_rodentia(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::omnivorous_rodentia(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// UNSPECIFIED CANIDAE

   if input1 == "--unspecified" && input2 == "--canidae" || input1 == "--canidae" && input2 == "--unspecified" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::unspecified_canidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::unspecified_canidae(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// UNSPECIFIED CARNIVORA

   if input1 == "--unspecified" && input2 == "--carnivora" || input1 == "--carnivora" && input2 == "--unspecified" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::unspecified_carnivora(clear, bright_yellow, cyan, diameter, dcal);
         println!("");
         coprosize::unspecified_canidae(clear, bright_yellow, cyan, diameter, dcal);
         println!("");
         coprosize::unspecified_herpestidae(clear, bright_yellow, cyan, diameter, dcal);
         println!("");
         coprosize::unspecified_mustelidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::unspecified_carnivora(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// UNSPECIFIED HERPESTIDAE

   if input1 == "--unspecified" && input2 == "--herpestidae" || input1 == "--herpestidae" && input2 == "--unspecified" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::unspecified_herpestidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::unspecified_herpestidae(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// UNSPECIFIED MUSTELIDAE

   if input1 == "--unspecified" && input2 == "--mustelidae" || input1 == "--mustelidae" && input2 == "--unspecified" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::unspecified_mustelidae(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::unspecified_mustelidae(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// UNSPECIFIED TESTUDINES

   if input1 == "--unspecified" && input2 == "--testudines" || input1 == "--testudines" && input2 == "--unspecified" {

   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));

      if diameter == "-s" || diameter == "--subgroups" {
         println!("{}", red.to_owned() + "No subgroups available for this taxon and diet" + clear);
         let diameter = args.get(4).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
         let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));
         coprosize::unspecified_testudines(clear, bright_yellow, cyan, diameter, dcal);
         return;
      }

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   coprosize::unspecified_testudines(clear, bright_yellow, cyan, diameter, dcal);
   return;
   }

// INVALID ARGUMENT [1] AND/OR [2]

   panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + clear);

}
