use std::collections::HashMap;

use crate::config::Config;
use crate::responses::{ComicRespData, EpisodeRespData};
use crate::types::AlbumPlus;
use crate::utils::filename_filter;

use chrono::{Datelike, NaiveDateTime};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use yaserde::{YaDeserialize, YaSerialize};

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
    #[allow(clippy::too_many_lines)]
    // TODO: 统一用from实现，以减少代码行数
    pub fn from(app: &AppHandle, comic: ComicRespData, album_plus: AlbumPlus) -> Self {
        let comic_title = filename_filter(&comic.title);
        let mut episode_infos: Vec<EpisodeInfo> = comic
            .ep_list
            .into_iter()
            .filter_map(|ep| {
                let episode_title = Self::get_episode_title(&ep);
                let comic_title = comic_title.clone();
                let is_downloaded = Self::get_is_downloaded(app, &episode_title, &comic_title);
                const TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
                let pub_time = NaiveDateTime::parse_from_str(&ep.pub_time, TIME_FORMAT).ok()?;

                let comic_info = ComicInfo {
                    manga: "Yes".to_string(),
                    series: comic_title.clone(),
                    publisher: "哔哩哔哩漫画".to_string(),
                    writer: comic
                        .authors
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<&str>>()
                        .join(", "),
                    genre: comic.styles.join(", "),
                    summary: comic.evaluate.clone(),
                    count: comic.total,
                    title: episode_title.clone(),
                    number: ep.ord.to_string(),
                    page_count: ep.image_count,
                    year: pub_time.year(),
                    month: pub_time.month(),
                    day: pub_time.day(),
                };

                let episode_info = EpisodeInfo {
                    episode_id: ep.id,
                    episode_title,
                    comic_id: comic.id,
                    comic_title,
                    is_locked: ep.is_locked,
                    is_downloaded,
                    comic_info,
                };
                Some(episode_info)
            })
            .collect();
        // 解决章节标题重复的问题
        let mut ep_title_count = HashMap::new();
        // 统计章节标题出现的次数
        for ep in &episode_infos {
            let Some(count) = ep_title_count.get_mut(&ep.episode_title) else {
                ep_title_count.insert(ep.episode_title.clone(), 1);
                continue;
            };
            *count += 1;
        }
        // 只保留重复的章节标题
        ep_title_count.retain(|_, v| *v > 1);
        // 为重复的章节标题添加序号
        for ep in &mut episode_infos {
            let Some(count) = ep_title_count.get_mut(&ep.episode_title) else {
                continue;
            };
            ep.episode_title = format!("{}-{}", ep.episode_title, count);
            *count -= 1;
        }

        episode_infos.reverse();

        let styles2 = comic
            .styles2
            .into_iter()
            .map(|s| Styles2 {
                id: s.id,
                name: s.name,
            })
            .collect();

        let fav_comic_info = FavComicInfo {
            has_fav_activity: comic.fav_comic_info.has_fav_activity,
            fav_free_amount: comic.fav_comic_info.fav_free_amount,
            fav_coupon_type: comic.fav_comic_info.fav_coupon_type,
        };

        let auto_pay_info = AutoPayInfo {
            auto_pay_orders: comic
                .auto_pay_info
                .auto_pay_orders
                .into_iter()
                .map(|order| AutoPayOrder {
                    id: order.id,
                    title: order.title,
                })
                .collect(),
            id: comic.auto_pay_info.id,
        };

        let story_elems = comic
            .story_elems
            .into_iter()
            .map(|elem| StoryElem {
                id: elem.id,
                name: elem.name,
            })
            .collect();

        let tags = comic
            .tags
            .into_iter()
            .map(|tag| Tag {
                id: tag.id,
                name: tag.name,
            })
            .collect();

        let rookie_fav_tip = RookieFavTip {
            is_show: comic.rookie_fav_tip.is_show,
            used: comic.rookie_fav_tip.used,
            total: comic.rookie_fav_tip.total,
        };

        let authors = comic
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
                read_score: comic.data_info.read_score.read_score,
                is_jump: comic.data_info.read_score.is_jump,
                increase: Increase {
                    days: comic.data_info.read_score.increase.days,
                    increase_percent: comic.data_info.read_score.increase.increase_percent,
                },
                percentile: comic.data_info.read_score.percentile,
                description: comic.data_info.read_score.description,
            },
            interactive_value: InteractiveValue {
                interact_value: comic.data_info.interactive_value.interact_value,
                is_jump: comic.data_info.interactive_value.is_jump,
                increase: Increase {
                    days: comic.data_info.interactive_value.increase.days,
                    increase_percent: comic.data_info.interactive_value.increase.increase_percent,
                },
                percentile: comic.data_info.interactive_value.percentile,
                description: comic.data_info.interactive_value.description,
            },
        };

        Self {
            id: comic.id,
            title: comic.title,
            comic_type: comic.comic_type,
            page_default: comic.page_default,
            page_allow: comic.page_allow,
            horizontal_cover: comic.horizontal_cover,
            square_cover: comic.square_cover,
            vertical_cover: comic.vertical_cover,
            author_name: comic.author_name,
            styles: comic.styles,
            last_ord: comic.last_ord,
            is_finish: comic.is_finish,
            status: comic.status,
            fav: comic.fav,
            read_order: comic.read_order,
            evaluate: comic.evaluate,
            total: comic.total,
            episode_infos,
            release_time: comic.release_time,
            is_limit: comic.is_limit,
            read_epid: comic.read_epid,
            last_read_time: comic.last_read_time,
            is_download: comic.is_download,
            read_short_title: comic.read_short_title,
            styles2,
            renewal_time: comic.renewal_time,
            last_short_title: comic.last_short_title,
            discount_type: comic.discount_type,
            discount: comic.discount,
            discount_end: comic.discount_end,
            no_reward: comic.no_reward,
            batch_discount_type: comic.batch_discount_type,
            ep_discount_type: comic.ep_discount_type,
            has_fav_activity: comic.has_fav_activity,
            fav_free_amount: comic.fav_free_amount,
            allow_wait_free: comic.allow_wait_free,
            wait_hour: comic.wait_hour,
            wait_free_at: comic.wait_free_at,
            no_danmaku: comic.no_danmaku,
            auto_pay_status: comic.auto_pay_status,
            no_month_ticket: comic.no_month_ticket,
            immersive: comic.immersive,
            no_discount: comic.no_discount,
            show_type: comic.show_type,
            pay_mode: comic.pay_mode,
            classic_lines: comic.classic_lines,
            pay_for_new: comic.pay_for_new,
            fav_comic_info,
            serial_status: comic.serial_status,
            album_count: comic.album_count,
            wiki_id: comic.wiki_id,
            disable_coupon_amount: comic.disable_coupon_amount,
            japan_comic: comic.japan_comic,
            interact_value: comic.interact_value,
            temporary_finish_time: comic.temporary_finish_time,
            introduction: comic.introduction,
            comment_status: comic.comment_status,
            no_screenshot: comic.no_screenshot,
            type_field: comic.type_field,
            no_rank: comic.no_rank,
            presale_text: comic.presale_text,
            presale_discount: comic.presale_discount,
            no_leaderboard: comic.no_leaderboard,
            auto_pay_info,
            orientation: comic.orientation,
            story_elems,
            tags,
            is_star_hall: comic.is_star_hall,
            hall_icon_text: comic.hall_icon_text,
            rookie_fav_tip,
            authors,
            comic_alias: comic.comic_alias,
            horizontal_covers: comic.horizontal_covers,
            data_info,
            last_short_title_msg: comic.last_short_title_msg,
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
    fn get_is_downloaded(app: &AppHandle, ep_title: &str, comic_title: &str) -> bool {
        let config = app.state::<RwLock<Config>>();
        let config = config.read();
        config
            .download_dir
            .join(comic_title)
            .join(ep_title)
            .with_extension(config.archive_format.extension())
            .exists()
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
    pub comic_info: ComicInfo,
}

#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type, YaSerialize, YaDeserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ComicInfo {
    #[yaserde(rename = "Manga")]
    pub manga: String,
    #[yaserde(rename = "Series")]
    pub series: String,
    #[yaserde(rename = "Publisher")]
    pub publisher: String,
    #[yaserde(rename = "Writer")]
    pub writer: String,
    #[yaserde(rename = "Genre")]
    pub genre: String,
    #[yaserde(rename = "Summary")]
    pub summary: String,
    #[yaserde(rename = "Count")]
    pub count: i64,
    #[yaserde(rename = "Title")]
    pub title: String,
    #[yaserde(rename = "Number")]
    pub number: String,
    #[yaserde(rename = "PageCount")]
    pub page_count: i64,
    #[yaserde(rename = "Year")]
    pub year: i32,
    #[yaserde(rename = "Month")]
    pub month: u32,
    #[yaserde(rename = "Day")]
    pub day: u32,
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
