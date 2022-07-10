// COPROSIZE VERSION 1.0.0 / THE MIT LICENSE (MIT) © 2022 PIOTR BAJDEK

// LIBRARY

pub mod colors;
use colors::*;

// CITATION

pub fn citation() {
    println!("Bajdek, P., 2022. coprosize (version 1.0.0). [computer software] https://github.com/piotrbajdek/coprosize");
}

// CARNIVOROUS AMNIOTA

pub fn carnivorous_amniota(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.4839053017);
    let mass = 0.0030393430 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0030393430 * {}²·⁴⁸³⁹⁰⁵³⁰¹⁷", diameter);
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous synapsids and sauropsids")
    );
}

// CARNIVOROUS AMPHIBIA

pub fn carnivorous_amphibia(diameter: f32) {
    let dcal: f32 = diameter;
    let power1 = f32::powi(dcal, 3);
    let power2 = f32::powi(dcal, 2);
    let mass = (0.0004064349 * power1) - (0.0041616775 * power2) + (0.0147514015 * dcal) - 0.0122201640;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0004064349 * {diameter}³ - 0.0041616775 * {diameter}² + 0.0147514015 * {diameter} - 0.0122201640");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant carnivorous batrachians"));
}

// CARNIVOROUS ARCHOSAURIA

pub fn carnivorous_archosauria(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.3933363596);
    let mass = 0.0056582325 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0056582325 * {diameter}²·³⁹³³³⁶³⁵⁹⁶");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous birds and crocodilians")
    );
}

// CARNIVOROUS FELIDAE

pub fn carnivorous_felidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 3.7646426827);
    let mass = 0.0001836115 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0001836115 * {diameter}³·⁷⁶⁴⁶⁴²⁶⁸²⁷");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant carnivorous felids"));
}

// CARNIVOROUS MAMMALIA

pub fn carnivorous_mammalia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.6010376216);
    let mass = 0.0029493394 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0029493394 * {diameter}²·⁶⁰¹⁰³⁷⁶²¹⁶");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous placentals and monotremes")
    );
}

// CARNIVOROUS SQUAMATA

pub fn carnivorous_squamata(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.3414629141);
    let mass = 0.0004822862 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));

    println!(" = 0.0004822862 * {diameter}²·³⁴¹⁴⁶²⁹¹⁴¹");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant carnivorous squamates"));
}

// CARNIVOROUS TETRAPODA

pub fn carnivorous_tetrapoda(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.6157600696);
    let mass = 0.0017879339 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0017879339 * {diameter}²·⁶¹⁵⁷⁶⁰⁰⁶⁹⁶");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous amphibians and amniotes")
    );
}

// HERBIVOROUS AFROTHERIA

pub fn herbivorous_afrotheria(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.7889164772);
    let mass = 0.0024866807 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0024866807 * {diameter}²·⁷⁸⁸⁹¹⁶⁴⁷⁷²");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous afrotherians"));
}

// HERBIVOROUS AMNIOTA

pub fn herbivorous_amniota(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.2123196655);
    let mass = 0.0733160890 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0733160890 * {diameter}²·²¹²³¹⁹⁶⁶⁵⁵");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant herbivorous synapsids and sauropsids")
    );
}

// HERBIVOROUS ARTIODACTYLA

pub fn herbivorous_artiodactyla(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.3109191553);
    let mass = 0.4817803510 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.4817803510 * {diameter}²·³¹⁰⁹¹⁹¹⁵⁵³");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous artiodactyls"));
}

// HERBIVOROUS AVES

pub fn herbivorous_aves(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.8100206869);
    let mass = 0.0078389131 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0078389131 * {diameter}²·⁸¹⁰⁰²⁰⁶⁸⁶⁹");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous birds"));
}

// HERBIVOROUS BOVIDAE

pub fn herbivorous_bovidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.1885588774);
    let mass = 0.6525042837 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.6525042837 * {diameter}²·¹⁸⁸⁵⁵⁸⁸⁷⁷⁴");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous bovids"));
}

// HERBIVOROUS CERVIDAE

pub fn herbivorous_cervidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 3.4595227094);
    let mass = 0.0209279246 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0209279246 * {diameter}³·⁴⁵⁹⁵²²⁷⁰⁹⁴");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous cervids"));
}

// HERBIVOROUS LAGOMORPHA

pub fn herbivorous_lagomorpha(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.3225288441);
    let mass = 0.0083097861 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0083097861 * {diameter}²·³²²⁵²⁸⁸⁴⁴¹");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous lagomorphs"));
}

// HERBIVOROUS MAMMALIA

pub fn herbivorous_mammalia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.1815554929);
    let mass = 0.0859205551 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0859205551 * {diameter}²·¹⁸¹⁵⁵⁵⁴⁹²⁹");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant herbivorous placentals and marsupials")
    );
}

// HERBIVOROUS MARSUPIALIA

pub fn herbivorous_marsupialia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.2193247015);
    let mass = 0.0224440978 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0224440978 * {diameter}²·²¹⁹³²⁴⁷⁰¹⁵");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous marsupials"));
}

// HERBIVOROUS PLACENTALIA

pub fn herbivorous_placentalia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.2424443852);
    let mass = 0.0879148644 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0879148644 * {diameter}²·²⁴²⁴⁴⁴³⁸⁵²");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous placentals"));
}

