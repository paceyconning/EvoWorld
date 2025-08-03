#!/usr/bin/env python3
"""
Test script to verify EvoWorld backend WebSocket connection and data flow.
This helps ensure the backend is properly sending data that the frontend can visualize.
"""

import asyncio
import websockets
import json
import time

async def test_websocket_connection():
    """Test WebSocket connection to EvoWorld backend"""
    uri = "ws://localhost:8080"
    
    try:
        print("🔌 Connecting to EvoWorld backend at:", uri)
        async with websockets.connect(uri) as websocket:
            print("✅ Connected successfully!")
            
            # Test basic message sending
            test_message = {
                "type": "GetWorldState"
            }
            
            print("📤 Sending test message:", test_message)
            await websocket.send(json.dumps(test_message))
            
            # Wait for response
            print("⏳ Waiting for response...")
            response = await asyncio.wait_for(websocket.recv(), timeout=5.0)
            
            print("📥 Received response:")
            try:
                data = json.loads(response)
                print("✅ Valid JSON response received")
                print("📊 Response type:", data.get("type", "unknown"))
                
                if data.get("type") == "WorldState":
                    world_data = data.get("data", {})
                    print("🌍 World data keys:", list(world_data.keys()))
                    
                    # Print entity counts
                    humanoids = world_data.get("humanoids", [])
                    resources = world_data.get("resources", [])
                    buildings = world_data.get("buildings", [])
                    tribes = world_data.get("tribes", [])
                    
                    print(f"👥 Humanoids: {len(humanoids)}")
                    print(f"🌿 Resources: {len(resources)}")
                    print(f"🏗️ Buildings: {len(buildings)}")
                    print(f"🏘️ Tribes: {len(tribes)}")
                    
                    # Show sample humanoid data
                    if humanoids:
                        sample_humanoid = humanoids[0]
                        print("👤 Sample humanoid:", sample_humanoid)
                    
                    # Show sample resource data
                    if resources:
                        sample_resource = resources[0]
                        print("🌿 Sample resource:", sample_resource)
                    
                    # Show sample building data
                    if buildings:
                        sample_building = buildings[0]
                        print("🏗️ Sample building:", sample_building)
                    
                    print("✅ Backend is working correctly and sending data!")
                    return True
                    
            except json.JSONDecodeError as e:
                print("❌ Invalid JSON response:", e)
                print("Raw response:", response)
                return False
                
    except websockets.exceptions.ConnectionRefused:
        print("❌ Connection refused - backend not running on port 8080")
        print("💡 Make sure to run: cd backend && cargo run -- --websocket")
        return False
    except asyncio.TimeoutError:
        print("❌ Timeout waiting for response")
        return False
    except Exception as e:
        print("❌ Error:", e)
        return False

async def test_event_subscription():
    """Test event subscription"""
    uri = "ws://localhost:8080"
    
    try:
        print("\n📡 Testing event subscription...")
        async with websockets.connect(uri) as websocket:
            # Subscribe to events
            subscribe_message = {
                "type": "SubscribeToEvents"
            }
            
            print("📤 Subscribing to events...")
            await websocket.send(json.dumps(subscribe_message))
            
            # Wait for a few events
            print("⏳ Waiting for events (10 seconds)...")
            start_time = time.time()
            
            while time.time() - start_time < 10:
                try:
                    response = await asyncio.wait_for(websocket.recv(), timeout=1.0)
                    data = json.loads(response)
                    
                    if data.get("type") == "Event":
                        event = data.get("event", {})
                        print(f"📢 Event received: {event.get('type', 'unknown')} - {event.get('description', 'No description')}")
                    elif data.get("type") == "WorldState":
                        print("🔄 World state update received")
                        
                except asyncio.TimeoutError:
                    continue
                except Exception as e:
                    print(f"❌ Error receiving event: {e}")
                    break
            
            print("✅ Event subscription test completed")
            
    except Exception as e:
        print(f"❌ Event subscription test failed: {e}")

async def main():
    """Main test function"""
    print("🧪 Testing EvoWorld Backend Connection")
    print("=" * 50)
    
    # Test basic connection
    success = await test_websocket_connection()
    
    if success:
        # Test event subscription
        await test_event_subscription()
        
        print("\n🎉 All tests completed!")
        print("✅ Backend is ready for frontend connection")
        print("💡 You can now open the frontend in Godot 4")
    else:
        print("\n❌ Backend connection failed")
        print("💡 Please ensure the backend is running with:")
        print("   cd backend && cargo run -- --websocket")

if __name__ == "__main__":
    asyncio.run(main()) 