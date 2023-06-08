// Standard Lib Stuff
use std::{collections, env, fs};

pub fn env_stuff() {

    println!("tmp: {:?}", env::temp_dir());
    println!("args: {:?}", env::args().collect::<Vec<_>>());
    println!("os: {:?}", env::consts::OS);
    println!("cpu: {:?}", env::consts::ARCH);

}

pub fn hashmap_stuff(unit_size:u32, limit:u32) -> collections::HashMap<String, u32>  {
    let mut dct:collections::HashMap<String, u32> = collections::HashMap::new();
    
    let mut idx = 0;
    let mut size = unit_size;

    while size <= limit {
        dct.insert(format!("k{}", idx), size);

        idx+=1;
        size+=unit_size;
    }
    
    dct
}

pub fn read_csv(path:&str, sep:&str, as_cols:bool) -> Result<Vec<Vec<String>>, ()> {
    // read lines from path
    let file = fs::read_to_string(path).expect("");
    
    // parse lines into string vectors
    let mut rows = vec![];
    for line in file.lines() {
        let line = line
            .split(sep)
            .map(|v| v.to_owned())
            .collect::<Vec<_>>();

        rows.push(line);
    }

    // rows to cols
    if as_cols == true {
        let mut cols: Vec<Vec<String>>  = vec![];

        for idx in 0..rows[0].len() {
            cols.push(rows.iter().map(|x| x[idx].to_owned()).collect::<Vec<_>>().clone());
        }
  
        Ok(cols)    
    } 
    
    else {
        Ok(rows)
    }
}

