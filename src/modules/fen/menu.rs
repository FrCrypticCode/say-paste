use crate::modules::{call_msgbox, get_req, lib::{
    BS_DEFPUSHBUTTON, CreateSolidBrush, CreateWindowExW, DefWindowProcW, DeleteObject, DestroyWindow, DispatchMessageW, ES_AUTOHSCROLL, Errs, GetDlgCtrlID, GetLastError, GetMessageW, GetModuleHandleW, HBRUSH, HMENU, HWND, ID_OK, LPARAM, LPSTR, LRESULT, Logs, MSG, PostQuitMessage, RegisterClassW, Req, SendMessageW, SetBkColor, SetFocus, SetTextColor, TranslateMessage, WM_COMMAND, WM_CTCOLORBTN, WM_CTCOLORSTATIC, WM_DESTROY, WM_GETTEXT, WM_SETFOCUS, WNDCLASSW, WPARAM, WS_BORDER, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_VISIBLE, load_url, rgb
}, new_link};
use std::{ffi::c_void, println, ptr::{null,null_mut}};
//static mut RESULT_TEXT: Option<String> = None;
struct App{
    hwnd_wnd: HWND,
    brush:HBRUSH,
    txtcolor: u32,
    bkgndcolor: u32,
    links:Req,
    logs:Logs
}
const ID_OPT1:usize = 151;
const ID_OPT2:usize = 152;

static mut APP: Option<App> = None;
unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_COMMAND => {
            let id = wparam & 0xFFFF;
            let ptr_app = &raw mut APP;
            let app = (*ptr_app).as_mut().unwrap();
            match id{
                ID_OPT1=>{
                    if let Some(data) = new_link(){
                        app.links.insert(data.0, data.1);
                    }
                },
                ID_OPT2=>{match get_req(){
                        Some(data)=>{
                            if app.links.contains_key(&data){
                                load_url(app.links.get(&data).unwrap());
                            }
                            else{call_msgbox("Unknown", "Pas d'URL connue");}
                        },
                        None=>{}
                    }
                },
                _=>{}
            }
            0
        },
        WM_CTCOLORSTATIC=>{
            //wparam => HDC
            //lparam => HWND
            /*let id = unsafe{GetDlgCtrlID(lparam as *mut c_void)}; => Au cas où*/
            SetTextColor(wparam as *mut c_void,rgb(245,245,245));
            SetBkColor(wparam as *mut c_void, rgb(6,0,0));
            let ptr_app = &raw mut APP;
            let app = (*ptr_app).as_mut().unwrap();
            return app.brush as isize
        }
        WM_CTCOLORBTN=>{
            let ptr_app = &raw mut APP;
            let app = (*ptr_app).as_mut().unwrap();
            return app.brush as isize
        }
        WM_DESTROY => {
            let ptr_app = &raw mut APP;
            let app = (*ptr_app).as_mut().unwrap();
            app.links.save(&mut app.logs);
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
// Générative de wnd
pub fn init(registry:Req,logs:Logs)->i32{
    unsafe{
        let ptrbrush = CreateSolidBrush(rgb(6,0,0));
        APP = Some(App{
            hwnd_wnd: null_mut(),
            brush: ptrbrush.clone(),
            txtcolor: rgb(240, 240, 240),
            bkgndcolor: rgb(5, 5, 5),
            links:registry,
            logs:logs
        });
        let ptr_app = &raw mut APP;
        let app = (*ptr_app).as_mut().unwrap();
        let instance = GetModuleHandleW(null_mut());
        let mut class_name= LPSTR::new("MenuBox");
        let ptrbrush = CreateSolidBrush(rgb(6,0,0));
        let wnd_class = WNDCLASSW{
            style:0,
            lpfnWndProc: wnd_proc,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: ptrbrush, // COLOR_WINDOW + 1
            lpszMenuName: null_mut(),
            lpszClassName: class_name.ptr(),
        };
    
        let v = RegisterClassW(&wnd_class);
        //println!("Class : {}",v);

        let mut title = LPSTR::new("GesLinks");
        app.hwnd_wnd = CreateWindowExW(0, class_name.ptr(), title.ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100, 100, 350, 180,
            null_mut(), null_mut(), instance, null_mut());
        if app.hwnd_wnd.is_null(){
            println!("Refus ?");
            println!("RegisterClassW Error : {}", GetLastError());
            return -1
        }
        
        // Bouton ADD
        let mut button_class = LPSTR::new("BUTTON");
        let mut ok_w = LPSTR::new("Ajouter");
        let handle = CreateWindowExW(
            0, button_class.ptr(), ok_w.ptr(),
            WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
            115, 25, 100, 30,
            app.hwnd_wnd, ID_OPT1 as HMENU, instance, null_mut(),
        );

        // Bouton GET
        let mut button_class = LPSTR::new("BUTTON");
        let mut ok_w = LPSTR::new("Rechercher");
        let handle = CreateWindowExW(
            0, button_class.ptr(), ok_w.ptr(),
            WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
            115, 85, 100, 30,
            app.hwnd_wnd, ID_OPT2 as HMENU, instance, null_mut(),
        );
/**/
        // Boucle de messages Win32
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        DeleteObject(app.brush);
        0
    }
}