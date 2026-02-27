# Polymove — Walkthrough

## Architecture

```mermaid
graph LR
    subgraph "Service Polytech :8080"
        A[REST API] --> B[InternshipService]
        A --> C[StudentRepository]
        C --> D[(PostgreSQL)]
        B --> C
        B --> E[OfferClient]
        B --> F[InternshipRepository]
        F --> D
    end
    subgraph "Service Erasmumu :8081"
        G[REST API] --> H[OfferRepository]
        H --> I[(MongoDB)]
    end
    E -->|HTTP GET /offer/:id| G
```

## How to Run

```bash
# 1. Start databases
docker compose up -d

# 2. Start Erasmumu (port 8081)
cargo run --bin erasmumu

# 3. Start Polytech (port 8080) in another terminal
cargo run --bin polytech
```

## API Quick Reference

### Polytech (`:8080`)
- `POST /student` — create student
- `GET /student/{id}` — get by ID
- `GET /student?domain=IT` — filter by domain
- `PUT /student/{id}` — update
- `DELETE /student/{id}` — delete
- `POST /internship` — register for an offer (domain matching)
- `GET /internship/{id}` — check status

### Erasmumu (`:8081`)
- `POST /offer` — create offer
- `GET /offer/{id}` — get (only if available)
- `GET /offer?domain=IT` — filter by domain
- `GET /offer?city=Paris` — filter by city
- `PUT /offer/{id}` — update
- `DELETE /offer/{id}` — delete
