pkgname="pretty_files"
pkgver=1.0.0
pkgrel=1
pkgdesc="A simple text file viewer"
arch=("x86_64")
license=("MIT")
depends=("glibc")
makedepends=("cargo" "oniguruma")
build() {
    cargo build --release
}

package() {
    install -Dm755 "$startdir/target/release/pretty_files" \
        "$pkgdir/usr/bin/pretty_files"
}
