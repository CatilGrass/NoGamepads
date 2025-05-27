@echo off
setlocal enabledelayedexpansion

:: ================= 路径 =================
:: 打包地址
set "RELEASE_DIR=release\dev"

:: ================= 构建 =================
:: 控制台程序
echo [INFO] Building ...
cargo build --workspace --release
echo Done.
echo

:: =============== 发布文件 ===============
echo
echo [INFO] Generating release files
call .\.cargo\scripts\release_console\generate_console_release_file.bat

explorer.exe ".\%RELEASE_DIR%\"

echo Done.