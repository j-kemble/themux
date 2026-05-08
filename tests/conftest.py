# themux Python integration test configuration.
#
# Tests communicate with the themux socket server (V2 JSON-RPC).
# Start themux before running: ./scripts/run.sh

import pytest
import os
import socket
import json
import uuid


def _resolve_socket_path():
    """Resolve the themux socket path from env or default."""
    return os.environ.get(
        "THEMUX_SOCKET_PATH",
        os.path.expanduser("~/.local/share/themux/themux.sock"),
    )


def _resolve_password():
    """Resolve socket password from env or file."""
    pw = os.environ.get("THEMUX_SOCKET_PASSWORD")
    if pw:
        return pw
    pw_file = os.path.expanduser("~/.local/share/themux/socket-password")
    if os.path.exists(pw_file):
        with open(pw_file) as f:
            return f.read().strip()
    return None


def _send_v2(sock, method, params=None):
    """Send a V2 JSON-RPC request and return the response."""
    request = {"id": str(uuid.uuid4()), "method": method, "params": params or {}}
    sock.sendall((json.dumps(request) + "\n").encode())
    response_line = b""
    while b"\n" not in response_line:
        chunk = sock.recv(4096)
        if not chunk:
            raise ConnectionError("Socket closed")
        response_line += chunk
    return json.loads(response_line.decode())


class ThemuxConnection:
    """Fixture wrapper for themux socket connection."""

    def __init__(self):
        self.sock = None

    def connect(self):
        self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.sock.settimeout(5)
        self.sock.connect(_resolve_socket_path())

        # Authenticate if password is set
        pw = _resolve_password()
        if pw:
            self.sock.sendall(f"auth {pw}\n".encode())
            response = self.sock.recv(1024).decode().strip()
            if response != "OK":
                raise PermissionError(f"Auth failed: {response}")

    def disconnect(self):
        if self.sock:
            self.sock.close()
            self.sock = None

    def rpc(self, method, params=None):
        return _send_v2(self.sock, method, params)


@pytest.fixture
def themux():
    """Create a themux socket connection for testing."""
    conn = ThemuxConnection()
    conn.connect()
    yield conn
    conn.disconnect()
