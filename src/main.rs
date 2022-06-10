// COPROSIZE VERSION 1.0.0-ALPHA.5 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

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
      println!("Version:  1.0.0-alpha.5");
      println!("Date:     June 10, 2022");
      println!("Author:   Piotr Bajdek");
      println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + clear);
      println!("ORCID:    {}", blue_underlined.to_owned() + "https://orcid.org/0000-0003-2678-3122" + clear);
      println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/coprosize" + clear);
      println!("License:  The MIT License (MIT) © 2022 Piotr Bajdek");
      return;
      }

// CHANGES

      if argument == "-c" || argument == "--changes" {
      println!("--.--.---- v1.0.0 – Implemented {}", cyan.to_owned() + "--carnivorous --amphibia" + clear + ", " + cyan + "--carnivorous --archosauria" + clear + ", " + cyan + "--carnivorous --squamata" + clear + ", " + cyan + "--herbivorous --archosauria" + clear + ", " + cyan + "--herbivorous --marsupialia" + clear + ", " + cyan + "--herbivorous --reptilia" + clear + ", " + cyan + "--omnivorous --archosauria" + clear + ", " + cyan + "--unspecified --testudines");
      return;
      }

// CITATION

      if argument == "-C" || argument == "--citation" {
      println!("Bajdek, P., 2022. coprosize (version 1.0.0-alpha.5). [computer software] https://github.com/piotrbajdek/coprosize");
      return;
      }

// HELP

      if argument == "-h" || argument == "--help" {
      println!("Usage:{}", bright_yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [COPROLITE DIAMETER IN MM]");
      println!("{}", clear);
      println!("   Rept.: {}", cyan.to_owned() + "--carnivorous --archosauria " + red + "[l/m]" + clear + " Model for carnivorous archosaurs");
      println!("          {}", cyan.to_owned() + "--carnivorous --squamata      " + red + "[m]" + clear + " Model for carnivorous squamates");
      println!("          {}", cyan.to_owned() + "--herbivorous --archosauria   " + red + "[h]" + clear + " Model for herbivorous archosaurs");
      println!("          {}", cyan.to_owned() + "--herbivorous --reptilia      " + red + "[h]" + clear + " Model for herbivorous reptiles");
      println!("          {}", cyan.to_owned() + "--omnivorous --archosauria  " + red + "[l/m]" + clear + " Model for omnivorous archosaurs");
      println!("          {}", cyan.to_owned() + "--unspecified --testudines    " + red + "[l]" + clear + " Model for turtles of all diets");
      println!("   Mamm.: {}", cyan.to_owned() + "--herbivorous --marsupialia " + red + "[m/h]" + clear + " Model for herbivorous marsupials");
      println!("   Other: {}", cyan.to_owned() + "--carnivorous --amphibia      " + red + "[h]" + clear + " Model for carnivorous amphibians");
      println!("");
      println!("          Model reliability: {}", red.to_owned() + "[l]" + clear + " - low, " + red + "[m]" + clear + " - moderate, " + red + "[h]" + clear + " - seemingly high");
      println!("");
      println!("Examples:{}", bright_yellow.to_owned() + " coprosize --carnivorous --archosauria 27.75" + clear + " [diameter given in mm]");
      println!("{}", bright_yellow.to_owned() + "          coprosize --archosauria --carnivorous 27.75" + clear + " [diameter given in mm]");
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
      println!("Version: 1.0.0-alpha.5");
      println!("June 10, 2022");
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
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power1 = f32::powi(dcal as f32, 3);
   let power2 = f32::powi(dcal as f32, 2);
   let mass = (0.0004064349 * power1) - (0.0041616775 * power2) + (0.0147514015 * dcal) - 0.0122201640;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0004064349 * {}", diameter.to_owned() + "³ - 0.0041616775 * " + diameter + "² + 0.0147514015 * " + diameter + " - 0.0122201640" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous batrachians");
   return;
   }

// CARNIVOROUS ARCHOSAURIA

   if input1 == "--carnivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--carnivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.3933363596);
   let mass = 0.0056582325 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0056582325 * {}", diameter.to_owned() + "²·³⁹³³³⁶³⁵⁹⁶" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous birds and crocodilians");
   return;
   }

// CARNIVOROUS SQUAMATA

   if input1 == "--carnivorous" && input2 == "--squamata" || input1 == "--squamata" && input2 == "--carnivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.3414629141);
   let mass = 0.0004822862 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0004822862 * {}", diameter.to_owned() + "²·³⁴¹⁴⁶²⁹¹⁴¹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous squamates");
   return;
   }

// HERBIVOROUS ARCHOSAURIA

   if input1 == "--herbivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--herbivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.8100206869);
   let mass = 0.0078389131 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0078389131 * {}", diameter.to_owned() + "²·⁸¹⁰⁰²⁰⁶⁸⁶⁹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous birds");
   return;
   }

// HERBIVOROUS MARSUPIALIA

   if input1 == "--herbivorous" && input2 == "--marsupialia" || input1 == "--marsupialia" && input2 == "--herbivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.2193247015);
   let mass = 0.0224440978 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0224440978 * {}", diameter.to_owned() + "²·²¹⁹³²⁴⁷⁰¹⁵" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous marsupials");
   return;
   }

// HERBIVOROUS REPTILIA

   if input1 == "--herbivorous" && input2 == "--reptilia" || input1 == "--reptilia" && input2 == "--herbivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.8272692414);
   let mass = 0.0076217107 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0076217107 * {}", diameter.to_owned() + "²·⁸²⁷²⁶⁹²⁴¹⁴" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous birds and turtles");
   return;
   }

// OMNIVOROUS ARCHOSAURIA

   if input1 == "--omnivorous" && input2 == "--archosauria" || input1 == "--archosauria" && input2 == "--omnivorous" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 3.0157553176);
   let mass = 0.0014342026 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0014342026 * {}", diameter.to_owned() + "³·⁰¹⁵⁷⁵⁵³¹⁷⁶" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant omnivorous birds");
   return;
   }

// UNSPECIFIED TESTUDINES

   if input1 == "--unspecified" && input2 == "--testudines" || input1 == "--testudines" && input2 == "--unspecified" {
   let diameter = args.get(3).expect(&(red.to_owned() + "No diameter inserted! See: --help" + clear));
   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + clear));

   let power = f32::powf(dcal as f32, 2.1345692041);
   let mass = 0.0251303814 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0251303814 * {}", diameter.to_owned() + "²·¹³⁴⁵⁶⁹²⁰⁴¹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous, omnivorous and herbivorous turtles");
   return;
   }

// INVALID ARGUMENT [1] AND/OR [2]

   panic!("{}", red.to_owned() + "Invalid arguments provided! See: --help" + clear);

}
