# Recipe App

> Full-stack recipe management application built with Svelte 5 and Rust

A modern web application for browsing, organizing, and managing recipes. Features drag-and-drop organization, favorites management, and a clean responsive interface.

## Tech Stack

- **Frontend**: SvelteKit + Svelte 5 (TypeScript, Tailwind CSS)
- **Backend**: Rust + Axum + Sled (embedded NoSQL database)
- **UI Components**: shadcn-svelte

## Quick Start

### Prerequisites

- Node.js 18+
- Rust 1.70+

### Run Locally

**Backend** (Terminal 1):
```bash
cd packages/backend
cargo run
```

**Frontend** (Terminal 2):
```bash
cd packages/frontend
npm install
npm run dev
```

Open **http://localhost:5173** in your browser.

## Documentation

📚 **[Complete Documentation →](./docs/README.md)**

| Documentation | Description |
|--------------|-------------|
| [Architecture](./docs/architecture.md) | System design, data flow, and key decisions |
| [API Reference](./docs/api.md) | Backend endpoints and usage |
| [Frontend Guide](./docs/frontend-guide.md) | Svelte 5 patterns and component structure |
| [Development Guide](./docs/development.md) | How to run, build, and deploy |
| [Teacher Logs](./docs/teacher-logs.md) | Implementation history and rationale |
| [Coding Guidelines](./docs/coding-guideline.md) | Code standards and conventions |

## Features

- ✅ Browse recipes with card-based UI
- ✅ Mark favorites for quick access
- ✅ Drag-and-drop recipe organization
- ✅ Search functionality
- ✅ Add new recipes via slide-in drawer
- ✅ REST API with persistent storage
- ✅ Full TypeScript type safety

## Project Structure

```
copilot-bootcamp-capstone/
├── docs/                    # 📚 Documentation
├── packages/
│   ├── backend/            # 🦀 Rust API (Axum + Sled)
│   └── frontend/           # 💻 Svelte 5 app (SvelteKit)
├── .github/                # GitHub Copilot agents
└── README.md              # This file
```

## Development

See the [Development Guide](./docs/development.md) for detailed instructions.

**Quick commands**:

```bash
# Backend
cd packages/backend
cargo run              # Run server
cargo test             # Run tests
cargo clippy           # Lint

# Frontend
cd packages/frontend
npm run dev            # Dev server
npm run check          # Type check
npm run lint           # Lint
npm test               # Run tests
```

## API Endpoints

- `GET /recipes` - Get all recipes
- `POST /recipes` - Create a recipe

See [API Documentation](./docs/api.md) for details.

## Learning Resources

