#[derive(Debug, Clone)]
pub struct StatementPart (pub String);

#[derive(Debug, Clone)]
pub enum Statement {
    MacroCall(String),
    Normal(Vec<StatementPart>)
}

#[derive(Debug)]
pub enum Expression {
    Statement(Statement),
    MacroDefinition(String, Vec<Statement>)
}