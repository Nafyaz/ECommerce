Command Results should contain all the necessary data to be returned to the client in plain format. No need to use
domain entity / value objects.

Modules:

- Identity
- Users
- Vendors
- Catalog
- Orders
- Payments
- Analytics
- Notifications
- Shipping
- Reviews
- Promotions
- shared
- cart

Roles Table is for platform-wide permissions.

# Multi-Vendor E-Commerce Backend - Rust/Axum

## Architecture Overview

This project follows a **Modular Monolith** architecture with **Hexagonal (Ports & Adapters)** principles inside each
module. Each module contains its own RESTful API layer, making the system potentially splittable into microservices in
the future.

### Key Architectural Principles

1. **Modular Monolith**: Logical separation into independent modules (vendors, products, orders, etc.)
2. **Hexagonal Architecture**: Each module has:
    - **Domain Layer**: Core business logic, entities, value objects
    - **Ports**: Interfaces (traits) defining contracts
    - **Adapters**: Implementations (API handlers, repositories, external services)
3. **RESTful API**: Standard REST conventions with proper HTTP methods and status codes
4. **CQRS-lite**: Separation between commands (writes) and queries (reads) within modules

## Directory Structure

```
ecommerce-backend/
├── Cargo.toml
├── rust-toolchain.toml
├── .env.example
├── .env.secrets.example
├── docker-compose.yml
├── Dockerfile
├── Makefile
├── README.md
│
├── src/
│   ├── main.rs                      # Application entry point
│   ├── lib.rs                       # Library root - re-exports public API
│   │
│   ├── bin/                         # Binary crates (migrations, CLI tools)
│   │   ├── main.rs                  # Main application binary
│   │   ├── migrate.rs               # Database migration runner
│   │   └── seed.rs                 # Database seeder
│   │
│   ├── api/                        # REST API Layer (Primary Adapters)
│   │   ├── mod.rs                  # API module definition
│   │   │
│   │   ├── router.rs               # Main Axum router configuration
│   │   ├── state.rs                # Application state management
│   │   │
│   │   ├── middleware/             # HTTP Middleware stack
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs             # JWT authentication middleware
│   │   │   ├── tenant.rs           # Multi-tenant context middleware
│   │   │   ├── logging.rs          # Request/response logging
│   │   │   │   ├── mod.rs
│   │   │   │   ├── request_log.rs
│   │   │   │   └── response_log.rs
│   │   │   ├── tracing.rs          # Distributed tracing middleware
│   │   │   ├── rate_limit.rs       # Rate limiting middleware
│   │   │   ├── cors.rs             # CORS configuration
│   │   │   ├── compression.rs      # Response compression
│   │   │   └── validation.rs       # Request validation middleware
│   │   │
│   │   ├── extractors/             # Custom Axum extractors
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs             # Authenticated user extractor
│   │   │   ├── tenant.rs           # Tenant context extractor
│   │   │   ├── pagination.rs       # Pagination parameters extractor
│   │   │   └── sorting.rs          # Sorting parameters extractor
│   │   │
│   │   ├── handlers/               # HTTP Request handlers (Controllers)
│   │   │   ├── mod.rs
│   │   │   ├── health.rs           # Health check endpoints
│   │   │   ├── readiness.rs        # Readiness probe endpoints
│   │   │   └── metrics.rs          # Prometheus metrics endpoint
│   │   │
│   │   ├── dto/                    # Data Transfer Objects
│   │   │   ├── mod.rs
│   │   │   ├── request/            # Incoming request DTOs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── pagination.rs   # Pagination request params
│   │   │   │   ├── sorting.rs      # Sorting request params
│   │   │   │   └── filtering.rs   # Filtering request params
│   │   │   │
│   │   │   └── response/           # Outgoing response DTOs
│   │   │       ├── mod.rs
│   │   │       ├── pagination.rs    # Paginated response wrapper
│   │   │       ├── error.rs         # Standardized error response
│   │   │       └── success.rs       # Standardized success response
│   │   │
│   │   └── error.rs                # API-layer error handling
│   │
│   ├── modules/                    # Business Modules (Core of Modular Monolith)
│   │   ├── mod.rs                  # Module registry
│   │   │
│   │   ├── users/                  # Module: User Management
│   │   │   ├── mod.rs
│   │   │   ├── config.rs           # Module-specific configuration
│   │   │   │
│   │   │   ├── domain/             # Domain Layer (Hexagonal Core)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/       # Domain entities
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user.rs
│   │   │   │   │   ├── role.rs
│   │   │   │   │   └── permission.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/ # Value objects
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── email.rs
│   │   │   │   │   ├── password.rs
│   │   │   │   │   └── user_id.rs
│   │   │   │   │
│   │   │   │   ├── events/        # Domain events
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user_created.rs
│   │   │   │   │   ├── user_updated.rs
│   │   │   │   │   └── user_deleted.rs
│   │   │   │   │
│   │   │   │   └── errors.rs       # Domain-specific errors
│   │   │   │
│   │   │   ├── ports/             # Ports Layer (Interfaces)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/       # Primary ports (use cases)
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user_commands.rs
│   │   │   │   │   └── user_queries.rs
│   │   │   │   │
│   │   │   │   └── outbound/       # Secondary ports (implementations)
│   │   │   │       ├── mod.rs
│   │   │   │       ├── user_repository.rs
│   │   │   │       ├── password_hasher.rs
│   │   │   │       └── token_service.rs
│   │   │   │
│   │   │   ├── application/        # Application Layer (Use Cases)
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/       # Command handlers (writes)
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── create_user.rs
│   │   │   │   │   ├── update_user.rs
│   │   │   │   │   ├── delete_user.rs
│   │   │   │   │   └── change_password.rs
│   │   │   │   │
│   │   │   │   │── queries/       # Query handlers (reads)
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── get_user_by_id.rs
│   │   │   │   │   ├── get_user_by_email.rs
│   │   │   │   │   └── list_users.rs
│   │   │   │   │
│   │   │   │   └── services/      # Application services
│   │   │   │       ├── mod.rs
│   │   │   │       ├── auth_service.rs
│   │   │   │       └── user_service.rs
│   │   │   │
│   │   │   └── adapters/           # Adapters Layer (Implementations)
│   │   │       ├── mod.rs
│   │   │       ├── api/            # REST API adapter
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs   # Module-specific routes
│   │   │       │   ├── handlers.rs # HTTP handlers
│   │   │       │   ├── dto.rs      # Request/Response DTOs
│   │   │       │   └── mapper.rs   # DTO <-> Domain mappers
│   │   │       │
│   │   │       └── persistence/    # Repository implementations
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs # SQL repository
│   │   │           └── redis.rs    # Cache repository
│   │   │
│   │   ├── vendors/                # Module: Vendor Management
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vendor.rs
│   │   │   │   │   ├── vendor_onboarding.rs
│   │   │   │   │   └── vendor_subscription.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vendor_id.rs
│   │   │   │   │   ├── commission_rate.rs
│   │   │   │   │   └── payout_info.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vendor_onboarding_service.rs
│   │   │   │   │   └── vendor_commission_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vendor_created.rs
│   │   │   │   │   ├── vendor_verified.rs
│   │   │   │   │   └── vendor_suspended.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── vendor_commands.rs
│   │   │   │   │   └── vendor_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── vendor_repository.rs
│   │   │   │       ├── verification_service.rs
│   │   │   │       └── payout_service.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── register_vendor.rs
│   │   │   │   │   ├── verify_vendor.rs
│   │   │   │   │   ├── update_vendor.rs
│   │   │   │   │   ├── suspend_vendor.rs
│   │   │   │   │   └── update_commission.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_vendor_by_id.rs
│   │   │   │       ├── get_vendor_by_slug.rs
│   │   │   │       ├── list_vendors.rs
│   │   │   │       └── get_vendor_stats.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── products/              # Module: Product Catalog
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── product.rs
│   │   │   │   │   ├── category.rs
│   │   │   │   │   ├── brand.rs
│   │   │   │   │   ├── sku.rs
│   │   │   │   │   ├── attribute.rs
│   │   │   │   │   └── variant.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── product_id.rs
│   │   │   │   │   ├── money.rs
│   │   │   │   │   ├── dimensions.rs
│   │   │   │   │   └── inventory.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── product_service.rs
│   │   │   │   │   ├── pricing_service.rs
│   │   │   │   │   └── inventory_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── product_created.rs
│   │   │   │   │   ├── product_updated.rs
│   │   │   │   │   ├── product_published.rs
│   │   │   │   │   ├── product_unpublished.rs
│   │   │   │   │   └── inventory_low.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── product_commands.rs
│   │   │   │   │   ├── product_queries.rs
│   │   │   │   │   ├── category_commands.rs
│   │   │   │   │   └── category_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── product_repository.rs
│   │   │   │       ├── category_repository.rs
│   │   │   │       ├── search_service.rs
│   │   │   │       └── media_service.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── create_product.rs
│   │   │   │   │   ├── update_product.rs
│   │   │   │   │   ├── delete_product.rs
│   │   │   │   │   ├── publish_product.rs
│   │   │   │   │   ├── create_category.rs
│   │   │   │   │   └── update_category.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_product_by_id.rs
│   │   │   │       ├── get_product_by_sku.rs
│   │   │   │       ├── search_products.rs
│   │   │       ├── list_products.rs
│   │   │   │       ├── get_category_tree.rs
│   │   │   │       └── get_product_inventory.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           ├── elasticsearch.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── inventory/              # Module: Inventory Management
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── stock.rs
│   │   │   │   │   ├── warehouse.rs
│   │   │   │   │   └── reservation.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── stock_level.rs
│   │   │   │   │   └── reorder_point.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── stock_service.rs
│   │   │   │   │   └── reservation_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── stock_reserved.rs
│   │   │   │   │   ├── stock_released.rs
│   │   │   │   │   └── stock_low_alert.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── inventory_commands.rs
│   │   │   │   │   └── inventory_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── inventory_repository.rs
│   │   │   │       └── warehouse_repository.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── reserve_stock.rs
│   │   │   │   │   ├── release_stock.rs
│   │   │   │   │   ├── adjust_stock.rs
│   │   │   │   │   └── transfer_stock.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_inventory.rs
│   │   │   │       ├── list_reservations.rs
│   │   │   │       └── get_low_stock_items.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── orders/                  # Module: Order Management
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── order.rs
│   │   │   │   │   ├── order_item.rs
│   │   │   │   │   ├── shipping_address.rs
│   │   │   │   │   └── order_status.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── order_id.rs
│   │   │   │   │   ├── tracking_number.rs
│   │   │   │   │   └── order_total.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── order_service.rs
│   │   │   │   │   ├── pricing_service.rs
│   │   │   │   │   └── fulfillment_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── order_created.rs
│   │   │   │   │   ├── order_confirmed.rs
│   │   │   │   │   ├── order_shipped.rs
│   │   │   │   │   ├── order_delivered.rs
│   │   │   │   │   ├── order_cancelled.rs
│   │   │   │   │   └── refund_initiated.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── order_commands.rs
│   │   │   │   │   └── order_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── order_repository.rs
│   │   │   │       ├── payment_gateway.rs
│   │   │   │       ├── shipping_service.rs
│   │   │   │       └── notification_service.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── create_order.rs
│   │   │   │   │   ├── confirm_order.rs
│   │   │   │   │   ├── cancel_order.rs
│   │   │   │   │   ├── update_shipping.rs
│   │   │   │   │   └── initiate_refund.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_order_by_id.rs
│   │   │   │       ├── list_orders.rs
│   │   │   │       ├── get_vendor_orders.rs
│   │   │   │       └── get_order_tracking.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── payments/                # Module: Payment Processing
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── payment.rs
│   │   │   │   │   ├── refund.rs
│   │   │   │   │   └── payment_method.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── payment_id.rs
│   │   │   │   │   ├── amount.rs
│   │   │   │   │   └── transaction_id.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── payment_service.rs
│   │   │   │   │   └── refund_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── payment_completed.rs
│   │   │   │   │   ├── payment_failed.rs
│   │   │   │   │   └── refund_processed.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── payment_commands.rs
│   │   │   │   │   └── payment_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── payment_repository.rs
│   │   │   │       ├── payment_gateway.rs
│   │   │   │       └── webhook_handler.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── process_payment.rs
│   │   │   │   │   ├── refund_payment.rs
│   │   │   │   │   └── capture_payment.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_payment.rs
│   │   │   │       ├── list_payments.rs
│   │   │   │       └── get_payment_status.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── shipping/                # Module: Shipping & Fulfillment
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── shipment.rs
│   │   │   │   │   ├── carrier.rs
│   │   │   │   │   └── rate.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── shipment_id.rs
│   │   │   │   │   └── tracking_info.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── shipping_service.rs
│   │   │   │   │   └── rate_calculator.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── shipment_created.rs
│   │   │   │   │   ├── shipment_in_transit.rs
│   │   │   │   │   └── shipment_delivered.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── shipping_commands.rs
│   │   │   │   │   └── shipping_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── shipment_repository.rs
│   │   │   │       └── carrier_service.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── create_shipment.rs
│   │   │   │   │   ├── update_tracking.rs
│   │   │   │   │   └── cancel_shipment.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_shipment.rs
│   │   │   │       ├── list_shipments.rs
│   │   │   │       └── get_shipping_rates.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── reviews/                  # Module: Reviews & Ratings
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── review.rs
│   │   │   │   │   ├── rating.rs
│   │   │   │   │   └── review_vote.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── review_id.rs
│   │   │   │   │   └── review_content.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── review_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── review_created.rs
│   │   │   │   │   ├── review_approved.rs
│   │   │   │   │   └── review_reported.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── review_commands.rs
│   │   │   │   │   └── review_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── review_repository.rs
│   │   │   │       └── moderation_service.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── create_review.rs
│   │   │   │   │   ├── approve_review.rs
│   │   │   │   │   ├── report_review.rs
│   │   │   │   │   └── vote_helpful.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_review.rs
│   │   │   │       ├── list_product_reviews.rs
│   │   │   │       ├── list_vendor_reviews.rs
│   │   │   │       └── get_review_stats.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   ├── notifications/           # Module: Notifications
│   │   │   ├── mod.rs
│   │   │   ├── config.rs
│   │   │   │
│   │   │   ├── domain/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── entities/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── notification.rs
│   │   │   │   │   └── template.rs
│   │   │   │   │
│   │   │   │   ├── value_objects/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── notification_id.rs
│   │   │   │   │   └── channel.rs
│   │   │   │   │
│   │   │   │   ├── services/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── notification_service.rs
│   │   │   │   │
│   │   │   │   ├── events/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── notification_sent.rs
│   │   │   │   │   └── notification_failed.rs
│   │   │   │   │
│   │   │   │   └── errors.rs
│   │   │   │
│   │   │   ├── ports/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── inbound/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── notification_commands.rs
│   │   │   │   │   └── notification_queries.rs
│   │   │   │   └── outbound/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── notification_repository.rs
│   │   │   │       ├── email_sender.rs
│   │   │   │       ├── sms_sender.rs
│   │   │   │       └── push_notification.rs
│   │   │   │
│   │   │   ├── application/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── send_notification.rs
│   │   │   │   │   └── mark_as_read.rs
│   │   │   │   │
│   │   │   │   └── queries/
│   │   │   │       ├── mod.rs
│   │   │   │       ├── get_notification.rs
│   │   │   │       └── list_notifications.rs
│   │   │   │
│   │   │   └── adapters/
│   │   │       ├── mod.rs
│   │   │       ├── api/
│   │   │       │   ├── mod.rs
│   │   │       │   ├── router.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── dto.rs
│   │   │       │   └── mapper.rs
│   │   │       └── persistence/
│   │   │           ├── mod.rs
│   │   │           ├── postgres.rs
│   │   │           └── redis.rs
│   │   │
│   │   └── cart/                    # Module: Shopping Cart
│   │       ├── mod.rs
│   │       ├── config.rs
│   │       │
│   │       ├── domain/
│   │       │   ├── mod.rs
│   │       │   ├── entities/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── cart.rs
│   │       │   │   └── cart_item.rs
│   │       │   │
│   │       │   ├── value_objects/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── cart_id.rs
│   │       │   │   └── cart_total.rs
│   │       │   │
│   │       │   ├── services/
│   │       │   │   ├── mod.rs
│   │       │   │   └── cart_service.rs
│   │       │   │
│   │       │   ├── events/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── item_added.rs
│   │       │   │   ├── item_removed.rs
│   │       │   │   └── cart_cleared.rs
│   │       │   │
│   │       │   └── errors.rs
│   │       │
│   │       ├── ports/
│   │       │   ├── mod.rs
│   │       │   ├── inbound/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── cart_commands.rs
│   │       │   │   └── cart_queries.rs
│   │       │   └── outbound/
│   │       │       ├── mod.rs
│   │       │       └── cart_repository.rs
│   │       │
│   │       ├── application/
│   │       │   ├── mod.rs
│   │       │   ├── commands/
│   │       │   │   ├── mod.rs
│   │       │   │   ├── add_item.rs
│   │       │   │   ├── remove_item.rs
│   │       │   │   ├── update_quantity.rs
│   │       │   │   └── clear_cart.rs
│   │       │   │
│   │       │   └── queries/
│   │       │       ├── mod.rs
│   │       │       └── get_cart.rs
│   │       │
│   │       └── adapters/
│   │           ├── mod.rs
│   │           ├── api/
│   │           │   ├── mod.rs
│   │           │   ├── router.rs
│   │           │   ├── handlers.rs
│   │           │   ├── dto.rs
│   │           │   └── mapper.rs
│   │           └── persistence/
│   │               ├── mod.rs
│   │               ├── postgres.rs
│   │               └── redis.rs
│   │
│   ├── shared/                      # Shared Infrastructure
│   │   ├── mod.rs
│   │   │
│   │   ├── kernel/                  # Shared domain kernel
│   │   │   ├── mod.rs
│   │   │   ├── error.rs             # Unified error handling
│   │   │   ├── result.rs            # Result type extensions
│   │   │   ├── value_objects/       # Cross-module value objects
│   │   │   │   ├── mod.rs
│   │   │   │   ├── email.rs
│   │   │   │   ├── phone.rs
│   │   │   │   ├── address.rs
│   │   │   │   ├── money.rs
│   │   │   │   ├── slug.rs
│   │   │   │   └── pagination.rs
│   │   │   │
│   │   │   └── events/              # Cross-module events
│   │   │       ├── mod.rs
│   │   │       ├── domain_event.rs
│   │   │       └── event_bus.rs
│   │   │
│   │   ├── ports/                   # Shared ports (cross-cutting)
│   │   │   │   ├── mod.rs
│   │   │   ├── unit_of_work.rs      # Transaction management
│   │   │   ├── cache_port.rs        # Caching interface
│   │   │   ├── event_bus_port.rs    # Event publishing
│   │   │   └── file_storage_port.rs # File storage interface
│   │   │
│   │   ├── middleware/              # Shared middleware
│   │   │   │   ├── mod.rs
│   │   │   │   └── timeout.rs
│   │   │
│   │   └── utils/                   # Shared utilities
│   │       ├── mod.rs
│   │       ├── id_generator.rs     # UUID/snowflake generation
│   │       ├── date_time.rs         # Date/time utilities
│   │       └── validation.rs        # Common validation helpers
│   │
│   ├── infrastructure/              # Infrastructure Layer
│   │   ├── mod.rs
│   │   │
│   │   ├── config/                  # Configuration management
│   │   │   ├── mod.rs
│   │   │   ├── app_config.rs        # Application configuration
│   │   │   ├── database_config.rs   # Database configuration
│   │   │   ├── cache_config.rs      # Cache configuration
│   │   │   ├── jwt_config.rs        # JWT configuration
│   │   │   ├── cors_config.rs       # CORS configuration
│   │   │   └── rate_limit_config.rs # Rate limiting config
│   │   │
│   │   ├── database/                # Database infrastructure
│   │   │   ├── mod.rs
│   │   │   ├── connection_pool.rs    # Connection pool management (sqlx::PgPool)
│   │   │   ├── repositories/         # Generic repository implementations
│   │   │   │   ├── mod.rs
│   │   │   │   ├── base_repository.rs
│   │   │   │   └── query_builder.rs
│   │   │   └── transaction.rs       # Transaction management (Unit of Work)
│   │   │
│   │   ├── cache/                   # Cache infrastructure
│   │   │   ├── mod.rs
│   │   │   ├── redis_pool.rs        # Redis connection pool
│   │   │   └── cache_layer.rs       # Cache abstraction layer
│   │   │
│   │   ├── logging/                 # Logging infrastructure
│   │   │   ├── mod.rs
│   │   │   ├── logger.rs            # Logger setup
│   │   │   ├── formatter.rs         # Log formatting
│   │   │   └── sinks.rs             # Log output sinks
│   │   │
│   │   ├── tracing/                 # Distributed tracing
│   │   │   ├── mod.rs
│   │   │   ├── tracer.rs            # Tracer setup
│   │   │   └── propagators.rs       # Context propagators
│   │   │
│   │   ├── auth/                    # Authentication infrastructure
│   │   │   ├── mod.rs
│   │   │   ├── jwt_service.rs       # JWT token handling
│   │   │   ├── password_hasher.rs   # Password hashing (argon2/bcrypt)
│   │   │   └── token_manager.rs     # Token lifecycle management
│   │   │
│   │   ├── external/                # External service clients
│   │   │   ├── mod.rs
│   │   │   ├── payment_gateways/     # Payment gateway adapters
│   │   │   │   ├── mod.rs
│   │   │   │   ├── stripe.rs
│   │   │   │   ├── paypal.rs
│   │   │   │   └── trait.rs         # Gateway trait definition
│   │   │   │
│   │   │   ├── email/               # Email service adapters
│   │   │   │   ├── mod.rs
│   │   │   │   ├── smtp.rs
│   │   │   │   ├── sendgrid.rs
│   │   │   │   └── trait.rs
│   │   │   │
│   │   │   ├── sms/                 # SMS service adapters
│   │   │   │   ├── mod.rs
│   │   │   │   ├── twilio.rs
│   │   │   │   └── trait.rs
│   │   │   │
│   │   │   ├── storage/             # File storage adapters
│   │   │   │   ├── mod.rs
│   │   │   │   ├── s3.rs
│   │   │   │   ├── local.rs
│   │   │   │   └── trait.rs
│   │   │   │
│   │   │   ├── search/              # Search engine adapters
│   │   │   │   ├── mod.rs
│   │   │   │   ├── elasticsearch.rs
│   │   │   │   └── trait.rs
│   │   │   │
│   │   │   └── shipping/            # Shipping carrier adapters
│   │   │       ├── mod.rs
│   │   │       ├── ups.rs
│   │   │       ├── fedex.rs
│   │   │       ├── usps.rs
│   │   │       └── trait.rs
│   │   │
│   │   ├── messaging/               # Event messaging
│   │   │   ├── mod.rs
│   │   │   ├── kafka/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── producer.rs
│   │   │   │   └── consumer.rs
│   │   │   ├── rabbitmq/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── publisher.rs
│   │   │   │   └── subscriber.rs
│   │   │   └── in_memory.rs         # In-memory for development
│   │   │
│   │   └── health/                  # Health check infrastructure
│   │       ├── mod.rs
│   │       ├── checks/
│   │       │   ├── mod.rs
│   │       │   ├── database.rs
│   │       │   ├── cache.rs
│   │       │   └── external.rs
│   │       └── reporter.rs
│   │
│   └── tests/                        # Test utilities
│       ├── mod.rs
│       ├── fixtures/                # Test fixtures
│       │   ├── mod.rs
│       │   ├── user_fixtures.rs
│       │   ├── vendor_fixtures.rs
│       │   ├── product_fixtures.rs
│       │   └── order_fixtures.rs
│       │
│       ├── helpers/                 # Test helpers
│       │   ├── mod.rs
│       │   ├── test_app.rs          # Test application setup
│       │   ├── test_client.rs       # HTTP test client
│       │   └── test_database.rs     # Test database setup
│       │
│       └── mocks/                   # Mock implementations
│           ├── mod.rs
│           ├── mock_repository.rs
│           ├── mock_event_bus.rs
│           └── mock_external_service.rs
│
├── migrations/                       # Database migrations (sqlx-cli)
│   ├── 001_initial_schema.up.sql    # sqlx migrate add creates this pattern
│   ├── 001_initial_schema.down.sql
│   ├── 002_add_vendors.up.sql
│   ├── 002_add_vendors.down.sql
│   └── ...
│   # Note: This is the CANONICAL location for sqlx migrations
│   # Run `sqlx migrate add <name>` to create new migrations
│   # sqlx-cli expects migrations here or at path specified in DATABASE_URL
│
├── scripts/                          # Utility scripts
│   ├── deploy.sh
│   ├── health_check.sh
│   └── benchmark.sh
│
├── config/                          # Configuration files
│   ├── development.yaml
│   ├── staging.yaml
│   ├── production.yaml
│   └── local.yaml
│
├── docs/                            # Documentation
│   ├── api/
│   │   ├── openapi.yaml
│   │   └── schemas/
│   └── architecture/
│       ├── domain_model.md
│       ├── module_relationships.md
│       └── deployment.md
│
└── .vscode/                        # VS Code settings
    ├── settings.json
    └── launch.json
```

