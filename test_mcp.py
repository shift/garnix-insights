#!/usr/bin/env python3
"""
Simple test script for the Garnix Fetcher MCP server
"""

import subprocess
import json
import sys

def test_mcp_server():
    """Test basic MCP server functionality"""
    
    # Start the MCP server process
    process = subprocess.Popen(
        ['nix', 'develop', '--command', './target/debug/garnix-fetcher', '--mcp'],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        cwd='/home/shift/code/garnix-report/garnix-fetcher'
    )
    
    # Test 1: Initialize the server
    initialize_msg = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2024-11-05",
            "capabilities": {"tools": {}},
            "clientInfo": {"name": "test-client", "version": "1.0.0"}
        }
    }
    
    print("🔄 Testing MCP server initialization...")
    process.stdin.write(json.dumps(initialize_msg) + '\n')
    process.stdin.flush()
    
    # Read the response
    response_line = process.stdout.readline()
    if response_line:
        try:
            response = json.loads(response_line)
            print("✅ Server initialized successfully")
            print(f"   Server: {response['result']['serverInfo']['name']} v{response['result']['serverInfo']['version']}")
            print(f"   Protocol: {response['result']['protocolVersion']}")
            print(f"   Instructions: {response['result']['instructions']}")
        except json.JSONDecodeError as e:
            print(f"❌ Failed to parse initialization response: {e}")
            print(f"   Raw response: {response_line}")
    else:
        print("❌ No response received from server")
    
    # Test 2: List available tools
    list_tools_msg = {
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    }
    
    print("\n🔄 Testing tool listing...")
    process.stdin.write(json.dumps(list_tools_msg) + '\n')
    process.stdin.flush()
    
    response_line = process.stdout.readline()
    if response_line:
        try:
            response = json.loads(response_line)
            if 'result' in response and 'tools' in response['result']:
                tools = response['result']['tools']
                print(f"✅ Found {len(tools)} available tools:")
                for tool in tools:
                    print(f"   - {tool['name']}: {tool['description']}")
            else:
                print("❌ Unexpected response format for tools/list")
                print(f"   Response: {response}")
        except json.JSONDecodeError as e:
            print(f"❌ Failed to parse tools/list response: {e}")
            print(f"   Raw response: {response_line}")
    else:
        print("❌ No response received for tools/list")
    
    # Clean up
    process.terminate()
    process.wait()
    
    print("\n🎉 MCP server test completed!")

if __name__ == "__main__":
    test_mcp_server()
