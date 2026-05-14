# mock

一个部署在 Cloudflare Workers 上的轻量 Mock API，当前提供图片列表接口，适合前端开发、瀑布流页面和接口联调时快速取数。

> 一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。

## 功能

- `/`：返回一句固定文本，用于快速确认服务可用。
- `/images`：返回分页图片数据，支持完整数据和前端友好的简化数据。
- 图片数据来自 [data/images.json](data/images.json)，服务启动时会按固定种子打乱顺序，便于每次部署保持稳定表现。

## 本地开发

```bash
npx wrangler dev
```

启动后访问：

```text
http://localhost:8787/images?page=1&size=20&mode=simple
```

## 接口

### `GET /images`

查询图片列表。

| 参数 | 类型 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `page` | `number` | `1` | 页码，小于 `1` 时会按 `1` 处理 |
| `size` | `number` | `20` | 每页数量，最大 `100` |
| `mode` | `all \| simple` | `all` | `all` 返回原始字段，`simple` 返回前端展示常用字段 |

### 完整模式

```text
GET /images?page=1&size=1
```

```json
{
  "page": 1,
  "max": 3189,
  "size": 1,
  "total": 3189,
  "list": [
    {
      "id": 110798120,
      "title": "お兄さんが格好良すぎてドキドキしてます。",
      "ts": 1691899200000,
      "url": "img/2023/08/13/21/00/04/110798120_p0.png",
      "width": 1280,
      "height": 1920,
      "tags": ["AIart", "女の子", "AIイラスト"],
      "author": {
        "id": 93897703,
        "name": "AIしいね",
        "bio": "2023年5月より、AIイラストはじめました。",
        "avatar": "user-profile/img/2023/05/06/10/25/24/24384457_ea7bab0639d071eb25bb7a216d1be04c.png",
        "background": "background/img/2023/05/07/12/39/30/93897703_2849dbf6a589cb58fafbb0c6265feb55.png"
      },
      "statistic": {
        "bookmarks": 140,
        "likes": 78,
        "comments": 3,
        "views": 1457
      }
    }
  ]
}
```

### 简化模式

```text
GET /images?page=1&size=1&mode=simple
```

```json
{
  "page": 1,
  "max": 3189,
  "size": 1,
  "total": 3189,
  "list": [
    {
      "id": 110798120,
      "title": "お兄さんが格好良すぎてドキドキしてます。",
      "url": "https://i.pixiv.re/c/540x540_70/img-master/img/2023/08/13/21/00/04/110798120_p0_master1200.jpg",
      "width": 1280,
      "height": 1920,
      "avatar": "https://i.pixiv.re/user-profile/img/2023/05/06/10/25/24/24384457_ea7bab0639d071eb25bb7a216d1be04c_50.png",
      "user": "AIしいね",
      "views": 1457
    }
  ]
}
```

## 说明

`simple` 模式会把 Pixiv 图片和头像路径转换为可直接访问的代理地址，方便前端直接渲染。图片代理规则可参考：[图片处理参数](https://kidonng.notion.site/pixiv-0c5a8ce110be4913a9cd437f67977f88)。
