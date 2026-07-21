pkgname='pf'
pkgver=2.5.0.r28.26a67d7
pkgrel=1
pkgdesc='A lightweight cat-like terminal file viewer with syntax highlighting and automatic binary detection'
arch=('x86_64')
url="https://github.com/yok1rai/pretty_files"
license=("GPL")
depends=("glibc")
makedepends=("cargo")
source=("git+https://github.com/yok1rai/pretty_files.git#branch=main")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/pretty_files"
  printf "%s.r%s.%s" \
    "$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)" \
    "$(git rev-list --count HEAD)" \
    "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/pretty_files"
  cargo build --release --locked
}

package() {
  cd "$srcdir/pretty_files"

  install -Dm755 "target/release/pf" \
    "$pkgdir/usr/bin/pf"

  install -Dm644 "man/pretty_files.1" \
    "$pkgdir/usr/share/man/man1/pf.1"
}
