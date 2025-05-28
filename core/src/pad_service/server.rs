pub mod nogamepads_server {
    use std::collections::{HashMap, VecDeque};
    use crate::pad_data::pad_messages::nogamepads_message_encoder::NgpdMessageEncoder;
    use crate::pad_data::pad_messages::nogamepads_message_transfer::{read_msg, send_msg};
    use crate::pad_data::pad_messages::nogamepads_messages::{ConnectionCallbackMessage, ConnectionMessage, ControlMessage, GameMessage, LeaveReason};
    use log::{error, info, warn};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::process::exit;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
    use std::time::Duration;
    use clap::CommandFactory;
    use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::{io, spawn};
    use tokio::runtime::Runtime;
    use nogamepads::console_utils::debug_console::read_cli;
    use nogamepads::convert_utils::convert_deque_to_vec;
    use nogamepads::logger_utils::logger_build;
    use crate::DEFAULT_PORT;
    use crate::pad_data::game_profile::game_profile::GameProfile;
    use crate::pad_data::pad_messages::nogamepads_messages::ConnectionErrorType::{ContainSamePlayer, GameLocked, PlayerBanned, WhatTheHell};
    use crate::pad_data::pad_messages::nogamepads_messages::GameMessage::Leave;
    use crate::pad_data::pad_messages::nogamepads_messages::LeaveReason::ServerClosed;
    use crate::pad_data::pad_player_info::nogamepads_player_info::PlayerInfo;
    use crate::pad_service::server_debug_cli::{process_debug_cmd, Psc};

    type PlayerMap = Arc<Mutex<HashMap<String, PlayerInfo>>>;
    type WriteList = Arc<Mutex<HashMap<String, VecDeque<GameMessage>>>>;
    type ReadList = Arc<Mutex<HashMap<String, VecDeque<ControlMessage>>>>;

    #[repr(C)]
    pub struct PadServer {

        // --- 主要参数 ---

        // 本地监听地址
        address: IpAddr,

        // 游戏信息
        game_profile: GameProfile,

        // 绑定端口
        port: u16,

        // 调试模式
        enable_console: bool,

        // 保持安静，不初始化 env_logger
        quiet: bool,

        // --- 运行时参数 ---

        // 发送信息列表
        write_list: WriteList,

        // 读取信息列表
        read_list: ReadList,

        // 在线玩家
        online_players: PlayerMap,

        // 被封禁的玩家
        banned_players: PlayerMap,

        // 是否锁定该游戏：禁止后续玩家加入
        game_locked: AtomicBool,

        // 是否停止服务器
        stop: AtomicBool,
    }

    impl Clone for PadServer {
        fn clone(&self) -> Self {
            PadServer {
                address: self.address.clone(),
                game_profile: self.game_profile.clone(),
                port: self.port.clone(),
                enable_console: self.enable_console,
                quiet: self.quiet,

                write_list: self.write_list.clone(),
                read_list: self.read_list.clone(),
                online_players: self.online_players.clone(),
                banned_players: self.banned_players.clone(),
                game_locked: AtomicBool::new((&self.game_locked.load(SeqCst)).clone()),
                stop: AtomicBool::new((&self.stop.load(SeqCst)).clone()),
            }
        }
    }

    impl Default for PadServer {
        fn default() -> Self {
            PadServer {
                address: IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)),
                game_profile: GameProfile::default(),
                port: DEFAULT_PORT,
                enable_console: false,
                quiet: false,

                write_list: WriteList::default(),
                read_list: ReadList::default(),
                online_players: PlayerMap::default(),
                banned_players: PlayerMap::default(),
                game_locked: AtomicBool::new(false),
                stop: AtomicBool::new(false),
            }
        }
    }

    // 服务端构建部分
    impl PadServer {

        pub fn build_simple() -> Arc<PadServer> {
            Arc::new(Self::default()
                .addr(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), DEFAULT_PORT)
                .put_profile(GameProfile::default()).to_owned())
        }

        pub fn addr(&mut self, ip_addr: IpAddr, port: u16) -> &mut PadServer {
            self.ip_addr(ip_addr).port(port)
        }

        pub fn socket_addr(&mut self, socket_addr: SocketAddr) -> &mut PadServer {
            self.ip_addr(socket_addr.ip()).port(socket_addr.port())
        }

        pub fn port(&mut self, port: u16) -> &mut PadServer {
            self.port = port;
            self
        }

        pub fn ip_addr(&mut self, ip_addr: IpAddr) -> &mut PadServer {
            self.address = ip_addr;
            self
        }

        pub fn put_profile(&mut self, profile: GameProfile) -> &mut PadServer {
            self.game_profile = profile;
            self
        }

        pub fn enable_console(&mut self) -> &mut PadServer {
            self.enable_console = true;
            self
        }

        pub fn quiet(&mut self) -> &mut PadServer {
            self.quiet = true;
            self
        }

        pub fn build(&self) -> Arc<PadServer> {
            Arc::new(self.clone())
        }

    }

    // 服务端消息管理
    impl PadServer {

        pub fn put_msg_to(&self, msg: GameMessage, player: &PlayerInfo) {
            match self.write_list.lock() {
                Ok(mut guard) => {
                    let hash = &player.account.player_hash.clone();
                    if ! guard.contains_key(hash.as_str()) {
                        guard.entry(player.account.player_hash.clone())
                            .or_insert_with(VecDeque::new)
                            .push_back(msg);
                    }
                }
                Err(_) => {
                    error!("Cannot lock \"{:?}\" in write_list", player.account.player_hash);
                }
            }
        }

        pub fn put_msg_to_all(&self, msg: &GameMessage) {
            match self.list_players() {
                Ok(list) => {
                    for player in list {
                        self.put_msg_to(msg.clone(), &player);
                    }
                }
                Err(_) => {
                    error!("Cannot put GameMessage with no players.");
                }
            }
        }

        pub fn pop_a_msg(&self, player: &PlayerInfo) -> Option<ControlMessage> {
            match self.read_list.lock() {
                Ok(mut guard) => {
                    match guard.get_mut(&player.account.player_hash) {
                        None => { None }
                        Some(queue) => {
                            if ! queue.is_empty() {
                                queue.pop_front()
                            } else {
                                guard.remove(&player.account.player_hash);
                                None
                            }
                        }
                    }
                }
                Err(_) => {
                    error!("Cannot lock \"{:?}\" in read_list", player.account.player_hash);
                    None
                }
            }
        }

        pub fn pop_msg_or(&self, player: &PlayerInfo, or: ControlMessage) -> ControlMessage {
            self.pop_a_msg(player).unwrap_or(or)
        }

        pub fn list_received(&self, player: &PlayerInfo) -> Vec<ControlMessage> {
            match self.read_list.lock() {
                Ok(guard) => {
                    match guard.get_key_value(player.account.player_hash.as_str()) {
                        None => { Vec::new() }
                        Some(result) => {
                            convert_deque_to_vec(result.1)
                        }
                    }
                }
                Err(_) => { Vec::new() }
            }
        }
    }

    // 服务端玩家管理
    impl PadServer {

        pub fn is_player_online (&self, player: &PlayerInfo) -> bool {
            let guard = self.online_players.lock().unwrap();
            guard.contains_key(&player.account.player_hash)
        }

        fn set_player_online (&self, player: &PlayerInfo, online: bool) {
            let online_current = self.is_player_online(player);
            if online_current && !online {
                let mut guard = self.online_players.lock().unwrap();
                guard.remove(&player.account.player_hash);
                info!("{} is OFFLINE!", player.account.id);
            } else if !online_current && online {
                let mut guard = self.online_players.lock().unwrap();
                guard.insert(player.account.player_hash.clone(), player.clone());
                info!("{} is ONLINE!", player.account.id);
            }
        }

        pub fn is_player_banned (&self, player: &PlayerInfo) -> bool {
            let guard = self.banned_players.lock().unwrap();
            guard.contains_key(&player.account.player_hash)
        }

        pub fn kick_player(&self, player: &PlayerInfo) {
            if self.is_player_online(player) {
                self.put_msg_to(Leave(LeaveReason::YouAreKicked), player);
            }
        }

        pub fn ban_player(&self, player: &PlayerInfo) {
            self.set_player_banned(player, true);
            if self.is_player_online(player) {
                self.put_msg_to(Leave(LeaveReason::YouAreBanned), player);
            }
        }

        pub fn pardon_player(&self, player: &PlayerInfo) {
            self.set_player_banned(player, false);
        }

        fn set_player_banned (&self, player: &PlayerInfo, banned: bool) {
            let banned_current = self.is_player_banned(player);
            if banned_current && !banned {
                let mut guard = self.banned_players.lock().unwrap();
                guard.remove(&player.account.player_hash);
                info!("Pardoned player {}", player.account.id);
            } else if !banned_current && banned {
                let mut guard = self.banned_players.lock().unwrap();
                guard.insert(player.account.player_hash.clone(), player.clone());
                info!("Banned player {}!", player.account.id);
            }
        }

        pub fn list_players(&self) -> Result<Vec<PlayerInfo>, PoisonError<MutexGuard<HashMap<String, PlayerInfo>>>> {
            match self.online_players.lock() {
                Ok(guard) => {
                    Ok(guard.values().cloned().collect())
                }
                Err(err) => Err(err)
            }
        }

        pub fn list_players_banned(&self) -> Result<Vec<PlayerInfo>, PoisonError<MutexGuard<HashMap<String, PlayerInfo>>>> {
            match self.banned_players.lock() {
                Ok(guard) => {
                    Ok(guard.values().cloned().collect())
                }
                Err(err) => Err(err)
            }
        }
    }

    // 服务端状态控制
    #[allow(dead_code)]
    impl PadServer {

        pub fn stop_server(&self) {
            self.put_msg_to_all(&Leave(ServerClosed));
            self.stop.store(true, SeqCst);
        }

        pub fn start_server(self: Arc<Self>) {

            // 构建 Logger
            if ! self.quiet {
                logger_build();
            }

            // 运行时
            let runtime = Self::get_runtime();

            info!("Starting \"NoGamepads Server\".");

            // 入口
            let console = self.enable_console;
            let entry = self.get_entry(console);

            // 阻塞运行
            runtime.block_on(entry);
        }

        fn get_runtime() -> Runtime {
            tokio::runtime::Builder::new_multi_thread()
                .thread_name("nogpad-server")
                .thread_stack_size(32 * 1024 * 1024)
                .enable_time()
                .enable_io()
                .build()
                .unwrap()
        }

        fn get_entry(self: Arc<Self>, debug: bool) -> impl Future<Output = ()> + Send + 'static {
            async move {
                let main_thread = spawn({
                    let client = Arc::clone(&self);
                    async move {
                        Self::main_request_thread(client).await
                    }
                });

                let background_thread = spawn({
                    let client = Arc::clone(&self);
                    async move {
                        Self::background_thread(client).await
                    }
                });

                if debug {
                    let debug_cli = spawn({
                        let client = Arc::clone(&self);
                        async move {
                            Self::process_debug_cli(client).await
                        }
                    });

                    let _ = tokio::join!(debug_cli, main_thread, background_thread);
                } else {
                    let _ = tokio::join!(main_thread, background_thread);
                }
            }
        }

        pub fn lock_game(&self) {
            self.game_locked.store(true, SeqCst);
        }

        pub fn unlock_game(&self) {
            self.game_locked.store(false, SeqCst);
        }

        pub fn is_game_locked(&self) -> bool {
            self.game_locked.load(SeqCst)
        }

        async fn main_request_thread(self: Arc<Self>) {

            let addr_str = format!("{}:{}", self.address.to_string(), self.port);
            info!("Server listening at {}", addr_str);

            // Tcp 监听器
            let listener : TcpListener;
            match TcpListener::bind(&addr_str).await {
                Ok(result) => {
                    info!("Listener created.");
                    listener = result;
                }
                Err(_) => {
                    error!("Server listening at {} failed!", addr_str);
                    exit(1);
                }
            }

            // 请求信息循环
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        spawn(Self::process_request(Arc::clone(&self), stream));
                    }
                    Err(error) => {
                        error!("Error: {}", error);
                    }
                }
            }
        }

        async fn process_request(self: Arc<Self>, mut stream: TcpStream) {
            let mut buffer = [0; 1024];
            let connection_msg : ConnectionMessage = read_msg(&mut buffer, &mut stream).await;
            match connection_msg {

                // 客户端请求加入游戏，并建立长连接
                ConnectionMessage::Connection(info) => {

                    // 加入游戏资格检测
                    info!("Account {} trying to connect.", info.account.player_hash);

                    // 0. 当前游戏是否已经锁定？
                    if self.is_game_locked() {
                        // 当前游戏已经锁定，禁止加入玩家，发送失败信息，并断开连接
                        send_msg(&mut stream, ConnectionCallbackMessage::Deny(GameLocked)).await;
                        return;
                    }

                    // 1. 是否存在重复玩家？
                    let online = self.is_player_online(&info);
                    if online {
                        // 当前玩家已在线，发送失败信息，并断开连接
                        send_msg(&mut stream, ConnectionCallbackMessage::Deny(ContainSamePlayer)).await;
                        return;
                    }

                    // 2. 该玩家是否被封禁？
                    let banned = self.is_player_banned(&info);
                    if banned {
                        // 当前玩家已被封禁，发送失败信息，并断开连接
                        send_msg(&mut stream, ConnectionCallbackMessage::Deny(PlayerBanned)).await;
                        return;
                    }

                    // OK！若执行到此处，说明该玩家具有加入资格，Welcome！

                    send_msg(&mut stream, ConnectionCallbackMessage::Ok).await;
                    let callback : ConnectionMessage = read_msg(&mut buffer, &mut stream).await;

                    match callback {
                        // 玩家已就绪，发送 Welcome 信息以邀请该玩家加入游戏
                        ConnectionMessage::Ready => {
                            info!("Player \"{}\" is ready!", info.account.id);

                            // 发送 Welcome
                            send_msg(&mut stream, ConnectionCallbackMessage::Welcome).await;

                            // 注册该玩家到在线列表
                            self.set_player_online(&info, true);

                            // 启动控制循环
                            
                            spawn(Self::long_connection(Arc::clone(&self), stream, info));
                        },
                        _ => {
                            send_msg(&mut stream, ConnectionCallbackMessage::Deny(WhatTheHell)).await; // WTH ?
                        }
                    }
                }

                // 客户端请求获得游戏信息
                ConnectionMessage::RequestProfile => {
                    // 发送游戏信息到客户端
                    send_msg(&mut stream, ConnectionCallbackMessage::Profile(self.game_profile.clone())).await;
                }

                // 客户端发来了错误信息
                ConnectionMessage::Err => {
                    match stream.peer_addr() {
                        Ok(addr) => {
                            warn!("Received an error message from {}.", addr.to_string());
                        }
                        Err(_) => {
                            warn!("Received an error message from unknown pad_service.");
                        }
                    }
                }

                // 客户端发来了不相干的信息
                _ => {
                    match stream.peer_addr() {
                        Ok(addr) => {
                            warn!("Received unknown connection message from {}.", addr.to_string());
                        }
                        Err(_) => {
                            warn!("Received unknown connection message from unknown pad_service.");
                        }
                    }
                }
            }
        }

        async fn long_connection(self: Arc<Self>, stream: TcpStream, player_info: PlayerInfo) {
            let player_info_arc = Arc::new(player_info);
            let (reader, writer) = io::split(stream);
            spawn(Self::read_task(Arc::clone(&self), reader, Arc::clone(&player_info_arc)));
            spawn(Self::write_task(Arc::clone(&self), writer, Arc::clone(&player_info_arc)));
        }

        async fn read_task(self: Arc<Self>,
                           mut reader: ReadHalf<TcpStream>,
                           player_info: Arc<PlayerInfo>) {
            let player_hash = player_info.account.player_hash.clone();
            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        let msg = ControlMessage::de(buf[0..n].to_vec());
                        {
                            match self.read_list.lock() {
                                Ok(mut guard) => {
                                    info!("{:?} from {}({})", &msg, player_info.customize.nickname, player_info.account.id);
                                    guard
                                        .entry(player_hash.clone())
                                        .or_insert_with(VecDeque::new)
                                        .push_back(msg);
                                    
                                }
                                Err(_) => {
                                }
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Error reading from stream: {}", e);
                        
                        self.set_player_online(&player_info, false);

                        // 放入一条错误信息到队列，使 write_task 及时发现该玩家离开
                        self.put_msg_to(GameMessage::Err, &player_info);

                        break;
                    }
                }
            }
        }

        async fn write_task(self: Arc<Self>,
                            mut writer: WriteHalf<TcpStream>,
                            player_info: Arc<PlayerInfo>) {
            let player_hash = player_info.account.player_hash.clone();
            let mut exit = false;
            loop {
                let msg : Option<GameMessage>;
                match self.write_list.lock() {
                    Ok(mut hash_map) => {
                        if ! hash_map.is_empty() {
                            match hash_map.get_mut(&player_hash) {
                                None => {
                                    msg = None;
                                }
                                Some(queue) => {
                                    if ! queue.is_empty() {
                                        msg = queue.pop_front();
                                    } else {
                                        msg = None;
                                        hash_map.remove(&player_hash);
                                    }
                                }
                            }
                        }
                        else { msg = None; }
                    }
                    Err(_) => {
                        msg = None;
                    }
                }
                if msg.is_some() {
                    let msg = msg.unwrap();
                    match &writer.write_all(NgpdMessageEncoder::en(&msg).as_slice()).await {
                        Ok(_) => {
                            
                            info!("Sent {:?} to {}", msg, &player_info.account.id);
                        }
                        Err(error) => {
                            warn!("Sent {:?} to {} failed!", msg, &player_info.account.id);
                            warn!("{:?}", error);

                            exit = true;
                        }
                    }
                }
                if exit {
                    warn!("Long connection between \"{}\" closed.", &player_info.account.id);
                    break
                }
            }
        }

        async fn background_thread(self: Arc<Self>) {
            loop {
                // 退出程序的监听
                if self.stop.load(SeqCst) {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    
                    info!("Main thread exited.");
                    exit(0);
                }
            }
        }

        async fn process_debug_cli(self: Arc<Self>) {
            loop {
                if self.stop.load(SeqCst) {
                    info!("Debug console exited");
                    return;
                }
                tokio::time::sleep(Duration::from_secs_f64(0.2)).await;
                let option: Option<Psc> = read_cli(
                    format!("SERVER {}> ", self.address.to_string()).as_str(),
                    "psc".to_string(),
                    Psc::command()
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