#!/usr/bin/env python3
import asyncio
import base64
import httpx
from mcp.server import Server
from mcp.server.stdio import stdio_server
from mcp import types

AWB_BASE_URL = 'http://localhost:8000'

app = Server( 'awb' )


def _ok( text ):
    return [ types.TextContent( type = 'text', text = str( text ) ) ]


def _err( text ):
    return [ types.TextContent( type = 'text', text = f'Error: {text}' ) ]


async def _get( path ):
    async with httpx.AsyncClient( timeout = 30 ) as client:
        response = await client.get( f'{AWB_BASE_URL}{path}' )
        response.raise_for_status()
        return response


async def _post( path, body ):
    async with httpx.AsyncClient( timeout = 30 ) as client:
        response = await client.post( f'{AWB_BASE_URL}{path}', json = body )
        response.raise_for_status()
        return response


@app.list_tools()
async def list_tools():
    return [
        types.Tool(
            name = 'awb_health',
            description = 'Check if the browser is running and responsive.',
            inputSchema = { 'type': 'object', 'properties': {}, 'required': [] }
        ),
        types.Tool(
            name = 'awb_goto',
            description = 'Navigate the browser to a URL.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'url': { 'type': 'string', 'description': 'Full URL to navigate to (include https://)' }
                },
                'required': [ 'url' ]
            }
        ),
        types.Tool(
            name = 'awb_get_title',
            description = 'Get the title of the current page.',
            inputSchema = { 'type': 'object', 'properties': {}, 'required': [] }
        ),
        types.Tool(
            name = 'awb_get_url',
            description = 'Get the current page URL.',
            inputSchema = { 'type': 'object', 'properties': {}, 'required': [] }
        ),
        types.Tool(
            name = 'awb_get_content',
            description = 'Get the full HTML content of the current page.',
            inputSchema = { 'type': 'object', 'properties': {}, 'required': [] }
        ),
        types.Tool(
            name = 'awb_screenshot',
            description = 'Take a screenshot of the current page. Returns base64-encoded PNG.',
            inputSchema = { 'type': 'object', 'properties': {}, 'required': [] }
        ),
        types.Tool(
            name = 'awb_click',
            description = 'Click an element using a CSS selector.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector for the element to click' }
                },
                'required': [ 'selector' ]
            }
        ),
        types.Tool(
            name = 'awb_click_text',
            description = 'Click an element by its visible text content.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'text': { 'type': 'string', 'description': 'Visible text of the element to click' }
                },
                'required': [ 'text' ]
            }
        ),
        types.Tool(
            name = 'awb_hover',
            description = 'Hover over an element using a CSS selector.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector for the element to hover' }
                },
                'required': [ 'selector' ]
            }
        ),
        types.Tool(
            name = 'awb_fill',
            description = 'Fill a form field with text (clears existing value first, simulates human typing).',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector for the input element' },
                    'text': { 'type': 'string', 'description': 'Text to fill the field with' }
                },
                'required': [ 'selector', 'text' ]
            }
        ),
        types.Tool(
            name = 'awb_type',
            description = 'Type text into an element (appends, simulates human keystroke timing).',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector for the element' },
                    'keys': { 'type': 'string', 'description': 'Text / key sequence to type' }
                },
                'required': [ 'selector', 'keys' ]
            }
        ),
        types.Tool(
            name = 'awb_press_key',
            description = 'Press a keyboard key globally (e.g. Enter, Tab, Escape, ArrowDown).',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'key': { 'type': 'string', 'description': 'Key name (e.g. Enter, Tab, Escape, ArrowDown)' }
                },
                'required': [ 'key' ]
            }
        ),
        types.Tool(
            name = 'awb_find',
            description = 'Find an element by CSS selector and return its outer HTML value.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector' }
                },
                'required': [ 'selector' ]
            }
        ),
        types.Tool(
            name = 'awb_find_text',
            description = 'Find an element by visible text and return its text content.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'text': { 'type': 'string', 'description': 'Text to search for on the page' }
                },
                'required': [ 'text' ]
            }
        ),
        types.Tool(
            name = 'awb_element_text',
            description = 'Get the text content of an element found by CSS selector.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector' }
                },
                'required': [ 'selector' ]
            }
        ),
        types.Tool(
            name = 'awb_element_exists',
            description = 'Check whether an element matching a CSS selector exists on the page.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'selector': { 'type': 'string', 'description': 'CSS selector' }
                },
                'required': [ 'selector' ]
            }
        ),
        types.Tool(
            name = 'awb_wait',
            description = 'Wait for a fixed number of milliseconds.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'delay_ms': { 'type': 'integer', 'description': 'Milliseconds to wait' }
                },
                'required': [ 'delay_ms' ]
            }
        ),
        types.Tool(
            name = 'awb_wait_for_network_idle',
            description = 'Wait until network traffic is idle for a given window, or until a timeout.',
            inputSchema = {
                'type': 'object',
                'properties': {
                    'idle_ms': { 'type': 'integer', 'description': 'Idle window in ms (e.g. 500)' },
                    'timeout_ms': { 'type': 'integer', 'description': 'Maximum wait time in ms (e.g. 10000)' }
                },
                'required': [ 'idle_ms', 'timeout_ms' ]
            }
        ),
    ]


