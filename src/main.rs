//#![windows_subsystem = "windows"]
mod modules;
fn main() {
    // Logs Erreurs
    let mut errs = modules::lib::Logs::new();
    // Registre
    if let Some(x) = modules::lib::Req::load(&mut errs){
        modules::fen::menu::init(x,errs);
    }
    else{
        let x = modules::lib::Req::new(&mut errs);
        modules::fen::menu::init(x,errs);
    }
    
    // Injection dans fichier des erreurs
}


