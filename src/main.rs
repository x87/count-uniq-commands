use std::collections::BTreeMap;
use std::fs;
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Attr {
    is_branch: Option<bool>,
    is_condition: Option<bool>,
    is_constructor: Option<bool>,
    is_destructor: Option<bool>,
    is_keyword: Option<bool>,
    is_nop: Option<bool>,
    is_overload: Option<bool>,
    is_segment: Option<bool>,
    is_static: Option<bool>,
    is_unsupported: Option<bool>,
    is_variadic: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Param {
    r#name: String,
    r#source: Option<String>,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    attrs: Option<Attr>,
    class: Option<String>,
    id: String,
    input: Option<Vec<Param>>,
    member: Option<String>,
    name: String,
    num_params: i32,
    output: Option<Vec<Param>>,
    short_desc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Extension {
    name: String,
    commands: Vec<Command>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    last_update: u64,
    url: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Library {
    meta: Meta,
    extensions: Vec<Extension>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let input_file = args
        .get(1)
        .unwrap_or_else(|| panic!("Provide input file name, e.g. main.txt"));

    let library_name = args
        .get(2)
        .unwrap_or_else(|| panic!("Provide library file name, e.g. data/gta3.json"));

    let content = fs::read_to_string(library_name)?;
    let library = serde_json::from_str::<Library>(content.as_str())?;

    let commands = library
        .extensions
        .iter()
        .flat_map(|ext| ext.commands.iter())
        .collect::<Vec<_>>();

    let mut map = BTreeMap::new();
    let content = fs::read_to_string(input_file)?;

    for line in content.lines() {
        if line.len() < 5 || !line[4..5].eq(":") {
            continue;
        }

        let op = format!("0{}", &line[1..4]);

        *map.entry(op).or_insert(0) += 1;
    }

    let mut v = Vec::from_iter(map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    for (k, v) in v {
        let name = if let Some(c) = commands
            .iter()
            .find(|x| x.id.eq_ignore_ascii_case(k.as_str()))
        {
            c.name.clone()
        } else {
            String::from("<name not found>")
        };
        println!("{0: <5} {1: <50} {2}", k, name, v);
    }

    Ok(())
}
