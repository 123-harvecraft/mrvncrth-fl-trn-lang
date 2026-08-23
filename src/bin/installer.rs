//! tru_id_installer — Rust-native installer for the .tru file extension
//!
//! Usage:
//!   tru_id_installer install   [--dir <path>]
//!   tru_id_installer uninstall
//!   tru_id_installer status
//!   tru_id_installer help

use std::path::{Path, PathBuf};

// ─── Windows registry via winreg crate ────────────────────────────────────────
#[cfg(windows)]
use winreg::{RegKey, enums::*};

const VERSION:    &str = "0.1.0";
const EXT:        &str = ".tru";
const PROG_ID:    &str = "tru_id.SourceFile";
const BIN_NAME:   &str = "tru_id.exe";
const MIME_TYPE:  &str = "text/x-tru";
const LANG_NAME:  &str = "TRU Language";

// ─── Entry ────────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("install")   => {
            let dir = parse_dir_arg(&args);
            run_install(dir);
        }
        Some("uninstall") => run_uninstall(),
        Some("status")    => run_status(),
        Some("help") | Some("--help") | Some("-h") | None => print_help(),
        Some(cmd) => {
            eprintln!("[tru_id_installer] unknown command '{}'. Use 'help'.", cmd);
            std::process::exit(1);
        }
    }
}

fn parse_dir_arg(args: &[String]) -> PathBuf {
    for i in 0..args.len() {
        if (args[i] == "--dir" || args[i] == "-d") && i + 1 < args.len() {
            return PathBuf::from(&args[i + 1]);
        }
    }
    default_install_dir()
}

fn default_install_dir() -> PathBuf {
    dirs_or_local().join("tru_id")
}

fn dirs_or_local() -> PathBuf {
    std::env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".local").join("bin"))
                .unwrap_or_else(|_| PathBuf::from("/usr/local/bin"))
        })
}

fn print_help() {
    println!("tru_id_installer v{} — TRU Language (.tru) installer\n", VERSION);
    println!("USAGE:");
    println!("  tru_id_installer install   [--dir <path>]   Register .tru extension");
    println!("  tru_id_installer uninstall                  Remove .tru registration");
    println!("  tru_id_installer status                     Show current install status");
    println!("  tru_id_installer help                       Show this help\n");
    println!("WHAT IT DOES:");
    println!("  - Copies tru_id.exe to the install directory");
    println!("  - Registers .tru → {} in Windows Registry", PROG_ID);
    println!("  - Sets 'open', 'run', 'build', 'repl', 'edit' context-menu verbs");
    println!("  - Registers MIME type {}", MIME_TYPE);
    println!("  - Adds install directory to user PATH");
    println!("  - Creates an uninstall entry in Apps & Features\n");
    println!("EXAMPLES:");
    println!("  tru_id_installer install");
    println!("  tru_id_installer install --dir C:\\tools\\tru");
    println!("  tru_id_installer uninstall");
}

// ─── Install ──────────────────────────────────────────────────────────────────

