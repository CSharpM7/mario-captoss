mod cappy;
mod specials;
mod capjump;

pub fn install() {
    cappy::install();
    capjump::install();
    specials::install();
}