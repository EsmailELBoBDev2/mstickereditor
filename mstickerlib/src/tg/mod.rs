use crate::CLIENT;
use anyhow::bail;
use monostate::MustBe;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub(crate) mod sticker;

pub(crate) mod stickerpack;
use stickerpack::Pack;

#[derive(Deserialize)]
pub struct Config {
	pub bot_key: String
}

/// File storage at Telegram; see https://core.telegram.org/bots/api#file
#[derive(Debug, Deserialize)]
struct File {
	file_path: String
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TgResponse<T> {
	Ok {
		#[allow(dead_code)]
		ok: MustBe!(true),

		result: T
	},
	Err {
		#[allow(dead_code)]
		ok: MustBe!(false),

		error_code: u32,
		description: String
	}
}

async fn tg_get<T, P>(tg_config: &Config, operation: &str, params: P) -> anyhow::Result<T>
where
	T: DeserializeOwned,
	P: Serialize,
	P: Sized
{
	let resp: TgResponse<T> = CLIENT
		.get()
		.await
		.get(format!("https://api.telegram.org/bot{}/{}", tg_config.bot_key, operation))
		.query(&params)
		.send()
		.await?
		.json()
		.await?;
	let result = match resp {
		TgResponse::Ok { result, .. } => result,
		TgResponse::Err {
			error_code, description, ..
		} => bail!("Telegram request was not successful: {error_code} {description}")
	};
	Ok(result)
}

pub(crate) async fn get_stickerpack(tg_config: &Config, name: &str) -> anyhow::Result<Pack> {
	tg_get(tg_config, "getStickerSet", [("name", name)]).await
}
