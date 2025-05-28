pub mod nogamepads_client {
    use std::collections::VecDeque;
    use crate::pad_data::pad_messages::nogamepads_message_transfer::{read_msg, send_msg};
    use crate::pad_data::pad_messages::nogamepads_messages::{ConnectionCallbackMessage, ConnectionErrorType, ConnectionMessage, ControlMessage, GameMessage, LeaveReason};
    use crate::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
    use log::{error, info};
    use std::net::{IpAddr, Ipv4Addr};
    use std::process::exit;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use clap::CommandFactory;
    use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
    use tokio::net::TcpStream;
    use tokio::{io, spawn};
    use nogamepads::console_utils::debug_console::read_cli;
    use nogamepads::convert_utils::convert_deque_to_vec;
    use nogamepads::logger_utils::logger_build;
    use crate::pad_service::client_debug_cli::{process_debug_cmd, Pcc};
    use crate::DEFAULT_PORT;
    use crate::pad_data::game_profile::game_profile::GameProfile;
    use crate::pad_data::pad_messages::nogamepads_message_encoder::NgpdMessageEncoder;

    type WriteList = Arc<Mutex<VecDeque<ControlMessage>>>;
    type ReadList = Arc<Mutex<VecDeque<GameMessage>>>;

    #[repr(C)]
    pub struct PadClient {

        // --- 主要参数 ---

        // 目标地址
        target_address: IpAddr,

        // 目标端口
        #[allow(dead_code)]
        target_port: u16,

        // 绑定的玩家
        bind_player: PlayerInfo,

        // 调试模式
        enable_console: bool,

        // 保持安静，不初始化 env_logger
        quiet: bool,

        // --- 运行时参数 ---

        // 发送信息列表
        write_list: WriteList,

        // 读取信息列表
        read_list: ReadList,

        // 是否退出
        exit: AtomicBool,
    }

    impl Default for PadClient {
        fn default() -> Self {
            PadClient {
                enable_console: false,
                target_address: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
                target_port: DEFAULT_PORT,
                bind_player: PlayerInfo::new(),
                quiet: false,

                write_list: WriteList::default(),
                read_list: ReadList::default(),
                exit: AtomicBool::new(false)
            }
        }
    }

    // 客户端构建部分
    impl PadClient {

        pub fn bind_addr(address: IpAddr) -> PadClient {
            PadClient {
                target_address: address,
                ..PadClient::default()
            }
        }

        pub fn bind_addr_with_port(address: IpAddr, port: u16) -> PadClient {
            PadClient {
                target_address: address,
                target_port: port,
                ..PadClient::default()
            }
        }

        pub fn enable_console(&mut self) {
            self.enable_console = true;
        }

        pub fn quiet(&mut self) -> &mut PadClient {
            self.quiet = true;
            self
        }

        pub fn bind_player(&mut self, player: PlayerInfo) {
            self.bind_player = player;
        }
    }

    // 客户端消息管理
    impl PadClient {

        pub fn put_msg(&self, msg: ControlMessage) {
            let mut guard = self.write_list.lock().unwrap();
            guard.push_back(msg);
        }

        pub fn pop_a_msg(&self) -> Option<GameMessage> {
            let mut guard = self.read_list.lock().unwrap();
            if !guard.is_empty() {
                guard.pop_front()
            } else {
                None
            }
        }

        pub fn pop_msg_or(&self, or: GameMessage) -> GameMessage {
            self.pop_a_msg().unwrap_or(or)
        }

        pub fn list_received(&self) -> Vec<GameMessage> {
            match self.read_list.lock() {
                Ok(guard) => {
                    convert_deque_to_vec(&guard.to_owned())
                }
                Err(_) => { Vec::new() }
            }
        }
    }

    // 客户端状态控制
    impl PadClient {

        pub fn connect(self) {
            
            self.exit.store(false, SeqCst);

            // 构建 Logger
            if !self.quiet {
                logger_build();
            }

            // 调试模式
            let debug = self.enable_console;

            // 客户端对象的 Arc
            let arc_client = Arc::new(self);

            // 部署环境
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .thread_name("nogpad-pad_service")
                .thread_stack_size(32 * 1024 * 1024)
                .enable_time()
                .enable_io()
                .build()
                .unwrap();

            info!("Starting \"NoGamepads Client\".");

            // 入口
            let entry = async move {
                let main_thread = spawn({
                    let client = Arc::clone(&arc_client);
                    async move {
                        Self::main_client_thread(client).await
                    }
                });

                let background_thread = spawn({
                    let client = Arc::clone(&arc_client);
                    async move {
                        Self::background_thread(client).await
                    }
                });

                if debug {
                    let debug_cli = spawn({
                        let client = Arc::clone(&arc_client);
                        async move {
                            Self::process_debug_cli(client).await
                        }
                    });
                    let _ = tokio::join!(debug_cli, main_thread, background_thread);
                } else {
                    let _ = tokio::join!(main_thread, background_thread);
                }
            };

            // 阻塞运行
            runtime.block_on(entry);
        }

        pub fn exit_server(&self) {
            self.exit.store(true, SeqCst);
        }

        async fn main_client_thread(self: Arc<Self>) {
            let mut buffer : [u8; 1024] = [0; 1024];
            let addr_str = format!("{}:{}", self.target_address.to_string(), DEFAULT_PORT);

            info!("Connected to {}", &addr_str);

            // 下载服务端配置文件
            {
                info!("Check: Downloaded game profile.");
                let profile = self.check_server_profile(&mut buffer, addr_str.clone()).await;
                if profile.is_some() {
                    info!("Success: Downloaded.");
                    let profile = profile.unwrap_or(GameProfile::default());
                    for line in profile.to_string().split('\n') {
                        info!("{}", line);
                    }
                }
                else {
                    error!("Failed: Can't download profile!");
                    self.exit_server();
                }
            }

            // 尝试加入服务端，并建立长连接
            {
                if !self.try_join_game(&mut buffer, addr_str.clone()).await {
                    error!("Failed: Can't join the game!");
                    self.exit_server();
                    return;
                }
            }
        }

        async fn check_server_profile(self: &Arc<Self>, buffer: &mut [u8], addr_str: String) -> Option<GameProfile> {
            match TcpStream::connect(&addr_str).await {
                Ok(mut stream) => {
                    send_msg(&mut stream, ConnectionMessage::RequestProfile).await;
                    let callback : ConnectionCallbackMessage = read_msg(buffer, &mut stream).await;
                    match callback {
                        ConnectionCallbackMessage::Profile(profile) => {
                            Some(profile)
                        }
                        ConnectionCallbackMessage::Deny(err_type) => {
                            error!("Request failed: Server denied your request! ({:?})", err_type);
                            None
                        }
                        ConnectionCallbackMessage::Err => {
                            error!("Connection failed: Can't connect to server!");
                            None
                        }
                        _ => { None }
                    }
                }
                Err(_err) => {
                    None
                }
            }
        }

        async fn try_join_game(self: &Arc<Self>, buffer: &mut [u8], addr_str: String) -> bool {

            match TcpStream::connect(&addr_str).await {
                Ok(mut stream) => {

                    // 发送连接请求
                    let info = self.bind_player.clone();
                    send_msg(&mut stream, ConnectionMessage::Connection(info)).await;

                    // 读取回调
                    let callback : ConnectionCallbackMessage = read_msg(buffer, &mut stream).await;
                    match callback {
                        ConnectionCallbackMessage::Deny(error) => {
                            match error {
                                ConnectionErrorType::ContainSamePlayer => {
                                    error!("Connection failed: Contains same player!");
                                    false
                                }
                                ConnectionErrorType::PlayerBanned => {
                                    error!("Connection failed: You are banned!");
                                    false
                                }
                                ConnectionErrorType::Timeout => {
                                    error!("Connection failed: Timeout!");
                                    false
                                }
                                ConnectionErrorType::GameLocked => {
                                    error!("Connection failed: Game was locked!");
                                    false
                                }
                                _ => { false }
                            }
                        }
                        ConnectionCallbackMessage::Ok => {

                            // 服务端检查完毕，发送 Ready 以示加入游戏
                            send_msg(&mut stream, ConnectionMessage::Ready).await;
                            let callback : ConnectionCallbackMessage = read_msg(buffer, &mut stream).await;
                            match callback {
                                ConnectionCallbackMessage::Welcome => {
                                    info!("Welcome!");
                                    Self::long_connection(Arc::clone(&self), stream).await;
                                }
                                ConnectionCallbackMessage::Deny(_error) => {
                                    error!("Request failed: Server denied your request",);
                                }
                                _ => {}
                            }
                            true
                        }
                        _ => { false }
                    }
                }
                Err(err) => {
                    error!("Failed to connect to server: {}", err);
                    false
                }
            }
        }

        async fn long_connection(self: Arc<Self>, stream: TcpStream) {
            let (reader, writer) = io::split(stream);
            spawn(Self::read_task(Arc::clone(&self), reader));
            spawn(Self::write_task(Arc::clone(&self), writer));
        }

        async fn read_task(self: Arc<Self>, mut reader: ReadHalf<TcpStream>) {
            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let msg = GameMessage::de(buf[0..n].to_vec());
                        {
                            match self.read_list.lock() {
                                Ok(mut guard) => {
                                    match &msg {
                                        GameMessage::Leave(reason) => {
                                            match reason {
                                                LeaveReason::GameOver => {
                                                    info!("Leave Game: Game Over!");
                                                    self.exit_server();
                                                }
                                                LeaveReason::ServerClosed => {
                                                    info!("Leave Game: Server closed!");
                                                    self.exit_server();
                                                }
                                                LeaveReason::YouAreKicked => {
                                                    error!("Kick Game: You are kicked!");
                                                    self.exit_server();
                                                }
                                                LeaveReason::YouAreBanned => {
                                                    error!("Kick Game: You are banned!");
                                                    self.exit_server();
                                                }
                                            }
                                        }
                                        _ => {
                                            info!("{:?}", &msg);
                                            guard.push_back(msg);
                                        }
                                    }
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading from stream: {}", e);
                        self.exit_server();
                        break;
                    }
                }
            }
        }

        async fn write_task(self: Arc<Self>, mut writer: WriteHalf<TcpStream>) {
            loop {
                let msg : Option<ControlMessage>;
                {
                    let lock = self.write_list.lock();
                    match lock {
                        Ok(mut guard) => {
                            if ! guard.is_empty() {
                                msg = guard.pop_front();
                            } else {
                                msg = None;
                            }
                        }
                        Err(_) => {
                            msg = None;
                        }
                    }
                }
                if msg.is_some() {
                    let msg = msg.unwrap();
                    match &writer.write_all(NgpdMessageEncoder::en(&msg).as_slice()).await {
                        Ok(_) => {
                            info!("Sent {:?}", msg);
                        }
                        Err(_error) => {
                            error!("Sent {:?} failed!", msg);
                        }
                    }
                }
            }
        }

        async fn background_thread(self: Arc<Self>) {
            loop {
                // 退出程序的监听
                if self.exit.load(SeqCst) {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    info!("Main thread exited.");
                    exit(0);
                }
            }
        }

        async fn process_debug_cli(self: Arc<Self>) {
            loop {
                if self.exit.load(SeqCst) {
                    info!("Debug console exited");
                    break
                }
                tokio::time::sleep(Duration::from_secs_f64(0.2)).await;
                let option: Option<Pcc> = read_cli(
                    format!("CLIENT {}/{}> ",
                            self.target_address.to_string(),
                            self.bind_player.account.id).as_str(),
                    "pcc".to_string(),
                    Pcc::command()
                ).await;
                match option {
                    None => {}
                    Some(cmd) => {
                        process_debug_cmd(cmd, Arc::clone(&self));
                    }
                }
            }
        }
    }
}