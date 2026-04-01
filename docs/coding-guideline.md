# Coding Guidelines

## Overview
This document outlines the coding standards, style conventions, and quality principles. All developers should follow these guidelines to maintain consistency, readability, and maintainability across the codebase.

## General Formatting Rules
File Structure
- Indentation: Use 2 spaces for all indentation (JavaScript, JSON, CSS, Markdown)
- Line Length: Keep lines to a reasonable length, ideally under 100 characters for code
- Trailing Whitespace: Remove all trailing whitespace
- Line Endings: Use LF (Unix-style) line endings
  
## Naming Conventions
- Variables and Functions
- Use camelCase for variables and function names
- Use descriptive names that clearly indicate purpose
- Avoid single-letter variables except in loops or destructuring
- Good: getUserId, calculateTotalPrice, isActive
- Bad: u, calc, x, tmp

## Constants
Use UPPER_SNAKE_CASE for constants
Good: MAX_RETRIES, API_BASE_URL, DEFAULT_TIMEOUT
Bad: maxRetries, apiBaseUrl

## Classes
Use PascalCase for class names
Good: ApiService, TodoRepository
Bad: apiService, todo_repository

## Code Organization
Logical Grouping: Group related code together
Single Responsibility: Each module, component, or function should have a single, well-defined responsibility
Order of Declaration:
Imports at the top
Constants
Utility functions
Main component/class
Helper functions (if any)
Exports at the bottom
