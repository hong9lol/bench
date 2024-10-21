#!/bin/bash

# 실행할 파일의 경로
EXECUTABLE="./run_test.sh"  # 여기서 your_executable은 실제 실행 파일로 변경하세요.

# 문자열 리스트
STRING_LIST=(
    "global_variable" 
    "global_struct" 
)

# 리스트의 각 항목을 인자로 사용하여 실행
for ARG in "${STRING_LIST[@]}"; do
    echo "Bench: $ARG"
    $EXECUTABLE "$ARG"  # 실행 파일을 호출하고 인자로 ARG를 전달
    echo
done