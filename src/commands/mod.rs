//! # Arabic Commands Module (وحدة الأوامر العربية)
//!
//! This module contains all built-in shell commands with Arabic names.
//! Each command returns a CommandResult for pipeline support.

mod builtin;
mod filesystem;

use crate::executor::CommandResult;
use crate::utils::{expand_tilde, shape_arabic};

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn execute_builtin(name: &str, args: &[&str], input: Option<&str>) -> Option<CommandResult> {
    match name {
        "خروج" | "exit" | "quit" => Some(CommandResult::Exit(0)),

        "مساعدة" | "help" | "?" => Some(cmd_help()),

        "اطبع" | "echo" => Some(cmd_echo(args, input)),

        "امسح" | "clear" | "cls" => Some(cmd_clear()),

        "اين" | "pwd" => Some(cmd_pwd()),

        "انتقل" | "cd" => Some(cmd_cd(args)),

        "اعرض" | "ls" | "dir" => Some(cmd_ls(args)),

        "اقرأ" | "cat" => Some(cmd_cat(args, input)),

        "انشئ" | "mkdir" => Some(cmd_mkdir(args)),

        "المس" | "touch" => Some(cmd_touch(args)),

        "احذف" | "rm" => Some(cmd_rm(args)),

        "انسخ" | "cp" => Some(cmd_cp(args)),

        "انقل" | "mv" => Some(cmd_mv(args)),

        "اصدار" | "version" => Some(cmd_version()),

        "ابحث" | "grep" | "search" => Some(cmd_search(args, input)),

        "صلاحيات" | "chmod" => Some(cmd_chmod(args)),

        "مالك" | "chown" => Some(cmd_chown(args)),

        "رابط" | "ln" | "link" => Some(cmd_ln(args)),

        _ => None,
    }
}


#[allow(dead_code)]
pub fn execute_command(input: &str) -> bool {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return false;
    }

    let command = parts[0];
    let args = &parts[1..];

    if let Some(result) = execute_builtin(command, args, None) {
        match result {
            CommandResult::Exit(_) => return true,
            CommandResult::Success(output) => {
                if !output.is_empty() {
                    print!("{}", output);
                }
            }
            CommandResult::Error(msg) => {
                eprintln!("{}", msg);
            }
            CommandResult::None => {}
        }
        return false;
    }

    use std::process::Command;
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

    false
}

