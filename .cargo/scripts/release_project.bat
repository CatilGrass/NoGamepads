@echo off
setlocal enabledelayedexpansion

:: ================= 路径 =================
:: 控制台程序的 CRATE
set "CONSOLE_CRATE_DIR=console"

:: C动态链接库的 CRATE
set "DLL_CRATE_DIR=core_c"

:: 打包地址
set "RELEASE_DIR=release\dev"

:: ================= 构建 =================
:: 控制台程序
echo [INFO] Building console ...
pushd ".\%CONSOLE_CRATE_DIR%\"
cargo build --release
popd
echo
:: C动态链接库
echo [INFO] Building dll ...
pushd ".\%DLL_CRATE_DIR%\"
cargo build --release
popd

echo Done.
echo

:: =============== 发布文件 ===============
echo
echo [INFO] Generating release files
call .\.cargo\scripts\release_console\generate_console_release_file.bat

explorer.exe ".\%RELEASE_DIR%\"

echo Done.