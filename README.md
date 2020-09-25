# crank

A theorem checker for the formal system P that describes propositional logic.

## Language

The alphabet of propositional logic includes:
1. propositional variables: a, b, c, ..., z
2. connectives: ¬, ∨. derived connectives: ->, <->, ∧, 
3. parentheses: (, )

The set of formulas FOR of propositional logic is defined inductively and is the smallest set that satisfies the following rules:
1. Every propositional variable p is a formula.
2. If p and q are formulas, then ¬p, (p ∨ q), (p -> q), (p <-> q) and (p ∧ q) are formulas.

```

```
