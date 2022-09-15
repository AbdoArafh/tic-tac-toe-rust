use std::collections::HashMap;

use fltk::{
    app::{self, App, Receiver, Sender},
    button::Button,
    enums::{Color, FrameType, Shortcut},
    group::{Pack, PackType},
    prelude::*,
    window::Window,
};

use crate::{Cell, Player};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Move(Cell),
}

pub struct UI {
    pub buttons: Vec<CellButton>,
    pub r: Receiver<Message>,
    pub s: Sender<Message>,
    pub app: App,
}

pub struct CellButton {
    pub button: Button,
    pub coords: Cell,
}

impl CellButton {
    pub fn new(coords: Cell, size: i32) -> CellButton {
        let buttons_map: HashMap<crate::Cell, char> = HashMap::from([
            ((0, 0), '7'),
            ((0, 1), '8'),
            ((0, 2), '9'),
            ((1, 0), '4'),
            ((1, 1), '5'),
            ((1, 2), '6'),
            ((2, 0), '1'),
            ((2, 1), '2'),
            ((2, 2), '3'),
        ]);

        let mut button = Button::default().with_label(" ").with_size(size, size);
        button.set_label_size(size / 2);
        button.set_label_color(Color::White);
        button.set_frame(FrameType::GleamThinDownBox);
        button.set_color(Color::from_u32(0xf0ad4e));
        button.set_shortcut(Shortcut::from_char(*buttons_map.get(&coords).unwrap()));
        CellButton { button, coords }
    }

    pub fn set_player(&mut self, player: Player) {
        self.button.set_color(match player {
            Player::X => Color::from_u32(0x5cb85c),
            Player::O => Color::from_u32(0xd9534f),
            Player::EMPTY => Color::from_u32(0xf0ad4e),
        });

        self.button.set_label(match player {
            Player::X => "X",
            Player::O => "O",
            Player::EMPTY => " ",
        });
    }
}

impl UI {
    pub fn new(window_label: &str) -> UI {
        let app = App::default().with_scheme(app::AppScheme::Gtk);
        app::background(0xf0, 0xad, 0x4e);

        let mut buttons: Vec<CellButton> = Vec::new();
        let (s, r) = app::channel::<Message>();

        let win_w = 480;
        let win_h = 480;
        let button_size = std::cmp::min(win_w, win_h) / 3;
        let mut window = Window::default()
            .with_size(win_w, win_h)
            .with_label(window_label);

        let vpack = Pack::default()
            .with_size(button_size * 3, button_size * 3)
            .center_of(&window);

        for i in 0..3 {
            let mut hpack = Pack::default().with_size(win_w, button_size);
            for j in 0..3 {
                let cell_button = CellButton::new((i, j), button_size);
                buttons.push(cell_button);
            }
            hpack.end();
            hpack.set_type(PackType::Horizontal);
        }

        vpack.end();

        window.end();
        window.show();

        UI { buttons, r, s, app }
    }
}
