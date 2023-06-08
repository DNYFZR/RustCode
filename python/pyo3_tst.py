import json, rustython as rpy

rust = rpy.get(url="www.tst.com", header_key="key", header_value="value")
print(rust, type(rust))

rust = json.loads(rust)
print(rust, type(rust))