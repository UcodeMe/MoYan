use serde::{Deserialize, Serialize};

/// 剪贴板条目（与 shared/types.ts 的 ClipboardItem 对齐）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String, // "text" | "image"
    pub content: String,
    pub hash: String,
    pub pinned: bool,
    pub created_at: u64,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// 应用设置（与 shared/types.ts 的 AppSettings 对齐）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub storage_dir: String,
    pub expiration_days: u32,
    pub clipboard_hotkey: String,
    pub screenshot_hotkey: String,
    pub launch_at_login: bool,
}

/// settings_update 的增量补丁（对应 Partial<AppSettings>）
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsPatch {
    pub storage_dir: Option<String>,
    pub expiration_days: Option<u32>,
    pub clipboard_hotkey: Option<String>,
    pub screenshot_hotkey: Option<String>,
    pub launch_at_login: Option<bool>,
}

/// 截图编辑器初始化数据
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotInit {
    pub data_url: String,
    pub width: u32,
    pub height: u32,
}

/// 截图完成参数（对应 CompleteScreenshotOptions）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteScreenshotOptions {
    pub data_url: String,
    #[allow(dead_code)]
    pub text: String,
    #[allow(dead_code)]
    pub save_to_file: bool,
}

/// 双击粘贴参数（对应 PastePayload）
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastePayload {
    #[serde(rename = "type")]
    pub item_type: String,
    pub text: Option<String>,
    pub data_url: Option<String>,
}
