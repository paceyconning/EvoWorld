# EvoWorld Testing Guide

This guide provides comprehensive instructions for testing the EvoWorld simulation system.

## 🚀 Quick Start

### 1. Backend Testing

The backend is currently running and listening on port 8080. You can verify this with:

```bash
# Check if backend is running
ps aux | grep evoworld

# Check if port 8080 is listening
netstat -tlnp | grep 8080
```

### 2. Frontend Testing

#### Option A: Using Godot 4 (Recommended)

1. **Install Godot 4**:
   ```bash
   # On Arch Linux
   sudo pacman -S godot
   
   # Or download from https://godotengine.org/download
   ```

2. **Open the Frontend**:
   ```bash
   cd frontend
   godot --path . --main-pack Main.tscn
   ```

3. **Run the Project**:
   - Press F5 or click the "Play" button in Godot
   - The frontend should connect to the backend automatically
   - You should see connection status and world data

#### Option B: Manual WebSocket Testing

If you don't have Godot installed, you can test the WebSocket connection manually:

```bash
# Install Python websockets (if needed)
pip install websockets --user

# Run the test script
python3 test_websocket.py
```

## 🧪 Testing Scenarios

### Backend Tests

```bash
# Run all backend tests
cd backend
cargo test

# Run specific test categories
cargo test simulation::engine::tests
cargo test simulation::world::tests
cargo test database::tests
```

### Frontend Tests

1. **Connection Test**:
   - Start the frontend
   - Verify connection status shows "✅ Connected"
   - Check console for connection messages

2. **World State Test**:
   - Verify world data is received
   - Check statistics panel shows population, tribes, resources
   - Verify 3D entities are displayed

3. **Interaction Test**:
   - Test camera controls (mouse wheel, drag)
   - Test view mode buttons
   - Test simulation controls (pause/resume, speed)

4. **Real-time Updates**:
   - Watch for real-time events
   - Verify statistics update automatically
   - Check for new entities appearing

## 🔧 Troubleshooting

### Backend Issues

1. **Backend not starting**:
   ```bash
   cd backend
   cargo check  # Check for compilation errors
   cargo run -- --websocket  # Start manually
   ```

2. **Database issues**:
   ```bash
   # Check database connection
   psql -U evoworld -d evoworld -c "SELECT 1;"
   
   # Reset database if needed
   ./scripts/setup_database.sh
   ```

3. **Port conflicts**:
   ```bash
   # Check what's using port 8080
   lsof -i :8080
   
   # Kill conflicting process
   sudo kill -9 <PID>
   ```

### Frontend Issues

1. **Connection failed**:
   - Verify backend is running
   - Check firewall settings
   - Try different server URL in WebSocketClient.gd

2. **No entities displayed**:
   - Check console for error messages
   - Verify world data is being received
   - Check scene files are properly loaded

3. **Performance issues**:
   - Reduce number of visible entities
   - Lower camera height
   - Disable detailed labels

## 📊 Expected Behavior

### Backend
- ✅ Compiles without errors
- ✅ All 33 tests pass
- ✅ WebSocket server listens on port 8080
- ✅ Database connection works
- ✅ Simulation runs continuously

### Frontend
- ✅ Connects to backend automatically
- ✅ Displays world statistics
- ✅ Shows 3D entities (humanoids, resources, buildings)
- ✅ Responds to user input
- ✅ Updates in real-time

## 🎮 Controls

### Keyboard
- `Escape`: Toggle pause/resume
- `Enter`: Request fresh world state
- `WASD`: Move camera (Spectator mode)

### Mouse
- **Left click + drag**: Move camera (Overview mode)
- **Mouse wheel**: Zoom in/out
- **Left click on entities**: View details

### UI
- **Speed slider**: Control simulation speed
- **Play/Pause button**: Control simulation state
- **View mode buttons**: Switch camera views

## 📈 Performance Monitoring

### Backend Metrics
```bash
# Check memory usage
ps aux | grep evoworld

# Monitor database connections
psql -U evoworld -d evoworld -c "SELECT count(*) FROM pg_stat_activity;"
```

### Frontend Metrics
- Entity count displayed
- Frame rate
- Memory usage
- Network latency

## 🐛 Debugging

### Backend Debugging
```bash
# Run with debug logging
RUST_LOG=debug cargo run -- --websocket

# Check logs
tail -f backend.log
```

### Frontend Debugging
- Check Godot console for error messages
- Enable debug output in scripts
- Monitor WebSocket connection status

## 📝 Test Results

### Current Status
- ✅ Backend: Running and healthy
- ✅ Database: Connected and functional
- ✅ WebSocket: Listening on port 8080
- ✅ Tests: 33/33 passing
- 🔄 Frontend: Ready for testing with Godot

### Next Steps
1. Install Godot 4
2. Open frontend project
3. Run and test visualization
4. Verify real-time updates
5. Test all interaction features

## 🆘 Getting Help

If you encounter issues:

1. Check the console output for error messages
2. Verify all prerequisites are installed
3. Ensure backend is running before starting frontend
4. Check network connectivity and firewall settings
5. Review the troubleshooting section above

For additional help, refer to:
- `CONTEXT.md` - Project overview and current status
- `README.md` - General project information
- `docs/ROADMAP.md` - Development roadmap 