extern crate reqwest;

use std::fs::File;
use std::io::prelude::*;

mod magic_card;

use magic_card::*;

fn main() -> Result<(), Box<std::error::Error>> {
    // let base_url = "https://api.magicthegathering.io";

    // let api_version = 1;

    // let api_action = "cards";

    // let card_id = 386_616;

    // let api_url = get_magic_api_url(base_url, api_version, api_action, card_id);

    // println!("Getting from url: {}", &api_url);

    // let mut partial_resp = reqwest::get(&api_url)?;

    // let text_resp = partial_resp.text()?;

    // let resp: MagicCardResponse = partial_resp.json()?;

    // println!("Text response:\n{}", text_resp);

    // println!("Json Response:\n{:#?}", resp);

    // Ok(())

    let mut file = File::open("teferi.json")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let card = serde_json::from_str::<MagicCard>(&contents)?;

    println!("{:?}", card);

    // match card.card {
    //     None => {
    //         println!("Found multiple cards");
    //         match card.cards {
    //             None => panic!("Couldn't unpack multiple cards"),
    //             Some(cards) => println!("{:?}", cards),
    //         };
    //     },
    //     Some (found_card) => println!("{:?}", found_card),
    // }

    Ok(())
}

fn get_magic_api_url(base_url: &str, api_version: u32, api_action: &str, card_id: u32) -> String {
    format!("{}/v{}/{}/{}", base_url, api_version, api_action, card_id)
}
