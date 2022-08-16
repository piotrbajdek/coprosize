// COPROSIZE VERSION 1.0.1 / MIT LICENSE © 2022 PIOTR BAJDEK

// LIBRARY

// CITATION

pub fn citation() {

   println!("Bajdek, P., 2022. coprosize (version 1.0.1). [computer software] https://github.com/piotrbajdek/coprosize");
   }

// CARNIVOROUS AMNIOTA

pub fn carnivorous_amniota(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.4839053017);
   let mass = 0.0030393430 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0030393430 * {}", diameter.to_owned() + "²·⁴⁸³⁹⁰⁵³⁰¹⁷" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous synapsids and sauropsids" + reset);
   }

// CARNIVOROUS AMPHIBIA

pub fn carnivorous_amphibia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power1 = f32::powi(dcal as f32, 3);
   let power2 = f32::powi(dcal as f32, 2);
   let mass = (0.0004064349 * power1) - (0.0041616775 * power2) + (0.0147514015 * dcal) - 0.0122201640;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0004064349 * {}", diameter.to_owned() + "³ - 0.0041616775 * " + diameter + "² + 0.0147514015 * " + diameter + " - 0.0122201640" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous batrachians" + reset);
   }

// CARNIVOROUS ARCHOSAURIA

pub fn carnivorous_archosauria(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.3933363596);
   let mass = 0.0056582325 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0056582325 * {}", diameter.to_owned() + "²·³⁹³³³⁶³⁵⁹⁶" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous birds and crocodilians" + reset);
   }

// CARNIVOROUS FELIDAE

pub fn carnivorous_felidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 3.7646426827);
   let mass = 0.0001836115 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0001836115 * {}", diameter.to_owned() + "³·⁷⁶⁴⁶⁴²⁶⁸²⁷" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous felids" + reset);
   }

// CARNIVOROUS MAMMALIA

pub fn carnivorous_mammalia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.6010376216);
   let mass = 0.0029493394 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0029493394 * {}", diameter.to_owned() + "²·⁶⁰¹⁰³⁷⁶²¹⁶" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous placentals and monotremes" + reset);
   }

// CARNIVOROUS SQUAMATA

pub fn carnivorous_squamata(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.3414629141);
   let mass = 0.0004822862 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0004822862 * {}", diameter.to_owned() + "²·³⁴¹⁴⁶²⁹¹⁴¹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous squamates" + reset);
   }

// CARNIVOROUS TETRAPODA

pub fn carnivorous_tetrapoda(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.6157600696);
   let mass = 0.0017879339 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0017879339 * {}", diameter.to_owned() + "²·⁶¹⁵⁷⁶⁰⁰⁶⁹⁶" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous amphibians and amniotes" + reset);
   }

// HERBIVOROUS AFROTHERIA

pub fn herbivorous_afrotheria(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.7889164772);
   let mass = 0.0024866807 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0024866807 * {}", diameter.to_owned() + "²·⁷⁸⁸⁹¹⁶⁴⁷⁷²" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous afrotherians" + reset);
   }

// HERBIVOROUS AMNIOTA

pub fn herbivorous_amniota(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.2123196655);
   let mass = 0.0733160890 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0733160890 * {}", diameter.to_owned() + "²·²¹²³¹⁹⁶⁶⁵⁵" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous synapsids and sauropsids" + reset);
   }

// HERBIVOROUS ARTIODACTYLA

pub fn herbivorous_artiodactyla(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.3109191553);
   let mass = 0.4817803510 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.4817803510 * {}", diameter.to_owned() + "²·³¹⁰⁹¹⁹¹⁵⁵³" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous artiodactyls" + reset);
   }

// HERBIVOROUS AVES

pub fn herbivorous_aves(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.8100206869);
   let mass = 0.0078389131 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0078389131 * {}", diameter.to_owned() + "²·⁸¹⁰⁰²⁰⁶⁸⁶⁹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous birds" + reset);
   }

// HERBIVOROUS BOVIDAE

pub fn herbivorous_bovidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.1885588774);
   let mass = 0.6525042837 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.6525042837 * {}", diameter.to_owned() + "²·¹⁸⁸⁵⁵⁸⁸⁷⁷⁴" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous bovids" + reset);
   }

// HERBIVOROUS CERVIDAE

pub fn herbivorous_cervidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 3.4595227094);
   let mass = 0.0209279246 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0209279246 * {}", diameter.to_owned() + "³·⁴⁵⁹⁵²²⁷⁰⁹⁴" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous cervids" + reset);
   }

// HERBIVOROUS LAGOMORPHA

pub fn herbivorous_lagomorpha(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.3225288441);
   let mass = 0.0083097861 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0083097861 * {}", diameter.to_owned() + "²·³²²⁵²⁸⁸⁴⁴¹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous lagomorphs" + reset);
   }

// HERBIVOROUS MAMMALIA

pub fn herbivorous_mammalia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.1815554929);
   let mass = 0.0859205551 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0859205551 * {}", diameter.to_owned() + "²·¹⁸¹⁵⁵⁵⁴⁹²⁹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous placentals and marsupials" + reset);
   }

// HERBIVOROUS MARSUPIALIA

pub fn herbivorous_marsupialia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.2193247015);
   let mass = 0.0224440978 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0224440978 * {}", diameter.to_owned() + "²·²¹⁹³²⁴⁷⁰¹⁵" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous marsupials" + reset);
   }

// HERBIVOROUS PLACENTALIA

