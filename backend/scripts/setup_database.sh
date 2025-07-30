#!/bin/bash

# EvoWorld Database Setup Script
# This script sets up a PostgreSQL database for EvoWorld without interfering with the host system

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up EvoWorld database...${NC}"

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo -e "${RED}PostgreSQL is not installed. Please install PostgreSQL first.${NC}"
    echo "On Ubuntu/Debian: sudo apt-get install postgresql postgresql-contrib"
    echo "On Arch Linux: sudo pacman -S postgresql"
    echo "On macOS: brew install postgresql"
    exit 1
fi

# Check if PostgreSQL service is running
if ! pg_isready -q; then
    echo -e "${YELLOW}PostgreSQL service is not running. Starting it...${NC}"
    if command -v systemctl &> /dev/null; then
        sudo systemctl start postgresql
    elif command -v brew &> /dev/null; then
        brew services start postgresql
    else
        echo -e "${RED}Please start PostgreSQL service manually${NC}"
        exit 1
    fi
fi

# Create database user and database
echo -e "${GREEN}Creating database user and database...${NC}"

# Try to create user (ignore if already exists)
psql -U postgres -h localhost -c "CREATE USER evoworld WITH PASSWORD 'evoworld_password';" 2>/dev/null || true

# Create database (ignore if already exists)
psql -U postgres -h localhost -c "CREATE DATABASE evoworld OWNER evoworld;" 2>/dev/null || true

# Grant privileges
psql -U postgres -h localhost -c "GRANT ALL PRIVILEGES ON DATABASE evoworld TO evoworld;" 2>/dev/null || true

echo -e "${GREEN}Database setup complete!${NC}"
echo -e "${GREEN}Connection URL: postgresql://evoworld:evoworld_password@localhost:5432/evoworld${NC}"
echo -e "${YELLOW}You can also set DATABASE_URL environment variable to override this configuration.${NC}" 