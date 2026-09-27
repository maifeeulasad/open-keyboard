# open-keyboard documentation

This directory is the single source of truth for how open-keyboard is designed,
decided, and built. Because the project is run **from the README with no GitHub
issues or feature branches** (see
[Project management](development/project-management.md)), these documents also
serve as our planning and tracking system.

## Map

| Area | Document | What it covers |
| ---- | -------- | -------------- |
| **Architecture** | [architecture/overview.md](architecture/overview.md) | System structure, crates, data flow, how the IME plugs into Linux. |
| | [architecture/security.md](architecture/security.md) | Threat model and security architecture for a keystroke-path component. |
| **Decisions** | [decisions/README.md](decisions/README.md) | Index of Architecture Decision Records (ADRs). |
| **Design** | [design/engine.md](design/engine.md) | The framework-agnostic core engine (`okb-engine`) internals. |
| | [design/phonetic-scheme.md](design/phonetic-scheme.md) | The clean-room English→Bengali romanization scheme. |
| **Roadmap** | [roadmap.md](roadmap.md) | Phased plan and current status (our task tracker). |
| **Development** | [development/build.md](development/build.md) | How to build, test, run, and lint. |
| | [development/project-management.md](development/project-management.md) | How we run the project without issues/branches. |

## Conventions

- **Language:** English for docs and code; Bengali (and later other scripts) is
  the *subject*, not the medium.
- **Decisions** are immutable once accepted. To change one, add a new ADR that
  supersedes it.
- **Diagrams** are written in [Mermaid](https://mermaid.js.org/) so they render on
  GitHub and stay diff-able as text.
