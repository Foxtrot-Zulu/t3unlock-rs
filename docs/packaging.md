# Packaging Guide (skeleton)

## Debian/Ubuntu (deb)
- See `packaging/deb/` for control/spec skeletons.

## RPM (Fedora/openSUSE)
- See `packaging/rpm/` for spec skeleton.

## Arch (PKGBUILD)
- See `packaging/arch/PKGBUILD`.

Remember to ship:
- `t3unlock` binary
- `t3unlock.1` man page
- Shell completions
- `contrib/udev/99-t3unlock.rules`
