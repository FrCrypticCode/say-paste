#![warn(unused_imports)]
use std::ffi::c_void;
pub use std::fs::OpenOptions;
pub use std::collections::HashMap;
use std::io::{Read, Write};
pub use std::ops::{Deref, DerefMut};
use std::fs::read_dir;
pub struct Req(HashMap<String,String>);
impl Deref for Req{
    type Target = HashMap<String,String>;
    fn deref(&self) -> &Self::Target {
        &self.0        
    }
}
impl DerefMut for Req{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Req{
    pub fn new(diary:&mut Logs)->Self{
        match OpenOptions::new().create(true).read(true).write(true).open("refs.ind"){
            Ok(f)=>{},
            Err(err)=>{println!("Erreur : {}",err.to_string());diary.add(Errs::predict(err.to_string()));}
        }
        Req(HashMap::new())
    }
    pub fn load(diary:&mut Logs)->Option<Self>{
        // Charger le fichier
        match OpenOptions::new().read(true).write(true).open("refs.ind"){
            Ok(mut f)=>{
                let mut d = Req::new(diary);
                let mut content = String::new();
                if let Err(err) = f.read_to_string(&mut content){
                    diary.add(Errs::predict(err.to_string()));
                    return None
                }
                for line in content.lines(){
                    let data:Vec<&str> = line.trim().split("|").collect();
                    d.insert(data[0].to_string(), data[1].to_string());
                }
                Some(d)
                // Load Content
            },
            Err(err)=>{
                diary.add(Errs::predict(err.to_string()));
                None
            }
        }
    }
    pub fn save(&mut self,diary:&mut Logs)->Option<String>{
        let mut check = false;
        if let Ok(dir) = read_dir("./"){
            for el in dir{
                match el{
                    Ok(x)=>{
                        let name = x.file_name().to_str().unwrap().to_string();
                        if name == "refs.ind"{check = true}
                        else{continue}
                    },
                    Err(_)=>{continue}
                }
            }
            if !check{
                if let Err(err)=OpenOptions::new().read(true).write(true).create(true).open("refs.ind"){
                    diary.add(Errs::predict(err.to_string()));
                    return Some(err.to_string())
                }
            }
            match OpenOptions::new().read(true).write(true).open("refs.ind"){
                Ok(mut file)=>{
                    for line in self.iter(){
                        let d = line.0.clone()+"|"+line.1.as_str()+"\n";
                        if let Err(err) = file.write_all(d.as_bytes()){
                            return Some(err.to_string())
                        }
                    }
                    return None
                },
                Err(err)=>{
                    return Some(err.to_string())
                }
            }   
        }
        Some(String::from("DirReader fail"))
        
    }
}
// Prévoir des mots clés refusés

// Journal d'erreurs -> Terminé

pub enum Errs{
    AccessDenied,
    Empty,
    NotFound
}
impl Errs{
    pub fn predict(err:String)->Errs{
        if err.contains("denied"){return Errs::AccessDenied}
        if err.contains("not found"){return Errs::NotFound}
        Errs::Empty
    }
}

pub struct Logs(Vec<Errs>);
impl Logs{
    pub fn new()->Self{Logs(Vec::new())}
    pub fn add(&mut self,v:Errs){self.0.push(v);}
    pub fn err(&mut self)->Option<String>{
        if let Some(nfo) = self.0.last(){
            match nfo{
                Errs::Empty=>{return Some("Aucune entrée trouvée".to_string())}
                Errs::NotFound=>{return Some("Élément non trouvé".to_string())},
                _=>{return None}
            }
        }
        self.add(Errs::Empty);
        None
        
    }
}



// Appel Clipboard -> Terminé

pub type HGLOBAL = *mut u8;
#[link(name = "user32")]
#[link(name = "kernel32")]
unsafe extern "system"{
    pub fn OpenClipboard(hwnd:*mut u8)->bool;
    pub fn SetClipboardData(uFormat:u32,hMem:HGLOBAL)->*const u8;
    pub fn GlobalAlloc(uFlags:u32,dwBytes:i32)->HGLOBAL;
    pub fn GlobalLock(hMem:HGLOBAL)->*mut u8;
    pub fn GlobalUnlock(hMem:HGLOBAL)->i32;
    pub fn EmptyClipboard()->i32;
    pub fn CloseClipboard()->i32;
}

pub const CF_TEXT:u32 = 1;
pub const CF_UNICODETEXT:u32 = 13;

pub fn load_url(url:&str){
    let mut str = url.to_string();
    if !verify(str.clone()){
        str = str+"\0";
    }
    let bytes = str.as_bytes();
    unsafe{
        let ptr = GlobalAlloc(0x0002, bytes.len() as i32);
        let origin = GlobalLock(ptr);
        for i in 0..bytes.len(){
            *(origin.add(i)) = bytes[i];
        }
        GlobalUnlock(origin);
        OpenClipboard(std::ptr::null_mut());
        EmptyClipboard();
        SetClipboardData(CF_TEXT,ptr);
        CloseClipboard();
    }
}

pub fn verify(x:String)->bool{
    if x.chars().last() == "\0".chars().next(){
        return true
    }
    false
}

// Appel Fenêtre
pub struct LPSTR(Vec<u16>);
impl Deref for LPSTR{
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0[0]
    }
}
impl LPSTR{
    pub fn new(x:&str)->Self{
        let mut value:Vec<u16> = x.encode_utf16().collect();
        if *(value.last().unwrap()) != 0{
            value.push(0);
        }
        LPSTR(value)
    }
    pub fn ptr(&mut self)->*mut u16{
        &mut self.0[0]
    }
    pub fn len(&self)->usize{
        self.0.len()
    }
    pub fn extend(&mut self,x:usize){
        self.0.resize(self.0.len()+x,0);
    }
}

