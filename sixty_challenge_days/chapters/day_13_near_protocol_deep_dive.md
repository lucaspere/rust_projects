# Day 13 - NEAR Protocol Deep Dive - 29-08-2024

## NEAR Fundamentals
### Account Model
NEAR uses an account to represents a user. Each account has a unique address and can hold tokens and smart contracts. The account is controlled by an access key that can be used to sign transactions on behalf of the account.

### [Transaction](https://docs.near.org/concepts/protocol/transactions)
Users interact with NEAR by creating transactions. A transaction is a signed message that is sent to the network to perform an action like transferring tokens or calling a smart contract. Transactions is composed of one or more actions, each action costs a amount of gas units. The gas is a measure of computational resources used to execute the action.

A transaction is composed of the following fields:
- **Signer**: The account that signs the transaction.
- **Actions**: The actions to be executed.
- **Receiver**: The account that will receive the transaction.
- **Block Hash**: The hash of a recent block, to limit the time-validity of the transaction.
- **Public Key**: The public key of the signer.
- **Nonce**: A number that is incremented for each transaction sent by the signer.

> The transaction is signed by the signer's private key and the signature is included in the transaction.

The Actions are executed in the order they are specified in the transaction. If an action fails, the transaction is reverted and the gas used is refunded.

#### Gas (Execution Fees)
NEAR uses gas to limit the amount of computation that can be done in a single transaction. The gas price is set by the network and is used to calculate the execution fees for a transaction. The gas price is measured in NEAR tokens per gas unit. For every transaction, users get charged a small $NEAR fee which has to be paid upfront. This fee is calculated using deterministic gas units, and transformed into a cost in $NEAR using the network's gas price.

##### Gas Units
Every actionsin NEAR costs a fixed amount of gas units, meaning that the same operation will always costs the same amount of gas units, where ``1Tgas`` gets you approx. ``1ms`` of CPU time.

Transaction can use a maximum of ``300Tgas``, meaning that a transaction can use up to ``300ms`` of CPU time.

##### Gas Price
The $NEAR fee is calculated by multiplying the cost of all actions in the transaction by a gas price. The gas price is recalculated each block basead on the network's congestion and floor at ``1Tgas = 0.000nNEAR``.

### [Storage Staking](https://docs.near.org/concepts/storage/storage-staking)
When you deploy a smart contract to NEAR, you pay for the storage that this contract requires using a mechanism called storage staking. The storage staking is a way to reserve storage on the network for your smart contract. The storage staking is done by locking a certain amount of NEAR tokens in a special account called a "staking pool". The staking pool is a smart contract that manages the storage staking for all the smart contracts on the network.


## Simple Interactions:
```rs
use near_sdk::{env, log, near, Gas, Promise, PromiseError};

#[near(contract_state)]
pub struct Contract {
    greeting: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            greeting: "Hello".to_string(),
        }
    }
}

use near_sdk::ext_contract;

pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

#[ext_contract(hello_near)]
trait HelloNear {
    fn get_greeting(&self) -> String;
    fn set_greeting(&self, greeting: String);
}

#[near]
impl Contract {
    pub fn query_account_balance(&self) -> String {
        env::account_balance().exact_amount_display()
    }
    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    pub fn call_view_method(&self) -> Promise {
        let promise = hello_near::ext(env::current_account_id())
            .with_static_gas(Gas::from_tgas(5))
            .get_greeting();

        return promise.then(
            // Create a promise to callback query_greeting_callback
            Self::ext(env::current_account_id())
                .with_static_gas(Gas::from_tgas(5))
                .query_greeting_callback(),
        );
    }

    #[private]
    pub fn query_greeting_callback(
        &self,
        #[callback_result] call_result: Result<String, PromiseError>,
    ) -> String {
        if call_result.is_err() {
            log!("There was an error contacting Hello NEAR");
            return "".to_string();
        }
        let greeting: String = call_result.unwrap();
        greeting
    }
}

```