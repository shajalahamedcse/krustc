project_root/
├── src/
│   ├── main.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   └── db/
│       ├── mod.rs
│       └── insertable.rs
├── Cargo.toml
├── .env
├── Dockerfile
├── docker-compose.yml
└── migrations/
    ├── 20231001000000_create_users_table.sql
    └── 20231001000001_create_products_table.sql