fn cmd_help() -> CommandResult {
    let mut help = String::new();
    help.push('\n');
    help.push_str("╔═══════════════════════════════════════════════════════════════════╗\n");
    help.push_str(&format!("║                    {}                    ║\n", shape_arabic("أوامر محيط - Ocean Commands")));
    help.push_str("╠═══════════════════════════════════════════════════════════════════╣\n");
    help.push_str("║                                                                   ║\n");
    help.push_str(&format!("║  {}:                               ║\n", shape_arabic("الأوامر الأساسية (Basic Commands)")));
    help.push_str("║  ─────────────────────────────────                                ║\n");
    help.push_str(&format!("║  {}        │ help     │ {}                      ║\n", shape_arabic("مساعدة"), shape_arabic("عرض هذه المساعدة")));
    help.push_str(&format!("║  {}          │ exit     │ {}                      ║\n", shape_arabic("خروج"), shape_arabic("الخروج من الصدفة")));
    help.push_str(&format!("║  {}          │ clear    │ {}                            ║\n", shape_arabic("امسح"), shape_arabic("مسح الشاشة")));
    help.push_str(&format!("║  {}         │ version  │ {}                           ║\n", shape_arabic("اصدار"), shape_arabic("عرض الإصدار")));
    help.push_str("║                                                                   ║\n");
    help.push_str(&format!("║  {}:                                   ║\n", shape_arabic("أوامر الملفات (File Commands)")));
    help.push_str("║  ─────────────────────────────                                    ║\n");
    help.push_str(&format!("║  {} <>      │ echo     │ {}                              ║\n", shape_arabic("اطبع"), shape_arabic("طباعة نص")));
    help.push_str(&format!("║  {}           │ pwd      │ {}                         ║\n", shape_arabic("اين"), shape_arabic("المسار الحالي")));
    help.push_str(&format!("║  {} <>   │ cd       │ {}                      ║\n", shape_arabic("انتقل"), shape_arabic("الانتقال إلى مجلد")));
    help.push_str(&format!("║  {} []   │ ls       │ {}                           ║\n", shape_arabic("اعرض"), shape_arabic("عرض الملفات")));
    help.push_str(&format!("║  {} <>    │ cat      │ {}                       ║\n", shape_arabic("اقرأ"), shape_arabic("قراءة محتوى ملف")));
    help.push_str(&format!("║  {} <>   │ mkdir    │ {}                            ║\n", shape_arabic("انشئ"), shape_arabic("إنشاء مجلد")));
    help.push_str(&format!("║  {} <>    │ touch    │ {}                        ║\n", shape_arabic("المس"), shape_arabic("إنشاء ملف فارغ")));
    help.push_str(&format!("║  {} <>    │ rm       │ {}                               ║\n", shape_arabic("احذف"), shape_arabic("حذف ملف")));
    help.push_str(&format!("║  {} <> <> │ cp       │ {}                               ║\n", shape_arabic("انسخ"), shape_arabic("نسخ ملف")));
    help.push_str(&format!("║  {} <> <> │ mv       │ {}                               ║\n", shape_arabic("انقل"), shape_arabic("نقل ملف")));
    help.push_str(&format!("║  {} <>     │ grep     │ {}                         ║\n", shape_arabic("ابحث"), shape_arabic("البحث في النص")));
    help.push_str(&format!("║  {}       │ chmod    │ {}                   ║\n", shape_arabic("صلاحيات"), shape_arabic("تغيير صلاحيات الملف")));
    help.push_str(&format!("║  {} <>     │ chown    │ {}                        ║\n", shape_arabic("مالك"), shape_arabic("تغيير مالك الملف")));
    help.push_str(&format!("║  {} <>      │ ln       │ {}                            ║\n", shape_arabic("رابط"), shape_arabic("إنشاء رابط")));
    help.push_str("║                                                                   ║\n");
    help.push_str(&format!("║  {}:                                             ║\n", shape_arabic("العوامل (Operators)")));
    help.push_str("║  ─────────────────                                                ║\n");
    help.push_str(&format!("║  cmd1 | cmd2   │ {}    │ {}          ║\n", shape_arabic("أنبوب"), shape_arabic("توصيل مخرج الأول بمدخل الثاني")));
    help.push_str(&format!("║  cmd > {}     │ {}      │ {}                  ║\n", shape_arabic("ملف"), shape_arabic("إلى"), shape_arabic("كتابة المخرج إلى ملف")));
    help.push_str(&format!("║  cmd >> {}    │ {}     │ {}                  ║\n", shape_arabic("ملف"), shape_arabic("الحق"), shape_arabic("إضافة المخرج إلى ملف")));
    help.push_str(&format!("║  cmd < {}     │ {}       │ {}                   ║\n", shape_arabic("ملف"), shape_arabic("من"), shape_arabic("قراءة المدخل من ملف")));
    help.push_str(&format!("║  cmd1 && cmd2  │ {}        │ {}             ║\n", shape_arabic("و"), shape_arabic("تنفيذ الثاني إذا نجح الأول")));
    help.push_str(&format!("║  cmd1 || cmd2  │ {}       │ {}             ║\n", shape_arabic("أو"), shape_arabic("تنفيذ الثاني إذا فشل الأول")));
    help.push_str(&format!("║  cmd1 ; cmd2   │ {}       │ {}                 ║\n", shape_arabic("ثم"), shape_arabic("تنفيذ الأوامر بالترتيب")));
    help.push_str("║                                                                   ║\n");
    help.push_str("╚═══════════════════════════════════════════════════════════════════╝\n");

    println!("{}", help);
    CommandResult::None
}

fn cmd_version() -> CommandResult {
    let version = format!(
        "{}\n{}\nhttps://github.com/osama1998H/ocean\n",
        shape_arabic(&format!("محيط (Ocean) v{}", env!("CARGO_PKG_VERSION"))),
        shape_arabic("مشروع ترقيم - Tarqeem Project")
    );
    CommandResult::Success(version)
}

fn cmd_echo(args: &[&str], input: Option<&str>) -> CommandResult {
    let output = if args.is_empty() {
        if let Some(inp) = input {
            inp.to_string()
        } else {
            String::new()
        }
    } else {
        format!("{}\n", args.join(" "))
    };
    CommandResult::Success(output)
}

fn cmd_clear() -> CommandResult {
    // ANSI escape code to clear screen and move cursor to top
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
    CommandResult::None
}

fn cmd_pwd() -> CommandResult {
    match env::current_dir() {
        Ok(path) => CommandResult::Success(format!("{}\n", path.display())),
        Err(e) => CommandResult::Error(format!(
            "خطأ: لا يمكن قراءة المسار الحالي - {} / Error: Cannot read current path - {}",
            e, e
        )),
    }
}

