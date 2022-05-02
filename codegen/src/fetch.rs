//We need functions to get the webpage, parse it and dump the instructions in yaml format
use std::io::{stdout, Write};
use curl::easy::Easy;
use log::{info, trace, debug, warn, error};
//use serde::{Serialize,Deserialize};

use crate::options::{Time,Instruction};

pub fn fetch(url: Option<String>, fname: Option<String>) -> Result <(),String>  {
   
    let mut buf = Vec::new();

    let url  = url.unwrap_or(String::from("https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html"));
    let fname: String = fname.unwrap_or(String::from("instruction_list.json"));

    debug!("[-]Running fetch with url:{} and file:{}", url, fname);
    let mut page = Easy::new();
    page.url(&url).map_err(|e| e.to_string())?;
    {
        let mut transfer = page.transfer();
        transfer.write_function(|new_data| {
            buf.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    };
    

    let doc = String::from_utf8(buf).map_err(|e| e.to_string())?;
    debug!("[-]Output webpage:");
    debug!("{}",doc);

    Ok(())
     

}