# Njalla API Reference

This document describes the Njalla API based on the [gonjalla](https://github.com/Sighery/gonjalla) library tests and official API behavior.

## Overview

- **Endpoint**: `https://njal.la/api/1/`
- **Protocol**: JSON-RPC 2.0
- **Method**: POST only
- **Authentication**: `Authorization: Njalla <token>` header
- **API Key**: Obtain from https://njal.la/settings/api/

## Request Format

All requests use the same JSON-RPC structure:

```json
{
  "method": "method-name",
  "params": { /* method-specific parameters */ }
}
```

## Response Format

### Success Response

```json
{
  "jsonrpc": "2.0",
  "result": { /* method-specific result */ }
}
```

### Error Response

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": 0,
    "message": "Error description"
  }
}
```

---

## Domain Methods

### `list-domains`

List all domains in the account.

**Request:**
```json
{
  "method": "list-domains",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "domains": [
      {
        "name": "testing1.com",
        "status": "active",
        "expiry": "2021-02-20T19:38:48Z"
      },
      {
        "name": "testing2.com",
        "status": "inactive",
        "expiry": "2021-02-20T19:38:48Z"
      }
    ]
  }
}
```

### `get-domain`

Get details for a specific domain.

**Request:**
```json
{
  "method": "get-domain",
  "params": {
    "domain": "testing.com"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "name": "testing.com",
    "status": "active",
    "expiry": "2021-02-20T19:38:48Z",
    "locked": true,
    "mailforwarding": false,
    "max_nameservers": 10
  }
}
```

### `find-domains`

Search for domain availability and pricing.

**Request:**
```json
{
  "method": "find-domains",
  "params": {
    "query": "testing"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "domains": [
      {"name": "testing.com", "status": "taken", "price": 45},
      {"name": "testing.net", "status": "available", "price": 30},
      {"name": "testing.rocks", "status": "in progress", "price": 15},
      {"name": "testing.express", "status": "failed", "price": 75}
    ]
  }
}
```

**Status Values:**
- `available` - Domain can be registered
- `taken` - Domain is already registered
- `in progress` - Registration in progress
- `failed` - Registration failed

### `register-domain`

Register a new domain.

**Request:**
```json
{
  "method": "register-domain",
  "params": {
    "domain": "example.com",
    "years": 1
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "task": "task-id-string"
  }
}
```

The `task` ID is used with `check-task` to poll registration status.

### `check-task`

Check the status of an async operation (like domain registration).

**Request:**
```json
{
  "method": "check-task",
  "params": {
    "id": "task-id-string"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "task-id-string",
    "status": "completed"
  }
}
```

**Status Values:**
- `pending` - Task is queued
- `processing` - Task is running
- `completed` - Task finished successfully
- `failed` - Task failed

---

## DNS Record Methods

### Valid Values

**TTL (seconds):**
```
60, 300, 900, 3600, 10800, 21600, 86400
```

**Priority (for MX/SRV records):**
```
0, 1, 5, 10, 20, 30, 40, 50, 60
```

**Record Types:**
```
A, AAAA, CNAME, MX, TXT, NS, SRV, CAA
```

### `list-records`

List all DNS records for a domain.

**Request:**
```json
{
  "method": "list-records",
  "params": {
    "domain": "example.com"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "records": [
      {
        "id": "1337",
        "name": "_acme-challenge",
        "type": "TXT",
        "content": "long-string",
        "ttl": 10800
      },
      {
        "id": "1338",
        "name": "@",
        "type": "A",
        "content": "1.2.3.4",
        "ttl": 3600
      },
      {
        "id": "1339",
        "name": "@",
        "type": "AAAA",
        "content": "2001:0DB8:0000:0000:0000:8A2E:0370:7334",
        "ttl": 900
      },
      {
        "id": "1340",
        "name": "@",
        "type": "MX",
        "content": "mail.protonmail.ch",
        "ttl": 300,
        "prio": 10
      }
    ]
  }
}
```

### `add-record`

Add a new DNS record.

**Request:**
```json
{
  "method": "add-record",
  "params": {
    "domain": "example.com",
    "name": "@",
    "type": "MX",
    "content": "testing.com",
    "ttl": 10800,
    "prio": 10
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "1337",
    "name": "@",
    "type": "MX",
    "content": "testing.com",
    "ttl": 10800,
    "prio": 10
  }
}
```

### `edit-record`

Edit an existing DNS record. Note: The record `type` cannot be changed.

**Request:**
```json
{
  "method": "edit-record",
  "params": {
    "domain": "example.com",
    "id": "1337",
    "name": "@",
    "type": "MX",
    "content": "updated.com",
    "ttl": 3600,
    "prio": 5
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {}
}
```

### `remove-record`

Delete a DNS record.

**Request:**
```json
{
  "method": "remove-record",
  "params": {
    "domain": "example.com",
    "id": "1337"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {}
}
```

---

## Wallet Methods

### `get-balance`

Get the current wallet balance.

**Request:**
```json
{
  "method": "get-balance",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "balance": 100
  }
}
```

Balance is in EUR (euros).

### `add-payment`

Create a payment to add funds to the wallet.

**Request:**
```json
{
  "method": "add-payment",
  "params": {
    "amount": 15,
    "currency": "bitcoin"
  }
}
```

**Constraints:**
- Amount: 5 EUR or multiples of 15, up to 300 EUR
- Currency: `bitcoin` only

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "pay123",
    "amount": 15,
    "currency": "EUR",
    "amount_btc": "0.0002564",
    "address": "bc1qtest",
    "uri": "bitcoin:bc1qtest?amount=0.0002564"
  }
}
```

### `get-payment`

Check the status of a payment.

**Request:**
```json
{
  "method": "get-payment",
  "params": {
    "id": "pay123"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "id": "pay123",
    "amount": 15,
    "currency": "EUR",
    "amount_btc": "0.0002564",
    "status": "Waiting for transaction of 15 € via Bitcoin to be confirmed",
    "address": "bc1qtest",
    "uri": "bitcoin:bc1qtest?amount=0.0002564"
  }
}
```

### `list-transactions`

List wallet transactions from the last 90 days.

**Request:**
```json
{
  "method": "list-transactions",
  "params": {}
}
```

**Response (Completed Transaction):**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "transactions": [
      {
        "id": "IKSELBVIY5JW4UAER7PGLFEPSGHOJNB7",
        "amount": 210,
        "status": "Added 210 € via Bitcoin",
        "completed": "2026-02-01",
        "pdf": "https://njal.la/invoice/IKSELBVIY5JW4UAER7PGLFEPSGHOJNB7/"
      }
    ]
  }
}
```

**Response (Pending Transaction):**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "transactions": [
      {
        "id": "4S4IQTHCP3URAUMYUXCY4UTUGU666CVK",
        "amount": 15,
        "status": "Waiting for transaction of 15 € via Bitcoin to be confirmed",
        "uri": "bitcoin:bc1qtest?amount=0.0002539",
        "address": "bc1qtest",
        "currency": "EUR",
        "amount_btc": "0.0002539"
      }
    ]
  }
}
```

