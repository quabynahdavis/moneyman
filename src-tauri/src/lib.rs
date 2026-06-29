mod commands;
mod db;
mod models;

use db::Database;
use std::path::PathBuf;
use tauri::Manager;

fn app_data_dir(_app: &tauri::AppHandle) -> PathBuf {
    let home = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("moneyman")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app_data_dir(app.handle());
            let database = Database::new(data_dir)
                .expect("failed to initialize database");
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::accounts::list_accounts,
            commands::accounts::get_account_tree,
            commands::accounts::create_account,
            commands::accounts::update_account,
            commands::accounts::delete_account,
            commands::transactions::post_transaction,
            commands::transactions::list_transactions,
            commands::transactions::void_transaction,
            commands::transactions::get_dashboard_summary,
            commands::reconciliation::start_reconciliation,
            commands::reconciliation::get_reconciliation_data,
            commands::reconciliation::toggle_split_reconcile_state,
            commands::reconciliation::finalize_reconciliation,
            commands::reconciliation::check_reconciled_split,
            commands::import::list_import_profiles,
            commands::import::save_import_profile,
            commands::import::delete_import_profile,
            commands::import::preview_csv_import,
            commands::import::commit_import,
            commands::recurring::list_recurring_transactions,
            commands::recurring::create_recurring_transaction,
            commands::recurring::update_recurring_transaction,
            commands::recurring::delete_recurring_transaction,
            commands::recurring::execute_recurring_transaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