fn cmd_cd(args: &[&str]) -> CommandResult {
    let path = if args.is_empty() {
        match dirs::home_dir() {
            Some(home) => home,
            None => {
                return CommandResult::Error(
                    "خطأ: لا يمكن إيجاد مجلد المنزل / Error: Cannot find home directory".to_string()
                );
            }
        }
    } else {
        expand_tilde(args[0])
    };

    match env::set_current_dir(&path) {
        Ok(_) => CommandResult::None,
        Err(e) => CommandResult::Error(format!(
            "خطأ: لا يمكن الانتقال إلى '{}' - {} / Error: Cannot change to '{}' - {}",
            path.display(), e, path.display(), e
        )),
    }
}

fn cmd_ls(args: &[&str]) -> CommandResult {
    use colored::Colorize;

    let path = if args.is_empty() {
        env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf())
    } else {
        expand_tilde(args[0])
    };

    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut items: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let metadata = entry.metadata();

                let formatted = if let Ok(meta) = metadata {
                    let is_dir = meta.is_dir();
                    let is_symlink = meta.file_type().is_symlink();

                    #[cfg(unix)]
                    let is_exec = {
                        use std::os::unix::fs::PermissionsExt;
                        meta.permissions().mode() & 0o111 != 0
                    };
                    #[cfg(not(unix))]
                    let is_exec = false;

                    if is_symlink {
                        // Symlinks in magenta
                        name.magenta().to_string()
                    } else if is_dir {
                        // Directories in bold blue with trailing /
                        format!("{}/", name.blue().bold())
                    } else if is_exec {
                        // Executable files in bold green
                        name.green().bold().to_string()
                    } else if meta.permissions().readonly() {
                        // Read-only files in red
                        name.red().to_string()
                    } else {
                        name
                    }
                } else {
                    name
                };
                items.push(formatted);
            }

            items.sort();
            let output = items.join("\n") + "\n";
            CommandResult::Success(output)
        }
        Err(e) => CommandResult::Error(format!(
            "خطأ: لا يمكن قراءة المجلد '{}' - {} / Error: Cannot read directory '{}' - {}",
            path.display(), e, path.display(), e
        )),
    }
}

fn cmd_cat(args: &[&str], input: Option<&str>) -> CommandResult {
    if let Some(inp) = input {
        if args.is_empty() {
            return CommandResult::Success(inp.to_string());
        }
    }

    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد ملف للقراءة\nالاستخدام: اقرأ <اسم_الملف>\nError: Please specify a file\nUsage: cat <filename>".to_string()
        );
    }

    let mut output = String::new();
    for file in args {
        match fs::read_to_string(file) {
            Ok(content) => output.push_str(&content),
            Err(e) => {
                return CommandResult::Error(format!(
                    "خطأ: لا يمكن قراءة '{}' - {} / Error: Cannot read '{}' - {}",
                    file, e, file, e
                ));
            }
        }
    }

    CommandResult::Success(output)
}

fn cmd_mkdir(args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد اسم المجلد\nالاستخدام: انشئ <اسم_المجلد>\nError: Please specify directory name\nUsage: mkdir <dirname>".to_string()
        );
    }

    for dir in args {
        if let Err(e) = fs::create_dir_all(dir) {
            return CommandResult::Error(format!(
                "خطأ: لا يمكن إنشاء '{}' - {} / Error: Cannot create '{}' - {}",
                dir, e, dir, e
            ));
        }
    }

    CommandResult::None
}

fn cmd_touch(args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد اسم الملف\nالاستخدام: المس <اسم_الملف>\nError: Please specify filename\nUsage: touch <filename>".to_string()
        );
    }

    for file in args {
        if let Err(e) = fs::OpenOptions::new().create(true).write(true).open(file) {
            return CommandResult::Error(format!(
                "خطأ: لا يمكن إنشاء '{}' - {} / Error: Cannot create '{}' - {}",
                file, e, file, e
            ));
        }
    }

    CommandResult::None
}

fn cmd_rm(args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد ملف للحذف\nالاستخدام: احذف <اسم_الملف>\nError: Please specify file to delete\nUsage: rm <filename>".to_string()
        );
    }

    for file in args {
        let path = Path::new(file);
        let result = if path.is_dir() {
            fs::remove_dir_all(path)
        } else {
            fs::remove_file(path)
        };

        if let Err(e) = result {
            return CommandResult::Error(format!(
                "خطأ: لا يمكن حذف '{}' - {} / Error: Cannot delete '{}' - {}",
                file, e, file, e
            ));
        }
    }

    CommandResult::None
}