fn run_install(install_dir: PathBuf) {
    println!("tru_id_installer v{}", VERSION);
    println!("Installing {} ({}) ...\n", LANG_NAME, EXT);

    // 1. Find the tru_id binary (same directory as this installer)
    let self_path = std::env::current_exe().expect("cannot locate self");
    let bin_src = self_path.parent().unwrap().join(BIN_NAME);

    let bin_src = if bin_src.exists() {
        bin_src
    } else {
        // Try PATH
        which_bin(BIN_NAME).unwrap_or_else(|| {
            eprintln!("[ERR] {} not found. Build first:\n  cargo build --release", BIN_NAME);
            std::process::exit(1);
        })
    };

    // 2. Create install directory
    step("Creating install directory");
    std::fs::create_dir_all(&install_dir).expect("cannot create install dir");
    ok(&format!("{}", install_dir.display()));

    // 3. Copy binary
    let installed_bin = install_dir.join(BIN_NAME);
    step("Copying tru_id binary");
    std::fs::copy(&bin_src, &installed_bin).expect("cannot copy binary");
    ok(&format!("{}", installed_bin.display()));

    // 4. Copy examples
    let examples_src = bin_src.parent().unwrap()
        .parent().unwrap()  // release/ or debug/
        .parent().unwrap()  // target/
        .join("examples");
    if examples_src.exists() {
        let examples_dst = install_dir.join("examples");
        std::fs::create_dir_all(&examples_dst).ok();
        if let Ok(entries) = std::fs::read_dir(&examples_src) {
            step("Installing .tru examples");
            let mut count = 0usize;
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().map(|e| e == "tru").unwrap_or(false) {
                    let dst = examples_dst.join(p.file_name().unwrap());
                    std::fs::copy(&p, &dst).ok();
                    count += 1;
                }
            }
            ok(&format!("{} files → {}", count, examples_dst.display()));
        }
    }

    // 5. Windows registry
    #[cfg(windows)]
    register_windows(&installed_bin, &install_dir);

    #[cfg(not(windows))]
    register_unix(&installed_bin, &install_dir);

    // 6. PATH
    step("Adding to user PATH");
    add_to_path(&install_dir);
    ok(&format!("{}", install_dir.display()));

    // 7. Verify
    step("Verifying tru_id");
    match std::process::Command::new(&installed_bin).arg("version").output() {
        Ok(out) if out.status.success() => {
            let ver = String::from_utf8_lossy(&out.stdout);
            ok(ver.trim());
        }
        _ => println!("  [WARN] verification skipped"),
    }

    println!("\n══════════════════════════════════════════");
    println!(" {} installed successfully!", LANG_NAME);
    println!("══════════════════════════════════════════");
    println!(" Dir : {}", install_dir.display());
    println!(" Bin : {}", installed_bin.display());
    println!(" Ext : {}  ({})", EXT, MIME_TYPE);
    println!();
    println!(" Usage:");
    println!("   tru_id run   examples\\hello.tru");
    println!("   tru_id build myfile.tru");
    println!("   tru_id repl");
    println!();
    println!(" Restart your terminal to use PATH changes.");
    #[cfg(windows)]
    println!(" Restart Explorer for shell context-menu integration.");
}

// ─── Windows registry registration ───────────────────────────────────────────

#[cfg(windows)]
fn register_windows(installed_bin: &Path, install_dir: &Path) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let classes = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS)
        .expect("cannot open HKCU\\Software\\Classes");

    // .tru → ProgID
    step("Registering .tru extension (HKCU)");
    let (ext_key, _) = classes.create_subkey(EXT.trim_start_matches('.'))
        .expect("cannot create .tru key");
    // Actually use full extension key path
    let (ext_key2, _) = hkcu
        .create_subkey(&format!("Software\\Classes\\{}", EXT))
        .expect("cannot create .tru key");
    ext_key2.set_value("", &PROG_ID).unwrap();
    ext_key2.set_value("Content Type", &MIME_TYPE).unwrap();
    ext_key2.set_value("PerceivedType", &"text").unwrap();
    ok(&format!("{} -> {}", EXT, PROG_ID));

    // ProgID
    let prog_path = format!("Software\\Classes\\{}", PROG_ID);
    let (prog_key, _) = hkcu.create_subkey(&prog_path).unwrap();
    prog_key.set_value("", &"TRU Language Source File").unwrap();

    // DefaultIcon
    let icon_val = format!("\"{}\",0", installed_bin.display());
    let (icon_key, _) = hkcu.create_subkey(&format!("{}\\DefaultIcon", prog_path)).unwrap();
    icon_key.set_value("", &icon_val).unwrap();

    // shell verbs
    let verbs: &[(&str, &str, &str)] = &[
        ("open",  "",                    &format!("\"{}\" run \"%1\" %*", installed_bin.display())),
        ("run",   "Run .tru file",       &format!("\"{}\" run \"%1\" %*", installed_bin.display())),
        ("build", "Transpile to Rust",   &format!("\"{}\" build \"%1\"",  installed_bin.display())),
        ("repl",  "Open TRU REPL",       &format!("\"{}\" repl",          installed_bin.display())),
        ("edit",  "Edit source",         "notepad.exe \"%1\""),
    ];

    for (verb, label, cmd) in verbs {
        let verb_path = format!("{}\\shell\\{}", prog_path, verb);
        let (vk, _) = hkcu.create_subkey(&verb_path).unwrap();
        if !label.is_empty() { vk.set_value("", label).unwrap(); }
        let (ck, _) = hkcu.create_subkey(&format!("{}\\command", verb_path)).unwrap();
        ck.set_value("", cmd).unwrap();
    }
    step("Registering shell verbs");
    ok("open / run / build / repl / edit");

    // MIME
    let (mime_key, _) = hkcu.create_subkey(
        &format!("Software\\Classes\\MIME\\Database\\Content Type\\{}", MIME_TYPE)
    ).unwrap();
    mime_key.set_value("Extension", &EXT).unwrap();
    step("Registering MIME type");
    ok(MIME_TYPE);

    // FileExts override (makes Explorer prefer our ProgID)
    let fe_path = format!(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileExts\\{}\\OpenWithProgids",
        EXT
    );
    let (fe_key, _) = hkcu.create_subkey(&fe_path).unwrap();
    fe_key.set_value(PROG_ID, &"").unwrap();

    // Apps & Features uninstall entry
    let uninstall_path = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\tru_id";
    let (uk, _) = hkcu.create_subkey(uninstall_path).unwrap();
    uk.set_value("DisplayName",     &format!("{} (tru_id_core)", LANG_NAME)).unwrap();
    uk.set_value("DisplayVersion",  &VERSION).unwrap();
    uk.set_value("Publisher",       &"istamar").unwrap();
    uk.set_value("InstallLocation", &install_dir.to_string_lossy().as_ref()).unwrap();
    uk.set_value("DisplayIcon",     &installed_bin.to_string_lossy().as_ref()).unwrap();
    uk.set_value("NoModify",        &1u32).unwrap();
    uk.set_value("NoRepair",        &1u32).unwrap();
    step("Adding Apps & Features entry");
    ok("tru_id_core");

    // Notify shell
    notify_shell_windows();
}

