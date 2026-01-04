mod morpion;
mod handler;

fn main() {
    // let mut mpn = morpion::Morpion::new();
    // mpn.gameloop();
    let mut mpn = handler::Handler::new();
    mpn.gameloop();
}
