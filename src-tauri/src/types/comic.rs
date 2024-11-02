use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

use crate::config::Config;
use crate::extensions::IgnoreRwLockPoison;
use crate::responses::{ComicRespData, EpisodeRespData};
use crate::types::AlbumPlus;
use crate::utils::filename_filter;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::struct_field_names)]
pub struct Comic {
    pub id: i64,
    pub title: String,
    #[serde(rename = "comic_type")]
    pub comic_type: i64,
    #[serde(rename = "page_default")]
    pub page_default: i64,
    #[serde(rename = "page_allow")]
    pub page_allow: i64,
    #[serde(rename = "horizontal_cover")]
    pub horizontal_cover: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "last_ord")]
    pub last_ord: f64,
    #[serde(rename = "is_finish")]
    pub is_finish: i64,
    pub status: i64,
    pub fav: i64,
    #[serde(rename = "read_order")]
    pub read_order: f64,
    pub evaluate: String,
    pub total: i64,
    pub episode_infos: Vec<EpisodeInfo>,
    #[serde(rename = "release_time")]
    pub release_time: String,
    #[serde(rename = "is_limit")]
    pub is_limit: i64,
    #[serde(rename = "read_epid")]
    pub read_epid: i64,
    #[serde(rename = "last_read_time")]
    pub last_read_time: String,
    #[serde(rename = "is_download")]
    pub is_download: i64,
    #[serde(rename = "read_short_title")]
    pub read_short_title: String,
    pub styles2: Vec<Styles2>,
    #[serde(rename = "renewal_time")]
    pub renewal_time: String,
    #[serde(rename = "last_short_title")]
    pub last_short_title: String,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    pub discount: i64,
    #[serde(rename = "discount_end")]
    pub discount_end: String,
    #[serde(rename = "no_reward")]
    pub no_reward: bool,
    #[serde(rename = "batch_discount_type")]
    pub batch_discount_type: i64,
    #[serde(rename = "ep_discount_type")]
    pub ep_discount_type: i64,
    #[serde(rename = "has_fav_activity")]
    pub has_fav_activity: bool,
    #[serde(rename = "fav_free_amount")]
    pub fav_free_amount: i64,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "wait_hour")]
    pub wait_hour: i64,
    #[serde(rename = "wait_free_at")]
    pub wait_free_at: String,
    #[serde(rename = "no_danmaku")]
    pub no_danmaku: i64,
    #[serde(rename = "auto_pay_status")]
    pub auto_pay_status: i64,
    #[serde(rename = "no_month_ticket")]
    pub no_month_ticket: bool,
    pub immersive: bool,
    #[serde(rename = "no_discount")]
    pub no_discount: bool,
    #[serde(rename = "show_type")]
    pub show_type: i64,
    #[serde(rename = "pay_mode")]
    pub pay_mode: i64,
    #[serde(rename = "classic_lines")]
    pub classic_lines: String,
    #[serde(rename = "pay_for_new")]
    pub pay_for_new: i64,
    #[serde(rename = "fav_comic_info")]
    pub fav_comic_info: FavComicInfo,
    #[serde(rename = "serial_status")]
    pub serial_status: i64,
    #[serde(rename = "album_count")]
    pub album_count: i64,
    #[serde(rename = "wiki_id")]
    pub wiki_id: i64,
    #[serde(rename = "disable_coupon_amount")]
    pub disable_coupon_amount: i64,
    #[serde(rename = "japan_comic")]
    pub japan_comic: bool,
    #[serde(rename = "interact_value")]
    pub interact_value: String,
    #[serde(rename = "temporary_finish_time")]
    pub temporary_finish_time: String,
    pub introduction: String,
    #[serde(rename = "comment_status")]
    pub comment_status: i64,
    #[serde(rename = "no_screenshot")]
    pub no_screenshot: bool,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "no_rank")]
    pub no_rank: bool,
    #[serde(rename = "presale_text")]
    pub presale_text: String,
    #[serde(rename = "presale_discount")]
    pub presale_discount: i64,
    #[serde(rename = "no_leaderboard")]
    pub no_leaderboard: bool,
    #[serde(rename = "auto_pay_info")]
    pub auto_pay_info: AutoPayInfo,
    pub orientation: i64,
    #[serde(rename = "story_elems")]
    pub story_elems: Vec<StoryElem>,
    pub tags: Vec<Tag>,
    #[serde(rename = "is_star_hall")]
    pub is_star_hall: i64,
    #[serde(rename = "hall_icon_text")]
    pub hall_icon_text: String,
    #[serde(rename = "rookie_fav_tip")]
    pub rookie_fav_tip: RookieFavTip,
    pub authors: Vec<Author>,
    #[serde(rename = "comic_alias")]
    pub comic_alias: Vec<String>,
    #[serde(rename = "horizontal_covers")]
    pub horizontal_covers: Vec<String>,
    #[serde(rename = "data_info")]
    pub data_info: DataInfo,
    #[serde(rename = "last_short_title_msg")]
    pub last_short_title_msg: String,
    pub album_plus: AlbumPlus,
}
impl Comic {
    pub fn from(app: &AppHandle, resp_data: ComicRespData, album_plus: AlbumPlus) -> Self {
        let comic_title = filename_filter(&resp_data.title);
        let mut episode_infos: Vec<EpisodeInfo> = resp_data
            .ep_list
            .into_iter()
            .filter_map(|ep| {
                let episode_id = ep.id;
                let episode_title = Self::get_episode_title(&ep);
                let comic_title = comic_title.clone();
                let is_downloaded =
                    Self::get_is_downloaded(app, &episode_title, &comic_title).ok()?;
                let episode_info = EpisodeInfo {
                    episode_id,
                    episode_title,
                    comic_id: resp_data.id,
                    comic_title,
                    is_locked: ep.is_locked,
                    is_downloaded,
                };
                Some(episode_info)
            })
            .collect();
        episode_infos.reverse();

        let styles2 = resp_data
            .styles2
            .into_iter()
            .map(|s| Styles2 {
                id: s.id,
                name: s.name,
            })
            .collect();

        let fav_comic_info = FavComicInfo {
            has_fav_activity: resp_data.fav_comic_info.has_fav_activity,
            fav_free_amount: resp_data.fav_comic_info.fav_free_amount,
            fav_coupon_type: resp_data.fav_comic_info.fav_coupon_type,
        };

        let auto_pay_info = AutoPayInfo {
            auto_pay_orders: resp_data
                .auto_pay_info
                .auto_pay_orders
                .into_iter()
                .map(|order| AutoPayOrder {
                    id: order.id,
                    title: order.title,
                })
                .collect(),
            id: resp_data.auto_pay_info.id,
        };

        let story_elems = resp_data
            .story_elems
            .into_iter()
            .map(|elem| StoryElem {
                id: elem.id,
                name: elem.name,
            })
            .collect();

        let tags = resp_data
            .tags
            .into_iter()
            .map(|tag| Tag {
                id: tag.id,
                name: tag.name,
            })
            .collect();

        let rookie_fav_tip = RookieFavTip {
            is_show: resp_data.rookie_fav_tip.is_show,
            used: resp_data.rookie_fav_tip.used,
            total: resp_data.rookie_fav_tip.total,
        };

        let authors = resp_data
            .authors
            .into_iter()
            .map(|author| Author {
                id: author.id,
                name: author.name,
                cname: author.cname,
            })
            .collect();

        let data_info = DataInfo {
            read_score: ReadScore {
                read_score: resp_data.data_info.read_score.read_score,
                is_jump: resp_data.data_info.read_score.is_jump,
                increase: Increase {
                    days: resp_data.data_info.read_score.increase.days,
                    increase_percent: resp_data.data_info.read_score.increase.increase_percent,
                },
                percentile: resp_data.data_info.read_score.percentile,
                description: resp_data.data_info.read_score.description,
            },
            interactive_value: InteractiveValue {
                interact_value: resp_data.data_info.interactive_value.interact_value,
                is_jump: resp_data.data_info.interactive_value.is_jump,
                increase: Increase {
                    days: resp_data.data_info.interactive_value.increase.days,
                    increase_percent: resp_data
                        .data_info
                        .interactive_value
                        .increase
                        .increase_percent,
                },
                percentile: resp_data.data_info.interactive_value.percentile,
                description: resp_data.data_info.interactive_value.description,
            },
        };

        Self {
            id: resp_data.id,
            title: resp_data.title,
            comic_type: resp_data.comic_type,
            page_default: resp_data.page_default,
            page_allow: resp_data.page_allow,
            horizontal_cover: resp_data.horizontal_cover,
            square_cover: resp_data.square_cover,
            vertical_cover: resp_data.vertical_cover,
            author_name: resp_data.author_name,
            styles: resp_data.styles,
            last_ord: resp_data.last_ord,
            is_finish: resp_data.is_finish,
            status: resp_data.status,
            fav: resp_data.fav,
            read_order: resp_data.read_order,
            evaluate: resp_data.evaluate,
            total: resp_data.total,
            episode_infos,
            release_time: resp_data.release_time,
            is_limit: resp_data.is_limit,
            read_epid: resp_data.read_epid,
            last_read_time: resp_data.last_read_time,
            is_download: resp_data.is_download,
            read_short_title: resp_data.read_short_title,
            styles2,
            renewal_time: resp_data.renewal_time,
            last_short_title: resp_data.last_short_title,
            discount_type: resp_data.discount_type,
            discount: resp_data.discount,
            discount_end: resp_data.discount_end,
            no_reward: resp_data.no_reward,
            batch_discount_type: resp_data.batch_discount_type,
            ep_discount_type: resp_data.ep_discount_type,
            has_fav_activity: resp_data.has_fav_activity,
            fav_free_amount: resp_data.fav_free_amount,
            allow_wait_free: resp_data.allow_wait_free,
            wait_hour: resp_data.wait_hour,
            wait_free_at: resp_data.wait_free_at,
            no_danmaku: resp_data.no_danmaku,
            auto_pay_status: resp_data.auto_pay_status,
            no_month_ticket: resp_data.no_month_ticket,
            immersive: resp_data.immersive,
            no_discount: resp_data.no_discount,
            show_type: resp_data.show_type,
            pay_mode: resp_data.pay_mode,
            classic_lines: resp_data.classic_lines,
            pay_for_new: resp_data.pay_for_new,
            fav_comic_info,
            serial_status: resp_data.serial_status,
            album_count: resp_data.album_count,
            wiki_id: resp_data.wiki_id,
            disable_coupon_amount: resp_data.disable_coupon_amount,
            japan_comic: resp_data.japan_comic,
            interact_value: resp_data.interact_value,
            temporary_finish_time: resp_data.temporary_finish_time,
            introduction: resp_data.introduction,
            comment_status: resp_data.comment_status,
            no_screenshot: resp_data.no_screenshot,
            type_field: resp_data.type_field,
            no_rank: resp_data.no_rank,
            presale_text: resp_data.presale_text,
            presale_discount: resp_data.presale_discount,
            no_leaderboard: resp_data.no_leaderboard,
            auto_pay_info,
            orientation: resp_data.orientation,
            story_elems,
            tags,
            is_star_hall: resp_data.is_star_hall,
            hall_icon_text: resp_data.hall_icon_text,
            rookie_fav_tip,
            authors,
            comic_alias: resp_data.comic_alias,
            horizontal_covers: resp_data.horizontal_covers,
            data_info,
            last_short_title_msg: resp_data.last_short_title_msg,
            album_plus,
        }
    }
    fn get_episode_title(ep: &EpisodeRespData) -> String {
        let title = filename_filter(&ep.title);
        let short_title = filename_filter(&ep.short_title);
        let ep_title = if title == short_title {
            title
        } else {
            format!("{short_title} {title}")
        };
        ep_title.trim().to_string()
    }
    fn get_is_downloaded(
        app: &AppHandle,
        ep_title: &str,
        comic_title: &str,
    ) -> anyhow::Result<bool> {
        let download_dir = app
            .state::<RwLock<Config>>()
            .read_or_panic()
            .download_dir
            .join(comic_title)
            .join(ep_title);
        let is_downloaded = download_dir.exists();
        Ok(is_downloaded)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeInfo {
    pub episode_id: i64,
    pub episode_title: String,
    pub comic_id: i64,
    pub comic_title: String,
    pub is_locked: bool,
    pub is_downloaded: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Styles2 {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FavComicInfo {
    #[serde(rename = "has_fav_activity")]
    pub has_fav_activity: bool,
    #[serde(rename = "fav_free_amount")]
    pub fav_free_amount: i64,
    #[serde(rename = "fav_coupon_type")]
    pub fav_coupon_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AutoPayInfo {
    #[serde(rename = "auto_pay_orders")]
    pub auto_pay_orders: Vec<AutoPayOrder>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AutoPayOrder {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StoryElem {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RookieFavTip {
    #[serde(rename = "is_show")]
    pub is_show: bool,
    pub used: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub cname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DataInfo {
    #[serde(rename = "read_score")]
    pub read_score: ReadScore,
    #[serde(rename = "interactive_value")]
    pub interactive_value: InteractiveValue,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct ReadScore {
    #[serde(rename = "read_score")]
    pub read_score: String,
    #[serde(rename = "is_jump")]
    pub is_jump: bool,
    pub increase: Increase,
    pub percentile: f64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveValue {
    #[serde(rename = "interact_value")]
    pub interact_value: String,
    #[serde(rename = "is_jump")]
    pub is_jump: bool,
    pub increase: Increase,
    pub percentile: f64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Increase {
    pub days: i64,
    #[serde(rename = "increase_percent")]
    pub increase_percent: i64,
}
