use clap::{Arg, Command};
use std::process;

#[cfg(windows)]
use windows::{
    Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives},
    core::PCWSTR,
};

fn main() {
    let app = Command::new("imdisk-rs")
        .version("0.1.0-prototype")
        .about("Rust CLI prototype for ImDisk Virtual Disk Driver (Phase 1 experiment)")
        .author("ImDisk Development Team")
        .arg(Arg::new("attach")
            .short('a')
            .help("Create (attach) virtual disk")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("detach")
            .short('d')
            .help("Remove (detach) virtual disk")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("force-detach")
            .short('D')
            .help("Force remove virtual disk")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("list")
            .short('l')
            .help("List virtual disk devices")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("edit")
            .short('e')
            .help("Edit virtual disk properties")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("type")
            .short('t')
            .help("Virtual disk type")
            .value_name("TYPE")
            .value_parser(["file", "vm", "proxy"]))
        .arg(Arg::new("file")
            .short('f')
            .help("Image file to attach")
            .value_name("FILE"))
        .arg(Arg::new("mountpoint")
            .short('m')
            .help("Mount point (drive letter)")
            .value_name("DRIVE"))
        .arg(Arg::new("size")
            .short('s')
            .help("Size of virtual disk")
            .value_name("SIZE"))
        .arg(Arg::new("unit")
            .short('u')
            .help("Device unit number")
            .value_name("NUMBER"));

    let matches = app.get_matches();

    // Check for no arguments - show help
    if std::env::args().len() == 1 {
        print_help_and_exit();
    }

    // Determine operation mode
    let operation = if matches.get_flag("attach") {
        OperationMode::Create
    } else if matches.get_flag("detach") || matches.get_flag("force-detach") {
        OperationMode::Remove
    } else if matches.get_flag("list") {
        OperationMode::Query
    } else if matches.get_flag("edit") {
        OperationMode::Edit
    } else {
        print_help_and_exit();
    };

    // Execute the operation
    match execute_operation(operation, &matches) {
        Ok(exit_code) => process::exit(exit_code),
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(-1);
        }
    }
}

#[derive(Debug)]
enum OperationMode {
    Create,
    Remove,
    Query,
    Edit,
}

fn print_help_and_exit() -> ! {
    println!(
        "Control program for the ImDisk Virtual Disk Driver (Rust prototype).\n\
         For copyrights and credits, type imdisk-rs --version\n\
         \n\
         Syntax:\n\
         imdisk-rs -a -t type -m mountpoint [-f file] [-s size] [-u unit]\n\
         imdisk-rs -d|-D [-u unit | -m mountpoint]\n\
         imdisk-rs -l [-u unit | -m mountpoint]\n\
         imdisk-rs -e [-u unit | -m mountpoint] [-s size]\n\
         \n\
         -a      Attach (create) virtual disk.\n\
         -d      Detach (remove) virtual disk.\n\
         -D      Force detach virtual disk.\n\
         -l      List virtual disks.\n\
         -e      Edit virtual disk properties.\n\
         -t type Virtual disk type: file, vm, or proxy.\n\
         -f file Image file to attach.\n\
         -m drv  Mount point (drive letter).\n\
         -s size Size of virtual disk (supports K, M, G, T suffixes).\n\
         -u num  Device unit number.\n\
         \n\
         NOTE: This is a Rust prototype (Phase 1 experiment) implementing basic\n\
               functionality. For production use, please use the original C version."
    );
    process::exit(0);
}

fn execute_operation(operation: OperationMode, _matches: &clap::ArgMatches) -> Result<i32, String> {
    match operation {
        OperationMode::Create => {
            println!("Creating virtual disk...");
            // TODO: Implement device creation
            println!("NOTE: Create operation not yet implemented in this prototype");
            Ok(0)
        }
        OperationMode::Remove => {
            println!("Removing virtual disk...");
            // TODO: Implement device removal
            println!("NOTE: Remove operation not yet implemented in this prototype");
            Ok(0)
        }
        OperationMode::Query => {
            println!("Querying virtual disks...");
            
            // Demonstrate cross-platform system integration
            list_system_info()?;
            
            println!("\nNOTE: ImDisk virtual disk enumeration not yet implemented in this prototype");
            println!("      The above shows system integration capabilities of Rust CLI");
            Ok(0)
        }
        OperationMode::Edit => {
            println!("Editing virtual disk...");
            // TODO: Implement device edit
            println!("NOTE: Edit operation not yet implemented in this prototype");
            Ok(0)
        }
    }
}

/// Demonstrates system integration capabilities
/// On Windows: Would use Windows APIs for drive enumeration
/// On Linux: Shows system information for demonstration
fn list_system_info() -> Result<(), String> {
    println!("System information (demonstrating Rust system integration):");
    
    #[cfg(windows)]
    {
        unsafe {
            let drives = GetLogicalDrives();
            
            for i in 0..26 {
                if (drives & (1 << i)) != 0 {
                    let drive_letter = (b'A' + i) as char;
                    let drive_path = format!("{}:\\\0", drive_letter);
                    let drive_path_wide: Vec<u16> = drive_path.encode_utf16().collect();
                    let drive_type = GetDriveTypeW(PCWSTR(drive_path_wide.as_ptr()));
                    
                    let drive_type_str = match drive_type {
                        1 => "Removable",
                        2 => "Fixed", 
                        3 => "Network",
                        4 => "CD-ROM",
                        5 => "RAM disk",
                        _ => "Unknown",
                    };
                    
                    println!("  {}: {} drive", drive_letter, drive_type_str);
                }
            }
        }
    }
    
    #[cfg(not(windows))]
    {
        // Fallback for non-Windows platforms (like this Linux build environment)
        use std::fs;
        
        println!("  Platform: Linux (demonstration mode)");
        println!("  Architecture: {}", std::env::consts::ARCH);
        println!("  OS: {}", std::env::consts::OS);
        
        // Show mount points as proxy for Windows drives
        match fs::read_to_string("/proc/mounts") {
            Ok(mounts) => {
                println!("  Active mount points (equivalent to Windows drives):");
                for line in mounts.lines().take(5) {  // Show first 5 mounts
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 && !parts[1].starts_with("/proc") && !parts[1].starts_with("/sys") {
                        println!("    {} -> {} ({})", parts[0], parts[1], parts[2]);
                    }
                }
            }
            Err(_) => {
                println!("    (Mount information not available)");
            }
        }
        
        println!("  NOTE: On Windows, this would use GetLogicalDrives() and GetDriveTypeW() APIs");
    }
    
    Ok(())
}

// Exit codes matching the original C implementation
#[allow(dead_code)]
enum ExitCode {
    Success = 0,
    DeviceNotFound = 1,
    DeviceInaccessible = 2,
    CreateDevice = 3,
    DriverNotInstalled = 4,
    DriverWrongVersion = 5,
    DriverInaccessible = 6,
    ServiceInaccessible = 7,
    Format = 8,
    BadMountPoint = 9,
    BadSyntax = 10,
    NotEnoughMemory = 11,
    PartitionNotFound = 12,
    WrongSyntax = 13,
    NoFreeDriveLetters = 14,
    Fatal = -1,
}
