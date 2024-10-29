#!/bin/sh
set -e

# Must set a deployment target so that Rust and Go agree
export MACOSX_DEPLOYMENT_TARGET=14.2

./build-rust.sh

# Fix for libolm through homebrew
if [[ -z "$LIBRARY_PATH" && -d /opt/homebrew ]]; then
	export LIBRARY_PATH=/opt/homebrew/lib:$LIBRARY_PATH
	export CPATH=/opt/homebrew/include:$CPATH
fi

export LIBRARY_PATH=.:$LIBRARY_PATH

cp -f pkg/rustpushgo/target/release/librustpushgo.a .

gofmt -s -w .
go mod tidy

# Build mautrix bridge
MAUTRIX_VERSION=$(cat go.mod | grep 'maunium.net/go/mautrix ' | awk '{ print $2 }' | head -n1)
GO_LDFLAGS="-s -w -X main.Tag=$(git describe --exact-match --tags 2>/dev/null) -X main.Commit=$(git rev-parse HEAD) -X 'main.BuildTime=`date -Iseconds`' -X 'maunium.net/go/mautrix.GoModVersion=$MAUTRIX_VERSION'"
go build -ldflags="$GO_LDFLAGS" ./cmd/imessage-rustpush "$@"
