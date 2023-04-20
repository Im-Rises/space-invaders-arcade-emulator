mod binary_lib;
mod si_arcade;
mod my_webgl2;

fn main() {
    println!("+--------------------------------------+");
    println!("|      Space Invaders Arcade Game      |");
    println!("|--------------------------------------|");
    println!("|       C to insert a coin             |");
    println!("|       1 to start 1 player mode       |");
    println!("|       2 to start 2 player mode       |");
    println!("|       → to move P1 left              |");
    println!("|       ← to move P1 right             |");
    println!("|       ↑ to shoot with P1             |");
    println!("|       S to move P2 left              |");
    println!("|       F to move P2 right             |");
    println!("|       E to shoot with P2             |");
    println!("|       K to get 1 extra life          |");
    println!("|       L to get 2 extra lives         |");
    println!("|       M extra ship at 1000 points    |");
    println!("|                                      |");
    println!("|      Prepare for the invasion!       |");
    println!("+--------------------------------------+");

    println!("\nClick the Space Invaders Window to play.");
    println!("\nLogs:");

    let mut space_invaders_arcade = si_arcade::SpaceInvadersArcade::new();
    space_invaders_arcade.start();
}
