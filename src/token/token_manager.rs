use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
}

pub struct TokenManager {
    tokens: HashMap<String, Token>,
    balances: HashMap<String, HashMap<String, u64>>, // user_id -> (token_symbol -> balance)
}

impl TokenManager {
    pub fn new() -> Self {
        TokenManager {
            tokens: HashMap::new(),
            balances: HashMap::new(),
        }
    }

    pub fn mint(&mut self, token_symbol: &str, amount: u64, to: &str) {
        if let Some(token) = self.tokens.get_mut(token_symbol) {
            token.total_supply += amount;
            let user_balances = self.balances.entry(to.to_string()).or_insert_with(HashMap::new);
            *user_balances.entry(token_symbol.to_string()).or_insert(0) += amount;
        }
    }

    pub fn burn(&mut self, token_symbol: &str, amount: u64, from: &str) {
        if let Some(token) = self.tokens.get_mut(token_symbol) {
            if let Some(user_balances) = self.balances.get_mut(from) {
                if let Some(balance) = user_balances.get_mut(token_symbol) {
                    if *balance >= amount {
                        *balance -= amount;
                        token.total_supply -= amount;
                    }
                }
            }
        }
    }

    pub fn transfer(&mut self, token_symbol: &str, amount: u64, from: &str, to: &str) {
        if let Some(user_balances_from) = self.balances.get_mut(from) {
            if let Some(balance_from) = user_balances_from.get_mut(token_symbol) {
                if *balance_from >= amount {
                    *balance_from -= amount;
                    let user_balances_to = self.balances.entry(to.to_string()).or_insert_with(HashMap::new);
                    *user_balances_to.entry(token_symbol.to_string()).or_insert(0) += amount;
                }
            }
        }
    }

    pub fn get_balance(&self, user_id: &str, token_symbol: &str) -> u64 {
        self.balances.get(user_id)
            .and_then(|balances| balances.get(token_symbol))
            .cloned()
            .unwrap_or(0)
    }
}
