#!/usr/bin/env bash

set -eo pipefail

export PATH="/root/.cargo/bin:$PATH"
export RUSTFLAGS="-C debuginfo=line-tables-only"
export CARGO_TARGET_DIR="$PWD/target"
export VARNISH_INCLUDE_PATHS="/usr/include/varnish"
export PYTHONPATH="$PYTHON_PATH:/usr/share/varnish"

VMOD_VERSION=$(cargo metadata --no-deps --format-version 1 | jq '.packages[0].version' -r)
VARNISH_MINOR=$(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "varnish-sys") | .metadata.libvarnishapi.version ')
VARNISH_PATCH=0
VARNISH_VERSION="$VARNISH_MINOR.$VARNISH_PATCH"

#cargo install --locked cargo-about
#cargo doc
cargo build #--release
cargo test #--release

CARGO_PACKAGES="$(ls | grep vmod-)"

for pkg in $CARGO_PACKAGES; do
  VMOD_VERSION=$(cargo metadata --no-deps --format-version 1 --manifest-path $pkg/Cargo.toml | jq '.packages[0].version' -r)
  VARNISH_MINOR=$(cargo metadata --format-version 1 --manifest-path $pkg/Cargo.toml | jq -r '.packages[] | select(.name == "varnish-sys") | .metadata.libvarnishapi.version ')
  VARNISH_PATCH=0
  VARNISH_VERSION="$VARNISH_MINOR.$VARNISH_PATCH"

  BUILD_ARCH="all"
  BUILD_ROOT="release/$pkg-$VMOD_VERSION"

  rm -rf $BUILD_ROOT

  echo "creating folders at $BUILD_ROOT"
  mkdir -p $BUILD_ROOT
  mkdir -p $BUILD_ROOT/opt
  mkdir -p $BUILD_ROOT/var
  mkdir -p $BUILD_ROOT/usr/lib/varnish/vmods
  mkdir -p $BUILD_ROOT/etc/$pkg

  echo "creating application files at $BUILD_ROOT from $PWD"
  cp LICENSE $BUILD_ROOT/etc/$pkg/LICENSE
  cp $pkg/README.md $BUILD_ROOT/etc/$pkg/README.md
  cp target/debug/lib$(echo $pkg | sed -e 's/-/_/g').so $BUILD_ROOT/usr/lib/varnish/vmods # $(pkg-config  --variable=vmoddir varnishapi)

  echo "creating debian files at $BUILD_ROOT"
  mkdir -p $BUILD_ROOT/debian
  touch $BUILD_ROOT/debian/prerm
  touch $BUILD_ROOT/debian/preinst
  touch $BUILD_ROOT/debian/postinst
  touch ${BUILD_ROOT}/debian/control

  echo ""

  echo "Package: $pkg" >>${BUILD_ROOT}/debian/control
  echo "Architecture: ${BUILD_ARCH}" >>${BUILD_ROOT}/debian/control
  echo "Maintainer: TheAuthors <the.authors@acme.corp>" >>${BUILD_ROOT}/debian/control
  echo "Section: utils" >>${BUILD_ROOT}/debian/control
  echo "Priority: optional" >>${BUILD_ROOT}/debian/control
  echo "Description: VMOD ${pkg}" >>${BUILD_ROOT}/debian/control
  echo -n 'Version: ' >>${BUILD_ROOT}/debian/control
  echo -n ${VMOD_VERSION} >>${BUILD_ROOT}/debian/control
  echo "" >>${BUILD_ROOT}/debian/control

  echo "Setting permissioning as required by debian"
  cd ${BUILD_ROOT}/..
  find . -type d | xargs chmod 755
  cd ~-
  chmod 755 ${BUILD_ROOT}/debian/*

  # the below permissioning is required by debian
  cd ${BUILD_ROOT}
  tar czf data.tar.gz opt etc usr var --owner=0 --group=0
  cd ~-
  cd ${BUILD_ROOT}
  tar czf control.tar.gz debian/control debian/postinst debian/preinst debian/prerm --owner=0 --group=0
  cd ~-

  echo "Creating the debian package"
  echo "Constructing the deb packagage"

  mkdir -p ${BUILD_ROOT}/bin
  ar r ${BUILD_ROOT}/bin/$pkg-${VMOD_VERSION}-1.deb ${BUILD_ROOT}/control.tar.gz
  ar r ${BUILD_ROOT}/bin/$pkg-${VMOD_VERSION}-1.deb ${BUILD_ROOT}/data.tar.gz

  dpkg-deb --build release/$pkg-$VMOD_VERSION

#   echo "Creating rpm files at ${BUILD_ROOT}"

#   mkdir -p ${BUILD_ROOT}/rpm
#   cat << EOF > ${BUILD_ROOT}/rpm/$pkg.spec
# Name:       libvmod-$pkg
# Version:    $VMOD_VERSION
# Release:    1%{?dist}
# Summary:    VMOD $pkg
# License:    Apache-2.0
# URL:        https://github.com/thomasklinger12345/vmods
# Source0:    https://github.com/thomasklinger12345/vmods/releases/%{name}-%{version}.tar.gz
# BuildArch:  noarch

# %description
# VMOD $pkg.

# %prep
# %setup -q

# %build

# %install
# mkdir -p %{buildroot}/%{_libdir}/varnish/vmods
# mkdir -p %{buildroot}/%{_sysconfdir}/%{name}
# install -m 644 usr/lib/varnish/vmods/lib$(echo $pkg | sed -e 's/-/_/g').so %{_libdir}/varnish/vmods/lib$(echo $pkg | sed -e 's/-/_/g').so

# %files
# %license LICENSE
# %{_sysconfdir}/%{name}/README.md
# %{_libdir}/varnish/vmods/lib$(echo $pkg | sed -e 's/-/_/g').so

# %changelog
# EOF

#   rpmbuild -ba --build-in-place --define "_topdir ${BUILD_ROOT}/rpm" ${BUILD_ROOT}/rpm/$pkg.spec
#   #mv ${BUILD_ROOT}/rpm/SRPMS/*.rpm .
#   #mv ${BUILD_ROOT}/rpm/RPMS/*/*.rpm .
#   #rm -rf ${BUILD_ROOT}/rpm
done
