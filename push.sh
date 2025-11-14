cargo doc
rm -rf doc
mv target/doc docs
git add .
git commit -m "$1"
git push