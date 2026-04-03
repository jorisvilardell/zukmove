# Polymove

## Architecture

```mermaid
graph LR
    subgraph RabbitMQ["RabbitMQ :5672"]
        MQ["zukmove.events exchange"]
    end
    subgraph Polytech["Polytech :8080"]
        A["REST API"] --> B["InternshipService"]
        A --> C["StudentRepository"]
        A --> NR["NotificationRepository"]
        C --> D[("PostgreSQL")]
        NR --> D
        B --> C
        B --> E["OfferClient HTTP"]
        B --> F["InternshipRepository"]
        F --> D
        A --> G["GrpcNewsClient"]
    end
    subgraph Erasmumu["Erasmumu :8081"]
        H["REST API"] --> I["OfferRepository"]
        I --> J[("MongoDB")]
    end
    subgraph MI8["MI8 :50051 gRPC"]
        K["MI8Service"] --> L["NewsRepository"]
        K --> M["CityScoreRepository"]
        K --> CS["CityStats"]
        L --> N[("Redis")]
        M --> N
        CS --> N
    end
    subgraph LaPoste["La Poste :8083"]
        LP["REST API"] --> LS["SubscriberStore"]
    end
    subgraph Colporteur
        O["RabbitMQ Publisher"]
    end
    subgraph Frontend["Frontend :5173"]
        FE["React SPA"]
    end
    O -->|"news.created"| MQ
    MQ -->|"news.created"| K
    A -->|"student.registered"| MQ
    MQ -->|"student.registered"| LP
    H -->|"offer.created"| MQ
    MQ -->|"offer.created"| A
    MQ -->|"offer.created"| K
    MQ -->|"offer.created"| LP
    E -->|"HTTP GET"| H
    G -->|"gRPC"| K
    FE -->|"HTTP"| A
    FE -->|"HTTP"| LP
```

## Services

