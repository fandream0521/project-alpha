#!/bin/bash

# Setup environment for Ticket Management System

echo "=== Ticket Management System Setup ==="

# Check if .env exists
if [ -f ".env" ]; then
    echo "✓ .env file already exists"
    echo "Current DATABASE_URL:"
    grep DATABASE_URL .env || echo "  DATABASE_URL not found"
else
    echo "Creating .env file..."
    if [ -f ".env.example" ]; then
        cp .env.example .env
        echo "✓ .env file created from template"
    else
        cat > .env << EOF
# Database Configuration
DATABASE_URL=postgresql://username:password@localhost/ticket_db

# Server Configuration
HOST=0.0.0.0
PORT=3000
RUST_LOG=debug
EOF
        echo "✓ Basic .env file created"
    fi
fi

echo ""
echo "=== Next Steps ==="
echo "1. Edit .env file with your database credentials"
echo "2. Initialize database: ./scripts/init-db.sh"
echo "3. Insert sample data: ./scripts/seed-db.sh"
echo "4. Test everything: ./scripts/test.sh"
echo ""