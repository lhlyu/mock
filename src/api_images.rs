use axum::{
    extract::Query,
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

/// 作者信息结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    pub name: String,
    pub avatar: String,
}

/// 统计信息结构体
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Statistic {
    pub views: u64,
}

/// 完整图片信息结构体
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

/// 简化图片信息结构体，用于前端展示
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

/// 图片列表响应结构体，T 可以是 Image 或 ImageSimple
#[derive(Debug, Serialize)]
pub struct ImagesResponse<T> {
    pub page: u32,   // 当前页码
    pub max: u32,    // 最大页码
    pub size: u32,   // 每页大小
    pub total: usize,// 总图片数量
    pub list: Vec<T>,// 图片列表
}

/// 图片查询参数结构体
#[derive(Debug, Deserialize)]
pub struct ImagesQuery {
    pub page: Option<u32>, // 页码，可选
    pub size: Option<u32>, // 每页大小，可选
    pub mode: Option<String>,// 模式 ("simple" 或 "all")，可选
}

/// 处理图片 URL，添加代理前缀并进行格式转换
fn handler_url(mut url: String) -> String {
    url = url.replace("_p0.", "_p0_master1200.");
    url = url.replace(".png", ".jpg");
    format!("{PROXY_BASE_URL}c/540x540_70/img-master/{url}")
}

/// 处理作者头像 URL，添加代理前缀并进行尺寸转换
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

/// Fisher-Yates 洗牌算法（不依赖 rand 库）
fn shuffle_images(images: &mut [Image]) {
    let rng = SimpleRng::new(123456); // 固定种子，如需更随机可传入时间戳
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
    shuffle_images(&mut images); // 打乱图片顺序
    Mutex::new(images)
});

// 使用 Lazy 存储图片总数，MOCK_IMAGES 初始化后其长度是确定的，无需重复计算
static MOCK_IMAGES_TOTAL: Lazy<usize> = Lazy::new(|| {
    MOCK_IMAGES.lock().unwrap().len()
});

/// 获取 u32 类型的整数值，如果 Option 为 None 则返回默认值
fn get_integer(val: Option<u32>, def: u32) -> u32 {
    val.unwrap_or(def)
}

pub async fn images_handler(Query(query): Query<ImagesQuery>) -> impl IntoResponse {
    let page = get_integer(query.page, 0);
    let mut size = get_integer(query.size, 0);
    let mode = query.mode.as_deref().unwrap_or("all");

    let total = *MOCK_IMAGES_TOTAL;

    // 如果页码或每页大小为 0，直接返回空列表
    if page == 0 || size == 0 {
        return Json(ImagesResponse::<Image>::new_empty(page, 0, 0, total)).into_response();
    }

    // 限制每页大小不超过 100
    if size > 100 {
        size = 100;
    }

    // 计算最大页码
    let max = (total as u32 + size - 1) / size;

    // 如果请求的页码超出最大页码，返回空列表
    if page > max {
        return Json(ImagesResponse::<Image>::new_empty(page, max, size, total)).into_response();
    }

    // 获取 MOCK_IMAGES 的锁
    let images_lock = MOCK_IMAGES.lock().unwrap();

    // 计算切片的起始和结束索引
    let start = ((page - 1) * size) as usize;
    let end = (start + size as usize).min(total); // 确保结束索引不超过总数

    // 根据模式返回不同类型的图片列表
    if mode == "simple" {
        let list: Vec<ImageSimple> = images_lock[start..end] // 切片获取当前页的图片
            .iter()
            .map(|img| ImageSimple { // 将 Image 转换为 ImageSimple
                id: img.id,
                title: img.title.clone(),
                url: handler_url(img.url.clone()), // 处理图片 URL
                width: img.width,
                height: img.height,
                avatar: handler_avatar(&img.author.avatar), // 处理头像 URL
                user: img.author.name.clone(),
                views: img.statistic.views,
            })
            .collect();
        Json(ImagesResponse { page, max, size, total, list }).into_response() // 构建并返回响应
    } else {
        let list = images_lock[start..end].to_vec(); // 直接获取 Image 列表
        Json(ImagesResponse { page, max, size, total, list }).into_response() // 构建并返回响应
    }
}

// 为 ImagesResponse 添加一个辅助方法，用于创建空的响应
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