fn cmd_cp(args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد المصدر والوجهة\nالاستخدام: انسخ <مصدر> <وجهة>\nError: Please specify source and destination\nUsage: cp <source> <dest>".to_string()
        );
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::copy(source, dest) {
        return CommandResult::Error(format!(
            "خطأ: لا يمكن نسخ '{}' إلى '{}' - {} / Error: Cannot copy '{}' to '{}' - {}",
            source, dest, e, source, dest, e
        ));
    }

    CommandResult::None
}

fn cmd_mv(args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد المصدر والوجهة\nالاستخدام: انقل <مصدر> <وجهة>\nError: Please specify source and destination\nUsage: mv <source> <dest>".to_string()
        );
    }

    let source = args[0];
    let dest = args[1];

    if let Err(e) = fs::rename(source, dest) {
        return CommandResult::Error(format!(
            "خطأ: لا يمكن نقل '{}' إلى '{}' - {} / Error: Cannot move '{}' to '{}' - {}",
            source, dest, e, source, dest, e
        ));
    }

    CommandResult::None
}

fn cmd_search(args: &[&str], input: Option<&str>) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد نص للبحث\nالاستخدام: ابحث <نمط> [ملف...]\nError: Please specify search pattern\nUsage: grep <pattern> [file...]".to_string()
        );
    }

    let pattern = args[0];

    if let Some(inp) = input {
        let matching_lines: Vec<&str> = inp
            .lines()
            .filter(|line| line.contains(pattern))
            .collect();

        if matching_lines.is_empty() {
            return CommandResult::Success(String::new());
        }
        return CommandResult::Success(matching_lines.join("\n") + "\n");
    }

    if args.len() < 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد ملف للبحث فيه أو استخدام الأنبوب\nError: Please specify a file to search or use pipe".to_string()
        );
    }

    let mut output = String::new();
    for file in &args[1..] {
        match fs::read_to_string(file) {
            Ok(content) => {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(pattern) {
                        if args.len() > 2 {
                            // Multiple files: show filename
                            output.push_str(&format!("{}:{}:{}\n", file, i + 1, line));
                        } else {
                            output.push_str(&format!("{}:{}\n", i + 1, line));
                        }
                    }
                }
            }
            Err(e) => {
                return CommandResult::Error(format!(
                    "خطأ: لا يمكن قراءة '{}' - {} / Error: Cannot read '{}' - {}",
                    file, e, file, e
                ));
            }
        }
    }

    CommandResult::Success(output)
}

#[cfg(unix)]
fn cmd_chmod(args: &[&str]) -> CommandResult {
    use std::os::unix::fs::PermissionsExt;

    if args.len() < 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد الصلاحيات والملف\nالاستخدام: صلاحيات <وضع> <ملف>\nError: Please specify mode and file\nUsage: chmod <mode> <file>".to_string()
        );
    }

    let mode_str = args[0];
    let file = args[1];

    let mode = match u32::from_str_radix(mode_str, 8) {
        Ok(m) => m,
        Err(_) => {
            return CommandResult::Error(format!(
                "خطأ: صلاحيات غير صالحة '{}' - استخدم صيغة ثمانية (مثل 755)\nError: Invalid mode '{}' - use octal format (e.g., 755)",
                mode_str, mode_str
            ));
        }
    };

    match fs::metadata(file) {
        Ok(metadata) => {
            let mut perms = metadata.permissions();
            perms.set_mode(mode);

            if let Err(e) = fs::set_permissions(file, perms) {
                return CommandResult::Error(format!(
                    "خطأ: لا يمكن تغيير صلاحيات '{}' - {} / Error: Cannot change permissions of '{}' - {}",
                    file, e, file, e
                ));
            }
            CommandResult::None
        }
        Err(e) => CommandResult::Error(format!(
            "خطأ: لا يمكن قراءة '{}' - {} / Error: Cannot read '{}' - {}",
            file, e, file, e
        )),
    }
}

#[cfg(not(unix))]
fn cmd_chmod(_args: &[&str]) -> CommandResult {
    CommandResult::Error(
        "خطأ: أمر صلاحيات غير مدعوم على هذا النظام\nError: chmod not supported on this platform".to_string()
    )
}

