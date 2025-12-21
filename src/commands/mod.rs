//! # Arabic Commands Module (وحدة الأوامر العربية)
//!
//! This module contains all built-in shell commands with Arabic names.

mod builtin;
mod filesystem;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Execute a command and return true if shell should exit
pub fn execute_command(input: &str) -> bool {
    // Parse command and arguments
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }

    let command = parts[0];
    let args = &parts[1..];

    // Match Arabic commands
    match command {
        // Exit commands
        "خروج" | "exit" | "quit" => return true,

        // Help
        "مساعدة" | "help" | "?" => show_help(),

        // Print/Echo
        "اطبع" | "echo" => cmd_echo(args),

        // Clear screen
        "امسح" | "clear" | "cls" => cmd_clear(),

        // Current directory
        "اين" | "pwd" => cmd_pwd(),

        // Change directory
        "انتقل" | "cd" => cmd_cd(args),

        // List files
        "اعرض" | "ls" | "dir" => cmd_ls(args),

        // Read file
        "اقرأ" | "cat" => cmd_cat(args),

        // Create directory
        "انشئ" | "mkdir" => cmd_mkdir(args),

        // Create file
        "المس" | "touch" => cmd_touch(args),

        // Delete
        "احذف" | "rm" => cmd_rm(args),

        // Copy
        "انسخ" | "cp" => cmd_cp(args),

        // Move
        "انقل" | "mv" => cmd_mv(args),

        // Version
        "اصدار" | "version" => show_version(),

        // External command - try to execute
        _ => execute_external(command, args),
    }

    false
}

/// Show help message (مساعدة)
fn show_help() {
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                    أوامر محيط - Ocean Commands                    ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║                                                                   ║");
    println!("║  الأوامر الأساسية (Basic Commands):                               ║");
    println!("║  ─────────────────────────────────                                ║");
    println!("║  مساعدة        │ help     │ عرض هذه المساعدة                      ║");
    println!("║  خروج          │ exit     │ الخروج من الصدفة                      ║");
    println!("║  امسح          │ clear    │ مسح الشاشة                            ║");
    println!("║  اصدار         │ version  │ عرض الإصدار                           ║");
    println!("║                                                                   ║");
    println!("║  أوامر الملفات (File Commands):                                   ║");
    println!("║  ─────────────────────────────                                    ║");
    println!("║  اطبع <نص>      │ echo     │ طباعة نص                              ║");
    println!("║  اين           │ pwd      │ المسار الحالي                         ║");
    println!("║  انتقل <مسار>   │ cd       │ الانتقال إلى مجلد                      ║");
    println!("║  اعرض [مسار]   │ ls       │ عرض الملفات                           ║");
    println!("║  اقرأ <ملف>    │ cat      │ قراءة محتوى ملف                       ║");
    println!("║  انشئ <مجلد>   │ mkdir    │ إنشاء مجلد                            ║");
    println!("║  المس <ملف>    │ touch    │ إنشاء ملف فارغ                        ║");
    println!("║  احذف <ملف>    │ rm       │ حذف ملف                               ║");
    println!("║  انسخ <من> <إلى> │ cp       │ نسخ ملف                               ║");
    println!("║  انقل <من> <إلى> │ mv       │ نقل ملف                               ║");
    println!("║                                                                   ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
}

/// Show version (اصدار)
fn show_version() {
    println!("محيط (Ocean) v{}", env!("CARGO_PKG_VERSION"));
    println!("مشروع ترقيم - Tarqeem Project");
    println!("https://github.com/osama1998H/ocean");
}

/// Echo command (اطبع)
fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

/// Clear screen (امسح)
fn cmd_clear() {
    // ANSI escape code to clear screen and move cursor to top
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

/// Print working directory (اين)
fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("خطأ: لا يمكن قراءة المسار الحالي - {}", e),
    }
}

