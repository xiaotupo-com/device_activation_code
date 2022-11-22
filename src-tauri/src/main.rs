#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_activation_code(device_number: &str) -> String {
    // 计算公式 激活算法：低2位 * 2 + 123 + 高2位 * 100
    let lb: i32 = device_number[2..].parse().unwrap();
    let hb: i32 = device_number[..2].parse().unwrap();

    let result = (lb * 2) + (hb * 100);

    result.to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_activation_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
