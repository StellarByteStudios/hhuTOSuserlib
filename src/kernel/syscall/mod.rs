pub mod user_api;

// TODO: Das vielleicht anders lösen, indem direkt ein Format an den Kernel übergeben wird
#[derive(Copy, Clone)]
pub enum SongID {
    super_mario = 0,
    starwars_imperial = 1,
    tetris = 2,
    aerodynamic = 3,
    nyancat = 4,
    alle_meine_entchen = 5,
    intro = 6,
    doom = 7,
}
