set -e
cargo test
git add .
git commit -m "$1"
git push