#[cfg(unix)]
fn cmd_chown(args: &[&str]) -> CommandResult {
    use nix::unistd::{chown, User, Group, Uid, Gid};

    if args.len() < 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد المالك والملف\nالاستخدام: مالك مستخدم[:مجموعة] ملف\nError: Please specify owner and file\nUsage: chown user[:group] file".to_string()
        );
    }

    let owner_spec = args[0];
    let file_path = expand_tilde(args[1]);

    let (user_str, group_str) = if owner_spec.contains(':') {
        let parts: Vec<&str> = owner_spec.split(':').collect();
        (parts[0], parts.get(1).copied())
    } else {
        (owner_spec, None)
    };

    let uid: Option<Uid> = if user_str.is_empty() {
        None
    } else if let Ok(uid_num) = user_str.parse::<u32>() {
        Some(Uid::from_raw(uid_num))
    } else {
        match User::from_name(user_str) {
            Ok(Some(user)) => Some(user.uid),
            Ok(None) => {
                return CommandResult::Error(format!(
                    "خطأ: المستخدم '{}' غير موجود / Error: User '{}' not found",
                    user_str, user_str
                ));
            }
            Err(e) => {
                return CommandResult::Error(format!(
                    "خطأ: فشل البحث عن المستخدم - {} / Error: Failed to lookup user - {}",
                    e, e
                ));
            }
        }
    };

    let gid: Option<Gid> = match group_str {
        Some(g) if !g.is_empty() => {
            if let Ok(gid_num) = g.parse::<u32>() {
                Some(Gid::from_raw(gid_num))
            } else {
                match Group::from_name(g) {
                    Ok(Some(group)) => Some(group.gid),
                    Ok(None) => {
                        return CommandResult::Error(format!(
                            "خطأ: المجموعة '{}' غير موجودة / Error: Group '{}' not found",
                            g, g
                        ));
                    }
                    Err(e) => {
                        return CommandResult::Error(format!(
                            "خطأ: فشل البحث عن المجموعة - {} / Error: Failed to lookup group - {}",
                            e, e
                        ));
                    }
                }
            }
        }
        _ => None,
    };

    match chown(&file_path, uid, gid) {
        Ok(_) => CommandResult::None,
        Err(e) => CommandResult::Error(format!(
            "خطأ: فشل تغيير مالك '{}' - {} / Error: Failed to change owner of '{}' - {}",
            file_path.display(), e, file_path.display(), e
        )),
    }
}

#[cfg(not(unix))]
fn cmd_chown(_args: &[&str]) -> CommandResult {
    CommandResult::Error(
        "خطأ: أمر مالك غير مدعوم على هذا النظام\nError: chown not supported on this platform".to_string()
    )
}

#[cfg(unix)]
fn cmd_ln(args: &[&str]) -> CommandResult {
    use std::os::unix::fs::symlink;
    use std::fs::hard_link;

    if args.is_empty() {
        return CommandResult::Error(
            "خطأ: يرجى تحديد المصدر والهدف\nالاستخدام: رابط [-s|-ر] مصدر هدف\nError: Please specify source and target\nUsage: ln [-s] source target".to_string()
        );
    }

    // Check for symbolic link flag (-s or -ر for Arabic)
    let (symbolic, source_idx) = if args[0] == "-s" || args[0] == "-ر" || args[0] == "--symbolic" {
        (true, 1)
    } else {
        (false, 0)
    };

    if args.len() < source_idx + 2 {
        return CommandResult::Error(
            "خطأ: يرجى تحديد المصدر والهدف\nالاستخدام: رابط [-s|-ر] مصدر هدف\nError: Please specify source and target\nUsage: ln [-s] source target".to_string()
        );
    }

    let source = expand_tilde(args[source_idx]);
    let target = expand_tilde(args[source_idx + 1]);

    let result = if symbolic {
        symlink(&source, &target)
    } else {
        hard_link(&source, &target)
    };

    match result {
        Ok(_) => CommandResult::None,
        Err(e) => {
            let link_type = if symbolic {
                "الرابط الرمزي / symbolic link"
            } else {
                "الرابط الصلب / hard link"
            };
            CommandResult::Error(format!(
                "خطأ: فشل إنشاء {} من '{}' إلى '{}' - {}\nError: Failed to create {} from '{}' to '{}' - {}",
                link_type, source.display(), target.display(), e,
                link_type, source.display(), target.display(), e
            ))
        }
    }
}

#[cfg(not(unix))]
fn cmd_ln(_args: &[&str]) -> CommandResult {
    CommandResult::Error(
        "خطأ: أمر رابط غير مدعوم على هذا النظام\nError: ln not supported on this platform".to_string()
    )
}
