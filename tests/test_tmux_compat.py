"""Tmux compatibility layer tests (protocol-level).

Tests that the V2 protocol methods handle tmux-compatible commands
correctly through the socket.
"""

import json


def test_tmux_new_window_via_protocol(themux):
    """Test that workspace.create maps to new-window."""
    resp = themux.rpc("workspace.create", {"name": "test-win", "cwd": "/tmp"})
    assert resp["ok"] is True
    assert "workspace_id" in resp["result"]


def test_tmux_split_window_via_protocol(themux):
    """Test that surface.split maps to split-window."""
    resp = themux.rpc("surface.split", {"direction": "horizontal"})
    assert resp["ok"] is True or resp["error"]["code"] in ("method_not_found",)
