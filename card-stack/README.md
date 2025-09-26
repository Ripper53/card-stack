# Card Stack

**NOTE:** crate name reserved for in-production crate. Not at all near completion. Will be used by `card-game` crate.

A priority and stack system for card games. Most card games like MTG, YuGiOh, and a plethora of other TCGs include a variant of priority and the stack that dictates the rules of what action and when an action can be placed on the stack, and how/when they resolve. This crate includes traits which help implement your own stack for your TCG.

## How to Use
1. Create an `enum` for inciting actions and stackable actions
2. Create a resolver for inciting actions and stackable actions
