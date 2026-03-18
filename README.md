run ./build_all.sh to build

sqlite-src-3510200 with ./configure --all --disable-amalgamation - were used to regenerate the entire sqlite source in order to generate CRust.
Also, we need sqlite-src to pull in shell.c and all requires for the testfixture as well as tcl tests.