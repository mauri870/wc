pkgname=wc-rs
pkgver=0.1.0
pkgrel=0
arch=('x86_64')
pkgdesc='Print newline, word, and byte counts for each file'
url='https://github.com/mauri870/wc'
license=('MIT')
makedepends=('rust' 'git')
source=("${pkgname}::git+${url}.git")
md5sums=("SKIP")

build() {
    cd "${srcdir}/${pkgname}"
    cargo build --release
}

check() {
    cd "${srcdir}/${pkgname}"
    cargo test --release
}

package() {
    cd "${srcdir}/${pkgname}"
    install -Dm755 target/release/wc "$pkgdir"/usr/bin/"$pkgname"
}