pub fn herbivorous_placentalia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.2424443852);
   let mass = 0.0879148644 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0879148644 * {}", diameter.to_owned() + "²·²⁴²⁴⁴⁴³⁸⁵²" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous placentals" + reset);
   }

// HERBIVOROUS REPTILIA

pub fn herbivorous_reptilia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.8272692414);
   let mass = 0.0076217107 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0076217107 * {}", diameter.to_owned() + "²·⁸²⁷²⁶⁹²⁴¹⁴" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous birds and turtles" + reset);
   }

// HERBIVOROUS RODENTIA

pub fn herbivorous_rodentia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.2974489103);
   let mass = 0.0196247359 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0196247359 * {}", diameter.to_owned() + "²·²⁹⁷⁴⁴⁸⁹¹⁰³" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous rodents" + reset);
   }

// OMNIVOROUS AMNIOTA

pub fn omnivorous_amniota(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.2146716042);
   let mass = 0.0128321404 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0128321404 * {}", diameter.to_owned() + "²·²¹⁴⁶⁷¹⁶⁰⁴²" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant omnivorous synapsids and sauropsids" + reset);
   }

// OMNIVOROUS ARTIODACTYLA

pub fn omnivorous_artiodactyla(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 1.0887395414);
   let mass = 1.2390608405 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 1.2390608405 * {}", diameter.to_owned() + "¹·⁰⁸⁸⁷³⁹⁵⁴¹⁴" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant omnivorous artiodactyls" + reset);
   }

// OMNIVOROUS AVES

pub fn omnivorous_aves(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 3.0157553176);
   let mass = 0.0014342026 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0014342026 * {}", diameter.to_owned() + "³·⁰¹⁵⁷⁵⁵³¹⁷⁶" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant omnivorous birds" + reset);
   }

// OMNIVOROUS MAMMALIA

pub fn omnivorous_mammalia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.0562969771);
   let mass = 0.0223667325 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0223667325 * {}", diameter.to_owned() + "²·⁰⁵⁶²⁹⁶⁹⁷⁷¹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant omnivorous placentals and marsupials" + reset);
   }

// OMNIVOROUS RODENTIA

pub fn omnivorous_rodentia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 1.6780088221);
   let mass = 0.0331020696 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0331020696 * {}", diameter.to_owned() + "¹·⁶⁷⁸⁰⁰⁸⁸²²¹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant omnivorous rodents" + reset);
   }

// UNSPECIFIED CANIDAE

pub fn unspecified_canidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.0396557894);
   let mass = 0.0205937247 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0205937247 * {}", diameter.to_owned() + "²·⁰³⁹⁶⁵⁵⁷⁸⁹⁴" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous and omnivorous canids" + reset);
   }

// UNSPECIFIED CARNIVORA

pub fn unspecified_carnivora(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.8590076963);
   let mass = 0.0015578285 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0015578285 * {}", diameter.to_owned() + "²·⁸⁵⁹⁰⁰⁷⁶⁹⁶³" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous and omnivorous carnivorans" + reset);
   }

// UNSPECIFIED CRICETIDAE

pub fn unspecified_cricetidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(1.8766049284, dcal as f32);
   let mass = 0.0180745732 * power;

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
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0180745732 * 1.8766049284{}", dmpw.to_owned() + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous and omnivorous cricetids" + reset);
   }

// UNSPECIFIED HERPESTIDAE

pub fn unspecified_herpestidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
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
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.3503054969 * 1.0767216732{}", dmpw.to_owned() + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous and omnivorous herpestids" + reset);
   }

// UNSPECIFIED MARSUPIALIA

pub fn unspecified_marsupialia(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.1922202080);
   let mass = 0.0231733783 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0231733783 * {}", diameter.to_owned() + "²·¹⁹²²²⁰²⁰⁸⁰" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous and omnivorous marsupials" + reset);
   }

// UNSPECIFIED MUSTELIDAE

pub fn unspecified_mustelidae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.5225036146);
   let mass = 0.0038273352 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0038273352 * {}", diameter.to_owned() + "²·⁵²²⁵⁰³⁶¹⁴⁶" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous and omnivorous mustelids" + reset);
   }

// UNSPECIFIED SCIURIDAE

pub fn unspecified_sciuridae(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(1.7069919381, dcal as f32);
   let mass = 0.0384600726 * power;

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
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0384600726 * 1.7069919381{}", dmpw.to_owned() + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant herbivorous and omnivorous sciurids" + reset);
   }

// UNSPECIFIED TESTUDINES

pub fn unspecified_testudines(diameter: &str) {

   let reset = "\x1b[0m"; // reset the font color
   let red = "\x1b[31m";
   let yellow = "\x1b[93m";
   let cyan = "\x1b[36m";

   let dcal: f32 = diameter.parse().expect(&(red.to_owned() + "Incorrect diameter value! Program only processes numbers!" + reset));
   let power = f32::powf(dcal as f32, 2.1345692041);
   let mass = 0.0251303814 * power;

   print!("{}", cyan);
   println!("Coprolite diameter: {}", yellow.to_owned() + diameter + " mm" + cyan);
   print!("Producer's body mass: {}", yellow.to_owned());
   print!("{:.3}", mass);
   print!(" kg{}", reset.to_owned());
   println!(" = 0.0251303814 * {}", diameter.to_owned() + "²·¹³⁴⁵⁶⁹²⁰⁴¹" + cyan);
   println!("Regression model based on:{}", yellow.to_owned() + " extant carnivorous, herbivorous and omnivorous turtles" + reset);
   }
