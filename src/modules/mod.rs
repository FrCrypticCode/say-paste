pub mod lib;
//pub mod wnd;
pub mod fen;
use std::{println, ptr::null_mut};
use fen::{input,set};

use crate::modules::lib::{LPSTR, MB_ICONSTOP, MB_OK, MessageBoxW};
//use wnd::entry::call_inputBox;
pub fn get_req()->Option<String>{
    input::call_inputBox("Url recherchée", "Tapez le nom du site recherché :")
}
pub fn new_link()->Option<(String,String)>{
    if let Some(rep) = set::call_insertLinkBox(){
        println!("{} : {}",rep.0,rep.1);
        Some((rep.0,rep.1))
    }
    else{
        None
    }
}

pub fn call_msgbox(title:&str,msg:&str){
    let mut msg = LPSTR::new(msg);
    let mut title = LPSTR::new(title);
    unsafe{MessageBoxW(null_mut(), msg.ptr(), title.ptr(), MB_OK|MB_ICONSTOP);}
}