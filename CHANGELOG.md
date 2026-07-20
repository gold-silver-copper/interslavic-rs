# Changelog

## 0.10.0 — 2026-07-20

Additive API release closing the gaps found by the mrzavec runtime-inflection
audit. Existing noun/adjective/verb behavior is byte-stable (verified by the
sonic16x parity harness at the same commit and dictionary snapshot).

### Added

- **Personal and reflexive pronouns** (`interslavic_core::pronoun`, new):
  complete closed-class paradigms for `ja`/`ty`/`on`/`ona`/`ono`/`my`/`vy`/
  `oni`/`one` and the reflexive `sebe`, following the `@interslavic/utils`
  3.4.0 `declensionPronoun` reference (documented discrepancies with the
  steen legacy table in the module docs). The three form series are explicit
  via the new `PronounStyle` enum: `Full`, `Clitic` (exactly the attested
  set: `mę`/`mi`, `tę`/`ti`, `go`/`mu`, `sę`/`si`), and `AfterPreposition`
  (the 3rd-person prepositional n- forms: `njego`, `njemu`, `njim`, `njej`,
  `njih`, …). Facade: `personal_pronoun()`, `reflexive_pronoun()`; bare
  personal lemmas also route through `pronoun()` (`pronoun("ty", Case::Gen,
  …)` → `tebe`).
- **`l_participle()`** on the facade: the gender/number-marked l-participle
  (`pisal`/`pisala`, `šėl`/`šla`, `ubil`/`ubila`) for fixed-gender past
  subjects.
- **`passive_participle()` / `active_participle()`** on the facade: the
  `pfpp`/`prap` participles declined as adjectives (`"komnata jest
  osvětljena"`), with the `Option` handling and lemma normalization in one
  place. Round-trip tested across the `-ny`, `-ty`, and iotated `-jeny`
  shapes.
- Parity harness: a pronoun scope comparing every personal/reflexive cell
  (198 forms across the three series, including cell nonexistence) against
  `declensionPronoun` — 100.0000% exact; reported in the README accuracy
  table alongside nouns/adjectives/verbs.
- Doc-tests pinning downstream-assumed behavior: `svoj` via the moj-class
  path, `comparative()` pairs for `bystry`/`blizky`/`dobry`/`silny`/`slaby`,
  populated imperative/gerund for the `-ati`/`-iti`/`-ovati` classes, and
  oblique forms of `dva`/`tri`/`pęť`.

### Fixed

- `interslavic_core::verb::l_participle` is rebuilt on the same stem
  derivation the paradigm's compound tenses use, fixing three bugs in the
  old standalone implementation: the `idti` special case had singular and
  plural swapped (every singular returned `šli`), prefixed non-regular
  verbs dropped their prefix (`pojesti` → `jedl` instead of `pojedl`), and
  the `žegti`/`idti`-stem alternations were missing (`žegla` instead of
  `žgla`).
- `decline_pronoun("oni", …)` previously fell through to the adjectival
  shape branch and declined `oni` as an adjective; it now returns the
  personal-pronoun forms (`jih`, `jim`, …).

### Notes

- Slovowiki cross-check of the generated pronoun forms: all full forms
  analyze as known except the instrumentals (`mnojų`, `nami`, `vami`,
  `jimi`, `jejų`, `tobojų`, `sobojų`), which are missing from slovowiki's
  own form index (its dictionary rows list them); all clitics known; the
  n- forms are absent from its index as expected.
- The noun parity count moved from 8 to 17 mismatches between measurements
  due to edits in the live upstream dictionary sheet (soft-o loans like
  *adadžo*, substantivized adjectives); reproduced identically on 0.9.0 at
  the same sonic commit — upstream data drift, not an engine change. The
  harness noun threshold is relaxed one notch (0.9999 → 0.9998).

## 0.9.0 — 2026-07-19

- Root API refactor; see git history.
