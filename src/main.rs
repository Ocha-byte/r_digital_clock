use std::time::Duration;

// Digital Clock
use chrono::{Datelike, FixedOffset, Timelike, Utc};
use eframe::egui;

fn main() {
    // GUI
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Simple Time App",
        native_options,
        Box::new(|cc| Ok(Box::new(TimeApp::new(cc)))),
    );
}

#[derive(Default)]
struct TimeApp {}

impl TimeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.system_theme();
        Self::default()
    }
}

impl eframe::App for TimeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let hour_secs = 3600;

            // Swiss time
            let ch_offset = FixedOffset::east_opt(hour_secs * 1).unwrap();
            let ch_dt = Utc::now()
                .with_timezone(&ch_offset)
                .with_nanosecond(0)
                .unwrap();
            let ch_time = ch_dt.time();
            let ch_date = ch_dt.date_naive().format("%d/%m/%Y");
            let ch_day = ch_dt.date_naive().weekday().to_string();
            let ch_time_str = ch_time.to_string();
            let ch_date_str = ch_date.to_string();

            // Brisbane time
            let au_offset = FixedOffset::east_opt(hour_secs * 10).unwrap();
            let au_dt = Utc::now()
                .with_timezone(&au_offset)
                .with_nanosecond(0)
                .unwrap();
            let au_time = au_dt.time();
            let au_date = au_dt.date_naive().format("%d/%m/%Y");
            let au_day = ch_dt.date_naive().weekday().to_string();
            let au_time_str = au_time.to_string();
            let au_date_str = au_date.to_string();

            ui.heading("Current Times (24hr)");

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Country");
                    ui.label("Current time: HH:MM:SS");
                    ui.label("Current date: Day, DD/MM/YY");
                })
            });

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Switzerland");
                    ui.label("Current time:");
                    ui.label(ch_time_str);
                    ui.label("Current date: ");
                    ui.label(ch_day);
                    ui.label(ch_date_str);
                })
            });

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Australia");
                    ui.label("Current time:");
                    ui.label(au_time_str);
                    ui.label("Current date: ");
                    ui.label(au_day);
                    ui.label(au_date_str);
                })
            });

            ui.add_space(10.);

            // Update GUI up to every second
            let dur = Duration::new(1, 0);
            egui::Context::request_repaint_after(ctx, dur);
        });
    }
}
