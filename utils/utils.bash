fs-change-usr-links() (
    set -e
    while ! [ -d participants ]; do
        cd ..
    done
    cd participants
    read -p 'Username: ' USERNAME
    cd "$USERNAME"
    nvim links
)

fs-new-usr() (
    set -e
    while ! [ -d participants ]; do
        cd ..
    done
    cd participants
    read -p 'Username: ' USERNAME
    mkdir "$USERNAME"
    cd "$USERNAME"
    python -m cbsave pfp
    nvim links
)

fs-img() (
    set -e
    read -p 'File name (without extension): ' FILE_NAME
    python -m cbsave -e "$FILE_NAME"
    ls | grep --color=never "^$FILE_NAME\."
)
