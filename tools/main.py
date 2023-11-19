import time

from utils import get_authors, get_images_by_author_id, get_user_by_author_id, get_image_by_image_id, handler_user_raw, \
    handler_image_raw
import json

# 作者ID
authors = """
90940250
17418615
92997909
92831443
89097795
94391662
91171473
93521759
92725420
91986769
93897703
3360208
20544646
"""


def run():
    # 提取作者ID
    author_ids = get_authors(authors)
    items = []
    for author_id in author_ids:
        time.sleep(3)
        print('当前正在处理作者:', author_id)
        # 获取作者信息
        user = get_user_by_author_id(author_id)
        if user is None:
            print('作者信息未查到:', author_id)
            continue

        # 处理作者信息
        author = handler_user_raw(user)
        # 查询作者所有作品
        page = 1
        while True:
            print(f'查询作者{author_id}第{page}页作品')
            images = get_images_by_author_id(author_id, page)
            print(f'作者{author_id}第{page}页作品共有{len(images)}件')
            if len(images) == 0:
                break
            page += 1

            for image in images:
                time.sleep(2)
                # 处理图片原始数据
                item = handler_image_raw(image)
                print('正在获取图片链接:', item['id'])
                # 获取图片链接
                url = get_image_by_image_id(item['id'])
                if url is None:
                    print('图片未找到:', author_id, item['id'])
                    continue
                item['url'] = str(url)
                item['author'] = author
                items.append(item)

    print('一共收集了', len(items), '张图片')

    with open('../data/mock_images_v2.json', 'w', encoding='utf-8') as json_file:
        buf = '['
        for item in items:
            buf += '\n  ' + json.dumps(item, ensure_ascii=False) + ','
        buf = buf.removesuffix(',')
        buf += '\n]'
        json_file.write(buf)


if __name__ == '__main__':
    run()