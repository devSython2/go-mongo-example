use mongodb::{
    bson::doc,
    options::{ClientOptions, Credential},
    sync::Client,
};

use std::error::Error;
use std::io::prelude::*;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub mod clinical_studies_structures;
use crate::clinical_studies_structures::{DrugIndication,DrugCompany};


use minijinja::{Environment, context};


fn main() {
    println!("Hello, world!");
}