- [Svelte 5 Documentation](https://svelte.dev/docs/svelte/overview)
- [Axum Documentation](https://docs.rs/axum/)
- [Teacher Logs](./docs/teacher-logs.md) - In-depth explanations of implementation details

## Contributing

1. Read the [Coding Guidelines](./docs/coding-guideline.md)
2. Review the [Architecture](./docs/architecture.md)
3. Make your changes
4. Run linters and type checks
5. Update documentation if needed

## License

See [LICENSE](./LICENSE)

---

*For complete documentation, visit [docs/README.md](./docs/README.md)*


## Intro
Teacher and Documentation Agent. This capstone project expand on the concepts learned in Session 5.

## Motivations

I wanted to tackle a real world problem that software engineers face when coding with AI. As I use AI to write more and more of the implementation on projects i'm often left not truly understanding what the AI wrote. There is a powerful psychological effect to move on to the next feature after quickly checking in local that everything is working. If this process repeats over the course of a project by the end the codebase feels completely foreign and you essentially get the new codebase effect.

- Debugging is basically guessing, you know nothing about the edge cases, tradeoffs, and assumptions
- Blind on security and performance issues. If you don't understand the codebase issues can slip into production
- Growth as a engineer stops. Building engineering skills is built upon struggling through problems, making decisions, and understanding consequences

## The possible solution

A possible solution to not understanding what AI wrote is using more AI pretty ironic but the idea is implementing two agents into the flow.

### Teacher agent

After you prompt the AI to implement something it will explain the diff and instruct you on what it did, why that approach was taken, what alternatives exists, and how it fits into the larger codebase.

This teacher agent focuses on writing explanations like a mentor teaching a student.

On demand slash command /teach


### Documentation agent

This agent runs after the teacher agent. It does the following

1. Takes the teacher agent's output as context
2. Writes the teacher's output to log file with a timestamp and a short summary
3. Decide to update an existing doc, create a new one, or split and reorganize the docs

On demand slash command /document

## The project

For the copilot bootcamp I decided to create a recipe web app. I choose technologies I know nothing about. Svelte as the frontend framework and rust as the backend. I came in without prior research and wanted to see if these agents can teach me as I prompt it to build.

Frontend
Svelete https://svelte.dev/
UI Component Library https://www.shadcn-svelte.com/

Backend
Rust https://rust-lang.org/
Axum https://docs.rs/axum/latest/axum/
Sled db https://docs.rs/sled/latest/sled/

## Explanding on Teacher Agent

You can use the copilot-customization.agent.md and the instructions below to generate your teacher agent

Create a custom Copilot agent for teaching the user about the implementation it has done. Implement this mode as a slash command. /teach

The mode should:
- This teacher agent focuses on writing explanations like a senior engineer teaching a junior engineer
- Explain the diff and instruct you what it did
- Add the appropriate code snippets to the explanation. Add comments to the code snippets when appropriate
- Add other visualization as appropriate, 
  - Diagrams (Mermaid — flowcharts, sequence diagrams, ER diagrams) 
  - Code snippets (syntax highlighted)
  - Tables (comparing options, showing data structures)
  - ASCII art (simple architecture sketches)
- Explain why that approach was taken
- Explain what alternatives exists
- Explain why those alternatives did not work or why they were inferior to the current approach
- Explain how it fits into the larger codebase
- Identify code smells and anti-patterns
- Create a log of what the teacher output and paste it into docs/teacher-logs.md Add metadata to the title
  - title + summary
  - timestamp
  - categorize change for example frontend or backend change use the context of the teacher's lesson to categorize
- When writing to teacher logs the timestamp and title summary should be header 1 "#". The agent should also add a table of content if it doesn't exist yet and update the table of contents for each entry

Available tools: "search", "read", "web", "edit"

Preferred model: Claude Sonnet 4.5 (copilot)

Format as a teacher.agent.md file ready to save in .github/agents/

## Expanding on Documentation Agent

You can use the copilot-customization agent and the instructions below to generate your documentation agent.

Create a custom Copilot agent that reads `docs/teacher-logs.md` and intelligently maintains project documentation. Implement this as a slash command: `/doc`

The mode should:
- Read teaching session logs and extract key architectural decisions, edge cases, and concepts
- Decide whether to update existing docs, create new ones, or split/reorganize documentation
- Maintain documentation standards with proper structure, Mermaid diagrams, tables, and code snippets
- Keep docs navigable by organizing content by topic (not chronology) and adding cross-references
- Support flags for different operations:
  - `--from-logs` — Process new teach-log entries (most common)
  - `--audit` — Scan existing docs for stale/incorrect content
  - `--file <path>` — Update a specific documentation file
  - `--topic <name>` — Consolidate teach-log entries about a topic
  - `--dry-run` — Preview changes without writing
- Follow documentation philosophy:
  - Write for engineers who haven't seen the code before
  - Short, accurate docs beat long, stale ones
  - Preserve the "why" behind decisions from teach-logs
  - Use visualizations only where they clarify

Available tools: "read", "edit", "search", "vscode"

Preferred model: Claude Sonnet 4.5 (copilot)

Format as a documentation.agent.md file ready to save in .github/agents/

## Future Enhancements

- The teacher and documentation agents can be automated on hooks, commit hooks, save hooks, etc
- Hook into PR's and paste the teacher's lesson into the PR readme section itself. Any engineer looking at these PR will have a full explanation of the diff.
- Hook into documentation platforms and auto push documentation to where docs are stored for example confuence