// @ts-nocheck
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

/** user-defined commands **/


export const commands = {
async greet(name: string) : Promise<string> {
    return await TAURI_INVOKE("greet", { name });
},
async getConfig() : Promise<Config> {
    return await TAURI_INVOKE("get_config");
},
async saveConfig(config: Config) : Promise<Result<null, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("save_config", { config }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async generateQrcode() : Promise<Result<QrcodeData, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("generate_qrcode") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getQrcodeStatus(authCode: string) : Promise<Result<QrcodeStatus, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_qrcode_status", { authCode }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async search(keyword: string, pageNum: number) : Promise<Result<SearchRespData, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("search", { keyword, pageNum }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getComic(comicId: number) : Promise<Result<Comic, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_comic", { comicId }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async downloadEpisodes(episodes: EpisodeInfo[]) : Promise<Result<null, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("download_episodes", { episodes }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async showPathInFileManager(path: string) : Promise<Result<null, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("show_path_in_file_manager", { path }) };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
},
async getUserProfile() : Promise<Result<UserProfileRespData, CommandError>> {
    try {
    return { status: "ok", data: await TAURI_INVOKE("get_user_profile") };
} catch (e) {
    if(e instanceof Error) throw e;
    else return { status: "error", error: e  as any };
}
}
}

/** user-defined events **/


export const events = __makeEvents__<{
downloadEpisodeEndEvent: DownloadEpisodeEndEvent,
downloadEpisodePendingEvent: DownloadEpisodePendingEvent,
downloadEpisodeStartEvent: DownloadEpisodeStartEvent,
downloadImageErrorEvent: DownloadImageErrorEvent,
downloadImageSuccessEvent: DownloadImageSuccessEvent,
downloadSpeedEvent: DownloadSpeedEvent,
removeWatermarkEndEvent: RemoveWatermarkEndEvent,
removeWatermarkErrorEvent: RemoveWatermarkErrorEvent,
removeWatermarkStartEvent: RemoveWatermarkStartEvent,
removeWatermarkSuccessEvent: RemoveWatermarkSuccessEvent,
updateOverallDownloadProgressEvent: UpdateOverallDownloadProgressEvent
}>({
downloadEpisodeEndEvent: "download-episode-end-event",
downloadEpisodePendingEvent: "download-episode-pending-event",
downloadEpisodeStartEvent: "download-episode-start-event",
downloadImageErrorEvent: "download-image-error-event",
downloadImageSuccessEvent: "download-image-success-event",
downloadSpeedEvent: "download-speed-event",
removeWatermarkEndEvent: "remove-watermark-end-event",
removeWatermarkErrorEvent: "remove-watermark-error-event",
removeWatermarkStartEvent: "remove-watermark-start-event",
removeWatermarkSuccessEvent: "remove-watermark-success-event",
updateOverallDownloadProgressEvent: "update-overall-download-progress-event"
})

/** user-defined constants **/



/** user-defined types **/

