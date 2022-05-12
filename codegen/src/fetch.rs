//! # Fetch module
//! We need functions to get the webpage, parse it and dump the instructions in yaml format
//! Inspired by https://github.com/YushiOMOTE/rgy/blob/master/codegen/src/fetcher.rs 
use curl::easy::Easy;
use log::{info, debug};
use scraper::{Html,Selector};
use scraper::element_ref::ElementRef;
//use serde::{Serialize,Deserialize};

use std::collections::HashMap;
use crate::inst_parser::{Instruction, parse_data};



lazy_static! {
    //color to instruction operand size
    static ref INSTR_HASH: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("#ff99cc",0);
        m.insert("#ffcc99",0);
        m.insert("#ccccff",8);
        m.insert("#ccffcc",16);
        m.insert("#ffff99",8);
        m.insert("#ffcccc",16);
        m.insert("#80ffff",8);
        m
    };
    static ref COUNT: usize = INSTR_HASH.len();

}

fn parse_table(table: ElementRef, op_prefix: u16) -> Result<Vec<Instruction>,String> {
    debug!("Entry for 0 in hash is {}",INSTR_HASH.get("#ffcc99").unwrap());

    let td_selector = Selector::parse("td").map_err(|_e| {"Unable to find data in tables!"})?;
    let mut tds     = table.select(&td_selector); 
    let mut out = Vec::new();
    let  (mut x, mut y) = (0,0); 
    while let Some(td) = tds.next() {
        //Skips headers
        if x > 0 && y > 0 {
            let code = ((y-1 << 4) | (x-1)) as u16 | (op_prefix << 8); 
            let line = td.inner_html();
            //it's fine even if it doesn't parse
            match parse_data( &line, code, *INSTR_HASH.get(td.value().attr("bgcolor").unwrap_or("")).unwrap_or(&0) ){
                Ok((_,d)) => {out.push(d.clone()); info!("Processed: code: {:#x}  Inst:{:?}", code, d)},
                Err(e)    => info!("Couldn't parse: {} x: {} y: {}", e.to_string(), x, y )
            };
        }
        debug!("Parsed: {:?} {}",td.value().attr("bgcolor"),td.inner_html());
        x = x+1;
        if x > 16  {
            x = 0;
            y = y+1;
        }
        
        
    }

    Ok(out)

}



pub fn fetch(url: String, fname: String) -> Result <(),String>  {
    //Learnt a lot of cool tricks from [https://github.com/YushiOMOTE/rgy/blob/master/codegen/src/fetcher.rs]
    let mut buf = Vec::new();

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
    

    let html = String::from_utf8(buf).map_err(|e| e.to_string())?;
    debug!("[-]Output webpage:");
    debug!("{}",html);

    
    let document = Html::parse_document(&html);
    let selector = Selector::parse("table").map_err(|_e| {"Selecting tables failed"})?;
    //select the tables
   
    let mut tables = document.select(&selector);

    let table1 = parse_table(tables.next().expect("No tables found!"), 0x0).map_err(|e| e.to_string())?; 
    let table2 = parse_table(tables.next().expect("No tables found!"), 0xCB).map_err(|e| e.to_string())?; 
    debug!("Table1: \n{:?}",table1);
    debug!("Table2: \n{:?}",table2);

    Ok(())     

}