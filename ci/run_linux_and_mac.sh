case $(uname | tr '[:upper:]' '[:lower:]') in
  linux*)
    export APPVEYOR_OS_NAME=linux
    ;;
  darwin*)
    export APPVEYOR_OS_NAME=osx
    ;;
esac
if [ "$APPVEYOR_OS_NAME" = "linux" ]; then export PATH="/home/appveyor/.cargo/bin:$PATH"; fi
if [ "$APPVEYOR_OS_NAME" = "osx" ]; then export PATH="$HOME/.cargo/bin:$PATH"; fi
if [ "$APPVEYOR_OS_NAME" = "linux" ]; then MIRI_NIGHTLY=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-unknown-linux-gnu/miri); fi
if [ "$APPVEYOR_OS_NAME" = "osx" ]; then MIRI_NIGHTLY=nightly-$(curl -s https://rust-lang.github.io/rustup-components-history/x86_64-apple-darwin/miri); fi
echo "Installing latest nightly with Miri: $MIRI_NIGHTLY"
rustup set profile minimal
rustup default "$MIRI_NIGHTLY"
rustup component add miri
# The `-Zmiri-disable-isolation` is so Miri can access the system clock
# while calling `SystemTime::now()` in one of the tests.
export MIRIFLAGS="-Zmiri-disable-isolation"
cargo miri test
cargo clean