---

## Server Methods

### Server Object

```json
{
  "name": "my-server",
  "type": "15g",
  "id": "server-id",
  "status": "running",
  "os": "debian-12",
  "expiry": "2026-03-01",
  "autorenew": true,
  "sshkey": "ssh-ed25519 AAAA...",
  "ips": ["1.2.3.4", "2001:db8::1"],
  "reversename": "my-server.example.com",
  "osstate": "ready"
}
```

### `list-servers`

List all servers in the account.

**Request:**
```json
{
  "method": "list-servers",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "servers": [ /* Server objects */ ]
  }
}
```

### `list-server-images`

List available OS images.

**Request:**
```json
{
  "method": "list-server-images",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "images": ["debian-12", "ubuntu-22.04", "rocky-9"]
  }
}
```

### `list-server-types`

List available server types.

**Request:**
```json
{
  "method": "list-server-types",
  "params": {}
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "types": ["15g", "30g", "60g"]
  }
}
```

### `add-server`

Create a new server.

**Request:**
```json
{
  "method": "add-server",
  "params": {
    "name": "my-server",
    "type": "15g",
    "os": "debian-12",
    "sshkey": "ssh-ed25519 AAAA...",
    "months": 1
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

### `start-server`

Start a stopped server.

**Request:**
```json
{
  "method": "start-server",
  "params": {
    "id": "server-id"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

### `stop-server`

Stop a running server.

**Request:**
```json
{
  "method": "stop-server",
  "params": {
    "id": "server-id"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

### `restart-server`

Restart a server.

**Request:**
```json
{
  "method": "restart-server",
  "params": {
    "id": "server-id"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

### `reset-server`

Reset a server with a new OS image.

**Request:**
```json
{
  "method": "reset-server",
  "params": {
    "id": "server-id",
    "os": "debian-12",
    "sshkey": "ssh-ed25519 AAAA...",
    "type": "15g"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

### `remove-server`

Delete a server.

**Request:**
```json
{
  "method": "remove-server",
  "params": {
    "id": "server-id"
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": { /* Server object */ }
}
```

---

## Error Handling

### Common Error Messages

| Message | Meaning |
|---------|---------|
| `"Invalid token"` | Bad or expired API token |
| `"Domain not found"` | Domain not in account |
| `"Insufficient funds"` | Wallet balance too low |
| `"Domain not available"` | Already registered |
| `"Testing error"` | Generic error (from test fixtures) |

### Error Response Example

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": 0,
    "message": "Testing error"
  }
}
```

---

## Testing

Njalla does **not** provide a sandbox environment. For development testing:

1. **Use HTTP mocking** (recommended) - Mock the API responses using libraries like `wiremock`
2. **Use a test domain** - Register a cheap test domain for integration tests
3. **Implement dry-run modes** - Add flags to skip actual API calls

### Example Mock Setup (Rust with wiremock)

```rust
use wiremock::{MockServer, Mock, matchers::*, ResponseTemplate};

let mock_server = MockServer::start().await;

Mock::given(method("POST"))
    .and(header("Authorization", "Njalla test-token"))
    .and(body_json(serde_json::json!({
        "method": "list-domains",
        "params": {}
    })))
    .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "jsonrpc": "2.0",
        "result": {
            "domains": [
                {
                    "name": "testing.com",
                    "status": "active",
                    "expiry": "2027-01-15T00:00:00Z"
                }
            ]
        }
    })))
    .mount(&mock_server)
    .await;
```

---

## Rate Limits

No documented rate limits, but be reasonable. Suggested:
- Max 10 requests/second
- Add exponential backoff on errors

---

## Sources

- [gonjalla](https://github.com/Sighery/gonjalla) - Go library with comprehensive test fixtures
- [terraform-provider-njalla](https://github.com/Sighery/terraform-provider-njalla) - Terraform provider
- [Njalla Python Library](https://github.com/DevCa-IO/Njalla) - Python implementation
