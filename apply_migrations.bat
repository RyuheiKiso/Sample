@echo off
setlocal

REM このバッチファイルのディレクトリに移動
pushd %~dp0

REM SQLite3のパス（必要に応じて修正）
set "SQLITE3=sqlite3"

REM データベースファイル（相対パス）
set "DBFILE=code\backend\data\app.db"

REM マイグレーションSQL格納ディレクトリ（英数字のみのディレクトリ名に変更）
set "MIGDIR=docs\03_db\migration"

REM デバッグ用: パスを表示
echo CD=%CD%
echo DBFILE=%DBFILE%
echo MIGDIR=%MIGDIR%

REM データベースファイルの親ディレクトリを作成（なければ）
for %%d in ("%DBFILE%") do (
    if not exist "%%~dpd" (
        echo Creating directory: %%~dpd
        mkdir "%%~dpd"
    )
)

REM マイグレーションディレクトリが存在しない場合は作成
if not exist "%MIGDIR%" (
    echo Migration directory not found: %MIGDIR%
    echo Creating directory: %MIGDIR%
    mkdir "%MIGDIR%"
)

REM SQLファイルが存在するかチェック
dir /b "%MIGDIR%\*.sql" >nul 2>&1
if errorlevel 1 (
    echo No migration SQL files found in %MIGDIR%
    pause
    popd
    exit /b 1
)

REM マイグレーション適用
for %%f in ("%MIGDIR%\*.sql") do (
    echo Applying %%f ...
    %SQLITE3% "%DBFILE%" ".read %%f"
    if errorlevel 1 (
        echo Error applying %%f
        pause
        popd
        exit /b 1
    )
)

REM テーブル一覧を表示して確認
echo.
echo === Tables in %DBFILE% ===
%SQLITE3% "%DBFILE%" ".tables"
echo ==========================
echo All migrations applied.
pause
popd
endlocal
