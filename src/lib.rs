// COPROSIZE VERSION 1.0.0-ALPHA.8 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// LIBRARY

use std::process::exit;

// ABOUT

pub fn about(clear: &str, blue_underlined: &str) {

   println!("Program:  coprosize");
   println!("Version:  1.0.0-alpha.8");
   println!("Date:     June 26, 2022");
   println!("Author:   Piotr Bajdek");
   println!("Contact:  {}", blue_underlined.to_owned() + "piotr.bajdek@proton.me" + clear);
   println!("ORCID:    {}", blue_underlined.to_owned() + "https://orcid.org/0000-0003-2678-3122" + clear);
   println!("Source:   {}", blue_underlined.to_owned() + "https://github.com/piotrbajdek/coprosize" + clear);
   println!("License:  The MIT License (MIT) © 2022 Piotr Bajdek");
   exit(0);
   }

// CHANGES

pub fn changes(clear: &str, cyan: &str) {

   println!("--.--.---- v1.0.0 – Implemented {}", cyan.to_owned() + "--carnivorous --amphibia" + clear + ", " + cyan + "--carnivorous --archosauria" + clear + ", " + cyan + "--carnivorous --felidae" + clear + ", " + cyan + "--carnivorous --mammalia" + clear + ", " + cyan + "--carnivorous --squamata" + clear + ", " + cyan + "--herbivorous --afrotheria" + clear + ", " + cyan + "--herbivorous --aves" + clear + ", " + cyan + "--herbivorous --lagomorpha" + clear + ", " + cyan + "--herbivorous --marsupialia" + clear + ", " + cyan + "--herbivorous --reptilia" + clear + ", " + cyan + "--herbivorous --rodentia" + clear + ", " + cyan + "--omnivorous --aves" + clear + ", " + cyan + "--omnivorous --rodentia" + clear + ", " + cyan + "--unspecified --canidae" + clear + ", " + cyan + "--unspecified --carnivora" + clear + ", " + cyan + "--unspecified --herpestidae" + clear + ", " + cyan + "--unspecified --mustelidae" + clear + ", " + cyan + "--unspecified --testudines");
   exit(0);
   }

// CITATION

pub fn citation() {

   println!("Bajdek, P., 2022. coprosize (version 1.0.0-alpha.8). [computer software] https://github.com/piotrbajdek/coprosize");
   exit(0);
   }

// HELP

