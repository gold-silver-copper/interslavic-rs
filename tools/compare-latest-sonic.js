const fs = require('fs');
const path = require('path');
const { execFileSync, spawnSync } = require('child_process');

const ROOT = path.resolve(__dirname, '..');
const OUT_DIR = path.join(ROOT, 'target/infl-comparison');
const SONIC_REPO = path.join(ROOT, '.external/sonic16x-interslavic');
updateSonicCheckout();
const UTILS = require(path.join(SONIC_REPO, 'node_modules/@interslavic/utils/dist/index.js'));
const { declensionNoun, declensionAdjective, conjugationVerb, parsePos } = UTILS;

const BASIC_URL = 'https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/export?format=tsv&gid=1987833874';
const CASES = ['nom', 'acc', 'gen', 'loc', 'dat', 'ins'];
const NUMBERS = ['sg', 'pl'];
const PERSONS = [['1', 'sg'], ['2', 'sg'], ['3', 'sg'], ['1', 'pl'], ['2', 'pl'], ['3', 'pl']];
const MIN_COMPATIBLE_RATES = {
  total: 0.99,
  noun: 0.9999,
  adjective: 0.999,
  verb: 0.95,
};

function updateSonicCheckout() {
  execFileSync('git', ['fetch', 'origin'], { cwd: SONIC_REPO, stdio: 'inherit' });
  execFileSync('git', ['checkout', 'master'], { cwd: SONIC_REPO, stdio: 'inherit' });
  execFileSync('git', ['reset', '--hard', 'origin/master'], { cwd: SONIC_REPO, stdio: 'inherit' });
  if (!fs.existsSync(path.join(SONIC_REPO, 'node_modules/@interslavic/utils/dist/index.js'))) {
    execFileSync('npm', ['ci'], { cwd: SONIC_REPO, stdio: 'inherit' });
  }
}

function shell(cwd, cmd, args) {
  return execFileSync(cmd, args, { cwd, encoding: 'utf8' }).trim();
}

function norm(value) {
  if (value == null) return null;
  return String(value)
    .trim()
    .replace(/!/g, '')
    .replace(/\s*\/\s*/g, ' / ')
    .replace(/\s*,\s*/g, ', ')
    .replace(/\s+/g, ' ');
}

function splitWords(value) {
  return String(value || '')
    .replace(/!/g, '')
    .split(',')
    .map((w) => w.trim())
    .filter(Boolean);
}


function isCoreLemma(word) {
  return word && !/[\s()\/]/.test(word);
}

function parseTsv(text) {
  const lines = text.replace(/\r/g, '').split('\n').filter(Boolean);
  const header = lines[0].split('\t');
  return lines.slice(1).map((line) => {
    const cols = line.split('\t');
    const obj = {};
    header.forEach((name, index) => { obj[name] = cols[index] || ''; });
    return obj;
  });
}

async function loadDictionary() {
  const res = await fetch(BASIC_URL);
  if (!res.ok) throw new Error(`Failed to fetch dictionary TSV: ${res.status} ${res.statusText}`);
  const text = await res.text();
  fs.writeFileSync(path.join(OUT_DIR, 'sonic16x-basic.tsv'), text);
  return parseTsv(text);
}

function nounForms(result) {
  if (!result) return null;
  const forms = {};
  for (const c of CASES) {
    forms[`${c}_sg`] = norm(result[c]?.[0]);
    forms[`${c}_pl`] = norm(result[c]?.[1]);
  }
  return forms;
}

function splitSlash(value) {
  return String(value || '').split('/').map((item) => norm(item)).filter(Boolean);
}

