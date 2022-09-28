use fltk::button::ReturnButton;
use fltk::enums::{Color, Font, FrameType};
use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::window::Window;

pub struct ColorTheme {
    pub break_color: Color,
    pub idle_color: Color,
    pub work_color: Color,
}

pub struct UserInterface {
    pub win: Window,
    pub time: Frame,
    pub next: ReturnButton,
    pub theme: ColorTheme
}

impl UserInterface {
    pub fn make_window(theme: ColorTheme) -> Self {
        let mut win = Window::default().with_size(170, 130).with_label("Azusa");
        win.set_color(theme.idle_color);
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
        next.set_color(theme.idle_color);
        next.set_frame(FrameType::EngravedFrame);
        next.set_label_color(Color::White);
        win.add(&next);

        Self { win, time, next, theme }
    }
    
    pub fn set_color(&mut self, c: Color) {
        self.win.set_color(c);
        self.next.set_color(c);
        self.win.redraw();
	}

    pub fn set_break(&mut self) {
        self.set_color(self.theme.break_color);
    }

    pub fn set_idle(&mut self) {
        self.set_color(self.theme.idle_color);
    }

    pub fn set_work(&mut self) {
        self.set_color(self.theme.work_color);
    }
}