pub fn help(clear: &str, red: &str, bright_yellow: &str, cyan: &str) {

   println!("Usage:{}", bright_yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [COPROLITE DIAMETER IN MM]");
   println!("      {}", bright_yellow.to_owned() + "    coprosize [DIET / TAXON] [TAXON / DIET] [OPTIONS] [DIAMETER IN MM]");
   println!("{}", clear);
   println!("   Mamm.: {}", cyan.to_owned() + "--carnivorous --felidae     " + red + "[m/h]" + clear + " Model for carnivorous felids");
   println!("          {}", cyan.to_owned() + "--carnivorous --mammalia      " + red + "[l]" + clear + " Model for carnivorous mammalians");
   println!("          {}", cyan.to_owned() + "--herbivorous --afrotheria    " + red + "[l]" + clear + " Model for herbivor. afrotherians");
   println!("          {}", cyan.to_owned() + "--herbivorous --lagomorpha    " + red + "[h]" + clear + " Model for herbivorous lagomorphs");
   println!("          {}", cyan.to_owned() + "--herbivorous --marsupialia " + red + "[m/h]" + clear + " Model for herbivorous marsupials");
   println!("          {}", cyan.to_owned() + "--herbivorous --rodentia    " + red + "[l/m]" + clear + " Model for herbivorous rodents");
   println!("          {}", cyan.to_owned() + "--omnivorous --rodentia       " + red + "[m]" + clear + " Model for omnivorous rodents");
   println!("          {}", cyan.to_owned() + "--unspecified --canidae       " + red + "[m]" + clear + " Model for canids");
   println!("          {}", cyan.to_owned() + "--unspecified --carnivora     " + red + "[l]" + clear + " Model for carnivorans");
   println!("          {}", cyan.to_owned() + "--unspecified --herpestidae   " + red + "[m]" + clear + " Model for herpestids");
   println!("          {}", cyan.to_owned() + "--unspecified --mustelidae  " + red + "[m/h]" + clear + " Model for mustelids");
   println!("");
   println!("   Rept.: {}", cyan.to_owned() + "--carnivorous --archosauria " + red + "[l/m]" + clear + " Model for carnivorous archosaurs");
   println!("          {}", cyan.to_owned() + "--carnivorous --squamata      " + red + "[m]" + clear + " Model for carnivorous squamates");
   println!("          {}", cyan.to_owned() + "--herbivorous --aves          " + red + "[h]" + clear + " Model for herbivorous birds");
   println!("          {}", cyan.to_owned() + "--herbivorous --reptilia      " + red + "[h]" + clear + " Model for herbivorous reptiles");
   println!("          {}", cyan.to_owned() + "--omnivorous --aves         " + red + "[l/m]" + clear + " Model for omnivorous birds");
   println!("          {}", cyan.to_owned() + "--unspecified --testudines    " + red + "[l]" + clear + " Model for turtles");
   println!("");
   println!("   Other: {}", cyan.to_owned() + "--carnivorous --amphibia      " + red + "[h]" + clear + " Model for carnivorous amphibians");
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
   exit(0);
   }

// LICENSE

pub fn license(clear: &str, bright_yellow: &str) {

   println!("{}", bright_yellow.to_owned() + "The MIT License (MIT)" + clear);
   println!("");
   println!("Copyright © 2022 Piotr Bajdek");
   println!("");
   println!("Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:");
   println!("");
   println!("The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.");
   println!("");
   println!("THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.");
   exit(0);
   }

// VERSION

pub fn version() {

   println!("Version: 1.0.0-alpha.8");
   println!("June 26, 2022");
   exit(0);
   }

// CARNIVOROUS AMPHIBIA

pub fn carnivorous_amphibia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

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
   }

// CARNIVOROUS ARCHOSAURIA

pub fn carnivorous_archosauria(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.3933363596);
   let mass = 0.0056582325 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0056582325 * {}", diameter.to_owned() + "²·³⁹³³³⁶³⁵⁹⁶" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous birds and crocodilians");
   }

// CARNIVOROUS FELIDAE

pub fn carnivorous_felidae(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 3.7646426827);
   let mass = 0.0001836115 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0001836115 * {}", diameter.to_owned() + "³·⁷⁶⁴⁶⁴²⁶⁸²⁷" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous felids");
   }

// CARNIVOROUS MAMMALIA

pub fn carnivorous_mammalia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.4614246462);
   let mass = 0.0041261855 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0041261855 * {}", diameter.to_owned() + "²·⁴⁶¹⁴²⁴⁶⁴⁶²" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous placentals and monotremes");
   }

// CARNIVOROUS SQUAMATA

pub fn carnivorous_squamata(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.3414629141);
   let mass = 0.0004822862 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0004822862 * {}", diameter.to_owned() + "²·³⁴¹⁴⁶²⁹¹⁴¹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous squamates");
   }

// HERBIVOROUS AFROTHERIA

pub fn herbivorous_afrotheria(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.7889164772);
   let mass = 0.0024866807 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0024866807 * {}", diameter.to_owned() + "²·⁷⁸⁸⁹¹⁶⁴⁷⁷²" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous afrotherians");
   }

// HERBIVOROUS AVES

pub fn herbivorous_aves(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.8100206869);
   let mass = 0.0078389131 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0078389131 * {}", diameter.to_owned() + "²·⁸¹⁰⁰²⁰⁶⁸⁶⁹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous birds");
   }

// HERBIVOROUS LAGOMORPHA

pub fn herbivorous_lagomorpha(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 1.7023616549);
   let mass = 0.0400923375 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0400923375 * {}", diameter.to_owned() + "¹·⁷⁰²³⁶¹⁶⁵⁴⁹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous lagomorphs");
   }

