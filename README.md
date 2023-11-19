# mock

一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。

## Api

### 批量获取图片: `/image/:page/:count`

- 请求示例:

```
/image/1/2
```

- 返回结果

```json
[
  {
    "id": "95667953",
    "height": 4093,
    "width": 2894,
    "img": "https://pixiv.yuki.sh/c/540x540_70/img-master/img/2022/01/21/13/12/59/95667953_p0_master1200.jpg"
  },
  {
    "id": "95860473",
    "height": 3252,
    "width": 2508,
    "img": "https://px.s.rainchan.win/c/540x540_70/img-master/img/2022/01/29/22/53/57/95860473_p0_master1200.jpg"
  }
]
```

### 批量获取图片2：`/images/:page/:size`

- 请求示例:

```
/images/1/20
```

- 返回结果

```json5
{
  "page": 1,
  "max": 3198,
  "size": 1,
  "total": 3198,
  "list": [
    {
      "id": 113082486,
      "title": "「見えて嬉しいものなの？......ふーん、そっか」",
      "ts": 1698904380000,
      "width": 800,
      "height": 1440,
      "tags": [
        "女の子",
        "美少女",
        "girl",
        "JK",
        "パンチラ",
        "女子高生",
        "片目隠れ",
        "ショートカット",
        "制服"
      ],
      "statistic": {
        "bookmarks": 114,
        "likes": 64,
        "comments": 1,
        "views": 512
      },
      "url": "img/2023/11/02/22/53/38/113082486_p0.png",
      "author": {
        "id": 3360208,
        "name": "カムカム",
        "bio": "普段はASMR音声作品を制作しています。気になった方はTwitterフォローよろしくです。\n\n投稿するイラストはNovelAIちゃんが描いています。\n私は偉そうに指示出すだけのシナリオライターです。",
        "avatar": "user-profile/img/2022/10/13/01/17/14/23455511_1c7f0662c01d7be794b9b5b6a8e18bd2.png",
        "background": "background/img/2022/10/13/01/17/12/3360208_85decf2c2d9203d6ae571606502e3459.png"
      }
    }
  ]
}
```

- [图片处理参数](https://kidonng.notion.site/pixiv-0c5a8ce110be4913a9cd437f67977f88)

