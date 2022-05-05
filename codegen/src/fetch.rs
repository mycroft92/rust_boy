//We need functions to get the webpage, parse it and dump the instructions in yaml format
use curl::easy::Easy;
use log::{info, trace, debug, warn, error};
use scraper::{Html,Selector};
use scraper::element_ref::ElementRef;
//use serde::{Serialize,Deserialize};

use std::collections::HashMap;
use crate::options::{Time,Instruction};
use pest::Parser;
#[derive(Parser)]
#[grammar = "inst_grammar.pest"]
pub struct InstParser;


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

fn parse_table(table: ElementRef) {
    println!("Entry for 0 in hash is {}",INSTR_HASH.get("#ffcc99").unwrap());

    let td_selector = Selector::parse("td").expect("Unable to find data in tables!");
    let mut tds     = table.select(&td_selector); 
    
    while let Some(td) = tds.next() {
        println!("{:?} {}",td.value().attr("bgcolor"),td.inner_html());
    }


}

pub fn fetch(url: String, fname: String) -> Result <(),String>  {
   
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
    //parse one by one
    //let mut inst = Vec::new();
    while let Some(x) = tables.next()  {
        parse_table(x);

    }
    

    
    // if let Some(table) = tables.next() {
    //     //inst.extend(table);
    // }
    Ok(())     

}