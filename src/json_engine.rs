use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use hanji::SyntaxGroup;
use hanji::SyntaxKind;
use hanji::SyntaxKind::*;
use hanji::SyntaxNode;

use hanji::TemplateEngine;
use serde_json::Value;

pub struct JSONEngine<'a> {
    repo: String,
    pub path: String,
    fallback_descriptions: &'a Value,
    pub templates: HashMap<String, String>,
    nodes: Vec<(SyntaxKind, String, usize)>,
    tokens: Vec<(SyntaxKind, String, String)>,
    pub ignored_nodes: HashMap<SyntaxKind, u8>,
    payload: String,
}

impl<'a> TemplateEngine for JSONEngine<'a> {
    fn init(&mut self, _db: &dyn SyntaxGroup) {}

    fn token(&mut self, description: &str, text: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let text = match kind {
            SyntaxKind::TokenNewline => ".",
            _ => text,
        };

        self.tokens.push((kind, description.into(), text.into()));
    }

    fn node_start(&mut self, description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }

        self.nodes
            .push((kind, description.to_string(), self.tokens.len()));
    }

    fn node_end(&mut self, _description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let node_tup = self.nodes.pop().unwrap();
        match kind {
            FunctionWithBody => self.process_function_doc(node_tup, node, db),
            _ => {}
        }
    }

    fn get_result(&self) -> String {
        self.payload.to_string()
    }
}

impl<'a> JSONEngine<'a> {
    pub fn new(repo: String, path: String, fallback_descriptions: &'a Value) -> Self {
        let mut ignored_nodes: HashMap<SyntaxKind, u8> = HashMap::new();
        // ignored_nodes.contains_key("");
        // ignored_nodes.insert(ItemList, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(SyntaxFile, 0);
        // ignored_nodes.insert(Trivia, 0);
        ignored_nodes.insert(TokenWhitespace, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(TokenNewline, 0);
        Self {
            repo,
            path,
            fallback_descriptions,
            templates: HashMap::new(),
            nodes: Vec::new(),
            tokens: Vec::new(),
            ignored_nodes,
            payload: "".into(),
        }
    }

    pub fn process_function_doc(
        &mut self,
        node_tup: (SyntaxKind, String, usize),
        node: &SyntaxNode,
        db: &dyn SyntaxGroup,
    ) {
        // Gets all children nodes
        let tokens = &self.tokens[node_tup.2..];
        let max_index = tokens.len();
        let mut i: usize = 0;

        let mut function_name = String::new();
        let mut function_macro = String::new();
        let mut function_comments = String::new();
        let mut function_args = String::new();
        let mut function_return = String::new();

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenSingleLineComment != *kind {
                break;
            }
            function_comments.push_str(&text.to_string().replace("//", "").trim());
            function_comments.push_str("\n");
            i += 1;
        }

        if !function_comments.is_empty() {
            function_comments = format!("\n{function_comments}\n");
        }

        while i < max_index {
            let (kind, _, desc) = &tokens[i];
            if TokenIdentifier == *kind {
                function_macro.push_str(desc);
            }
            if TokenFunction == *kind {
                i += 1;
                break;
            }

            i += 1;
        }

        function_name.push_str(&tokens[i].2);

        while i < max_index {
            let (kind, _, _) = &tokens[i];
            if TokenLParen == *kind {
                i += 1;
                break;
            }
            i += 1;
        }

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenRParen == *kind {
                i += 1;
                break;
            }
            match kind {
                TokenComma => {
                    function_args.push_str("\n");
                }
                _ => {
                    function_args.push_str(text);
                    function_args.push_str(" ");
                }
            }
            i += 1;
        }

        if !function_args.is_empty() {
            function_args = format!("\n\n#### Parameters:\n{function_args}\n");
        }

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenLBrace == *kind {
                break;
            }
            match kind {
                TokenArrow => {
                    function_return = "".into();
                }
                _ => {
                    function_return.push_str(text);
                    function_return.push(' ');
                }
            }
            i += 1;
        }

        if !function_return.is_empty() {
            function_return = format!("\n\n#### Returns:\n{function_return}\n");
        }

        let mut code = "".to_string();
        node.children(db)
            .for_each(|x| code.push_str(&x.get_text(db)));
        code = code.trim_matches('\n').to_string();

        let mut push_payload = |s: &str| self.payload.push_str(s);

        let hash = format!("{}", calculate_hash(&code));
        if &function_macro == "event" {
            return;
        }

        push_payload("  {\n");
        push_payload(&format!("    _id:'{hash}',\n"));
        push_payload(&format!("    name:'{function_name}',\n"));
        if function_comments.trim().len() == 0 {
            // @TODO get function comments from OpenAI calls
            if let Some(desc) = self.fallback_descriptions.get(hash) {
                push_payload(&format!(
                    "    desc:{},\n",
                    desc.to_string().replace("\n", "\\n")
                ));
            }
        } else {
            push_payload(&format!(
                "    desc:`{}`,\n",
                function_comments.replace("`", "\\`")
            ));
        }
        // push_payload(&format!("    code:'{code}',\n"));
        push_payload(&format!("    repo:'{}',\n", self.repo));
        push_payload(&format!("    path:'{}',\n", self.path));
        push_payload("  },\n");
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
