# Maintainer: Aditya Gupta <me dot adityag15 at gmail dot com>
pkgname=leetcode-practice-queue-git
pkgver=r5.e10a810
pkgrel=1
pkgdesc="Keeps track of your 'to practice' list of leetcode questions"
arch=('x86_64')
url="https://github.com/adi-g15/leetcode-practice-queue"
license=('GPL3')
depends=()
makedepends=('gcc' 'git' 'cargo')
provides=("lsl")
source=('leetcode-practice-queue::git+https://github.com/adi-g15/leetcode-practice-queue')
md5sums=('SKIP')

pkgver() {
	cd "$srcdir/${pkgname%-git}"

	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
	cd "$srcdir/${pkgname%-git}"
        cargo build --release
}

package() {
	cd "$srcdir/${pkgname%-git}"
        cargo install --root "$pkgdir"/usr --path . --no-track
}
