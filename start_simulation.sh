#!/bin/bash

# EvoWorld Simulation Startup Script
# This script starts the backend simulation and provides instructions for the frontend

echo "🚀 Starting EvoWorld Simulation..."
echo "=================================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo not found. Please install Rust first."
    echo "   Visit: https://rustup.rs/"
    exit 1
fi

# Check if PostgreSQL is running (optional check)
if ! pg_isready -q; then
    echo "⚠️  Warning: PostgreSQL might not be running."
    echo "   The simulation will work without a database, but analytics won't be saved."
    echo "   To enable database features, start PostgreSQL first."
fi

# Navigate to backend directory
cd backend

echo "🔧 Building backend..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi

echo "✅ Backend built successfully!"

echo ""
echo "🎮 Starting simulation server..."
echo "   Backend will be available at: ws://127.0.0.1:8080"
echo ""

# Start the backend simulation
echo "📊 Simulation is starting..."
echo "   - Press Ctrl+C to stop the simulation"
echo "   - The simulation will run continuously"
echo "   - Humanoids will evolve and interact autonomously"
echo ""

# Run the backend
cargo run --release -- --websocket 