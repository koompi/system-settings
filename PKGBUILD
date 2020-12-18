# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Your Name <youremail@domain.com>
pkgname=system_settings
pkgver=0.1.0
pkgrel=1
pkgdesc="KOOMPI OS System Settings"
arch=('x86_64')
url="https://github.com"
license=('MIT')
provides=('system_settings')
source=("git+https://github.com/koompi/system-settings")
md5sums=() #generate with 'makepkg -g'

build() {
	cargo build --release
}

package() {
	# cd "$srcdir/$pkgname-$pkgver"
	# make DESTDIR="$pkgdir/" install

    install "${srcdir}/"
}