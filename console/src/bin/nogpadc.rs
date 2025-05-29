use crate::AccountCommands::{Add, Customize, List, Remove};
use crate::Commands::{Account, Connect};
use clap::{Args, Parser, Subcommand};
use prettytable::{row, Table};
use rand::Rng;
use std::env::current_dir;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{BufReader, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use nogamepads_lib_rs::DEFAULT_PORT;
use nogamepads_lib_rs::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
use nogamepads_lib_rs::pad_service::client::nogamepads_client::PadClient;

/// NoGamePads Console - Client Cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct NoGamepadClientCli {
    #[command(subcommand)]
    command: Commands,
}

/// 主要命令
#[derive(Subcommand, Debug)]
enum Commands {

    // 账户设置
    #[command(subcommand, about = "Operation of player account")]
    Account(AccountCommands),

    // 连接到服务器
    #[command(about = "Connect a player to server")]
    Connect(ConnectArgs)
}

/// 账户设置 命令
#[derive(Subcommand, Debug)]
enum AccountCommands {

    // 列出所有账号
    #[command(about = "List all local players")]
    List(ListAccountArgs),

    // 添加账号
    #[command(about = "Add a local player")]
    Add(AddAccountArgs),

    // 移除账号
    #[command(about = "Remove a local player")]
    Remove(RemoveAccountArgs),

    // 自定义账号显示信息
    #[command(about = "Customize how the player appears")]
    Customize(CustomizeAccountArgs)
}

/// 列出所有账号 参数
#[derive(Args, Debug)]
struct ListAccountArgs { }

/// 添加账号 参数
#[derive(Args, Debug)]
struct AddAccountArgs {

    // 注册的角色 ID
    #[arg(value_name = "NAME")]
    id: String
}

/// 移除账号 参数
#[derive(Args, Debug)]
struct RemoveAccountArgs {

    // 删除的角色 ID
    #[arg(value_name = "NAME")]
    id: String
}

/// 自定义账号显示信息 参数
#[derive(Args, Debug)]
struct CustomizeAccountArgs {

    // 定制的角色 ID
    #[arg(value_name = "WHO")]
    id: String,

    // 昵称
    #[arg(short, long, help = "Nickname")]
    name: Option<String>,

    // 颜色
    #[arg(short = 'H', long = "hsv", num_args = 3, value_names = ["H", "S", "V"], help = "Player color, h(0 - 360), s(0 - 1), v(0 - 1)")]
    hsv: Option<Vec<f64>>
}

/// 连接到服务器
#[derive(Args, Debug)]
struct ConnectArgs {

    // 连接的玩家
    #[arg(value_name = "WHO")]
    id: String,

    // 目标服务器
    #[arg(value_name = "WHERE", default_value = "127.0.0.1")]
    target: String,

    // 目标端口
    #[arg(short, long, default_value = "5989")] // <---- DEFAULT_PORT
    port: Option<u16>,

    // 启用调试
    #[arg(long)]
    debug: bool,
}

/// 配置文件后缀名称
const EXTENSION_NAME : &str = "yaml";

