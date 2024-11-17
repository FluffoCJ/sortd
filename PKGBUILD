
pkgname=sortd
pkgver=1.0.0
pkgrel=1
pkgdesc="Organize your downloads folder into subfolders"
arch=('x86_64')
makedepends=('cargo') 
source=("https://github.com/FluffoCJ/sortd.git")
sha256sums=('SKIP')

build() {
    cd "$srcdir"
    cargo build --release --locked
}

package() {
    cd ..
    cd target
    cd release
    install -Dm755 "sortd" "$pkgdir/usr/bin/sortd"
}

