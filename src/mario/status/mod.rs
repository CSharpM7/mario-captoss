mod cappy;
mod capjump;
mod capdive;
mod specials;
mod appeal;

pub fn install() {
    cappy::install();
    capjump::install();
    capdive::install();
    specials::install();
    appeal::install();
}