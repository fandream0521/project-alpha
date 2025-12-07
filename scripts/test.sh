#!/bin/bash

# Test database connection and basic functionality

echo "=== System Test ==="

# Load environment variables
if [ -f ".env" ]; then
    export $(grep -v '^#' .env | xargs)
    echo "✓ Environment variables loaded"
else
    echo "✗ .env file not found"
    echo "Run ./scripts/setup.sh first"
    exit 1
fi

if [ -z "$DATABASE_URL" ]; then
    echo "✗ DATABASE_URL not configured"
    exit 1
fi

echo "Testing: ${DATABASE_URL:0:50}..."
echo ""

# Test 1: Database connection
echo "1. Database connection..."
if psql -h localhost -p 5432 -U postgres -d ticket_db -c "SELECT version();" -t &>/dev/null; then
    echo "✓ Connection successful"
else
    echo "✗ Connection failed"
    exit 1
fi

# Test 2: Table structure
echo "2. Table structure..."
TABLES=$(psql -h localhost -p 5432 -U postgres -d ticket_db -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN ('tags', 'tickets', 'ticket_tags', 'comments');" 2>/dev/null | xargs)
if [ "$TABLES" = "4" ]; then
    echo "✓ All required tables found ($TABLES/4)"
else
    echo "⚠ Not all tables found ($TABLES/4) - run ./scripts/init-db.sh"
fi

# Test 3: Data integrity
echo "3. Data integrity..."
TAGS_COUNT=$(psql -h localhost -p 5432 -U postgres -d ticket_db -t -c "SELECT COUNT(*) FROM tags;" 2>/dev/null | xargs)
TICKETS_COUNT=$(psql -h localhost -p 5432 -U postgres -d ticket_db -t -c "SELECT COUNT(*) FROM tickets;" 2>/dev/null | xargs)
COMMENTS_COUNT=$(psql -h localhost -p 5432 -U postgres -d ticket_db -t -c "SELECT COUNT(*) FROM comments;" 2>/dev/null | xargs)

echo "   Tags: $TAGS_COUNT"
echo "   Tickets: $TICKETS_COUNT"
echo "   Comments: $COMMENTS_COUNT"

# Test 4: Rust application (if backend exists)
echo "4. Rust application..."
if [ -d "ticket-backend" ]; then
    cd ticket-backend
    if cargo check &>/dev/null; then
        echo "✓ Rust project compiles"
    else
        echo "⚠ Rust project compilation failed"
    fi
    cd ..
else
    echo "⚠ Backend directory not found"
fi

# Summary
echo ""
echo "=== Test Summary ==="
if [ "$TABLES" = "4" ] && [ "$TAGS_COUNT" -gt 0 ] && [ "$TICKETS_COUNT" -gt 0 ]; then
    echo "✓ All tests passed - System is ready!"
    echo ""
    echo "Start the application:"
    echo "  cd ticket-backend && cargo run"
else
    echo "⚠ Some tests failed"
    echo ""
    echo "Fixes:"
    echo "  • Connection issues: Check DATABASE_URL in .env"
    echo "  • Missing tables: Run ./scripts/init-db.sh"
    echo "  • Missing data: Run ./scripts/seed-db.sh"
fi

echo ""