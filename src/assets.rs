use crate::world::World;

use serde::{Serialize, Deserialize};
use std::{fmt, error::Error};

#[cfg(feature = "native")]
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
enum AssetType {
    Texture,
    Unknown
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    path: String,
    r#type: AssetType,
    entity_id: Option<String>,
}

#[derive(Debug)]
struct AssetImportError {
    asset: String
}
impl fmt::Display for AssetImportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not import asset: {}", self.asset)
    }
}
impl Error for AssetImportError {}

#[cfg(feature = "native")]
const ASSET_BASE_PATH: &str = "./assets";

#[cfg(target_arch = "wasm32")]
const ASSET_BASE_PATH: &str = "/assets";

pub async fn import_asset(asset: &Asset, world: &mut World) -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}", ASSET_BASE_PATH, asset.path);

    match asset.r#type {
        AssetType::Texture => {
            let texture;
            
            #[cfg(feature = "native")]
            { texture = image::io::Reader::open(path.clone())?.decode()?.to_rgb8(); }

            #[cfg(target_arch = "wasm32")]
            {
                let buf = path.clone().as_mut_ptr();
                let len = path.len();
                let data = crate::wasm_utils::load_asset_file(buf, len).await;
                texture = crate::wasm_utils::load_image_from_array(&data).to_rgb8();
            }

            if let Some(id) = &asset.entity_id {
                if id == "player" {
                    world.player.entity.set_texture(Some(texture));
                    return Ok(())
                }
            }
        },
        _ => {}
    }
    return Err(Box::new(AssetImportError { asset: path }))
}

pub async fn import_assets(assets: Vec<Asset>, world: &mut World) -> Result<(), Box<dyn Error>> {
    for asset in assets.iter() {
        import_asset(asset, world).await?;
    }
    Ok(())
}

#[cfg(feature = "native")]
pub fn load_assets() -> Result<Vec<Asset>, Box<dyn Error>> {
    let index: Vec<Asset> = serde_json::from_str(&fs::read_to_string("./assets/index.json")?)?;
    println!("Loaded asset index: {:#?}", index);
    Ok(index)
}

#[cfg(target_arch = "wasm32")]
pub async fn load_assets() -> Result<Vec<Asset>, Box<dyn Error>> {
    let data = crate::wasm_utils::load_asset_index().await;
    let index: Vec<Asset> = serde_json::from_slice(&data).unwrap();
    crate::log!("Loaded asset index: {:#?}", index);
    Ok(index)
}