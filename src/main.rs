use std::{collections::HashMap, io::Read, path::PathBuf, str::FromStr};

use clap::{arg, Parser};
use hcl::eval::Evaluate;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long = "template")]
    template: PathBuf,

    #[arg(long = "values")]
    values: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut context = hcl::eval::Context::new();

    // Load the values.
    let mut file = std::fs::File::open(args.values).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut values: HashMap<String, serde_yaml::value::Value> =
        serde_yaml::from_str(&data).unwrap();
    values.drain().for_each(|(k, v)| {
        match v {
            serde_yaml::value::Value::Null => {
                context.declare_var(k, hcl::Value::Null);
            }
            serde_yaml::value::Value::Bool(b) => {
                context.declare_var(k, b);
            }
            serde_yaml::value::Value::Number(n) => {
                context.declare_var(k, n.as_f64().unwrap());
            }
            serde_yaml::value::Value::String(s) => {
                context.declare_var(k, s);
            }
            // serde_yaml::value::Value::Sequence(s) => {
            //     context.declare_var(k, s);
            // },
            // serde_yaml::Value::Mapping(m) => {
            //     context.declare_var(k, m);
            // },
            _ => (),
        }
    });

    // Load the template.
    let template = std::fs::read(args.template).unwrap();
    let template = std::str::from_utf8(&template).unwrap();
    let template = hcl::Template::from_str(template).unwrap();

    let rendered = template.evaluate(&context).unwrap();
    println!("{}", rendered);
}
