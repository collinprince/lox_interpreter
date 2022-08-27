use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: define_ast <output directory>");
        return;
    }

    let output_dir = args.get(1).unwrap().to_string();

    define_ast(
        output_dir,
        "Expr".to_string(),
        vec![
            "Binary : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping : Box<Expr> expression".to_string(),
            "Literal : Literal value".to_string(),
            "Unary : Token operator, Box<Expr> right".to_string(),
        ],
    );
    // Ok(())
}

// create metaprogramming for ast

pub struct TreeType {
    base_name: String,
    class_name: String,
    fields: Vec<String>,
}

impl TreeType {
    pub fn full_name(&self) -> String {
        format!("{}{}", self.class_name, self.base_name)
    }

    pub fn snake_case_full_name(&self) -> String {
        format!(
            "{}_{}",
            self.class_name.to_lowercase(),
            self.base_name.to_lowercase()
        )
    }

    pub fn to_struct(&self) -> String {
        let mut out = String::from(format!("pub struct {} {{\n", self.full_name()));
        for field in self.fields.iter() {
            let (field_type, field_name) = field.split_once(' ').unwrap();
            out.push_str(format!("\tpub {}: {},\n", field_name, field_type).as_str());
        }
        out.push_str("}\n\n");
        out
    }
}

pub fn define_ast(output_dir: String, base_name: String, types: Vec<String>) {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let path = Path::new(path.as_str());
    let display = path.display();

    // open file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let mut out = String::new();
    out.push_str("pub use crate::lexer::{Literal, Token};\n\n");
    let mut tree_types: Vec<TreeType> = Vec::new();

    for ttype in types {
        let (class_name, raw_fields) = ttype.split_once(":").unwrap();
        let mut fields = Vec::new();
        for field in raw_fields.split(",") {
            fields.push(field.trim().to_string());
        }

        tree_types.push(TreeType {
            base_name: base_name.clone(),
            class_name: class_name.trim().to_string(),
            fields,
        });
    }

    // define base enum
    out.push_str(format!("pub enum {} {{\n", base_name).as_str());
    for tree_type in tree_types.iter() {
        out.push_str(format!("\t{}({}),\n", tree_type.class_name, tree_type.full_name()).as_str());
    }
    out.push_str(format!("}}\n\n").as_str());

    // create struct for each of the rules in the base enum
    for tree_type in tree_types.iter() {
        out.push_str(tree_type.to_struct().as_str());
    }
    out.push_str(define_visitor(base_name, tree_types).as_str());

    // define visitor trait

    match file.write_all(out.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successsfully wrote to {}", display),
    }
}

pub fn define_visitor(base_name: String, tree_types: Vec<TreeType>) -> String {
    let mut out = String::new();
    out.push_str(format!("pub trait {}Visitor<T> {{\n", base_name).as_str());
    for ttype in tree_types.iter() {
        out.push_str(
            format!(
                "\tfn visit_{}_{}(&self, e: &{}) -> T;\n",
                ttype.class_name.to_lowercase(),
                ttype.base_name.to_lowercase(),
                ttype.full_name()
            )
            .as_str(),
        );
    }
    out.push_str(format!("}}\n\n").as_str());

    // create walk_* function for each type

    // first, create walk_<base_name> to match to each enum type
    out.push_str(format!("impl {} {{\n", base_name).as_str());
    out.push_str(
        format!(
            "\tpub fn walk_{}<T>(&self, v: &dyn {}Visitor<T>) -> T {{\n",
            base_name.to_lowercase(),
            base_name.to_string()
        )
        .as_str(),
    );
    out.push_str(format!("\t\tmatch self {{\n").as_str());
    for ttype in tree_types.iter() {
        out.push_str(
            format!(
                "\t\t\t{}::{}(e) => e.walk_{}(v),\n",
                base_name,
                ttype.class_name,
                ttype.snake_case_full_name()
            )
            .as_str(),
        );
    }
    out.push_str(format!("\t\t}}\n\t}}\n}}\n\n").as_str());

    // now create walk_* for each of the tree_types
    for ttype in tree_types.iter() {
        out.push_str(format!("impl {} {{\n", ttype.full_name()).as_str());
        out.push_str(
            format!(
                "\tpub fn walk_{}<T>(&self, v: &dyn {}Visitor<T>) -> T {{\n",
                ttype.snake_case_full_name(),
                base_name,
            )
            .as_str(),
        );
        out.push_str(format!("\t\tv.visit_{}(self)\n", ttype.snake_case_full_name()).as_str());
        out.push_str("\t}\n}\n\n");
    }
    out
}
