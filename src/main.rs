#![windows_subsystem = "console"]
use std::{
    env,
    ffi::{c_void, OsStr},
    iter::once,
    mem::zeroed,
    os::windows::ffi::OsStrExt,
    ptr::null,
};

// =========================
// Win32 Types
// =========================
type HANDLE = *mut c_void;
type HWND = *mut c_void;

// =========================
// Constants
// =========================
const SEE_MASK_NOCLOSEPROCESS: u32 = 0x00000040;
const SEE_MASK_FLAG_NO_UI: u32 = 0x00000400;

const SW_HIDE: i32 = 0;
const SW_NORMAL: i32 = 1;
const SW_MINIMIZE: i32 = 6;
const SW_MAXIMIZE: i32 = 3;

const INFINITE: u32 = 0xffffffff;

const TOKEN_QUERY: u32 = 0x0008;
const TOKEN_ELEVATION: u32 = 20;

// =========================
// SHELLEXECUTEINFO
// =========================
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
struct SHELLEXECUTEINFOW {
    cbSize: u32,
    fMask: u32,
    hwnd: HWND,
    lpVerb: *const u16,
    lpFile: *const u16,
    lpParameters: *const u16,
    lpDirectory: *const u16,
    nShow: i32,

    hInstApp: HANDLE,

    lpIDList: *mut c_void,

    lpClass: *const u16,

    hkeyClass: HANDLE,

    dwHotKey: u32,

    hIcon: HANDLE,

    hProcess: HANDLE,
}

// =========================
// Win32 API
// =========================
#[link(name="shell32")]
unsafe extern "system" {
    fn ShellExecuteExW(
        pExecInfo: *mut SHELLEXECUTEINFOW
    ) -> i32;
}

#[link(name="kernel32")]
unsafe extern "system" {
    fn WaitForSingleObject(
        hHandle: HANDLE,
        dwMilliseconds: u32
    ) -> u32;

    fn CloseHandle(
        hObject: HANDLE
    ) -> i32;

    fn GetCurrentProcess() -> HANDLE;
}

#[link(name="advapi32")]
unsafe extern "system" {
    fn OpenProcessToken(
        ProcessHandle: HANDLE,
        DesiredAccess: u32,
        TokenHandle: *mut HANDLE
    ) -> i32;

    fn GetTokenInformation(
        TokenHandle: HANDLE,
        TokenInformationClass: u32,
        TokenInformation: *mut c_void,
        TokenInformationLength: u32,
        ReturnLength: *mut u32
    ) -> i32;
}

// =========================
// Utils
// =========================
fn to_wide(value:&OsStr)->Vec<u16>{
    value
        .encode_wide()
        .chain(once(0))
        .collect()
}

fn quote_arg(arg:&OsStr)->String{
    let s =
        arg.to_string_lossy();

    if s.contains(' ')
        || s.contains('\t')
        || s.contains('"')
    {
        format!(
            "\"{}\"",
            s.replace('"',"\\\"")
        )
    }
    else {
        s.to_string()
    }
}

fn build_parameters(args:&[String])->Option<String>{
    if args.is_empty(){
        return None;
    }

    Some(
        args.iter()
            .map(|x|
                quote_arg(OsStr::new(x))
            )
            .collect::<Vec<_>>()
            .join(" ")
    )
}

fn is_elevated()->bool{
    unsafe{
        let mut token:HANDLE =
            std::ptr::null_mut();

        if OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token
        )==0 {
            return false;
        }

        let mut elevated:u32=0;
        let mut ret_len:u32=0;

        let ok =
            GetTokenInformation(
                token,
                TOKEN_ELEVATION,
                &mut elevated as *mut u32 as *mut c_void,
                4,
                &mut ret_len
            );

        CloseHandle(token);

        ok!=0 && elevated!=0
    }
}

// =========================
// Config
// =========================
struct Config {
    verb:Option<String>,
    wait:bool,
    directory:Option<String>,
    show:i32,
    admin_check:bool,
    show_version:bool,
    help:bool,
    file:String,
    args:Vec<String>,
}


fn usage(){

println!(r#"Usage:
 {0} [-v Verb] [-w] [-d Directory] [-s WindowStyle] <Program> [Arguments...]

Options:
 -v <Verb>          Shell Verb example: open/runas/edit/print/...
 -w                 Wait for process exit
 -d <Directory>     Working directory
 -s <WindowStyle>   Normal/Hidden/Minimized/Maximized
 -a                 Check admin rights (exit 0=admin, 1=not admin)
 -V                 Show version (current: v{1})
 -h                 Show help and version

Examples:
  {0} notepad.exe
  {0} notepad.exe "C:\Documents\hello world.txt"
  {0} -w notepad.exe "C:\Documents\test.txt"
  {0} -v runas cmd.exe /k whoami
  {0} -v runas -w cmd.exe /c "whoami && pause"
  {0} -d "C:\Work" app.exe --config "dev config.json"
  {0} -s Minimized app.exe
  {0} -s Hidden cmd.exe /c "echo hello > C:\Temp\result.txt"
  {0} -- "-special-name.exe" -v child-argument
  {0} -a && echo running as admin
"#,
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_VERSION")
);
}


