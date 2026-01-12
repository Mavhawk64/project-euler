if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <problem_number>"
    exit 1
fi

PROBLEM_NUM=$1

git add . && git commit -m "Problem $PROBLEM_NUM" && git push