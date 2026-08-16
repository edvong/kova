use kova_core::dsl::ExtensionOperator;
use kova_core::render::Render;

pub enum Operator {
    
}

impl Render for Operator {
    fn render(&self, sql: &mut String) {
        todo!()
    }
}

impl ExtensionOperator for Operator {
    
}