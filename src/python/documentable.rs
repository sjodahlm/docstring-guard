use rustpython_parser::{
    ast::{Identifier, Stmt, StmtClassDef, StmtFunctionDef},
    text_size::TextRange,
};

pub trait Documentable {
    fn body(&self) -> &[Stmt];
    fn name(&self) -> &Identifier;
    fn range(&self) -> TextRange;
}

impl Documentable for StmtFunctionDef {
    fn body(&self) -> &[Stmt] {
        &self.body
    }
    fn name(&self) -> &Identifier {
        &self.name
    }
    fn range(&self) -> TextRange {
        self.range
    }
}

impl Documentable for StmtClassDef {
    fn body(&self) -> &[Stmt] {
        &self.body
    }
    fn name(&self) -> &Identifier {
        &self.name
    }
    fn range(&self) -> TextRange {
        self.range
    }
}
