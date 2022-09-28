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
        let font_color = Self::font_color(theme.idle_color);

        let mut win = Window::default().with_size(170, 86).with_label("Azusa");
        win.set_color(theme.idle_color);
        win.end();
        win.show();

        let mut time = Frame::default()
            .with_size(150, 50)
            .with_pos(0, 0)
            .with_label("Welcome")
            .center_x(&win);
        time.set_label_font(Font::HelveticaBold);
        time.set_label_size(36);
        time.set_label_color(font_color);
        win.add(&time);

        let mut next = ReturnButton::default()
            .with_size(150, 25)
            .below_of(&time, 2)
            .with_label("Next");
        next.set_color(theme.idle_color);
        next.set_frame(FrameType::EngravedFrame);
        next.set_label_color(font_color);
        win.add(&next);

        Self { win, time, next, theme }
    }

    fn font_color(background: Color) -> Color {
        let (r, g, b) = background.to_rgb();

        let brightness = (r as f32 + g as f32 + b as f32) / (3.0 * 255.0);

        if brightness > 0.5 {
            Color::Black
        } else {
            Color::White
        }
    }
    
    pub fn set_color(&mut self, c: Color) {
        let font_color = Self::font_color(c);
        self.time.set_label_color(font_color);
        self.next.set_label_color(font_color);
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