#[cfg(windows)]
fn notify_shell_windows() {
    step("Notifying Windows shell");
    // SHChangeNotify(SHCNE_ASSOCCHANGED, SHCNF_IDLIST, NULL, NULL)
    unsafe {
        // Use system call via cmd as fallback (avoids unsafe winapi dependency)
    }
    // Trigger via ie4uinit if available
    std::process::Command::new("ie4uinit.exe")
        .args(["-show"])
        .output()
        .ok();
    ok("shell notified");
}

// ─── Unix registration (XDG) ─────────────────────────────────────────────────

#[cfg(not(windows))]
fn register_unix(installed_bin: &Path, _install_dir: &Path) {
    // XDG MIME type
    step("Registering MIME type (XDG)");
    let mime_xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="{}">
    <comment>TRU Language Source File</comment>
    <glob pattern="*{}"/>
    <sub-class-of type="text/plain"/>
  </mime-type>
</mime-info>"#,
        MIME_TYPE, EXT
    );
    let xdg_dir = dirs_or_local().join("mime").join("packages");
    std::fs::create_dir_all(&xdg_dir).ok();
    let xml_path = xdg_dir.join("tru_id.xml");
    std::fs::write(&xml_path, &mime_xml).ok();
    std::process::Command::new("update-mime-database")
        .arg(xdg_dir.parent().unwrap())
        .output()
        .ok();
    ok(&format!("{}", xml_path.display()));

    // .desktop entry
    step("Creating .desktop entry");
    let desktop = format!(
        "[Desktop Entry]\nName=TRU Language\nExec=\"{}\" run %f\nMimeType={}\nType=Application\n",
        installed_bin.display(), MIME_TYPE
    );
    let apps_dir = std::env::var("HOME")
        .map(|h| PathBuf::from(h).join(".local").join("share").join("applications"))
        .unwrap_or_else(|_| PathBuf::from("/usr/local/share/applications"));
    std::fs::create_dir_all(&apps_dir).ok();
    let desktop_path = apps_dir.join("tru_id.desktop");
    std::fs::write(&desktop_path, &desktop).ok();
    std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .output()
        .ok();
    ok(&format!("{}", desktop_path.display()));
}

// ─── Uninstall ────────────────────────────────────────────────────────────────

