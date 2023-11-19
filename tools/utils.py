from typing import List, Any, Dict
from datetime import datetime
import requests, json

__all__ = ['get_authors', 'get_images_by_author_id', 'get_user_by_author_id', 'get_image_by_image_id', 'handler_user_raw', 'handler_image_raw']


def get_authors(s: str) -> List[str]:
    """
    将作者列表按分行切割
    :param s:
    :return:
    """
    lines = s.strip().splitlines()
    items = []
    for line in lines:
        line = line.strip()
        if line == '':
            continue
        items.append(line)
    return list(set(items))


headers = {
    "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) "
                  "Chrome/117.0.0.0 Safari/537.36"
}


def get_images_by_author_id(author_id: str, page: int = 1) -> List[Any]:
    """
    根据作者id获取他的作品
    :param author_id:
    :param page:
    :return:
    """

    try:
        response = requests.get(
            f"https://api.pixivel.moe/v2/pixiv/user/{author_id}/illusts?page={page}",
            headers=headers
        )
        if response.status_code == 200:
            return response.json()['data']['illusts']
        return []
    except requests.exceptions.RequestException as e:
        # 捕获异常，打印错误信息
        print(f"get_images_by_author_id -> Error: {e}")
    return []


def get_user_by_author_id(author_id: str) -> Any:
    """
    获取用户信息
    :param author_id:
    :return:
    """
    try:
        response = requests.get(
            f"https://api.pixivel.moe/v2/pixiv/user/{author_id}",
            headers=headers
        )
        if response.status_code == 200:
            return response.json()['data']
        return None
    except requests.exceptions.RequestException as e:
        # 捕获异常，打印错误信息
        print(f"get_user_by_author_id -> Error: {e}")
    return None


def get_image_by_image_id(image_id: str):
    """
    获取图片信息
    :param image_id:
    :return:
    """
    try:
        response = requests.get(
            f"https://pixiv.shojo.cn/{image_id}",
        )
        if response.status_code == 200:
            return response.request.url.replace('https://proxy.pixiv.shojo.cn/img-original/', '')
        if response.status_code == 403:
            start = response.text.rfind('[')
            if start == -1:
                return None
            end = response.text.rfind(']')
            if end == -1:
                return None
            arr = json.loads(response.text[start: end + 1])
            return arr[0].replace('https://i.pximg.net/img-original/', '')
        print(response.status_code, response.text)
        return None
    except requests.exceptions.RequestException as e:
        # 捕获异常，打印错误信息
        print(f"get_image_by_image_id -> Error: {e}")
    return None


def handler_user_raw(v: Any) -> Dict[str, Any]:
    """
    处理作者信息原始数据
    :param v:
    :return:
    """
    avatar = ''
    background = ''

    try:
        avatar = str(v['image']['url'])
        avatar = avatar.removeprefix('https://i.pximg.net/')
        avatar = avatar.replace('_50.', '.')

        background = str(v['image']['background'])
        background = background[background.find('background'):]
    except Exception as e:
        pass

    return {
        'id': v['id'],
        'name': v['name'],
        'bio': v['bio'],
        'avatar': avatar,
        'background': background,
    }


def handler_image_raw(v: Any) -> Dict[str, Any]:
    """
    处理作品原始数据
    :param url:
    :param v:
    :return:
    """
    tags = []
    for tag in v['tags']:
        tags.append(tag['name'])

    return {
        'id': v['id'],
        'title': v['title'],
        'ts': int(datetime.strptime(v['createDate'], "%Y-%m-%dT%H:%M:%S").timestamp() * 1000),
        'width': v['width'],
        'height': v['height'],
        'tags': tags,
        'statistic': {
            'bookmarks': v['statistic']['bookmarks'],
            'likes': v['statistic']['likes'],
            'comments': v['statistic']['comments'],
            'views': v['statistic']['views'],
        }
    }

