@echo off
setlocal enabledelayedexpansion

:: ================= 路径 =================
:: 源文件目录
set "CONSOLE_DIR=.cargo\shared\target\release\deps"
:: 待复制文件列表
set "FILE_LIST=.cargo\scripts\release_files_win"
:: 控制台文件目标目录（基础路径）
set "BASE_DEST_DIR=release\dev\"
:: 证书文件
set "LICENSE_FILE=LICENSE-LGPL-2"
set "LICENSE_TP_FILE=LICENSE-THIRD-PARTY"

:: ============= 创建基础目录 =============
if not exist "%BASE_DEST_DIR%" mkdir "%BASE_DEST_DIR%"

:: =========== 清空目标目录内容 ===========
if exist "%BASE_DEST_DIR%" (
    echo [INFO] Cleaning target directory "%BASE_DEST_DIR%"
    :: 删除所有文件（包括隐藏/只读文件）
    del /f /s /q "%BASE_DEST_DIR%\*.*" >nul 2>&1
    :: 递归删除所有子目录
    for /d %%D in ("%BASE_DEST_DIR%\*") do rd /s /q "%%D" >nul 2>&1
)

:: ======== 读取文件列表并复制文件 ========
for /f "delims=" %%F in ('findstr /v /r "^// ^$" "%FILE_LIST%"') do (
    set "line=%%F"
    :: 分割行内容为 filename 和相对路径
    for /f "tokens=1,* delims=:" %%A in ("!line!") do (
        set "filename=%%A"
        set "rel_path=%%B"
    )
    :: 去除两端的空格
    for /f "tokens=* delims= " %%L in ("!filename!") do set "filename=%%L"
    for /f "tokens=* delims= " %%P in ("!rel_path!") do set "rel_path=%%P"
    :: 构造完整目标路径
    set "dest_path=%BASE_DEST_DIR%!rel_path:.\=!"
    :: 提取目标目录并创建
    for %%I in ("!dest_path!") do set "dest_dir=%%~dpI"
    if not exist "!dest_dir!" mkdir "!dest_dir!"
    :: 复制文件
    set "source_file=%CONSOLE_DIR%\!filename!"
    if exist "!source_file!" (
        echo [INFO] Copying "!filename!" to "!dest_path!"
        copy /Y "!source_file!" "!dest_path!" >nul
        if errorlevel 1 (
            echo [ERROR] Failed to copy "!filename!"
        ) else (
            echo [SUCCESS] Copied "!filename!"
        )
    ) else (
        echo [ERROR] Source file "!source_file!" not found
    )
)

:: ============= 复制证书文件 =============
copy /Y "%LICENSE_FILE%" "%BASE_DEST_DIR%\" >nul
copy /Y "%LICENSE_TP_FILE%" "%BASE_DEST_DIR%\" >nul

endlocal
exit /b 0