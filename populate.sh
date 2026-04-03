#!/bin/bash

echo "Populating Students in Polytech (port 8080)"

curl -X POST http://localhost:8080/student \
    -H "Content-Type: application/json" \
    -d '{"firstname": "Joris", "name": "Doe", "domain": "IT"}'

curl -X POST http://localhost:8080/student \
    -H "Content-Type: application/json" \
    -d '{"firstname": "Alice", "name": "Smith", "domain": "Business"}'

echo -e "\n\nPopulating Offers in Erasmumu (port 8081)"

# Paris
curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Backend Engineer (Rust)", "link": "https://example.com/rust", "city": "Paris", "domain": "IT", "salary": 1500.0, "start_date": "2026-09-01", "end_date": "2027-02-28", "available": true}'

curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Data Scientist", "link": "https://example.com/data", "city": "Paris", "domain": "IT", "salary": 1800.0, "start_date": "2026-09-01", "end_date": "2027-02-28", "available": true}'

curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Marketing Assistant", "link": "https://example.com/marketing", "city": "Paris", "domain": "Business", "salary": 1100.0, "start_date": "2026-09-01", "end_date": "2027-02-28", "available": true}'

# Berlin
curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Frontend Developer", "link": "https://example.com/frontend", "city": "Berlin", "domain": "IT", "salary": 1600.0, "start_date": "2026-10-01", "end_date": "2027-03-31", "available": true}'

curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Mechanical Engineer Intern", "link": "https://example.com/mech", "city": "Berlin", "domain": "Engineering", "salary": 1400.0, "start_date": "2026-10-01", "end_date": "2027-03-31", "available": true}'

# Barcelona
curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Fullstack Developer", "link": "https://example.com/fullstack", "city": "Barcelona", "domain": "IT", "salary": 1300.0, "start_date": "2026-09-15", "end_date": "2027-03-15", "available": true}'

curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Business Analyst", "link": "https://example.com/ba", "city": "Barcelona", "domain": "Business", "salary": 1200.0, "start_date": "2026-09-15", "end_date": "2027-03-15", "available": true}'

# London
curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "AI Research Intern", "link": "https://example.com/ai", "city": "London", "domain": "IT", "salary": 2000.0, "start_date": "2026-08-01", "end_date": "2027-01-31", "available": true}'

curl -X POST http://localhost:8081/offer \
    -H "Content-Type: application/json" \
    -d '{"title": "Biotech Researcher", "link": "https://example.com/bio", "city": "London", "domain": "Life Science", "salary": 1900.0, "start_date": "2026-08-01", "end_date": "2027-01-31", "available": true}'

echo -e "\n\nPopulating News in MI8 via Colporteur..."
cargo run -p colporteur
