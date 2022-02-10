use crate::*;
use mmrbi::*;
use std::collections::*;



pub struct Settings {
    pub test:       bool,
    pub demangle:   bool,
    pub open:       bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            test:       true,
            demangle:   true,
            open:       false,
        }
    }
}

pub fn from_args(mut args: std::env::Args) {
    let mut settings = Settings::default();

    while let Some(arg) = args.next() {
        match &*arg {
            "--open"                        => settings.open = true,

            "--test"                        => settings.test = true,
            "--no-test"                     => settings.test = false,
            "--skip-test"                   => settings.test = false,

            "--demangle"                    => settings.demangle = true,
            "--no-demangle"                 => settings.demangle = false,
            "--skip-demangle"               => settings.demangle = false,

            flag if flag.starts_with("-")   => fatal!("unrecognized flag {} (did you mean --open or --skip-test?)", flag),
            pos                             => fatal!("unexpected positional argument {:?}", pos),
        }
    }

    from_settings(settings)
}

pub fn from_settings(settings: Settings) {
    // Dependencies
    let channel = "nightly";

    let rustup = Rustup::default().or_die();
    let toolchains = rustup.toolchains();
    let toolchain = toolchains.get(channel).unwrap_or_else(|| toolchains.install(channel).or_die());
    let toolchain = toolchain.as_str();

    let toolchain_arch      = toolchain.strip_prefix(channel).unwrap().strip_prefix("-").unwrap();
    let toolchain_bin       = PathBuf::from(std::env::var_os("USERPROFILE").expect("%USERPROFILE%")).join(format!(r".rustup\toolchains\{toolchain}\lib\rustlib\{toolchain_arch}\bin"));
    let llvm_profdata_exe   = toolchain_bin.join("llvm-profdata.exe");
    let llvm_cov_exe        = toolchain_bin.join("llvm-cov.exe");

    if !llvm_profdata_exe.exists() { run(format!("rustup component add --toolchain={toolchain} llvm-tools-preview")) }
    cmd("grcov --version").stdout0().map(|_| {}).unwrap_or_else(|_| run("cargo install grcov"));



    // Run tests for coverage

    let target_dir = std::env::current_dir().unwrap().join(r"target\coverage");
    let deps    = target_dir.join(r"debug\deps");
    let profraw = target_dir.join(r"profraw");

    if settings.test {
        let _ = std::fs::create_dir_all(&deps);
        for e in std::fs::read_dir(&deps).unwrap_or_else(|err| fatal!("unable to enumerate target/coverage/debug/deps: {}", err)) {
            let e = e.unwrap_or_else(|err| fatal!("error enumerating target/coverage/debug/deps: {}", err));
            let path = e.path();
            let ty = e.file_type().unwrap_or_else(|err| fatal!("error getting file type of {}: {}", path.display(), err));
            if !ty.is_file() { continue }
            let name = e.file_name();
            let name = name.to_string_lossy();

            // ⚠️ some of these files cache coverage information from expanded proc macros, and
            // don't pick up changes to `#[no_coverage]`, even if we set CARGO_INCREMENTAL=0
            let del = "thindx- fuzz_ dev-".split(' ').any(|frag| name.contains(frag));
            if del { std::fs::remove_file(&path).unwrap_or_else(|err| fatal!("unable to delete {}: {}", path.display(), err)) }
        }
        let _ = std::fs::remove_dir_all(&profraw);

        let mut cargo_test = cmd("cargo test --tests");
        cargo_test.env("RUSTUP_TOOLCHAIN",   toolchain);
        status!("Running", "{:?}", cargo_test);
        cargo_test.env("CARGO_TARGET_DIR",   &target_dir);
        cargo_test.env("LLVM_PROFILE_FILE",  target_dir.join(r"profraw\thindx-%p-%m.profraw"));
        cargo_test.env("RUSTFLAGS",          "-Zinstrument-coverage --cfg unsafe_unsound_unstable_remove_static_asserts_for_coverage");
        cargo_test.status0().or_die();
    }



    // Process coverage information

    let mut llvm_profdata_merge = Command::new(&llvm_profdata_exe);
    llvm_profdata_merge
        .arg("merge")
        .arg("-o").arg(r"target\coverage\tests.profdata")
        .arg("-sparse")
        .arg(r"target\coverage\profraw\thindx-*.profraw")
        ;
    status!("Running", "{:?}", llvm_profdata_merge);
    llvm_profdata_merge.status0().or_die();



    // For "Coverage Gutters" vscode extension

    let mut llvm_cov_export = Command::new(&llvm_cov_exe);
    llvm_cov_export
        .arg("export")
        .arg(r"--instr-profile=target\coverage\tests.profdata")
        .arg("--format=lcov")
        .arg(r"target\coverage\debug\deps\thindx-*.exe")
        ;
    status!("Running", "{:?}", llvm_cov_export);
    let lcov_info_data = llvm_cov_export.stdout0().unwrap_or_else(|_| fatal!("error merging profraw data"));
    let lcov_info_path = r"target\coverage\lcov.info";

    status!("Processing", "{}", lcov_info_path);
    mmrbi::fs::write_if_modified_with(&lcov_info_path, |o|{
        use std::io::Write;

        let mut fns = HashSet::<String>::new();
        let mut fndas = HashMap::<String, u32>::new();
        let mut lines = lcov_info_data.lines();

        if !settings.demangle {
            while let Some(line) = lines.next() {
                writeln!(o, "{line}")?;
            }
        }

        let mut lines = lines.peekable();
        while lines.peek().is_some() {
            fndas.clear();
            while let Some(fnda) = lines.peek().and_then(|line| line.strip_prefix("FNDA:")) {
                let _ = lines.next();
                // FNDA:1,_RNvXs0_NtNtNtCsl50jtzU5cN6_6thindx8xinput_h10structures16audio_device_idsNtB5_14AudioDeviceIdsNtNtCsdQ3D4PjhuiR_4core7default7Default7defaultBb_
                if let Some((hits, mangled)) = fnda.split_once(",") {
                    match hits.parse::<u32>() {
                        Ok(hits) => {
                            let demangled = format!("{:#}", rustc_demangle::demangle(mangled));
                            *fndas.entry(demangled).or_default() += hits;
                        },
                        Err(_err) => {
                            // `hit` invalid in `FNDA:hit,symbol` ?
                            // TODO: warning?
                            writeln!(o, "FNDA:{}", fnda)?;
                        },
                    }
                } else {
                    // FN:... without a hit / symbol?
                    // TODO: warning?
                    writeln!(o, "FNDA:{}", fnda)?;
                }
            }
            for (sym, hit) in fndas.iter() {
                writeln!(o, "FNDA:{hit},{sym}")?;
            }
            fndas.clear();

            if let Some(line) = lines.next() {
                if let Some(f) = line.strip_prefix("FN:") {
                    // FN:17,_RNCNvNtNtNtCsl50jtzU5cN6_6thindx8xinput_h10structures16audio_device_ids24test_traits_for_coverage0B9_
                    // This declares a function at line 17 with a mangled name
                    if let Some((line, mangled)) = f.split_once(',') {
                        let demangled = rustc_demangle::demangle(mangled);
                        let demangled = format!("FN:{line},{demangled:#}");
                        if fns.insert(demangled.clone()) {
                            writeln!(o, "{demangled}")?;
                        }
                        // else already emitted
                    } else {
                        // FN:... without a line number / symbol?
                        // TODO: warning?
                        writeln!(o, "{line}")?;
                    }
                } else {
                    if line == "end_of_record" {
                        fns.clear();
                    }
                    writeln!(o, "{line}")?;
                }
            }
        }
        Ok(())
    }).unwrap_or_else(|err| fatal!("unable to write lcov.info: {:?}", err));



    // For summaries in your browser of choice

    let mut grcov = Command::new("grcov");
    grcov
        .arg(&lcov_info_path)
        .arg("--source-dir").arg(".")
        .arg("--binary-path").arg(target_dir.join("debug"))
        .arg("-t").arg("html")
        .arg("--branch")
        .arg("--ignore-not-existing")
        .arg("--ignore").arg("dev/*")
        .arg("--ignore").arg("thindx/build.rs")
        .arg("--ignore").arg("thindx/tests/*")
        .arg("--ignore").arg("thindx/src/headers/xinput.h/enumerations/*")
        .arg("--ignore").arg("thindx/src/headers/xinput.h/flags/*")
        .arg("--excl-line").arg(r"^\s*#\[derive\(")
        // XXX: we can't specify --excl-line multiple times currently... boo!
        // .arg("--excl-line").arg(r"^\s*unsafe impl AsSafe")
        .arg("-o").arg(target_dir.join("grcov"))
        ;
    status!("Running", "{:?}", grcov);
    grcov.status0().or_die();

    // Open said summary in your browser of choice
    if settings.open { browser::open("target/coverage/grcov/index.html") }
}



// References
// https://doc.rust-lang.org/cargo/reference/config.html
// https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html