// Type Win32
pub type HWND = *mut c_void;
pub type HDC = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMENU = *mut c_void;
pub type HBRUSH = *mut c_void;
pub type LRESULT = isize;
pub type WPARAM = usize;
pub type LPARAM = isize;
pub type WNDPROC = unsafe extern "system" fn(HWND,u32,WPARAM,LPARAM)->LRESULT;

// Constantes Win32
pub const WS_OVERLAPPEDWINDOW: u32 = 0x00CF0000;
pub const WS_VISIBLE: u32 = 0x10000000;
pub const WS_CHILD: u32 = 0x40000000;
pub const WS_BORDER: u32 = 0x00800000;
pub const ES_AUTOHSCROLL: u32 = 0x0080;
pub const BS_DEFPUSHBUTTON: u32 = 0x0001;

pub const WM_DESTROY: u32 = 0x0002;
pub const WM_COMMAND: u32 = 0x0111;
pub const WM_GETTEXT: u32 = 0x000D;
pub const WM_SETFOCUS: u32 = 0x0007;
pub const WM_CTCOLORSTATIC: u32 = 0x0138;
pub const WM_CTCOLORBTN: u32 = 0x0135;
pub const ID_OK: usize = 1;

#[repr(C)]
pub struct WNDCLASSW {
    pub style: u32,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: *mut c_void,
    pub hCursor: *mut c_void,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: *const u16,
    pub lpszClassName: *const u16,
}

#[repr(C)]
pub struct MSG {
    hwnd: HWND,
    message: u32,
    wParam: WPARAM,
    lParam: LPARAM,
    time: u32,
    pt_x: i32,
    pt_y: i32,
}

type DWORD = u32;
pub const fn rgb(r:u8,g:u8,b:u8)->DWORD{
    (r as u32)|(g as u32)<<8|(b as u32)<<16
}

#[link(name="gdi32")]
unsafe extern "system"{
    pub fn GetModuleHandleW(lpModuleName:*const u16)->HINSTANCE;
    pub fn RegisterClassW(lpWndClass:*const WNDCLASSW)->u16;
    pub fn CreateWindowExW(
        dwExStyle: u32, lpClassName: *const u16, lpWindowName: *const u16,
        dwStyle: u32, x: i32, y: i32, nWidth: i32, nHeight: i32,
        hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: *mut c_void,
    ) -> HWND;
    pub fn GetMessageW(lpMsg: *mut MSG, hWnd: HWND, wMsgFilterMin: u32, wMsgFilterMax: u32) -> i32;
    pub fn TranslateMessage(lpMsg: *const MSG) -> i32;
    pub fn DispatchMessageW(lpMsg: *const MSG) -> LRESULT;
    pub fn PostQuitMessage(nExitCode: i32);
    pub fn DefWindowProcW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SendMessageW(hWnd: HWND, Msg: u32, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn SetFocus(hWnd: HWND) -> HWND;
    pub fn DestroyWindow(hWnd: HWND) -> i32;
    pub fn GetLastError()->u32;
    pub fn GetDlgCtrlID(hwnd:HWND)->i32;
    pub fn CreateSolidBrush(color:DWORD)->HBRUSH;
    pub fn SetTextColor(hdc:HWND,color:DWORD)->DWORD;
    pub fn SetBkColor(hdc:HWND,color:DWORD)->DWORD;
    pub fn DeleteObject(ho:*mut c_void)->bool;
    pub fn MessageBoxW(hwnd:HWND,lpText:*mut u16,lpCaption:*mut u16,uType:usize)->i32;
}

pub const MB_OK:usize = 0x00000000;
pub const MB_ICONSTOP:usize = 0x00000010;