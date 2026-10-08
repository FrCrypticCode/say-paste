use crate::modules::lib::{
    BS_DEFPUSHBUTTON, CreateSolidBrush, CreateWindowExW, DefWindowProcW, DeleteObject, DestroyWindow, DispatchMessageW, ES_AUTOHSCROLL, GetDlgCtrlID, GetLastError, GetMessageW, GetModuleHandleW, HBRUSH, HMENU, HWND, ID_OK, LPARAM, LPSTR, LRESULT, MSG, PostQuitMessage, RegisterClassW, SendMessageW,SetTextColor ,SetBkColor, SetFocus, TranslateMessage, WM_COMMAND, WM_CTCOLORBTN, WM_CTCOLORSTATIC, WM_DESTROY, WM_GETTEXT, WM_SETFOCUS, WNDCLASSW, WPARAM, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE, rgb
};
use std::{ffi::c_void,ptr::{null,null_mut}};

struct App{
    hwnd_wnd: HWND,
    brush:HBRUSH,
    edit_hwnd1: HWND,
    edit_hwnd2: HWND,
    result_text1: Option<String>,
    result_text2: Option<String>
}

const ID_ADD:usize = 101;
const ID_CANCEL:usize = 102;
const ID_EDIT1:usize = 103;
const ID_EDIT2:usize = 104;
const ID_LABEL1:usize = 105;
const ID_LABEL2:usize = 106;

static mut APP: App = App{
    hwnd_wnd: null_mut(),
    brush: null_mut(),
    edit_hwnd1: null_mut(),
    edit_hwnd2: null_mut(),
    result_text1: None, 
    result_text2: None
};
///////////////////////////////////////////////////////////////////////
// -> Insert Link
///////////////////////////////////////////////////////////////////////
unsafe extern "system" fn wnd_proc_insert(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let id = wparam & 0xFFFF;
            match id{
                ID_ADD=>{
                    if id == ID_ADD {
                        // Récupération du texte saisi dans le champ EDIT
                        let mut buffer = [0u16; 256];                    
                        let len = SendMessageW(APP.edit_hwnd1, WM_GETTEXT, buffer.len(), buffer.as_mut_ptr() as LPARAM);
                        if len > 0{
                            APP.result_text1 = Some(String::from_utf16_lossy(&buffer[..len as usize]));
                        }
                        let len = SendMessageW(APP.edit_hwnd2, WM_GETTEXT, buffer.len(), buffer.as_mut_ptr() as LPARAM);
                        if len > 0{
                            APP.result_text2 = Some(String::from_utf16_lossy(&buffer[..len as usize]));
                        }
                        DestroyWindow(hwnd);
                    }
                },
                ID_CANCEL=>{DestroyWindow(hwnd);},
                _=>{}
            }
            0
        }
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
// => Voir pour le retour, pas adapté au contexte
pub fn call_insertLinkBox()->Option<(String,String)>{
    unsafe{
        APP.brush = CreateSolidBrush(rgb(6,0,0));
        let instance = unsafe{GetModuleHandleW(null_mut())};
        let mut class_name= LPSTR::new("LinkBox");
        let wnd_class = WNDCLASSW{
            style:0,
            lpfnWndProc: wnd_proc_insert,
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
        let mut title = LPSTR::new("New Entry");
        APP.hwnd_wnd = CreateWindowExW(0, class_name.ptr(), title.ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                100, 100, 360, 220,
                null_mut(), null_mut(), instance, null_mut());
        
        if APP.hwnd_wnd.is_null(){
            println!("Refus ?");
            return None
        }

        // A éditer pour intégrer un nouveau lien
        // Label
        let mut static_class = LPSTR::new("STATIC");
        let mut prompt_w = LPSTR::new("Votre mot clé :");
        let handle = CreateWindowExW(
            0, static_class.ptr(), prompt_w.ptr(),
            WS_CHILD | WS_VISIBLE,
            20, 15, 300, 20,
            APP.hwnd_wnd, ID_LABEL1 as HMENU, instance, null_mut(),
        );

        // Champ texte (EDIT)
        let mut edit_class = LPSTR::new("EDIT");
        // - PlaceHolder -
        //let mut empty_w = LPSTR::new("data");
        APP.edit_hwnd1 = CreateWindowExW(
            0, edit_class.ptr(), null(),
            WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL,
            20, 45,300, 25,
            APP.hwnd_wnd, ID_EDIT1 as HMENU, instance, null_mut(),
        );

        // Label
        let mut static_class = LPSTR::new("STATIC");
        let mut prompt_w = LPSTR::new("Votre URL :");
        let handle = CreateWindowExW(
            0, static_class.ptr(), prompt_w.ptr(),
            WS_CHILD | WS_VISIBLE,
            20, 80, 300, 20,
            APP.hwnd_wnd, ID_LABEL2 as HMENU, instance, null_mut(),
        );

        // Champ texte (EDIT)
        let mut edit_class = LPSTR::new("EDIT");
        // - PlaceHolder -
        //let mut empty_w = LPSTR::new("data");
        APP.edit_hwnd2 = CreateWindowExW(
            0, edit_class.ptr(), null(),
            WS_CHILD | WS_VISIBLE | WS_BORDER | ES_AUTOHSCROLL,
            20, 105, 300, 25,
            APP.hwnd_wnd, ID_EDIT2 as HMENU, instance, null_mut(),
        );

        // Bouton OK
        let mut button_class = LPSTR::new("BUTTON");
        let mut ok_w = LPSTR::new("Ajouter");
        let handle = CreateWindowExW(
            0, button_class.ptr(), ok_w.ptr(),
            WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
            90, 140, 100, 30,
            APP.hwnd_wnd, ID_ADD as HMENU, instance, null_mut(),
        );
        
        // Bouton Annuler
        let mut button_class = LPSTR::new("BUTTON");
        let mut ok_w = LPSTR::new("Annuler");
        let handle = CreateWindowExW(
            0, button_class.ptr(), ok_w.ptr(),
            WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
            210, 140, 100, 30,
            APP.hwnd_wnd, ID_CANCEL as HMENU, instance, null_mut(),
        );
        // Fenêtre générée
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        // ================================>>> Récupérer le txt
        //unsafe{Some(APP.edit_hwnd1,APP.edit_hwnd2)}
        let edit1 = &raw mut APP.result_text1;
        let edit2 = &raw mut APP.result_text2;
        let v1 = std::ptr::read(edit1);
        let v2 = std::ptr::read(edit2);
        std::ptr::write(edit1, None);
        std::ptr::write(edit2, None);
        if verify(&v1)&&verify(&v2){
            return Some((v1.unwrap(),v2.unwrap()))
        }
        None
    }
}

fn verify(x:&Option<String>)->bool{
    match x{
        Some(y)=>true,
        None=>false
    }
}