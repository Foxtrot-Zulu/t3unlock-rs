# Changelog

## v0.2.0 — Bulk unlock working for T3
- Implemented real Samsung T3 unlock using **bulk** I/O:
  - OUT unlock(31), OUT password(512), IN return(512), OUT relink(31), IN return(512)
  - Return byte[9] == 0x02 → failure
- Default PID now **0x61f4** (T3 locked)
- Updated udev rule to 61f4; improved README and protocol docs
- Tidied logging and permissions guidance

## v0.1.0 — Scaffold
- Initial CLI + structure, placeholder protocol, docs, CI
