mod handler;
mod morpion;

fn main() {
    if let Some(arg) = std::env::args().nth(1)
        && arg == "cli"
    {
        let mut mpn = morpion::Morpion::new();
        mpn.gameloop();
    } else {
        let mut mpn = handler::Handler::new();
        mpn.gameloop();
    }
}
