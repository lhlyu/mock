import {Application, Router} from "https://deno.land/x/oak/mod.ts"
import {oakCors} from "https://deno.land/x/cors/mod.ts"
import images from './data/images.json' assert {type: 'json'}
import images2 from './data/images2.json' assert {type: 'json'}


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

const MOCK_IMAGES = shuffle(images2)


const router = new Router()

// ---------------- ----------------

router.get("/", ctx => {
    ctx.response.body = '一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。'
})


// ---------------- ----------------
router.get("/image/:page/:count", ctx => {
    ctx.response.type = "json"
    let page = Number(ctx.params.page)
    let count = Number(ctx.params.count)
    page = Number.isInteger(page) ? page : 0
    count = Number.isInteger(count) ? count : 0
    if (page <= 0 || count <= 0) {
        ctx.response.body = []
        return
    }
    const maxPage = Math.ceil(images.length / count)
    if (page > maxPage) {
        ctx.response.body = []
        return
    }
    if (count > 100) {
        count = 100
    }
    const start = (page - 1) * count
    const end = start + count
    ctx.response.body = images.slice(start, end)
})

// ---------------------------------
router.get("/images/:page/:size", ctx => {
    ctx.response.type = "json"
    let page = Number(ctx.params.page)
    let size = Number(ctx.params.size)
    page = Number.isInteger(page) ? page : 0
    size = Number.isInteger(size) ? size : 0

    const total = MOCK_IMAGES.length

    if (page <= 0 || size <= 0) {
        ctx.response.body = {
            page: 0,
            max: 0,
            size: 0,
            total: total,
            list: []
        }
        return
    }
    const max = Math.ceil(MOCK_IMAGES.length / size)
    if (page > max) {
        ctx.response.body = {
            page: page,
            max: max,
            size: size,
            total: total,
            list: []
        }
        return
    }
    if (size > 100) {
        size = 100
    }
    const start = (page - 1) * size
    const end = start + size

    ctx.response.body = {
        page: page,
        max: max,
        size: size,
        total: total,
        list: MOCK_IMAGES.slice(start, end)
    }
})

// ---------------- ----------------

const app = new Application()
app.use(oakCors({
    origin: '*',
    methods: 'OPTION,GET,HEAD,PUT,PATCH,POST,DELETE'
}))
app.use(router.routes())

console.info("web server listening on port 8080")
app.listen({port: 8080})

