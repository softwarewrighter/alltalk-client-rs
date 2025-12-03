# Project Status

## Current Phase: Pre-Development

**Last Updated:** 2025-12-03

## Overall Progress

| Phase | Status | Progress |
|-------|--------|----------|
| Phase 1: Foundation (tts-spec) | Not Started | 0% |
| Phase 2: Engine Client (tts-engine) | Not Started | 0% |
| Phase 3: Audio Pipeline (tts-render) | Not Started | 0% |
| Phase 4: CLI (ttsctl) | Not Started | 0% |
| Phase 5: Integration | Not Started | 0% |
| Phase 6: Enhancements | Not Started | 0% |

## What Exists

### Documentation
- [x] `docs/ai_agent_instructions.md` - AI coding agent guidelines
- [x] `docs/process.md` - Development process (TDD, pre-commit)
- [x] `docs/tools.md` - Development tools reference
- [x] `docs/research.txt` - ChatGPT research session on architecture
- [x] `docs/architecture.md` - System architecture
- [x] `docs/prd.md` - Product requirements
- [x] `docs/design.md` - Detailed design
- [x] `docs/plan.md` - Implementation plan
- [x] `docs/status.md` - This file

### Code
- [x] `Cargo.toml` - Basic package definition (needs workspace config)
- [x] `src/main.rs` - Hello world placeholder
- [ ] Component structure - Not created
- [ ] Core types - Not implemented
- [ ] Tests - None

### Infrastructure
- [x] Git repository initialized
- [ ] `.gitignore` - Needs creation
- [ ] `LICENSE` - Exists (needs verification)
- [ ] `COPYRIGHT` - Exists (needs verification)
- [ ] CI/CD - Not configured

## Blockers

None currently. Ready to begin Phase 1.

## Recent Activity

| Date | Activity |
|------|----------|
| 2025-12-03 | Created documentation structure (architecture, prd, design, plan, status) |
| 2025-12-03 | Initial project setup with Cargo.toml |

## Decisions Made

1. **Architecture**: Multi-component workspace with strict layering
2. **Code constraints**: Max 4 functions/module, 5 modules/crate, 5 crates/component
3. **TTS Hub**: AllTalk v2 as the backend server
4. **Primary engines**: Parler (Apache-2.0), Piper (GPL-3.0)
5. **Pause control**: Post-processing silence insertion, not engine-dependent

## Decisions Pending

1. **Config format**: TOML vs YAML for configuration files
2. **Error crate**: thiserror vs anyhow for error types
3. **Sample rate**: Default 24kHz or 48kHz
4. **Non-verbal assets**: Directory structure and naming convention

## Next Milestone

**Phase 1 Complete**: Core data types and script parsing working with tests.

Target deliverables:
- `tts-spec-model` crate with all types
- `tts-spec-script` crate with YAML/JSON parsing
- `tts-spec-layout` crate with timeline building
- Unit tests for all functionality
- Documentation for all public items

## Quality Metrics

| Metric | Target | Current |
|--------|--------|---------|
| Test coverage | > 80% | N/A |
| Clippy warnings | 0 | N/A |
| Doc coverage | 100% public | N/A |
| TODO count | < 3/file | N/A |

## Pre-Commit Checklist (Every Commit)

Per `docs/process.md`, these steps are **mandatory before every commit**:

1. `cargo test` - All tests pass
2. `cargo clippy --all-targets --all-features -- -D warnings` - Zero warnings
3. `cargo fmt --all` - Code formatted
4. `markdown-checker -f "**/*.md"` - Markdown validated (if docs changed)
5. `git status` - Verify .gitignore is correct
6. `sw-checklist` - Project requirements met
7. Update `docs/learnings.md` if issues were found
8. Commit with clear message and co-authorship

## Notes

- Rust edition is set to 2024 (verify toolchain compatibility)
- AllTalk server not yet set up (needed for Phase 2 testing)
- Consider creating a mock server for offline development
