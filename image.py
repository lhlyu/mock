import json

# 读取json文件内容
with open('mock_images.json', 'r') as file:
    json_data = file.read()

# 解析json数据为对象
data = json.loads(json_data)

# 根据id保留唯一
unique_data = {item['id']: item for item in data}.values()

# 每个元素单独占一行
output_data = '[\n  ' + ',\n  '.join(json.dumps(item) for item in unique_data) + '\n]'

# 将结果输出到文件
with open('output.json', 'w') as file:
    file.write(output_data)