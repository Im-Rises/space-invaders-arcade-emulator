pub mod si_arcade;

fn main() {
    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new("romsPaths");
    space_invaders_arcade.start();
}
