# Security & Threat Model

- **Password handling**: kept in memory briefly and zeroized after use.
- **Logging**: secrets are never logged; redact lengths only.
- **Attack surface**: vendor control endpoints; avoid malformed writes; set conservative timeouts.
- **User guidance**: advise interactive prompt, warn about command-line args.