## Module Interaction Diagram

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              API Layer                                    │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│  │  Users   │ │ Vendors │ │Products │ │ Orders  │ │ Payments│        │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘        │
└───────┼─────────────┼─────────────┼─────────────┼─────────────┼─────────────┘
        │             │             │             │             │
        ▼             ▼             ▼             ▼             ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                        Application Layer                                  │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│  │Commands │ │Commands │ │Commands │ │Commands │ │Commands │        │
│  │& Queries│ │& Queries│ │& Queries│ │& Queries│ │& Queries│        │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘        │
└───────┼─────────────┼─────────────┼─────────────┼─────────────┼─────────────┘
        │             │             │             │             │
        ▼             ▼             ▼             ▼             ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                          Domain Layer                                     │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
│  │ Entities│ │ Entities│ │ Entities│ │ Entities│ │ Entities│        │
│  │Services │ │Services │ │Services │ │Services │ │Services │        │
│  │ Value   │ │ Value   │ │ Value   │ │ Value   │ │ Value   │        │
│  │ Objects │ │ Objects │ │ Objects │ │ Objects │ │ Objects │        │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘ └────┬─────┘        │
└───────┼─────────────┼─────────────┼─────────────┼─────────────┼─────────────┘
        │             │             │             │             │
        ▼             ▼             ▼             ▼             ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         Ports Layer                                      │
