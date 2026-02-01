# Njalla API Reference

Documentation for the Njalla API used by this CLI.

## Overview

- **Endpoint**: `POST https://njal.la/api/1/`
- **Auth**: `Authorization: Njalla <token>` header
- **Format**: JSON-RPC style (method + params in body)

## Request Format

```json
{
  "method": "method-name",
  "params": {
    "key": "value"
  }
}
```

## Response Format

### Success
```json
{
  "result": {
    // method-specific response
  }
}
```

### Error
```json
{
  "error": {
    "message": "Error description"
  }
}
```

---

## Methods

### list-domains

List all domains in account.

**Params**: none

**Response**:
```json
{
  "result": {
    "domains": [
      {
        "name": "example.com",
        "status": "active",
        "expiry": "2027-01-15T00:00:00Z",
        "locked": false,
        "mailforwarding": false,
        "max_nameservers": 8
      }
    ]
  }
}
```

### get-domain

Get detailed info for a specific domain.

**Params**:
```json
{
  "domain": "example.com"
}
```

**Response**: Same structure as list-domains, single domain.

### find-domains

Search for available domains.

**Params**:
```json
{
  "query": "example"
}
```

**Response**:
```json
{
  "result": {
    "domains": [
      {
        "name": "example.com",
        "status": "available",
        "price": 15
      },
      {
        "name": "example.net",
        "status": "taken",
        "price": 15
      }
    ]
  }
}
```

### register-domain

Register a new domain. Requires sufficient wallet balance.

**Params**:
```json
{
  "domain": "example.com",
  "years": 1
}
```

**Response**:
```json
{
  "result": {
    "task": "abc123-task-id"
  }
}
```

### check-task

Check status of an async task (e.g., registration).

**Params**:
```json
{
  "id": "abc123-task-id"
}
```

**Response**:
```json
{
  "result": {
    "id": "abc123-task-id",
    "status": "completed"
  }
}
```

Status values: `pending`, `processing`, `completed`, `failed`

### list-records

List DNS records for a domain.

**Params**:
```json
{
  "domain": "example.com"
}
```

**Response**:
```json
{
  "result": {
    "records": [
      {
        "id": "12345",
        "name": "@",
        "type": "A",
        "content": "192.0.2.1",
        "ttl": 10800,
        "prio": null
      }
    ]
  }
}
```

### add-record

Add a DNS record.

**Params**:
```json
{
  "domain": "example.com",
  "name": "@",
  "type": "A",
  "content": "192.0.2.1",
  "ttl": 10800
}
```

### edit-record

Edit an existing DNS record.

**Params**:
```json
{
  "domain": "example.com",
  "id": "12345",
  "name": "@",
  "type": "A",
  "content": "192.0.2.2",
  "ttl": 10800
}
```

### remove-record

Delete a DNS record.

**Params**:
```json
{
  "domain": "example.com",
  "id": "12345"
}
```

---

## Valid Values

### TTL (seconds)
- 60, 300, 900, 3600, 10800, 21600, 86400

### Priority (for MX, SRV)
- 0, 1, 5, 10, 20, 30, 40, 50, 60

### Record Types
- A, AAAA, CNAME, MX, TXT, NS, SRV, CAA

---

## Error Codes

Common error messages:
- `"Invalid token"` - Bad or expired API token
- `"Domain not found"` - Domain not in account
- `"Insufficient funds"` - Wallet balance too low
- `"Domain not available"` - Already registered

---

## Rate Limits

No documented rate limits, but be reasonable. Suggested:
- Max 10 requests/second
- Add exponential backoff on errors