function adjectiveFormFor(result, genderKey, c, n) {
  const section = n === 'sg' ? result.singular : result.plural;
  const values = section?.[c];
  if (!values) return null;
  if (n === 'sg') {
    if (c === 'nom') {
      if (genderKey.startsWith('masc')) return norm(values[0]);
      if (genderKey === 'neut') return norm(values[1]);
      return norm(values[2]);
    }
    if (c === 'acc') {
      if (genderKey === 'masc_anim') return splitSlash(values[0])[0] || null;
      if (genderKey === 'masc_inan') return splitSlash(values[0])[1] || splitSlash(values[0])[0] || null;
      if (genderKey === 'neut') return norm(values[1]);
      return norm(values[2]);
    }
    if (genderKey === 'fem') return norm(values[1]);
    return norm(values[0]);
  }
  if (c === 'nom' || c === 'acc') {
    if (genderKey === 'masc_anim') return splitSlash(values[0])[0] || null;
    if (genderKey === 'masc_inan') return splitSlash(values[0])[1] || splitSlash(values[0])[0] || null;
    return norm(values[1]);
  }
  return norm(values[0]);
}

function adjectiveForms(result) {
  if (!result) return null;
  const forms = {};
  for (const genderKey of ['masc_anim', 'masc_inan', 'fem', 'neut']) {
    for (const n of NUMBERS) {
      for (const c of CASES) {
        forms[`${genderKey}_${c}_${n}`] = adjectiveFormFor(result, genderKey, c, n);
      }
    }
  }
  return forms;
}

function cleanOptional(value) {
  return value == null ? null : norm(value);
}

function splitImperative(value) {
  return String(value || '').split(',').map((item) => norm(item)).filter(Boolean);
}

function verbForms(result) {
  if (!result || !Array.isArray(result.present)) return null;
  const forms = { infinitive: norm(result.infinitive) };
  PERSONS.forEach(([p, n], i) => { forms[`present_${p}_${n}`] = norm(result.present[i]); });
  PERSONS.forEach(([p, n], i) => { forms[`imperfect_${p}_${n}`] = norm(result.imperfect?.[i]); });
  PERSONS.forEach(([p, n], i) => { forms[`future_${p}_${n}`] = norm(result.future?.[i]); });
  ['1_sg', '2_sg', '3_sg_m', '3_sg_f', '3_sg_n', '1_pl', '2_pl', '3_pl'].forEach((key, i) => {
    forms[`perfect_${key}`] = norm(result.perfect?.[i]);
    forms[`pluperfect_${key}`] = norm(result.pluperfect?.[i]);
    forms[`conditional_${key}`] = norm(result.conditional?.[i]);
  });
  const imperative = splitImperative(result.imperative);
  forms.imperative_2_sg = imperative[0] || null;
  forms.imperative_1_pl = imperative[1] || null;
  forms.imperative_2_pl = imperative[2] || null;
  forms.prap = cleanOptional(result.prap);
  forms.prpp = cleanOptional(result.prpp);
  forms.pfap = cleanOptional(result.pfap);
  forms.pfpp = cleanOptional(result.pfpp);
  forms.gerund = cleanOptional(result.gerund);
  return forms;
}

