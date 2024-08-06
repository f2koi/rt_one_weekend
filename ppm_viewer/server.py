#!/usr/bin/env python3
# https://gist.github.com/acdha/925e9ffc3d74ad59c3ea

from http.server import HTTPServer, SimpleHTTPRequestHandler


class CORSRequestHandler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET")
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate")
        return super(CORSRequestHandler, self).end_headers()


host, port = "0.0.0.0", 8000

httpd = HTTPServer((host, port), CORSRequestHandler)
print(f"Serving at http://{host}:{port}")
httpd.serve_forever()
