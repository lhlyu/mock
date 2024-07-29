import {Application, Router} from "https://deno.land/x/oak/mod.ts";
import {oakCors} from "https://deno.land/x/cors/mod.ts";
import images from './data/images.json' assert {type: 'json'};

// 下面第一个代理地址官方不让用，第二个消耗自己的流量扛不住
// 实在不行自己搭个：https://mirai.mamoe.net/topic/1322/rs-pixiv-%E6%98%93%E4%BA%8E%E6%90%AD%E5%BB%BA%E7%9A%84pixiv%E4%BB%A3%E7%90%86%E6%9C%8D%E5%8A%A1
// const proxy_base_url = "https://proxy.pixivel.moe/" // 不让用
// const proxy_base_url = 'https://pixiv.tatakai.top/'
// const proxy_base_url = "https://px.s.rainchan.win/"
const proxy_base_url = "https://i.pixiv.re/"

const handlerUrl = (url: string): string => {
    url = url.replace('_p0.', '_p0_master1200.')
    url = url.replace('.png', '.jpg')
    return proxy_base_url + 'c/540x540_70/img-master/' + url
}

const handlerAvatar = (url: string): string => {
    url = url.replace('.', '_50.')
    return proxy_base_url + url
}

function shuffle(array: unknown[]) {
    // 创建一个副本，以防修改原始数组
    const shuffledArray = array.slice();
    for (let i = shuffledArray.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        // 交换元素
        [shuffledArray[i], shuffledArray[j]] = [shuffledArray[j], shuffledArray[i]];
    }
    return shuffledArray;
}

function getInteger(val: string | null, def: number): number {
    if (val === null) {
        return def;
    }
    const v = Number.parseInt(val)
    if (Number.isNaN(v)) {
        return def;
    }
    return v;
}

const MOCK_IMAGES: any[] = shuffle(images);


const router = new Router();

// ---------------- ----------------

router.get("/", ctx => {
    ctx.response.body = '一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。';
})

// ---------------------------------
router.get("/images", ctx => {
    ctx.response.type = "json";
    const page = getInteger(ctx.request.url.searchParams.get('page'), 0);
    let size = getInteger(ctx.request.url.searchParams.get('size'), 0);
    // 获取数据的模式：simple | all
    const mode = ctx.request.url.searchParams.get('mode') ?? 'all';
    const total = MOCK_IMAGES.length;

    if (page <= 0 || size <= 0) {
        ctx.response.body = {
            page: 0,
            max: 0,
            size: 0,
            total: total,
            list: []
        }
        return;
    }
    const max = Math.ceil(MOCK_IMAGES.length / size);
    if (page > max) {
        ctx.response.body = {
            page: page,
            max: max,
            size: size,
            total: total,
            list: []
        }
        return;
    }
    if (size > 100) {
        size = 100;
    }
    const start = (page - 1) * size;
    const end = start + size;

    let list = [];

    switch (mode) {
        case 'simple':
            list = MOCK_IMAGES.slice(start, end).map(value => ({
                id: value.id,
                title: value.title,
                url: handlerUrl(value.url),
                width: value.width,
                height: value.height,
                avatar: handlerAvatar(value.author.avatar),
                user: value.author.name,
                views: value.statistic.views
            }));
            break;
        default:
            list = MOCK_IMAGES.slice(start, end);
            break;
    }

    ctx.response.body = {
        page: page,
        max: max,
        size: size,
        total: total,
        list: list
    }
})

// ---------------- ----------------

const app = new Application()
app.use(async (ctx, next) => {
    try {
        await next();
    } catch (error) {
        console.error("异常:", error.message);
        ctx.response.body = error.message;
        ctx.response.status = 500;
    }
})
app.use(oakCors({
    origin: '*',
    methods: 'OPTION,GET,HEAD,PUT,PATCH,POST,DELETE'
}))
app.use(router.routes())

console.info("web server listening on port 8080")
app.listen({port: 8080})

