# No Gamepads
| ITEM        | SHIELD                                                       |
| ----------- | ------------------------------------------------------------ |
| License MIT | [![License](https://img.shields.io/github/license/CatilGrass/NoGamepads)](https://github.com/CatilGrass/NoGamepads/blob/main/LICENSE-MIT) |
| Status      | ![Development Status](https://img.shields.io/badge/status-Development_In_Progress-yellow) |



​	`No Gamepads` expands your game's control capabilities: enabling players to connect mobile devices as in-game controllers. For local multiplayer games, it eliminates the need for each player to own a physical controller, avoids crowding around a single keyboard, and removes the hassle of configuring complex input mappings – just use a smartphone!

---

​	`No Gamepads` 能够扩展您游戏的控制能力：让玩家通过移动设备作为游戏控制器进行连接。对于本地多人游戏，它使玩家无需配备独立手柄、无需挤在同一键盘前操作，也免除了配置复杂按键映射的麻烦——只需一部智能手机即可实现！



## Get Started

​	If you want to develop a project or compile and run a library using this project, please follow the steps below:

### Compile and Export Executables and Runtime Library

Clone the repository locally.

```bash
git clone git@github.com:CatilGrass/NoGamepads.git
cd NoGamepads
```

Then run `Cargo`.

```bash
# Build in development environment
cargo dev_build
# Or build in release environment
cargo release_build

# Export the project to ./export/'current_version_number'/
cargo release

# Export the project to ./export/dev/
cargo dev
```

### Run the Console Program

​	You can run the `console` program in the following ways:

Use `Cargo` to run.

```bash
cargo padc --help
```

Run directly after compiling the executable.

```bash
# Linux & Unix
cargo dev_build
cargo dev
./export/dev/bin/padc --help # TODO :: Not adapted yet
```
```powershell
:: Windows
cargo dev_build
cargo dev
.\export\dev\bin\padc.exe --help
```



## Apps

1. Console App [[Goto]](https://github.com/CatilGrass/NoGamepads/tree/main/apps/console)
2. Client App (Bevy) [[Goto]](https://github.com/CatilGrass/NoGamepads/tree/main/apps/client)



## Related Documentation

​	If you want to learn more about this, please refer to the following documentation:

1. [NoGamepads Bindings](./core/bindings/About_bindings.md)

