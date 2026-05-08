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

1. Roles Table is for platform-wide permissions.
2. vendor status should be enum. because each state change may require custom validation. Therefore, code must know
   about all possible values beforehand.
3. Product tags should be look-up tables. State changes do not require custom validation
4. otp_status is needed for applying unique database
   constraints. This is invalid:
   ```CREATE UNIQUE INDEX uniq_active_otp ON otps (identity_id, purpose) WHERE expires_at > NOW();```
5. OTP status ACTIVE -> EXPIRED should be handled lazily.
6. Display implementations are for logs, debugging, CLI output, and APIs.
7. Only Id wrappers should implement Copy.