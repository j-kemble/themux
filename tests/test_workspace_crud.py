"""Workspace CRUD tests."""


def test_workspace_list(themux):
    """Test listing workspaces."""
    resp = themux.rpc("workspace.list")
    assert resp["ok"] is True
    assert "workspaces" in resp["result"]


def test_workspace_create(themux):
    """Test creating a workspace."""
    resp = themux.rpc("workspace.create", {"name": "test", "cwd": "/tmp"})
    assert resp["ok"] is True
    assert "workspace_id" in resp["result"]