function buildSonicReferences(rows) {
  const refs = [];
  const skipped = [];
  const skippedPhrases = [];
  for (const row of rows) {
    const details = row.partOfSpeech || '';
    let pos;
    try { pos = parsePos(details); } catch (e) { skipped.push({ id: row.id, isv: row.isv, details, reason: `parsePos: ${e.message}` }); continue; }
    if (!pos || !['noun', 'adjective', 'verb'].includes(pos.name)) continue;
    for (const word of splitWords(row.isv)) {
      if (!isCoreLemma(word)) {
        skippedPhrases.push({ id: row.id, word, kind: pos.name, details, reason: 'phrase strings are not part of the core typed lemma API' });
        continue;
      }
      try {
        if (pos.name === 'noun') {
          const genders = pos.gender === 'masculineOrFeminine' ? ['masculine', 'feminine'] : [pos.gender];
          for (const gender of genders) {
            const result = declensionNoun(word, row.addition || '', gender, !!pos.animate, !!pos.plural, !!pos.singular, !!pos.indeclinable);
            const forms = nounForms(result);
            if (!forms) { skipped.push({ id: row.id, word, kind: 'noun', details, reason: 'sonic returned null' }); continue; }
            {
              const refKey = `${row.id}|noun|${word}|${gender}|${!!pos.animate}|${!!pos.plural}|${!!pos.singular}|${!!pos.indeclinable}`;
              refs.push({ refKey, input: `${refKey}\tnoun\t${word}\t${gender}\t${!!pos.animate}\t${!!pos.plural}\t${!!pos.singular}\t${!!pos.indeclinable}`, id: row.id, kind: 'noun', word, details, addition: row.addition || '', meta: { gender, animate: !!pos.animate, plural: !!pos.plural, singular: !!pos.singular, indeclinable: !!pos.indeclinable }, forms });
            }
          }
        } else if (pos.name === 'adjective') {
          const result = declensionAdjective(word, '', details);
          const forms = adjectiveForms(result);
          if (!forms) { skipped.push({ id: row.id, word, kind: 'adjective', details, reason: 'sonic returned null' }); continue; }
          {
            const refKey = `${row.id}|adjective|${word}`;
            refs.push({ refKey, input: `${refKey}\tadjective\t${word}`, id: row.id, kind: 'adjective', word, details, addition: row.addition || '', meta: {}, forms });
          }
        } else if (pos.name === 'verb') {
          const result = conjugationVerb(word, row.addition || '', details);
          const forms = verbForms(result);
          if (!forms) { skipped.push({ id: row.id, word, kind: 'verb', details, reason: 'sonic returned null' }); continue; }
          {
            const refKey = `${row.id}|verb|${word}`;
            refs.push({ refKey, input: `${refKey}\tverb\t${word}\t${row.addition || ''}\t${!!pos.transitive}\t${!!pos.imperfective}`, id: row.id, kind: 'verb', word, details, addition: row.addition || '', meta: { reflexive: !!pos.reflexive, transitive: !!pos.transitive, perfective: !!pos.perfective, imperfective: !!pos.imperfective }, forms, allSonicVerbForms: result });
          }
        }
      } catch (e) {
        skipped.push({ id: row.id, word, kind: pos.name, details, reason: e.message });
      }
    }
  }
  return { refs, skipped, skippedPhrases };
}

