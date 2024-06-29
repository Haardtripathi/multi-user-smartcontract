#![no_std]
use soroban_sdk::{Env,contract, contractimpl, token, Address, Vec};

#[contract]
pub struct MultiTransfer;

#[contractimpl]
impl MultiTransfer {
    pub fn transfer_multiple(env: Env, token: Address, from: Address, to: Vec<Address>, amount: i128) {

        //to is used as vector as it would be faster to iterate through this vector of different receipent addresses

        //some checks for checking if the user has the amount which he is sending or not

        // some code here to handle multi transfer the sender and the receiver and the amount
    }
}