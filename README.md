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
[{
  "id": "95667953",
  "height": 4093,
  "width": 2894,
  "img": "https://pixiv.yuki.sh/c/540x540_70/img-master/img/2022/01/21/13/12/59/95667953_p0_master1200.jpg"
} ,{
  "id": "95860473",
  "height": 3252,
  "width": 2508,
  "img": "https://px.s.rainchan.win/c/540x540_70/img-master/img/2022/01/29/22/53/57/95860473_p0_master1200.jpg"
}]
```
