# copilot-bootcamp-capstone

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
3. Scans /docs for existing files and their structure
4. Decide to update an existing doc, create a new one, or split and reorganize the docs

On demand slash command /document

## The project

For the copilot bootcamp I decided to create a recipe web app. I choose technologies I know nothing about. Svelte as the frontend framework and rust as the backend. I came in without prior research and wanted to see if these agents can teach me as I prompt it to build.

## Explanding on Teacher Agent

Create a custom Copilot agent for teaching the user about the implementation it has done. Implement this mode as a slash command. /teach

The mode should:
- This teacher agent focuses on writing explanations like a senior engineer teaching a junior engineer
- Explain the diff and instruct you what it did
- Explain why that approach was taken
- Explain what alternatives exists
- Explain why those alternatives did not work or why they were inferior to the current approach
- Explain how it fits into the larger codebase
- Identify code smells and anti-patterns
- Create a log of what the teacher output and paste it into docs/teacher-logs.md Add a timestamp and summary

Available tools: "search", "read", "web", "edit"

Preferred model: Claude Sonnet 4.5 (copilot)

Format as a teacher.agent.md file ready to save in .github/agents/

## Explanding on Documentation Agent