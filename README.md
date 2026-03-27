### run ./build_all.sh to build

- sqlite-src-3510200 with ./configure --all --disable-amalgamation - were used to regenerate the entire sqlite source in order to generate CRust.
- Also, we need sqlite-src to pull in shell.c and all requires for the testfixture as well as tcl tests.

### Execution:
```bash
docker build -t sqlite-crust .
docker run -it --rm \
    --user 1000:1000 \
    -v $(pwd)/.credentials:/home/node/:Z \
    -v $(pwd):/workspace:Z \
    sqlite-crust
```

## For debug with Claude (from scripts folder)
docker run --rm \
    --user 1000:1000 \
    -v $(pwd)/../crust-sqlite/.credentials:/home/node/:Z \
    -v $(pwd)/../crust-sqlite:/workspace:Z \
    sqlite-crust -c "./build_all.sh"
