import json
import os

SRC_DIR = "./logs"
DST_DIR = "./logs/json"
ENCODING = "utf-8"


def decision() -> str:
    lists = os.listdir(SRC_DIR)
    for a in lists:
        if not a.startswith("youtube_api"):
            lists.remove(a)
    lists.sort()
    return os.path.join(SRC_DIR, lists[0])


def dst_path(src_path: str) -> str:
    basename = os.path.basename(src_path)
    return os.path.join(DST_DIR, basename) + ".json"


def log_to_json(src_path: str) -> str:
    with open(src_path, encoding=ENCODING) as f:
        content = f.read()
    # 個々のログがjson形式になっており改行で分割されているので
    # これらを配列の要素にする
    return f"[{content.replace('\n', ',').strip(',')}]"


def save_as_json(save_path: str, content: str):
    with open(save_path, encoding=ENCODING, mode="w") as f:
        json.dump(json.loads(content), f, ensure_ascii=False, indent=2)


src_path = decision()
content = log_to_json(src_path)
save_as_json(dst_path(src_path), content)
print("process finished!")