@app.call_tool()
async def call_tool( name, arguments ):
    try:
        if name == 'awb_health':
            response = await _get( '/health-check' )
            return _ok( 'browser is healthy' if response.status_code == 200 else 'browser unavailable' )

        elif name == 'awb_goto':
            await _post( '/page/goto', { 'url': arguments[ 'url' ] } )
            return _ok( f'navigated to {arguments["url"]}' )

        elif name == 'awb_get_title':
            response = await _get( '/page/title' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_get_url':
            response = await _get( '/page/url' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_get_content':
            response = await _get( '/page/content' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_screenshot':
            response = await _get( '/page/screenshot' )
            encoded = base64.b64encode( response.content ).decode( 'utf-8' )
            return [ types.ImageContent( type = 'image', data = encoded, mimeType = 'image/png' ) ]

        elif name == 'awb_click':
            await _post( '/input/click', { 'selector': arguments[ 'selector' ] } )
            return _ok( f'clicked {arguments["selector"]}' )

        elif name == 'awb_click_text':
            await _post( '/input/click/text', { 'selector': arguments[ 'text' ] } )
            return _ok( f'clicked element with text: {arguments["text"]}' )

        elif name == 'awb_hover':
            await _post( '/input/hover', { 'selector': arguments[ 'selector' ] } )
            return _ok( f'hovered over {arguments["selector"]}' )

        elif name == 'awb_fill':
            await _post( '/input/fill', { 'selector': arguments[ 'selector' ], 'keys': arguments[ 'text' ] } )
            return _ok( f'filled {arguments["selector"]}' )

        elif name == 'awb_type':
            await _post( '/input/keys', { 'selector': arguments[ 'selector' ], 'keys': arguments[ 'keys' ] } )
            return _ok( f'typed into {arguments["selector"]}' )

        elif name == 'awb_press_key':
            await _post( '/input/key', { 'key': arguments[ 'key' ] } )
            return _ok( f'pressed key: {arguments["key"]}' )

        elif name == 'awb_find':
            selector = arguments[ 'selector' ]
            response = await _get( f'/find/{selector}' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_find_text':
            text = arguments[ 'text' ]
            response = await _get( f'/find/text/{text}' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_element_text':
            selector = arguments[ 'selector' ]
            response = await _get( f'/find/element-text/{selector}' )
            data = response.json()
            return _ok( data.get( 'data', '' ) )

        elif name == 'awb_element_exists':
            selector = arguments[ 'selector' ]
            response = await _get( f'/find/exists/{selector}' )
            data = response.json()
            return _ok( data.get( 'data', 'false' ) )

        elif name == 'awb_wait':
            delay = arguments[ 'delay_ms' ]
            await _get( f'/wait/{delay}' )
            return _ok( f'waited {delay}ms' )

        elif name == 'awb_wait_for_network_idle':
            idle = arguments[ 'idle_ms' ]
            timeout = arguments[ 'timeout_ms' ]
            await _get( f'/wait/network/idle/{idle}/{timeout}' )
            return _ok( 'network idle' )

        else:
            return _err( f'unknown tool: {name}' )

    except httpx.ConnectError:
        return _err( 'cannot connect to AWB — is the server running on port 8000?' )
    except httpx.HTTPStatusError as exc:
        return _err( f'HTTP {exc.response.status_code}: {exc.response.text}' )
    except Exception as exc:
        return _err( str( exc ) )


async def main():
    async with stdio_server() as ( read_stream, write_stream ):
        await app.run( read_stream, write_stream, app.create_initialization_options() )


if __name__ == '__main__':
    asyncio.run( main() )
