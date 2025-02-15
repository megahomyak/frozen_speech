fs-relink-usr() (
    set -e
    while ! [ -d participants ]; do
        cd ..
    done
    cd participants
    read -p 'Username: ' USERNAME
    cd "$USERNAME"
    nvim links
)

fs-usr() (
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

fs-dsc() {
    cd contents/discussions
    read -p 'Name: ' NAME
    mkdir -p "$NAME"
    cd "$NAME"
    nvim content
}
_fs-dsc() {
    COMPREPLY=( $( ls contents/discussions | grep "^$2" ) );
}
complete -F _fs-dsc fs-dsc
