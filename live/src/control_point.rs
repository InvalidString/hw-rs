use raylib_ffi::colors;

use crate::{graphics::Pos2, Ui};


enum Interaction{
    Hovered,
    Dragging,
    None,
}

struct State{
    interaction: Interaction,
    pos: Pos2,
}


fn view(ui: &mut Ui, state: State) {
    let color = match state.interaction{
        Interaction::Hovered => colors::WHITE,
        Interaction::Dragging |
        Interaction::None => colors::GRAY,
    };

    let radius = 3.0;
    ui.draw_circle(state.pos, radius, color);
}
