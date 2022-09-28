use fltk::button::ReturnButton;
use fltk::enums::{Color, Font, FrameType};
use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::window::Window;

const BREAK_COLOR: Color = Color::from_hex(0x3f7cac);
const IDLE_COLOR: Color = Color::from_hex(0x993955);
const WORK_COLOR: Color = Color::from_hex(0x69995d);

pub struct UserInterface {
    pub win: Window,
    pub time: Frame,
    pub next: ReturnButton,
}

impl UserInterface {
    pub fn make_window() -> Self {
        let mut win = Window::default().with_size(170, 130).with_label("Azusa");
        win.set_color(IDLE_COLOR);
        win.end();
        win.show();

        let mut time = Frame::default()
            .with_size(150, 70)
            .with_pos(0, 11)
            .with_label("Welcome")
            .center_x(&win);
        time.set_label_font(Font::HelveticaBold);
        time.set_label_size(36);
        time.set_label_color(Color::White);
        win.add(&time);

        let mut next = ReturnButton::default()
            .with_size(150, 25)
            .below_of(&time, 11)
            .with_label("Next");
        next.set_color(IDLE_COLOR);
        next.set_frame(FrameType::EngravedFrame);
        next.set_label_color(Color::White);
        win.add(&next);

        Self { win, time, next }
    }

    pub fn set_break(&mut self) {
        self.win.set_color(BREAK_COLOR);
        self.next.set_color(BREAK_COLOR);
        self.win.redraw();
    }

    pub fn set_idle(&mut self) {
        self.win.set_color(IDLE_COLOR);
        self.next.set_color(IDLE_COLOR);
        self.win.redraw();
    }

    pub fn set_work(&mut self) {
        self.win.set_color(WORK_COLOR);
        self.next.set_color(WORK_COLOR);
        self.win.redraw();
    }
}