function compileRustHelper() {
  execFileSync('cargo', ['build'], { cwd: ROOT, stdio: 'inherit' });
  const deps = path.join(ROOT, 'target/debug/deps');
  const candidates = fs.readdirSync(deps)
    .filter((name) => /^libinterslavic(?:-[^-]+)?\.rlib$/.test(name))
    .map((name) => ({ name, mtime: fs.statSync(path.join(deps, name)).mtimeMs }))
    .sort((a, b) => b.mtime - a.mtime);
  if (!candidates.length) throw new Error('Could not find libinterslavic rlib');
  const source = path.join(OUT_DIR, 'rust_inflect_compare.rs');
  const bin = path.join(OUT_DIR, 'rust_inflect_compare');
  fs.writeFileSync(source, String.raw`
use interslavic::*;
use std::io::{self, BufRead};
use std::panic;

fn clean(s: String) -> String { s.replace('\t', " ").replace('\n', " ") }
fn emit(ref_key: &str, key: &str, value: String) { println!("{}\t{}\t{}", ref_key, key, clean(value)); }
fn parse_bool(s: &str) -> bool { matches!(s, "true" | "True" | "1") }
fn gender(s: &str) -> NounGender { match s { "feminine" => NounGender::Feminine, "neuter" => NounGender::Neuter, _ => NounGender::Masculine } }
fn anim(b: bool) -> Animacy { if b { Animacy::Animate } else { Animacy::Inanimate } }
fn noun_form(word: &str, case: Case, number: Number, gender: NounGender, animate: bool) -> String {
    panic::catch_unwind(|| ISV::noun_with(word, case, number, gender, anim(animate))).unwrap_or_else(|_| "<PANIC>".to_string())
}
fn adj_form(word: &str, case: Case, number: Number, gender: Gender, animate: bool) -> String {
    panic::catch_unwind(|| ISV::adj(word, case, number, gender, anim(animate))).unwrap_or_else(|_| "<PANIC>".to_string())
}
fn verb_forms(word: &str, present_hint: &str, transitive: bool, imperfective: bool) -> Vec<(String, String)> {
    panic::catch_unwind(|| {
        let p = ISV::verb_forms_with_metadata(word, present_hint, transitive, imperfective);
        let mut out = Vec::new();
        out.push(("infinitive".to_string(), p.infinitive));
        for (i, key) in ["present_1_sg", "present_2_sg", "present_3_sg", "present_1_pl", "present_2_pl", "present_3_pl"].iter().enumerate() { out.push((key.to_string(), p.present[i].clone())); }
        for (i, key) in ["imperfect_1_sg", "imperfect_2_sg", "imperfect_3_sg", "imperfect_1_pl", "imperfect_2_pl", "imperfect_3_pl"].iter().enumerate() { out.push((key.to_string(), p.imperfect[i].clone())); }
        for (i, key) in ["future_1_sg", "future_2_sg", "future_3_sg", "future_1_pl", "future_2_pl", "future_3_pl"].iter().enumerate() { out.push((key.to_string(), p.future[i].clone())); }
        for (i, key) in ["1_sg", "2_sg", "3_sg_m", "3_sg_f", "3_sg_n", "1_pl", "2_pl", "3_pl"].iter().enumerate() {
            out.push((format!("perfect_{}", key), p.perfect[i].clone()));
            out.push((format!("pluperfect_{}", key), p.pluperfect[i].clone()));
            out.push((format!("conditional_{}", key), p.conditional[i].clone()));
        }
        for (i, key) in ["imperative_2_sg", "imperative_1_pl", "imperative_2_pl"].iter().enumerate() { out.push((key.to_string(), p.imperative[i].clone())); }
        if let Some(v) = p.prap { out.push(("prap".to_string(), v)); }
        if let Some(v) = p.prpp { out.push(("prpp".to_string(), v)); }
        out.push(("pfap".to_string(), p.pfap));
        if let Some(v) = p.pfpp { out.push(("pfpp".to_string(), v)); }
        out.push(("gerund".to_string(), p.gerund));
        out
    }).unwrap_or_else(|_| vec![("<PANIC>".to_string(), "<PANIC>".to_string())])
}
fn main() {
    let cases: [(&str, Case); 6] = [("nom", Case::Nom), ("acc", Case::Acc), ("gen", Case::Gen), ("loc", Case::Loc), ("dat", Case::Dat), ("ins", Case::Ins)];
    let numbers: [(&str, Number); 2] = [("sg", Number::Singular), ("pl", Number::Plural)];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('\t').collect();
        let ref_key = parts[0];
        let kind = parts[1];
        let word = parts[2];
        if kind == "noun" {
            let g = gender(parts[3]);
            let animate = parse_bool(parts[4]);
            for (n_key, number) in numbers {
                for (c_key, case) in cases {
                    emit(ref_key, &format!("{}_{}", c_key, n_key), noun_form(word, case, number, g, animate));
                }
            }
        } else if kind == "adjective" {
            for (gender_key, g, animate) in [("masc_anim", Gender::Masculine, true), ("masc_inan", Gender::Masculine, false), ("fem", Gender::Feminine, false), ("neut", Gender::Neuter, false)] {
                for (n_key, number) in numbers {
                    for (c_key, case) in cases {
                        emit(ref_key, &format!("{}_{}_{}", gender_key, c_key, n_key), adj_form(word, case, number, g, animate));
                    }
                }
            }
        } else if kind == "verb" {
            let present_hint = parts.get(3).copied().unwrap_or("");
            let transitive = parts.get(4).copied().map(parse_bool).unwrap_or(true);
            let imperfective = parts.get(5).copied().map(parse_bool).unwrap_or(true);
            for (key, value) in verb_forms(word, present_hint, transitive, imperfective) {
                emit(ref_key, &key, value);
            }
        }
    }
}
`);
  execFileSync('rustc', ['--edition=2021', source, '--extern', `interslavic=${path.join(deps, candidates[0].name)}`, '-L', `dependency=${deps}`, '-o', bin], { cwd: ROOT, stdio: 'inherit' });
  return bin;
}

