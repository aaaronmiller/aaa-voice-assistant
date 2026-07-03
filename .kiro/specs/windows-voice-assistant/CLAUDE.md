# Project Workflow Instructions

## Core Principles

1. **Use tasks.md as a gated todo list**: All implementation work follows the tasks.md file in `.kiro/specs/windows-voice-assistant/`. Each task must be completed before moving to the next.

2. **Quality code, not overcomplex**: Write simple, clear, maintainable code. Avoid premature abstraction, unnecessary complexity, or "clever" solutions. Favor straightforward implementations that solve the problem.

3. **No placeholders or mock code**: Never use `TODO`, `placeholder`, `stub`, or mock implementations. Every function must be fully implemented with real logic. If something can't be implemented yet, mark the task as pending and move to a different task, but DO NOT write placeholder code.

4. **Start at last completed task**: When beginning work, audit the codebase to determine which tasks are truly complete (not just marked with [x]). Begin implementation at the last actually-completed task and continue sequentially.

5. **Automatic phase progression**: Upon completion of a phase checkpoint, DO NOT wait for user instructions or approval. Immediately continue to the next phase's first task.

6. **Continuous work until done**: Keep implementing tasks until all tasks in tasks.md are marked complete and verified working. Do not stop unless blocked by a genuine technical obstacle that requires user input.

## Execution Protocol

### When Starting Work

1. Read `.kiro/specs/windows-voice-assistant/tasks.md`
2. Audit the current codebase to verify actual completion status
3. Create a task list from the uncompleted tasks
4. Begin implementing the first uncompleted task
5. Mark tasks as complete only when fully implemented AND tested

### Implementation Standards

- Write real, working code that integrates with existing components
- Follow existing code patterns and architecture
- Add appropriate error handling
- Write tests for new code (unit/property tests as specified)
- Commit progress frequently with descriptive messages
- Verify functionality before marking tasks complete

### Quality Checklist

Before marking any task complete:
- [ ] Code compiles without errors
- [ ] All tests pass
- [ ] No placeholder/TODO comments
- [ ] No mock/fake implementations
- [ ] Integrates properly with existing system
- [ ] Follows project architecture and patterns
- [ ] Includes appropriate error handling

## Important Instructions

- **DO NOT** ask user for permission to proceed to next phase
- **DO NOT** wait for user confirmation after checkpoints
- **DO NOT** implement placeholder code
- **DO** audit code carefully to determine real completion status
- **DO** write production-quality code from the start
- **DO** follow property-based testing strategy outlined in design
- **DO** maintain modular, extensible architecture

This workflow ensures continuous, autonomous progress toward a complete, working implementation.
