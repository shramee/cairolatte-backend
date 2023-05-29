// use serde::{Deserialize, Serialize};
use serde_json::Value;
mod json_engine;
// use std::fs::File;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

use hanji::run_printer;
use hanji::utils::get_cairo_files_in_path;
fn main() {
    let out_path = PathBuf::from("out");

    let repos = vec![
        "_test",
        // "greged93/2wrds_cntrcts",
        // "bowbowzai/hello-cairo",
        // "lambdaclass/cairo-rs",
        // "enitrat/erc721-cairo1",
        // "0xs34n/starknet.js",
        // "argentlabs/starknet-build",
        // "dojoengine/dojo",
        // "starknet-edu/deploy-cairo1-demo",
        // "CeliktepeMurat/Cairo1.0_by_Examples",
        // "gyan0890/EDEN23Cairo1",
        // "ruleslabs/kass",
        // "software-mansion/protostar",
        // "gaetbout/starknet-commit-reveal",
        // "0xSpaceShard/starknet-hardhat-example",
        // "enitrat/cairo1-template",
        // "milancermak/cairo_nft",
        // "lambdaclass/starknet_in_rust",
        // "starknet-edu/starknetbook",
        // "cartridge-gg/rollyourown",
        // "topology-gg/shoshin-cairo-1",
        // "Dev43/aa-cairo1",
        // "starknet-edu/starknet-cairo-101",
        // "WTFAcademy/WTF-Cairo",
        // "BibliothecaDAO/eternum",
        // "gsgalloway/zksnark-sudokus",
        // "gizatechxyz/orion",
        // "zkLinkProtocol/zklink-starknet-contracts",
        // // "shramee/starklings-cairo1",
        // "Nadai2010/Nadai-Cairo-1.0-Sierra",
        // "TheArkProjekt/Starklane",
        // "NethermindEth/warp-plugin",
        // "BlockchainAsset/cairo-contracts",
        // "finiam/cairo-workshop",
        // "Th0rgal/erc721",
        // "glihm/cairol",
        // "auditless/suna",
        // "ExtropyIO/ZeroKnowledgeBootcamp",
        // "BibliothecaDAO/InstaSwap",
        // "Th0rgal/contract",
        // "kkrt-labs/kakarot-ssj",
        // "Seraph-Labs/Cairo-Contracts",
        // "augustbleeds/quaireaux",
        // "keep-starknet-strange/alexandria",
        // "smartcontractkit/chainlink-starknet",
        // "starkware-libs/cairo",
    ];

    create_dir_all(&out_path).unwrap();
    let mut doc_file_path = out_path.clone();
    doc_file_path.push("functions-source");
    doc_file_path.set_extension("js");
    create_dir_all(&doc_file_path.parent().unwrap()).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&doc_file_path.into_os_string().to_str().unwrap())
        .unwrap();
    let mut fn_json_docs = String::from("let function_docs = [\n");

    for (i, repo) in repos.iter().enumerate() {
        // let mut json_path = out_path.clone();
        println!("{i} Processing {repo}");
        process_files(format!("cairo-repos/{repo}"), repo, &mut fn_json_docs);
    }

    fn_json_docs.push_str("];");
    // println!("----------\n\n{fn_json_docs}\n----------");
    file.write_all(fn_json_docs.as_bytes()).unwrap();
}

fn process_files(path: String, repo: &str, docs: &mut String) {
    let repo_dir = PathBuf::from(&path);
    // remove_dir_all(&out_path).unwrap();
    let cairo_files = get_cairo_files_in_path(&repo_dir);

    for cairo_file in cairo_files.iter() {
        // println!("Processing {cairo_file:?}");
        // let rel_path = cairo_file.to_str().unwrap();
        let rel_path = &cairo_file.to_str().unwrap()[repo_dir.to_str().unwrap().len() + 1..];
        println!("{rel_path}");
        docs.push_str(&run_printer(
            cairo_file.to_str().unwrap(),
            json_engine::JSONEngine::new(repo.into(), rel_path.into()),
        ));
    }
}
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
fn untyped_example(data: &str) -> Value {
    serde_json::from_str(data).unwrap()
}
fn main2() {
    let data = r#"
	{
		"name": "John Doe",
		"age": 43,
		"phones": [
			"+44 1234567",
			"+44 2345678"
		]
	}"#;
    let v = untyped_example(data);
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
}
