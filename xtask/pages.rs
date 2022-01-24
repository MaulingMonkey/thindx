use crate::*;
use std::path::*;



pub struct Settings {
    // pub create_worktree_if_missing: bool, // TODO?

    pub update_coverage:            bool,
    pub update_docs:                bool,

    pub copy_coverage:              bool,
    pub copy_docs:                  bool,

    pub git_add:                    bool,
    pub git_commit:                 bool,
    pub git_push:                   bool,

    pub open_hosted_coverage:       bool,
    pub open_hosted_docs:           bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            update_coverage:        true,
            update_docs:            true,

            copy_coverage:          true,
            copy_docs:              true,

            git_add:                true,
            git_commit:             true,
            git_push:               true,

            open_hosted_coverage:   true,
            open_hosted_docs:       true,
        }
    }
}

pub fn from_args(_args: std::env::Args) {
    from_settings(Settings::default())
}

pub fn from_settings(settings: Settings) {
    let gh_pages_dir = Path::new(".worktree/gh-pages");
    if !gh_pages_dir.exists() { return error!(".worktree/gh-pages does not exist (use `git worktree add .worktree/gh-pages github/gh-pages`?)"); }

    let _ = std::fs::create_dir_all(".worktree/gh-pages/preview/coverage");
    let _ = std::fs::create_dir_all(".worktree/gh-pages/preview/docs");

    if settings.update_docs     { doc::from_settings(doc::Settings::default()) }
    if settings.update_coverage { coverage::from_settings(coverage::Settings::default()) }

    if settings.copy_coverage   { robocopy(r"target\coverage\grcov",    r".worktree\gh-pages\preview\coverage", None) }
    if settings.copy_docs       { robocopy(r"target\all-features\doc",  r".worktree\gh-pages\preview\docs",     None) }

    if settings.git_add         { run_in(gh_pages_dir, "git add -A preview") }
    if settings.git_commit      { run_in(gh_pages_dir, "git commit -m \"cargo xtask pages\"") }
    if settings.git_push        { run_in(gh_pages_dir, "git push github gh-pages") }

    if settings.open_hosted_coverage    { browser::open("https://maulingmonkey.com/thindx/preview/coverage/") }
    if settings.open_hosted_docs        { browser::open("https://maulingmonkey.com/thindx/preview/docs/thindx/") }
}

fn robocopy<'a>(source: &str, dest: &str, patterns: impl IntoIterator<Item = &'a str>) {
    let mut cmd = Command::new("robocopy");
    cmd.arg(source);
    cmd.arg(dest);
    cmd.args(patterns);
    cmd.arg("/MIR");    // Add+delete
    cmd.arg("/NJH");    // No Job Header
    cmd.arg("/NJS");    // No Job Summary
    cmd.arg("/NP");     // No Progress (% spam)
    cmd.arg("/NFL");    // No File List (path spam)
    cmd.arg("/NDL");    // No Directory List (path spam)
    cmd.stdout(|| std::process::Stdio::null()); // *still* spams an EOL despite all those settings
    status!("Running", "{}", cmd);
    // N.B. robocopy uses nonzero for lots of success cases, so we only check for death-by-"signal"
    let _ = cmd.status().map_err(|err| error!("{}", err));
}