| Service | Port | Stack | Base de donnees | Role |
|---------|------|-------|-----------------|------|
| **Polytech** | 8080 | Actix-web REST | PostgreSQL | API Gateway, etudiants, stages, notifications |
| **Erasmumu** | 8081 | Actix-web REST | MongoDB | Gestion des offres de stage |
| **MI8** | 50051 | Tonic gRPC | Redis | Intelligence : news, city scores, city stats |
| **La Poste** | 8083 | Actix-web REST | In-memory | Notifications, preferences abonnement |
| **Colporteur** | — | Script CLI | — | Injection de news via RabbitMQ |
| **Frontend** | 5173 | React + Vite | — | Interface utilisateur |
| **RabbitMQ** | 5672 / 15672 | Message broker | — | Communication asynchrone inter-services |

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- [Node.js](https://nodejs.org/) >= 18 + pnpm
- [Docker](https://docs.docker.com/get-docker/) + Docker Compose
- [protoc](https://grpc.io/docs/protoc-installation/) (Protocol Buffers compiler)

## How to Run

### 1. Start infrastructure

```bash
docker compose up -d
# Starts: PostgreSQL :5432, MongoDB :27017, Redis :6379, RabbitMQ :5672
```

RabbitMQ management UI: http://localhost:15672 (guest/guest)

### 2. Start backend services

Each service in its own terminal:

```bash
# MI8 — Intelligence service (gRPC + RabbitMQ consumer)
cargo run --bin mi8

# Erasmumu — Offer management (REST + RabbitMQ publisher)
cargo run --bin erasmumu

# Polytech — API Gateway (REST + RabbitMQ pub/sub)
cargo run --bin polytech

# La Poste — Notification preferences (REST + RabbitMQ consumer)
cargo run --bin laposte
```

### 3. Seed data

```bash
# Inject sample news via RabbitMQ
cargo run --bin colporteur
```

### 4. Start frontend

```bash
cd apps/frontend
pnpm install
pnpm dev
# http://localhost:5173
```

## RabbitMQ Event Flows

Exchange: `zukmove.events` (topic, durable)

| Event | Publisher | Consumers | Description |
|-------|-----------|-----------|-------------|
| `news.created` | Colporteur | MI8 | News injection, city score update |
| `student.registered` | Polytech | La Poste | New student auto-subscribes |
| `offer.created` | Erasmumu | Polytech, MI8, La Poste | Notifications, city stats, alerts |

### Flow: News Creation
```
Colporteur --[news.created]--> RabbitMQ --> MI8 (save news + update city score)
```

### Flow: Student Registration
```
Client --POST /student--> Polytech --[student.registered]--> RabbitMQ --> La Poste (create subscriber)
```

### Flow: Offer Creation
```
Client --POST /offer--> Erasmumu --[offer.created]--> RabbitMQ --> Polytech (create notifications)
                                                               --> MI8 (update city stats)
                                                               --> La Poste (send mock alerts)
```

## API

### Polytech `:8080`

Swagger UI: http://localhost:8080/swagger-ui/

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/student` | Create student (publishes `student.registered`) |
| GET | `/student/{id}` | Get student by ID |
| GET | `/student?domain=IT` | List students by domain |
| PUT | `/student/{id}` | Update student |
| DELETE | `/student/{id}` | Delete student |
| POST | `/internship` | Apply for internship (domain matching) |
| GET | `/internship/{id}` | Get internship status |
| GET | `/offer` | Aggregated offers (Erasmumu + MI8) |
| GET | `/offer?domain=IT&city=Paris` | Filtered aggregated offers |
| GET | `/student/{id}/recommended-offers` | Personalized recommendations |
| GET | `/students/{id}/notifications` | Student notifications |
| PUT | `/notifications/{id}/read` | Mark notification as read |
| GET | `/news?limit=5&city=Paris` | News (via MI8 gRPC) |

### Erasmumu `:8081`

Swagger UI: http://localhost:8081/swagger-ui/

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/offer` | Create offer (publishes `offer.created`) |
| GET | `/offer/{id}` | Get offer by ID |
| GET | `/offer?domain=IT` | List by domain |
| GET | `/offer?city=Paris` | List by city |
| PUT | `/offer/{id}` | Update offer |
| DELETE | `/offer/{id}` | Delete offer |

### La Poste `:8083`

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/subscribers/{studentId}` | Get notification preferences |
| PUT | `/subscribers/{studentId}` | Update preferences (channel, contact, enabled) |
| DELETE | `/subscribers/{studentId}` | Unsubscribe |

### MI8 `:50051` (gRPC)

| RPC | Description |
|-----|-------------|
| `GetLatestNews(limit)` | Latest N news |
| `GetLatestNewsInCity(city, limit)` | News by city |
| `CreateNews(news)` | Create news + update city score |
| `GetCityScore(city)` | City score (4 metrics) |
| `GetTopCities(limit)` | City ranking |
| `GetCityStats(city)` | City offer statistics |

## Frontend Features

| Page | Route | Description |
|------|-------|-------------|
| **Explorer** | `/` | Browse offers with city scores, filter by domain/city |
| **Dashboard** | `/dashboard` | Login, recommendations, notification center |
| **Preferences** | `/preferences` | La Poste subscription settings (email/Discord) |

## City Scoring

Each city starts at **1000 pts** per metric. News tags modify scores:

| Tag | QoL | Safety | Economy | Culture |
|-----|:---:|:------:|:-------:|:-------:|
| innovation | +30 | +20 | +60 | +5 |
| crime | -40 | -80 | -20 | -10 |
| festival | +20 | 0 | +10 | +60 |
| economy | +10 | 0 | +50 | 0 |
| pollution | -50 | -10 | -5 | -5 |
| tourism | +20 | +5 | +30 | +40 |
| education | +30 | +10 | +20 | +30 |
| health | +40 | +20 | +10 | 0 |
| sports | +20 | +5 | +15 | +30 |
| politics | 0 | -10 | +10 | 0 |

> Scores never go below **0**.

## Environment Variables

| Variable | Default | Used by |
|----------|---------|---------|
| `DATABASE_URL` | `postgres://polytech:polytech@localhost:5432/polytech` | Polytech |
| `ERASMUMU_URL` | `http://localhost:8081` | Polytech |
| `MI8_URL` | `http://localhost:50051` | Polytech |
| `RABBITMQ_URL` | `amqp://guest:guest@localhost:5672` | All services |
| `MONGO_URL` | `mongodb://localhost:27017` | Erasmumu |
| `MONGO_DB` | `erasmumu` | Erasmumu |
| `REDIS_URL` | `redis://localhost:6379` | MI8 |
| `MI8_PORT` | `50051` | MI8 |
| `PORT` | `8080` / `8081` / `8083` | Polytech / Erasmumu / La Poste |

## Tests

```bash
cargo test --workspace
```

## Project Structure

```
zukmove/
├── proto/mi8.proto                    # gRPC service definition
├── docker-compose.yml                 # PostgreSQL, MongoDB, Redis, RabbitMQ
├── libs/zukmove-core/                 # Shared domain (entities, ports, services)
├── apps/
│   ├── polytech/                      # API Gateway (REST + RabbitMQ)
│   ├── erasmumu/                      # Offer management (REST + RabbitMQ)
│   ├── mi8/                           # Intelligence (gRPC + RabbitMQ)
│   ├── laposte/                       # Notification preferences (REST + RabbitMQ)
│   ├── colporteur/                    # News seeder (RabbitMQ publisher)
│   └── frontend/                      # React SPA (Vite + Zustand)
```
