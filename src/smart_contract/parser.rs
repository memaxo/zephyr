use crate::ast::*;
use crate::token::*;
use crate::lexer::*;
use crate::expression::*;
use crate::statement::*;
use crate::function::*;
use crate::type_annotation::*;
use crate::contract::*;
use crate::symbol::*;
use crate::error::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    symbol_table_builder: SymbolTableBuilder,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            symbol_table_builder: SymbolTableBuilder::new(),
        }
    }

    pub fn parse_contract(&mut self, error_reporter: &mut ErrorReporter) -> Result<Contract, ParserError> {
        let contract = parse_contract(self)?;
        let symbol_table = self.symbol_table_builder.build();
        // Perform additional checks or analysis using the symbol table
        Ok(contract)
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn consume_token(&mut self) {
        self.current += 1;
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParserError> {
        if let Some(token) = self.current_token() {
            if *token == expected {
                self.consume_token();
                Ok(())
            } else {
                Err(ParserError::UnexpectedToken)
            }
        } else {
            Err(ParserError::UnexpectedEOF)
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParserError> {
        if let Some(Token::Identifier(name)) = self.current_token() {
            let name = name.clone();
            self.consume_token();
            Ok(name)
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn enter_scope(&mut self) {
        self.symbol_table_builder.enter_scope();
    }

    fn exit_scope(&mut self) {
        self.symbol_table_builder.exit_scope();
    }

    fn insert_variable(&mut self, name: String, var_type: Type) {
        self.symbol_table_builder.insert_variable(name, var_type);
    }

    fn insert_function(&mut self, name: String, param_types: Vec<Type>, return_type: Option<Type>) {
        self.symbol_table_builder.insert_function(name, param_types, return_type);
    }

    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table_builder.lookup_symbol(name)
    }
}