# AWB HTTP API

Base URL: `http://localhost:8000`

All JSON responses follow:
```json
{ "success": true, "data": "...", "error": null }
```

---

## Health

### `GET /health-check`
Returns `200 OK` when the browser is alive, `503` otherwise.

```sh
curl http://localhost:8000/health-check
```

---

## Page

### `GET /page/title`
Returns the current page title.

```sh
curl http://localhost:8000/page/title
# { "success": true, "data": "Google", "error": null }
```

### `GET /page/url`
Returns the current page URL.

```sh
curl http://localhost:8000/page/url
```

### `GET /page/content`
Returns the full HTML of the current page.

```sh
curl http://localhost:8000/page/content
```

### `GET /page/screenshot`
Returns a raw PNG image of the current page.

```sh
curl http://localhost:8000/page/screenshot -o screenshot.png
```

### `POST /page/goto`
Navigate to a URL.

```sh
curl -X POST http://localhost:8000/page/goto \
  -H 'Content-Type: application/json' \
  -d '{"url": "https://example.com"}'
```

---

## Input

### `POST /input/click`
Click an element by CSS selector.

```sh
curl -X POST http://localhost:8000/input/click \
  -H 'Content-Type: application/json' \
  -d '{"selector": "#submit-btn"}'
```

### `POST /input/click/text`
Click an element by visible text.

```sh
curl -X POST http://localhost:8000/input/click/text \
  -H 'Content-Type: application/json' \
  -d '{"selector": "Sign in"}'
```

### `POST /input/hover`
Hover over an element by CSS selector.

```sh
curl -X POST http://localhost:8000/input/hover \
  -H 'Content-Type: application/json' \
  -d '{"selector": ".dropdown-trigger"}'
```

### `POST /input/fill`
Clear and fill an input field (simulates human typing).

```sh
curl -X POST http://localhost:8000/input/fill \
  -H 'Content-Type: application/json' \
  -d '{"selector": "input[name=email]", "keys": "user@example.com"}'
```

### `POST /input/keys`
Type into an element without clearing it first.

```sh
curl -X POST http://localhost:8000/input/keys \
  -H 'Content-Type: application/json' \
  -d '{"selector": "textarea", "keys": "Hello world"}'
```

### `POST /input/key`
Press a single keyboard key globally.

Common keys: `Enter`, `Tab`, `Escape`, `ArrowDown`, `ArrowUp`, `Backspace`, `Space`

```sh
curl -X POST http://localhost:8000/input/key \
  -H 'Content-Type: application/json' \
  -d '{"key": "Enter"}'
```

---

## Find

### `GET /find/<selector>`
Find element by CSS selector, returns outer HTML.

```sh
curl 'http://localhost:8000/find/h1'
curl 'http://localhost:8000/find/.product-title'
```

### `GET /find/all/<selector>`
Find all matching elements (WIP — returns placeholder).

### `GET /find/text/<text>`
Find element containing text, returns its text content.

```sh
curl 'http://localhost:8000/find/text/Add%20to%20Cart'
```

### `GET /find/all/text/<text>`
Find all elements containing text (WIP — returns placeholder).

### `GET /find/element-text/<selector>`
Get text content of element matched by CSS selector.

```sh
curl 'http://localhost:8000/find/element-text/.price'
```

### `GET /find/exists/<selector>`
Returns `"true"` or `"false"` depending on whether the element exists.

```sh
curl 'http://localhost:8000/find/exists/%23modal'
# { "success": true, "data": "false", "error": null }
```

---

## Wait

### `GET /wait/<delay_ms>`
Wait for a fixed number of milliseconds.

```sh
curl http://localhost:8000/wait/1000
```

### `GET /wait/network/idle/<idle_ms>/<timeout_ms>`
Wait until network has been idle for `idle_ms` ms, or until `timeout_ms` elapses.

```sh
curl http://localhost:8000/wait/network/idle/500/10000
```

---

## MCP Access (for Claude agents)

The `mcp-server.py` in this repo exposes all endpoints above as native Claude tools. The `.mcp.json` file registers it automatically for any Claude Code session in this directory.

Start AWB locally first:
```sh
cargo run
# or
docker-compose up
```

Then in any Claude Code session inside this project, the following tools are available:

| Tool | Description |
|------|-------------|
| `awb_health` | Check browser is alive |
| `awb_goto` | Navigate to URL |
| `awb_get_title` | Current page title |
| `awb_get_url` | Current page URL |
| `awb_get_content` | Full page HTML |
| `awb_screenshot` | PNG screenshot |
| `awb_click` | Click by selector |
| `awb_click_text` | Click by text |
| `awb_hover` | Hover by selector |
| `awb_fill` | Clear + fill input |
| `awb_type` | Type into element |
| `awb_press_key` | Press global key |
| `awb_find` | Find element HTML |
| `awb_find_text` | Find element by text |
| `awb_element_text` | Get element text |
| `awb_element_exists` | Check element presence |
| `awb_wait` | Fixed delay |
| `awb_wait_for_network_idle` | Wait for network quiet |
