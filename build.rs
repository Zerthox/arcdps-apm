use futures::{stream::FuturesOrdered, TryStreamExt};
use quote::quote;
use serde::{de::DeserializeOwned, Deserialize};
use std::{collections::HashMap, env, fs, path::PathBuf, pin::Pin};

#[tokio::main]
async fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("failed to get out dir"));

    // fetch data
    let ids = fetch_all_skill_ids()
        .await
        .expect("failed to fetch all skill ids");
    let skills = fetch_skills(&ids).await.expect("failed to fetch skills");

    // parse overwrites
    const OVERWRITES_PATH: &str = "src/data/overwrites.yml";
    let overwrites: Vec<u32> = serde_yaml::from_str(
        &fs::read_to_string(OVERWRITES_PATH).expect("failed to read overwrites"),
    )
    .expect("failed to parse overwrites");
    println!("cargo:rerun-if-changed={}", OVERWRITES_PATH);

    // insert into map
    let skill_data: HashMap<_, _> = skills
        .iter()
        .map(|skill| (skill.id, skill.is_action()))
        .chain(overwrites.into_iter().map(|id| (id, true)))
        .collect();

    // generate file contents
    let contents = skill_data
        .iter()
        .map(|(id, is_auto)| quote! { (#id, #is_auto) });
    let result = quote! {
        [ #(#contents),* ]
    };

    // save file
    fs::write(out_dir.join("skill_data.rs"), result.to_string()).expect("failed to write file");
}

const API_URL: &str = "https://api.guildwars2.com";

async fn fetch_api<T, P>(params: P) -> reqwest::Result<T>
where
    T: DeserializeOwned,
    P: AsRef<str>,
{
    let response =
        reqwest::get(format!("{}/v2/skills?lang=en&{}", API_URL, params.as_ref())).await?;
    response.json().await
}

async fn fetch_all_skill_ids() -> reqwest::Result<Vec<u32>> {
    fetch_api("").await
}

async fn fetch_skills(ids: &[u32]) -> reqwest::Result<Vec<Skill>> {
    const LIMIT: usize = 200;

    if ids.is_empty() {
        Ok(Vec::new())
    } else {
        let mut requests = FuturesOrdered::from_iter(ids.chunks(LIMIT).map(|ids| async move {
            fetch_api::<Vec<Skill>, _>(format!(
                "ids={}",
                ids.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .await
        }));

        let results: Vec<_> = Pin::new(&mut requests).try_collect().await?;
        Ok(results.into_iter().flatten().collect())
    }
}

#[derive(Debug, Deserialize)]
struct Skill {
    pub id: u32,

    #[serde(default)]
    pub slot: String,
}

impl Skill {
    pub fn is_action(&self) -> bool {
        // everything that has a slot set except weapon and downed skill 1
        !matches!(self.slot.as_str(), "" | "Weapon_1" | "Downed_1")
    }
}
