#!/bin/bash

# Initialize database with required tables

echo "=== Database Initialization ==="

# Load environment variables
if [ -f ".env" ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "✗ .env file not found"
    echo "Run ./scripts/setup.sh first"
    exit 1
fi

if [ -z "$DATABASE_URL" ]; then
    echo "✗ DATABASE_URL not configured"
    exit 1
fi

echo "Using database: ${DATABASE_URL:0:150}..."

# Test database connection
echo "Testing connection..."
if ! psql -h localhost -p 5432 -U postgres -d ticket_db -c "SELECT 1;" &>/dev/null; then
    echo ""
    echo "✗ Cannot connect to database"
    echo ""
    echo "Troubleshooting:"
    echo "1. Check if PostgreSQL is running: pg_isready"
    echo "2. Verify DATABASE_URL in .env file"
    echo "3. Create database manually:"
    echo "   psql -U postgres -c \"CREATE DATABASE ticket_db;\""
    echo ""
    echo "Or try using postgres user:"
    echo "   DATABASE_URL=postgresql://postgres:your_password@localhost:5432/ticket_db"
    exit 1
fi

echo "✓ Database connection successful"

# Create tables
echo ""
echo "Creating tables..."
psql -h localhost -p 5432 -U postgres -d ticket_db -f "scripts/create-tables.sql" 2>/dev/null
if [ $? -eq 0 ]; then
    echo "✓ Tables created successfully"
else
    echo "⚠ Tables creation had warnings, but continuing..."
fi

echo ""
echo "=== Initialization Complete ==="
echo "Next: ./scripts/seed-db.sh"
echo ""