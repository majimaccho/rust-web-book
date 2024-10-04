INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT 
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$PK1n9PJoid7Rq8N2C3sowu7TGJqB0qtAIr1hf1MwY89Xt/FWNt4te',
    role_id
FROM 
    roles
WHERE 
    name LIKE 'Admin';