function runRust(bin, refs) {
  const inputs = refs.map((r) => r.input).join('\n') + '\n';
  const child = spawnSync(bin, { input: inputs, encoding: 'utf8', maxBuffer: 1024 * 1024 * 300 });
  if (child.status !== 0) throw new Error(`Rust helper failed: ${child.stderr}`);
  const actual = new Map();
  for (const line of child.stdout.split('\n')) {
    if (!line) continue;
    const [refKey, key, ...rest] = line.split('\t');
    if (!actual.has(refKey)) actual.set(refKey, {});
    actual.get(refKey)[key] = norm(rest.join('\t'));
  }
  return actual;
}

function alternatives(expected) {
  const s = norm(expected);
  if (!s) return [];
  const set = new Set([s]);
  for (const part of s.split(/\s*,\s*|\s*\/\s*/)) if (part) set.add(part);
  const paren = s.match(/^(.+?)\s*\((.+?)\)$/);
  if (paren) { set.add(norm(paren[1])); set.add(norm(paren[2])); }
  return [...set];
}

function compare(refs, actual) {
  const summary = { referenceParadigms: refs.length, comparedForms: 0, exactMatches: 0, compatibleMatches: 0, mismatches: 0, byKind: {} };
  const mismatches = [];
  for (const ref of refs) {
    if (!summary.byKind[ref.kind]) summary.byKind[ref.kind] = { paradigms: 0, comparedForms: 0, exactMatches: 0, compatibleMatches: 0, mismatches: 0 };
    summary.byKind[ref.kind].paradigms++;
    const gotForms = actual.get(ref.refKey) || {};
    for (const [key, expected] of Object.entries(ref.forms)) {
      if (!expected) continue;
      const got = gotForms[key] ?? null;
      const exact = got === expected;
      const compatible = exact || alternatives(expected).includes(got);
      summary.comparedForms++;
      summary.byKind[ref.kind].comparedForms++;
      if (exact) { summary.exactMatches++; summary.byKind[ref.kind].exactMatches++; }
      if (compatible) { summary.compatibleMatches++; summary.byKind[ref.kind].compatibleMatches++; }
      if (!compatible) {
        summary.mismatches++;
        summary.byKind[ref.kind].mismatches++;
        mismatches.push({ id: ref.id, kind: ref.kind, word: ref.word, details: ref.details, addition: ref.addition, meta: ref.meta, form: key, expected, actual: got });
      }
    }
  }
  summary.exactRate = summary.comparedForms ? summary.exactMatches / summary.comparedForms : 0;
  summary.compatibleRate = summary.comparedForms ? summary.compatibleMatches / summary.comparedForms : 0;
  for (const item of Object.values(summary.byKind)) {
    item.exactRate = item.comparedForms ? item.exactMatches / item.comparedForms : 0;
    item.compatibleRate = item.comparedForms ? item.compatibleMatches / item.comparedForms : 0;
  }
  return { summary, mismatches };
}

function thresholdFailures(summary) {
  const failures = [];
  if (summary.compatibleRate < MIN_COMPATIBLE_RATES.total) {
    failures.push(`total compatible rate ${summary.compatibleRate} < ${MIN_COMPATIBLE_RATES.total}`);
  }
  for (const kind of ['noun', 'adjective', 'verb']) {
    const item = summary.byKind[kind];
    if (item && item.compatibleRate < MIN_COMPATIBLE_RATES[kind]) {
      failures.push(`${kind} compatible rate ${item.compatibleRate} < ${MIN_COMPATIBLE_RATES[kind]}`);
    }
  }
  return failures;
}