/// Change directory (انتقل)
fn cmd_cd(args: &[&str]) {
    let path = if args.is_empty() {
        // Go to home directory
        match dirs::home_dir() {
            Some(home) => home,
            None => {
                eprintln!("خطأ: لا يمكن إيجاد مجلد المنزل");
                return;
            }
        }
    } else {
        // Expand ~ to home directory
        let path_str = args[0];
        if path_str.starts_with('~') {
            match dirs::home_dir() {
                Some(home) => home.join(&path_str[1..].trim_start_matches('/')),
                None => {
                    eprintln!("خطأ: لا يمكن إيجاد مجلد المنزل");
                    return;
                }
            }
        } else {
            Path::new(path_str).to_path_buf()
        }
    };

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("خطأ: لا يمكن الانتقال إلى '{}' - {}", path.display(), e);
    }
}

/// List directory (اعرض)
fn cmd_ls(args: &[&str]) {
    let path = if args.is_empty() {
        env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
    } else {
        Path::new(args[0]).to_path_buf()
    };

    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut items: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let metadata = entry.metadata();

                let formatted = if let Ok(meta) = metadata {
                    if meta.is_dir() {
                        format!("\x1B[1;34m{}/\x1B[0m", name)  // Blue for directories
                    } else if meta.permissions().readonly() {
                        format!("\x1B[1;31m{}\x1B[0m", name)   // Red for readonly
                    } else {
                        name
                    }
                } else {
                    name
                };
                items.push(formatted);
            }

            // Sort and print
            items.sort();
            for item in items {
                println!("{}", item);
            }
        }
        Err(e) => eprintln!("خطأ: لا يمكن قراءة المجلد '{}' - {}", path.display(), e),
    }
}

/// Read file (اقرأ)
fn cmd_cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("خطأ: يرجى تحديد ملف للقراءة");
        eprintln!("الاستخدام: اقرأ <اسم_الملف>");
        return;
    }

    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => print!("{}", content),
            Err(e) => eprintln!("خطأ: لا يمكن قراءة '{}' - {}", file, e),
        }
    }
}

/// Create directory (انشئ)
fn cmd_mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("خطأ: يرجى تحديد اسم المجلد");
        eprintln!("الاستخدام: انشئ <اسم_المجلد>");
        return;
    }

    for dir in args {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("خطأ: لا يمكن إنشاء '{}' - {}", dir, e);
        }
    }
}

/// Create empty file (المس)
fn cmd_touch(args: &[&str]) {
    if args.is_empty() {
        eprintln!("خطأ: يرجى تحديد اسم الملف");
        eprintln!("الاستخدام: المس <اسم_الملف>");
        return;
    }

    for file in args {
        if let Err(e) = fs::OpenOptions::new().create(true).write(true).open(file) {
            eprintln!("خطأ: لا يمكن إنشاء '{}' - {}", file, e);
        }
    }
}

/// Delete file (احذف)
fn cmd_rm(args: &[&str]) {
    if args.is_empty() {
        eprintln!("خطأ: يرجى تحديد ملف للحذف");
        eprintln!("الاستخدام: احذف <اسم_الملف>");
        return;
    }

    for file in args {
        let path = Path::new(file);
        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            eprintln!("خطأ: لا يمكن حذف '{}' - {}", file, e);
        }
    }
}

/// Copy file (انسخ)
fn cmd_cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("خطأ: يرجى تحديد المصدر والوجهة");
        eprintln!("الاستخدام: انسخ <مصدر> <وجهة>");
        return;
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::copy(source, dest) {
        eprintln!("خطأ: لا يمكن نسخ '{}' إلى '{}' - {}", source, dest, e);
    }
}

/// Move file (انقل)
fn cmd_mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("خطأ: يرجى تحديد المصدر والوجهة");
        eprintln!("الاستخدام: انقل <مصدر> <وجهة>");
        return;
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::rename(source, dest) {
        eprintln!("خطأ: لا يمكن نقل '{}' إلى '{}' - {}", source, dest, e);
    }
}

/// Execute external command
fn execute_external(command: &str, args: &[&str]) {
    match Command::new(command).args(args).status() {
        Ok(status) => {
            if !status.success() {
                if let Some(code) = status.code() {
                    eprintln!("الأمر انتهى برمز: {}", code);
                }
            }
        }
        Err(_) => {
            eprintln!("خطأ: الأمر '{}' غير موجود", command);
            eprintln!("اكتب 'مساعدة' لعرض الأوامر المتاحة");
        }
    }
}