│  ┌─────────────────────────────────────────────────────────────────┐      │
│  │                     Outbound Ports                              │      │
│  │  (Repository Traits, External Service Traits, Event Bus)        │      │
│  └─────────────────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                       Adapters Layer                                     │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐     │
│  │Postgres  │ │  Redis   │ │ Stripe   │ │  Email   │ │    S3    │     │
│  │Repository│ │  Cache   │ │ Gateway  │ │ Service  │ │ Storage  │     │
│  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └──────────┘     │
└─────────────────────────────────────────────────────────────────────────┘
```

## Key Architecture Decisions

1. **Module Isolation**: Each module (`users`, `vendors`, `products`, etc.) is self-contained with its own domain,
   ports, application, and adapters layers.

2. **Hexagonal Boundaries**: The hexagonal architecture is applied within each module, allowing for easy testing and
   swapping of implementations.

3. **Shared Kernel**: Cross-cutting concerns (error handling, value objects like Email, Address, Money) are centralized
   in `shared/kernel`.

4. **CQRS-lite**: Each module separates commands (write operations) and queries (read operations) in its application
   layer.

5. **RESTful API**: Each module exposes a RESTful API through its API adapter, using proper HTTP methods, status codes,
   and resource naming.

6. **Multi-tenancy**: Built-in support for multi-vendor (multi-tenant) through middleware and tenant-aware repositories.

7. **Event-Driven**: Domain events are published for cross-module communication (e.g., `OrderCreated` event triggers
   inventory reservation).

8. **Infrastructure Separation**: External services (payments, email, storage) are abstracted behind ports and have
   multiple adapter implementations.

## File Naming Conventions

- **Rust files**: `snake_case.rs` (e.g., `order_service.rs`, `create_order.rs`)
- **Modules**: `mod.rs` for module declarations
- **Tests**: `*_test.rs` or `mod tests` blocks within source files
- **Integration tests**: Located in `tests/` directory
- **Migrations**: `XXX_description.up.sql` and `XXX_description.down.sql`
