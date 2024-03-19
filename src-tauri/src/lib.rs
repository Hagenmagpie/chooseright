// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use base64;
use serde_json::json;

#[tauri::command]
fn files() -> serde_json::Value {
    let mut file_map= get_file_map();
    return get_random_keys(&file_map, 4);
}

fn get_file_map() -> HashMap<String, String> {
    // 定义 assets/flags/ 目录路径
    let assets_flags_dir = "./assets/flags/";

    // 读取 assets/flags/ 目录下的所有文件
    let files: fs::ReadDir = fs::read_dir(assets_flags_dir).unwrap();

    //  创建一个 HashMap 来存储键值对
    let mut map: HashMap<String, String> = HashMap::new();

    // 遍历所有文件并将其添加到 HashMap 中
    for file in files {
        let file_name = file.unwrap().file_name();
        let file_name_str = file_name.to_string_lossy();

        // 从文件名中提取国家名称
        // println!("{}", file_name_str);
        let country_name = match file_name_str.split("-").next(){
            Some(value) => value,
            None => "",
        };

        // 读取文件内容
        let file_path = format!("{}/{}", assets_flags_dir, file_name_str);
        let file_bytes = fs::read(file_path).unwrap();

        // 将文件内容转换为 base64 编码
        let base64_encoded = base64::encode(&file_bytes);
        // 将国家名称和文件名添加到 HashMap 中
        map.insert(country_name.to_string(), base64_encoded);
    }
    // for (key, value) in map.iter() {
    //     println!("{}: {}", key, value);
    // }
    // 返回 HashMap
    map
}

fn get_random_keys(map: &HashMap<String, String>, count: usize) -> serde_json::Value {
    // 使用 Set 来存储 HashMap 的 key
    let mut name_file_map: HashMap<String,String> = HashMap::new();
    let mut name: String = String::new();
    // 随机选择 4 个 key
    for _ in 0..count {
        let index = rand::random::<usize>() % map.len();
        //随机获取map 的 key
        let key = map.keys().nth(index).unwrap().to_string();
        //name_file_map.push(key.to_string(),map.get(&key).unwrap().to_string());
        name_file_map.insert(key.to_string(),map.get(&key).unwrap().to_string());
    }
    //let keys:Vec<String> = name_file_map.values().cloned().collect();
    //打印 keys
    // for (key, value) in map.iter() {
    //     println!("{}: {}", key, value);
    // }
    //从4个key中随机选择一个
    let index = rand::random::<usize>() % name_file_map.len();
    name = name_file_map.keys().nth(index).unwrap().to_string();
    let json = json!({
        "name": name,
        "maps": name_file_map,
    });
    // 返回 keys
    return json;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
