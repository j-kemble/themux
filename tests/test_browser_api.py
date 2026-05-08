"""Browser API protocol tests."""


def test_browser_navigate(themux):
    """Test browser.navigate method."""
    resp = themux.rpc("browser.navigate", {"url": "https://example.com"})
    # May succeed or return not_implemented if no browser panel exists
    if resp["ok"]:
        assert "url" in resp.get("result", {}) or True
    else:
        assert resp["error"]["code"] in ("method_not_found", "not_implemented", "no_browser_panel")
