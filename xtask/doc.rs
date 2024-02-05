use crate::*;



pub struct Settings {
    pub copy_thindx_files:      bool,
    pub build_examples:         bool, // TODO: move to `examples`?
    pub update_examples:        bool,
    pub update_headers:         bool,

    pub all_features_cargo_doc: (), // always true

    pub open:                   bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            copy_thindx_files:      true,
            build_examples:         true,
            update_examples:        true,
            update_headers:         true,

            all_features_cargo_doc: (),

            open:                   false,
        }
    }
}

/// `cargo doc` subcommand entry point
pub fn from_args(_args: std::env::Args, help: bool) {
    from_settings(Settings { open: help, .. Default::default() })
}

pub fn from_settings(settings: Settings) {
    if settings.copy_thindx_files   { copy_thindx_files() }
    if settings.build_examples      { run("cargo build --examples") }
    if settings.update_examples     { examples::update() }
    if settings.update_headers      { headers::update() }

    let _always : () = settings.all_features_cargo_doc;
    run(r"cargo doc --no-deps --frozen --workspace --all-features --target-dir=target\all-features");

    if settings.open                { browser::open("target/all-features/doc/thindx/index.html") }
}

