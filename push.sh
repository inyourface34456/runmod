cargo doc
rm -rf docs
mv target/doc docs
git add .
git commit -m "$1"
git push