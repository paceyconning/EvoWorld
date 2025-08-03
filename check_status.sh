#!/bin/bash

# EvoWorld Status Check Script
# This script checks if the backend is running and provides status information

echo "🔍 EvoWorld Status Check"
echo "========================"

# Check if backend process is running
BACKEND_PID=$(pgrep -f "evoworld-sim")
if [ -n "$BACKEND_PID" ]; then
    echo "✅ Backend is running (PID: $BACKEND_PID)"
else
    echo "❌ Backend is not running"
    echo "   Start with: ./start_simulation.sh"
fi

# Check if port 8080 is open
if netstat -tuln 2>/dev/null | grep -q ":8080 "; then
    echo "✅ WebSocket server is listening on port 8080"
else
    echo "❌ WebSocket server is not listening on port 8080"
fi

# Check if Godot is installed
if command -v godot &> /dev/null; then
    echo "✅ Godot is installed"
    GODOT_VERSION=$(godot --version 2>/dev/null | head -n1)
    echo "   Version: $GODOT_VERSION"
else
    echo "⚠️  Godot not found in PATH"
    echo "   Make sure Godot 4.4+ is installed"
fi

# Check if Rust is installed
if command -v cargo &> /dev/null; then
    echo "✅ Rust/Cargo is installed"
    RUST_VERSION=$(cargo --version)
    echo "   $RUST_VERSION"
else
    echo "❌ Rust/Cargo not found"
    echo "   Install from: https://rustup.rs/"
fi

# Check if PostgreSQL is running (optional)
if pg_isready -q 2>/dev/null; then
    echo "✅ PostgreSQL is running (database features enabled)"
else
    echo "⚠️  PostgreSQL not running (database features disabled)"
fi

echo ""
echo "🎮 To start the simulation:"
echo "   1. Run: ./start_simulation.sh"
echo "   2. Open Godot and load the frontend project"
echo "   3. Press F5 to run the frontend"
echo ""
echo "📊 The simulation will show:"
echo "   - Humanoids evolving and interacting"
echo "   - Resources being gathered and used"
echo "   - Tribes forming and developing"
echo "   - Technology discoveries and progress" 