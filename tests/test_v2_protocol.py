"""V2 JSON-RPC protocol compatibility tests."""

import json
import uuid


def test_system_ping(themux):
    """Test system.ping returns pong."""
    resp = themux.rpc("system.ping")
    assert resp["ok"] is True
    assert resp["result"]["pong"] is True


def test_system_identify(themux):
    """Test system.identify returns server info."""
    resp = themux.rpc("system.identify")
    assert resp["ok"] is True
    assert resp["result"]["server"] == "themux"
    assert "version" in resp["result"]


def test_system_capabilities(themux):
    """Test system.capabilities returns feature list."""
    resp = themux.rpc("system.capabilities")
    assert resp["ok"] is True
    assert "protocols" in resp["result"]
    assert "features" in resp["result"]


def test_unknown_method(themux):
    """Test unknown methods return errors."""
    resp = themux.rpc("nonexistent.method")
    assert resp["ok"] is False
    assert resp["error"]["code"] == "method_not_found"