// HERBIVOROUS REPTILIA

pub fn herbivorous_reptilia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.8272692414);
    let mass = 0.0076217107 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0076217107 * {diameter}²·⁸²⁷²⁶⁹²⁴¹⁴");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous birds and turtles"));
}

// HERBIVOROUS RODENTIA

pub fn herbivorous_rodentia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.2974489103);
    let mass = 0.0196247359 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0196247359 * {diameter}²·²⁹⁷⁴⁴⁸⁹¹⁰³");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant herbivorous rodents"));
}

// OMNIVOROUS AMNIOTA

pub fn omnivorous_amniota(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.2146716042);
    let mass = 0.0128321404 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0128321404 * {diameter}²·²¹⁴⁶⁷¹⁶⁰⁴²");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant omnivorous synapsids and sauropsids")
    );
}

// OMNIVOROUS ARTIODACTYLA

pub fn omnivorous_artiodactyla(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 1.0887395414);
    let mass = 1.2390608405 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 1.2390608405 * {diameter}¹·⁰⁸⁸⁷³⁹⁵⁴¹⁴");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant omnivorous artiodactyls"));
}

// OMNIVOROUS AVES

pub fn omnivorous_aves(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 3.0157553176);
    let mass = 0.0014342026 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0014342026 * {diameter}³·⁰¹⁵⁷⁵⁵³¹⁷⁶");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant omnivorous birds"));
}

// OMNIVOROUS MAMMALIA

pub fn omnivorous_mammalia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.0562969771);
    let mass = 0.0223667325 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0223667325 * {diameter}²·⁰⁵⁶²⁹⁶⁹⁷⁷¹");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant omnivorous placentals and marsupials")
    );
}

// OMNIVOROUS RODENTIA

pub fn omnivorous_rodentia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 1.6780088221);
    let mass = 0.0331020696 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0331020696 * {diameter}¹·⁶⁷⁸⁰⁰⁸⁸²²¹");
    println!("{}{}", colorize(CYAN, "Regression model based on: "), colorize(BRIGHT_YELLOW, "extant omnivorous rodents"));
}

// UNSPECIFIED CANIDAE

pub fn unspecified_canidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.0396557894);
    let mass = 0.0205937247 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0205937247 * {diameter}²·⁰³⁹⁶⁵⁵⁷⁸⁹⁴");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous and omnivorous canids")
    );
}

// UNSPECIFIED CARNIVORA

pub fn unspecified_carnivora(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.8590076963);
    let mass = 0.0015578285 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0015578285 * {diameter}²·⁸⁵⁹⁰⁰⁷⁶⁹⁶³");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous and omnivorous carnivorans")
    );
}

// UNSPECIFIED CRICETIDAE

pub fn unspecified_cricetidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(1.8766049284, dcal);
    let mass = 0.0180745732 * power;

    let dmpw = superscript_number(diameter);

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0180745732 * 1.8766049284{dmpw}");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant herbivorous and omnivorous cricetids")
    );
}

// UNSPECIFIED HERPESTIDAE

pub fn unspecified_herpestidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(1.0767216732, dcal);
    let mass = 0.3503054969 * power;

    let dmpw = superscript_number(diameter);

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.3503054969 * 1.0767216732{}", dmpw);
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous and omnivorous herpestids")
    );
}

// UNSPECIFIED MARSUPIALIA

pub fn unspecified_marsupialia(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.1922202080);
    let mass = 0.0231733783 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0231733783 * {diameter}²·¹⁹²²²⁰²⁰⁸⁰");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant herbivorous and omnivorous marsupials")
    );
}

// UNSPECIFIED MUSTELIDAE

pub fn unspecified_mustelidae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.5225036146);
    let mass = 0.0038273352 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0038273352 * {diameter}²·⁵²²⁵⁰³⁶¹⁴⁶");
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous and omnivorous mustelids")
    );
}

// UNSPECIFIED SCIURIDAE

pub fn unspecified_sciuridae(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(1.7069919381, dcal);
    let mass = 0.0384600726 * power;

    let dmpw = superscript_number(diameter);

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0384600726 * 1.7069919381{}", dmpw);
    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant herbivorous and omnivorous sciurids")
    );
}

// UNSPECIFIED TESTUDINES

pub fn unspecified_testudines(diameter: f32) {
    let dcal: f32 = diameter;
    let power = f32::powf(dcal, 2.1345692041);
    let mass = 0.0251303814 * power;

    println!("{}{}", colorize(CYAN, "Coprolite diameter: "), colorize(BRIGHT_YELLOW, &format!("{diameter} mm")));
    print!("{}{}", colorize(CYAN, "Producer's body mass: "), colorize(BRIGHT_YELLOW, &format!("{mass:.3} kg")));
    println!(" = 0.0251303814 * {diameter}²·¹³⁴⁵⁶⁹²⁰⁴¹");

    println!(
        "{}{}",
        colorize(CYAN, "Regression model based on: "),
        colorize(BRIGHT_YELLOW, "extant carnivorous, herbivorous and omnivorous turtles")
    );
}

fn superscript_number(num: f32) -> String {
    num.to_string()
        .chars()
        .map(|c| match c {
            '.' => '·',
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            x => x,
        })
        .collect()
}
