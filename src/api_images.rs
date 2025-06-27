use axum::{
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::cell::Cell;

// 下面第一个代理地址官方不让用，第二个消耗自己的流量扛不住
// 实在不行自己搭个：https://mirai.mamoe.net/topic/1322/rs-pixiv-%E6%98%93%E4%BA%8E%E6%90%AD%E5%BB%BA%E7%9A%84pixiv%E4%BB%A3%E7%90%86%E6%9C%8D%E5%8A%A1
// const proxy_base_url = "https://proxy.pixivel.moe/" // 不让用
// const proxy_base_url = 'https://pixiv.tatakai.top/'
// const proxy_base_url = "https://px.s.rainchan.win/"
static PROXY_BASE_URL: &str = "https://i.pixiv.re/";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    pub name: String,
    pub avatar: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Statistic {
    pub views: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub author: Author,
    pub statistic: Statistic,
}

#[derive(Debug, Serialize)]
pub struct ImageSimple {
    pub id: u64,
    pub title: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub avatar: String,
    pub user: String,
    pub views: u64,
}

#[derive(Debug, Serialize)]
pub struct ImagesResponse<T> {
    pub page: u32,
    pub max: u32,
    pub size: u32,
    pub total: usize,
    pub list: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct ImagesQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub mode: Option<String>,
}

fn handler_url(mut url: String) -> String {
    url = url.replace("_p0.", "_p0_master1200.");
    url = url.replace(".png", ".jpg");
    format!("{PROXY_BASE_URL}c/540x540_70/img-master/{url}")
}

fn handler_avatar(url: &str) -> String {
    let parts = url.rsplitn(2, '.').collect::<Vec<_>>();
    if parts.len() == 2 {
        format!("{PROXY_BASE_URL}{}_50.{}", parts[1], parts[0])
    } else {
        format!("{PROXY_BASE_URL}{url}")
    }
}

/// 简单线性同余伪随机数生成器（WASM兼容）
struct SimpleRng {
    seed: Cell<u32>,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { seed: Cell::new(seed) }
    }

    fn next_u32(&self) -> u32 {
        let old = self.seed.get();
        let new = old.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed.set(new);
        new
    }
}

/// Fisher-Yates 洗牌（不依赖 rand）
fn shuffle_images(images: &mut [Image]) {
    let rng = SimpleRng::new(123456); // 固定种子，需更随机可传入时间戳
    for i in (1..images.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        images.swap(i, j);
    }
}

// 加载并打乱图片列表
static MOCK_IMAGES: Lazy<Mutex<Vec<Image>>> = Lazy::new(|| {
    let images: Vec<Image> = {
        let data = include_str!("../data/images.json");
        serde_json::from_str(data).expect("images.json is invalid")
    };
    let mut images = images;
    shuffle_images(&mut images);
    Mutex::new(images)
});

fn get_integer(val: Option<u32>, def: u32) -> u32 {
    val.unwrap_or(def)
}

pub async fn images_handler(Query(query): Query<ImagesQuery>) -> axum::response::Response {
    let page = get_integer(query.page, 0);
    let mut size = get_integer(query.size, 0);
    let mode = query.mode.as_deref().unwrap_or("all");

    let images = MOCK_IMAGES.lock().unwrap();
    let total = images.len();

    if page == 0 || size == 0 {
        if mode == "simple" {
            let resp: ImagesResponse<ImageSimple> = ImagesResponse {
                page,
                max: 0,
                size: 0,
                total,
                list: vec![],
            };
            return (StatusCode::OK, Json(resp)).into_response();
        } else {
            let resp: ImagesResponse<Image> = ImagesResponse {
                page,
                max: 0,
                size: 0,
                total,
                list: vec![],
            };
            return (StatusCode::OK, Json(resp)).into_response();
        }
    }

    if size > 100 {
        size = 100;
    }

    let max = (total as u32 + size - 1) / size;

    if page > max {
        if mode == "simple" {
            let resp: ImagesResponse<ImageSimple> = ImagesResponse {
                page,
                max,
                size,
                total,
                list: vec![],
            };
            return (StatusCode::OK, Json(resp)).into_response();
        } else {
            let resp: ImagesResponse<Image> = ImagesResponse {
                page,
                max,
                size,
                total,
                list: vec![],
            };
            return (StatusCode::OK, Json(resp)).into_response();
        }
    }

    let start = ((page - 1) * size) as usize;
    let end = (start + size as usize).min(total);

    if mode == "simple" {
        let list: Vec<ImageSimple> = images[start..end]
            .iter()
            .map(|img| ImageSimple {
                id: img.id.clone(),
                title: img.title.clone(),
                url: handler_url(img.url.clone()),
                width: img.width,
                height: img.height,
                avatar: handler_avatar(&img.author.avatar),
                user: img.author.name.clone(),
                views: img.statistic.views,
            })
            .collect();

        let resp = ImagesResponse {
            page,
            max,
            size,
            total,
            list,
        };
        (StatusCode::OK, Json(resp)).into_response()
    } else {
        let list = images[start..end].to_vec();
        let resp = ImagesResponse {
            page,
            max,
            size,
            total,
            list,
        };
        (StatusCode::OK, Json(resp)).into_response()
    }
}

pub fn images_router() -> Router {
    Router::new().route("/images", get(images_handler))
}
