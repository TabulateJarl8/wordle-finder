.PHONY: all

all: linux windows-x64

pre-build:
	mkdir -p dist/

linux: pre-build
	cargo build --release
	cp -f ./target/release/wordle_finder ./dist/wordle_finder
	strip ./dist/wordle_finder

windows-x64: pre-build
	cargo build --target x86_64-pc-windows-gnu --release
	strip ./target/x86_64-pc-windows-gnu/release/wordle_finder.exe
	cp -f ./winlib/WebView2Loader_x64.dll ./target/x86_64-pc-windows-gnu/release/build/WebView2Loader.dll
	rm -f ./dist/windows_x64.zip
	zip -rj \
		./dist/windows_x64.zip \
		./target/x86_64-pc-windows-gnu/release/build/WebView2Loader.dll \
		./target/x86_64-pc-windows-gnu/release/wordle_finder.exe \
		LICENSE \
		./winlib/WebView2LoaderDLL-LICENSE.txt \
		README.md

clean:
	rm -rf dist/
