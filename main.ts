import { Application, Router, Status } from "https://deno.land/x/oak/mod.ts";
import { oakCors } from "https://deno.land/x/cors/mod.ts";
import mockImages from './mock_images.json' assert { type: "json" }

const router = new Router();

// ---------------- ----------------

router.get("/", ctx => {
    ctx.response.body = '一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。'
})


// ---------------- ----------------

interface MockImageOption {
    id: string
    height: number
    width: number
    img: string
}

const images:MockImageOption[] = mockImages
const imageSize = images.length
router.get("/image/:page/:count", ctx => {
    ctx.response.type = "json";

    let page = Number(ctx.params.page)
    let count = Number(ctx.params.count)

    page = Number.isInteger(page) ? page : 0
    count = Number.isInteger(count) ? count : 0

    if (page <= 0 || count <= 0) {
        ctx.response.body = []
        return
    }
    const maxPage = Math.ceil(imageSize / count)
    if (page > maxPage) {
        ctx.response.body = []
        return
    }
    if (count > 100) {
        count = 100
    }
    ctx.response.body = images.slice(page * count, page * count + count);
})

// ---------------- ----------------

const app = new Application();
app.use(oakCors({
    origin: '*',
    methods: 'OPTION,GET,HEAD,PUT,PATCH,POST,DELETE'
}));
app.use(router.routes());

console.info("web server listening on port 8080");
app.listen({ port: 8080 });

