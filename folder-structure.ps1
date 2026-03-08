# ================================
#   MODULAR MONOLITH RUST BACKEND
#   Axum + MongoDB + DragonflyDB
#   TDD + Prometheus + Grafana
#   GitLab CI/CD Ready
# ================================

# -------- Base Directories ------
$baseDirs = @(
    "src",
    "src/config",
    "src/core",
    "src/core/middleware",
    "src/core/guards",
    "src/core/errors",
    "src/core/utils",

    "src/metrics",  # <-- for Prometheus metrics

    "src/shared",
    "src/shared/validation",
    "src/shared/pagination",
    "src/shared/sorting",

    "src/domain",
    "src/domain/users",
    "src/domain/orders",
    "src/domain/products",
    "src/domain/cart",

    "src/modules",
    "src/modules/users",
    "src/modules/auth",
    "src/modules/products",
    "src/modules/orders",
    "src/modules/cart",

    "src/infrastructure",
    "src/infrastructure/mongo",
    "src/infrastructure/dragonfly",
    "src/infrastructure/email",
    "src/infrastructure/s3",

    # Tests (TDD)
    "tests",
    "tests/users",
    "tests/products",
    "tests/orders",
    "tests/cart",

    # CI/CD
    ".gitlab"
)

foreach ($dir in $baseDirs) {
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
}

# -------- Helper for file creation -------
function New-File($path, $content = "") {
    New-Item -ItemType File -Path $path -Force | Out-Null
    if ($content -ne "") {
        $content | Out-File -FilePath $path -Encoding utf8
    }
}

# --------- Root Rust files ----------
$mainFiles = @(
    "src/main.rs",
    "src/lib.rs",
    "src/app.rs",
    "src/bootstrap.rs"
)
foreach ($file in $mainFiles) { New-File $file }

# --------- Metrics file ----------
New-File "src/metrics/mod.rs" "// Prometheus metrics registry placeholder"

# -------- Module boilerplate ----------
$modules = @("users","auth","products","orders","cart")

foreach ($mod in $modules) {
    $path = "src/modules/$mod"
    New-File "$path/mod.rs"
    New-File "$path/controller.rs"
    New-File "$path/service.rs"
    New-File "$path/repository.rs"
    New-File "$path/dto.rs"
    New-File "$path/mapper.rs"
    New-File "$path/routes.rs"
}

# -------- Infrastructure ----------
$infraFiles = @(
    "src/infrastructure/mongo/mod.rs",
    "src/infrastructure/mongo/user_repo.rs",
    "src/infrastructure/mongo/product_repo.rs",
    "src/infrastructure/mongo/order_repo.rs",

    "src/infrastructure/dragonfly/mod.rs",
    "src/infrastructure/dragonfly/cache.rs",
    "src/infrastructure/dragonfly/pubsub.rs",

    "src/infrastructure/email/mod.rs",
    "src/infrastructure/s3/mod.rs"
)

foreach ($file in $infraFiles) { New-File $file }

# ---------- Shared Tools ----------
$sharedFiles = @(
    "src/shared/validation/mod.rs",
    "src/shared/pagination/mod.rs",
    "src/shared/sorting/mod.rs"
)

foreach ($file in $sharedFiles) { New-File $file }

# ---------- Test Files (TDD) ----------
$testFiles = @(
    "tests/users/user_service_test.rs",
    "tests/products/product_service_test.rs",
    "tests/orders/order_service_test.rs",
    "tests/cart/cart_service_test.rs"
)

foreach ($file in $testFiles) { New-File $file "// TDD test placeholder" }

# ---------- README placeholders ----------
$readmeDirs = @(
    "src","src/infrastructure","src/modules","src/domain","src/shared"
)

foreach ($dir in $readmeDirs) {
    New-File "$dir/README.md" "# $dir"
}

# ---------- Cargo.toml ----------
$CargoToml = @"
[package]
name = "rust_ecom"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
mongodb = { version = "2.7", features = ["sync", "tokio-runtime"] }
redis = "0.26" # DragonflyDB compatible
uuid = { version = "1", features = ["v4"] }
dotenvy = "0.15"
jsonwebtoken = "9"
bcrypt = "0.15"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors","trace"] }
chrono = "0.4"
thiserror = "1"
async-graphql = "5"
async-graphql-axum = "5"

# Prometheus metrics
prometheus = "0.13"

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
serde_json = "1"
"@
New-File "Cargo.toml" $CargoToml

# -------- Dockerfile ----------
$Dockerfile = @"
FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust_ecom /usr/local/bin/rust_ecom
EXPOSE 8080
EXPOSE 9000  # Prometheus metrics
CMD ["rust_ecom"]
"@
New-File "Dockerfile" $Dockerfile

# -------- docker-compose.yml ----------
$DockerCompose = @"
version: '3.9'

services:
  app:
    build: .
    ports:
      - '8080:8080'
      - '9000:9000'      # Prometheus metrics endpoint
    environment:
      - MONGO_URI=mongodb://mongo:27017
      - DRAGONFLY_URI=redis://dragonfly:6379
    depends_on:
      - mongo
      - dragonfly
      - prometheus

  mongo:
    image: mongo:6
    ports:
      - '27017:27017'
    volumes:
      - mongo_data:/data/db

  dragonfly:
    image: dragonflydb/dragonfly:latest
    ports:
      - '6379:6379'
    command: ["--logtostderr"]

  prometheus:
    image: prom/prometheus
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - '9090:9090'

  grafana:
    image: grafana/grafana
    ports:
      - "3000:3000"
    depends_on:
      - prometheus

volumes:
  mongo_data:
"@
New-File "docker-compose.yml" $DockerCompose

# -------- Prometheus config ----------
$PromConfig = @"
global:
  scrape_interval: 5s

scrape_configs:
  - job_name: 'rust_app'
    static_configs:
      - targets: ['app:9000']
"@
New-File "prometheus.yml" $PromConfig

Write-Host "`n🚀 Full Scaffold with Prometheus + Grafana Created Successfully!"
Write-Host "📦 Dragonfly, MongoDB, Metrics, TDD, CI-ready Monolith Ready."
