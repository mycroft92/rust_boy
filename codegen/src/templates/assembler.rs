

lazy_static! {
    static ref inst_map: HashMap<u16 , &'static str> = {
        let mut m = HashMap::new();
// instruction mnemonic to hex code map
        %for i in insts%
        m
    };
}