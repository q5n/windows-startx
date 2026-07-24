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

// =========================
// Config
// =========================
struct Config {
    verb:Option<String>,
    wait:bool,
    directory:Option<String>,
    show:i32,
    file:String,
    args:Vec<String>,
}


fn usage(){
println!(r#"
Usage:
 startx.exe [-v Verb] [-w] [-d Directory] [-s WindowStyle] <Program> [Arguments...]

Options:
 -v <Verb>          Shell Verb example: open/runas/edit/print/...
 -w                 Wait for process exit
 -d <Directory>     Working directory
 -s <Style>         Normal/Hidden/Minimized/Maximized

Examples:
  startx.exe notepad.exe
  startx.exe notepad.exe "C:\Documents\hello world.txt"
  startx.exe -w notepad.exe "C:\Documents\test.txt"
  startx.exe -v runas cmd.exe /k whoami
  startx.exe -v runas -w cmd.exe /c "whoami && pause"
  startx.exe -d "C:\Work" app.exe --config "dev config.json"
  startx.exe -s Minimized app.exe
  startx.exe -s Hidden cmd.exe /c "echo hello > C:\Temp\result.txt"
  startx.exe -- "-special-name.exe" -v child-argument
"#);
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
            if let Err(e)=execute(cfg){
                eprintln!(
                    "startx: {}",
                    e
                );
                std::process::exit(1);
            }
        }

        Err(e)=>{
            eprintln!(
                "startx: {}\n",
                e
            );
            usage();
            std::process::exit(2);
        }

    }
}