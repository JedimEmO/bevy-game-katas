use crate::MovementAction;
use bevy::prelude::*;

pub fn gamepad_input_system(
    mut event_sender: EventWriter<MovementAction>,
    gamepad_query: Query<&Gamepad>,
) {
    let mut direction = Vec2::ZERO;
    let Ok(gamepad) = gamepad_query.get_single() else {
        return;
    };

    if gamepad.just_pressed(GamepadButton::West) {
        event_sender.send(MovementAction::Attack);
    }

    if gamepad.pressed(GamepadButton::DPadRight) {
        direction.x = 1.;
    } else if gamepad.pressed(GamepadButton::DPadLeft) {
        direction.x = -1.;
    }

    if direction.length() > 0.1 {
        event_sender.send(MovementAction::Horizontal(direction));
    }

    if gamepad.pressed(GamepadButton::South) {
        event_sender.send(MovementAction::Jump);
    }

    if gamepad.just_released(GamepadButton::South) {
        event_sender.send(MovementAction::JumpAbort);
    }
}
