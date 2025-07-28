---
name: RFC (Request for Comments)
about: Propose a major language feature or design change
title: '[RFC] '
labels: ['RFC', 'discussion']
assignees: ''
---

**Summary**
A one paragraph explanation of the feature/change.

**Motivation**
Why are we doing this? What use cases does it support? What is the expected outcome?

**Detailed design**
This is the bulk of the RFC. Explain the design in enough detail for somebody familiar with Ferrum to understand, and for somebody familiar with the implementation to implement.

**Syntax**
```ferrum
// Show the proposed syntax with examples
```

**Semantics**
Explain how this feature works at runtime:
- Memory management implications
- Type system interactions
- Error handling behavior

**Implementation plan**
- [ ] Frontend changes (lexer/parser)
- [ ] AST/IR modifications
- [ ] Backend code generation
- [ ] Runtime support
- [ ] Standard library updates
- [ ] Documentation
- [ ] Tests

**Drawbacks**
Why should we *not* do this?

**Rationale and alternatives**
- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

**Prior art**
Discuss prior art, both the good and the bad, in relation to this proposal. How do other languages handle this?

**Unresolved questions**
- What parts of the design do you expect to resolve through the RFC process?
- What related issues do you consider out of scope but worth noting?

**Future possibilities**
Think about what the natural extension and evolution of your proposal would be and how it would affect the language as a whole.
