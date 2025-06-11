use clap::{Args, ColorChoice, CommandFactory, Parser, Subcommand};
use nogamepads::string_utils::process_id_text;
use nogamepads_console::utils::{confirm, read_password, read_password_and_confirm};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::current_dir;
use std::fs::File;
use std::io;
use std::io::{BufReader, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use clap_complete::{generate, Shell};
use log::{info, LevelFilter};
use tokio::{select, spawn};
use tokio::signal::ctrl_c;
use tokio::time::sleep;
use nogamepads::entry_mutex;
use nogamepads::logger_utils::logger_build;
use nogamepads_core::data::controller::controller_cli::{process_controller_cli, ControllerCli};
use nogamepads_core::data::controller::controller_data::ControllerData;
use nogamepads_core::data::game::game_cli::{process_game_cli, GameCli};
use nogamepads_core::data::game::game_data::{GameData, GameRuntimeDataArchive};
use nogamepads_core::data::player::player_data::Player;
use nogamepads_core::service::cli_addition::runtime_consoles::RuntimeConsole;
use nogamepads_core::service::service_runner::{NoGamepadsService, ServiceRunner};
use nogamepads_core::service::tcp_network::DEFAULT_PORT;
use nogamepads_core::service::tcp_network::pad_client::pad_client_service::PadClientNetwork;
use nogamepads_core::service::tcp_network::pad_server::pad_server_service::PadServerNetwork;
use nogamepads_core::service::tcp_network::utils::tokio_utils::build_tokio_runtime;

#[derive(Parser, Debug)]
#[command(version, color = ColorChoice::Auto)]
struct Padc {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {

    GenerateCompletion {
        #[arg(value_enum)]
        shell: Shell,
    },

    #[command(subcommand, about = "Manage accounts")]
    Account(AccountCommands),

    #[command(about = "List accounts")]
    Accounts,

    #[command(subcommand, about = "Manage game infos")]
    Game(GameCommands),

    #[command(about = "List all games")]
    Games,

    #[command(about = "Connect to a service")]
    Connect(ConnectArgs),

    #[command(about = "Start a service")]
    Listen(ListenArgs)
}

#[derive(Subcommand, Debug)]
enum AccountCommands {

    #[command(about = "Add a account")]
    Add(AccountArgs),

    #[command(about = "Remove a account")]
    Remove(AccountArgs),

    #[command(about = "Edit the profile of account")]
    Edit(EditAccountArgs)
}

#[derive(Args, Debug)]
struct AccountArgs{

    #[arg(value_name = "Account")]
    account: String,

    #[arg(short, long, value_name = "Password")]
    password: Option<String>
}

#[derive(Args, Debug)]
struct EditAccountArgs{

    #[arg(value_name = "Account")]
    account: String,

    #[arg(short, long, help = "Nickname")]
    nickname: Option<String>,

    #[arg(short = 'c', long = "color", num_args = 3, value_names = ["H", "S", "V"], help = "Player color, h(0 - 360), s(0 - 1), v(0 - 1)")]
    color: Option<Vec<f64>>
}

#[derive(Subcommand, Debug)]
enum GameCommands {

    #[command(about = "Add a game")]
    Add(GameArgs),

    #[command(about = "Remove a game")]
    Remove(GameArgs),

    #[command(about = "Edit the profile of game")]
    Edit(EditGameArgs),

    #[command(subcommand, about = "Register keys to game")]
    Register(RegisterKeysCommands)
}

#[derive(Args, Debug)]
struct GameArgs {

    #[arg(value_name = "Game Name")]
    name: String,

    #[arg(short, long)]
    confirm: bool
}

#[derive(Args, Debug)]
struct EditGameArgs {

    #[arg(value_name = "Game Name")]
    name: String,

    #[arg(value_name = "Key")]
    key: String,

    #[arg(value_name = "Value")]
    value: String,
}

#[derive(Subcommand, Debug)]
enum RegisterKeysCommands {

    #[command(subcommand, about = "Register a button")]
    Button(KeyManageCommands),

    #[command(subcommand, about = "Register a axis")]
    Axis(KeyManageCommands),

    #[command(subcommand, about = "Register a direction")]
    Direction(KeyManageCommands)
}

#[derive(Subcommand, Debug)]
enum KeyManageCommands {

    #[command(about = "Add or rename a key")]
    Add(AddKeyArgs),

    #[command(about = "Remove a key")]
    Remove(RemoveKeyArgs),

    #[command(about = "List all")]
    List(ListKeyArgs)
}

#[derive(Args, Debug)]
struct AddKeyArgs{

    #[arg(value_name = "Game Name")]
    name: String,

    #[arg(value_name = "Key")]
    key: u8,

    #[arg(value_name = "Name")]
    key_name: String
}

#[derive(Args, Debug)]
struct RemoveKeyArgs{

    #[arg(value_name = "Game Name")]
    name: String,

    #[arg(value_name = "Key")]
    key: u8,
}

#[derive(Args, Debug)]
struct ListKeyArgs{

    #[arg(value_name = "Game Name")]
    name: String
}

#[derive(Args, Debug)]
struct ConnectArgs {

    #[arg(short, long, value_name = "Account")]
    account: Option<String>,

    #[arg(short, long, value_name = "Address")]
    tcp_addr: Option<String>,

    #[arg(short, long, value_name = "Methods")]
    method: Option<String>,

    #[arg(long)]
    cmd: bool,

    #[arg(long)]
    debug: bool,
}

#[derive(Args, Debug, Clone)]
struct ListenArgs {

    #[arg(value_name = "Game")]
    game: String,

    #[arg(short = 'a', long, value_name = "Address")]
    tcp_addr: Option<String>,

    #[arg(long)]
    cmd: bool,

    #[arg(long)]
    tcp: bool,

    #[arg(long)]
    bluetooth: bool,

    #[arg(long)]
    usb: bool,

    #[arg(long)]
    debug: bool,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct LocalData {
    game_data: LocalGameData,
    controller_data: LocalControllerData,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct LocalGameData {
    games: HashMap<String, GameData>
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
struct LocalControllerData {
    players: HashMap<String, Player>
}

fn main () {
    let cli = Padc::parse();

    let mut data = read();

    match cli.command {
        Commands::GenerateCompletion {shell} => {
            let cmd = Padc::command();
            generate(shell, &mut cmd.clone(), "padc", &mut io::stdout());
        }

        Commands::Account(cmds) => {
            match cmds {
                AccountCommands::Add(args) => {
                    add_player(&mut data, args.account, args.password);
                }
                
                AccountCommands::Remove(args) => {
                    remove_player(&mut data, args.account, args.password);
                }
                
                AccountCommands::Edit(args) => {
                    edit_player(&mut data, args);
                }
            }
        }

        Commands::Accounts => {
            for player in data.controller_data.players.values() {
                let id = &player.account.id;
                if player.customize.is_some() {
                    let nickname = &player.clone().customize.unwrap().nickname;
                    println!("{}({})", id, nickname);
                } else {
                    println!("{}", id);
                }
            }
        }

        Commands::Game(cmds) => {
            match cmds {
                GameCommands::Add(args) => {
                    let name = process_id_text(args.name);
                    data.game_data.games.entry(name.clone())
                        .or_insert_with(GameData::default);
                    println!("Game configuration added or reset: \"{}\"", name.clone());
                }

                GameCommands::Remove(args) => {
                    if ! args.confirm {
                        if ! confirm("Confirm ") {
                            exit(1);
                        }
                    }

                    let name = process_id_text(args.name);
                    let game = data.game_data.games.remove(&name);
                    if game.is_none() {
                        eprintln!("Removal of the game \"{}\" failed: game not found.", name.clone());
                    } else {
                        println!("The game \"{}\" has been removed.", name.clone());
                    }
                }

                GameCommands::Edit(args) => {
                    let name = process_id_text(args.name);
                    let game = data.game_data.games.get_mut(&name);
                    if game.is_none() {
                        eprintln!("Edit the game \"{}\" failed: game not found.", name.clone());
                    } else {
                        let game = game.unwrap();
                        game.info(args.key.clone(), args.value.clone());
                        println!("Set the game info \"{}\" to \"{}\".", args.key, args.value);
                    }
                }

                GameCommands::Register(cmds) => {
                    match cmds {
                        RegisterKeysCommands::Button(cmds) => {
                            manage_keys(&mut data, cmds,
                                        |game| &mut game.control.button_keys);
                        }

                        RegisterKeysCommands::Axis(cmds) => {
                            manage_keys(&mut data, cmds,
                                        |game| &mut game.control.axis_keys);
                        }

                        RegisterKeysCommands::Direction(cmds) => {
                            manage_keys(&mut data, cmds,
                                        |game| &mut game.control.direction_keys);
                        }
                    }
                }
            }
        }

        Commands::Games => {
            for (key, _) in data.game_data.games.iter() {
                println!("{}", key);
            }
        }

        Commands::Connect(args) => {
            connect(&mut data, args);
        }

        Commands::Listen(args) => {
            let archive = listen(&mut data, args.clone());
            if let Some(archive) = archive {
                if let Some(game) = data.game_data.games.get_mut(&process_id_text(args.game)) {
                    game.archive = archive;
                    info!("Game data archived.");
                }
            }
        }
    }

    write(data);
}

fn connect(data: &mut LocalData, args: ConnectArgs) {

    let mut result: Option<&Player> = None;
    if args.account.is_some() {
        // Account specified
        let id = &args.account.unwrap();
        let player = data.controller_data.players.get(id);

        // Account not found
        if player.is_none() {
            eprintln!("Account not found: \"{}\"", id);
            exit(1);
        } else {
            // Account exists
            result = Some(player.unwrap());
        }
    } else {
        // Account not specified
        for found in data.controller_data.players.values() {
            // Found a replaceable account
            result = Some(found);
            println!("Account not specified! Using account \"{}\" instead!", found.account.id);
            break;
        }
        // No replaceable account found
        if result.is_none() {
            eprintln!("Cannot find any replaceable account! Please ensure at least one account exists locally!");
            exit(1);
        }
    }

    let player = result.unwrap().clone();

    let mut controller = ControllerData::default();
    controller.bind_player(player);

    let runtime = controller.runtime();

    let method = args.method.unwrap_or("tcp".to_string());
    let mut entry: Option<NoGamepadsService> = None;

    match method.trim().to_lowercase().as_str() {
        "tcp" => {
            println!("Using TCP connection.");
            let mut client = PadClientNetwork::build(Arc::clone(&runtime));
            let addr = args.tcp_addr.unwrap_or(format!("127.0.0.1:{}", DEFAULT_PORT));
            client.bind_addr(SocketAddr::from_str(&addr).unwrap_or(
                SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT)),
            ));

            entry = Some(client.build_entry());
        },

        "bluetooth" => {
            println!("Using Bluetooth connection.");
            // TODO :: BLUETOOTH METHOD
        },

        "usb" => {
            println!("Using USB connection.");
            // TODO :: USB METHOD
        },

        _ => {
            eprintln!("Unknown connection method: \"{}\"", method);
            exit(1);
        }
    }

    if let Some(entry) = entry {
        let mut services = Vec::new();
        services.push(entry);

        if args.cmd {
            println!("Enable command line.");

            services.push(RuntimeConsole::build(
                ControllerCli::command(), "ControllerCli".to_string(), Arc::clone(&runtime),

                // Process command line
                |runtime, cmd| {
                    process_controller_cli(runtime, cmd)
                },

                // Check close
                |runtime| {
                    let mut close = false;
                    entry_mutex!(runtime, |guard| {
                        close = guard.close.load(SeqCst);
                    });
                    close
                },

                // Close
                |runtime| {
                    entry_mutex!(runtime, |guard| {
                        guard.close();
                    });
                },

            ).build_entry());
        }

        if args.debug {
            logger_build(LevelFilter::Trace);
        } else {
            logger_build(LevelFilter::Info);
        }

        ServiceRunner::run(services);
    }
}

fn listen(data: &mut LocalData, args: ListenArgs) -> Option<GameRuntimeDataArchive> {
    let id = process_id_text(args.game);
    let game = data.game_data.games.get(&id);
    if game.is_none() {
        eprintln!("Game not found: \"{}\"", id);
        exit(1);
    }

    let game_data = game.unwrap().clone();

    let runtime = game_data.runtime();

    let mut services = Vec::new();

    if args.tcp {
        let mut server = PadServerNetwork::build(Arc::clone(&runtime));
        if args.tcp_addr.is_some() {
            let addr = SocketAddr::from_str(&args.tcp_addr.unwrap())
                .unwrap_or(SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT)));
            server.bind_ip(addr.ip());
            server.bind_port(addr.port());
        }
        services.push(server.build_entry());
        println!("Setup TCP Service!")
    }

    if args.bluetooth {

    }

    if args.usb {

    }

    if args.cmd {
        services.push(RuntimeConsole::build(
            GameCli::command(), "GameCli".to_string(), Arc::clone(&runtime),

            // Process command line
            |runtime, cmd| {
                process_game_cli(runtime, cmd)
            },

            // Check close
            |runtime| {
                let mut close = false;
                entry_mutex!(runtime, |guard| {
                    close = guard.data.close.load(SeqCst);
                });
                close
            },

            // Close
            |runtime| {
                entry_mutex!(runtime, |guard| {
                    guard.close_game();
                });
            },

        ).build_entry());
    }

    if args.debug {
        logger_build(LevelFilter::Trace);
    } else {
        logger_build(LevelFilter::Info);
    }

    ServiceRunner::run(services);

    let mut archived_data: Option<GameRuntimeDataArchive> = None;
    if let Ok(mutex) = Arc::try_unwrap(runtime) {
        let rt = mutex.into_inner()
            .unwrap_or_else(|poison_error| poison_error.into_inner());
        archived_data = Some(GameRuntimeDataArchive::from(rt.data));
    }
    archived_data
}

fn add_player(data: &mut LocalData, account_args: String, password_args: Option<String>) {
    if data.controller_data.players.contains_key(process_id_text(account_args.clone()).as_str()) {
        eprintln!("This account already exists. Please do not create it again.");
        exit(1);
    } else {
        // Read password
        let mut password = "".to_string();
        if password_args.is_none() {
            let input = read_password_and_confirm("Enter password: ", "Confirm: ");
            if input.is_some() {
                password = input.unwrap();
            }
        } else if password_args.is_some() {
            password = password_args.unwrap();
        }

        // Create player
        let player = Player::register(account_args, password);
        let player_key = player.clone().account.id;

        data.controller_data.players.insert(player_key, player.clone());
    }
}

fn remove_player(data: &mut LocalData, account_args: String, password_args: Option<String>) {
    // Read password
    let mut password = "".to_string();
    if password_args.is_none() {
        password = read_password("Enter password: ").unwrap_or("".to_string());
    } else if password_args.is_some() {
        password = password_args.unwrap();
    }

    // Remove
    let player_id = process_id_text(account_args.clone());

    let player = data.controller_data.players.get(&player_id);
    if player.is_none() {
        eprintln!("Failed to remove account \"{}\": Account not found.", account_args.clone());
        exit(1);
    }
    let player = player.unwrap();

    if player.check(password.clone()) {
        data.controller_data.players.remove(player_id.as_str());
        println!("The account \"{}\" has been removed!", player_id);
    } else {
        eprintln!("Failed to remove account \"{}\": Password is incorrect!", player_id)
    }
}

fn edit_player(data: &mut LocalData, args: EditAccountArgs) {
    let account_id = process_id_text(args.account);
    if ! data.controller_data.players.contains_key(&account_id) {
        eprintln!("Edit failed: Account \"{}\" not found!", account_id);
        exit(1);
    }

    let player = data.controller_data.players.get_mut(&account_id).unwrap();

    if args.nickname.is_some() {
        let nickname = args.nickname.unwrap();
        player.nickname(&nickname);
        println!("Change the nickname of account \"{}\" to \"{}\"", account_id, &nickname);
    }

    if args.color.is_some() {
        let color = args.color.unwrap();
        let hue = color[0].round().clamp(0.0, 360.0);
        let sat = color[1].clamp(0.0, 1.0);
        let val = color[2].clamp(0.0, 1.0);
        player.hsv(hue as i32, sat, val);
        println!("Change the color of account \"{}\" to \"{}\"", account_id, hsv_to_hex(hue as i32, sat, val));
    }
}

macro_rules! check_game {
    ($data:expr, $args:expr, |$game:ident| $code:block) => {
        let name = process_id_text($args.name);
        let game = $data.game_data.games.get_mut(&name);
        if game.is_none() {
            eprintln!("Edit the game \"{}\" failed: game not found.", name.clone());
            exit(1);
        } else {
            let $game = game.unwrap();
            $code
        }
    };
}

fn manage_keys(data: &mut LocalData, cmds: KeyManageCommands, get_map: fn(game: &mut GameData) -> &mut HashMap<u8, String>) {
    match cmds {
        KeyManageCommands::Add(args) => {
            check_game!(data, args, |game| {
                get_map(game).entry(args.key)
                    .or_insert_with(|| args.key_name.clone());
                println!("Registered key \"{}\"", args.key_name);
            });
        }

        KeyManageCommands::Remove(args) => {
            check_game!(data, args, |game| {
                let result = get_map(game).remove(&args.key);
                if result.is_some() {
                    println!("Removed key \"{}\"", result.unwrap());
                }
            });
        }

        KeyManageCommands::List(args) => {
            check_game!(data, args, |game| {
                for (key, key_name) in get_map(game).iter() {
                    println!("{} - \"{}\"", key, key_name);
                }
            });
        }
    }
}

fn local_config() -> PathBuf {
    current_dir().unwrap().join("nogamepads.yaml")
}

fn read() -> LocalData {
    let file_path = local_config();

    if ! file_path.exists() {
        let data = LocalData::default();
        let content = serde_yaml::to_string(&data).unwrap();
        File::create(&file_path).unwrap().write_all(content.as_bytes()).unwrap();
        data
    } else {
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).unwrap()
    }
}

fn write(config: LocalData) {
    let file_path = local_config();
    let content = serde_yaml::to_string(&config).unwrap();

    File::create(file_path).unwrap().write_all(content.as_bytes()).unwrap();
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