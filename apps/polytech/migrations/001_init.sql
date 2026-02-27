-- Create students table
CREATE TABLE IF NOT EXISTS students (
    id UUID PRIMARY KEY,
    firstname VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    domain VARCHAR(100) NOT NULL
);

-- Create internships table
CREATE TABLE IF NOT EXISTS internships (
    id UUID PRIMARY KEY,
    student_id UUID NOT NULL REFERENCES students(id),
    offer_id UUID NOT NULL,
    status VARCHAR(20) NOT NULL,
    message TEXT NOT NULL
);
