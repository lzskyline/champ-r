use std::sync::{Arc, Mutex, RwLock};

use eframe::egui;
use lcu::{cmd::CommandLineOutput, task};

pub mod config;
mod toogle_ui;
pub mod ui;

pub async fn run() -> Result<(), eframe::Error> {
    // Log to stderr (if you run with `RUST_LOG=debug`).
    env_logger::init();

    let lcu_auth = Arc::new(RwLock::new(CommandLineOutput::default()));
    let lcu_auth_ui = lcu_auth.clone();
    let lcu_auth_task = lcu_auth.clone();

    let ui_cc: Arc<Mutex<Option<egui::Context>>> = Arc::new(Mutex::new(None));
    let ui_cc_clone = ui_cc.clone();
    let champion_id = Arc::new(RwLock::new(None));
    let champion_id_ui = champion_id.clone();

    let random_mode = Arc::new(Mutex::new(false));
    let random_mode_ui = random_mode.clone();

    let watch_task_handle = tokio::spawn(async move {
        task::watch_auth_and_champion(ui_cc, lcu_auth_task, champion_id, random_mode).await;
    });
    let lcu_task_handle = Some(watch_task_handle.abort_handle());

    let main_win_opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ChampR",
        main_win_opts,
        Box::new(move |cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            let app_data = ui::SourceApp::new(
                lcu_auth_ui.clone(),
                lcu_task_handle,
                ui_cc_clone,
                champion_id_ui,
                random_mode_ui,
            );
            Box::new(app_data)
        }),
    )?;

    Ok(())
}