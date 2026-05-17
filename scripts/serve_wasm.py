#!/usr/bin/env python3
"""Local WASM-friendly HTTP server for itch.io emulation testing.

Python's stdlib http.server doesn't serve .wasm with the correct MIME type
by default. This override fixes WASM module loading and lets local_itch_preview.html
load the iframe from the same origin (no file:// cross-origin violation).

Usage from project root:
    python scripts/serve_wasm.py [port]
    # Then open http://127.0.0.1:8080/scripts/local_itch_preview.html
"""
import http.server
import os
import socketserver
import sys

PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8080

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    pass

WasmHandler.extensions_map['.wasm'] = 'application/wasm'
WasmHandler.extensions_map['.js']   = 'application/javascript'
WasmHandler.extensions_map['.mjs']  = 'application/javascript'

with socketserver.TCPServer(("127.0.0.1", PORT), WasmHandler) as httpd:
    print(f"Serving {os.getcwd()} on http://127.0.0.1:{PORT}")
    httpd.serve_forever()
