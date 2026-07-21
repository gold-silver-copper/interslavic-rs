# Changelog

**Policy — variant order is API.** Multi-byform cells are `" / "`-joined
strings and consumers bless first-variant outputs into their expectations:
reordering variants is a breaking change and must be called out explicitly
in the release notes (fenced by `tests/variant_order.rs`).

## 0.11.0 — 2026-07-21

Follow-up release from the mrzavec zero-pre-inflection conversion: one
confirmed bug, two ergonomics items, one contract fence, one parity-scope
extension. Existing parity numbers are byte-stable; the one intentional
output change is enumerated below.

### Fixed

- **`l_participle` diverged from the paradigm on d/t-stem `-sti` verbs**
  (`ukrasti` gave `ukrasla`; the parity-verified paradigm path gives
  `ukradla` — same for `krasti`, `pasti`, `sěsti`, `vesti`, `gnesti`).
  The facade never did the dictionary lookup, so it had no present-stem
  hint, and the hint is what recovers the dental stem. The stem
  derivation is now genuinely shared (`stem_context` in core), the
  facade resolves the hint like `verb()` does, and a self-consistency
  sweep locks the invariant: all 3,859 embedded dictionary verb lemmas ×
  six gender/number cells (23,154 cells) agree with the l-participle
  inside `verb_forms`' perfect cells — 0 divergent.
- `active_participle` leaked the internal `ĵ` marker into declined forms
  for `-aj` stems (`dělaĵųća` → now `dělajųća`); fixed by the surface
  cleaning below.

### Changed

- **`verb_forms`/`try_verb_forms` cells are now surface-ready**: the
  builders' internal intervocalic-j marker `ĵ` is flattened to `j`.
  Exactly two fields ever carried it (dictionary-wide sweep): the
  `imperative` (5,433 cells, e.g. `počivaĵ` → `počivaj`) and `prap`
  (1,597 cells, e.g. `počivaĵųćí…` → `počivajųćí…`). Variant structure,
  gender-form conventions, and citation accents are unchanged. The
  previous marker-carrying cells remain available via the new
  `verb_forms_raw()`/`try_verb_forms_raw()`, and
  `verb_forms_with_metadata` stays raw — it is the parity/integration
  entry point, and the JS reference itself emits the marker (which is
  why the parity numbers are unaffected). slovowiki's exporter (218
  tests) verified green against the new default: its `clean_cell`
  already flattens via `cells::variants`, so its output is unchanged.

### Added

- **`perfect_parts()`** (facade) / `verb::perfect_parts_with_hint`
  (core): the perfect tense as data — `PerfectParts { auxiliary:
  Option<String>, participle: String }`. The auxiliary is
  `Some("jesm"/"jesi"/"jesmo"/"jeste")` in the 1st/2nd persons and
  `None` in the 3rd (the standard normally omits it; the emphatic
  `je`/`jest`/`sųt` is the caller's to add). The participle comes from
  the same shared stem context as the compound tenses, so the accessor
  can never disagree with the paradigm.
- Parity harness: an **out-of-vocabulary verb scope** — a fixed sample
  of plausible non-dictionary lemmas per class (`-jati`, `-nųti`,
  `-ovati`, prefixed `-sti`), membership re-checked each run — tracks
  the hintless rule-engine path: 12 lemmas / 612 forms, 100% compatible.
- Variant-order fence: `tests/variant_order.rs` pins byform order for a
  representative cell per declension class, and the changelog gains the
  policy line above.

### Notes

- The `stajati` question is settled against the parity reference: the JS
  implementation does **not** contract OOV `-jati` presents (`stajaje,
  staja`; even `dajati` gives `dajaje, daja`, never `daje`), so this
  crate keeps `stajaje`. *(Correction, post-release: this note originally
  claimed slovowiki's form index carried `staje` as `prez.3jd.` of
  `stajati` and diverged from the parity standard. That was a folded-key
  homograph misreading: slovowiki's `staje` record is the noun `staja`
  — gen.sg/nom.pl/acc.pl — and it has no `stajati` verb record at all,
  so it never took a position on the contraction. When attributing a
  slovowiki form-API hit, read the record's `lemmas`/analyses fields
  before crediting it to a lemma.)*

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