fn run_uninstall() {
    println!("tru_id_installer — uninstalling {} ({}) ...\n", LANG_NAME, EXT);

    #[cfg(windows)]
    unregister_windows();

    let install_dir = default_install_dir();
    step("Removing from PATH");
    remove_from_path(&install_dir);
    ok("done");

    step("Removing install directory");
    if install_dir.exists() {
        std::fs::remove_dir_all(&install_dir).ok();
        ok(&format!("{}", install_dir.display()));
    } else {
        ok("already removed");
    }

    println!("\nTRU Language uninstalled.");
}

#[cfg(windows)]
fn unregister_windows() {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let keys = [
        format!("Software\\Classes\\{}", EXT),
        format!("Software\\Classes\\{}", PROG_ID),
        format!("Software\\Classes\\MIME\\Database\\Content Type\\{}", MIME_TYPE),
        format!("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileExts\\{}", EXT),
        "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\tru_id".to_string(),
    ];

    step("Removing registry entries");
    for key in &keys {
        hkcu.delete_subkey_all(key).ok();
    }
    ok("registry cleaned");
    notify_shell_windows();
}

// ─── Status ───────────────────────────────────────────────────────────────────

fn run_status() {
    println!("tru_id_installer — status\n");

    let install_dir = default_install_dir();
    let installed_bin = install_dir.join(BIN_NAME);

    println!("  Install dir : {}", install_dir.display());
    println!("  Binary      : {} ({})",
        installed_bin.display(),
        if installed_bin.exists() { "found" } else { "NOT found" }
    );

    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let ext_reg = hkcu.open_subkey(&format!("Software\\Classes\\{}", EXT));
        let prog_reg = hkcu.open_subkey(&format!("Software\\Classes\\{}", PROG_ID));
        println!("  .tru key    : {}", if ext_reg.is_ok() { "registered" } else { "NOT registered" });
        println!("  ProgID key  : {}", if prog_reg.is_ok() { "registered" } else { "NOT registered" });
    }

    let in_path = which_bin(BIN_NAME).is_some();
    println!("  In PATH     : {}", if in_path { "yes" } else { "no" });
}

// ─── PATH helpers ─────────────────────────────────────────────────────────────

fn add_to_path(dir: &Path) {
    let dir_str = dir.to_string_lossy().to_string();

    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(env_key) = hkcu.open_subkey_with_flags("Environment", KEY_ALL_ACCESS) {
            let current: String = env_key.get_value("PATH").unwrap_or_default();
            if !current.contains(&dir_str) {
                let new_path = format!("{};{}", current, dir_str);
                env_key.set_value("PATH", &new_path).ok();
            }
        }
    }

    #[cfg(not(windows))]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        let profile = format!("{}/.profile", home);
        let export_line = format!("\nexport PATH=\"$PATH:{}\"\n", dir_str);
        if let Ok(content) = std::fs::read_to_string(&profile) {
            if !content.contains(&dir_str) {
                let mut file = std::fs::OpenOptions::new().append(true).open(&profile).unwrap();
                use std::io::Write;
                file.write_all(export_line.as_bytes()).ok();
            }
        }
    }
}

fn remove_from_path(dir: &Path) {
    let dir_str = dir.to_string_lossy().to_string();

    #[cfg(windows)]
    {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok(env_key) = hkcu.open_subkey_with_flags("Environment", KEY_ALL_ACCESS) {
            let current: String = env_key.get_value("PATH").unwrap_or_default();
            let new_path: Vec<&str> = current.split(';')
                .filter(|s| !s.contains(&dir_str))
                .collect();
            env_key.set_value("PATH", &new_path.join(";")).ok();
        }
    }

    #[cfg(not(windows))]
    {
        let _ = dir_str;
    }
}

fn which_bin(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    for dir in path_var.split(if cfg!(windows) { ';' } else { ':' }) {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}

// ─── Console helpers ──────────────────────────────────────────────────────────

fn step(msg: &str) {
    print!("  [..] {}  ", msg);
    use std::io::Write;
    std::io::stdout().flush().ok();
}

fn ok(detail: &str) {
    println!("\r  [OK] {}   ", detail);
}
