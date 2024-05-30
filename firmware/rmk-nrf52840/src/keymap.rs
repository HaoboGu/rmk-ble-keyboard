use crate::{COL, NUM_LAYER, ROW};
use rmk::action::KeyAction;
use rmk::{a, k, layer, mo};

#[rustfmt::skip]
pub static KEYMAP: [[[KeyAction; COL]; ROW]; NUM_LAYER] = [
    layer!([
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)]
    ]),
    layer!([
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)]
    ]),

    layer!([
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)],
        [k!(AudioVolUp), k!(B), k!(AudioVolDown), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A), k!(A)]
    ]),
];