export type Author = { id: number; name: string; cname: string }
export type AutoPayInfo = { auto_pay_orders: AutoPayOrder[]; id: number }
export type AutoPayOrder = { id: number; title: string }
export type BannerRespData = { icon: string; title: string; url: string }
export type Comic = { id: number; title: string; comic_type: number; page_default: number; page_allow: number; horizontal_cover: string; square_cover: string; vertical_cover: string; author_name: string[]; styles: string[]; last_ord: number; is_finish: number; status: number; fav: number; read_order: number; evaluate: string; total: number; episodeInfos: EpisodeInfo[]; release_time: string; is_limit: number; read_epid: number; last_read_time: string; is_download: number; read_short_title: string; styles2: Styles2[]; renewal_time: string; last_short_title: string; discount_type: number; discount: number; discount_end: string; no_reward: boolean; batch_discount_type: number; ep_discount_type: number; has_fav_activity: boolean; fav_free_amount: number; allow_wait_free: boolean; wait_hour: number; wait_free_at: string; no_danmaku: number; auto_pay_status: number; no_month_ticket: boolean; immersive: boolean; no_discount: boolean; show_type: number; pay_mode: number; classic_lines: string; pay_for_new: number; fav_comic_info: FavComicInfo; serial_status: number; album_count: number; wiki_id: number; disable_coupon_amount: number; japan_comic: boolean; interact_value: string; temporary_finish_time: string; introduction: string; comment_status: number; no_screenshot: boolean; type: number; no_rank: boolean; presale_text: string; presale_discount: number; no_leaderboard: boolean; auto_pay_info: AutoPayInfo; orientation: number; story_elems: StoryElem[]; tags: Tag[]; is_star_hall: number; hall_icon_text: string; rookie_fav_tip: RookieFavTip; authors: Author[]; comic_alias: string[]; horizontal_covers: string[]; data_info: DataInfo; last_short_title_msg: string }
export type ComicInSearchRespData = { id: number; title: string; square_cover: string; vertical_cover: string; author_name: string[]; styles: string[]; is_finish: number; allow_wait_free: boolean; discount_type: number; type: number; wiki: WikiRespData }
export type CommandError = string
export type Config = { accessToken: string; downloadDir: string }
export type CookieInfoRespData = { cookies: CookieRespData[]; domains: string[] }
export type CookieRespData = { name: string; value: string; http_only: number; expires: number; secure: number }
export type DataInfo = { read_score: ReadScore; interactive_value: InteractiveValue }
export type DownloadEpisodeEndEvent = DownloadEpisodeEndEventPayload
export type DownloadEpisodeEndEventPayload = { epId: number; errMsg: string | null }
export type DownloadEpisodePendingEvent = DownloadEpisodePendingEventPayload
export type DownloadEpisodePendingEventPayload = { epId: number; title: string }
export type DownloadEpisodeStartEvent = DownloadEpisodeStartEventPayload
export type DownloadEpisodeStartEventPayload = { epId: number; title: string; total: number }
export type DownloadImageErrorEvent = DownloadImageErrorEventPayload
export type DownloadImageErrorEventPayload = { epId: number; url: string; errMsg: string }
export type DownloadImageSuccessEvent = DownloadImageSuccessEventPayload
export type DownloadImageSuccessEventPayload = { epId: number; url: string; current: number }
export type DownloadSpeedEvent = DownloadSpeedEventPayload
export type DownloadSpeedEventPayload = { speed: string }
export type EpisodeInfo = { episodeId: number; episodeTitle: string; mangaId: number; mangaTitle: string; isLocked: boolean; isDownloaded: boolean }
export type FavComicInfo = { has_fav_activity: boolean; fav_free_amount: number; fav_coupon_type: number }
export type Increase = { days: number; increase_percent: number }
export type InteractiveValue = { interact_value: string; is_jump: boolean; increase: Increase; percentile: number; description: string }
export type NovelInSearchRespData = { novel_id: number; title: string; v_cover: string; finish_status: number; status: number; discount_type: number; numbers: number; style: StyleRespData; evaluate: string; author: string; tag: TagRespData }
export type QrcodeData = { base64: string; auth_code: string }
export type QrcodeStatus = { code: number; message: string; is_new: boolean; mid: number; access_token: string; refresh_token: string; expires_in: number; token_info: TokenInfoRespData; cookie_info: CookieInfoRespData; sso: string[] }
export type ReadScore = { read_score: string; is_jump: boolean; increase: Increase; percentile: number; description: string }
export type RemoveWatermarkEndEvent = RemoveWatermarkEndEventPayload
export type RemoveWatermarkEndEventPayload = { dirPath: string }
export type RemoveWatermarkErrorEvent = RemoveWatermarkErrorEventPayload
export type RemoveWatermarkErrorEventPayload = { dirPath: string; imgPath: string; errMsg: string }
export type RemoveWatermarkStartEvent = RemoveWatermarkStartEventPayload
export type RemoveWatermarkStartEventPayload = { dirPath: string; total: number }
export type RemoveWatermarkSuccessEvent = RemoveWatermarkSuccessEventPayload
export type RemoveWatermarkSuccessEventPayload = { dirPath: string; imgPath: string; current: number }
export type RookieFavTip = { is_show: boolean; used: number; total: number }
export type SearchComicRespData = { list: ComicInSearchRespData[]; total_page: number; total_num: number; similar: string; se_id: string; banner: BannerRespData }
export type SearchNovelRespData = { total: number; list: NovelInSearchRespData[] }
export type SearchRespData = { comic_data: SearchComicRespData; novel_data: SearchNovelRespData }
export type StoryElem = { id: number; name: string }
export type StyleRespData = { id: number; name: string }
export type Styles2 = { id: number; name: string }
export type Tag = { id: number; name: string }
export type TagRespData = { id: number; name: string }
export type TokenInfoRespData = { mid: number; access_token: string; refresh_token: string; expires_in: number }
export type UpdateOverallDownloadProgressEvent = UpdateOverallDownloadProgressEventPayload
export type UpdateOverallDownloadProgressEventPayload = { downloadedImageCount: number; totalImageCount: number; percentage: number }
export type UserProfileRespData = { face: string; name: string }
export type WikiRespData = { id: number; title: string; origin_title: string; vertical_cover: string; producer: string; author_name: string[]; publish_time: string; frequency: string }

/** tauri-specta globals **/

import {
	invoke as TAURI_INVOKE,
	Channel as TAURI_CHANNEL,
} from "@tauri-apps/api/core";
import * as TAURI_API_EVENT from "@tauri-apps/api/event";
import { type WebviewWindow as __WebviewWindow__ } from "@tauri-apps/api/webviewWindow";

type __EventObj__<T> = {
	listen: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.listen<T>>;
	once: (
		cb: TAURI_API_EVENT.EventCallback<T>,
	) => ReturnType<typeof TAURI_API_EVENT.once<T>>;
	emit: null extends T
		? (payload?: T) => ReturnType<typeof TAURI_API_EVENT.emit>
		: (payload: T) => ReturnType<typeof TAURI_API_EVENT.emit>;
};

export type Result<T, E> =
	| { status: "ok"; data: T }
	| { status: "error"; error: E };

function __makeEvents__<T extends Record<string, any>>(
	mappings: Record<keyof T, string>,
) {
	return new Proxy(
		{} as unknown as {
			[K in keyof T]: __EventObj__<T[K]> & {
				(handle: __WebviewWindow__): __EventObj__<T[K]>;
			};
		},
		{
			get: (_, event) => {
				const name = mappings[event as keyof T];

				return new Proxy((() => {}) as any, {
					apply: (_, __, [window]: [__WebviewWindow__]) => ({
						listen: (arg: any) => window.listen(name, arg),
						once: (arg: any) => window.once(name, arg),
						emit: (arg: any) => window.emit(name, arg),
					}),
					get: (_, command: keyof __EventObj__<any>) => {
						switch (command) {
							case "listen":
								return (arg: any) => TAURI_API_EVENT.listen(name, arg);
							case "once":
								return (arg: any) => TAURI_API_EVENT.once(name, arg);
							case "emit":
								return (arg: any) => TAURI_API_EVENT.emit(name, arg);
						}
					},
				});
			},
		},
	);
}