fn main() {

    // 初始化
    let root = get_config_folder_path();
    if ! root.exists() {
        create_dir_all(root.as_path()).unwrap();
    }

    // 命令行
    let cli = NoGamepadClientCli::parse();
    match cli.command {

        // 账户设置部分
        Account(commands) => {
            match commands {

                // 添加账号
                Add(args) => {
                    if is_account_exist(&args.id) {
                        println!("Account already exists!");
                    } else {
                        // 账号 ID 和 配置路径
                        let account_id = process_inputted_text(args.id);
                        let account_config_path = get_account_config_path(&account_id);

                        // 为新号准备的配置文件
                        let mut new_info = PlayerInfo::new();

                        // 密码输入和密码验证
                        let password = rpassword::prompt_password("Type password: ").unwrap();
                        let password_confirm = rpassword::prompt_password("Confirm password: ").unwrap();
                        if ! password_confirm.eq(&password) {
                            println!("Password does not match!");
                            exit(1);
                        }

                        // 随机生成 色调 值
                        let mut rng = rand::rng();
                        let random_hue: i32 = rng.random_range(0..=360);

                        // 建立账号信息并预填入信息
                        new_info.setup_account_info(account_id.as_str(), password.as_str());
                        new_info.set_nickname(account_id.as_str());
                        new_info.set_customize_color_hsv(random_hue, 0.8, 0.8);

                        // 新配置信息的文本
                        let new_info_yaml = serde_yaml::to_string(&new_info);

                        // 将信息写入文件系统
                        let mut buffer = File::create(account_config_path).unwrap();
                        buffer.write_all(new_info_yaml.unwrap().as_bytes()).unwrap();

                        println!("Account created.");
                    }
                },

                // 移除账号
                Remove(args) => {
                    if ! is_account_exist(&args.id) {
                        println!("Account not found!");
                    } else {
                        // 账号 ID 和 配置路径
                        let account_id = process_inputted_text(args.id);
                        let account_config_path = get_account_config_path(&account_id);

                        // 删除文件
                        remove_file(account_config_path).unwrap();

                        println!("Account removed.");
                    }
                },

                // 列出所有账号
                List(_args) => {
                    // 账号信息文件夹
                    let folder_path = get_config_folder_path();

                    // 输出表格
                    let mut info_table = Table::new();

                    // 表头
                    info_table.add_row(row!["ACCOUNT_ID", "NICKNAME", "COLOR", "HASH"]);

                    // 遍历目录下文件，将信息逐一填入表格
                    for item in folder_path.read_dir().unwrap() {
                        if let Ok(path) = item {
                            // 文件名
                            let file_name = path.file_name().into_string().unwrap();
                            let ext = format!(".{}", EXTENSION_NAME);

                            // 判断是否为指定后缀
                            if file_name.contains(ext.as_str()) {

                                // 去除后缀内容，截取为 ID
                                let id = file_name.replace(ext.as_str(), "");

                                // 读取并加载其中的玩家信息
                                let file = File::open(get_account_config_path(&id)).unwrap();
                                let reader = BufReader::new(file);
                                let info: PlayerInfo = serde_yaml::from_reader(reader).unwrap();

                                // 填入表格
                                info_table.add_row(row![
                                    &id,                        // ACCOUNT_ID
                                    info.customize.nickname,    // NICKNAME
                                    hsv_to_hex(                 // COLOR
                                        info.customize.color_hue,
                                        info.customize.color_saturation,
                                        info.customize.color_value),
                                    info.account.player_hash    // HASH
                                ]);
                            }
                        }
                    }
                    println!("{}", info_table.to_string())
                },

                // 自定义账号显示信息
                Customize(args) => {

                    // 加载配置文件
                    let file = File::open(get_account_config_path(&args.id)).unwrap();
                    let reader = BufReader::new(file);
                    let mut info: PlayerInfo = serde_yaml::from_reader(reader).unwrap();

                    // HSV 参数
                    if args.hsv.is_some() {
                        let hsv = args.hsv.unwrap();
                        let hue = hsv[0].round().clamp(0.0, 360.0);
                        let sat = hsv[1].clamp(0.0, 1.0);
                        let val = hsv[2].clamp(0.0, 1.0);

                        info.customize.color_hue = hue as i32;
                        info.customize.color_saturation = sat;
                        info.customize.color_value = val;

                        println!("Set {}'s HSV color to: {}, {}, {}.", &args.id, hue, sat, val);
                    }

                    // 昵称 参数
                    if args.name.is_some() {
                        let name = args.name.unwrap();
                        info.customize.nickname = name.clone();

                        println!("Set {}'s display name to: {}.", &args.id, name);
                    }

                    // 写入配置文件
                    let yaml_content = serde_yaml::to_string(&info);
                    let mut buffer = File::create(get_account_config_path(&args.id)).unwrap();
                    buffer.write_all(yaml_content.unwrap().as_bytes()).unwrap();
                },
            }
        },

        // 连接到服务器
        Connect(args) => {
            if ! is_account_exist(&args.id) {
                println!("Player not found!");
                exit(1);
            }

            // 加载配置文件
            let file = File::open(get_account_config_path(&args.id)).unwrap();
            let reader = BufReader::new(file);
            let info: PlayerInfo = serde_yaml::from_reader(reader).unwrap();

            // 从参数获得 Ip 地址 (或默认)
            let addr : IpAddr;
            match IpAddr::from_str(&args.target) {
                Ok(result) => { addr = result; }
                Err(_err) => {
                    addr = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
                }
            }

            // 从参数获得端口地址 (或默认)
            let port : u16 = if args.port.is_some() { args.port.unwrap() } else { DEFAULT_PORT }
                .clamp(0, 65535);

            // 绑定目标地址
            let mut client = PadClient::bind_addr_with_port(addr, port);

            // 启动调试模式 ？
            if args.debug {
                println!("- DEBUG MODE -");
                client.enable_console();
            }

            // 写入玩家信息
            client.bind_player(info);

            // 连接
            client.connect();
            println!("Connected {} to {}:{}", args.id, addr.to_string(), port.to_string());
        }
    }
}

fn process_inputted_text(input: String) -> String {
    // 截取前后文本，并转换为小写
    let s = input.trim().to_lowercase();
    let mut result = String::new();

    // 处理其中的特殊符号，部分用于分割的符号需要转换为下划线
    for c in s.chars() {
        match c {
            '\n' | '_' => continue,
            '-' | '.' | ',' | ' ' => result.push('_'),
            _ => result.push(c),
        }
    }

    // 仅保留 ASCII 字符
    result.chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '_')
        .collect()
}

fn hsv_to_hex(h: i32, s: f64, v: f64) -> String {
    let h = (h as f64).clamp(0.0, 360.0);
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match (h / 60.0) as usize {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn get_config_folder_path() -> PathBuf {
    current_dir().unwrap().join(".nogpadc")
}

fn get_account_config_path(id: &str) -> PathBuf {
    get_config_folder_path().join(format!("{}.{}", process_inputted_text(id.to_string()), EXTENSION_NAME))
}

fn is_account_exist(id: &str) -> bool {
    let id = process_inputted_text(id.to_string());
    let path = get_config_folder_path();
    let dir = path.as_path().read_dir().unwrap();
    let mut found = false;
    for item in dir {
        if let Ok(path) = item {
            if path.file_name().eq(format!("{}.{}", id, EXTENSION_NAME).as_str()) {
                found = true;
            }
        }
    }
    found
}