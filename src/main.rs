use crate::config::app_config;
use also::Also;
use fltk::app;
use fltk::prelude::*;
use std::ops::Sub;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod audio;
mod config;
mod mainview;

fn main() {
    let cfg = app_config();
    let alert_path = Arc::new(
        cfg.audio_file
            .unwrap_or(crate::config::DEFAULT_AUDIO.to_string()),
    );

    let app = app::App::default();
    let view = Arc::new(Mutex::new(mainview::UserInterface::make_window(
        cfg.color_theme,
    )));

    // The number of cycles elapsed (cycle = work + break)
    let cycles = Arc::new(Mutex::new(0));

    // If the current (or last in case the timer does not run) is a work phase
    let work = Arc::new(Mutex::new(true));

    {
        let view_ref = Arc::clone(&view);
        let cycles = Arc::clone(&cycles);
        let work = Arc::clone(&work);

        view.lock().unwrap().next.set_callback(move |_| {
            view_ref.lock().unwrap().next.deactivate();

            let view_ref = Arc::clone(&view_ref);
            let cycles = Arc::clone(&cycles);
            let work = Arc::clone(&work);
            let alert_path = Arc::clone(&alert_path);

            thread::spawn(move || {
                let start = Instant::now();
                let duration = if *work.lock().unwrap() {
                    view_ref.lock().unwrap().set_work();
                    Duration::from_secs(cfg.work * 60)
                } else {
                    view_ref.lock().unwrap().set_break();
                    let break_mins = if *cycles.lock().unwrap() % cfg.long_break_after
                        == cfg.long_break_after - 1
                    {
                        cfg.long_break
                    } else {
                        cfg.short_break
                    };
                    Duration::from_secs(break_mins * 60)
                };
                let update_interval = Duration::new(0, 997000000);
                while start.elapsed().as_millis() < duration.as_millis() {
                    let remaining = duration.sub(start.elapsed());
                    view_ref.lock().unwrap().time.set_label(
                        format!(
                            "{:0>2}:{:0>2}",
                            remaining.as_secs() / 60,
                            remaining.as_secs() % 60
                        )
                        .as_str(),
                    );
                    app::awake();
                    sleep(update_interval);
                }
                if !*work.lock().unwrap() {
                    // A cycle is considered to be completed after the break
                    *cycles.lock().unwrap() += 1;
                }
                *work.lock().unwrap() ^= true;

                {
                    // Play sound to notify user
                    thread::spawn(move || audio::ring(&alert_path));
                }

                view_ref.lock().unwrap().lets(|mut view| {
                    view.next.activate();
                    view.set_idle();
                });
                app::awake();
            });
        });
    }

    app.run().unwrap();
}
