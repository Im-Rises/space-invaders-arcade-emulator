use crate::si_arcade::InputsOutputs;

pub enum GameInput {
    Coin,
    Player1Start,
    Player2Start,
    Left,
    Right,
    Shot,
    Dip3,
    Dip5,
    Dip6,
    Dip7,
}

pub fn update_input(inputs_outputs: &mut InputsOutputs, input_index: GameInput, value: bool) {
    match input_index {
        GameInput::Coin => inputs_outputs.coin = value,
        GameInput::Player1Start => inputs_outputs.player1.start = value,
        GameInput::Player2Start => inputs_outputs.player2.start = value,
        GameInput::Left => inputs_outputs.player1.left = value,
        GameInput::Right => inputs_outputs.player1.right = value,
        GameInput::Shot => inputs_outputs.player1.shot = value,
        GameInput::Dip3 => inputs_outputs.dip3 = value,
        GameInput::Dip5 => inputs_outputs.dip5 = value,
        GameInput::Dip6 => inputs_outputs.dip6 = value,
        GameInput::Dip7 => inputs_outputs.dip7 = value,
    }
}
