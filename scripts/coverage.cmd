@setlocal && pushd "%~dp0.."

:: Configuration
@set CHANNEL=nightly
@set OPEN=0
@set SKIP_TEST=0

:: Arguments
:next-arg
@if "%~1" == "--open"      shift /1 && set "OPEN=1"      && goto :next-arg
@if "%~1" == "--skip-test" shift /1 && set "SKIP_TEST=1" && goto :next-arg
@if "%~1" == "" goto :end-of-args
@echo Unrecognized argument: %~1 (did you mean --open or --skip-test?)
@endlocal && popd && exit /b 1
:end-of-args

:: Display configuration
@set CHANNEL
@set OPEN
@set SKIP_TEST

:: Inferred
@set RUSTUP_TOOLCHAIN=%CHANNEL%
@set RUSTUP_TOOLCHAIN_DIR=
@set CARGO_TARGET_DIR=%~dp0..\target\coverage



:: Dependencies

@if EXIST "%USERPROFILE%\.rustup\toolchains\%CHANNEL%-x86_64-pc-windows-msvc" goto :skip-install-toolchain
    rustup toolchain install %RUSTUP_TOOLCHAIN%
    @if ERRORLEVEL 1 goto :cleanup
:skip-install-toolchain

@if EXIST "%USERPROFILE%\.rustup\toolchains\%CHANNEL%-x86_64-pc-windows-msvc" goto :skip-install-tools-preview
    rustup component add llvm-tools-preview
    @if ERRORLEVEL 1 goto :cleanup
:skip-install-tools-preview

@grcov --version >NUL 2>NUL && goto :skip-install-grcov
    cargo install grcov
    @if ERRORLEVEL 1 goto :cleanup
:skip-install-grcov



:: Run tests for coverage

@set RUSTFLAGS=-Zinstrument-coverage --cfg unsafe_unsound_unstable_remove_static_asserts_for_coverage
@set LLVM_PROFILE_FILE=%CARGO_TARGET_DIR%\profraw\thindx-%%p-%%m.profraw
@if "%SKIP_TEST%" == "1" goto :skip-test
    :: ⚠️ some of these files cache coverage information from expanded proc macros, and
    :: don't pick up changes to `#[no_coverage]`, even if we set CARGO_INCREMENTAL=0
    del "%CARGO_TARGET_DIR%\debug\deps\*thindx-*"
    del "%CARGO_TARGET_DIR%\debug\deps\*fuzz_*"
    del "%CARGO_TARGET_DIR%\debug\deps\*dev-*"
    :: don't include coverage from previous test runs
    rmdir "%CARGO_TARGET_DIR%\profraw" /S /Q 2>NUL
    :: ✔️ inline #[test]s
    :: ✔️ integration tests/
    :: ❌ doc tests
    cargo test --tests
:skip-test
@if ERRORLEVEL 1 goto :cleanup



:: Process coverage information

@set PATH=%USERPROFILE%\.rustup\toolchains\%CHANNEL%-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\bin;%PATH%
llvm-profdata merge -sparse "%CARGO_TARGET_DIR%\profraw\thindx-*.profraw" -o "%CARGO_TARGET_DIR%\tests.profdata"

:: For "Coverage Gutters" vscode extension
llvm-cov export "--instr-profile=%CARGO_TARGET_DIR%\tests.profdata" --format=lcov "%CARGO_TARGET_DIR%\debug\deps\thindx-*.exe" > "%CARGO_TARGET_DIR%\lcov.info"

:: For summaries in your browser of choice
grcov ^
    "%CARGO_TARGET_DIR%\lcov.info" ^
    --source-dir . ^
    --binary-path "%CARGO_TARGET_DIR%\debug" ^
    -t html --branch ^
    --ignore-not-existing ^
    --ignore dev/* ^
    --ignore thindx/build.rs ^
    --ignore thindx/tests/* ^
    --ignore thindx/src/headers/xinput.h/enumerations/* ^
    --ignore thindx/src/headers/xinput.h/flags/* ^
    --excl-line "^\s*#\[derive\(" ^
    -o "%CARGO_TARGET_DIR%\grcov"
:: XXX: we can't specify --excl-line multiple times currently... boo!
:: --excl-line "^\s*unsafe impl AsSafe" ^

:: Open said summary in your browser of choice
@if "%OPEN%" == "1" start "" "%CARGO_TARGET_DIR%\grcov\index.html"


:cleanup
@endlocal && popd



:: References
:: https://doc.rust-lang.org/cargo/reference/config.html
:: https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html
