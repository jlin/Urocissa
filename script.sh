#!/bin/bash

# 設定 .env 文件的路徑
ENV_FILE="./gallery-backend/.env"

# 確認 .env 文件存在
if [[ ! -f "$ENV_FILE" ]]; then
    echo "錯誤: 找不到文件 $ENV_FILE"
    exit 1
fi

# 讀取 SYNC_PATH 從 .env 文件
SYNC_PATH=$(grep -E '^SYNC_PATH\s*=' "$ENV_FILE" | sed 's/^SYNC_PATH\s*=\s*//')

# 檢查是否讀取到 SYNC_PATH
if [[ -z "$SYNC_PATH" ]]; then
    echo "錯誤: 未找到 SYNC_PATH 變數或其值為空"
    exit 1
fi

# 如果值有引號，去除引號
SYNC_PATH="${SYNC_PATH%\"}"
SYNC_PATH="${SYNC_PATH#\"}"

echo "原始 SYNC_PATH 是: $SYNC_PATH"

# 將 SYNC_PATH 以逗號分隔並轉換為陣列
IFS=',' read -ra PATHS <<< "$SYNC_PATH"

ABS_PATHS=()

# 獲取 .env 文件所在的目錄
ENV_DIR=$(dirname "$ENV_FILE")

for path in "${PATHS[@]}"; do
    # 去除前後空格
    path=$(echo "$path" | xargs)

    # 檢查路徑是否為絕對路徑
    if [[ "$path" = /* ]]; then
        abs_path="$path"
    else
        # 使用 realpath 將相對路徑轉為絕對路徑，基於 ENV_DIR
        if command -v realpath &> /dev/null; then
            abs_path=$(realpath -m "$ENV_DIR/$path")
        else
            # 如果 realpath 不存在，使用其他方法
            abs_path="$(cd "$ENV_DIR/$path" 2>/dev/null && pwd)"
            if [[ -z "$abs_path" ]]; then
                echo "警告: 無法解析路徑 $path"
                abs_path="$ENV_DIR/$path"
            fi
        fi
    fi

    ABS_PATHS+=("$abs_path")
done

# 將絕對路徑陣列轉換為以逗號分隔的字串
ABS_SYNC_PATH=$(IFS=, ; echo "${ABS_PATHS[*]}")

echo "絕對路徑 SYNC_PATH 是: $ABS_SYNC_PATH"

# 定義容器內的基礎路徑
CONTAINER_BASE_PATH="/Urocissa/gallery-backend"

# 定義預設的卷掛載（相對路徑）
PREDEFINED_VOLUMES=(
    "./gallery-backend/db:/Urocissa/gallery-backend/db"
    "./gallery-backend/object:/Urocissa/gallery-backend/object"
    "./gallery-backend/.env:/Urocissa/gallery-backend/.env"
    "./gallery-backend/Rocket.toml:/Urocissa/gallery-backend/Rocket.toml"
    "./gallery-frontend/config.ts:/Urocissa/gallery-frontend/config.ts"
)

# 準備格式化的預設卷掛載輸出
PREDEFINED_VOLUME_OUTPUT=""
for vol in "${PREDEFINED_VOLUMES[@]}"; do
    PREDEFINED_VOLUME_OUTPUT+=" -v \"$vol\""
done

# 準備格式化的動態卷掛載輸出
DYNAMIC_VOLUME_OUTPUT=""
for abs_path in "${ABS_PATHS[@]}"; do
    # 提取路徑的 basename 作為容器內的目標路徑
    basename=$(basename "$abs_path")
    container_path="$CONTAINER_BASE_PATH/$basename"

    DYNAMIC_VOLUME_OUTPUT+=" -v \"$abs_path\":\"$container_path\""
done

# 最終的 Docker Run 命令
DOCKER_RUN_COMMAND="docker run -it --rm${PREDEFINED_VOLUME_OUTPUT}${DYNAMIC_VOLUME_OUTPUT} -p 4000:4000 urocissa"

# 輸出最終的 Docker Run 命令
echo -e "\n生成的 Docker Run 命令:\n"
echo "$DOCKER_RUN_COMMAND"

# 提示用戶確認是否執行
read -p "是否立即執行此 Docker Run 命令？(y/n): " confirm
if [[ "$confirm" =~ ^[Yy]$ ]]; then
    echo -e "\n正在執行 Docker Run 命令...\n"
    eval "$DOCKER_RUN_COMMAND"

    # 檢查 Docker Run 命令是否成功
    if [[ $? -ne 0 ]]; then
        echo "錯誤: Docker Run 命令執行失敗"
        exit 1
    else
        echo "Docker 容器已成功啟動"
    fi
else
    echo "已取消執行 Docker Run 命令。"
fi
