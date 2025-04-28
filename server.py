#!/usr/bin/env python3
"""
Simple API Server

This script sets up a local API server using Python's built-in http.server module.
It handles API requests and can be configured to use a custom port.
"""

import http.server
import socketserver
import argparse
import signal
import sys
import os
import json
import time
from urllib.parse import urlparse, parse_qs

import to_polytonic

def signal_handler(sig, frame):
  """Handle Ctrl+C gracefully"""
  print("\nShutting down server...")
  sys.exit(0)


class CustomHTTPRequestHandler(http.server.BaseHTTPRequestHandler):
  """Custom HTTP request handler for API endpoints only"""

  def log_request(self, code="-", size="-"):
    """Override to customize the request logging"""
    timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
    path = self.path
    if "?" in path:
      path, query = path.split("?", 1)
    else:
      query = ""

    client_address = self.client_address[0]
    method = self.command

    print(f"[{timestamp}] {client_address} - {method} {path} {query} - {code}")

  def _set_cors_headers(self):
    """Set headers for CORS support"""
    self.send_header("Access-Control-Allow-Origin", "*")
    self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
    self.send_header("Access-Control-Allow-Headers", "Content-Type")

  def do_OPTIONS(self):
    """Handle OPTIONS requests for CORS preflight"""
    self.send_response(200)
    self._set_cors_headers()
    self.end_headers()

  def do_GET(self):
    """Handle GET requests"""
    parsed_url = urlparse(self.path)
    path = parsed_url.path

    # API endpoint for NLP processing
    if path == "/nlp":
      self.send_response(200)
      self.send_header("Content-type", "application/json")
      self._set_cors_headers()
      self.end_headers()

      # Get query parameters if any
      query = parse_qs(parsed_url.query)
      if "text" not in query:
        response = {"error": "Missing 'text' parameter", "status": "error"}
      else:
        response = {'words': to_polytonic.to_polytonic(query['text'][0])}

      self.wfile.write(json.dumps(response, indent=2).encode())
      return

    # Return 404 for all other paths
    self.send_response(404)
    self.send_header("Content-type", "application/json")
    self._set_cors_headers()
    self.end_headers()
    response = {"error": "Not found", "status": "error", "path": path}
    self.wfile.write(json.dumps(response, indent=2).encode())


def start_server(port):
  """Start the HTTP server on the specified port"""
  try:
    # Set up the custom handler
    handler = CustomHTTPRequestHandler

    # Create the server
    with socketserver.TCPServer(("", port), handler) as httpd:
      # Print server information
      print(f"API Server started at http://localhost:{port}")
      print(f"Available API endpoints:")
      print(f"  - http://localhost:{port}/nlp?text=YourText - NLP processing")
      print("Server is running in the background")
      print("Press Ctrl+C to stop the server")

      # Register the signal handler for Ctrl+C
      signal.signal(signal.SIGINT, signal_handler)

      # Start serving requests
      httpd.serve_forever()

  except OSError as e:
    if e.errno == 98:  # Address already in use
      print(
          f"Error: Port {port} is already in use. Try a different port.")
    else:
      print(f"Error starting server: {e}")
    sys.exit(1)
  except Exception as e:
    print(f"Unexpected error: {e}")
    sys.exit(1)


def main():
  """Parse arguments and start the server"""
  parser = argparse.ArgumentParser(description="Start a simple HTTP server")
  parser.add_argument(
      "-p",
      "--port",
      type=int,
      default=8000,
      help="Port to run the server on (default: 8000)",
  )

  args = parser.parse_args()
  start_server(args.port)


if __name__ == "__main__":
  main()
