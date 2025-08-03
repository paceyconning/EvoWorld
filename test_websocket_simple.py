#!/usr/bin/env python3
"""
Simple WebSocket test for EvoWorld backend
"""

import socket
import time

def test_websocket_port():
    """Test if port 8080 is open"""
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(2)
        result = sock.connect_ex(('localhost', 8080))
        sock.close()
        
        if result == 0:
            print("✅ Port 8080 is open - WebSocket server is running")
            return True
        else:
            print("❌ Port 8080 is closed - WebSocket server not running")
            return False
    except Exception as e:
        print(f"❌ Error testing port: {e}")
        return False

def test_backend_process():
    """Test if backend process is running"""
    import subprocess
    try:
        result = subprocess.run(['pgrep', '-f', 'evoworld'], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            pids = result.stdout.strip().split('\n')
            print(f"✅ Backend process running with PIDs: {pids}")
            return True
        else:
            print("❌ Backend process not found")
            return False
    except Exception as e:
        print(f"❌ Error checking process: {e}")
        return False

def main():
    print("🧪 Testing EvoWorld Backend Connection")
    print("=" * 40)
    
    # Test backend process
    process_ok = test_backend_process()
    
    # Test WebSocket port
    port_ok = test_websocket_port()
    
    if process_ok and port_ok:
        print("\n🎉 Backend is ready for frontend connection!")
        print("💡 You can now open the frontend in Godot 4")
    else:
        print("\n❌ Backend needs to be started")
        print("💡 Run: cd backend && cargo run -- --websocket")

if __name__ == "__main__":
    main() 