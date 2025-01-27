// SPDX-License-Identifier: Apache-2.0

use crate::{build_solidity, SorobanEnv};
use soroban_sdk::{testutils::Logs, Val};

#[test]
fn log_runtime_error() {
    let wasm = build_solidity(
        r#"contract counter {
            uint64 public count = 1;
        
            function decrement() public returns (uint64){
                count -= 1;
                return count;
            }
        }"#,
    );
    let mut env = SorobanEnv::new();
    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&env.env);
    let address = env.register_contract(wasm, constructor_args);

    env.invoke_contract(&address, "decrement", vec![]);

    let logs = env.invoke_contract_expect_error(&address, "decrement", vec![]);

    assert!(logs[0].contains("runtime_error: math overflow in test.sol:5:17-27"));
}

#[test]
fn print() {
    let wasm = build_solidity(
        r#"contract Printer {

            function print() public {
                print("Hello, World!");
            }
        }"#,
    );
    let mut src = SorobanEnv::new();
    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&src.env);
    let address = src.register_contract(wasm, constructor_args);

    src.invoke_contract(&address, "print", vec![]);

    let logs = src.env.logs().all();

    assert!(logs[0].contains("Hello, World!"));
}

#[test]
fn print_then_runtime_error() {
    let wasm = build_solidity(
        r#"contract counter {
            uint64 public count = 1;

            function decrement() public returns (uint64){
                print("Second call will FAIL!");
                count -= 1;
                return count;
            }
        }"#,
    );
    let mut src = SorobanEnv::new();
    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&src.env);
    let address = src.register_contract(wasm, constructor_args);

    src.invoke_contract(&address, "decrement", vec![]);

    let logs = src.invoke_contract_expect_error(&address, "decrement", vec![]);

    assert!(logs[0].contains("Second call will FAIL!"));
    assert!(logs[1].contains("Second call will FAIL!"));
    assert!(logs[2].contains("runtime_error: math overflow in test.sol:6:17-27"));
}