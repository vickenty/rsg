use std::{fs::File, io::BufReader, io::Read};

use anyhow::Error;
use clap::Arg;
use ignore::{DirEntry, WalkState};
use sxd_document::Package;
use sxd_xpath::{evaluate_xpath, nodeset::Node, Value};
use syn::{export::ToTokens, visit::Visit};

fn main() {
    let matches = clap::App::new("rsg")
        .arg(
            Arg::with_name("expr")
                .required(true)
                .help("xpath expression to evaluate"),
        )
        .arg(
            Arg::with_name("path")
                .multiple(true)
                .help("file or glob to search"),
        )
        .get_matches();

    let xpath = matches.value_of("expr").unwrap();
    if let Some(mut paths) = matches.values_of("path") {
        let mut builder = ignore::WalkBuilder::new(paths.next().unwrap());
        for path in paths {
            builder.add(path);
        }

        let mut types_builder = ignore::types::TypesBuilder::new();
        types_builder.add_defaults();
        types_builder.select("rust");
        builder.types(types_builder.build().unwrap());

        builder
            .build_parallel()
            .run(|| Box::new(|path| match_file(path, xpath)));
    } else {
        match_reader(&mut std::io::stdin().lock(), xpath, "<stdin>").unwrap();
    }
}

fn match_file(path: Result<DirEntry, ignore::Error>, xpath: &str) -> WalkState {
    let path = match path {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            return WalkState::Quit;
        }
    };

    if let Err(e) = match_file_inner(&path, xpath) {
        eprintln!("{}: {}", path.path().to_string_lossy(), e);
    }
    WalkState::Continue
}

fn match_file_inner(path: &DirEntry, xpath: &str) -> Result<(), Error> {
    let path = path.path();
    if path.is_file() {
        let name = path.to_string_lossy();
        match_reader(&mut BufReader::new(File::open(path)?), xpath, &name)
    } else {
        Ok(())
    }
}

fn match_reader(r: &mut dyn Read, xpath: &str, name: &str) -> Result<(), Error> {
    let mut content = String::new();
    r.read_to_string(&mut content)?;
    let parse = syn::parse_file(&content)?;

    let package = Package::new();
    let mut builder = rsg::Builder::new(package.as_document());
    builder.visit_file(&parse);

    let value = evaluate_xpath(&builder.doc, &xpath)?;

    match value {
        Value::Nodeset(set) => {
            for node in set {
                match node {
                    Node::Element(el) => {
                        let tokens = builder.get(el);
                        println!("{}: {}", name, tokens.to_token_stream());
                    }
                    Node::Attribute(attr) => {
                        println!("{}: {}", name, attr.value());
                    }
                    Node::Text(text) => {
                        println!("{}: {}", name, text.text());
                    }
                    _ => {}
                }
            }
        }
        _ => println!("{}: {}", name, value.string()),
    }

    Ok(())
}
