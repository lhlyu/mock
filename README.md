# mock

一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。

## Api

### 批量获取图片：`/images?page=1&size=1[&mode=<'all' | 'simple'>]`

- 请求示例1:

```
/images?page=1&size=1
```

- 返回结果1

```json5
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
      "width": 1280,
      "height": 1920,
      "tags": [
        "AIart",
        "女の子",
        "AIイラスト",
        "ロリ",
        "制服",
        "セーラー服"
      ],
      "statistic": {
        "bookmarks": 140,
        "likes": 78,
        "comments": 3,
        "views": 1457
      },
      "url": "img/2023/08/13/21/00/04/110798120_p0.png",
      "author": {
        "id": 93897703,
        "name": "AIしいね",
        "bio": "2023年5月より、AIイラストはじめました。\nいろいろと試行錯誤中のためクオリティが安定していませんがお許し下さい。\n\nFrom May 2023, I started AI illustration.\nPlease forgive me for not being stable in quality because I am in trial and error.\n\nhttps://lit.link/aishiine",
        "avatar": "user-profile/img/2023/05/06/10/25/24/24384457_ea7bab0639d071eb25bb7a216d1be04c.png",
        "background": "background/img/2023/05/07/12/39/30/93897703_2849dbf6a589cb58fafbb0c6265feb55.png"
      }
    }
  ]
}
```

- 请求示例2:

```
/images?page=1&size=1&mode=simple
```

- 返回结果2

```json5
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


- [图片处理参数](https://kidonng.notion.site/pixiv-0c5a8ce110be4913a9cd437f67977f88)

