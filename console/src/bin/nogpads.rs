use std::collections::HashMap;
use clap::{arg, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::{BufReader, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use nogamepads_lib_rs::DEFAULT_PORT;
use nogamepads_lib_rs::pad_data::game_profile::game_profile::GameProfile;
use nogamepads_lib_rs::pad_data::layout::layout_data::LayoutKeyRegisters;
use nogamepads_lib_rs::pad_service::server::nogamepads_server::PadServer;

/// NoGamePads Console - Server Cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct NoGamepadServerCli {
    #[command(subcommand)]
    command: Commands,
}

/// 主要命令
#[derive(Subcommand, Debug)]
enum Commands {

    #[command(subcommand, about = "Manage buttons")]
    Button(ManageCommands),

    #[command(subcommand, about = "Manage directions")]
    Direction(ManageCommands),

    #[command(subcommand, about = "Manage axes")]
    Axis(ManageCommands),

    // 服务端配置
    #[command(about = "Configure the server")]
    Config(ConfigArgs),

    // 运行服务端
    #[command(about = "Run the server")]
    Run(RunArgs)
}

/// 服务端配置 参数
#[derive(Args, Debug)]
struct ConfigArgs {

    // 绑定的端口号
    #[arg(short, long, help = "Server port (0 = Default)")] // <---- DEFAULT_PORT
    port: Option<u16>,

    // 游戏名称
    #[arg(short ='n', long = "name")]
    game_name: Option<String>,

    // 游戏描述
    #[arg(short = 'd', long = "description")]
    game_description: Option<String>,

    // 游戏组织
    #[arg(short = 'o', long = "organization")]
    organization: Option<String>,

    // 游戏版本
    #[arg(short = 'v', long = "version")]
    version: Option<String>,

    // 工作室 & 游戏 主页
    #[arg(short = 'w', long = "website")]
    website: Option<String>,

    // 交流邮箱
    #[arg(short = 'e', long = "email")]
    email: Option<String>
}

/// 运行服务端 参数
#[derive(Args, Debug)]
struct RunArgs {

    // 调试模式
    #[arg(long)]
    debug: bool,
}

/// 管理键
#[derive(Subcommand, Debug)]
enum ManageCommands {

    #[command(about = "Add or rename a key")]
    Add(AddKeyArgs),

    #[command(about = "Remove a key")]
    Remove(RemoveKeyArgs),

    #[command(about = "List all")]
    List
}

/// 添加键
#[derive(Args, Debug)]
struct AddKeyArgs{

    // 添加的键
    #[arg(value_name = "KEY")]
    key: u8,

    // 事件编号
    #[arg(value_name = "NAME")]
    name: String
}

/// 移除键
#[derive(Args, Debug)]
struct RemoveKeyArgs{

    // 删除的键
    #[arg(value_name = "KEY")]
    key: u8,
}

/// 本地存储的配置信息
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ServerConfig {
    port: u16,
    registered_keys: LayoutKeyRegisters,
    profile: GameProfile,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            port: DEFAULT_PORT,
            registered_keys: Default::default(),
            profile: GameProfile::default(),
        }
    }
}

/// # 快速生成 更新服务端信息 的宏
macro_rules! update_config {
    ($config:expr, $args:expr, $($field:ident),+) => {
        $(
            if let Some(ref value) = $args.$field {
                $config.profile.$field = value.clone();
                println!("Changed profile \"{}\" to \"{}\"",
                    stringify!($field),
                    value
                );
            }
        )+
    };
}

fn main () {

    // 命令行
    let cli = NoGamepadServerCli::parse();

    // 读取服务端配置
    let mut config = read_config();

    match cli.command {

        // 服务端配置
        Commands::Config(args) => {

            // 端口信息配置：
            // 端口数值被限定在 0 - 65535，但是若端口参数为 0，则会被设置为默认端口
            if args.port.is_some() {
                let port = args.port.unwrap_or(DEFAULT_PORT).clamp(0, 65535);
                config.port = if port == 0 { DEFAULT_PORT } else { port };
            }

            // 其他信息配置
            update_config!(
                config, args,
                game_name,
                game_description,
                organization,
                version,
                website,
                email
            );
        },

        // 运行服务端
        Commands::Run(args) => {
            // 读取服务端配置
            let config = read_config();

            // 根据调试选项启动服务端
            if args.debug {
                println!("- DEBUG MODE -");
                println!("Server started!");
                PadServer::default()
                    .addr(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), config.port)
                    .put_profile(config.profile)
                    .register_keys(config.registered_keys)
                    .enable_console()
                    .build()
                    .start_server();
            } else {
                println!("Server started!");
                PadServer::default()
                    .addr(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), config.port)
                    .put_profile(config.profile)
                    .register_keys(config.registered_keys)
                    .build()
                    .start_server();
            }
        }

        Commands::Button(manage) => {
            process_manage_command("btn", manage, &mut config.registered_keys.button_keys);
        }

        Commands::Direction(manage) => {
            process_manage_command("dir", manage, &mut config.registered_keys.direction_keys);
        }

        Commands::Axis(manage) => {
            process_manage_command("ax", manage, &mut config.registered_keys.axis_keys);
        }
    }

    // 写入配置
    write_config(config);
}

fn process_manage_command(prefix: &str, manage: ManageCommands, map: &mut HashMap<u8, String>) {
    match manage {
        ManageCommands::Add(args) => {
            map.entry(args.key)
                .or_insert_with(|| args.name.clone());
            println!("Added(Renamed) key {}_{} : \"{}\".", prefix, args.key, args.name);
        }
        ManageCommands::Remove(args) => {
            let removed = map.remove(&args.key);
            if removed.is_some() {
                println!("Removed key {}_{}.", prefix, removed.is_some())
            } else {
                println!("Removed key failed: Cannot found key {}", args.key);
            }
        }
        ManageCommands::List => {
            for button_key in map {
                println!("{}_{} : \"{}\"", prefix, button_key.0, button_key.1)
            }
        }
    }
}

#[allow(dead_code)]
fn get_config_folder_path () -> PathBuf {
    current_dir().unwrap().join(".nogpads")
}

#[allow(dead_code)]
fn get_config_file_path () -> PathBuf {
    get_config_folder_path().join("config.yaml")
}

#[allow(dead_code)]
fn get_layout_file_path () -> PathBuf {
    get_config_folder_path().join("layout.yaml")
}

#[allow(dead_code)]
fn get_assets_package_path () -> PathBuf {
    get_config_folder_path().join("assets.zip")
}

fn read_config () -> ServerConfig {
    let config_folder_path = get_config_folder_path();
    let config_file_path = get_config_file_path();

    if ! config_folder_path.exists() {
        create_dir(&config_folder_path).unwrap();
    }

    if ! config_file_path.exists() {
        let config = ServerConfig::default();
        let config_text = serde_yaml::to_string(&config).unwrap();
        File::create(&config_file_path).unwrap().write_all(config_text.as_bytes()).unwrap();
        config
    } else {
        let config_file = File::open(&config_file_path).unwrap();
        let config_reader = BufReader::new(config_file);
        serde_yaml::from_reader(config_reader).unwrap()
    }
}

fn write_config (config: ServerConfig) {
    let config_file_path = get_config_file_path();

    let config_text = serde_yaml::to_string(&config).unwrap();
    File::create(config_file_path).unwrap().write_all(config_text.as_bytes()).unwrap();
}