fn parse_style(
    value:&str
)->Result<i32,String>{

    match value.to_lowercase().as_str(){
        "normal" =>
            Ok(SW_NORMAL),

        "hidden" =>
            Ok(SW_HIDE),

        "minimized" =>
            Ok(SW_MINIMIZE),

        "maximized" =>
            Ok(SW_MAXIMIZE),

        _ =>
            Err(
                format!(
                    "invalid window style {}",
                    value
                )
            )
    }
}


fn parse_args()->Result<Config,String>{
    let args:Vec<String> =
        env::args()
            .skip(1)
            .collect();

    if args.is_empty(){
        usage();
        std::process::exit(0);
    }

    let mut index=0;

    let mut verb=None;
    let mut wait=false;
    let mut directory=None;
    let mut show=SW_NORMAL;
    let mut admin_check=false;
    let mut show_version=false;
    let mut help=false;

    while index < args.len(){

        match args[index].as_str(){

            "-v" => {
                index+=1;
                if index>=args.len(){
                    return Err(
                        "-v need value"
                        .into()
                    );
                }
                verb=
                    Some(
                        args[index].clone()
                    );
            }

            "-w" => {
                wait=true;
            }

            "-a" => {
                admin_check=true;
            }

            "-V" => {
                show_version=true;
            }

            "-h" => {
                help=true;
            }

            "-d" => {
                index+=1;
                if index>=args.len(){
                    return Err(
                        "-d need value"
                        .into()
                    );
                }
                directory=
                    Some(
                        args[index].clone()
                    );
            }

            "-s" => {
                index+=1;
                if index>=args.len(){
                    return Err(
                        "-s need value"
                        .into()
                    );
                }
                show=
                    parse_style(
                        &args[index]
                    )?;
            }

            "--" => {
                index+=1;
                break;
            }

            x if x.starts_with('-') => {
                return Err(
                    format!(
                        "unknown option {}",
                        x
                    )
                );
            }

            _ => {
                break;
            }
        }

        index+=1;
    }

    if index>=args.len(){
        if admin_check
            || show_version
            || help
        {
            return Ok(Config{
                verb,
                wait,
                directory,
                show,
                admin_check,
                show_version,
                help,
                file:
                    String::new(),
                args:
                    Vec::new(),
            });
        }
        return Err(
            "missing program"
            .into()
        );
    }

    Ok(Config{
        verb,
        wait,
        directory,
        show,
        admin_check,
        show_version,
        help,
        file:
            args[index].clone(),
        args:
            args[index+1..]
                .to_vec(),
    })
}


// =========================
// ShellExecuteEx
// =========================
fn execute(
    cfg:Config
)
->Result<(),String>{

    let file_w =
        to_wide(
            OsStr::new(
                &cfg.file
            )
        );

    let verb_w =
        cfg.verb
            .as_ref()
            .map(|x|
                to_wide(
                    OsStr::new(x)
                )
            );

    let dir_w =
        cfg.directory
            .as_ref()
            .map(|x|
                to_wide(
                    OsStr::new(x)
                )
            );

    let params =
        build_parameters(
            &cfg.args
        );

    let params_w =
        params
            .as_ref()
            .map(|x|
                to_wide(
                    OsStr::new(x)
                )
            );

    let mut info:SHELLEXECUTEINFOW =
        unsafe {
            zeroed()
        };

    info.cbSize =
        std::mem::size_of::<SHELLEXECUTEINFOW>()
            as u32;

    info.fMask =
        SEE_MASK_FLAG_NO_UI;

    if cfg.wait {
        info.fMask |=
            SEE_MASK_NOCLOSEPROCESS;
    }

    info.lpVerb =
        verb_w
        .as_ref()
        .map_or(
            null(),
            |x|x.as_ptr()
        );

    info.lpFile =
        file_w.as_ptr();

    info.lpParameters =
        params_w
        .as_ref()
        .map_or(
            null(),
            |x|x.as_ptr()
        );

    info.lpDirectory =
        dir_w
        .as_ref()
        .map_or(
            null(),
            |x|x.as_ptr()
        );

    info.nShow =
        cfg.show;

    let result =
        unsafe{
            ShellExecuteExW(
                &mut info
            )
        };

    if result==0 {
        return Err(
            format!(
                "ShellExecuteExW failed: {}",
                std::io::Error::last_os_error()
            )
        );
    }


    if cfg.wait
        && !info.hProcess.is_null()
    {

        unsafe{
            WaitForSingleObject(
                info.hProcess,
                INFINITE
            );

            CloseHandle(
                info.hProcess
            );
        }
    }

    Ok(())
}

// =========================
// main
// =========================
fn main(){

    match parse_args(){

        Ok(cfg)=>{
            if cfg.help{
                usage();
                std::process::exit(0);
            }

            if cfg.show_version{
                println!(
                    "{} {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                );
                std::process::exit(0);
            }

            if cfg.admin_check{
                let admin=is_elevated();

                println!(
                    "{}",
                    if admin{
                        "elevated"
                    }
                    else{
                        "not elevated"
                    }
                );

                std::process::exit(
                    if admin{0}else{1}
                );
            }

            if let Err(e)=execute(cfg){
                eprintln!(
                    "{}: {}",
                    env!("CARGO_PKG_NAME"),
                    e
                );
                std::process::exit(1);
            }
        }

        Err(e)=>{
            eprintln!(
                "{}: {}\n",
                env!("CARGO_PKG_NAME"),
                e
            );
            usage();
            std::process::exit(2);
        }

    }
}