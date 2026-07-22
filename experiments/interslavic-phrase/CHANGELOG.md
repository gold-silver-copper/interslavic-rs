# interslavic-phrase changelog

## 0.2.0 — 2026-07-22

Stages 1–4 of `phrase-improvement.md`. Breaking AST revision (0.1.x
warned it would move); the 0.1.0 builder call-sites survive, the struct
layout does not:

- `Clause.vp` → `Clause.core` (`ClauseCore::Verbal(Coordination<VerbPhrase>)`
  | `ClauseCore::Copular`); `Clause` gained `mood`, `voice`, `topic`,
  `focus`. `Force` gained `Imperative(Addressee)`. `Nominal::Pron` gained
  `clitic`; `Nominal` gained `Name` and `Coord`. `NounPhrase` gained
  `relative` and `entity`. `VerbPhrase` gained `adverbs`.

### Added

- **Copular clauses** (`Predicate::{Nominal, Adjectival, Participial}`),
  nominative predicate default with `:pred-case ins` (policy; steen
  silent), and **passive voice** (`byti` + pfpp, agent `od`+Gen or Ins —
  steen-sourced).
- **Imperative** (three addressees, subject-dropped, `!`) and
  **conditional** (person-marked auxiliary from the facade's own
  conditional row).
- **Typed verb government**: object case from `verb_info().governs`
  ("Ja dękujų tobě", "Krålj vladaje zemjejų"); "intransitive" +
  government annotation licenses the oblique object. Warnings channel
  (`realize_checked` → `Realized { text, warnings }`):
  `GovernsConflict`, `PerfectivePresent`, `AmbiguousOrder`.
- **Relative clauses**: `ktory` agreeing with the head, case from the
  gap (subject / object / PP object); `iže` declared-unsupported (no
  facade paradigm). Comma-delimited (policy).
- **Coordination**: NPs (plural agreement, masculine for mixed gender —
  policy) and VPs ("kupil i pročital knigų").
- **Clitics**: pronominal clitic objects (`go`/`mu`/`mi`),
  `CliticStyle::{Postverbal, SecondPosition}` with the Franks & King
  cluster order (li > dat > acc > sę).
- **Information structure**: `:topic` (fronts) and `:focus` (final;
  fronted with `li` in li-questions); syncretism clarity warning
  (steen's own caveat).
- **Discourse module**: entity-tagged referring expressions
  (pronominalize after first mention, full NP after interference),
  aggregation to VP coordination (tree-to-tree), sentence-initial
  connectives (potom/ale/zato/tomu/i).
- Adverbs (preverbal — policy), declinable proper names (`:indecl`
  escape).

All 0.1.0 goldens byte-stable; 16 golden tests; slovowiki gate: 112
tokens, 0 unknown, 0 agreement errors.

### Post-review root-cause revision (same release)

The PR #34 code review confirmed ten defects traceable to five root
mechanisms; all five were replaced, not patched (the review probes are
pinned in `tests/review_regressions.rs`):

- **Typed linearization**: labeled constituents with Word/Clitic/Punct
  tokens carried until one final join — the string-search clitic
  placement, word-counting second position, and marker-character comma
  protocol no longer exist. 2P clitics respect constituent boundaries
  ("Toj dobry krålj go vidi.").
- **Per-verb-complex clitic clusters** (Franks & King: conjuncts are
  separate clitic domains): coordinated VPs keep their clitics
  ("Krålj kupil knigų i viděl go."); clitics inside relatives attach
  there; unclitickable contexts (conjuncts, PP objects, topic/focus
  marking) force full forms as a cited rule — a focused clitic object
  now yields "Jego li vidi krålj?".
- **One verb-complex builder**: imperative/conditional/passive/copular
  all route through it; incoherent Force/Mood/Voice combinations error
  (passive imperative, conditional imperative) instead of silently
  dropping a member; relatives render through the same VP machinery
  (valence and adverbs now apply inside them).
- **Derived order-markedness + pure syncretism probe**: the
  AmbiguousOrder warning fires only on genuinely inverted subject/object
  order, once; no probe re-renders through the warning context.
- **Discourse through the single pipeline**: `realize_with_lead_in`
  carries connectives; `narrate` (now by-value) assembles no strings and
  honors the caller's options in every sentence.

Facade 0.14.0 gained `conditional_parts` (the conditional as data, from
the same auxiliary table the paradigm row uses) — the phrase crate's
hand-copied slot map is gone. Output changes vs the pre-review branch:
focused/topicalized clitic pronouns now render full forms; narrate
applies sentence casing/punctuation uniformly.

## 0.1.0 — 2026-07-22

Initial experiment: declaratives, questions (li/či/intonation),
quantified phrases, reflexives, negation, S-expression surface.