function writeMarkdownReport(report) {
  const s = report.summary;
  const kindRows = Object.entries(s.byKind).map(([kind, item]) =>
    `| ${kind} | ${item.paradigms} | ${item.comparedForms} | ${(item.compatibleRate * 100).toFixed(4)}% | ${item.mismatches} |`,
  ).join('\n');
  const md = `# sonic16x/interslavic comparison\n\nGenerated: ${report.generatedAt}\n\n- Sonic commit: \`${report.sonic.commit}\` (${report.sonic.branch})\n- Sonic utility package: \`${report.sonic.utilsPackage}@${report.sonic.utilsVersion}\`\n- Dictionary rows: ${report.dictionaryRows}\n- Core comparable paradigms: ${s.referenceParadigms}\n- Skipped phrase rows: ${report.skippedPhraseRows}\n- Skipped Sonic-null/reference rows: ${report.skippedReferenceParadigms}\n\n## Core comparable accuracy\n\n- Forms compared: **${s.comparedForms}**\n- Compatible matches: **${s.compatibleMatches}**\n- Compatible accuracy: **${(s.compatibleRate * 100).toFixed(4)}%**\n- Mismatches: **${s.mismatches}**\n\n| Kind | Paradigms | Forms | Compatible accuracy | Mismatches |\n|---|---:|---:|---:|---:|\n${kindRows}\n\nPhrase strings from the dictionary are reported separately because the Rust core API accepts typed lemmas/metadata, not arbitrary phrase strings.\n`;
  fs.writeFileSync(path.join(OUT_DIR, 'README.md'), md);
}

(async () => {
  fs.mkdirSync(OUT_DIR, { recursive: true });
  const sonicCommit = shell(SONIC_REPO, 'git', ['rev-parse', 'HEAD']);
  const sonicBranch = shell(SONIC_REPO, 'git', ['branch', '--show-current']);
  const sonicUtilsVersion = require(path.join(SONIC_REPO, 'node_modules/@interslavic/utils/package.json')).version;
  const rustCommit = shell(ROOT, 'git', ['rev-parse', 'HEAD']);
  const rustStatus = shell(ROOT, 'git', ['status', '--short']);
  const rows = await loadDictionary();
  const { refs, skipped, skippedPhrases } = buildSonicReferences(rows);
  fs.writeFileSync(path.join(OUT_DIR, 'sonic-reference-forms.json'), JSON.stringify(refs, null, 2));
  fs.writeFileSync(path.join(OUT_DIR, 'skipped-sonic-reference.json'), JSON.stringify(skipped, null, 2));
  fs.writeFileSync(path.join(OUT_DIR, 'skipped-phrases.json'), JSON.stringify(skippedPhrases, null, 2));
  const bin = compileRustHelper();
  const actual = runRust(bin, refs);
  fs.writeFileSync(path.join(OUT_DIR, 'rust-forms.json'), JSON.stringify(Object.fromEntries(actual.entries()), null, 2));
  const { summary, mismatches } = compare(refs, actual);
  fs.writeFileSync(path.join(OUT_DIR, 'mismatches.json'), JSON.stringify(mismatches, null, 2));
  const csv = ['id,kind,word,details,addition,form,expected,actual'];
  for (const m of mismatches.slice(0, 2000)) {
    csv.push([m.id, m.kind, m.word, m.details, m.addition, m.form, m.expected, m.actual].map((v) => '"' + String(v ?? '').replace(/"/g, '""') + '"').join(','));
  }
  fs.writeFileSync(path.join(OUT_DIR, 'mismatch-sample.csv'), csv.join('\n'));
  const failures = thresholdFailures(summary);
  const report = { generatedAt: new Date().toISOString(), dictionaryUrl: BASIC_URL, dictionaryRows: rows.length, sonic: { repo: 'https://github.com/sonic16x/interslavic', branch: sonicBranch, commit: sonicCommit, utilsPackage: '@interslavic/utils', utilsVersion: sonicUtilsVersion }, rust: { repoPath: ROOT, commit: rustCommit, dirtyStatus: rustStatus }, skippedReferenceParadigms: skipped.length, skippedPhraseRows: skippedPhrases.length, summary, thresholdFailures: failures, artifactFiles: ['sonic16x-basic.tsv', 'sonic-reference-forms.json', 'rust-forms.json', 'mismatches.json', 'mismatch-sample.csv', 'skipped-sonic-reference.json', 'skipped-phrases.json'] };
  fs.writeFileSync(path.join(OUT_DIR, 'summary.json'), JSON.stringify(report, null, 2));
  writeMarkdownReport(report);
  console.log(JSON.stringify(report, null, 2));
  if (failures.length) {
    console.error(`Comparison thresholds failed:\n- ${failures.join('\n- ')}`);
    process.exit(1);
  }
})().catch((err) => { console.error(err); process.exit(1); });
