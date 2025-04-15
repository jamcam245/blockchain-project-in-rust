use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Transaction {
    id: u32,
    sender: String,
    receiver: String,
    amount: f64,
    signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidatedTransaction {
    pub id: u32,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: String,
}

impl Transaction {
    pub fn new(id: u32, sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            id,
            sender,
            receiver,
            amount,
            signature: None,
        }
    }

    pub fn sign(&mut self, signature: String) {
        self.signature = Some(signature);
    }

    pub fn validate(&self) -> Result<ValidatedTransaction, String> {
        if let Some(ref sig) = self.signature {
            Ok(ValidatedTransaction {
                id: self.id,
                sender: self.sender.clone(),
                receiver: self.receiver.clone(),
                amount: self.amount,
                signature: sig.clone(),
            })
        } else {
            Err("Transaction not signed".to_string())
        }
    }
}

impl ValidatedTransaction {
        pub fn serialize(&self) -> String {
            format!(
                "ValidatedTransaction(id: {}, sender: {}, receiver: {}, amount: {}, signature: {})",
                self.id, self.sender, self.receiver, self.amount, self.signature
            )
        }
    }