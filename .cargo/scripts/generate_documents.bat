@echo off
setlocal enabledelayedexpansion

:: ================= 路径 =================
set "CARGO_TARGET_DIR=.\.cargo\shared\target"
set "DOC_SOURCE_DIR=%CARGO_TARGET_DIR%\doc"
set "DOC_DEST_DIR=.\documents\cargo_doc"

:: ============== Crate 列表 ==============
set "CRATE_PATHS=core console"

:: =============== 文档路径 ===============
set "DOC_LINK_PATHS=nogamepads_core nogpadc nogpads"

:: =============== 生成文档 ===============
if not exist "%DOC_DEST_DIR%" mkdir "%DOC_DEST_DIR%"
:: cargo clean --target-dir "%CARGO_TARGET_DIR%"
for %%c in (%CRATE_PATHS%) do (
    echo [INFO] Generating documentation for crate: %%c
    cargo doc --no-deps --manifest-path "./%%c/Cargo.toml"
    if !errorlevel! neq 0 (
        echo [ERROR] Failed to generate docs for crate: %%c
        exit /b !errorlevel!
    )
)

:: ============== 清理旧文件 ==============
if exist "%DOC_DEST_DIR%" (
    echo [INFO] Removing old documents...
    rmdir /s /q "%DOC_DEST_DIR%"
)

:: ============== 复制新文件 ==============
echo [INFO] Copying new documents to %DOC_DEST_DIR%...
xcopy /E /I /Q /Y "%DOC_SOURCE_DIR%" "%DOC_DEST_DIR%"
if !errorlevel! neq 0 (
    echo [ERROR] Failed to copy documents
    exit /b !errorlevel!
)

:: ================= 清理 =================
:: Uncomment to clean generated docs from target directory
:: rmdir /s /q "%DOC_SOURCE_DIR%"

:: =============== 生成链接 ===============
echo.
echo [DOCUMENTATION LINKS]

:: 获取当前目录绝对路径（转换为URL格式）
set "CURRENT_DIR=%CD:\=/%"

:: 遍历自定义路径列表
for %%p in (%DOC_LINK_PATHS%) do (
    set "HTML_PATH=file:///%CURRENT_DIR%/documents/cargo_doc/%%p/index.html"
    echo   !HTML_PATH!
)

echo [SUCCESS] Documentation generated at: %DOC_DEST_DIR%
endlocal