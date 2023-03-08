dist:
    ./scripts/dist.sh

clean:
    cd pastebin-front && rm -rf node_modules
    cd pastebin-server && rm -rf target
    rm -rf dist
