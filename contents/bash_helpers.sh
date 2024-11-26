# Add these to your bashrc!

saveimg() {
    python -m cbsave "$1"
    cwebp -q 80 "$1" -o "$1"
}
