#!/usr/bin/env python3
"""
Simple WebSocket test client for EvoWorld backend
This script tests the WebSocket connection to the EvoWorld simulation server.
"""

import asyncio
import websockets
import json
import sys
from datetime import datetime

class EvoWorldTestClient:
    def __init__(self, uri="ws://127.0.0.1:8080"):
        self.uri = uri
        self.connected = False
        
    async def connect(self):
        """Connect to the WebSocket server"""
        try:
            print(f"🔌 Connecting to {self.uri}...")
            self.websocket = await websockets.connect(self.uri)
            self.connected = True
            print("✅ Connected successfully!")
            return True
        except Exception as e:
            print(f"❌ Connection failed: {e}")
            return False
    
    async def send_message(self, message):
        """Send a message to the server"""
        if not self.connected:
            print("❌ Not connected to server")
            return None
        
        try:
            message_str = json.dumps(message)
            print(f"📤 Sending: {message_str}")
            await self.websocket.send(message_str)
        except Exception as e:
            print(f"❌ Failed to send message: {e}")
            return None
    
    async def receive_message(self):
        """Receive a message from the server"""
        if not self.connected:
            print("❌ Not connected to server")
            return None
        
        try:
            message = await self.websocket.recv()
            print(f"📥 Received: {message}")
            return json.loads(message)
        except Exception as e:
            print(f"❌ Failed to receive message: {e}")
            return None
    
    async def test_connection(self):
        """Test basic connection and communication"""
        if not await self.connect():
            return False
        
        # Test 1: Get world state
        print("\n🧪 Test 1: Requesting world state...")
        await self.send_message({"type": "GetWorldState"})
        
        try:
            response = await asyncio.wait_for(self.receive_message(), timeout=5.0)
            if response:
                print("✅ World state request successful")
                print(f"📊 Response type: {response.get('type', 'unknown')}")
            else:
                print("❌ No response received")
        except asyncio.TimeoutError:
            print("❌ Timeout waiting for response")
        
        # Test 2: Subscribe to events
        print("\n🧪 Test 2: Subscribing to events...")
        await self.send_message({"type": "SubscribeToEvents"})
        
        try:
            response = await asyncio.wait_for(self.receive_message(), timeout=5.0)
            if response:
                print("✅ Event subscription successful")
            else:
                print("❌ No response to subscription")
        except asyncio.TimeoutError:
            print("❌ Timeout waiting for subscription response")
        
        # Test 3: Get population stats
        print("\n🧪 Test 3: Requesting population stats...")
        await self.send_message({"type": "GetPopulationStats"})
        
        try:
            response = await asyncio.wait_for(self.receive_message(), timeout=5.0)
            if response:
                print("✅ Population stats request successful")
            else:
                print("❌ No response to population stats")
        except asyncio.TimeoutError:
            print("❌ Timeout waiting for population stats")
        
        # Test 4: Set simulation speed
        print("\n🧪 Test 4: Setting simulation speed...")
        await self.send_message({"type": "SetSimulationSpeed", "speed": 1.5})
        
        try:
            response = await asyncio.wait_for(self.receive_message(), timeout=5.0)
            if response:
                print("✅ Simulation speed set successfully")
            else:
                print("❌ No response to speed change")
        except asyncio.TimeoutError:
            print("❌ Timeout waiting for speed change response")
        
        return True
    
    async def close(self):
        """Close the connection"""
        if self.connected:
            await self.websocket.close()
            self.connected = False
            print("🔌 Connection closed")

async def main():
    """Main test function"""
    print("🧪 EvoWorld WebSocket Test Client")
    print("=" * 40)
    
    client = EvoWorldTestClient()
    
    try:
        success = await client.test_connection()
        if success:
            print("\n✅ All tests completed successfully!")
        else:
            print("\n❌ Tests failed!")
            sys.exit(1)
    except KeyboardInterrupt:
        print("\n⏹️ Test interrupted by user")
    except Exception as e:
        print(f"\n❌ Test failed with error: {e}")
        sys.exit(1)
    finally:
        await client.close()

if __name__ == "__main__":
    asyncio.run(main()) 