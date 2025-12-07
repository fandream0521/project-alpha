#!/bin/bash

# Insert sample data into database

echo "=== Seeding Database ==="

# Load environment variables
if [ -f ".env" ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo "✗ .env file not found"
    echo "Run ./scripts/setup.sh first"
    exit 1
fi

echo "Inserting sample data..."

# Clear existing data
psql -h localhost -p 5432 -U postgres -d ticket_db -c "
    DELETE FROM ticket_tags;
    DELETE FROM comments;
    DELETE FROM tickets;
    DELETE FROM tags;
" &>/dev/null

# Insert data using psql
psql -h localhost -p 5432 -U postgres -d ticket_db << 'EOF'
-- Insert tags
INSERT INTO tags (name, color) VALUES
    ('bug', '#EF4444'),
    ('feature', '#10B981'),
    ('improvement', '#3B82F6'),
    ('documentation', '#F59E0B'),
    ('urgent', '#DC2626');

-- Insert tickets
INSERT INTO tickets (title, description, status, priority) VALUES
    ('Login page display issue', 'Users report login page displays abnormally in Chrome browser', 'open', 'high'),
    ('Add user permission management', 'Need to set different permissions for different user roles', 'in_progress', 'medium'),
    ('API documentation needs update', 'New version API documentation needs to be updated', 'open', 'low'),
    ('System performance optimization', 'System response time is too slow under high concurrency', 'resolved', 'urgent');

-- Insert comments
INSERT INTO comments (ticket_id, content) VALUES
    ((SELECT id FROM tickets WHERE title = 'Login page display issue'), 'I have investigated the issue and found it is caused by CSS style conflicts.'),
    ((SELECT id FROM tickets WHERE title = 'System performance optimization'), 'By adding database indexes, response time was reduced from 5 seconds to 500 milliseconds.');

-- Show results
SELECT 'Sample data inserted successfully!' as status,
       (SELECT COUNT(*) FROM tags) as tags_count,
       (SELECT COUNT(*) FROM tickets) as tickets_count,
       (SELECT COUNT(*) FROM comments) as comments_count;
EOF

if [ $? -eq 0 ]; then
    echo "✓ Sample data inserted successfully"
else
    echo "✗ Failed to insert sample data"
    exit 1
fi

echo ""
echo "=== Seeding Complete ==="
echo "Next: ./scripts/test.sh"
echo ""