poetry run sphinx-build -b html docs/source/ docs/build/html

mv docs/build/html/_autosummary docs/build/html/autosummary
mv docs/build/html/_modules docs/build/html/modules
mv docs/build/html/_sources docs/build/html/sources
mv docs/build/html/_static docs/build/html/static

LC_ALL=C find ./docs/build/html -type f -exec sed -i -e 's/_autosummary/.\/autosummary/g' {} +
LC_ALL=C find ./docs/build/html -type f -exec sed -i -e 's/_modules/.\/modules/g' {} +
LC_ALL=C find ./docs/build/html -type f -exec sed -i -e 's/_sources/.\/sources/g' {} +
LC_ALL=C find ./docs/build/html -type f -exec sed -i -e 's/_static/.\/static/g' {} +