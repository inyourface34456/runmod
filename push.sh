cargo doc
mv target/doc doc
git add .
git commit -m "$1"
git push