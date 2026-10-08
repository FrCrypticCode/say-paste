///////////////////////////////////////////////////////////////////////
// -> Request Entry
///////////////////////////////////////////////////////////////////////
use crate::modules::lib::{
    BS_DEFPUSHBUTTON, CreateSolidBrush, CreateWindowExW, DefWindowProcW, DeleteObject, DestroyWindow, DispatchMessageW, ES_AUTOHSCROLL, GetDlgCtrlID, GetLastError, GetMessageW, GetModuleHandleW, HBRUSH, HMENU, HWND, ID_OK, LPARAM, LPSTR, LRESULT, MSG, PostQuitMessage, RegisterClassW, SendMessageW,SetTextColor ,SetBkColor, SetFocus, TranslateMessage, WM_COMMAND, WM_CTCOLORBTN, WM_CTCOLORSTATIC, WM_DESTROY, WM_GETTEXT, WM_SETFOCUS, WNDCLASSW, WPARAM, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE, rgb
};
use std::{ffi::c_void, println, ptr::{null,null_mut}};
//static mut RESULT_TEXT: Option<String> = None;
struct App{
    hwnd_wnd: HWND,
    brush:HBRUSH,
    edit_hwnd: HWND,
    result_text: Option<String>
}
const ID_EDIT:usize = 111;
const ID_LABEL:usize = 112;
const ID_BTN:usize = 113;

static mut APP: App = App{
    hwnd_wnd: null_mut(),
    brush: null_mut(),
    edit_hwnd: null_mut(),
    result_text: None
};
unsafe extern "system" fn wnd_proc_get(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let id = wparam & 0xFFFF;
            //println!("{}",id);
            if id == ID_BTN {
                // Récupération du texte saisi dans le champ EDIT
                let mut buffer = [0u16; 64];                    
                let len = SendMessageW(APP.edit_hwnd, WM_GETTEXT, buffer.len(), buffer.as_mut_ptr() as LPARAM);
                if len > 0{
                    APP.result_text = Some(String::from_utf16_lossy(&buffer[..len as usize]));
                }
                DestroyWindow(hwnd);
            }
            0
        },
        WM_CTCOLORSTATIC=>{
            //wparam => HDC
            //lparam => HWND
            /*let id = unsafe{GetDlgCtrlID(lparam as *mut c_void)}; => Au cas où*/
            SetTextColor(wparam as *mut c_void,rgb(245,245,245));
            SetBkColor(wparam as *mut c_void, rgb(6,0,0));
            return APP.brush as isize
        }
        WM_CTCOLORBTN=>{
            return APP.brush as isize
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
// Générative de wnd
pub fn call_inputBox(title:&str,prompt:&str)->Option<String>{
    unsafe{
        APP.brush = CreateSolidBrush(rgb(6,0,0));
        APP.result_text = None;
        let instance = GetModuleHandleW(null_mut());
        let mut class_name= LPSTR::new("InputBox");
        let wnd_class = WNDCLASSW{
            style:0,
            lpfnWndProc: wnd_proc_get,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: APP.brush, // COLOR_WINDOW + 1
            lpszMenuName: null_mut(),
            lpszClassName: class_name.ptr(),
        };
    
        let v = RegisterClassW(&wnd_class);
        //println!("Class : {}",v);

        let mut title = LPSTR::new("New Entry");
        APP.hwnd_wnd = CreateWindowExW(0, class_name.ptr(), title.ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100, 100, 350, 180,
            null_mut(), null_mut(), instance, null_mut());
        if APP.hwnd_wnd.is_null(){
            println!("Refus ?");
            println!("RegisterClassW Error : {}", GetLastError());
            return Some(String::from("Fenêtre refusée"))
        }
        // Label
        let mut static_class = LPSTR::new("STATIC");
        let mut prompt_w = LPSTR::new(prompt);
        let handle = CreateWindowExW(
            0, static_class.ptr(), prompt_w.ptr(),
            WS_CHILD | WS_VISIBLE,
            20, 15, 300, 20,
            APP.hwnd_wnd, ID_LABEL as HMENU, instance, null_mut(),
        );    

        // Champ texte (EDIT)
        let mut edit_class = LPSTR::new("EDIT");
        // - PlaceHolder -
        //let mut empty_w = LPSTR::new("data");
        APP.edit_hwnd = CreateWindowExW(
            0, edit_class.ptr(), null(),
            WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL,
            20, 45, 290, 25,
            APP.hwnd_wnd, ID_EDIT as HMENU, instance, null_mut(),
        );
        //SetFocus(handle);
    
        // Bouton OK
        let mut button_class = LPSTR::new("BUTTON");
        let mut ok_w = LPSTR::new("Rechercher");
        let handle = CreateWindowExW(
            0, button_class.ptr(), ok_w.ptr(),
            WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
            115, 85, 100, 30,
            APP.hwnd_wnd, ID_BTN as HMENU, instance, null_mut(),
        );
/**/
        // Boucle de messages Win32
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        let ptr = &raw mut APP.result_text;
        let valeur = std::ptr::read(ptr);
        std::ptr::write(ptr, None);
        return valeur
    }
}