// HERBIVOROUS MARSUPIALIA

pub fn herbivorous_marsupialia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.2193247015);
   let mass = 0.0224440978 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0224440978 * {}", diameter.to_owned() + "²·²¹⁹³²⁴⁷⁰¹⁵" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous marsupials");
   }

// HERBIVOROUS REPTILIA

pub fn herbivorous_reptilia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.8272692414);
   let mass = 0.0076217107 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0076217107 * {}", diameter.to_owned() + "²·⁸²⁷²⁶⁹²⁴¹⁴" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous birds and turtles");
   }

// HERBIVOROUS RODENTIA

pub fn herbivorous_rodentia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.1269594421);
   let mass = 0.0287406644 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0287406644 * {}", diameter.to_owned() + "²·¹²⁶⁹⁵⁹⁴⁴²¹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant herbivorous rodents");
   }

// OMNIVOROUS AVES

pub fn omnivorous_aves(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 3.0157553176);
   let mass = 0.0014342026 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0014342026 * {}", diameter.to_owned() + "³·⁰¹⁵⁷⁵⁵³¹⁷⁶" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant omnivorous birds");
   }

// OMNIVOROUS RODENTIA

pub fn omnivorous_rodentia(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 1.6033593367);
   let mass = 0.0378726355 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0378726355 * {}", diameter.to_owned() + "¹·⁶⁰³³⁵⁹³³⁶⁷" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant omnivorous rodents");
   }

// UNSPECIFIED CANIDAE

pub fn unspecified_canidae(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.0396557894);
   let mass = 0.0205937247 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0205937247 * {}", diameter.to_owned() + "²·⁰³⁹⁶⁵⁵⁷⁸⁹⁴" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous and omnivorous canids");
   }

// UNSPECIFIED CARNIVORA

pub fn unspecified_carnivora(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.7691659092);
   let mass = 0.0018622336 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0018622336 * {}", diameter.to_owned() + "²·⁷⁶⁹¹⁶⁵⁹⁰⁹²" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous and omnivorous carnivorans");
   }

// UNSPECIFIED HERPESTIDAE

pub fn unspecified_herpestidae(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(1.0767216732, dcal as f32);
   let mass = 0.3503054969 * power;

   let str1 = diameter;
   let str2 = &str1.replace(".", "·");
   let str3 = &str2.replace("0", "⁰");
   let str4 = &str3.replace("1", "¹");
   let str5 = &str4.replace("2", "²");
   let str6 = &str5.replace("3", "³");
   let str7 = &str6.replace("4", "⁴");
   let str8 = &str7.replace("5", "⁵");
   let str9 = &str8.replace("6", "⁶");
   let str10 = &str9.replace("7", "⁷");
   let str11 = &str10.replace("8", "⁸");
   let dmpw = &str11.replace("9", "⁹");

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.3503054969 * 1.0767216732{}", dmpw.to_owned() + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous and omnivorous herpestids");
   }

// UNSPECIFIED MUSTELIDAE

pub fn unspecified_mustelidae(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.5225036146);
   let mass = 0.0038273352 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0038273352 * {}", diameter.to_owned() + "²·⁵²²⁵⁰³⁶¹⁴⁶" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous and omnivorous mustelids");
   }

// UNSPECIFIED TESTUDINES

pub fn unspecified_testudines(clear: &str, bright_yellow: &str, cyan: &str, diameter: &str, dcal: f32) {

   let power = f32::powf(dcal as f32, 2.1345692041);
   let mass = 0.0251303814 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", bright_yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", bright_yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", clear.to_owned());
   println!(" = 0.0251303814 * {}", diameter.to_owned() + "²·¹³⁴⁵⁶⁹²⁰⁴¹" + cyan);
   println!("Regression model based on:{}", bright_yellow.to_owned() + " extant carnivorous, omnivorous and herbivorous turtles");
   }
