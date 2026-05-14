use axum::{extract::Query, response::IntoResponse, routing::get, Json, Router};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::Mutex;

const PROXY_BASE_URL: &str = "https://i.pixiv.re/";
const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;
const MAX_PAGE_SIZE: u32 = 100;

/// 图片作者信息。
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    pub id: u64,
    pub name: String,
    pub bio: String,
    pub avatar: String,
    pub background: String,
}

/// 图片互动与访问统计。
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Statistic {
    pub bookmarks: u64,
    pub likes: u64,
    pub comments: u64,
    pub views: u64,
}

/// 原始图片信息，对齐 `data/images.json`。
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Image {
    pub id: u64,
    pub title: String,
    pub ts: u64,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub tags: Vec<String>,
    pub author: Author,
    pub statistic: Statistic,
}

/// 适合前端瀑布流展示的简化图片信息。
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

/// 图片列表分页响应。
#[derive(Debug, Serialize)]
pub struct ImagesResponse<T> {
    pub page: u32,
    pub max: u32,
    pub size: u32,
    pub total: usize,
    pub list: Vec<T>,
}

/// `/images` 查询参数。
#[derive(Debug, Deserialize)]
pub struct ImagesQuery {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub mode: Option<String>,
}

/// 转换为可直接访问的 Pixiv 代理缩略图地址。
fn proxied_image_url(url: &str) -> String {
    let mut url = url.to_string();
    url = url.replace("_p0.", "_p0_master1200.");
    url = url.replace(".png", ".jpg");
    format!("{PROXY_BASE_URL}c/540x540_70/img-master/{url}")
}

/// 转换为 Pixiv 代理头像地址，并使用 50px 缩略图。
fn proxied_avatar_url(url: &str) -> String {
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
        Self {
            seed: Cell::new(seed),
        }
    }

    fn next_u32(&self) -> u32 {
        let old = self.seed.get();
        let new = old.wrapping_mul(1664525).wrapping_add(1013904223);
        self.seed.set(new);
        new
    }
}

/// Fisher-Yates 洗牌算法（不依赖 rand 库）
fn shuffle_images(images: &mut [Image]) {
    let rng = SimpleRng::new(123456);
    for i in (1..images.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        images.swap(i, j);
    }
}

// 使用 Lazy 加载并打乱图片列表，只在首次访问时执行
static MOCK_IMAGES: Lazy<Mutex<Vec<Image>>> = Lazy::new(|| {
    let images: Vec<Image> = {
        let data = include_str!("../data/images.json");
        serde_json::from_str(data).expect("images.json is invalid")
    };
    let mut images = images;
    shuffle_images(&mut images);
    Mutex::new(images)
});

// MOCK_IMAGES 初始化后长度固定，单独缓存可避免每次请求重复加锁计算。
static MOCK_IMAGES_TOTAL: Lazy<usize> = Lazy::new(|| MOCK_IMAGES.lock().unwrap().len());

pub async fn images_handler(Query(query): Query<ImagesQuery>) -> impl IntoResponse {
    let page = query.page.unwrap_or(DEFAULT_PAGE).max(1);
    let size = query
        .size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE);
    let mode = query.mode.as_deref().unwrap_or("all");

    let total = *MOCK_IMAGES_TOTAL;
    let max = (total as u32 + size - 1) / size;

    if page > max {
        return Json(ImagesResponse::<Image>::new_empty(page, max, size, total)).into_response();
    }

    let images_lock = MOCK_IMAGES.lock().unwrap();
    let start = ((page - 1) * size) as usize;
    let end = (start + size as usize).min(total);

    if mode == "simple" {
        let list: Vec<ImageSimple> = images_lock[start..end]
            .iter()
            .map(|img| ImageSimple {
                id: img.id,
                title: img.title.clone(),
                url: proxied_image_url(&img.url),
                width: img.width,
                height: img.height,
                avatar: proxied_avatar_url(&img.author.avatar),
                user: img.author.name.clone(),
                views: img.statistic.views,
            })
            .collect();
        Json(ImagesResponse {
            page,
            max,
            size,
            total,
            list,
        })
        .into_response()
    } else {
        let list = images_lock[start..end].to_vec();
        Json(ImagesResponse {
            page,
            max,
            size,
            total,
            list,
        })
        .into_response()
    }
}

impl<T> ImagesResponse<T> {
    fn new_empty(page: u32, max: u32, size: u32, total: usize) -> Self {
        ImagesResponse {
            page,
            max,
            size,
            total,
            list: Vec::new(),
        }
    }
}

pub fn images_router() -> Router {
    Router::new().route("/images", get(images_handler))
}
