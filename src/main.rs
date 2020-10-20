pub mod ats;

#[macro_use] extern crate lalrpop_util;

use crate::ats::{Expression, Statement};
use std::collections::HashMap;

lalrpop_mod!(pub preprocessor);

fn main() {
    let file = std::env::args().collect::<Vec<String>>().get(1).expect("FATAL ERROR ! You need to specify an input file.").clone();
    let file = std::fs::read_to_string(file).expect("FATAL ERROR ! Could not read the input file.\n");

    let mut macro_definitions: HashMap<String, Vec<Statement>> = HashMap::new();
    let mut output = "".to_string();

    let expressions: Vec<Expression> = preprocessor::ExpressionListParser::new().parse(&(file + "\n")).expect("FATAL ERROR ! Could not parse the program.\n");

    for i in expressions {
        match i {
            Expression::Statement(statement) => {
                process_statement(statement, &mut output, &macro_definitions, vec![]);
            }
            Expression::MacroDefinition(name, definition) => {
                if macro_definitions.contains_key(&name) {
                    *macro_definitions.get_mut(&name).unwrap() = definition;
                }
                else {
                    macro_definitions.insert(name, definition);
                }
            }
        }
    }

    println!("{}", output);
}

fn process_statement(statement: Statement, output: &mut String, macro_definitions: &HashMap<String, Vec<Statement>>, anti_recursion: Vec<String>) {
    match statement {
        Statement::Normal(parts) => {
            for i in 0..parts.len() {
                *output += &parts[i].0;
                if i != parts.len() - 1 {
                    *output += " ";
                }
            }
            *output += "\n";
        }
        Statement::MacroCall(name) => {
            if macro_definitions.contains_key(&name) {
                if anti_recursion.contains(&name) {
                    panic!("FATAL ERROR ! Macro recursion is NOT allowed !")
                }
                for i in &macro_definitions[&name] {
                    let mut anti_recursion = anti_recursion.clone();
                    anti_recursion.push(name.clone());
                    process_statement(i.clone(), output, macro_definitions, anti_recursion);
                }
            }
            else {
                panic!("FATAL ERROR ! No such macro {}.", name);
            }
        }
    }
}