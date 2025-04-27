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
    pub fn to_bytes(&self) -> Vec<u8> {
        assert!(!self.sender.contains('\0'), "Sender contains null byte");
        assert!(!self.receiver.contains('\0'), "Receiver contains null byte");
        assert!(!self.signature.contains('\0'), "Signature contains null byte");

        let mut bytes: Vec<u8> = Vec::with_capacity(
            4 +
            self.sender.len() + 1 + 
            self.receiver.len() + 1 + 
            8 +
            self.signature.len()
        );

        bytes.extend_from_slice(&self.id.to_le_bytes());
        bytes.extend_from_slice(self.sender.as_bytes());
        bytes.extend_from_slice(&[0u8]); // separator
        bytes.extend_from_slice(self.receiver.as_bytes());
        bytes.extend_from_slice(&[0u8]); // separator
        bytes.extend_from_slice(&self.amount.to_le_bytes());
        bytes.extend_from_slice(self.signature.as_bytes());
        bytes
    }
}