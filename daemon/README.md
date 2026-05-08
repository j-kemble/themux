# cmuxd-remote Go daemon
#
# This directory should contain the cmuxd-remote Go module for
# remote SSH workspace support. Copy or symlink from the cmux repo:
#
#   cp -r <cmux-repo>/daemon/remote daemon/cmuxd-remote
#
# cmuxd-remote provides:
# - WebSocket RPC server (PTY multiplexing, proxy streams)
# - JSON-RPC over stdio (for SSH transport)
# - Session management with multiple attachments
# - Proxy broker for remote browser traffic
#
# Build: cd daemon/cmuxd-remote && go build -o ../../build/cmuxd-remote ./cmd/cmuxd-remote/
