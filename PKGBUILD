pkgname='pretty_files'
pkgver=1.5.0.r12.01c2784
pkgrel=1
pkgdesc='A simple cat-like syntax highlighting file viewer'
arch=('x86_64')
url="https://github.com/yok1rai/pretty_files"
license=("GPL")
depends=("glibc")
makedepends=("cargo")
source=("git+https://github.com/yok1rai/pretty_files.git#branch=main")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  printf "%s.r%s.%s" "$(cat Cargo.toml | grep -m1 '^version' | cut -d'"' -f2)" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release --locked
}

package() {
  install -Dm755 "$srcdir/$pkgname/target/release/$pkgname" \
    "$pkgdir/usr/bin/$pkgname"
}
