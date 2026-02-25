# Bayarcash CLI + MCP Server - Design Document

**Date:** 2026-02-25
**Status:** Approved

## Overview

Add a CLI binary (`bayarcash`) and MCP server binary (`bayarcash-mcp`) to the existing bayarcash-sdk crate. Both share the same library code. Designed for AI agent usage with JSON output.

## Decisions

| Aspect | Decision |
|--------|----------|
| CLI framework | clap (derive) |
| MCP framework | rmcp |
| MCP transport | stdio |
| Output format | JSON to stdout, errors to stderr |
| Config | ~/.bayarcash/config.toml + env vars + CLI flags |
| Config precedence | CLI flags > env vars > config file |
| Binary targets | `bayarcash` (CLI), `bayarcash-mcp` (MCP server) |

## CLI Commands

```
bayarcash [--sandbox] [--api-version v2|v3]

  init                              # Create config interactively
  payment create [flags]            # Create payment intent
  payment get <id>                  # Get payment intent (v3)
  transaction get <id>              # Get transaction
  transaction list [filters]        # List transactions (v3)
  banks list                        # List FPX banks
  portal list                       # List portals
  portal channels <key>             # List portal channels
  checksum payment [flags]          # Generate payment checksum
  checksum enrollment [flags]       # Generate DD enrollment checksum
  checksum maintenance [flags]      # Generate DD maintenance checksum
  verify transaction <json|stdin>   # Verify callback checksums
  verify pre-transaction <json>
  verify return-url <json>
  verify dd-approval <json>
  verify dd-authorization <json>
  verify dd-transaction <json>
  mandate create <json>             # DD enrollment
  mandate update <id> <json>        # DD maintenance
  mandate terminate <id> <json>     # DD termination
  mandate get <id>                  # Get mandate
  mandate transaction <id>          # Get mandate transaction
```

## MCP Server Tools

13 tools exposed via stdio transport:
- create_payment_intent, get_payment_intent
- get_transaction, list_transactions
- list_banks, list_portals, get_channels
- generate_checksum, verify_callback
- create_mandate, update_mandate, terminate_mandate, get_mandate

## Config File

`~/.bayarcash/config.toml`:
```toml
token = "your_api_token"
secret_key = "your_secret_key"
sandbox = false
api_version = "v2"
```

Env vars: BAYARCASH_TOKEN, BAYARCASH_SECRET_KEY, BAYARCASH_SANDBOX, BAYARCASH_API_VERSION

## New Dependencies

- clap (derive feature) - CLI argument parsing
- rmcp (transport-stdio feature) - MCP server
- dirs - find home directory for config
- toml - parse config file
- serde (already present)
